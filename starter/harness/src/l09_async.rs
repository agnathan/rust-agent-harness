//! Lesson 09: async/await and tokio.
use std::time::Duration;
use tokio::time::sleep;

/// Pretend tool: sleeps `ms` then returns "<name> done".
pub async fn slow_tool(name: &str, ms: u64) -> String {
    // Hint: sleep(Duration::from_millis(ms)).await
    todo!("lesson 09")
}

/// Run jobs one after another.
pub async fn run_sequential(jobs: &[(&str, u64)]) -> Vec<String> {
    // Hint: for loop with .await inside
    todo!("lesson 09")
}

/// Run jobs concurrently on ONE task; results in input order.
pub async fn run_concurrent(jobs: &[(&str, u64)]) -> Vec<String> {
    // Hint: futures::future::join_all
    todo!("lesson 09")
}

/// Run slow_tool("job", job_ms) but give up after limit_ms -> Err("timeout").
pub async fn with_timeout(job_ms: u64, limit_ms: u64) -> Result<String, &'static str> {
    // Hint: tokio::time::timeout
    todo!("lesson 09")
}

/// Spawn n tasks (tokio::spawn / JoinSet), task i returns i*i; return the sum.
pub async fn spawn_and_sum(n: u64) -> u64 {
    // Hint: tokio::task::JoinSet; join_next().await in a while let loop
    todo!("lesson 09")
}
