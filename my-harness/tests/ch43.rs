use my_harness::{chapter_43_lifecycle_allowed, chapter_43_retries_claimed};

#[test]
fn a_claimed_job_retries_at_most_once_and_a_lifecycle_command_is_blocked() {
    assert!(!chapter_43_retries_claimed(1));
    assert!(!chapter_43_lifecycle_allowed("hermes gateway restart"));
}
