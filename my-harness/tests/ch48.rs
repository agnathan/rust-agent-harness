use my_harness::chapter_48_propose_waits;

#[test]
fn skill_propose_enqueues_without_waiting() {
    assert!(!chapter_48_propose_waits());
}
