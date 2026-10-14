use std::sync::Arc;

use harness_core::{
    fixture_binary, register_remote, McpSession, StaticTool, ToolKind, ToolRegistry,
};

#[tokio::main]
async fn main() {
    let session = Arc::new(McpSession::stdio(fixture_binary()).await.expect("fixture"));
    let mut registry = ToolRegistry::new();
    registry
        .register(StaticTool::arc("ping", ToolKind::Read))
        .expect("ping");
    register_remote(&mut registry, Arc::clone(&session))
        .await
        .expect("register");
    let names: Vec<_> = registry
        .schemas()
        .into_iter()
        .map(|schema| schema.name)
        .collect();
    println!("tools: {}", names.join(", "));
    let text = registry
        .get("echo")
        .expect("echo")
        .call(serde_json::json!({"text": "hello"}))
        .await
        .expect("call");
    println!("echo: {}", text.content[0].text);
}
