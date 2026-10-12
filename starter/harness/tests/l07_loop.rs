use harness::l02_message::*;
use harness::l06_tools::*;
use harness::l07_loop::*;
fn reg() -> ToolRegistry {
    let mut r = ToolRegistry::new();
    r.register(Box::new(AddTool));
    r
}
fn tc(id: &str, name: &str, args: &str) -> ToolCall {
    ToolCall {
        id: id.into(),
        name: name.into(),
        arguments: args.into(),
    }
}
#[test]
fn text_only() {
    let mut m = ScriptedModel::new(vec![ModelReply::Text("hello".into())]);
    let out = run_agent_loop(&mut m, &reg(), "sys", "hi", 5).unwrap();
    assert_eq!(out.final_text, "hello");
    assert_eq!(out.api_calls, 1);
    assert_eq!(out.messages.len(), 3);
}
#[test]
fn tool_round_then_answer() {
    let mut m = ScriptedModel::new(vec![
        ModelReply::ToolCalls(vec![
            tc("c1", "add", "{\"a\":2,\"b\":2}"),
            tc("c2", "nope", "{}"),
            tc("c3", "add", "not json"),
        ]),
        ModelReply::Text("4".into()),
    ]);
    let out = run_agent_loop(&mut m, &reg(), "sys", "2+2?", 5).unwrap();
    assert_eq!(out.api_calls, 2);
    let tools: Vec<_> = out
        .messages
        .iter()
        .filter(|m| m.role == Role::Tool)
        .collect();
    assert_eq!(tools[0].content, "4");
    assert_eq!(tools[0].tool_call_id.as_deref(), Some("c1"));
    assert_eq!(tools[1].content, "error: unknown tool: nope");
    assert_eq!(tools[2].content, "error: invalid JSON arguments");
    assert_eq!(out.messages.last().unwrap().content, "4");
}
#[test]
fn budget_and_errors() {
    let looping = (0..5)
        .map(|i| ModelReply::ToolCalls(vec![tc(&i.to_string(), "add", "{\"a\":1,\"b\":1}")]))
        .collect();
    let mut m = ScriptedModel::new(looping);
    assert_eq!(
        run_agent_loop(&mut m, &reg(), "s", "u", 3).unwrap_err(),
        LoopError::BudgetExhausted
    );
    assert_eq!(m.calls, 3);
    let mut empty = ScriptedModel::new(vec![]);
    assert_eq!(
        run_agent_loop(&mut empty, &reg(), "s", "u", 3).unwrap_err(),
        LoopError::Model("script exhausted".into())
    );
}
