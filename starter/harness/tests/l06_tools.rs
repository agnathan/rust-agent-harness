use harness::l06_tools::*;
use serde_json::json;
#[test]
fn echo_and_add() {
    assert_eq!(EchoTool.call(&json!({"text":"hi"})), Ok("hi".into()));
    assert_eq!(
        EchoTool.call(&json!({})),
        Err(ToolError::BadArgs("text".into()))
    );
    assert_eq!(AddTool.call(&json!({"a":2,"b":3})), Ok("5".into()));
    assert_eq!(
        AddTool.call(&json!({"a":2})),
        Err(ToolError::BadArgs("b".into()))
    );
    assert_eq!(AddTool.name(), "add");
}
#[test]
fn registry() {
    let mut r = ToolRegistry::new();
    assert!(r.register(Box::new(EchoTool)));
    assert!(r.register(Box::new(AddTool)));
    assert!(!r.register(Box::new(AddTool)), "duplicates rejected");
    assert_eq!(r.len(), 2);
    assert_eq!(
        r.specs()
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        vec!["add", "echo"]
    );
    assert_eq!(r.dispatch("add", &json!({"a":1,"b":1})), Ok("2".into()));
    assert_eq!(
        r.dispatch("nope", &json!({})),
        Err(ToolError::NotFound("nope".into()))
    );
}
