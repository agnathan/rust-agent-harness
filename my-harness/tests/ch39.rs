use my_harness::{chapter_39_enqueue_calls_model, chapter_39_gateway_restart_allowed};

#[test]
fn a_gateway_restart_is_blocked_and_enqueue_is_not_a_model_call() {
    assert!(!chapter_39_gateway_restart_allowed(true));
    assert!(!chapter_39_enqueue_calls_model());
}
