use my_harness::chapter_37_pick_opens_connection;

#[test]
fn least_outstanding_pick_does_not_open_a_connection() {
    assert!(!chapter_37_pick_opens_connection());
}
