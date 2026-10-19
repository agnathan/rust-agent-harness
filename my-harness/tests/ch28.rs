use my_harness::chapter_28_same_context;

#[test]
fn the_next_stage_receives_arc_context() {
    assert!(chapter_28_same_context(1, 1));
}
