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
    let passed = outcome.stop() == StopReason::Finished
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

pub fn stress_eval_names() -> &'static [&'static str] {
    &[
        "tool-fails-then-recovers",
        "hits-iteration-cap",
        "unknown-tool-repair",
        "stall",
    ]
}

pub async fn run_stress_evals() -> Vec<EvalScore> {
    let mut scores = Vec::new();
    for name in stress_eval_names() {
        scores.push(run_stress_eval(name).await);
    }
    scores
}

pub async fn run_stress_eval(name: &str) -> EvalScore {
    match name {
        "tool-fails-then-recovers" => fails_then_recovers().await,
        "hits-iteration-cap" => hits_iteration_cap().await,
        "unknown-tool-repair" => unknown_tool_repair().await,
        "stall" => stall_eval().await,
        _ => EvalScore {
            name: "unknown",
            passed: false,
            detail: format!("unknown eval {name}"),
        },
    }
}

/// Returns the missing-key error before any client is built. A present key
/// means the caller may place a live call. This function does not.
pub fn prepare_live_evals(api_key: Option<&str>) -> Result<(), String> {
    let present = api_key.map(str::trim).filter(|value| !value.is_empty());
    if present.is_none() {
        Err("missing OPENROUTER_API_KEY: pass --api-key, or run with --mock".into())
    } else {
        Ok(())
    }
}

async fn fails_then_recovers() -> EvalScore {
    let hits = Arc::new(AtomicUsize::new(0));
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(Flaky {
            hits: Arc::clone(&hits),
        }) as Arc<dyn Tool>)
        .expect("flaky");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "flaky", json!({})),
        ScriptedReply::tool("call-2", "flaky", json!({})),
        ScriptedReply::text("recovered"),
    ]);
    let mut messages = vec![Message::user("retry the tool")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::new(DISPATCH_BOUND),
        &Budget::default(),
        &crate::budget::CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let saw_error = messages
        .iter()
        .any(|message| message.content.contains("boom"));
    let saw_ok = messages.iter().any(|message| message.content == "ok");
    let passed = outcome.stop() == StopReason::Finished
        && outcome.text() == "recovered"
        && saw_error
        && saw_ok
        && hits.load(Ordering::SeqCst) == 2;
    EvalScore {
        name: "tool-fails-then-recovers",
        passed,
        detail: format!("stop={} text={}", outcome.label(), outcome.text()),
    }
}

async fn hits_iteration_cap() -> EvalScore {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(crate::tool::EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "one"})),
        ScriptedReply::tool("call-2", "echo", json!({"text": "two"})),
        ScriptedReply::text("partial"),
    ]);
    let budget = Budget {
        max_iterations: 2,
        ..Budget::default()
    };
    let mut messages = vec![Message::user("keep going")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::new(DISPATCH_BOUND),
        &budget,
        &crate::budget::CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let passed = outcome.stop() == StopReason::Budget && outcome.text() == "partial";
    EvalScore {
        name: "hits-iteration-cap",
        passed,
        detail: format!("stop={} text={}", outcome.label(), outcome.text()),
    }
}

async fn unknown_tool_repair() -> EvalScore {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(crate::tool::EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "eco", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let mut messages = vec![Message::user("say hello")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::new(DISPATCH_BOUND),
        &Budget::default(),
        &crate::budget::CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let repaired = messages
        .iter()
        .any(|message| message.tool_calls.iter().any(|call| call.name == "echo"));
    let passed = outcome.stop() == StopReason::Finished && outcome.text() == "hello" && repaired;
    EvalScore {
        name: "unknown-tool-repair",
        passed,
        detail: format!("stop={} repaired={repaired}", outcome.label()),
    }
}

async fn stall_eval() -> EvalScore {
    let payload = "x".repeat(crate::stall::STUB_MIN_CHARS);
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(crate::tool::EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    let replies = (0..crate::stall::STALL_HALT_AFTER)
        .map(|index| ScriptedReply::tool(format!("call-{index}"), "echo", json!({"text": payload})))
        .collect();
    let provider = ScriptedProvider::new(replies);
    let mut messages = vec![Message::user("again")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::new(DISPATCH_BOUND),
        &Budget::default(),
        &crate::budget::CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let stubbed = messages
        .iter()
        .any(|message| message.content.contains("byte-identical"));
    let passed = outcome.stop() == StopReason::Stalled && stubbed;
    EvalScore {
        name: "stall",
        passed,
        detail: format!("stop={} stubbed={stubbed}", outcome.label()),
    }
}

struct Flaky {
    hits: Arc<AtomicUsize>,
}

#[async_trait]
impl Tool for Flaky {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "flaky".into(),
            description: "Fails once, then succeeds.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        let hit = self.hits.fetch_add(1, Ordering::SeqCst);
        if hit == 0 {
            Err(ToolError::Failed {
                message: "boom".into(),
            })
        } else {
            Ok(ToolResult::text("ok"))
        }
    }
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
