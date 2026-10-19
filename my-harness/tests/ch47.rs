use my_harness::chapter_47_published;

#[test]
fn a_published_envelope_scrubs_the_key() {
    let body = chapter_47_published("see sk-live-0123456789abcdef");
    assert!(!body.contains("sk-live"), "{body}");
    assert!(body.contains("sk-***"), "{body}");
}
