use std::sync::Arc;
use std::time::Instant;

use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, EchoTool, Provider, ScriptedProvider, ScriptedReply,
    StopReason, Tool, ToolRegistry, STALL_HALT_AFTER, STUB_MIN_CHARS,
};
use serde_json::json;

fn echo_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    registry
}

fn echo_script(text: &str, times: u32) -> Vec<ScriptedReply> {
    (0..times)
        .map(|index| ScriptedReply::tool(format!("call-{index}"), "echo", json!({"text": text})))
        .collect()
}

#[tokio::test]
async fn a_long_duplicate_is_stubbed_on_the_second_repeat_and_halts() {
    let payload = "x".repeat(STUB_MIN_CHARS);
    let provider = ScriptedProvider::new(echo_script(&payload, STALL_HALT_AFTER));
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &echo_registry(),
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop, StopReason::Stalled);
    assert_eq!(provider.calls(), u64::from(STALL_HALT_AFTER));
    let tools: Vec<_> = messages
        .iter()
        .filter(|message| message.tool_call_id.is_some())
        .map(|message| message.content.clone())
        .collect();
    assert_eq!(tools[0], payload);
    assert!(tools[1].contains("byte-identical"));
    assert!(!tools[1].contains(&payload));
    assert!(tools[2].contains("identical call repeated 3"));
}

#[tokio::test]
async fn an_alternating_cycle_halts_after_three_laps() {
    let mut replies = Vec::new();
    for index in 0..6 {
        let text = if index % 2 == 0 { "a" } else { "b" };
        replies.push(ScriptedReply::tool(
            format!("call-{index}"),
            "echo",
            json!({"text": text}),
        ));
    }
    replies.push(ScriptedReply::text("should not run"));
    let provider = ScriptedProvider::new(replies);
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &echo_registry(),
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop, StopReason::Stalled);
    assert_eq!(provider.calls(), 6);
    assert!(messages
        .last()
        .unwrap()
        .content
        .contains("tool-call cycle repeated 3 laps"));
}
