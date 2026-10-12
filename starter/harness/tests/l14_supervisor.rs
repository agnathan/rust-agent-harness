use harness::l14_supervisor::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
#[tokio::test]
async fn cancel_work() {
    let t = tokio_util::sync::CancellationToken::new();
    assert_eq!(cancellable_work(t.clone(), 10).await, Ok("done"));
    let t2 = t.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        t2.cancel();
    });
    let start = Instant::now();
    assert_eq!(cancellable_work(t, 5_000).await, Err("cancelled"));
    assert!(start.elapsed() < Duration::from_secs(1));
}
#[test]
fn budget_is_atomic() {
    let b = Arc::new(Budget::new(1000));
    let hs: Vec<_> = (0..8)
        .map(|_| {
            let b = b.clone();
            std::thread::spawn(move || (0..200).filter(|_| b.try_consume()).count())
        })
        .collect();
    let total: usize = hs.into_iter().map(|h| h.join().unwrap()).sum();
    assert_eq!(total, 1000);
    assert_eq!(b.remaining(), 0);
}
#[test]
fn depth_limits_and_tree() {
    let root = Supervisor::new(1, 2, 10);
    let child = root.child().unwrap();
    assert_eq!(child.depth, 1);
    assert_eq!(child.child().err().unwrap(), "max depth");
    child.budget().try_consume();
    assert_eq!(root.budget().remaining(), 9, "budget is shared");
    root.cancel_all();
    assert!(
        child.token().is_cancelled(),
        "cancelling the parent cancels the child"
    );
}
#[tokio::test]
async fn delegate_outcomes() {
    let sup = Supervisor::new(1, 2, 3);
    let tasks = vec![
        "ok".to_string(),
        "fail".into(),
        "boom".into(),
        "extra".into(),
    ];
    let out = sup
        .delegate(tasks, |t, _tok| async move {
            match t.as_str() {
                "fail" => Err("nope".to_string()),
                "boom" => panic!("child crashed"),
                _ => Ok(format!("did {t}")),
            }
        })
        .await;
    assert_eq!(
        out[0],
        ChildOutcome::Done {
            task: "ok".into(),
            summary: "did ok".into()
        }
    );
    assert_eq!(
        out[1],
        ChildOutcome::Failed {
            task: "fail".into(),
            error: "nope".into()
        }
    );
    assert_eq!(
        out[2],
        ChildOutcome::Failed {
            task: "boom".into(),
            error: "panic".into()
        }
    );
    assert_eq!(
        out[3],
        ChildOutcome::Failed {
            task: "extra".into(),
            error: "budget exhausted".into()
        }
    );
}
#[tokio::test]
async fn delegate_cancel() {
    let sup = Arc::new(Supervisor::new(1, 4, 10));
    let s2 = sup.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        s2.cancel_all();
    });
    let start = Instant::now();
    let out = sup
        .delegate(vec!["slow".into(), "slow2".into()], |t, _tok| async move {
            tokio::time::sleep(Duration::from_secs(10)).await;
            Ok(t)
        })
        .await;
    assert!(start.elapsed() < Duration::from_secs(2));
    assert!(out
        .iter()
        .all(|o| matches!(o, ChildOutcome::Cancelled { .. })));
}
