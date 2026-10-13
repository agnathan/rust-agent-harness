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
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

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
}

impl Dispatcher {
    pub fn new(bound: usize) -> Self {
        Self {
            bound: bound.max(1),
        }
    }

    pub fn bound(&self) -> usize {
        self.bound
    }

    pub async fn run(&self, registry: &ToolRegistry, calls: &[ToolCall]) -> Vec<CallOutput> {
        self.execute(registry, calls, true).await
    }

    /// One call at a time, including reads. The comparison for the bench.
    pub async fn run_sequential(
        &self,
        registry: &ToolRegistry,
        calls: &[ToolCall],
    ) -> Vec<CallOutput> {
        self.execute(registry, calls, false).await
    }

    async fn execute(
        &self,
        registry: &ToolRegistry,
        calls: &[ToolCall],
        parallel_reads: bool,
    ) -> Vec<CallOutput> {
        let mut outputs = Vec::with_capacity(calls.len());
        let mut index = 0;
        while index < calls.len() {
            if !parallel_reads || self.is_write(registry, &calls[index]) {
                outputs.push(self.call_one(registry, &calls[index]).await);
                index += 1;
                continue;
            }
            let start = index;
            index += 1;
            while index < calls.len() && !self.is_write(registry, &calls[index]) {
                index += 1;
            }
            let mut grouped = self.call_reads(registry, &calls[start..index]).await;
            outputs.append(&mut grouped);
        }
        outputs
    }

    fn is_write(&self, registry: &ToolRegistry, call: &ToolCall) -> bool {
        registry
            .get(&call.name)
            .is_some_and(|tool| tool.kind() == ToolKind::Write)
    }

    async fn call_one(&self, registry: &ToolRegistry, call: &ToolCall) -> CallOutput {
        CallOutput {
            name: call.name.clone(),
            result: invoke(registry, call).await,
        }
    }

    async fn call_reads(&self, registry: &ToolRegistry, calls: &[ToolCall]) -> Vec<CallOutput> {
        let semaphore = Arc::new(Semaphore::new(self.bound));
        let mut tasks = JoinSet::new();
        for (offset, call) in calls.iter().cloned().enumerate() {
            let semaphore = Arc::clone(&semaphore);
            let tool = registry.get(&call.name).map(Arc::clone);
            tasks.spawn(async move {
                let _permit = semaphore.acquire_owned().await.expect("semaphore");
                let result = match tool {
                    Some(tool) => tool.call(call.arguments).await,
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

async fn invoke(registry: &ToolRegistry, call: &ToolCall) -> Result<ToolResult, ToolError> {
    match registry.get(&call.name) {
        Some(tool) => tool.call(call.arguments.clone()).await,
        None => Err(ToolError::Unknown {
            name: call.name.clone(),
        }),
    }
}
