use std::sync::Arc;

use gwservice::service::{
    bearer_ok, demo, elicit, Elicit, FakeModel, OsvCache, OsvVerdict, Reject, Sampler, Service,
    ELICIT_TIMEOUT_SECONDS, OSV_CACHE_TTL_SECONDS, SAMPLING_MAX_RPM,
};

#[test]
fn kill_moves_the_session_to_the_next_replica() {
    let mut service = Service::new(&["a", "b"], 4);
    assert_eq!(service.place("s1").as_deref(), Some("a"));
    let held = service.config();
    let reloaded = service.reload(2);
    assert_eq!(held.generation, 1);
    assert_eq!(reloaded.generation, 2);
    assert_eq!(Arc::strong_count(&held), 1);
    service.kill("a");
    assert_eq!(service.route("s1"), Some("b"));
    assert!(service.healthy());
    assert!(service.ready());
}

#[test]
fn quota_audit_and_drain() {
    let mut service = Service::new(&["a"], 1);
    assert!(bearer_ok("Bearer token"));
    assert!(!bearer_ok("token"));
    assert!(!bearer_ok("Bearer "));
    let id = service.accept("tenant", "s1", "Bearer token").unwrap();
    assert_eq!(
        service.accept("tenant", "s2", "Bearer token"),
        Err(Reject::Quota)
    );
    assert_eq!(
        service.accept("tenant", "s1", "nope"),
        Err(Reject::BadToken)
    );
    assert!(service
        .audit()
        .iter()
        .any(|line| line.starts_with("accept ")));
    assert_eq!(service.spans(), &["gateway.call 1".to_string()]);
    service.start_drain();
    assert!(!service.ready());
    assert_eq!(
        service.accept("other", "s3", "Bearer token"),
        Err(Reject::Draining)
    );
    assert!(!service.drained());
    assert!(service.finish(id, "tenant"));
    assert!(service.drained());
    assert_eq!(service.inflight(), 0);
    let _ = FakeModel { reply: "ok" }.complete("a");
}

#[test]
fn a_dead_pool_refuses_the_call() {
    let mut service = Service::new(&["a"], 4);
    service.kill("a");
    assert!(!service.healthy());
    assert_eq!(
        service.accept("tenant", "s1", "Bearer token"),
        Err(Reject::NoReplica)
    );
}

#[test]
fn sampling_stops_at_ten_per_minute() {
    let mut sampler = Sampler::new();
    for _ in 0..SAMPLING_MAX_RPM {
        assert!(sampler.allow(0));
    }
    assert!(!sampler.allow(0));
    assert!(sampler.allow(60));
}

#[test]
fn url_elicitation_is_declined() {
    assert_eq!(
        elicit(false),
        Elicit::Approval {
            timeout_seconds: ELICIT_TIMEOUT_SECONDS
        }
    );
    assert_eq!(elicit(true), Elicit::Declined);
}

#[test]
fn osv_blocks_malware_and_fails_open_on_the_network() {
    let mut cache = OsvCache::new();
    assert_eq!(
        cache.check("echo", "leftpad", 0, true, &["MAL-1"]),
        OsvVerdict::Skipped
    );
    assert!(cache.is_empty());
    assert_eq!(
        cache.check("npx", "leftpad", 0, true, &["CVE-2020-1"]),
        OsvVerdict::Clean
    );
    assert_eq!(
        cache.check("npx", "leftpad", 10, false, &["MAL-1"]),
        OsvVerdict::Clean
    );
    assert_eq!(
        cache.check("uvx", "bad", 0, false, &["MAL-9"]),
        OsvVerdict::FailOpen
    );
    assert_eq!(cache.len(), 1);
    assert_eq!(
        cache.check("uvx", "bad", 0, true, &["MAL-9", "CVE-1"]),
        OsvVerdict::Blocked("MAL-9".into())
    );
    assert_eq!(
        cache.check("uvx", "bad", OSV_CACHE_TTL_SECONDS - 1, true, &["CVE-1"]),
        OsvVerdict::Blocked("MAL-9".into())
    );
}

#[test]
fn demo_routes_to_the_first_replica() {
    assert_eq!(demo(), "route: a");
}
