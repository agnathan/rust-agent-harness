use my_harness::chapter_26_review_runs_on_user_turn;

#[test]
fn spawning_the_reviewer_does_not_run_it_on_the_user_turn() {
    assert!(!chapter_26_review_runs_on_user_turn());
}
