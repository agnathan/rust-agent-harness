use my_harness::chapter_38_unscoped_secret_in_prompt;

#[test]
fn an_unscoped_secret_is_the_error_and_stays_out_of_the_prompt() {
    assert!(!chapter_38_unscoped_secret_in_prompt());
}
