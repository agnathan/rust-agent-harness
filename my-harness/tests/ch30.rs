use my_harness::chapter_30_keeps_scratch;

#[test]
fn a_hundred_message_filter_drops_what_the_next_agent_must_not_see() {
    assert!(!chapter_30_keeps_scratch());
}
