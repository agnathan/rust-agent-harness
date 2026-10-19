use my_harness::chapter_23_read_invokes_scorer;

#[test]
fn session_tools_reads_the_predicted_set_and_does_not_call_the_scorer() {
    assert!(!chapter_23_read_invokes_scorer());
}
