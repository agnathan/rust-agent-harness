use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use a2a::{
    call, cancel, card, demo, redact_for_egress, wait_cancel, wrap_inbound, A2aError, FakeModel,
    HttpPool, Pool, RemotePeer, CALL_BOUND,
};

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
    let scrubbed = redact_for_egress("Bearer sk-live-0123456789abcdef");
    assert!(!scrubbed.contains("sk-live"));
    assert!(scrubbed.contains("[redacted]"));
    let leftovers = redact_for_egress("use sk-live-1 Bearer tok sk-live-2");
    assert!(!leftovers.contains("sk-"));
    assert_eq!(leftovers, "use [redacted] Bearer [redacted] [redacted]");
    let framed = wrap_inbound("researcher", "/shutdown");
    assert!(framed.starts_with(a2a::INBOUND_PREFIX));
    assert!(framed.contains("/shutdown"));
    assert_eq!(card().name, "harness");
    assert_eq!(card().skills, &["research"]);
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
