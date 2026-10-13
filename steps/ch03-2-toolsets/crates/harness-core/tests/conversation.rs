use harness_core::{Conversation, ConversationError, Message, Role};

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

    // A hand-edited file that breaks the rules is rejected.
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
