use harness_core::{InboxError, SessionInbox, INBOX_BOUND};

#[test]
fn submit_then_pop_is_fifo() {
    let inbox = SessionInbox::new(INBOX_BOUND);
    inbox.submit("first").unwrap();
    inbox.submit("second").unwrap();
    assert_eq!(inbox.len(), 2);
    assert_eq!(inbox.pop().as_deref(), Some("first"));
    assert_eq!(inbox.pop().as_deref(), Some("second"));
    assert!(inbox.is_empty());
    assert!(inbox.pop().is_none());
}

#[test]
fn a_full_inbox_refuses_the_next_turn() {
    let inbox = SessionInbox::new(2);
    inbox.submit("one").unwrap();
    inbox.submit("two").unwrap();
    let error = inbox.submit("three").unwrap_err();
    assert_eq!(error, InboxError::Full { bound: 2 });
    assert_eq!(inbox.len(), 2);
    assert!(error.to_string().contains("full"));
}
