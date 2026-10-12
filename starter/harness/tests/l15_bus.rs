use harness::l15_bus::*;
fn env(to: Address, body: &str) -> Envelope {
    Envelope {
        from: "t".into(),
        to,
        corr_id: 7,
        body: body.into(),
    }
}
#[tokio::test]
async fn direct_and_topics() {
    let r = Router::new();
    let mut inbox = r.register("w1", 4);
    assert_eq!(r.send(env(Address::Agent("w1".into()), "hi")).await, Ok(1));
    assert_eq!(inbox.recv().await.unwrap().body, "hi");
    assert_eq!(
        r.send(env(Address::Agent("ghost".into()), "x")).await,
        Err(RouteError::UnknownAgent("ghost".into()))
    );
    drop(inbox);
    assert_eq!(
        r.send(env(Address::Agent("w1".into()), "x")).await,
        Err(RouteError::AgentGone("w1".into()))
    );
    let mut s1 = r.subscribe("news");
    let mut s2 = r.subscribe("news");
    assert_eq!(r.send(env(Address::Topic("news".into()), "n")).await, Ok(2));
    assert_eq!(s1.recv().await.unwrap().body, "n");
    assert_eq!(s2.recv().await.unwrap().body, "n");
    assert_eq!(
        r.send(env(Address::Topic("empty".into()), "n")).await,
        Ok(0)
    );
}
#[test]
fn blackboard() {
    let b = Blackboard::default();
    assert_eq!(b.add("a"), 1);
    assert_eq!(b.add("b"), 2);
    let t = b.claim("w1").unwrap();
    assert_eq!(t.id, 1);
    assert_eq!(t.status, TaskStatus::Claimed("w1".into()));
    assert_eq!(b.claim("w2").unwrap().id, 2);
    assert!(b.claim("w3").is_none());
    assert_eq!(b.complete(1, "ok"), Ok(()));
    assert_eq!(b.complete(1, "again"), Err("not claimed".into()));
    assert_eq!(b.complete(9, "x"), Err("no such task".into()));
    assert_eq!(b.snapshot()[0].status, TaskStatus::Done("ok".into()));
}
#[tokio::test]
async fn team() {
    let r = Router::new();
    let out = orchestrate(
        r,
        &["alice", "bob"],
        vec!["t0".into(), "t1".into(), "t2".into()],
    )
    .await;
    assert_eq!(out, vec!["alice:t0", "bob:t1", "alice:t2"]);
}
