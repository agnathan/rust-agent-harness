use harness_core::acp::{
    demo, log_line, map_permission, stop_reason, translate, write_result, Approval, Inbox, Mail,
    Streams, Submit, Update, INBOX_CAP,
};

#[test]
fn cancel_and_end_turn_are_different_reasons() {
    assert_eq!(stop_reason(false).as_str(), "end_turn");
    assert_eq!(stop_reason(true).as_str(), "cancelled");
}

#[test]
fn logs_stay_off_stdout() {
    let mut streams = Streams::default();
    log_line(&mut streams, "session started");
    write_result(&mut streams, stop_reason(false));
    assert_eq!(streams.stderr, vec!["session started".to_string()]);
    assert_eq!(streams.stdout.len(), 1);
    assert!(streams.stdout[0].contains("end_turn"));
    assert!(!streams.stdout[0].contains("session started"));
    write_result(&mut streams, stop_reason(true));
    assert!(streams.stdout[1].contains("cancelled"));
}

#[test]
fn unknown_permissions_deny() {
    assert_eq!(map_permission("allow_once"), Approval::Once);
    assert_eq!(map_permission("allow_session"), Approval::Session);
    assert_eq!(map_permission("allow_always"), Approval::Always);
    assert_eq!(map_permission("deny"), Approval::Deny);
    assert_eq!(map_permission("deny_always"), Approval::Deny);
    assert_eq!(map_permission("allow_forever"), Approval::Deny);
}

#[test]
fn an_update_lands_on_the_inbox() {
    let mut inbox = Inbox::new(INBOX_CAP);
    let mail = translate(&Update {
        text: "hello".into(),
        cancelled: false,
    });
    assert_eq!(
        mail,
        Mail {
            text: "hello".into()
        }
    );
    assert_eq!(inbox.submit(mail), Submit::Accepted);
    assert_eq!(inbox.front().map(|mail| mail.text.as_str()), Some("hello"));
    for n in 0..(INBOX_CAP - 1) {
        assert_eq!(
            inbox.submit(Mail {
                text: n.to_string()
            }),
            Submit::Accepted
        );
    }
    assert_eq!(
        inbox.submit(Mail {
            text: "more".into()
        }),
        Submit::Full
    );
    assert_eq!(inbox.len(), INBOX_CAP);
    assert!(!inbox.is_empty());
}

#[test]
fn demo_ends_the_turn() {
    assert_eq!(demo(), "acp: end_turn");
}
