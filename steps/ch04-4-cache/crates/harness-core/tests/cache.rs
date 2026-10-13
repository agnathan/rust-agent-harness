use std::sync::Arc;

use harness_core::{fixture_binary, McpSession, ToolListCache};

#[tokio::test]
async fn a_hit_does_not_list_again_until_list_changed() {
    let session = Arc::new(McpSession::stdio(fixture_binary()).await.expect("connect"));
    let mut cache = ToolListCache::new(Arc::clone(&session));
    let first = cache.list().await.expect("cold");
    let second = cache.list().await.expect("hit");
    assert!(std::sync::Arc::ptr_eq(&first, &second));
    assert_eq!(cache.list_calls(), 1);
    assert!(first.iter().all(|tool| tool.name != "extra"));

    assert_eq!(
        session
            .call_text("grow", serde_json::json!({}))
            .await
            .unwrap(),
        "grew"
    );
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(2);
    let mut saw_extra = false;
    while tokio::time::Instant::now() < deadline {
        let listed = cache.list().await.expect("after");
        if listed.iter().any(|tool| tool.name == "extra") {
            saw_extra = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    assert!(saw_extra, "list_changed did not refresh the cache");
    assert!(cache.list_calls() >= 2, "calls {}", cache.list_calls());
}
