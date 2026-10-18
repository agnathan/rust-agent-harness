use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use harness_core::a2a::{
    call, cancel, card, demo, wait_cancel, wrap_inbound, A2aError, FakeModel, HttpPool, Pool,
    RemotePeer, CALL_BOUND,
};
use harness_core::redact_for_egress;

fn peer(url: &'static str) -> RemotePeer {
    RemotePeer {
        name: "researcher",
        url,
        http: Arc::new(HttpPool::default()),
        model: FakeModel { reply: "ok" },
    }
}

#[test]
fn an_in_process_url_is_not_a2a() {
    let pool = Pool::new();
    let local = peer("local://supervisor");
    assert_eq!(
        call(&pool, &local, "hello", &AtomicBool::new(false)),
        Err(A2aError::InProcess)
    );
    assert_eq!(pool.in_flight(), 0);
    assert_eq!(local.http.calls(), 0);
}

#[test]
fn the_ninth_call_is_refused() {
    let pool = Pool::new();
    let remote = peer("http://research.example:9900");
    for _ in 0..CALL_BOUND {
        assert!(pool.hold());
    }
    assert_eq!(
        call(&pool, &remote, "hello", &AtomicBool::new(false)),
        Err(A2aError::Bound)
    );
    pool.release_one();
    assert_eq!(
        call(&pool, &remote, "hello", &AtomicBool::new(false)).unwrap(),
        "ok"
    );
    assert!(!remote.http.urls.lock().unwrap()[0].starts_with("local"));
    for _ in 1..CALL_BOUND {
        pool.release_one();
    }
    assert_eq!(pool.in_flight(), 0);
}

#[test]
fn cancel_stops_an_in_flight_wait() {
    let flag = Arc::new(AtomicBool::new(false));
    let started = Arc::new(AtomicBool::new(false));
    let flag_worker = Arc::clone(&flag);
    let started_worker = Arc::clone(&started);
    let handle = thread::spawn(move || wait_cancel(&flag_worker, &started_worker));
    while !started.load(Ordering::SeqCst) {
        thread::yield_now();
    }
    cancel(&flag);
    handle.join().unwrap();
    assert!(flag.load(Ordering::SeqCst));
}

#[test]
fn egress_and_inbound_are_wrapped() {
    let key = redact_for_egress("Bearer sk-live-0123456789abcdef");
    assert!(!key.contains("sk-live"), "{key}");
    assert!(key.contains("sk-***"), "{key}");
    let bearer = redact_for_egress("Bearer abcdefghijklmnopqrstuvwxyz012345");
    assert!(bearer.contains("Bearer [redacted]"), "{bearer}");
    assert!(
        !bearer.contains("abcdefghijklmnopqrstuvwxyz012345"),
        "{bearer}"
    );
    let assigned = redact_for_egress("OPENAI_API_KEY=supersecret");
    assert!(!assigned.contains("supersecret"), "{assigned}");
    assert!(assigned.contains("OPENAI_API_KEY=***"), "{assigned}");
    let framed = wrap_inbound("researcher", "/shutdown");
    assert!(framed.starts_with(harness_core::a2a::INBOUND_PREFIX));
    assert!(framed.contains("/shutdown"));
    assert_eq!(card().name, "harness");
    assert_eq!(card().skills, &["research"]);

    let pool = Pool::new();
    let remote = peer("https://research.example/a2a");
    let reply = call(
        &pool,
        &remote,
        "see sk-live-0123456789abcdef",
        &AtomicBool::new(false),
    )
    .unwrap();
    assert_eq!(reply, "ok");
    let body = remote.http.bodies.lock().unwrap()[0].clone();
    assert!(!body.contains("sk-live"), "{body}");
    assert!(body.contains("sk-***"), "{body}");
}

#[test]
fn a_cancelled_flag_never_records_the_url() {
    let pool = Pool::new();
    let remote = peer("http://research.example:9900");
    let error = call(&pool, &remote, "hello", &AtomicBool::new(true)).unwrap_err();
    assert_eq!(error, A2aError::Cancelled);
    assert_eq!(remote.http.calls(), 0);
    assert_eq!(pool.in_flight(), 0);
}

#[test]
fn demo_needs_no_key() {
    assert_eq!(demo(), "a2a: ok");
}
