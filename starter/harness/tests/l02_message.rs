use harness::l02_message::*;
fn call(id: &str) -> ToolCall {
    ToolCall {
        id: id.into(),
        name: "echo".into(),
        arguments: "{}".into(),
    }
}
#[test]
fn role_strings() {
    assert_eq!(Role::Tool.as_str(), "tool");
    assert_eq!(Role::Assistant.as_str(), "assistant");
}
#[test]
fn constructors() {
    let m = Message::user("hi");
    assert_eq!(m.role, Role::User);
    assert_eq!(m.content, "hi");
    assert!(!m.has_tool_calls());
    let t = Message::tool_result("c1", "ok");
    assert_eq!(t.tool_call_id.as_deref(), Some("c1"));
    assert_eq!(t.role, Role::Tool);
    let a = Message::assistant_tool_calls(vec![call("c1")]);
    assert!(a.has_tool_calls());
    assert_eq!(a.content, "");
    assert_eq!(Message::system("s").role, Role::System);
}
#[test]
fn conversation() {
    let mut c = Conversation::new("be helpful");
    c.push(Message::user("q"));
    c.push(Message::assistant("a1"));
    c.push(Message::user("q2"));
    c.push(Message::assistant("a2"));
    assert_eq!(c.last_assistant().unwrap().content, "a2");
    assert_eq!(c.count_role(&Role::User), 2);
    assert!(c.roles_alternate());
}
#[test]
fn alternation_rules() {
    let mut c = Conversation::new("s");
    c.push(Message::user("q"));
    c.push(Message::assistant_tool_calls(vec![call("1"), call("2")]));
    c.push(Message::tool_result("1", "x"));
    c.push(Message::tool_result("2", "y"));
    assert!(c.roles_alternate(), "consecutive tool results are allowed");
    c.push(Message::user("u"));
    c.push(Message::user("u2"));
    assert!(!c.roles_alternate());
}
