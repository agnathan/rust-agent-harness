use my_harness::chapter_22_discover_hides_subagent;

#[test]
fn discover_hides_a_subagent() {
    assert!(chapter_22_discover_hides_subagent("subagent"));
    assert!(!chapter_22_discover_hides_subagent("user"));
}
