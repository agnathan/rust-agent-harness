use my_harness::chapter_21_snapshot_changes_on_write;

#[test]
fn a_write_does_not_replace_the_snapshot() {
    assert!(!chapter_21_snapshot_changes_on_write());
}
