use harness_core::{fixture_binary, HttpFixture, McpSession};

#[tokio::test]
async fn streamable_http_calls_echo_on_localhost() {
    let fixture = HttpFixture::spawn(fixture_binary()).await.expect("http");
    let session = McpSession::streamable_http(&fixture.url)
        .await
        .expect("connect");
    let tools = session.list_tools().await.expect("list");
    assert!(tools.iter().any(|tool| tool.name == "echo"));
    let text = session
        .call_text("echo", serde_json::json!({"text": "web"}))
        .await
        .expect("echo");
    assert_eq!(text, "web");
    session.shutdown().await.expect("shutdown");
}
