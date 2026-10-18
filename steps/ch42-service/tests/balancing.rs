use std::time::Duration;

use gwservice::federation::{
    discover, federate, pool_for, Backend, Policy, Pool, Upstream, DISCOVERY_CONCURRENCY,
};

#[test]
fn least_outstanding_picks_the_quietest_backend() {
    let pool = Pool::new(vec![
        Backend::new("a", "echo", 3),
        Backend::new("b", "echo", 1),
        Backend::new("c", "echo", 4),
        Backend::new("d", "echo", 2),
    ]);
    assert_eq!(pool.pick(Policy::LeastOutstanding), Some(1));
    assert_eq!(
        pool.call(Policy::LeastOutstanding).as_deref(),
        Some("echo via b")
    );
    assert_eq!(pool.pick(Policy::LeastOutstanding), Some(1));
}

#[test]
fn power_of_two_picks_the_quieter_of_the_pair() {
    let pool = Pool::new(vec![
        Backend::new("a", "echo", 5),
        Backend::new("b", "echo", 1),
        Backend::new("c", "echo", 4),
    ]);
    assert_eq!(pool.pick(Policy::PowerOfTwo { a: 0, b: 1 }), Some(1));
    assert_eq!(pool.pick(Policy::PowerOfTwo { a: 2, b: 0 }), Some(2));
    assert!(pool.pick(Policy::LeastOutstanding).is_some());
    assert!(Pool::new(vec![]).pick(Policy::LeastOutstanding).is_none());
}

#[test]
fn federation_groups_a_tool_across_upstreams() {
    let upstreams = [
        Upstream {
            server: "alpha".into(),
            tools: vec!["echo".into(), "time".into()],
        },
        Upstream {
            server: "beta".into(),
            tools: vec!["echo".into()],
        },
    ];
    let map = federate(&upstreams);
    assert_eq!(
        map.get("echo").map(Vec::as_slice),
        Some(["alpha".to_string(), "beta".to_string()].as_slice())
    );
    let pool = pool_for("echo", &upstreams);
    assert_eq!(pool.len(), 2);
    assert_eq!(
        pool.call(Policy::LeastOutstanding).as_deref(),
        Some("echo via alpha")
    );
    assert!(pool_for("missing", &upstreams).is_empty());
}

#[test]
fn discovery_stays_at_four() {
    let seen = discover(6, Duration::from_millis(40));
    assert_eq!(seen, DISCOVERY_CONCURRENCY);
    assert_eq!(DISCOVERY_CONCURRENCY, 4);
}

#[test]
fn the_demo_picks_alpha() {
    assert_eq!(gwservice::federation::demo(), "pick: echo via alpha");
}
