use my_harness::chapter_13_execute_code;

#[test]
fn cron_cannot_run_a_local_script() {
    assert_eq!(chapter_13_execute_code(true, true), "deny");
    assert_eq!(chapter_13_execute_code(true, false), "allow");
}
