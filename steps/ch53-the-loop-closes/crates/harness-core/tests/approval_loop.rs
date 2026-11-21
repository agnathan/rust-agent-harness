use std::time::Instant;

use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, Message, ScriptedProvider, ScriptedReply, StaticTool,
    StopReason, ToolKind, ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

#[tokio::test]
async fn a_hardline_terminal_command_never_reaches_the_tool() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "terminal", json!({"command": "rm -rf /"})),
        ScriptedReply::text("stopped"),
    ]);
    let mut registry = ToolRegistry::new();
    registry
        .register(StaticTool::arc("terminal", ToolKind::Write))
        .unwrap();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let mut messages = vec![Message::user("clean the disk")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .expect("tool result");
    assert!(
        tool.content.contains("BLOCKED"),
        "the denial is the tool result, not the tool body: {}",
        tool.content
    );
    assert_ne!(tool.content, "terminal");
    assert_eq!(outcome.stop(), StopReason::Finished);
}
