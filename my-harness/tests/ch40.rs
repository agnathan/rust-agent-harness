use my_harness::chapter_40_navigates_metadata;

#[test]
fn a_metadata_url_is_not_navigated() {
    assert!(!chapter_40_navigates_metadata(
        "http://169.254.169.254/latest/meta-data"
    ));
}
