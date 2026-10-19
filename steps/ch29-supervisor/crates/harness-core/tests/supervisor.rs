use std::time::{Duration, Instant};

use harness_core::supervisor::{
    child_call, run_specialists, worktree, Job, Pool, TeamError, BLOCKED_TOOLS,
    HERMES_MAX_CONCURRENT_CHILDREN, MAX_DEPTH, WORKER_BOUND,
};

fn jobs(n: usize, hold: Option<Duration>) -> Vec<Job> {
    (0..n)
        .map(|i| Job {
            name: "specialist",
            reply: format!("reply {i}"),
            hold,
        })
        .collect()
}

#[test]
fn eight_specialists_share_the_bound_and_the_context() {
    let pool = Pool::new(WORKER_BOUND);
    let context = pool.context();
    let hold = Duration::from_millis(40);
    let started = Instant::now();
    let replies = run_specialists(&pool, &jobs(WORKER_BOUND, Some(hold)));
    let elapsed = started.elapsed();
    assert_eq!(replies.len(), WORKER_BOUND);
    assert_eq!(pool.max_seen(), WORKER_BOUND);
    assert!(std::sync::Arc::ptr_eq(&context, &pool.context()));
    // Eight serial sleeps would be hold * 8. macOS CI measured about 186 ms,
    // which is still under seven holds and above a fully parallel 40 ms.
    assert!(elapsed < hold * 7, "elapsed {elapsed:?}");
    assert_eq!(HERMES_MAX_CONCURRENT_CHILDREN, 10);
    assert_ne!(pool.limit(), HERMES_MAX_CONCURRENT_CHILDREN);
}

#[test]
fn a_ninth_does_not_raise_the_bound() {
    let pool = Pool::new(WORKER_BOUND);
    let replies = run_specialists(
        &pool,
        &jobs(WORKER_BOUND + 1, Some(Duration::from_millis(20))),
    );
    assert_eq!(replies.len(), WORKER_BOUND + 1);
    assert_eq!(pool.max_seen(), WORKER_BOUND);
}

#[test]
fn a_child_cannot_call_the_blocked_tools_and_depth_stops_at_one() {
    for tool in BLOCKED_TOOLS {
        let refused = child_call(tool, MAX_DEPTH);
        assert!(matches!(refused, Err(TeamError::Blocked(name)) if name == *tool));
    }
    assert!(child_call("read_file", MAX_DEPTH).is_ok());
    assert!(child_call("delegate_task", 0).is_ok());
    assert!(matches!(
        child_call("read_file", MAX_DEPTH + 1),
        Err(TeamError::Blocked(_))
    ));
}

#[test]
fn the_worktree_is_off_unless_asked_and_reflection_is_a_queue() {
    assert_eq!(worktree(false), None);
    assert_eq!(worktree(true), Some("opt-in"));
    let pool = Pool::new(WORKER_BOUND);
    pool.enqueue_reflection("team run finished");
    assert_eq!(pool.reflection_len(), 1);
    assert_eq!(pool.max_seen(), 0);
}

#[test]
fn the_demo_joins_two_replies() {
    assert_eq!(harness_core::supervisor::demo(), "three steps | the draft");
}
