use harness_core::{generate, milestone, ToolSession, SEARCH_TOOL};

#[test]
fn search_does_not_reveal_schemas_until_the_next_turn() {
    let mut session = ToolSession::new(generate(400), 8);
    let before = session.visible_prompt();
    let listing = session.search_tools("rename a file", 4);
    assert!(session.staged_len() > 0, "{listing}");
    assert_eq!(session.visible_prompt(), before);
    assert_eq!(session.visible_names(), vec![SEARCH_TOOL.to_string()]);
    assert!(!listing.contains("input_schema"));
    session.begin_turn();
    assert_eq!(session.staged_len(), 0);
    assert!(session.revealed_len() > 0);
    let names = session.visible_names();
    assert_eq!(names[0], SEARCH_TOOL);
    assert!(names.len() > 1);
    assert!(session.visible_prompt().starts_with(&before));
}

#[test]
fn core_prefix_survives_eviction() {
    let mut session = ToolSession::new(generate(800), 2);
    let core = session.core_prompt();
    session.search_tools("rename a file", 2);
    session.begin_turn();
    assert_eq!(session.revealed_len(), 2);
    let first = session.visible_names();
    session.search_tools("delete a branch", 2);
    session.begin_turn();
    assert_eq!(session.core_prompt(), core);
    assert!(session.visible_prompt().starts_with(&core));
    assert_eq!(session.visible_names().len(), 3);
    let next = session.visible_names();
    assert_ne!(first, next);
    assert!(!next.contains(&first[1]));
}

#[test]
fn milestone_keeps_the_core_and_shrinks_the_prompt() {
    let report = milestone(2_000, "rename a file");
    assert_eq!(report.tools, 2_000);
    assert!(report.all_tokens > report.visible_before * 20);
    assert_eq!(report.this_turn, vec![SEARCH_TOOL.to_string()]);
    assert!(report.next_turn.len() > 1);
    assert!(report.visible_after < report.all_tokens / 10);
    assert!(report.core_same);
    assert!(report.listing.contains("rename_file"), "{}", report.listing);
}
