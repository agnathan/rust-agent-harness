use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};

use harness_core::failure::{
    call_with_timeout, deterministic_empty, is_repetition_dominated, length_verdict, retry_budget,
    retry_in_bound, run_models, EmptyAttempt, FakeMcp, Kind, LengthVerdict, Model, Reply,
    ToolPermits, COST_THRESHOLD_USD, DEFAULT_EMPTY_RETRY_BUDGET, MIN_FRAGMENT_LENGTH, PERMIT_BOUND,
    REDUCED_EMPTY_RETRY_BUDGET, REPETITION_NOTE,
};

fn empty(cost: Option<f64>) -> Reply {
    Reply::Empty {
        cost: cost.map(harness_core::failure::OrderedF64::usd),
        usage_present: true,
        zero_output: true,
        finish_reason: "stop".into(),
    }
}

fn model(name: &str, replies: Vec<Reply>) -> Model {
    Model {
        name: name.into(),
        provider: "openrouter".into(),
        replies,
    }
}

#[test]
fn a_cheap_empty_gets_three_retries_and_an_expensive_one_gets_one() {
    assert_eq!(retry_budget(None), DEFAULT_EMPTY_RETRY_BUDGET);
    assert_eq!(retry_budget(Some(COST_THRESHOLD_USD)), 3);
    assert_eq!(retry_budget(Some(0.26)), REDUCED_EMPTY_RETRY_BUDGET);
}

#[test]
fn two_matching_zero_output_empties_are_deterministic() {
    let attempt = EmptyAttempt {
        model: "m".into(),
        provider: "p".into(),
        finish_reason: "stop".into(),
        usage_present: true,
        zero_output: true,
    };
    assert!(!deterministic_empty(std::slice::from_ref(&attempt)));
    assert!(deterministic_empty(&[attempt.clone(), attempt.clone()]));
    let mut other = attempt.clone();
    other.zero_output = false;
    assert!(!deterministic_empty(&[attempt.clone(), other]));
}

#[test]
fn deterministic_empties_skip_to_the_fallback() {
    let turn = run_models(&[
        model(
            "primary",
            vec![
                empty(Some(0.01)),
                empty(Some(0.01)),
                Reply::Text("late".into()),
            ],
        ),
        model("side", vec![Reply::Text("hello".into())]),
    ]);
    assert_eq!(turn.text, "hello");
    assert_eq!(turn.fallbacks, 1);
}

#[test]
fn missing_usage_keeps_the_full_empty_budget() {
    let unclear = Reply::Empty {
        cost: None,
        usage_present: false,
        zero_output: false,
        finish_reason: "stop".into(),
    };
    let turn = run_models(&[model(
        "primary",
        vec![
            unclear.clone(),
            unclear.clone(),
            unclear,
            Reply::Text("ok".into()),
        ],
    )]);
    assert_eq!(turn.text, "ok");
    assert_eq!(turn.fallbacks, 0);
    assert!(turn.kind.is_none());
}

#[test]
fn a_short_repeat_is_kept_and_a_long_one_stops_continuation() {
    assert!(!is_repetition_dominated(&"ab".repeat(10)));
    let _ = MIN_FRAGMENT_LENGTH;
    let long = "abcdefghij".repeat(80);
    assert!(is_repetition_dominated(&long));
    assert_eq!(length_verdict("length", &long), LengthVerdict::StopRepeated);
    let prose = "a short cut off answer";
    assert!(matches!(
        length_verdict("length", prose),
        LengthVerdict::Continue { .. }
    ));
    assert!(matches!(
        length_verdict("stop", prose),
        LengthVerdict::Finished
    ));
    let turn = run_models(&[model("primary", vec![Reply::Length(long)])]);
    assert_eq!(turn.text, REPETITION_NOTE);
    assert_eq!(turn.kind, Some(Kind::Repeated));
}

#[tokio::test]
async fn a_timeout_drops_the_permit() {
    let permits = ToolPermits::new(PERMIT_BOUND);
    let started = Instant::now();
    let result = call_with_timeout(&permits, Duration::from_millis(30), || async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        "late".into()
    })
    .await;
    assert_eq!(result, Err(Kind::Timeout));
    assert_eq!(permits.available(), PERMIT_BOUND);
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[tokio::test]
async fn retries_hold_one_permit_at_a_time() {
    let permits = ToolPermits::new(PERMIT_BOUND);
    let peak = AtomicUsize::new(0);
    let done = retry_in_bound(&permits, 3, &peak).await;
    assert_eq!(done, 3);
    assert_eq!(peak.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(permits.available(), PERMIT_BOUND);
}

#[test]
fn a_crashed_session_reconnects_once() {
    let mut session = FakeMcp::new(1);
    assert_eq!(session.call_until_up(), Ok("pong"));
    assert_eq!(session.reconnects(), 1);
}
