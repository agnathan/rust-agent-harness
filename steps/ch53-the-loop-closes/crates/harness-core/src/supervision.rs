//! Book chapter 36. A failed child is restarted once. The restart decision
//! is bookkeeping: the child's work is not part of it. Three chained
//! restarts trip the breaker and skip auto-resume.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

/// Hermes `DEFAULT_MAX_RESTARTS`.
pub const MAX_RESTARTS: u32 = 3;
/// Hermes `DEFAULT_WINDOW_SECONDS`. The gap is floored by this.
pub const WINDOW_SECONDS: u64 = 60;
/// Hermes `DEFAULT_MAX_GAP_SECONDS`. Boots farther apart are a new chain.
pub const MAX_GAP_SECONDS: u64 = 300;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartError {
    Failed,
    Cancelled,
    Budget,
    Breaker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildFailed;

/// Scripted child. `complete` never opens a socket.
#[derive(Debug)]
pub struct FakeModel {
    results: VecDeque<Result<String, ChildFailed>>,
    pub calls: u32,
}

impl FakeModel {
    pub fn script(results: impl IntoIterator<Item = Result<String, ChildFailed>>) -> Self {
        Self {
            results: results.into_iter().collect(),
            calls: 0,
        }
    }

    pub fn complete(&mut self) -> Result<String, ChildFailed> {
        self.calls += 1;
        self.results.pop_front().unwrap_or(Err(ChildFailed))
    }
}

pub struct Call {
    cancelled: AtomicBool,
    budget: AtomicU32,
}

impl Call {
    pub fn new(budget: u32) -> Self {
        Self {
            cancelled: AtomicBool::new(false),
            budget: AtomicU32::new(budget),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub fn budget(&self) -> u32 {
        self.budget.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
pub struct Supervisor {
    boots: Mutex<Vec<u64>>,
    restarts: AtomicU32,
    io_failed: AtomicBool,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            boots: Mutex::new(Vec::new()),
            restarts: AtomicU32::new(0),
            io_failed: AtomicBool::new(false),
        }
    }

    pub fn fail_io(&self) {
        self.io_failed.store(true, Ordering::Relaxed);
    }

    pub fn restarts(&self) -> u32 {
        self.restarts.load(Ordering::Relaxed)
    }

    /// Record one restart at `now` (seconds). True when the chain has reached
    /// `MAX_RESTARTS` and auto-resume should be skipped. An I/O failure records
    /// nothing and returns false: fail open.
    pub fn record(&self, now: u64) -> bool {
        if self.io_failed.load(Ordering::Relaxed) {
            return false;
        }
        let gap = WINDOW_SECONDS.max(MAX_GAP_SECONDS);
        let mut boots = self.boots.lock().expect("boots");
        let mut chain = Vec::new();
        let mut prev = now;
        for stamp in boots.iter().rev() {
            if prev.saturating_sub(*stamp) > gap {
                break;
            }
            chain.push(*stamp);
            prev = *stamp;
        }
        chain.reverse();
        chain.push(now);
        *boots = chain;
        self.restarts.fetch_add(1, Ordering::Relaxed);
        boots.len() as u32 >= MAX_RESTARTS
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the child. On failure, record a restart and try once more, unless the
/// call is cancelled, the budget is zero, or the breaker tripped. The record
/// is the bookkeeping. The child's `complete` is the work, and the bench does
/// not call this function.
pub fn restart_child(
    child: &mut FakeModel,
    call: &Call,
    supervisor: &Supervisor,
    now: u64,
) -> Result<String, RestartError> {
    if call.cancelled() || call.budget() == 0 {
        return Err(if call.cancelled() {
            RestartError::Cancelled
        } else {
            RestartError::Budget
        });
    }
    match child.complete() {
        Ok(text) => Ok(text),
        Err(ChildFailed) => {
            if supervisor.record(now) {
                return Err(RestartError::Breaker);
            }
            if call.cancelled() {
                return Err(RestartError::Cancelled);
            }
            child.complete().map_err(|_| RestartError::Failed)
        }
    }
}

pub fn demo() -> String {
    let mut child = FakeModel::script([Err(ChildFailed), Ok("ok".into())]);
    let call = Call::new(1);
    let supervisor = Supervisor::new();
    match restart_child(&mut child, &call, &supervisor, 0) {
        Ok(text) => format!("restarted: {text}"),
        Err(_) => "failed".into(),
    }
}

pub const HEARTBEAT_INTERVAL_SECS: u64 = 30;
pub const HEARTBEAT_STALE_CYCLES_IDLE: u64 = 15;
pub const HEARTBEAT_STALE_CYCLES_IN_TOOL: u64 = 40;

/// One observation of a child. Progress is a new iteration, a different tool,
/// or a newer `last_activity_ts`. An in-flight model wait is that timestamp
/// moving, so a slow model is not idle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pulse {
    pub iteration: u32,
    pub tool: Option<String>,
    pub last_activity_ts: u64,
    pub in_tool: bool,
}

/// True when nothing has progressed for the idle cap (15 cycles, 450s) or,
/// while a tool is current, the longer cap (40 cycles, 1200s).
pub fn is_stale(
    previous: &Pulse,
    now: u64,
    iteration: u32,
    tool: Option<&str>,
    activity: u64,
) -> bool {
    let progressed = iteration != previous.iteration
        || tool != previous.tool.as_deref()
        || activity > previous.last_activity_ts;
    if progressed {
        return false;
    }
    let cycles = now.saturating_sub(previous.last_activity_ts) / HEARTBEAT_INTERVAL_SECS;
    let limit = if previous.in_tool {
        HEARTBEAT_STALE_CYCLES_IN_TOOL
    } else {
        HEARTBEAT_STALE_CYCLES_IDLE
    };
    cycles >= limit
}
