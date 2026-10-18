use my_harness::chapter_19_fold_matches_buffer;

#[test]
fn folding_plain_chunks_matches_a_buffer() {
    assert!(chapter_19_fold_matches_buffer(&["hel", "lo"]));
}
