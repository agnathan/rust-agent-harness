use my_harness::chapter_49_cancel_reached;

#[test]
fn cancel_reaches_an_in_flight_remote_call() {
    assert!(chapter_49_cancel_reached());
}
