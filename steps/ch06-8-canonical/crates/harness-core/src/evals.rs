//! A small saved eval set. Later chapters re-run these tasks on the mock.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::{json, Value};

use crate::agent::{run_loop, TurnOutcome};
use crate::budget::{Budget, CancelFlag, StopReason};
use crate::dispatch::{Dispatcher, DISPATCH_BOUND};
use crate::message::{Message, RequestedTool};
use crate::provider::{ScriptedProvider, ScriptedReply};
use crate::registry::ToolRegistry;
use crate::tool::{Tool, ToolError, ToolKind, ToolResult, ToolSchema};

pub struct SavedEval {
    pub name: &'static str,
    pub prompt: &'static str,
    pub expect_final: &'static str,
    pub expect_tool: Option<&'static str>,
}

pub fn saved_evals() -> &'static [SavedEval] {
    &[
        SavedEval {
            name: "echo-hello",
            prompt: "say hello",
            expect_final: "hello",
            expect_tool: Some("echo"),
        },
        SavedEval {
            name: "direct",
            prompt: "status",
            expect_final: "ok",
            expect_tool: None,
        },
        SavedEval {
            name: "eight-reads",
            prompt: "fan out",
            expect_final: "done",
            expect_tool: Some("delay_read"),
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalScore {
    pub name: &'static str,
    pub passed: bool,
    pub detail: String,
}

pub struct DelayRead {
    pub delay: Duration,
    pub current: Arc<AtomicUsize>,
    pub peak: Arc<AtomicUsize>,
}

impl DelayRead {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            current: Arc::new(AtomicUsize::new(0)),
            peak: Arc::new(AtomicUsize::new(0)),
        }
    }
}

#[async_trait]
impl Tool for DelayRead {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "delay_read".into(),
            description: "A read that waits, so overlap is visible.".into(),
            input_schema: json!({"type": "object", "properties": {}}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        let now = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        self.peak.fetch_max(now, Ordering::SeqCst);
        if !self.delay.is_zero() {
            tokio::time::sleep(self.delay).await;
        }
        self.current.fetch_sub(1, Ordering::SeqCst);
        Ok(ToolResult::text("waited"))
    }
}

fn script_for(eval: &SavedEval) -> Vec<ScriptedReply> {
    match eval.expect_tool {
        Some(name) => vec![
            ScriptedReply::tool("call-1", name, json!({"text": eval.expect_final})),
            ScriptedReply::text(eval.expect_final),
        ],
        None => vec![ScriptedReply::text(eval.expect_final)],
    }
}

pub async fn run_saved_eval(eval: &SavedEval) -> (EvalScore, TurnOutcome) {
    let replies = if eval.name == "eight-reads" {
        vec![
            ScriptedReply::tools(eight_delay_calls()),
            ScriptedReply::text(eval.expect_final),
        ]
    } else {
        script_for(eval)
    };
    let provider = ScriptedProvider::new(replies);
    let mut registry = ToolRegistry::new();
    let delay = DelayRead::new(Duration::from_millis(20));
    let peak = Arc::clone(&delay.peak);
    registry
        .register(Arc::new(crate::tool::EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    registry
        .register(Arc::new(delay) as Arc<dyn Tool>)
        .expect("delay");
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let mut messages = vec![Message::user(eval.prompt)];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &Budget::default(),
        &CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let final_text = messages
        .iter()
        .rev()
        .find(|message| {
            message.role == crate::message::Role::Assistant && message.tool_calls.is_empty()
        })
        .map(|message| message.content.clone())
        .unwrap_or_default();
    let tool_ok = match eval.expect_tool {
        Some(name) => messages
            .iter()
            .any(|message| message.tool_calls.iter().any(|call| call.name == name)),
        None => messages.iter().all(|message| message.tool_calls.is_empty()),
    };
    let within_bound = peak.load(std::sync::atomic::Ordering::SeqCst) <= DISPATCH_BOUND;
    let passed = outcome.stop == StopReason::Finished
        && final_text == eval.expect_final
        && tool_ok
        && within_bound;
    let detail = format!("stop={outcome:?} final={final_text} peak_ok={within_bound}");
    (
        EvalScore {
            name: eval.name,
            passed,
            detail,
        },
        outcome,
    )
}

pub async fn run_saved_evals() -> Vec<EvalScore> {
    let mut scores = Vec::new();
    for eval in saved_evals() {
        scores.push(run_saved_eval(eval).await.0);
    }
    scores
}

pub fn eight_delay_calls() -> Vec<RequestedTool> {
    (0..8)
        .map(|index| RequestedTool {
            id: format!("call-{index}"),
            name: "delay_read".into(),
            arguments: json!({}),
            raw_arguments: None,
        })
        .collect()
}
