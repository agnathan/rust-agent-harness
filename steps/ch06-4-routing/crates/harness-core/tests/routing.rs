use std::collections::HashSet;

use harness_core::{
    generate, prepare_card, rewrite_description, schema_json, shrink_card, Router, ToolCard,
};

#[test]
fn queries_route_to_the_server_that_owns_the_noun() {
    let router = Router::build(generate(512));
    assert_eq!(router.route("rename a file"), "files");
    assert_eq!(router.route("rename a branch"), "git");
    let hits = router.search("rename a file", 5);
    assert!(hits.iter().all(|hit| hit.server == "files"), "{hits:?}");
    assert!(hits.iter().any(|hit| hit.name.contains("rename")));
}

#[test]
fn shrink_normalizes_the_name_and_trims_on_a_word() {
    let card = ToolCard {
        name: "Rename File".into(),
        description: "Use the files server to rename a file and then write a long note about the path that the caller passed in so the description exceeds eighty characters.".into(),
        input_schema: serde_json::json!({"type": "object"}),
        server: "files".into(),
    };
    let shrunk = shrink_card(&card);
    assert_eq!(shrunk.name, "files_rename_file");
    assert!(shrunk.description.chars().count() <= 80);
    assert!(card.description.starts_with(&shrunk.description));
    assert!(shrunk.description.len() < card.description.len());
}

#[test]
fn rewrite_drops_a_tool_the_agent_cannot_call() {
    let text = "Use the files server to rename a file. See also tool:shell_run_command_9.";
    let available = HashSet::from(["files_rename_file_1".to_string()]);
    let rewritten = rewrite_description(text, &available);
    assert!(!rewritten.contains("tool:"));
    assert!(rewritten.contains("rename"));

    let kept = HashSet::from(["shell_run_command_9".to_string()]);
    let still = rewrite_description(text, &kept);
    assert!(still.contains("tool:shell_run_command_9"));
}

#[test]
fn prepare_is_smaller_than_the_raw_catalog() {
    let cards = generate(200);
    let names: HashSet<String> = cards.iter().map(|card| card.name.clone()).collect();
    let prepared: Vec<_> = cards
        .iter()
        .map(|card| prepare_card(card, &names))
        .collect();
    assert!(schema_json(&prepared).len() <= schema_json(&cards).len());
    assert!(prepared
        .iter()
        .all(|card| card.name.starts_with(&format!("{}_", card.server))));
}
