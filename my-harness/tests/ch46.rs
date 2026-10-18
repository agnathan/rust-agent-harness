use my_harness::chapter_46_claims_won;

#[test]
fn two_claims_of_one_task_one_wins() {
    assert_eq!(chapter_46_claims_won(2), 1);
}
