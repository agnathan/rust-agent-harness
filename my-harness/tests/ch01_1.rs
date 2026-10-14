use my_harness::provider::ChatRequest;
use my_harness::{Message, Role};

#[test]
fn message_serializes_to_openai_shape() {
    let message = Message::user("hi");
    let json = serde_json::to_value(&message).unwrap();
    assert_eq!(json, serde_json::json!({ "role": "user", "content": "hi" }));
    assert_eq!(message.role, Role::User);
}

#[test]
fn request_serializes_to_openai_shape() {
    let msgs = [Message::user("hi")];
    let req = ChatRequest {
        model: "gpt-4o-mini",
        messages: &msgs,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [{ "role": "user", "content": "hi" }]
        })
    );
}
