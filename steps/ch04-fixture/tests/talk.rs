use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rmcp::model::CallToolRequestParams;
use rmcp::service::NotificationContext;
use rmcp::transport::{StreamableHttpClientTransport, TokioChildProcess};
use rmcp::{ClientHandler, RoleClient, ServiceExt};
use serde_json::json;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

fn fixture() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_mcp_fixture"))
}

fn arguments(value: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    value.as_object().expect("object").clone()
}

#[derive(Clone)]
struct Watch {
    changed: Arc<AtomicBool>,
}

impl ClientHandler for Watch {
    fn on_tool_list_changed(
        &self,
        _context: NotificationContext<RoleClient>,
    ) -> impl std::future::Future<Output = ()> + Send + '_ {
        self.changed.store(true, Ordering::SeqCst);
        std::future::ready(())
    }
}

async fn text_of(result: &rmcp::model::CallToolResult) -> String {
    result
        .content
        .first()
        .and_then(|block| block.as_text())
        .map(|text| text.text.clone())
        .unwrap_or_default()
}

#[tokio::test]
async fn stdio_lists_and_calls_echo() {
    let running =
        ().serve(TokioChildProcess::new(Command::new(fixture())).expect("spawn"))
            .await
            .expect("connect");
    let names: Vec<_> = running
        .list_all_tools()
        .await
        .expect("list")
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect();
    assert!(names.contains(&"echo".to_string()), "{names:?}");
    assert!(names.contains(&"note".to_string()), "{names:?}");
    assert!(!names.contains(&"extra".to_string()), "{names:?}");
    let echo = running
        .list_all_tools()
        .await
        .expect("list")
        .into_iter()
        .find(|tool| tool.name == "echo")
        .expect("echo");
    assert_eq!(
        echo.annotations.and_then(|note| note.read_only_hint),
        Some(true)
    );
    let result = running
        .call_tool(
            CallToolRequestParams::new("echo").with_arguments(arguments(json!({"text": "hi"}))),
        )
        .await
        .expect("call");
    assert_eq!(text_of(&result).await, "hi");
    let _ = running.cancel().await;
}

#[tokio::test]
async fn stdio_grow_sends_list_changed() {
    let changed = Arc::new(AtomicBool::new(false));
    let running = Watch {
        changed: Arc::clone(&changed),
    }
    .serve(TokioChildProcess::new(Command::new(fixture())).expect("spawn"))
    .await
    .expect("connect");
    let result = running
        .call_tool(CallToolRequestParams::new("grow"))
        .await
        .expect("grow");
    assert_eq!(text_of(&result).await, "grew");
    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
    while !changed.load(Ordering::SeqCst) && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert!(
        changed.load(Ordering::SeqCst),
        "list_changed did not arrive"
    );
    let names: Vec<_> = running
        .list_all_tools()
        .await
        .expect("list")
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect();
    assert!(names.contains(&"extra".to_string()), "{names:?}");
    let _ = running.cancel().await;
}

#[tokio::test]
async fn http_lists_and_calls_echo() {
    let mut child = Command::new(fixture())
        .arg("--http")
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("spawn");
    let stderr = child.stderr.take().expect("stderr");
    let mut lines = BufReader::new(stderr).lines();
    let line = tokio::time::timeout(Duration::from_secs(5), lines.next_line())
        .await
        .expect("timeout")
        .expect("read")
        .expect("line");
    let url = line.strip_prefix("MCP_URL=").expect("url").to_string();
    let running = ().serve(StreamableHttpClientTransport::from_uri(url)).await.expect("connect");
    let result = running
        .call_tool(
            CallToolRequestParams::new("echo").with_arguments(arguments(json!({"text": "web"}))),
        )
        .await
        .expect("call");
    assert_eq!(text_of(&result).await, "web");
    let _ = running.cancel().await;
    let _ = child.kill().await;
}
