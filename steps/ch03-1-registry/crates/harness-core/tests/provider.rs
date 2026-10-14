use harness_core::{
    events_for_reply, resolve_endpoint, AgentUsage, ChatRequest, ChatResponse, EndpointInput,
    Event, Message, MockProvider, OpenAiCompatible, Provider, Role, Usage, DEFAULT_MODEL,
    OPENROUTER_BASE_URL,
};
use std::time::{Duration, Instant};

use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

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

#[tokio::test]
async fn mock_latency_is_configurable() {
    let mock = MockProvider::with_latency("ok", Duration::from_millis(40));
    let started = Instant::now();
    let reply = mock.complete(&[]).await.unwrap();
    assert_eq!(reply.content, "ok");
    assert!(started.elapsed() >= Duration::from_millis(40));
}

#[test]
fn request_serializes_to_openai_shape() {
    let msgs = [Message::user("hi")];
    let req = ChatRequest {
        model: DEFAULT_MODEL,
        messages: &msgs,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert!(json.get("stream").is_none());
    assert_eq!(
        json,
        serde_json::json!({
            "model": "openai/gpt-4o-mini",
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
    let usage = parsed.usage.unwrap();
    assert_eq!(usage.prompt_tokens, 5);
    assert_eq!(usage.cost, None);
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

    let missing = resolve_endpoint(EndpointInput {
        base_url: "",
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
        base_url: "https://api.openai.com/v1/",
        api_key_flag: None,
        openrouter_api_key: Some("sk-or-test"),
        openai_api_key: Some("sk-openai"),
        model: "gpt-4o-mini",
        http_referer: None,
        title: None,
    })
    .unwrap();
    assert_eq!(endpoint.base_url, "https://api.openai.com/v1");
    assert_eq!(endpoint.api_key, "sk-openai");
}

#[tokio::test]
async fn mock_events_are_text_then_done() {
    let mock = MockProvider::new("hello");
    let events = mock.complete_events(&[Message::user("hi")]).await.unwrap();
    assert!(matches!(
        events.as_slice(),
        [Event::TextDelta(text), Event::Done { model: None, .. }] if text == "hello"
    ));
}

#[test]
fn scripted_cost_accumulates_without_a_network() {
    let message = Message::assistant("pong");
    let usage = Usage {
        prompt_tokens: 5,
        completion_tokens: 2,
        total_tokens: 7,
        cost: Some(0.25),
    };
    let events = events_for_reply(&message, Some(&usage), Some(DEFAULT_MODEL));
    assert!(matches!(&events[1], Event::Usage(seen) if seen.cost == Some(0.25)));
    let mut ledger = AgentUsage::default();
    ledger.record(&usage);
    ledger.record(&Usage {
        prompt_tokens: 1,
        completion_tokens: 1,
        total_tokens: 2,
        cost: None,
    });
    assert_eq!(ledger.calls, 2);
    assert_eq!(ledger.cost_usd, 0.25);
    assert!(ledger.cost_known);
}

#[tokio::test]
async fn local_fake_reports_cost_and_the_answered_model() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "model": "openai/gpt-4o-mini",
            "choices": [{
                "message": { "role": "assistant", "content": "pong" }
            }],
            "usage": { "prompt_tokens": 5, "completion_tokens": 2, "total_tokens": 7, "cost": 0.25 }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenAiCompatible::new(&server.uri(), "test-key", DEFAULT_MODEL);
    let events = provider
        .complete_events(&[Message::user("ping")])
        .await
        .unwrap();
    assert!(matches!(&events[0], Event::TextDelta(text) if text == "pong"));
    assert!(matches!(&events[1], Event::Usage(usage) if usage.cost == Some(0.25)));
    assert!(
        matches!(&events[2], Event::Done { model: Some(model), .. } if model == "openai/gpt-4o-mini")
    );
}
