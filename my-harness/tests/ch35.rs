use my_harness::chapter_35_full_inbox_grows;

#[test]
fn a_full_inbox_does_not_grow() {
    assert!(!chapter_35_full_inbox_grows(64, 64));
}
