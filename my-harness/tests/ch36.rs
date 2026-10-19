use my_harness::chapter_36_idle_is_stale;

#[test]
fn idle_stale_uses_15_cycles_and_a_tool_uses_40() {
    assert!(chapter_36_idle_is_stale(15, false));
    assert!(!chapter_36_idle_is_stale(15, true));
    assert!(chapter_36_idle_is_stale(40, true));
}
