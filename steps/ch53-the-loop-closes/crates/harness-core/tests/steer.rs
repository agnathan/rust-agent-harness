use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use harness_core::{
    run_loop_with_inbox, Budget, CancelFlag, Dispatcher, ScriptedProvider, ScriptedReply,
    SessionInbox, StopReason, Tool, ToolError, ToolKind, ToolRegistry, ToolResult, ToolSchema,
};
use serde_json::{json, Value};

struct SteerTool {
    inbox: SessionInbox,
    cancel: CancelFlag,
    hard: bool,
}

#[async_trait]
impl Tool for SteerTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "steer_tool".into(),
            description: "Queues a steer from inside the call.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        self.inbox.submit("use the short form").expect("steer");
        if self.hard {
            self.cancel.hard_stop();
        }
        Ok(ToolResult::text("looked"))
    }
}

#[tokio::test]
async fn a_steer_is_appended_to_the_latest_tool_result() {
    let inbox = SessionInbox::default();
    let cancel = CancelFlag::new();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SteerTool {
            inbox: inbox.clone(),
            cancel: cancel.clone(),
            hard: false,
        }) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "steer_tool", json!({})),
        ScriptedReply::text("done"),
    ]);
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop_with_inbox(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &cancel,
        Instant::now(),
        Some(&inbox),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert!(tool.content.starts_with("looked"));
    assert!(tool.content.contains("\n\nuse the short form"));
    assert!(inbox.is_empty());
}

#[tokio::test]
async fn a_hard_stop_drops_the_pending_steer_instead_of_appending_it() {
    let inbox = SessionInbox::default();
    let cancel = CancelFlag::new();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SteerTool {
            inbox: inbox.clone(),
            cancel: cancel.clone(),
            hard: true,
        }) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "steer_tool", json!({})),
        ScriptedReply::text("should not run"),
    ]);
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop_with_inbox(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &cancel,
        Instant::now(),
        Some(&inbox),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Cancelled);
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert_eq!(tool.content, "looked");
    assert!(inbox.is_empty());
}
