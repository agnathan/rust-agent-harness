use std::time::Duration;

use composition::{
    dispatch, limits, nested_fanout, nested_fanout_holding, run_continuation, wait_target,
    FakeModel, Gate, LoopStop, Route, WaitTarget, DEFAULT_GATE_MAX_RETRIES,
};
use handoffs::{Board as HandoffBoard, Message};
use supervisor::{Pool, WORKER_BOUND};

#[test]
fn a_nested_fan_out_past_the_bound_is_refused() {
    let pool = Pool::new(WORKER_BOUND);
    let refused = nested_fanout(&pool, 0, WORKER_BOUND + 1).unwrap_err();
    assert!(matches!(
        refused,
        composition::ComposeError::ExceedsBound {
            held: 0,
            asked: 9,
            limit: 8
        }
    ));
    assert_eq!(pool.max_seen(), 0);
    let also = nested_fanout(&pool, 1, WORKER_BOUND).unwrap_err();
    assert!(matches!(
        also,
        composition::ComposeError::ExceedsBound {
            held: 1,
            asked: 8,
            ..
        }
    ));
    assert_eq!(pool.max_seen(), 0);
}

#[test]
fn eight_nested_children_use_the_same_pool() {
    let pool = Pool::new(WORKER_BOUND);
    let summary = nested_fanout_holding(&pool, 0, WORKER_BOUND, Duration::from_millis(40)).unwrap();
    assert_eq!(summary, "n0\nn1\nn2\nn3\nn4\nn5\nn6\nn7");
    assert_eq!(pool.max_seen(), WORKER_BOUND);
    let quiet = Pool::new(WORKER_BOUND);
    let again = nested_fanout(&quiet, 0, WORKER_BOUND).unwrap();
    assert_eq!(again, summary);
    assert!(quiet.max_seen() <= WORKER_BOUND);
}

#[test]
fn a_handoff_runs_only_when_ownership_moves() {
    let board = HandoffBoard::new("supervisor");
    let messages = [
        Message {
            role: "user".into(),
            text: "the note".into(),
        },
        Message {
            role: "scratch".into(),
            text: "private".into(),
        },
    ];
    assert_eq!(
        dispatch(Route::Stay, &board, "billing", &messages).unwrap(),
        None
    );
    assert_eq!(board.owner(), "supervisor");
    assert_eq!(board.count(), 0);
    let moved = dispatch(Route::Move, &board, "billing", &messages)
        .unwrap()
        .unwrap();
    assert_eq!(moved.len(), 1);
    assert_eq!(board.owner(), "billing");
}

#[test]
fn the_loop_accepts_done_and_parks_a_wait_that_names_a_target() {
    let (turns, timeout, gate_timeout, retries) = limits();
    assert_eq!(turns, 20);
    assert_eq!(timeout, 30);
    assert_eq!(gate_timeout, 300);
    assert_eq!(retries, 3);
    let pool = Pool::new(WORKER_BOUND);
    let mut model = FakeModel::script(["verdict: continue", "verdict: done"]);
    assert_eq!(run_continuation(&mut model, &pool, &mut []), LoopStop::Done);
    let mut waiting = FakeModel::script([r#"{"verdict": "wait", "wait_on_session": "sess-9"}"#]);
    assert_eq!(
        run_continuation(&mut waiting, &pool, &mut []),
        LoopStop::Parked(WaitTarget::Session("sess-9".into()))
    );
    assert_eq!(
        wait_target(r#"{"verdict": "wait", "wait_on_pid": 44}"#),
        Some(WaitTarget::Pid(44))
    );
    assert_eq!(
        wait_target(r#"{"verdict": "wait", "wait_for_seconds": 12}"#),
        Some(WaitTarget::Seconds(12))
    );
    assert_eq!(wait_target(r#"{"verdict": "wait"}"#), None);
    let mut bare = FakeModel::script(["verdict: wait", "verdict: done"]);
    assert_eq!(run_continuation(&mut bare, &pool, &mut []), LoopStop::Done);
}

#[test]
fn a_gate_failure_does_not_ask_the_judge_and_three_retries_then_pause() {
    let pool = Pool::new(WORKER_BOUND);
    let mut gates = [Gate::scripted("cargo test", [1, 0])];
    let mut model = FakeModel::script(["ignored while the gate is red", "verdict: done"]);
    assert_eq!(
        run_continuation(&mut model, &pool, &mut gates),
        LoopStop::Done
    );
    assert_eq!(gates[0].attempts, 0);
    assert_eq!(pool.max_seen(), 1);
    let fresh = Pool::new(WORKER_BOUND);
    let mut stuck = [Gate::scripted(
        "cargo test",
        std::iter::repeat_n(1, DEFAULT_GATE_MAX_RETRIES as usize + 1),
    )];
    let mut unused = FakeModel::script(std::iter::repeat_n("verdict: done", 4));
    assert_eq!(
        run_continuation(&mut unused, &fresh, &mut stuck),
        LoopStop::GatePaused {
            command: "cargo test".into(),
        }
    );
    assert_eq!(fresh.max_seen(), 0);
}

#[test]
fn twenty_continues_stop_the_loop() {
    let pool = Pool::new(WORKER_BOUND);
    let mut model = FakeModel::script(std::iter::repeat_n("verdict: continue", 21));
    assert_eq!(
        run_continuation(&mut model, &pool, &mut []),
        LoopStop::Turns
    );
}

#[test]
fn a_missing_verdict_rejects() {
    let pool = Pool::new(WORKER_BOUND);
    let mut model = FakeModel::script(["looks finished"]);
    assert_eq!(
        run_continuation(&mut model, &pool, &mut []),
        LoopStop::Reject
    );
}

#[test]
fn the_demo_stays_with_the_supervisor() {
    assert_eq!(composition::demo(), "fan n0 | n1; owner supervisor; done");
}
