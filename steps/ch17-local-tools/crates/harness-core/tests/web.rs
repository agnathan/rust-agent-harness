use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;

use harness_core::web::{
    extract_page, is_safe_url, EnvSearch, FakeSearch, MapResolver, SearchBackend, WebExtractTool,
    WebSearchTool, WEB_SEARCH_API_KEY,
};
use harness_core::{
    AgentSession, Budget, CancelFlag, Dispatcher, ScriptedProvider, ScriptedReply, StopReason,
    Tool, ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

fn resolver_for(host: &str, ip: &str) -> MapResolver {
    let mut hosts = HashMap::new();
    hosts.insert(host.into(), Ok(vec![ip.parse::<IpAddr>().unwrap()]));
    MapResolver::new(hosts)
}

#[tokio::test]
async fn the_loop_runs_web_search_on_the_fake_backend() {
    let backend = Arc::new(FakeSearch::example());
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(WebSearchTool::new(backend)) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "web_search", json!({"query": "rust", "limit": 1})),
        ScriptedReply::text("done"),
    ]);
    let mut session = AgentSession::new();
    session.inbox.submit("search").unwrap();
    let outcome = session
        .turn_from_inbox(
            &provider,
            &registry,
            &Dispatcher::new(DISPATCH_BOUND),
            &Budget::default(),
            &CancelFlag::new(),
        )
        .await
        .unwrap();
    assert_eq!(outcome.stop(), StopReason::Finished);
    let tool_text = session
        .messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .map(|message| message.content.clone())
        .unwrap_or_default();
    assert!(tool_text.contains("example.com"));
    assert!(tool_text.contains("untrusted_tool_result"));
    assert_eq!(WebSearchTool::schema_value().name, "web_search");
}

#[test]
fn a_private_ip_is_blocked_unless_allowed_and_metadata_never_is() {
    let resolver = resolver_for("files.example", "10.1.2.3");
    assert!(is_safe_url("https://files.example/a", false, &resolver).is_err());
    assert!(is_safe_url("https://files.example/a", true, &resolver).is_ok());
    assert!(is_safe_url(
        "http://169.254.1.1/",
        true,
        &MapResolver::new(HashMap::new())
    )
    .is_err());
}

#[test]
fn extract_refuses_a_binary_body_and_a_missing_key_does_not_dial() {
    let mut pages = HashMap::new();
    pages.insert("https://example.com/bin".into(), "%PDF-1.7 binary".into());
    let mut hosts = HashMap::new();
    hosts.insert(
        "example.com".into(),
        Ok(vec!["93.184.216.34".parse::<IpAddr>().unwrap()]),
    );
    let err = extract_page(
        &FakeSearch::new(Vec::new(), pages),
        "https://example.com/bin",
        None,
        false,
        &MapResolver::new(hosts),
    )
    .unwrap_err();
    assert!(err.to_string().contains("binary body"));
    std::env::remove_var(WEB_SEARCH_API_KEY);
    std::env::remove_var("WEB_SEARCH_ALLOW_NETWORK");
    let env = EnvSearch::from_env();
    assert!(!env.key_is_set());
    assert!(env
        .search("rust", 5)
        .unwrap_err()
        .to_string()
        .contains("not set"));
    let _tool = WebExtractTool::schema_value();
}
