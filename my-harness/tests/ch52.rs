use my_harness::chapter_52_saved_passed;

#[test]
fn the_saved_set_still_passes_on_the_mock() {
    assert!(chapter_52_saved_passed("echo-hello"));
    assert!(chapter_52_saved_passed("direct"));
    assert!(chapter_52_saved_passed("eight-reads"));
}
