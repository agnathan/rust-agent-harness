use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, EchoTool, Provider, ScriptedProvider, ScriptedReply,
    StopReason, Tool, ToolError, ToolKind, ToolRegistry, ToolResult, ToolSchema,
    MAX_INVALID_STRIKES, MAX_JSON_RETRIES,
};
use serde_json::{json, Value};

fn echo_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    registry
}

fn roles(messages: &[harness_core::Message]) -> Vec<&'static str> {
    messages
        .iter()
        .map(|message| match message.role {
            harness_core::Role::System => "system",
            harness_core::Role::User => "user",
            harness_core::Role::Assistant => "assistant",
            harness_core::Role::Tool => "tool",
        })
        .collect()
}

#[test]
fn the_strike_and_retry_caps_match_hermes() {
    assert_eq!(MAX_INVALID_STRIKES, 3);
    assert_eq!(MAX_JSON_RETRIES, 3);
}

#[tokio::test]
async fn an_unknown_name_repairs_to_the_close_tool() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "eco", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let registry = echo_registry();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    let assistant = messages
        .iter()
        .find(|message| !message.tool_calls.is_empty())
        .unwrap();
    assert_eq!(assistant.tool_calls[0].name, "echo");
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert_eq!(tool.content, "hello");
    assert_eq!(
        roles(&messages),
        vec!["user", "assistant", "tool", "assistant"]
    );
}

#[tokio::test]
async fn three_unknown_batches_return_the_partial_transcript() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "nope", json!({})),
        ScriptedReply::tool("call-2", "nope", json!({})),
        ScriptedReply::tool("call-3", "nope", json!({})),
        ScriptedReply::text("should not run"),
    ]);
    let registry = echo_registry();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::InvalidToolCalls);
    assert_eq!(provider.calls(), 3);
    let last = messages.last().unwrap();
    assert_eq!(last.role, harness_core::Role::Tool);
    assert!(last.content.contains("does not exist"));
    assert!(last.content.contains("echo"));
    assert_eq!(roles(&messages).last(), Some(&"tool"));
}

#[tokio::test]
async fn a_mixed_batch_runs_the_valid_call_and_resets_strikes() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "nope", json!({})),
        ScriptedReply::tools(vec![
            harness_core::RequestedTool {
                id: "call-2".into(),
                name: "echo".into(),
                arguments: json!({"text": "hello"}),
                raw_arguments: None,
            },
            harness_core::RequestedTool {
                id: "call-3".into(),
                name: "nope".into(),
                arguments: json!({}),
                raw_arguments: None,
            },
        ]),
        ScriptedReply::tool("call-4", "nope", json!({})),
        ScriptedReply::text("done"),
    ]);
    let registry = echo_registry();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert!(messages.iter().any(|message| message.content == "hello"));
    assert!(messages
        .iter()
        .any(|message| message.content.contains("does not exist")));
}

#[tokio::test]
async fn malformed_json_is_retried_then_returned_as_a_tool_error() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::raw_tool("call-1", "echo", r#"{"text":}"#),
        ScriptedReply::raw_tool("call-2", "echo", r#"{"text":}"#),
        ScriptedReply::raw_tool("call-3", "echo", r#"{"text":}"#),
        ScriptedReply::text("could not parse"),
    ]);
    let registry = echo_registry();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(provider.calls(), 4);
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert!(tool.content.contains("malformed JSON"));
    assert_eq!(messages.last().unwrap().content, "could not parse");
    assert_eq!(
        roles(&messages),
        vec!["user", "assistant", "tool", "assistant"]
    );
}

#[tokio::test]
async fn one_malformed_reply_is_replaced_by_the_retry() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::raw_tool("call-1", "echo", r#"{"text":}"#),
        ScriptedReply::tool("call-2", "echo", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let registry = echo_registry();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(provider.calls(), 3);
    assert_eq!(
        messages
            .iter()
            .filter(|message| message.role == harness_core::Role::Assistant)
            .count(),
        2
    );
    assert!(messages
        .iter()
        .any(|message| message.content == "hello" && message.tool_call_id.is_some()));
}

#[tokio::test]
async fn truncated_arguments_stop_without_a_retry() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::raw_tool("call-1", "echo", r#"{"text":"hel"#),
        ScriptedReply::text("should not run"),
    ]);
    let registry = echo_registry();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Partial);
    assert_eq!(provider.calls(), 1);
    assert!(messages.last().unwrap().content.contains("truncated JSON"));
}

struct SlowTool;

#[async_trait]
impl Tool for SlowTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "slow".into(),
            description: "Sleeps.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        tokio::time::sleep(Duration::from_secs(30)).await;
        Ok(ToolResult::text("too late"))
    }
}

#[tokio::test]
async fn a_tool_timeout_comes_back_as_a_tool_result() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SlowTool) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "slow", json!({})),
        ScriptedReply::text("continued"),
    ]);
    let started = Instant::now();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default().with_tool_timeout(Duration::from_millis(40)),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(
        messages
            .iter()
            .find(|message| message.tool_call_id.is_some())
            .unwrap()
            .content,
        "error: timeout"
    );
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[tokio::test]
async fn duplicate_tool_call_ids_are_made_unique() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tools(vec![
            harness_core::RequestedTool {
                id: "call-1".into(),
                name: "echo".into(),
                arguments: json!({"text": "a"}),
                raw_arguments: None,
            },
            harness_core::RequestedTool {
                id: "call-1".into(),
                name: "echo".into(),
                arguments: json!({"text": "b"}),
                raw_arguments: None,
            },
        ]),
        ScriptedReply::text("done"),
    ]);
    let registry = echo_registry();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    let ids: Vec<_> = messages
        .iter()
        .filter_map(|message| message.tool_call_id.clone())
        .collect();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
}
