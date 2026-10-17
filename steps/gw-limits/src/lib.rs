//! Book chapter 15. A tool has its own accept and execute permits.
//! A retry runs only when the tool is idempotent. An open breaker
//! does not call the backend. Web search and delegate_task stop at 50.

use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gw_session::SessionGateway;
use serde_json::Value;
use thiserror::Error;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub use gw_catalog::Gateway as Catalog;
pub use gw_session::{SessionError, SessionView};

/// Calls waiting or running for one tool. The 33rd is refused.
pub const ACCEPT_BOUND: usize = 32;
/// Calls inside one tool body at once.
pub const EXECUTE_BOUND: usize = 8;
pub const BREAKER_THRESHOLD: u32 = 3;
pub const MAX_WEB_SEARCHES: u32 = 50;
pub const MAX_SUBAGENTS: u32 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Transport,
    Status(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Ok(String),
    Fail(FailureKind),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CallOpts {
    pub hedge: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CallError {
    #[error("open")]
    OpenBreaker,
    #[error("saturated")]
    Saturated,
    #[error("timeout")]
    Timeout,
    #[error("{0}")]
    Cap(String),
    #[error("failed")]
    Failed(FailureKind),
}

pub trait Backend: Send + Sync {
    fn call(&self, tool: &str) -> Outcome;
}

#[derive(Debug)]
pub struct Script {
    steps: Mutex<VecDeque<Outcome>>,
    calls: AtomicU32,
}

impl Script {
    pub fn new(steps: Vec<Outcome>) -> Self {
        Self {
            steps: Mutex::new(steps.into()),
            calls: AtomicU32::new(0),
        }
    }

    pub fn calls(&self) -> u32 {
        self.calls.load(Ordering::SeqCst)
    }
}

impl Backend for Script {
    fn call(&self, _tool: &str) -> Outcome {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.steps
            .lock()
            .expect("script")
            .pop_front()
            .unwrap_or(Outcome::Fail(FailureKind::Transport))
    }
}

#[derive(Clone)]
struct ToolGate {
    accept: Arc<Semaphore>,
    execute: Arc<Semaphore>,
}

impl ToolGate {
    fn new() -> Self {
        Self {
            accept: Arc::new(Semaphore::new(ACCEPT_BOUND)),
            execute: Arc::new(Semaphore::new(EXECUTE_BOUND)),
        }
    }
}

#[derive(Default)]
struct Breaker {
    failures: u32,
    open: bool,
}

pub struct LimitedGateway {
    inner: SessionGateway,
    idempotent: Mutex<HashMap<String, bool>>,
    gates: Mutex<HashMap<String, ToolGate>>,
    breakers: Mutex<HashMap<String, Breaker>>,
    web_searches: AtomicU32,
    subagents: AtomicU32,
}

impl LimitedGateway {
    pub fn new() -> Self {
        Self {
            inner: SessionGateway::new(),
            idempotent: Mutex::new(HashMap::new()),
            gates: Mutex::new(HashMap::new()),
            breakers: Mutex::new(HashMap::new()),
            web_searches: AtomicU32::new(0),
            subagents: AtomicU32::new(0),
        }
    }

    pub fn catalog(&self) -> &Catalog {
        self.inner.catalog()
    }

    pub fn open_session(&self, session_id: impl Into<String>) {
        self.inner.open_session(session_id);
    }

    pub fn session_tools(&self, session_id: &str) -> Option<SessionView> {
        self.inner.session_tools(session_id)
    }

    pub fn set_idempotent(&self, tool: &str, idempotent: bool) {
        self.idempotent
            .lock()
            .expect("policies")
            .insert(tool.to_string(), idempotent);
    }

    pub fn breaker_open(&self, tool: &str) -> bool {
        self.breakers
            .lock()
            .expect("breakers")
            .get(tool)
            .is_some_and(|state| state.open)
    }

    pub fn accept_available(&self, tool: &str) -> usize {
        self.gate(tool).accept.available_permits()
    }

    pub fn execute_available(&self, tool: &str) -> usize {
        self.gate(tool).execute.available_permits()
    }

    /// Zero the per-turn web-search and subagent counters.
    pub fn reset_for_turn(&self) {
        self.web_searches.store(0, Ordering::SeqCst);
        self.subagents.store(0, Ordering::SeqCst);
    }

    /// Take one execute permit and drop it. The bench times this pair.
    pub fn roundtrip_execute(&self, tool: &str) -> Result<(), CallError> {
        let permit = self.try_execute(tool)?;
        drop(permit);
        Ok(())
    }

    pub fn hold_accept(&self, tool: &str) -> Result<OwnedSemaphorePermit, CallError> {
        self.try_accept(tool)
    }

    pub fn dispatch(
        &self,
        tool: &str,
        args: &str,
        opts: CallOpts,
        backend: &dyn Backend,
    ) -> Result<String, CallError> {
        if self.breaker_open(tool) {
            return Err(CallError::OpenBreaker);
        }
        if let Some(message) = self.block_cap(tool, args) {
            return Err(CallError::Cap(message));
        }
        self.note_cap(tool, args);
        let idempotent = self.is_idempotent(tool);
        let first = backend.call(tool);
        let retry = matches!(
            &first,
            Outcome::Fail(kind) if idempotent && retriable(*kind) && !opts.hedge
        );
        let outcome = if opts.hedge && idempotent {
            let second = backend.call(tool);
            match first {
                Outcome::Ok(body) => Outcome::Ok(body),
                Outcome::Fail(_) => second,
            }
        } else if retry {
            backend.call(tool)
        } else {
            first
        };
        match outcome {
            Outcome::Ok(body) => {
                self.note_success(tool);
                Ok(body)
            }
            Outcome::Fail(kind) => {
                self.note_failure(tool);
                Err(CallError::Failed(kind))
            }
        }
    }

    pub async fn run_with_timeout<F, Fut>(
        &self,
        tool: &str,
        budget: Duration,
        body: F,
    ) -> Result<String, CallError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = String>,
    {
        let accept = self.try_accept(tool)?;
        let execute = self
            .gate(tool)
            .execute
            .clone()
            .acquire_owned()
            .await
            .expect("execute semaphore");
        let result = match tokio::time::timeout(budget, body()).await {
            Ok(value) => Ok(value),
            Err(_elapsed) => Err(CallError::Timeout),
        };
        drop(execute);
        drop(accept);
        result
    }

    fn is_idempotent(&self, tool: &str) -> bool {
        self.idempotent
            .lock()
            .expect("policies")
            .get(tool)
            .copied()
            .unwrap_or(false)
    }

    fn block_cap(&self, tool: &str, args: &str) -> Option<String> {
        match tool {
            "web_search" if self.web_searches.load(Ordering::SeqCst) >= MAX_WEB_SEARCHES => {
                Some(format!(
                    "Blocked web_search: this turn has already made {MAX_WEB_SEARCHES} web searches, the per-turn limit. This looks like a runaway search loop. Work with the results you already have and give the user your answer."
                ))
            }
            "delegate_task"
                if spawn_count(args) > 0
                    && self.subagents.load(Ordering::SeqCst) >= MAX_SUBAGENTS =>
            {
                Some(format!(
                    "Blocked delegate_task: this turn has already spawned {MAX_SUBAGENTS} subagents (limit {MAX_SUBAGENTS}). This looks like a runaway delegation loop. Finish the work with the results you have and answer the user."
                ))
            }
            _ => None,
        }
    }

    fn note_cap(&self, tool: &str, args: &str) {
        match tool {
            "web_search" => {
                self.web_searches.fetch_add(1, Ordering::SeqCst);
            }
            "delegate_task" => {
                self.subagents
                    .fetch_add(spawn_count(args), Ordering::SeqCst);
            }
            _ => {}
        }
    }

    fn note_failure(&self, tool: &str) {
        let mut breakers = self.breakers.lock().expect("breakers");
        let state = breakers.entry(tool.to_string()).or_default();
        if state.open {
            return;
        }
        state.failures += 1;
        if state.failures >= BREAKER_THRESHOLD {
            state.open = true;
        }
    }

    fn note_success(&self, tool: &str) {
        let mut breakers = self.breakers.lock().expect("breakers");
        if let Some(state) = breakers.get_mut(tool) {
            if !state.open {
                state.failures = 0;
            }
        }
    }

    fn gate(&self, tool: &str) -> ToolGate {
        let mut gates = self.gates.lock().expect("gates");
        gates
            .entry(tool.to_string())
            .or_insert_with(ToolGate::new)
            .clone()
    }

    fn try_accept(&self, tool: &str) -> Result<OwnedSemaphorePermit, CallError> {
        self.gate(tool)
            .accept
            .clone()
            .try_acquire_owned()
            .map_err(|_| CallError::Saturated)
    }

    fn try_execute(&self, tool: &str) -> Result<OwnedSemaphorePermit, CallError> {
        self.gate(tool)
            .execute
            .clone()
            .try_acquire_owned()
            .map_err(|_| CallError::Saturated)
    }
}

impl Default for LimitedGateway {
    fn default() -> Self {
        Self::new()
    }
}

fn retriable(kind: FailureKind) -> bool {
    match kind {
        FailureKind::Transport => true,
        FailureKind::Status(429 | 503) => false,
        FailureKind::Status(_) => true,
    }
}

fn spawn_count(args: &str) -> u32 {
    let Ok(value) = serde_json::from_str::<Value>(args) else {
        return 1;
    };
    if let Some(action) = value.get("action").and_then(Value::as_str) {
        if matches!(
            action.to_ascii_lowercase().as_str(),
            "list" | "steer" | "stop"
        ) {
            return 0;
        }
    }
    match value.get("tasks") {
        Some(Value::Array(items)) if !items.is_empty() => items.len() as u32,
        _ => 1,
    }
}
