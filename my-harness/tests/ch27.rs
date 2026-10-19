use my_harness::chapter_27_enqueue_writes_skill;

#[test]
fn enqueue_does_not_write() {
    assert!(!chapter_27_enqueue_writes_skill());
}
