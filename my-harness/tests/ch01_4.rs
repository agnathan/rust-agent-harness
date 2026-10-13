use my_harness::timeout_after;
use std::time::Duration;

#[test]
fn a_slow_request_is_a_timeout() {
    let err = timeout_after(Duration::from_millis(200));
    let text = err.to_string();
    assert!(
        text.contains("200ms"),
        "timeout error should name the duration, got {text}"
    );
}
