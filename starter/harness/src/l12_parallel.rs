//! Lesson 12: async tool registry and safe parallel tool calls.
//! Hermes: agent/tool_dispatch_helpers.py (_PARALLEL_SAFE_TOOLS, _PATH_SCOPED_TOOLS) + agent/tool_executor.py.
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

#[derive(Debug, Clone, PartialEq)]
pub enum Concurrency {
    ReadOnly,
    PathScoped(String),
    Exclusive,
}

/// Group call indices into batches that may run in parallel, preserving order:
/// - ReadOnly joins the current batch
/// - PathScoped(p) joins the current batch unless the batch already has PathScoped(p); then start a new batch
/// - Exclusive always gets a batch of its own
pub fn plan_batches(calls: &[Concurrency]) -> Vec<Vec<usize>> {
    // Hint: keep `current: Vec<usize>`; push it into `out` when you need to start fresh
    todo!("lesson 12")
}

#[async_trait]
pub trait AsyncTool: Send + Sync {
    fn name(&self) -> &str;
    fn concurrency(&self, args: &Value) -> Concurrency;
    async fn call(&self, args: Value) -> Result<String, String>;
}

/// Read-only tool that sleeps `ms` and returns its name.
pub struct SleepTool {
    pub name: String,
    pub ms: u64,
}

#[async_trait]
impl AsyncTool for SleepTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn concurrency(&self, _args: &Value) -> Concurrency {
        Concurrency::ReadOnly
    }
    async fn call(&self, _args: Value) -> Result<String, String> {
        // Hint: tokio::time::sleep
        todo!("lesson 12")
    }
}

pub type AsyncRegistry = HashMap<String, Arc<dyn AsyncTool>>;

/// Run all calls concurrently, at most `max_parallel` at once (Semaphore), results in input order.
/// Unknown tool -> Err("unknown tool: <name>").
pub async fn execute_parallel(
    reg: &AsyncRegistry,
    calls: Vec<(String, Value)>,
    max_parallel: usize,
) -> Vec<Result<String, String>> {
    // Hint: Arc<Semaphore>; for each call build an async block that acquires a permit; join_all
    todo!("lesson 12")
}

/// Plan with plan_batches (using each tool's concurrency; unknown tools count as Exclusive),
/// run batches one after another, each batch via execute_parallel. Results in input order.
pub async fn execute_planned(
    reg: &AsyncRegistry,
    calls: Vec<(String, Value)>,
    max_parallel: usize,
) -> Vec<Result<String, String>> {
    // Hint: compute Vec<Concurrency>, plan, then fill a Vec<Option<Result<..>>> by index
    todo!("lesson 12")
}
