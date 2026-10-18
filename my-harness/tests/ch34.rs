use my_harness::chapter_34_shares_snapshot;

#[test]
fn the_hot_path_sends_the_arc_snapshot() {
    assert!(chapter_34_shares_snapshot());
}
