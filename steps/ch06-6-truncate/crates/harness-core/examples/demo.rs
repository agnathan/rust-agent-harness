use std::sync::Arc;

use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, EchoTool, ScriptedProvider, ScriptedReply, Tool,
    ToolRegistry,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let payload = "y".repeat(2_000);
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .expect("echo");
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
        std::time::Instant::now(),
    )
    .await;
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .expect("tool");
    let notice = tool
        .content
        .lines()
        .find(|line| line.contains("TRUNCATED"))
        .unwrap_or("");
    println!("{notice}");
    println!("stop: {:?}", outcome.stop);
}
