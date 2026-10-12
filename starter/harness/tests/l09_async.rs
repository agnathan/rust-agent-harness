use harness::l09_async::*;
use std::time::Instant;
#[tokio::test]
async fn sequential_vs_concurrent() {
    let jobs = [("a", 100), ("b", 100), ("c", 100)];
    let t = Instant::now();
    let r = run_sequential(&jobs).await;
    assert!(t.elapsed().as_millis() >= 300);
    assert_eq!(r, vec!["a done", "b done", "c done"]);
    let t = Instant::now();
    let r = run_concurrent(&jobs).await;
    assert!(t.elapsed().as_millis() < 250, "should overlap");
    assert_eq!(r, vec!["a done", "b done", "c done"]);
}
#[tokio::test]
async fn timeouts() {
    assert_eq!(with_timeout(10, 200).await, Ok("job done".into()));
    assert_eq!(with_timeout(500, 50).await, Err("timeout"));
}
#[tokio::test]
async fn spawning() {
    assert_eq!(spawn_and_sum(10).await, 285);
}
