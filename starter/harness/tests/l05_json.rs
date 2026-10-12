use harness::l02_message::*;
use harness::l05_json::*;
use serde_json::json;
#[test]
fn user_message_json() {
    let w = WireMessage::from(&Message::user("hi"));
    assert_eq!(
        serde_json::to_value(&w).unwrap(),
        json!({"role":"user","content":"hi"})
    );
}
#[test]
fn tool_call_json() {
    let m = Message::assistant_tool_calls(vec![ToolCall {
        id: "c1".into(),
        name: "add".into(),
        arguments: "{\"a\":1}".into(),
    }]);
    let v = serde_json::to_value(WireMessage::from(&m)).unwrap();
    assert_eq!(
        v,
        json!({"role":"assistant","tool_calls":[{"id":"c1","type":"function","function":{"name":"add","arguments":"{\"a\":1}"}}]})
    );
    let t = serde_json::to_value(WireMessage::from(&Message::tool_result("c1", "2"))).unwrap();
    assert_eq!(t, json!({"role":"tool","content":"2","tool_call_id":"c1"}));
}
#[test]
fn roundtrip() {
    let m = Message::assistant_tool_calls(vec![ToolCall {
        id: "c1".into(),
        name: "add".into(),
        arguments: "{}".into(),
    }]);
    let s = serde_json::to_string(&WireMessage::from(&m)).unwrap();
    let back: WireMessage = serde_json::from_str(&s).unwrap();
    assert_eq!(back.to_message().unwrap(), m);
    let bad = WireMessage {
        role: "robot".into(),
        content: None,
        tool_calls: None,
        tool_call_id: None,
    };
    assert_eq!(bad.to_message().unwrap_err(), "unknown role: robot");
}
#[test]
fn schema() {
    let s = object_schema(&[("a", "number", "first")], &["a"]);
    assert_eq!(
        s,
        json!({"type":"object","properties":{"a":{"type":"number","description":"first"}},"required":["a"]})
    );
    let spec = ToolSpec {
        name: "add".into(),
        description: "Add".into(),
        parameters: s.clone(),
    };
    assert_eq!(
        spec.to_openai(),
        json!({"type":"function","function":{"name":"add","description":"Add","parameters":s}})
    );
}
#[test]
fn args() {
    let a: AddArgs = parse_args("{\"a\":1.5,\"b\":2}").unwrap();
    assert_eq!(a, AddArgs { a: 1.5, b: 2.0 });
    assert!(parse_args::<AddArgs>("{\"a\":1}").is_err());
}
