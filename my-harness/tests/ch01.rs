use my_harness::provider::{ChatRequest, ChatResponse};
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

#[test]
fn response_deserializes_from_openai_shape() {
    let body = r#"{
        "id": "chatcmpl-123",
        "choices": [{ "index": 0,
                      "message": { "role": "assistant", "content": "Hi there" },
                      "finish_reason": "stop" }],
        "usage": { "prompt_tokens": 5, "completion_tokens": 2 }
    }"#;
    let parsed: ChatResponse = serde_json::from_str(body).unwrap();
    assert_eq!(parsed.choices[0].message, Message::assistant("Hi there"));
}
