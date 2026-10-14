use harness_core::{
    ChatCommand, ChatSession, Event, MockProvider, Outcome, Role, Usage, INBOX_BOUND,
};
use std::sync::Arc;

#[test]
fn parses_slash_commands() {
    assert_eq!(ChatCommand::parse("/help"), Some(ChatCommand::Help));
    assert_eq!(ChatCommand::parse("  /quit "), Some(ChatCommand::Quit));
    assert_eq!(
        ChatCommand::parse("/save chat.json"),
        Some(ChatCommand::Save("chat.json".into()))
    );
    assert_eq!(
        ChatCommand::parse("/save"),
        Some(ChatCommand::Unknown("save".into()))
    );
    assert_eq!(ChatCommand::parse("hello /help"), None);
}

#[tokio::test]
async fn multi_turn_chat_sends_full_history() {
    let mock = Arc::new(MockProvider::scripted([
        "Hi Daniel!",
        "You said your name is Daniel.",
    ]));
    let mut chat = ChatSession::new(mock.clone(), "Be friendly.", 4_000);

    assert_eq!(
        chat.handle_line("Hi, I'm Daniel").await,
        Outcome::Reply("Hi Daniel!".into())
    );
    assert_eq!(
        chat.handle_line("What's my name?").await,
        Outcome::Reply("You said your name is Daniel.".into())
    );

    let seen = mock.seen.lock().unwrap();
    let second_call = &seen[1];
    let roles: Vec<Role> = second_call.iter().map(|m| m.role).collect();
    assert_eq!(
        roles,
        [Role::System, Role::User, Role::Assistant, Role::User]
    );
    assert_eq!(second_call[0].content, "Be friendly.");
}

#[tokio::test]
async fn failed_call_rolls_back_the_user_turn() {
    let mock = Arc::new(MockProvider::scripted(["first"]).then_fail("overloaded"));
    let mut chat = ChatSession::new(mock.clone(), "sys", 4_000);
    chat.handle_line("one").await;
    let out = chat.handle_line("two").await;
    assert!(matches!(out, Outcome::Info(ref s) if s.contains("overloaded")));
    assert_eq!(
        chat.conversation.history().len(),
        2,
        "the failed user turn was removed"
    );
    // Retrying works because the history still alternates.
    assert!(matches!(
        chat.handle_line("two again").await,
        Outcome::Reply(_)
    ));
}

#[tokio::test]
async fn reset_and_quit() {
    let mut chat = ChatSession::new(MockProvider::new("ok"), "sys", 4_000);
    chat.handle_line("hello").await;
    assert_eq!(
        chat.handle_line("/reset").await,
        Outcome::Info("Conversation cleared.".into())
    );
    assert!(chat.conversation.history().is_empty());
    assert_eq!(chat.handle_line("/quit").await, Outcome::Quit);
}

#[tokio::test]
async fn a_full_inbox_does_not_call_the_provider() {
    let mock = Arc::new(MockProvider::new("ok"));
    let mut chat = ChatSession::new(mock.clone(), "sys", 4_000);
    for index in 0..INBOX_BOUND {
        chat.inbox.submit(format!("queued {index}")).unwrap();
    }
    let outcome = chat.handle_line("one more").await;
    assert!(matches!(outcome, Outcome::Info(ref text) if text.contains("full")));
    assert_eq!(mock.calls(), 0);
    assert!(chat.conversation.history().is_empty());
}

#[tokio::test]
async fn scripted_usage_lands_on_the_session_ledger() {
    let usage = Usage {
        prompt_tokens: 3,
        completion_tokens: 1,
        total_tokens: 4,
        cost: Some(0.25),
    };
    let mock = MockProvider::new("pong").with_usage("openai/gpt-4o-mini", usage);
    let mut chat = ChatSession::new(mock, "sys", 4_000);
    assert_eq!(
        chat.handle_line("ping").await,
        Outcome::Reply("pong".into())
    );
    assert!(matches!(
        chat.last_events.as_slice(),
        [
            Event::TextDelta(text),
            Event::Usage(usage),
            Event::Done { model: Some(model), .. },
        ] if text == "pong" && usage.cost == Some(0.25) && model == "openai/gpt-4o-mini"
    ));
    assert_eq!(chat.usage.calls, 1);
    assert_eq!(chat.usage.cost_usd, 0.25);
    assert!(chat.usage.cost_known);
    assert!(chat.inbox.is_empty());
}

#[tokio::test]
async fn mock_events_have_no_usage() {
    let mut chat = ChatSession::new(MockProvider::new("ok"), "sys", 4_000);
    chat.handle_line("hi").await;
    assert!(matches!(
        chat.last_events.as_slice(),
        [Event::TextDelta(text), Event::Done { model: None, .. }] if text == "ok"
    ));
    assert_eq!(chat.usage.calls, 0);
    assert!(!chat.usage.cost_known);
}
