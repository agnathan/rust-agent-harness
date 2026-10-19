//! Run a batch of tool calls.
//!
//! Consecutive read-only calls run together, up to `bound` at once.
//! A write runs alone: the reads before it finish first, and the reads
//! after it wait. `run_sequential` is the same calls one at a time, so a
//! bench can compare the two.

use crate::registry::ToolRegistry;
use crate::tool::{Tool, ToolError, ToolKind, ToolResult};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

pub const DISPATCH_BOUND: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub name: String,
    pub arguments: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallOutput {
    pub name: String,
    pub result: Result<ToolResult, ToolError>,
}

pub struct Dispatcher {
    bound: usize,
    tool_timeout: Duration,
}

impl Dispatcher {
    pub fn new(bound: usize) -> Self {
        Self {
            bound: bound.max(1),
            tool_timeout: Duration::from_secs(30),
        }
    }

    pub fn with_tool_timeout(mut self, timeout: Duration) -> Self {
        self.tool_timeout = timeout;
        self
    }

    pub fn bound(&self) -> usize {
        self.bound
    }

    pub async fn run(&self, registry: &ToolRegistry, calls: &[ToolCall]) -> Vec<CallOutput> {
        self.execute(registry, calls, true, None).await
    }

    /// Each call is raced against `token` with `select!`. A fired token
    /// becomes a tool error for that call. It does not leave the batch half
    /// recorded.
    pub async fn run_raced(
        &self,
        registry: &ToolRegistry,
        calls: &[ToolCall],
        token: &CancellationToken,
    ) -> Vec<CallOutput> {
        self.execute(registry, calls, true, Some(token.clone()))
            .await
    }

    /// One call at a time, including reads. The comparison for the bench.
    pub async fn run_sequential(
        &self,
        registry: &ToolRegistry,
        calls: &[ToolCall],
    ) -> Vec<CallOutput> {
        self.execute(registry, calls, false, None).await
    }

    async fn execute(
        &self,
        registry: &ToolRegistry,
        calls: &[ToolCall],
        parallel_reads: bool,
        token: Option<CancellationToken>,
    ) -> Vec<CallOutput> {
        let mut outputs = Vec::with_capacity(calls.len());
        let mut index = 0;
        while index < calls.len() {
            if !parallel_reads || self.is_write(registry, &calls[index]) {
                outputs.push(self.call_one(registry, &calls[index], token.clone()).await);
                index += 1;
                continue;
            }
            let start = index;
            index += 1;
            while index < calls.len() && !self.is_write(registry, &calls[index]) {
                index += 1;
            }
            let mut grouped = self
                .call_reads(registry, &calls[start..index], token.clone())
                .await;
            outputs.append(&mut grouped);
        }
        outputs
    }

    fn is_write(&self, registry: &ToolRegistry, call: &ToolCall) -> bool {
        registry
            .get(&call.name)
            .is_some_and(|tool| tool.kind() == ToolKind::Write)
    }

    async fn call_one(
        &self,
        registry: &ToolRegistry,
        call: &ToolCall,
        token: Option<CancellationToken>,
    ) -> CallOutput {
        CallOutput {
            name: call.name.clone(),
            result: invoke(registry, call, token, self.tool_timeout).await,
        }
    }

    async fn call_reads(
        &self,
        registry: &ToolRegistry,
        calls: &[ToolCall],
        token: Option<CancellationToken>,
    ) -> Vec<CallOutput> {
        let semaphore = Arc::new(Semaphore::new(self.bound));
        let mut tasks = JoinSet::new();
        for (offset, call) in calls.iter().cloned().enumerate() {
            let semaphore = Arc::clone(&semaphore);
            let tool = registry.get(&call.name).map(Arc::clone);
            let token = token.clone();
            let timeout = self.tool_timeout;
            tasks.spawn(async move {
                let _permit = semaphore.acquire_owned().await.expect("semaphore");
                let result = match tool {
                    Some(tool) => race_call(tool, call.arguments, token, timeout).await,
                    None => Err(ToolError::Unknown {
                        name: call.name.clone(),
                    }),
                };
                (
                    offset,
                    CallOutput {
                        name: call.name,
                        result,
                    },
                )
            });
        }
        let mut slotted = vec![None; calls.len()];
        while let Some(joined) = tasks.join_next().await {
            let (offset, output) = joined.expect("tool task");
            slotted[offset] = Some(output);
        }
        slotted.into_iter().flatten().collect()
    }
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new(DISPATCH_BOUND)
    }
}

async fn invoke(
    registry: &ToolRegistry,
    call: &ToolCall,
    token: Option<CancellationToken>,
    timeout: Duration,
) -> Result<ToolResult, ToolError> {
    match registry.get(&call.name) {
        Some(tool) => race_call(Arc::clone(tool), call.arguments.clone(), token, timeout).await,
        None => Err(ToolError::Unknown {
            name: call.name.clone(),
        }),
    }
}

async fn race_call(
    tool: Arc<dyn Tool>,
    arguments: Value,
    token: Option<CancellationToken>,
    timeout: Duration,
) -> Result<ToolResult, ToolError> {
    let work = async {
        match token {
            None => tool.call(arguments).await,
            Some(token) => {
                tokio::select! {
                    biased;
                    _ = token.cancelled() => Err(ToolError::Failed {
                        message: "cancelled".into(),
                    }),
                    result = tool.call(arguments) => result,
                }
            }
        }
    };
    match tokio::time::timeout(timeout, work).await {
        Ok(result) => result,
        Err(_elapsed) => Err(ToolError::Failed {
            message: "timeout".into(),
        }),
    }
}
