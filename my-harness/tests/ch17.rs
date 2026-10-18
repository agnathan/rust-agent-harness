use my_harness::chapter_17_fold_matches_buffer;

#[test]
fn folding_plain_chunks_matches_a_buffer() {
    assert!(chapter_17_fold_matches_buffer(&["hel", "lo"]));
}
