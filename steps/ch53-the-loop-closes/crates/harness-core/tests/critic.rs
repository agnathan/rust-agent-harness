use std::sync::Arc;

use harness_core::critic::{as_tool, final_gate, limits, parse_verdict, Verdict, SNIPPET_CHARS};
use harness_core::supervisor::{Pool, WORKER_BOUND};

#[test]
fn done_accepts_and_a_missing_verdict_rejects() {
    assert_eq!(parse_verdict("verdict: done"), Verdict::Done);
    assert_eq!(parse_verdict("verdict: blocked"), Verdict::Blocked);
    assert_eq!(parse_verdict("verdict: wait"), Verdict::Wait);
    assert_eq!(parse_verdict("verdict: continue"), Verdict::Continue);
    assert_eq!(parse_verdict(""), Verdict::Reject);
    assert_eq!(parse_verdict("looks fine"), Verdict::Reject);
    assert_eq!(parse_verdict("verdict: maybe"), Verdict::Reject);
    let (turns, timeout) = limits();
    assert_eq!(turns, 20);
    assert_eq!(timeout, 30);
}

#[test]
fn the_tool_and_the_gate_take_one_permit_and_do_not_write() {
    let pool = Pool::new(WORKER_BOUND);
    let context = pool.context();
    let before = context.notes.lock().unwrap().len();
    let outcome = as_tool(&pool, "verdict: done\nship it");
    assert!(outcome.accepted);
    assert_eq!(outcome.verdict, Verdict::Done);
    assert!(Arc::ptr_eq(&context, &pool.context()));
    assert_eq!(pool.max_seen(), 1);
    assert_eq!(context.notes.lock().unwrap().len(), before);
    let refused = final_gate(&pool, "verdict: blocked\nmissing file");
    assert_eq!(refused, Err(Verdict::Blocked));
    assert_eq!(pool.max_seen(), 1);
    assert_eq!(final_gate(&pool, "").unwrap_err(), Verdict::Reject);
}

#[test]
fn the_snippet_stops_at_four_thousand_chars() {
    let pool = Pool::new(WORKER_BOUND);
    let raw = format!("verdict: done\n{}", "x".repeat(5_000));
    let outcome = as_tool(&pool, &raw);
    assert_eq!(outcome.snippet.len(), SNIPPET_CHARS);
    assert!(outcome.accepted);
}

#[test]
fn the_demo_accepts() {
    assert_eq!(harness_core::critic::demo(), "accepted: done");
}
