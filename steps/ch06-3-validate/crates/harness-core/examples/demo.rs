use std::sync::Arc;

use harness_core::{
    Budget, CancelFlag, Dispatcher, EchoTool, ScriptedProvider, ScriptedReply, Tool, ToolRegistry,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "eco", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let mut messages = vec![harness_core::Message::user("say hello")];
    let outcome = harness_core::run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let assistant = messages
        .iter()
        .find(|message| !message.tool_calls.is_empty())
        .expect("assistant");
    println!("repaired: {}", assistant.tool_calls[0].name);
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .expect("tool");
    println!(
        "tool-result: {} {}",
        assistant.tool_calls[0].name, tool.content
    );
    println!("stop: {:?}", outcome.stop);
}
