use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use harness_core::approval::Session;
use harness_core::local_tools::{
    child_may_call, run_terminal, truncate_terminal, ClarifyTool, BLOCKED_CHILD_TOOLS,
};
use harness_core::{
    AgentSession, Budget, CancelFlag, Dispatcher, ScriptedProvider, ScriptedReply, StopReason,
    Tool, ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

#[test]
fn a_hardline_terminal_command_does_not_reach_the_worker() {
    let mut session = Session::new();
    let missing = Path::new("/no/such/sandbox-worker");
    let output = run_terminal(&mut session, "rm -rf /", Duration::from_secs(1), missing);
    assert!(output.denied);
    assert!(output.text.contains("BLOCKED"));
    assert!(!output.text.contains("echo hello ran"));
}

#[test]
fn an_allowed_command_is_the_worker_line_and_long_output_is_cut() {
    let mut session = Session::new();
    let worker = Path::new(env!("CARGO_BIN_EXE_sandbox-worker"));
    let output = run_terminal(&mut session, "echo hello", Duration::from_secs(2), worker);
    assert!(!output.denied, "{}", output.text);
    assert!(output.text.contains("ok echo hello"), "{}", output.text);
    let mut session = Session::new();
    let timed = run_terminal(
        &mut session,
        "sleep-ms 5000",
        Duration::from_millis(200),
        worker,
    );
    assert!(!timed.denied);
    assert_eq!(timed.text, "timeout");
    let long = "x".repeat(60_000);
    let cut = truncate_terminal(&long);
    assert!(cut.chars().count() < 3_000, "{}", cut.chars().count());
    assert!(cut.contains('…'));
    let wide = "y".repeat(100);
    let many = vec![wide.as_str(); 600].join("\n");
    let cut = truncate_terminal(&many);
    assert!(cut.contains("bytes omitted"), "{}", cut.len());
}

#[tokio::test]
async fn the_parent_loop_pauses_for_the_fake_user_and_a_child_does_not() {
    assert!(child_may_call("clarify", 1).is_err());
    assert!(BLOCKED_CHILD_TOOLS.contains(&"clarify"));
    let child = ClarifyTool::new(1, vec!["should-not-be-read".into()]);
    let err = child
        .call(json!({"questions": [{"text": "which?", "choices": ["left"]}]}))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("child agent may not call clarify"));
    assert_eq!(child.asked(), 0);
    assert_eq!(child.pending(), 1);

    let parent = Arc::new(ClarifyTool::new(0, vec!["the left one".into()]));
    let mut registry = ToolRegistry::new();
    registry.register(parent.clone() as Arc<dyn Tool>).unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool(
            "call-1",
            "clarify",
            json!({"questions": [{"text": "which side?", "choices": ["left", "right"]}]}),
        ),
        ScriptedReply::text("done"),
    ]);
    let mut session = AgentSession::new();
    session.inbox.submit("ask").unwrap();
    let outcome = session
        .turn_from_inbox(
            &provider,
            &registry,
            &Dispatcher::new(DISPATCH_BOUND),
            &Budget::default(),
            &CancelFlag::new(),
        )
        .await
        .unwrap();
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(parent.asked(), 1);
    assert_eq!(parent.pending(), 0);
    let tool_text = session
        .messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .map(|message| message.content.clone())
        .unwrap_or_default();
    assert!(tool_text.contains("the left one"));
}
