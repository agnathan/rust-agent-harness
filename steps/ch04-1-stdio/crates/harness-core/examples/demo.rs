use harness_core::{fixture_binary, McpSession};

#[tokio::main]
async fn main() {
    let session = McpSession::stdio(&fixture_binary()).await.expect("fixture");
    let tools = session.list_tools().await.expect("list");
    println!(
        "tools: {}",
        tools
            .iter()
            .map(|tool| tool.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let text = session
        .call_text("echo", serde_json::json!({"text": "hello"}))
        .await
        .expect("echo");
    println!("echo: {text}");
    session.shutdown().await.expect("shutdown");
}
