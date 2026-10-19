use my_harness::chapter_50_span_finishes_turn;

#[test]
fn a_span_per_event_still_finishes_the_mock_turn() {
    assert!(chapter_50_span_finishes_turn());
}
