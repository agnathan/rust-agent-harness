use std::sync::Arc;

use harness_core::{
    fixture_binary, register_remote, AgentSession, Budget, CancelFlag, Dispatcher, McpSession,
    ScriptedProvider, ScriptedReply, DISPATCH_BOUND,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let session_mcp = Arc::new(McpSession::stdio(fixture_binary()).await.expect("fixture"));
    let mut registry = harness_core::ToolRegistry::new();
    register_remote(&mut registry, session_mcp)
        .await
        .expect("register");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let mut session = AgentSession::new();
    session.inbox.submit("say hello").expect("submit");
    let outcome = session
        .turn_from_inbox(
            &provider,
            &registry,
            &Dispatcher::new(DISPATCH_BOUND),
            &Budget::default(),
            &CancelFlag::new(),
        )
        .await
        .expect("turn");
    for event in &outcome.events {
        println!("{}", event.report_line());
    }
    let tool = session
        .messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .expect("tool result");
    println!("tool-result: echo {}", tool.content);
    println!("stop: {:?}", outcome.stop);
}
