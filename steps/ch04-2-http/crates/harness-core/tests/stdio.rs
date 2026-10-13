use harness_core::{fixture_binary, McpSession};

fn fixture() -> std::path::PathBuf {
    fixture_binary()
}

#[tokio::test]
async fn stdio_lists_echo_and_calls_it() {
    let session = McpSession::stdio(fixture()).await.expect("connect");
    let tools = session.list_tools().await.expect("list");
    let names: Vec<_> = tools.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(names, ["echo", "grow", "note"]);
    assert!(
        tools
            .iter()
            .find(|tool| tool.name == "echo")
            .unwrap()
            .read_only
    );
    assert!(
        !tools
            .iter()
            .find(|tool| tool.name == "note")
            .unwrap()
            .read_only
    );
    let text = session
        .call_text("echo", serde_json::json!({"text": "hello"}))
        .await
        .expect("call");
    assert_eq!(text, "hello");
    session.shutdown().await.expect("shutdown");
}

#[tokio::test]
async fn grow_sets_the_list_changed_flag_and_reveals_extra() {
    let session = McpSession::stdio(fixture()).await.expect("connect");
    assert!(!session.take_list_changed());
    let before = session.list_tools().await.expect("list");
    assert!(before.iter().all(|tool| tool.name != "extra"));
    assert_eq!(
        session
            .call_text("grow", serde_json::json!({}))
            .await
            .unwrap(),
        "grew"
    );
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(2);
    let mut saw = false;
    while tokio::time::Instant::now() < deadline {
        if session.take_list_changed() {
            saw = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    assert!(saw, "notifications/tools/list_changed did not arrive");
    let after = session.list_tools().await.expect("list");
    assert!(
        after.iter().any(|tool| tool.name == "extra"),
        "{:?}",
        after.iter().map(|tool| &tool.name).collect::<Vec<_>>()
    );
}
