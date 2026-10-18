use my_harness::chapter_11_terminal_verdict;

#[test]
fn a_hardline_terminal_command_is_denied() {
    let verdict = chapter_11_terminal_verdict("rm -rf /");
    assert!(verdict.contains("BLOCKED"), "{verdict}");
    assert!(!verdict.contains("echo hello ran"));
}
