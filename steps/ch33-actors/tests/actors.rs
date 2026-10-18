use actors::{fan, submit, FakeModel, Inbox, InboxError, Mail, INBOX_CAP};
use tokio::sync::mpsc;

#[test]
fn a_full_inbox_refuses_the_next_message() {
    let mut inbox = Inbox::new();
    for index in 0..INBOX_CAP {
        inbox
            .try_submit(Mail::Turn(format!("job {index}")))
            .unwrap();
    }
    assert_eq!(inbox.len(), INBOX_CAP);
    assert_eq!(
        inbox.try_submit(Mail::Turn("one more".into())),
        Err(InboxError::Full)
    );
    assert_eq!(inbox.len(), INBOX_CAP);
}

#[test]
fn a_steer_is_appended_and_an_interrupt_skips_the_model() {
    let mut inbox = Inbox::new();
    inbox.try_submit(Mail::Steer("  ".into())).unwrap();
    inbox.try_submit(Mail::Steer("also this".into())).unwrap();
    inbox.try_submit(Mail::Steer("and that".into())).unwrap();
    inbox.try_submit(Mail::Turn("job".into())).unwrap();
    let mut model = FakeModel::new("ok");
    assert_eq!(inbox.step(&mut model), None);
    assert_eq!(inbox.step(&mut model), None);
    assert_eq!(inbox.step(&mut model), None);
    assert_eq!(
        inbox.step(&mut model).as_deref(),
        Some("ok\nalso this\nand that")
    );
    assert_eq!(model.calls, 1);

    let mut stopped = Inbox::new();
    stopped.try_submit(Mail::Interrupt).unwrap();
    stopped.try_submit(Mail::Turn("job".into())).unwrap();
    let mut model = FakeModel::new("ok");
    assert_eq!(stopped.step(&mut model), None);
    assert_eq!(stopped.step(&mut model).as_deref(), Some("interrupted"));
    assert_eq!(model.calls, 0);
}

#[tokio::test]
async fn one_hundred_agents_each_answer_once() {
    let replies = fan(100).await;
    assert_eq!(replies.len(), 100);
    assert!(replies.contains(&"ok 0".to_string()));
    assert!(replies.contains(&"ok 99".to_string()));
}

#[tokio::test]
async fn the_channel_matches_the_inbox_cap() {
    let (tx, _rx) = mpsc::channel(INBOX_CAP);
    for _ in 0..INBOX_CAP {
        submit(&tx, Mail::Turn("hello".into())).unwrap();
    }
    assert_eq!(
        submit(&tx, Mail::Turn("overflow".into())),
        Err(InboxError::Full)
    );
}

#[test]
fn the_demo_answers() {
    assert_eq!(actors::demo(), "agent: ok");
}
