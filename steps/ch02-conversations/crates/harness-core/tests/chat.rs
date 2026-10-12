use harness_core::{ChatCommand, ChatSession, MockProvider, Outcome, Role};
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
