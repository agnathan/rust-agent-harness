use my_harness::chapter_15_web_search_allowed;

#[test]
fn the_fifty_first_web_search_is_blocked() {
    assert!(chapter_15_web_search_allowed(49));
    assert!(!chapter_15_web_search_allowed(50));
}
