use std::sync::Arc;
use std::time::{Duration, Instant};

use harness_core::{Event, Message, MockProvider, Provider};
use tokio::task::JoinSet;

#[tokio::test]
async fn mock_provider_answers_and_records() {
    let mock = MockProvider::new("Hello, Daniel's harness!");
    let convo = vec![Message::system("Be brief."), Message::user("Say hi")];

    let reply = mock.complete(&convo).await.unwrap();

    assert_eq!(reply.content, "Hello, Daniel's harness!");
    let seen = mock.seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0][1].content, "Say hi");
}

#[tokio::test]
async fn mock_emits_text_then_done() {
    let mock = MockProvider::new("ok");
    let events = mock.complete_events(&[Message::user("hi")]).await.unwrap();
    assert!(matches!(&events[0], Event::TextDelta(text) if text == "ok"));
    assert!(matches!(
        &events[1],
        Event::Done { message, model: None } if message.content == "ok"
    ));
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].report_line(), "event: text-delta ok");
    assert_eq!(events[1].report_line(), "event: done");
}

#[tokio::test]
async fn mock_latency_is_configurable() {
    let mock = MockProvider::with_latency("ok", Duration::from_millis(40));
    let started = Instant::now();
    let reply = mock.complete(&[]).await.unwrap();
    assert_eq!(reply.content, "ok");
    assert!(started.elapsed() >= Duration::from_millis(40));
}

#[tokio::test]
async fn one_hundred_mock_calls_run_concurrently() {
    let mock = Arc::new(MockProvider::with_latency("ok", Duration::from_millis(80)));
    let messages = vec![Message::user("hello")];
    let started = Instant::now();
    let mut set = JoinSet::new();
    for _ in 0..100 {
        let mock = Arc::clone(&mock);
        let messages = messages.clone();
        set.spawn(async move { mock.complete(&messages).await.unwrap() });
    }
    let mut replies = 0;
    while let Some(joined) = set.join_next().await {
        assert_eq!(joined.expect("task").content, "ok");
        replies += 1;
    }
    let elapsed = started.elapsed();
    assert_eq!(replies, 100);
    assert_eq!(mock.seen.lock().unwrap().len(), 100);
    // Serial would be about 8 seconds. Concurrent stays near one wait.
    assert!(
        elapsed < Duration::from_millis(2_000),
        "elapsed {elapsed:?}"
    );
}
