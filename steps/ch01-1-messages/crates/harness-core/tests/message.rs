use harness_core::{ChatRequest, Message, OwnedChatRequest, Role};

#[test]
fn message_serializes_to_openai_shape() {
    let message = Message::user("hi");
    let json = serde_json::to_value(&message).unwrap();
    assert_eq!(json, serde_json::json!({ "role": "user", "content": "hi" }));
    assert_eq!(message.role, Role::User);
}

#[test]
fn request_round_trips_fifty_messages() {
    let messages: Vec<Message> = (0..50)
        .map(|index| {
            if index % 2 == 0 {
                Message::user(format!("question {index}"))
            } else {
                Message::assistant(format!("answer {index}"))
            }
        })
        .collect();
    let request = ChatRequest {
        model: "gpt-4o-mini",
        messages: &messages,
    };
    let bytes = serde_json::to_vec(&request).unwrap();
    let parsed: OwnedChatRequest = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(parsed.model, "gpt-4o-mini");
    assert_eq!(parsed.messages, messages);
    assert_eq!(parsed.messages[0].role, Role::User);
    assert_eq!(parsed.messages[1].role, Role::Assistant);
}
