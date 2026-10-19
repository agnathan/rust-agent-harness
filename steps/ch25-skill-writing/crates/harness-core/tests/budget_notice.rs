use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use harness_core::{
    normalize_warning_ratio, run_loop, Budget, CancelFlag, Dispatcher, EchoTool, Provider,
    ScriptedProvider, ScriptedReply, StopReason, Tool, ToolError, ToolKind, ToolRegistry,
    ToolResult, ToolSchema, DEFAULT_SUBAGENT_ITERATIONS, DEFAULT_WARNING_RATIO,
};
use serde_json::{json, Value};

fn named_tool(name: &'static str, hits: Arc<AtomicUsize>) -> Arc<dyn Tool> {
    Arc::new(CountTool { name, hits })
}

struct CountTool {
    name: &'static str,
    hits: Arc<AtomicUsize>,
}

#[async_trait]
impl Tool for CountTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name.into(),
            description: "Counts calls.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        self.hits.fetch_add(1, Ordering::SeqCst);
        Ok(ToolResult::text("ran"))
    }
}

#[test]
fn the_warning_ratio_is_on_here_and_off_outside_the_open_interval() {
    assert_eq!(DEFAULT_WARNING_RATIO, 0.5);
    assert_eq!(DEFAULT_SUBAGENT_ITERATIONS, 50);
    assert_eq!(normalize_warning_ratio(0.5), Some(0.5));
    assert_eq!(normalize_warning_ratio(0.0), None);
    assert_eq!(normalize_warning_ratio(1.0), None);
    assert_eq!(normalize_warning_ratio(f64::NAN), None);
}

#[tokio::test]
async fn a_warning_is_appended_to_the_newest_tool_result() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "one"})),
        ScriptedReply::tool("call-2", "echo", json!({"text": "two"})),
        ScriptedReply::text("done"),
    ]);
    let budget = Budget {
        max_iterations: 4,
        warning_ratio: Some(0.5),
        ..Budget::default()
    };
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &budget,
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    let newest = messages
        .iter()
        .rev()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert!(newest.content.starts_with("two"));
    assert!(newest.content.contains("iteration budget checkpoint"));
    assert!(newest.content.contains("used 2 of 4"));
    let first = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert_eq!(first.content, "one");
}

#[tokio::test]
async fn past_eighty_percent_of_the_time_budget_appends_the_wrap_up() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "partial"})),
        ScriptedReply::text("done"),
    ]);
    let budget = Budget {
        max_duration: Some(Duration::from_millis(2_000)),
        warning_ratio: None,
        ..Budget::default()
    };
    let started = Instant::now() - Duration::from_millis(1_700);
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &budget,
        &CancelFlag::new(),
        started,
    )
    .await;
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert!(tool.content.contains("run time budget nearly exhausted"));
    assert!(outcome.stop() == StopReason::Finished || outcome.stop() == StopReason::Budget);
    assert_eq!(tool.content.matches("run time budget").count(), 1);
}

#[tokio::test]
async fn execute_code_refunds_one_iteration() {
    let hits = Arc::new(AtomicUsize::new(0));
    let mut registry = ToolRegistry::new();
    registry
        .register(named_tool("execute_code", Arc::clone(&hits)))
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "execute_code", json!({})),
        ScriptedReply::text("done"),
    ]);
    let budget = Budget {
        max_iterations: 1,
        ..Budget::default()
    };
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &budget,
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(provider.calls(), 2);
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    assert_eq!(messages.last().unwrap().content, "done");
}

#[tokio::test]
async fn a_delegate_past_the_subagent_cap_is_an_error_result() {
    let hits = Arc::new(AtomicUsize::new(0));
    let mut registry = ToolRegistry::new();
    registry
        .register(named_tool("delegate", Arc::clone(&hits)))
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tools(vec![
            harness_core::RequestedTool {
                id: "call-1".into(),
                name: "delegate".into(),
                arguments: json!({}),
                raw_arguments: None,
            },
            harness_core::RequestedTool {
                id: "call-2".into(),
                name: "delegate".into(),
                arguments: json!({}),
                raw_arguments: None,
            },
        ]),
        ScriptedReply::text("done"),
    ]);
    let budget = Budget {
        max_subagent_iterations: 1,
        ..Budget::default()
    };
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &budget,
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    assert_eq!(outcome.usage().subagent_calls, 1);
    assert!(messages
        .iter()
        .any(|message| message.content == "error: subagent iteration cap"));
}
