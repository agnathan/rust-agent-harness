use std::sync::Arc;

use harness_core::{
    run_loop, Budget, CancelFlag, Dispatcher, EchoTool, ScriptedProvider, ScriptedReply, Tool,
    ToolRegistry, Usage,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .expect("echo");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "partial"})).with_usage(Usage {
            prompt_tokens: 40,
            completion_tokens: 60,
            total_tokens: 100,
            cost: Some(0.25),
        }),
        ScriptedReply::text("budget answer"),
    ]);
    let budget = Budget {
        max_tokens: Some(50),
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
    println!("stop: {}", outcome.label());
    println!("text: {}", outcome.text());
}
