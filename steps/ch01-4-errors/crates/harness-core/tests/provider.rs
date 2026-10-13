use harness_core::{
    resolve_endpoint, ChatRequest, ChatResponse, EndpointInput, Message, OpenAiCompatible,
    Provider, ProviderError, DEFAULT_MODEL, OPENROUTER_BASE_URL,
};
use std::time::Duration;

use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok_body(content: &str) -> serde_json::Value {
    serde_json::json!({
        "id": "gen-test",
        "model": "openai/gpt-4o-mini",
        "choices": [{
            "index": 0,
            "message": { "role": "assistant", "content": content },
            "finish_reason": "stop"
        }],
        "usage": { "prompt_tokens": 5, "completion_tokens": 2, "total_tokens": 7 }
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
    assert_eq!(parsed.usage.unwrap().prompt_tokens, 5);
}

#[test]
fn openrouter_is_the_default_and_names_its_key() {
    let endpoint = resolve_endpoint(EndpointInput {
        base_url: OPENROUTER_BASE_URL,
        api_key_flag: None,
        openrouter_api_key: Some("sk-or-test"),
        openai_api_key: Some("sk-openai"),
        model: DEFAULT_MODEL,
        http_referer: Some("https://example.com"),
        title: Some("harness"),
    })
    .unwrap();
    assert_eq!(endpoint.base_url, OPENROUTER_BASE_URL);
    assert_eq!(endpoint.api_key, "sk-or-test");
    assert_eq!(endpoint.model, "openai/gpt-4o-mini");
    assert_eq!(
        endpoint.http_referer.as_deref(),
        Some("https://example.com")
    );
    assert_eq!(endpoint.title.as_deref(), Some("harness"));

    let missing = resolve_endpoint(EndpointInput {
        base_url: OPENROUTER_BASE_URL,
        api_key_flag: None,
        openrouter_api_key: None,
        openai_api_key: Some("sk-openai"),
        model: "anthropic/claude-3.5-sonnet",
        http_referer: None,
        title: None,
    })
    .unwrap_err();
    assert!(
        missing.to_string().contains("OPENROUTER_API_KEY"),
        "{missing}"
    );
}

#[test]
fn another_base_url_uses_the_openai_key() {
    let endpoint = resolve_endpoint(EndpointInput {
        base_url: "https://api.openai.com/v1",
        api_key_flag: None,
        openrouter_api_key: Some("sk-or-test"),
        openai_api_key: Some("sk-openai"),
        model: "gpt-4o-mini",
        http_referer: None,
        title: None,
    })
    .unwrap();
    assert_eq!(endpoint.api_key, "sk-openai");

    let flagged = resolve_endpoint(EndpointInput {
        base_url: "http://127.0.0.1:11434/v1",
        api_key_flag: Some("local"),
        openrouter_api_key: None,
        openai_api_key: None,
        model: "llama3.2",
        http_referer: None,
        title: None,
    })
    .unwrap();
    assert_eq!(flagged.api_key, "local");
    assert_eq!(flagged.model, "llama3.2");
}

#[test]
fn request_body_does_not_ask_for_a_stream() {
    let messages = [Message::user("ping")];
    let request = ChatRequest {
        model: DEFAULT_MODEL,
        messages: &messages,
    };
    let value = serde_json::to_value(&request).unwrap();
    assert!(value.get("stream").is_none());
    assert_eq!(value["model"], "openai/gpt-4o-mini");
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

    let provider = OpenAiCompatible::new(&server.uri(), "test-key", DEFAULT_MODEL);
    let messages = vec![Message::user("ping")];
    let first = provider.complete_detailed(&messages).await.unwrap();
    let second = provider.complete(&messages).await.unwrap();
    assert_eq!(first.message.content, "pong");
    assert_eq!(first.model.as_deref(), Some("openai/gpt-4o-mini"));
    assert_eq!(first.usage.unwrap().total_tokens, 7);
    assert_eq!(second.content, "pong");
}

#[tokio::test]
async fn openrouter_headers_are_sent_when_set() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .and(header("HTTP-Referer", "https://example.com"))
        .and(header("X-Title", "harness"))
        .and(body_partial_json(serde_json::json!({
            "model": "meta-llama/llama-3.1-8b-instruct"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_body("ok")))
        .expect(1)
        .mount(&server)
        .await;

    let endpoint = resolve_endpoint(EndpointInput {
        base_url: &server.uri(),
        api_key_flag: Some("test-key"),
        openrouter_api_key: None,
        openai_api_key: None,
        model: "meta-llama/llama-3.1-8b-instruct",
        http_referer: Some("https://example.com"),
        title: Some("harness"),
    })
    .unwrap();
    let provider = OpenAiCompatible::from_endpoint(&endpoint);
    let reply = provider.complete(&[Message::user("ping")]).await.unwrap();
    assert_eq!(reply.content, "ok");
}

#[tokio::test]
async fn empty_choices_are_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"model":"openai/gpt-4o-mini","choices":[]}"#),
        )
        .mount(&server)
        .await;

    let provider = OpenAiCompatible::new(&server.uri(), "test-key", DEFAULT_MODEL);
    let err = provider
        .complete(&[Message::user("ping")])
        .await
        .unwrap_err();
    assert!(matches!(err, ProviderError::EmptyResponse));
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

#[test]
fn missing_usage_still_keeps_the_answered_model() {
    let body = r#"{"model":"anthropic/claude-3.5-sonnet","choices":[{"message":{"role":"assistant","content":"hi"}}]}"#;
    let parsed: ChatResponse = serde_json::from_str(body).unwrap();
    assert_eq!(parsed.model.as_deref(), Some("anthropic/claude-3.5-sonnet"));
    assert!(parsed.usage.is_none());
    assert_eq!(parsed.choices[0].message.content, "hi");
}

#[tokio::test]
async fn slow_server_is_a_timeout() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(ok_body("late"))
                .set_delay(Duration::from_secs(2)),
        )
        .mount(&server)
        .await;

    let endpoint = resolve_endpoint(EndpointInput {
        base_url: &server.uri(),
        api_key_flag: Some("test-key"),
        openrouter_api_key: None,
        openai_api_key: None,
        model: DEFAULT_MODEL,
        http_referer: None,
        title: None,
    })
    .unwrap();
    let provider = OpenAiCompatible::with_timeout(&endpoint, Duration::from_millis(200));
    let err = provider
        .complete(&[Message::user("ping")])
        .await
        .unwrap_err();
    assert!(matches!(err, ProviderError::Timeout { .. }), "{err}");
}
