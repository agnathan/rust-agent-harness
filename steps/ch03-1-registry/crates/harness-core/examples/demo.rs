use harness_core::{EchoTool, Tool, ToolRegistry};
use serde_json::json;

/// Register one tool and call it. No network, no model.
#[tokio::main]
async fn main() {
    let mut registry = ToolRegistry::new();
    registry
        .register(std::sync::Arc::new(EchoTool))
        .expect("register");
    let schema = registry.schemas();
    println!("tools: {}", schema.len());
    println!(
        "schema: {}",
        serde_json::to_string(&schema[0].input_schema).expect("schema")
    );
    let tool = registry.get("echo").expect("echo");
    let result = tool.call(json!({"text": "hello"})).await.expect("call");
    println!("echo: {}", result.content[0].text);
}
