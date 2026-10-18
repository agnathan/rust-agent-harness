use my_harness::chapter_31_refuses_overflow;

#[test]
fn the_101st_task_is_refused() {
    assert!(!chapter_31_refuses_overflow(99));
    assert!(chapter_31_refuses_overflow(100));
}
