use harness_core::{
    missing_live_key, tool_definitions, EchoTool, Message, NoteTool, OpenAiCompatible, Tool,
    DEFAULT_MODEL,
};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn a_missing_key_is_the_established_string() {
    let error = missing_live_key(None, None).unwrap();
    assert_eq!(
        error,
        "missing OPENROUTER_API_KEY: pass --api-key, or run with --mock"
    );
    assert!(missing_live_key(Some("present"), None).is_none());
    assert!(missing_live_key(None, Some("present")).is_none());
    assert!(missing_live_key(Some(""), Some("")).is_some());
}

#[tokio::test]
async fn the_live_client_posts_echo_and_note_and_parses_a_tool_call() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "model": "openai/gpt-4o-mini",
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call-1",
                        "type": "function",
                        "function": { "name": "echo", "arguments": "{\"text\":\"hi\"}" }
                    }]
                }
            }],
            "usage": {
                "prompt_tokens": 3,
                "completion_tokens": 4,
                "total_tokens": 7,
                "cost": 0.25
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let echo = EchoTool.schema();
    let note = NoteTool.schema();
    let defs = tool_definitions(&[echo.clone(), note.clone()]);
    let names: Vec<_> = defs
        .iter()
        .map(|tool| tool["function"]["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, vec!["echo", "note"]);

    let client = OpenAiCompatible::new(&server.uri(), "test-key", DEFAULT_MODEL);
    let events = client
        .complete_with_tools(&[Message::user("hi")], &[echo, note])
        .await
        .unwrap();
    let call = events.iter().find_map(|event| match event {
        harness_core::Event::ToolCall {
            name, arguments, ..
        } => Some((name.clone(), arguments.clone())),
        _ => None,
    });
    assert_eq!(call.unwrap().0, "echo");

    let requests = server.received_requests().await.unwrap();
    let body: serde_json::Value = requests[0].body_json().unwrap();
    let posted: Vec<_> = body["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["function"]["name"].as_str().unwrap())
        .collect();
    assert_eq!(posted, vec!["echo", "note"]);
    assert_eq!(body["model"], "openai/gpt-4o-mini");
}
