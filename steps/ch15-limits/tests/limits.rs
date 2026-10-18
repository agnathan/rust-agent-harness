use std::time::{Duration, Instant};

use gw_limits::{
    CallError, CallOpts, FailureKind, LimitedGateway, Outcome, Script, ACCEPT_BOUND, EXECUTE_BOUND,
    MAX_SUBAGENTS, MAX_WEB_SEARCHES,
};

#[test]
fn an_idempotent_transport_error_retries_once() {
    let gateway = LimitedGateway::new();
    gateway.set_idempotent("echo", true);
    let script = Script::new(vec![
        Outcome::Fail(FailureKind::Transport),
        Outcome::Ok("ok".into()),
    ]);
    let body = gateway
        .dispatch("echo", "", CallOpts::default(), &script)
        .expect("retry");
    assert_eq!(body, "ok");
    assert_eq!(script.calls(), 2);
    assert!(!gateway.breaker_open("echo"));
}

#[test]
fn a_write_does_not_retry_and_status_429_does_not_retry() {
    let gateway = LimitedGateway::new();
    gateway.set_idempotent("write", false);
    gateway.set_idempotent("fetch", true);
    let writes = Script::new(vec![
        Outcome::Fail(FailureKind::Transport),
        Outcome::Ok("should-not-run".into()),
    ]);
    let err = gateway
        .dispatch("write", "", CallOpts::default(), &writes)
        .expect_err("no retry");
    assert_eq!(err, CallError::Failed(FailureKind::Transport));
    assert_eq!(writes.calls(), 1);

    let limited = Script::new(vec![
        Outcome::Fail(FailureKind::Status(429)),
        Outcome::Ok("should-not-run".into()),
    ]);
    let err = gateway
        .dispatch("fetch", "", CallOpts::default(), &limited)
        .expect_err("backpressure");
    assert_eq!(err, CallError::Failed(FailureKind::Status(429)));
    assert_eq!(limited.calls(), 1);

    let unavailable = Script::new(vec![Outcome::Fail(FailureKind::Status(503))]);
    let err = gateway
        .dispatch("fetch", "", CallOpts::default(), &unavailable)
        .expect_err("503");
    assert_eq!(err, CallError::Failed(FailureKind::Status(503)));
    assert_eq!(unavailable.calls(), 1);
}

#[test]
fn three_failures_open_the_breaker_and_the_next_call_does_not_run() {
    let gateway = LimitedGateway::new();
    let script = Script::new(vec![
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Ok("late".into()),
    ]);
    for _ in 0..3 {
        let err = gateway
            .dispatch("search", "", CallOpts::default(), &script)
            .expect_err("fail");
        assert!(matches!(err, CallError::Failed(_)));
    }
    assert!(gateway.breaker_open("search"));
    assert!(!gateway.breaker_open("echo"));
    let err = gateway
        .dispatch("search", "", CallOpts::default(), &script)
        .expect_err("open");
    assert_eq!(err, CallError::OpenBreaker);
    assert_eq!(script.calls(), 3);
}

#[test]
fn a_success_clears_the_failure_count() {
    let gateway = LimitedGateway::new();
    let script = Script::new(vec![
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Ok("ok".into()),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
    ]);
    let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    gateway
        .dispatch("search", "", CallOpts::default(), &script)
        .expect("reset");
    let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    assert!(!gateway.breaker_open("search"));
}

#[test]
fn web_search_stops_at_50_and_delegate_task_counts_a_batch() {
    let gateway = LimitedGateway::new();
    let script = Script::new(vec![Outcome::Ok("hit".into()); MAX_WEB_SEARCHES as usize]);
    for n in 1..=MAX_WEB_SEARCHES {
        gateway
            .dispatch("web_search", "", CallOpts::default(), &script)
            .unwrap_or_else(|_| panic!("search {n}"));
    }
    let err = gateway
        .dispatch("web_search", "", CallOpts::default(), &script)
        .expect_err("cap");
    let CallError::Cap(message) = err else {
        panic!("cap");
    };
    assert!(message.contains("already made 50 web searches"));
    assert_eq!(script.calls(), MAX_WEB_SEARCHES);

    let gateway = LimitedGateway::new();
    let script = Script::new(vec![
        Outcome::Ok("child".into());
        (MAX_SUBAGENTS - 1) as usize
    ]);
    gateway
        .dispatch(
            "delegate_task",
            r#"{"tasks":[1,2,3]}"#,
            CallOpts::default(),
            &script,
        )
        .expect("batch");
    for _ in 0..(MAX_SUBAGENTS - 3) {
        gateway
            .dispatch("delegate_task", "", CallOpts::default(), &script)
            .expect("one");
    }
    let err = gateway
        .dispatch("delegate_task", "", CallOpts::default(), &script)
        .expect_err("subagents");
    let CallError::Cap(message) = err else {
        panic!("cap");
    };
    assert!(message.contains("already spawned 50 subagents"));
    let before = script.calls();
    gateway
        .dispatch(
            "delegate_task",
            r#"{"action":"list"}"#,
            CallOpts::default(),
            &script,
        )
        .expect("control action");
    assert_eq!(script.calls(), before + 1);
}

#[test]
fn a_hedge_runs_a_second_call_only_when_the_tool_is_idempotent() {
    let gateway = LimitedGateway::new();
    gateway.set_idempotent("echo", true);
    let script = Script::new(vec![
        Outcome::Ok("first".into()),
        Outcome::Ok("second".into()),
    ]);
    let body = gateway
        .dispatch("echo", "", CallOpts { hedge: true }, &script)
        .expect("hedge");
    assert_eq!(body, "first");
    assert_eq!(script.calls(), 2);

    gateway.set_idempotent("write", false);
    let writes = Script::new(vec![
        Outcome::Ok("once".into()),
        Outcome::Ok("second".into()),
    ]);
    let body = gateway
        .dispatch("write", "", CallOpts { hedge: true }, &writes)
        .expect("no hedge");
    assert_eq!(body, "once");
    assert_eq!(writes.calls(), 1);
}

#[test]
fn the_33rd_accept_is_refused_and_a_timeout_returns_both_permits() {
    let gateway = LimitedGateway::new();
    let mut held = Vec::new();
    for _ in 0..ACCEPT_BOUND {
        held.push(gateway.hold_accept("echo").expect("accept"));
    }
    assert_eq!(
        gateway.hold_accept("echo").expect_err("full"),
        CallError::Saturated
    );
    drop(held);
    assert_eq!(gateway.accept_available("echo"), ACCEPT_BOUND);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("runtime");
    let started = Instant::now();
    let err =
        runtime.block_on(
            gateway.run_with_timeout("slow", Duration::from_millis(30), || async {
                tokio::time::sleep(Duration::from_secs(5)).await;
                "late".into()
            }),
        );
    assert_eq!(err.expect_err("timeout"), CallError::Timeout);
    assert!(started.elapsed() < Duration::from_secs(2));
    assert_eq!(gateway.accept_available("slow"), ACCEPT_BOUND);
    assert_eq!(gateway.execute_available("slow"), EXECUTE_BOUND);
}

#[test]
fn the_process_gateway_still_starts_empty() {
    let gateway = LimitedGateway::new();
    assert!(gateway.catalog().is_empty());
    gateway.open_session("session-1");
    let view = gateway.session_tools("session-1").expect("session");
    assert!(view.tools.is_empty());
}
