//! Lesson 14: cancellation token trees, shared budgets, and a delegating supervisor.
//! Hermes: tools/delegate_tool.py (_build_child_agent, _run_single_child), agent/iteration_budget.py.
use std::future::Future;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

/// Sleep `ms` -> Ok("done"), unless the token is cancelled first -> Err("cancelled").
pub async fn cancellable_work(
    token: CancellationToken,
    ms: u64,
) -> Result<&'static str, &'static str> {
    // Hint: tokio::select! { _ = token.cancelled() => .., _ = sleep(..) => .. }
    todo!("lesson 14")
}

/// Shared across parent and children, like Hermes's IterationBudget.
pub struct Budget {
    remaining: AtomicU32,
}

impl Budget {
    pub fn new(n: u32) -> Self {
        Budget {
            remaining: AtomicU32::new(n),
        }
    }
    /// Atomically take one unit; false if none left. Must never underflow even with many threads.
    pub fn try_consume(&self) -> bool {
        // Hint: fetch_update or a compare_exchange loop
        todo!("lesson 14")
    }
    pub fn remaining(&self) -> u32 {
        self.remaining.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChildOutcome {
    Done { task: String, summary: String },
    Failed { task: String, error: String },
    Cancelled { task: String },
}

pub struct Supervisor {
    pub depth: u32,
    pub max_depth: u32,
    pub max_concurrent: usize,
    token: CancellationToken,
    budget: Arc<Budget>,
}

impl Supervisor {
    pub fn new(max_depth: u32, max_concurrent: usize, budget: u32) -> Self {
        Supervisor {
            depth: 0,
            max_depth,
            max_concurrent,
            token: CancellationToken::new(),
            budget: Arc::new(Budget::new(budget)),
        }
    }
    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }
    pub fn budget(&self) -> Arc<Budget> {
        self.budget.clone()
    }
    pub fn cancel_all(&self) {
        self.token.cancel()
    }
    /// A child supervisor one level deeper: child token (cancelling the parent cancels it),
    /// SAME budget. Err("max depth") when depth+1 > max_depth.
    pub fn child(&self) -> Result<Supervisor, String> {
        // Hint: self.token.child_token(), Arc::clone(&self.budget)
        todo!("lesson 14")
    }

    /// Run each task as its own tokio task, at most max_concurrent at once.
    /// Per task: if budget.try_consume() fails -> Failed{error:"budget exhausted"}.
    /// Give work(task, child_token) the child token; race it against cancellation:
    /// cancelled -> Cancelled; Ok(s) -> Done{summary:s}; Err(e) -> Failed{error:e}; panic -> Failed{error:"panic"}.
    /// Return outcomes in task order (Hermes returns only these summaries to the parent).
    pub async fn delegate<F, Fut>(&self, tasks: Vec<String>, work: F) -> Vec<ChildOutcome>
    where
        F: Fn(String, CancellationToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, String>> + Send + 'static,
    {
        // Hint: JoinSet + Arc<Semaphore> + Arc<F>; spawn returns (index, outcome); sort by index at the end
        todo!("lesson 14")
    }
}
