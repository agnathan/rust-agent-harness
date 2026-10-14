use my_harness::provider::ChatResponse;
use my_harness::{Message, OpenAiCompatible, Provider};

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

#[tokio::test]
async fn openai_provider_constructs_without_a_real_key() {
    let provider = OpenAiCompatible::new("http://127.0.0.1:9/v1", "test", "gpt-4o-mini");
    let err = provider
        .complete(&[Message::user("ping")])
        .await
        .unwrap_err();
    let _ = err;
}
