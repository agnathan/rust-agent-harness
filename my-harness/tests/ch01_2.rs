use std::time::Duration;

use my_harness::{Message, MockProvider, Provider, Role};

#[tokio::test]
async fn mock_provider_answers_and_records() {
    let mock = MockProvider::new("Hello, Daniel's harness!");
    let convo = vec![Message::system("Be brief."), Message::user("Say hi")];

    let reply = mock.complete(&convo).await.unwrap();

    assert_eq!(reply.role, Role::Assistant);
    assert_eq!(reply.content, "Hello, Daniel's harness!");
    let seen = mock.seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0][1].content, "Say hi");
}

#[tokio::test]
async fn mock_latency_is_configurable() {
    let mock = MockProvider::with_latency("ok", Duration::from_millis(20));
    let reply = mock.complete(&[]).await.unwrap();
    assert_eq!(reply.content, "ok");
}
