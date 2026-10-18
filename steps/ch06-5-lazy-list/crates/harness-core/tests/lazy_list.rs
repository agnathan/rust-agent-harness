use std::sync::Arc;

use harness_core::{
    resolve_endpoint, AgentView, EndpointInput, ListCache, ProviderError, DEFAULT_MODEL,
    OPENROUTER_BASE_URL,
};

#[test]
fn a_cache_hit_does_not_list_again_until_the_notification() {
    let mut cache = ListCache::new(1_000);
    let first = cache.list();
    let second = cache.list();
    assert_eq!(cache.list_calls(), 1);
    assert!(Arc::ptr_eq(&first, &second));
    cache.notify_list_changed();
    let third = cache.list();
    assert_eq!(cache.list_calls(), 2);
    assert!(!Arc::ptr_eq(&first, &third));
    assert_eq!(first.len(), third.len());
}

#[test]
fn a_real_tools_list_uses_the_same_cache() {
    use harness_core::{cards_from_listed, ListedTool};
    let listed = vec![ListedTool {
        name: "echo".into(),
        description: "Return the text argument unchanged.".into(),
        input_schema: serde_json::json!({"type": "object"}),
    }];
    let mut cache = ListCache::from_cards(cards_from_listed("fixture", &listed));
    let first = cache.list();
    let second = cache.list();
    assert_eq!(cache.list_calls(), 1);
    assert!(Arc::ptr_eq(&first, &second));
    assert_eq!(first[0].name, "echo");
    assert_eq!(first[0].server, "fixture");
    cache.replace_cards(cards_from_listed(
        "fixture",
        &[
            ListedTool {
                name: "echo".into(),
                description: "Return the text argument unchanged.".into(),
                input_schema: serde_json::json!({"type": "object"}),
            },
            ListedTool {
                name: "extra".into(),
                description: "An extra tool, hidden until grow.".into(),
                input_schema: serde_json::json!({"type": "object"}),
            },
        ],
    ));
    let third = cache.list();
    assert_eq!(cache.list_calls(), 2);
    assert_eq!(third.len(), 2);
    assert!(!Arc::ptr_eq(&first, &third));
}

#[test]
fn two_agents_see_different_servers_from_one_cache() {
    let mut cache = ListCache::new(256);
    let cards = cache.list();
    let files = AgentView::new("files-agent", ["files"]);
    let git = AgentView::new("git-agent", ["git"]);
    let files_tools = files.filter(&cards);
    let git_tools = git.filter(&cards);
    assert!(files_tools.iter().all(|card| card.server == "files"));
    assert!(git_tools.iter().all(|card| card.server == "git"));
    assert!(!files_tools.is_empty());
    assert!(!git_tools.is_empty());
    assert_ne!(files_tools[0].name, git_tools[0].name);
}

#[test]
fn openrouter_without_a_key_is_missing_key() {
    let error = resolve_endpoint(EndpointInput {
        base_url: OPENROUTER_BASE_URL,
        api_key_flag: None,
        openrouter_api_key: None,
        openai_api_key: None,
        model: DEFAULT_MODEL,
        http_referer: None,
        title: None,
    })
    .unwrap_err();
    match error {
        ProviderError::MissingKey { name } => assert_eq!(name, "OPENROUTER_API_KEY"),
        other => panic!("unexpected {other}"),
    }
}
