use my_harness::chapter_16_fallback_model;

#[test]
fn an_invalid_sort_uses_the_fallback() {
    assert_eq!(chapter_16_fallback_model(false), "fallback");
}
