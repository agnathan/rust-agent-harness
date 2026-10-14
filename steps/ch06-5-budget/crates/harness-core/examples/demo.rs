use std::sync::Arc;

use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, EchoTool, ScriptedProvider, ScriptedReply, Tool,
    ToolRegistry,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "one"})),
        ScriptedReply::tool("call-2", "echo", json!({"text": "two"})),
        ScriptedReply::text("done"),
    ]);
    let budget = Budget {
        max_iterations: 4,
        warning_ratio: Some(0.5),
        ..Budget::default()
    };
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &budget,
        &CancelFlag::new(),
        std::time::Instant::now(),
    )
    .await;
    let newest = messages
        .iter()
        .rev()
        .find(|message| message.tool_call_id.is_some())
        .expect("tool");
    println!("{newest_content}", newest_content = newest.content);
    println!("stop: {:?}", outcome.stop);
}
