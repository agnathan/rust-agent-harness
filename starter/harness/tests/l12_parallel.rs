use harness::l12_parallel::*;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;
use Concurrency::*;
#[test]
fn planning() {
    assert_eq!(
        plan_batches(&[ReadOnly, ReadOnly, ReadOnly]),
        vec![vec![0, 1, 2]]
    );
    assert_eq!(
        plan_batches(&[ReadOnly, Exclusive, ReadOnly]),
        vec![vec![0], vec![1], vec![2]]
    );
    let a = || PathScoped("a.rs".into());
    assert_eq!(
        plan_batches(&[a(), PathScoped("b.rs".into()), a(), ReadOnly]),
        vec![vec![0, 1], vec![2, 3]]
    );
    assert!(plan_batches(&[]).is_empty());
}
fn reg() -> AsyncRegistry {
    let mut r = AsyncRegistry::new();
    for n in ["s1", "s2", "s3", "s4"] {
        r.insert(
            n.into(),
            Arc::new(SleepTool {
                name: n.into(),
                ms: 100,
            }),
        );
    }
    r
}
fn calls() -> Vec<(String, Value)> {
    ["s1", "s2", "s3", "s4"]
        .iter()
        .map(|n| (n.to_string(), json!({})))
        .collect()
}
#[tokio::test]
async fn parallel_respects_limit() {
    let t = Instant::now();
    let r = execute_parallel(&reg(), calls(), 4).await;
    assert!(t.elapsed().as_millis() < 250);
    assert_eq!(
        r,
        vec![
            Ok("s1".into()),
            Ok("s2".into()),
            Ok("s3".into()),
            Ok("s4".into())
        ]
    );
    let t = Instant::now();
    execute_parallel(&reg(), calls(), 1).await;
    assert!(t.elapsed().as_millis() >= 400);
}
#[tokio::test]
async fn planned_with_unknown() {
    let mut c = calls();
    c.insert(1, ("ghost".into(), json!({})));
    let r = execute_planned(&reg(), c, 8).await;
    assert_eq!(r[1], Err("unknown tool: ghost".into()));
    assert_eq!(r[4], Ok("s4".into()));
    assert_eq!(r.len(), 5);
}
