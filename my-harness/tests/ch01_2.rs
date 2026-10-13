use std::time::Duration;

use my_harness::{buffered_events, Message, MockProvider, Provider, Role};

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

#[test]
fn buffered_call_emits_text_then_done() {
    let events = buffered_events(&Message::assistant("ok"));
    assert!(events.len() >= 2, "a buffered reply emits text, then done");
}

#[tokio::test]
async fn mock_latency_is_configurable() {
    let mock = MockProvider::with_latency("ok", Duration::from_millis(20));
    let reply = mock.complete(&[]).await.unwrap();
    assert_eq!(reply.content, "ok");
}
