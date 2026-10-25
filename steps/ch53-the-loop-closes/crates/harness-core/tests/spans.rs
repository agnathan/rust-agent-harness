use harness_core::spans::{
    demo, mock_turn, research_team, safe_field, CountingSubscriber, FakeModel, Memory,
};

#[test]
fn a_turn_records_three_event_names_and_not_the_prompt() {
    let subscriber = CountingSubscriber::new();
    let watched = subscriber.clone();
    let mut sink = Vec::new();
    let secret = "sk-live-0123456789abcdef";
    let reply = tracing::subscriber::with_default(subscriber, || {
        mock_turn(&FakeModel { reply: "ok" }, secret, &mut sink)
    });
    assert_eq!(reply, "ok");
    assert_eq!(sink, vec!["request", "response", "gateway"]);
    assert!(watched.spans_opened() >= 3);
    assert_eq!(safe_field(secret), "request");
    assert!(!format!("{sink:?}").contains("sk-"));
}

#[test]
fn one_hundred_agents_learn_across_two_runs() {
    let memory = Memory::new();
    let first = research_team(100, &memory, &FakeModel { reply: "found" });
    assert_eq!(first, 100);
    assert_eq!(memory.len(), 100);
    let second = research_team(1, &memory, &FakeModel { reply: "kept" });
    assert_eq!(second, 101);
    assert!(memory.last().unwrap().ends_with("kept"));
}

#[test]
fn demo_names_the_team() {
    let line = demo();
    assert!(line.starts_with("trace: ok events=3 "), "{line}");
    assert!(line.contains("team=101"), "{line}");
}
