use harness::l13_actor::*;
#[tokio::test]
async fn ask_and_steer() {
    let (h, join) = spawn_agent("a1", 8, |text, steers| {
        format!("{text}|{}", steers.join(","))
    });
    assert_eq!(h.ask("hi").await, Ok("hi|".into()));
    h.try_steer("be brief").unwrap();
    h.try_steer("use rust").unwrap();
    assert_eq!(h.ask("q").await, Ok("q|be brief,use rust".into()));
    assert_eq!(h.ask("q2").await, Ok("q2|".into()), "steers are consumed");
    h.stop().await;
    assert_eq!(join.await.unwrap(), 3);
    assert_eq!(h.ask("late").await, Err(SendError::Gone));
}
#[tokio::test(flavor = "current_thread")]
async fn backpressure() {
    // On a current_thread runtime the actor can't run until we .await, so the inbox fills up.
    let (h, _join) = spawn_agent("a2", 1, |t, _| t.to_string());
    assert_eq!(h.try_steer("1"), Ok(()));
    assert_eq!(h.try_steer("2"), Err(SendError::Busy));
}
