use my_harness::chapter_17_stale_write_refused;

#[test]
fn a_write_without_a_read_is_refused() {
    assert!(chapter_17_stale_write_refused(false));
}
