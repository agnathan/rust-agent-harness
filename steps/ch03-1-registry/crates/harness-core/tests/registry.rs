use harness_core::{
    check_arguments, EchoTool, StaticTool, Tool, ToolError, ToolKind, ToolRegistry,
};
use serde_json::json;

#[test]
fn echo_schema_is_mcp_shaped() {
    let schema = EchoTool.schema();
    assert_eq!(schema.name, "echo");
    assert!(!schema.description.is_empty());
    assert_eq!(schema.input_schema["type"], "object");
    assert_eq!(schema.input_schema["required"][0], "text");
    assert_eq!(EchoTool.kind(), ToolKind::Read);
}

#[tokio::test]
async fn echo_returns_the_text_argument() {
    let result = EchoTool.call(json!({"text": "hello"})).await.unwrap();
    assert_eq!(result.content[0].text, "hello");
    assert!(!result.is_error);
}

#[tokio::test]
async fn echo_rejects_a_missing_required_field() {
    let error = EchoTool.call(json!({})).await.unwrap_err();
    assert_eq!(
        error,
        ToolError::InvalidArgs {
            name: "echo".into(),
            reason: "missing text".into(),
        }
    );
}

#[test]
fn arguments_that_are_not_an_object_fail_when_the_schema_requires_keys() {
    let schema = EchoTool.schema();
    let error = check_arguments(&schema, &json!("nope")).unwrap_err();
    assert!(error.to_string().contains("JSON object"), "{error}");
}

#[test]
fn registry_lookup_and_sorted_schemas() {
    let mut registry = ToolRegistry::new();
    registry
        .register(StaticTool::arc("zeta", ToolKind::Read))
        .unwrap();
    registry
        .register(std::sync::Arc::new(EchoTool) as std::sync::Arc<dyn Tool>)
        .unwrap();
    registry
        .register(StaticTool::arc("alpha", ToolKind::Write))
        .unwrap();
    assert!(registry.get("missing").is_none());
    assert_eq!(registry.get("alpha").unwrap().kind(), ToolKind::Write);
    let names: Vec<_> = registry
        .schemas()
        .into_iter()
        .map(|schema| schema.name)
        .collect();
    assert_eq!(names, ["alpha", "echo", "zeta"]);
}

#[test]
fn a_second_tool_with_the_same_name_is_refused() {
    let mut registry = ToolRegistry::new();
    registry
        .register(StaticTool::arc("echo", ToolKind::Read))
        .unwrap();
    let error = registry
        .register(StaticTool::arc("echo", ToolKind::Read))
        .unwrap_err();
    assert_eq!(
        error,
        ToolError::Duplicate {
            name: "echo".into()
        }
    );
    assert_eq!(registry.len(), 1);
}
