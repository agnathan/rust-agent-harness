use harness_core::{fixture_binary, HttpFixture, McpSession};

#[tokio::main]
async fn main() {
    let fixture = HttpFixture::spawn(fixture_binary()).await.expect("http");
    let session = McpSession::streamable_http(&fixture.url)
        .await
        .expect("connect");
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
    println!("url: {}", fixture.url);
    session.shutdown().await.expect("shutdown");
}
