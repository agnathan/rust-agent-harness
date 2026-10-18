use my_harness::chapter_18_stored_line;

#[test]
fn a_stored_turn_does_not_keep_the_key() {
    let stored = chapter_18_stored_line("sk-abcdefghijklmnopqrstuvwxyz");
    assert!(stored.contains("sk-***"), "{stored}");
    assert!(!stored.contains("abcdefghijklmnopqrstuvwxyz"), "{stored}");
}
