use my_harness::chapter_44_update_submits_once;

#[test]
fn one_acp_update_becomes_one_inbox_submit() {
    assert_eq!(chapter_44_update_submits_once(1), 1);
}
