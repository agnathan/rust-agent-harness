use std::sync::Arc;

use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, EchoTool, ScriptedProvider, ScriptedReply, Tool,
    ToolRegistry, STUB_MIN_CHARS,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let payload = "x".repeat(STUB_MIN_CHARS);
    let replies = (0..4)
        .map(|index| ScriptedReply::tool(format!("call-{index}"), "echo", json!({"text": payload})))
        .collect();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    let provider = ScriptedProvider::new(replies);
    let mut messages = vec![harness_core::Message::user("again")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let stub = messages
        .iter()
        .filter(|message| message.tool_call_id.is_some())
        .nth(1)
        .expect("second result");
    println!("stub: {}", stub.content.lines().next().unwrap_or(""));
    println!("stop: {:?}", outcome.stop);
}
