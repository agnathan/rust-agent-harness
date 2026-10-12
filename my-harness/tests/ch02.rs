use my_harness::{
    ChatCommand, ChatSession, Conversation, ConversationError, Message, MockProvider, Outcome, Role,
};
use std::sync::Arc;

#[test]
fn roles_must_alternate() {
    let mut conv = Conversation::new("Be brief.");
    conv.push(Message::user("hi")).unwrap();
    assert_eq!(
        conv.push(Message::user("hello?")),
        Err(ConversationError::OutOfOrder {
            expected: Role::Assistant,
            got: Role::User
        })
    );
    conv.push(Message::assistant("Hello!")).unwrap();
    assert_eq!(
        conv.push(Message::system("new rules")),
        Err(ConversationError::SystemInHistory)
    );
    assert_eq!(conv.next_role(), Role::User);
}

#[test]
fn system_prompt_stays_first_and_unchanged() {
    let mut conv = Conversation::new("You are a careful assistant.");
    for i in 0..3 {
        conv.push(Message::user(format!("q{i}"))).unwrap();
        conv.push(Message::assistant(format!("a{i}"))).unwrap();
    }
    conv.trim_to(5);
    conv.reset();
    let msgs = conv.to_messages();
    assert_eq!(msgs[0], Message::system("You are a careful assistant."));
    assert_eq!(msgs.len(), 1);
}

#[test]
fn trimming_drops_oldest_pairs_but_keeps_latest() {
    let mut conv = Conversation::new("sys");
    for i in 0..5 {
        conv.push(Message::user(format!(
            "question number {i} {}",
            "x".repeat(40)
        )))
        .unwrap();
        conv.push(Message::assistant(format!(
            "answer number {i} {}",
            "y".repeat(40)
        )))
        .unwrap();
    }
    conv.push(Message::user("latest")).unwrap();
    let before = conv.approx_tokens();
    let removed = conv.trim_to(40);
    assert!(removed > 0 && removed % 2 == 0);
    assert!(conv.approx_tokens() < before);
    assert_eq!(conv.history().last().unwrap().content, "latest");
    assert_eq!(
        conv.history()[0].role,
        Role::User,
        "still starts with a user turn"
    );
}

#[test]
fn save_and_load_round_trip() {
    let path = std::env::temp_dir().join(format!("conv-{}.json", std::process::id()));
    let mut conv = Conversation::new("sys");
    conv.push(Message::user("hi")).unwrap();
    conv.push(Message::assistant("hello")).unwrap();
    conv.save(&path).unwrap();
    assert_eq!(Conversation::load(&path).unwrap(), conv);

    std::fs::write(
        &path,
        r#"{"system":"s","history":[{"role":"assistant","content":"first?"}]}"#,
    )
    .unwrap();
    assert!(matches!(
        Conversation::load(&path),
        Err(ConversationError::OutOfOrder { .. })
    ));
    std::fs::remove_file(&path).ok();
}

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
