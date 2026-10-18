use std::time::Duration;

use fanout::{Board, FanError, Task, QUEUE_CAP};
use supervisor::{Pool, WORKER_BOUND};

fn task(index: usize, reply: &str) -> Task {
    Task {
        task_index: index,
        reply: reply.into(),
        tool_trace: "secret-tool".into(),
    }
}

#[test]
fn the_queue_holds_one_hundred_and_refuses_the_next() {
    let board = Board::new();
    for index in 0..QUEUE_CAP {
        board.schedule(task(index, "ok")).unwrap();
    }
    assert_eq!(board.len(), QUEUE_CAP);
    assert_eq!(board.schedule(task(QUEUE_CAP, "no")), Err(FanError::Full));
    assert_eq!(board.len(), QUEUE_CAP);
}

#[test]
fn gather_sorts_summaries_and_hides_tool_traces() {
    let board = Board::new();
    board.schedule(task(2, "c")).unwrap();
    board.schedule(task(0, "a")).unwrap();
    board.schedule(task(1, "b")).unwrap();
    let pool = Pool::new(WORKER_BOUND);
    let summary = board.gather(&pool);
    assert_eq!(summary, "a\nb\nc");
    assert!(!summary.contains("secret-tool"));
    assert_eq!(board.len(), 0);
}

#[test]
fn gather_respects_the_worker_bound() {
    let board = Board::new();
    for index in 0..16 {
        board.schedule(task(index, "ok")).unwrap();
    }
    let pool = Pool::new(WORKER_BOUND);
    let summary = board.gather_holding(&pool, Duration::from_millis(20));
    assert_eq!(summary.lines().count(), 16);
    assert_eq!(pool.max_seen(), WORKER_BOUND);
}

#[test]
fn the_demo_is_sorted() {
    assert_eq!(fanout::demo(), "a\nb\nc");
}
