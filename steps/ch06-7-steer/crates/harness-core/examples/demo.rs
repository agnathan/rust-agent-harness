use std::sync::Arc;

use harness_core::{
    run_loop_with_inbox, Budget, CancelFlag, Dispatcher, ScriptedProvider, ScriptedReply,
    SessionInbox, Tool, ToolError, ToolKind, ToolRegistry, ToolResult, ToolSchema,
};
use serde_json::{json, Value};

struct SteerTool {
    inbox: SessionInbox,
}

#[async_trait::async_trait]
impl Tool for SteerTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "steer_tool".into(),
            description: "Queues a steer.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        self.inbox.submit("use the short form").expect("steer");
        Ok(ToolResult::text("looked"))
    }
}

#[tokio::main]
async fn main() {
    let inbox = SessionInbox::default();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SteerTool {
            inbox: inbox.clone(),
        }) as Arc<dyn Tool>)
        .expect("register");
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
        &CancelFlag::new(),
        std::time::Instant::now(),
        Some(&inbox),
    )
    .await;
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .expect("tool");
    println!("{}", tool.content);
    println!("stop: {:?}", outcome.stop);
}
