use my_harness::chapter_41_accepts_image;

#[test]
fn an_oversized_or_bad_type_image_is_rejected() {
    assert!(!chapter_41_accepts_image(50 * 1024 * 1024 + 1, "image/png"));
    assert!(!chapter_41_accepts_image(32, "image/svg+xml"));
}
