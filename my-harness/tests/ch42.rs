use my_harness::chapter_42_route_after_kill;

#[test]
fn killing_the_route_fails_over_to_the_other_replica() {
    assert_eq!(chapter_42_route_after_kill("a", "b").as_deref(), Some("b"));
}
