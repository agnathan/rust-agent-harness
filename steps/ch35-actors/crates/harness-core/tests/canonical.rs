use std::sync::Arc;
use std::time::Instant;

use harness_core::{
    canonical_json, prompt_prefix, run_loop, Budget, CancelFlag, Dispatcher, EchoTool, Message,
    RequestedTool, ScriptedProvider, ScriptedReply, StopReason, Tool, ToolRegistry,
};
use serde_json::json;

#[test]
fn unsorted_keys_become_sorted_and_the_prefix_is_byte_identical() {
    assert_eq!(
        canonical_json(r#"{"b":1,"a":2}"#).as_deref(),
        Some(r#"{"a":2,"b":1}"#)
    );
    let messages = vec![Message::assistant(
        "",
        vec![RequestedTool {
            id: "call-1".into(),
            name: "echo".into(),
            arguments: json!({"b": 1, "a": 2}),
            raw_arguments: Some(r#"{"b":1,"a":2}"#.into()),
        }],
    )];
    let once = prompt_prefix(&messages);
    let twice = prompt_prefix(&messages);
    assert_eq!(once.as_bytes(), twice.as_bytes());
    assert!(once.contains(r#"{"a":2,"b":1}"#));
    assert!(!once.contains(r#"{"b":1,"a":2}"#));
    assert_eq!(
        messages[0].tool_calls[0].raw_arguments.as_deref(),
        Some(r#"{"b":1,"a":2}"#)
    );
}

#[tokio::test]
async fn the_provider_sees_the_canonical_copy() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::raw_tool("call-1", "echo", r#"{"text":"hello","b":1}"#),
        ScriptedReply::text("done"),
    ]);
    provider.start_recording();
    let mut messages = vec![Message::user("go")];
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
    let seen = provider.seen();
    assert_eq!(seen.len(), 2);
    assert!(seen[1].contains(r#"{"b":1,"text":"hello"}"#));
    assert!(!seen[1].contains(r#"{"text":"hello","b":1}"#));
    let again = prompt_prefix(&messages);
    let once = prompt_prefix(&messages);
    assert_eq!(once.as_bytes(), again.as_bytes());
}
