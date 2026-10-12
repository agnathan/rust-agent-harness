use harness::l02_message::*;
use harness::l05_json::*;
use harness::l07_loop::ModelReply;
use harness::l08_openai::*;
use serde_json::json;
#[test]
fn config() {
    let c = OpenAiConfig::from_lookup(|k| match k {
        "OPENAI_API_KEY" => Some("sk".into()),
        "OPENAI_BASE_URL" => Some("http://localhost:11434/v1/".into()),
        _ => None,
    })
    .unwrap();
    assert_eq!(
        c,
        OpenAiConfig {
            base_url: "http://localhost:11434/v1".into(),
            api_key: "sk".into(),
            model: "gpt-4o-mini".into()
        }
    );
    assert_eq!(
        OpenAiConfig::from_lookup(|_| None).unwrap_err(),
        "OPENAI_API_KEY not set"
    );
}
#[test]
fn request_body() {
    let b = build_request_body("m", &[Message::user("hi")], &[]);
    assert_eq!(
        b,
        json!({"model":"m","messages":[{"role":"user","content":"hi"}]})
    );
    let spec = ToolSpec {
        name: "t".into(),
        description: "d".into(),
        parameters: json!({}),
    };
    let b2 = build_request_body("m", &[], &[spec.clone()]);
    assert_eq!(b2["tools"][0], spec.to_openai());
}
#[test]
fn parse_response() {
    let text = json!({"choices":[{"message":{"role":"assistant","content":"yo"}}]});
    assert_eq!(
        parse_chat_response(&text),
        Ok(ModelReply::Text("yo".into()))
    );
    let tools = json!({"choices":[{"message":{"role":"assistant","content":null,"tool_calls":[{"id":"c1","type":"function","function":{"name":"add","arguments":"{\"a\":1}"}}]}}]});
    assert_eq!(
        parse_chat_response(&tools),
        Ok(ModelReply::ToolCalls(vec![ToolCall {
            id: "c1".into(),
            name: "add".into(),
            arguments: "{\"a\":1}".into()
        }]))
    );
    assert_eq!(
        parse_chat_response(&json!({"error":{"message":"bad key"}})),
        Err("bad key".into())
    );
    assert_eq!(parse_chat_response(&json!({})), Err("no choices".into()));
}
