use my_harness::chapter_16_blocks_metadata;

#[test]
fn a_metadata_address_is_not_a_safe_extract_url() {
    assert!(chapter_16_blocks_metadata("http://169.254.169.254/latest"));
}
