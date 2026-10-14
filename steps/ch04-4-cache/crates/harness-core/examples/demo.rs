use std::sync::Arc;

use harness_core::{fixture_binary, McpSession, ToolListCache};

#[tokio::main]
async fn main() {
    let session = Arc::new(McpSession::stdio(fixture_binary()).await.expect("fixture"));
    let mut cache = ToolListCache::new(Arc::clone(&session));
    let first = cache.list().await.expect("list");
    let _ = cache.list().await.expect("hit");
    println!(
        "tools: {}",
        first
            .iter()
            .map(|tool| tool.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("list_calls_after_two_reads={}", cache.list_calls());
    let echo = session
        .call_text("echo", serde_json::json!({"text": "hello"}))
        .await
        .expect("echo");
    println!("echo: {echo}");
}
