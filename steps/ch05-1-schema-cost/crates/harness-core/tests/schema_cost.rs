use harness_core::{generate, mention_count, schema_cost, schema_json};

#[test]
fn names_are_unique_and_stable() {
    let cards = generate(64);
    assert_eq!(cards[0].name, "files_list_file_0");
    assert_eq!(cards[24].name, "files_rename_file_24");
    assert_eq!(cards.len(), 64);
    let again = generate(64);
    assert_eq!(cards[24].name, again[24].name);
    let mut names: Vec<_> = cards.iter().map(|card| card.name.clone()).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), 64);
}

#[test]
fn schema_is_mcp_shaped_and_omits_the_server() {
    let card = &generate(1)[0];
    let value = card.schema_value();
    assert_eq!(value["name"], "files_list_file_0");
    assert!(value["description"].as_str().unwrap().contains("files"));
    assert_eq!(value["input_schema"]["type"], "object");
    assert!(value["input_schema"]["required"].as_array().unwrap().len() == 1);
    assert!(value.get("server").is_none());
    let parsed: serde_json::Value =
        serde_json::from_str(&schema_json(std::slice::from_ref(card))).unwrap();
    assert_eq!(parsed[0]["name"], card.name);
}

#[test]
fn token_estimate_is_characters_over_four() {
    let cards = generate(30);
    let body = schema_json(&cards);
    let cost = schema_cost(&cards);
    assert_eq!(cost.tools, 30);
    assert_eq!(cost.bytes, body.len());
    assert_eq!(cost.tokens, body.chars().count().div_ceil(4));
}

#[test]
fn ten_thousand_schemas_cost_more_than_eight_times_one_thousand() {
    let small = schema_cost(&generate(1_000));
    let large = schema_cost(&generate(10_000));
    assert!(large.tokens > small.tokens * 8, "{large:?} vs {small:?}");
    assert!(large.bytes > small.bytes * 8);
}

#[test]
fn a_short_query_still_mentions_many_tools() {
    let cards = generate(10_000);
    let mentions = mention_count(&cards, "rename a file");
    assert!(mentions > 50, "mentions={mentions}");
    assert!(mentions < cards.len());
}

#[test]
fn every_fifth_description_names_another_tool() {
    let cards = generate(10);
    assert!(cards[0].description.contains("tool:"));
    assert!(!cards[1].description.contains("tool:"));
}
