use harness_core::supervision::{
    restart_child, Call, ChildFailed, FakeModel, RestartError, Supervisor, MAX_GAP_SECONDS,
};

#[test]
fn a_failed_child_is_restarted_once() {
    let mut child = FakeModel::script([Err(ChildFailed), Ok("ok".into())]);
    let call = Call::new(1);
    let supervisor = Supervisor::new();
    assert_eq!(
        restart_child(&mut child, &call, &supervisor, 0).unwrap(),
        "ok"
    );
    assert_eq!(child.calls, 2);
    assert_eq!(supervisor.restarts(), 1);
}

#[test]
fn three_chained_restarts_trip_and_a_wide_gap_does_not() {
    let supervisor = Supervisor::new();
    assert!(!supervisor.record(0));
    assert!(!supervisor.record(10));
    assert!(supervisor.record(20));
    let fresh = Supervisor::new();
    assert!(!fresh.record(0));
    assert!(!fresh.record(MAX_GAP_SECONDS + 1));
    assert_eq!(fresh.restarts(), 2);
}

#[test]
fn an_io_failure_does_not_trip() {
    let supervisor = Supervisor::new();
    supervisor.fail_io();
    assert!(!supervisor.record(0));
    assert!(!supervisor.record(1));
    assert!(!supervisor.record(2));
    assert_eq!(supervisor.restarts(), 0);
}

#[test]
fn cancel_and_an_empty_budget_do_not_start_the_child() {
    let mut child = FakeModel::script([Ok("ok".into())]);
    let call = Call::new(1);
    call.cancel();
    let supervisor = Supervisor::new();
    assert_eq!(
        restart_child(&mut child, &call, &supervisor, 0),
        Err(RestartError::Cancelled)
    );
    assert_eq!(child.calls, 0);
    let mut child = FakeModel::script([Ok("ok".into())]);
    let call = Call::new(0);
    assert_eq!(
        restart_child(&mut child, &call, &supervisor, 0),
        Err(RestartError::Budget)
    );
    assert_eq!(child.calls, 0);
}

#[test]
fn the_third_failure_in_one_supervisor_stops_auto_resume() {
    let supervisor = Supervisor::new();
    let call = Call::new(1);
    let mut child = FakeModel::script([Err(ChildFailed), Ok("ok".into())]);
    assert!(restart_child(&mut child, &call, &supervisor, 0).is_ok());
    let mut child = FakeModel::script([Err(ChildFailed), Ok("ok".into())]);
    assert!(restart_child(&mut child, &call, &supervisor, 1).is_ok());
    let mut child = FakeModel::script([Err(ChildFailed), Ok("should-not-run".into())]);
    assert_eq!(
        restart_child(&mut child, &call, &supervisor, 2),
        Err(RestartError::Breaker)
    );
    assert_eq!(child.calls, 1);
}

#[test]
fn the_demo_restarts() {
    assert_eq!(harness_core::supervision::demo(), "restarted: ok");
}

use harness_core::supervision::{
    is_stale, Pulse, HEARTBEAT_INTERVAL_SECS, HEARTBEAT_STALE_CYCLES_IDLE,
    HEARTBEAT_STALE_CYCLES_IN_TOOL,
};

fn idle_pulse(at: u64) -> Pulse {
    Pulse {
        iteration: 1,
        tool: None,
        last_activity_ts: at,
        in_tool: false,
    }
}

#[test]
fn an_idle_child_is_stale_at_fifteen_cycles() {
    let previous = idle_pulse(0);
    let almost = HEARTBEAT_INTERVAL_SECS * (HEARTBEAT_STALE_CYCLES_IDLE - 1);
    assert!(!is_stale(&previous, almost, 1, None, 0));
    let due = HEARTBEAT_INTERVAL_SECS * HEARTBEAT_STALE_CYCLES_IDLE;
    assert!(is_stale(&previous, due, 1, None, 0));
}

#[test]
fn a_child_inside_a_tool_uses_the_longer_threshold() {
    let previous = Pulse {
        iteration: 3,
        tool: Some("terminal".into()),
        last_activity_ts: 0,
        in_tool: true,
    };
    let idle_due = HEARTBEAT_INTERVAL_SECS * HEARTBEAT_STALE_CYCLES_IDLE;
    assert!(!is_stale(&previous, idle_due, 3, Some("terminal"), 0));
    let tool_due = HEARTBEAT_INTERVAL_SECS * HEARTBEAT_STALE_CYCLES_IN_TOOL;
    assert!(is_stale(&previous, tool_due, 3, Some("terminal"), 0));
}

#[test]
fn a_moved_activity_timestamp_is_not_idle() {
    let previous = idle_pulse(0);
    let far = HEARTBEAT_INTERVAL_SECS * HEARTBEAT_STALE_CYCLES_IN_TOOL;
    assert!(!is_stale(&previous, far, 1, None, far));
}
