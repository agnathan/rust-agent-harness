use my_harness::chapter_33_nested_width_fits;

#[test]
fn nested_width_9_is_refused_before_tasks_start() {
    assert!(!chapter_33_nested_width_fits(0, 9, 8));
}
