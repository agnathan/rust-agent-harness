use harness_core::{ChatResponse, Message, OpenAiCompatible, Provider};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok_body(content: &str) -> serde_json::Value {
    serde_json::json!({
        "id": "chatcmpl-test",
        "choices": [{
            "index": 0,
            "message": { "role": "assistant", "content": content },
            "finish_reason": "stop"
        }]
    })
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

#[tokio::test]
async fn two_calls_share_the_client_against_a_local_server() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_body("pong")))
        .expect(2)
        .mount(&server)
        .await;

    let provider = OpenAiCompatible::new(&server.uri(), "test-key", "gpt-4o-mini");
    let messages = vec![Message::user("ping")];
    let first = provider.complete(&messages).await.unwrap();
    let second = provider.complete(&messages).await.unwrap();
    assert_eq!(first.content, "pong");
    assert_eq!(second.content, "pong");
}

#[tokio::test]
async fn http_error_is_classified() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(401).set_body_string("bad key"))
        .mount(&server)
        .await;

    let provider = OpenAiCompatible::new(&server.uri(), "nope", "gpt-4o-mini");
    let err = provider
        .complete(&[Message::user("ping")])
        .await
        .unwrap_err();
    let text = err.to_string();
    assert!(text.contains("401"), "{text}");
    assert!(text.contains("bad key"), "{text}");
}
