use std::sync::Arc;
use std::time::Instant;

use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, EchoTool, ScriptedProvider, ScriptedReply,
    StopReason, Tool, ToolRegistry, TOOL_OUTPUT_MAX_CHARS,
};
use serde_json::json;

#[tokio::test]
async fn a_long_tool_result_keeps_the_head_and_the_tail() {
    let payload = "y".repeat(2_000);
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": payload})),
        ScriptedReply::text("done"),
    ]);
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
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert!(tool.content.contains("OUTPUT TRUNCATED"));
    assert!(tool.content.contains("976 chars omitted"));
    assert!(tool.content.contains("out of 2,000 total"));
    assert!(tool.content.starts_with("y"));
    assert!(tool.content.ends_with("y"));
    assert!(tool.content.len() > TOOL_OUTPUT_MAX_CHARS);
}
