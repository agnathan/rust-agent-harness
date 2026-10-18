use my_harness::{chapter_29_pool_limit, chapter_29_reflection_starts_reviewer};

#[test]
fn eight_specialists_share_one_pool_and_a_reflection_note_does_not_run_the_reviewer() {
    assert_eq!(chapter_29_pool_limit(), 8);
    assert!(!chapter_29_reflection_starts_reviewer());
}
