use gateway::{
    authorize, demo, fold, handoff_request, is_engaged, BotAdmit, BotGuard, Chunk, Handoff, Inbox,
    Message, PairError, Pairing, Stat, Submit, BOT_COOLDOWN_SECONDS, BOT_MAX_EVENTS, CODE_LENGTH,
    CODE_TTL_SECONDS, INBOX_CAP, LOCKOUT_SECONDS, MAX_FAILED_ATTEMPTS, MAX_PENDING_PER_PLATFORM,
    RATE_LIMIT_SECONDS, SESSION_BUSY,
};

const CODE: &str = "ABCDEFGH";

#[test]
fn strangers_are_denied_and_allowlisted_or_paired_are_not() {
    assert!(!authorize(&["owner"], false, "stranger"));
    assert!(authorize(&["owner"], false, "owner"));
    assert!(authorize(&[], true, "stranger"));
    assert!(!authorize(&[], false, "stranger"));
}

#[test]
fn pairing_code_rules() {
    assert_eq!(CODE.len(), CODE_LENGTH);
    let mut pairing = Pairing::new();
    assert!(pairing.request(0, "IIIIIIII").is_err());
    pairing.request(0, CODE).unwrap();
    assert_eq!(
        pairing.request(RATE_LIMIT_SECONDS - 1, "ABCD2345"),
        Err(PairError::RateLimited)
    );
    pairing.request(RATE_LIMIT_SECONDS, "ABCD2345").unwrap();
    pairing.request(RATE_LIMIT_SECONDS * 2, "ZZZZZZZ2").unwrap();
    assert_eq!(pairing.pending_len(), MAX_PENDING_PER_PLATFORM);
    assert_eq!(
        pairing.request(RATE_LIMIT_SECONDS * 3, "YYYYYYY2"),
        Err(PairError::TooManyPending)
    );
    pairing.approve(0, "abcd efgh").unwrap();
    assert!(pairing.paired());
}

#[test]
fn expired_code_and_five_failures_lock_the_platform() {
    let mut pairing = Pairing::new();
    pairing.request(0, CODE).unwrap();
    assert_eq!(
        pairing.approve(CODE_TTL_SECONDS + 1, CODE),
        Err(PairError::Unknown)
    );
    assert!(!pairing.paired());
    let mut pairing = Pairing::new();
    for n in 0..MAX_FAILED_ATTEMPTS {
        assert_eq!(
            pairing.approve(n as u64, "NOPE2345"),
            Err(PairError::Unknown)
        );
    }
    assert_eq!(pairing.request(10, CODE), Err(PairError::Locked));
    assert_eq!(
        pairing.approve(LOCKOUT_SECONDS + 10, CODE),
        Err(PairError::Unknown)
    );
}

#[test]
fn twenty_bot_messages_then_a_cooldown() {
    let mut guard = BotGuard::new();
    for _ in 0..BOT_MAX_EVENTS {
        assert_eq!(guard.admit(0), BotAdmit::Ok);
    }
    assert_eq!(guard.admit(0), BotAdmit::Tripped);
    assert_eq!(guard.admit(BOT_COOLDOWN_SECONDS - 1), BotAdmit::Cooldown);
    assert_eq!(guard.admit(BOT_COOLDOWN_SECONDS), BotAdmit::Ok);
}

#[test]
fn estop_fails_safe() {
    assert!(is_engaged(&[Stat::Present]));
    assert!(is_engaged(&[Stat::Absent, Stat::Error]));
    assert!(!is_engaged(&[Stat::Absent, Stat::Absent]));
    assert!(is_engaged(&[Stat::Error, Stat::Present]));
}

#[test]
fn inbox_holds_a_steer_and_folds_text() {
    let mut inbox = Inbox::new(INBOX_CAP);
    assert_eq!(
        inbox.submit(Message::Steer("look".into())),
        Submit::Accepted
    );
    assert_eq!(inbox.front(), Some(&Message::Steer("look".into())));
    for n in 0..(INBOX_CAP - 1) {
        assert_eq!(inbox.submit(Message::Turn(n.to_string())), Submit::Accepted);
    }
    assert_eq!(inbox.submit(Message::Turn("more".into())), Submit::Full);
    assert_eq!(inbox.len(), INBOX_CAP);
    let text = fold(&[
        Chunk::Text("hello".into()),
        Chunk::Tool("nope".into()),
        Chunk::Text("!".into()),
    ]);
    assert_eq!(text, "hello!");
}

#[test]
fn a_busy_session_rejects_the_rebind() {
    assert_eq!(
        handoff_request(true, "telegram", &["telegram"]),
        Handoff::Busy { code: SESSION_BUSY }
    );
    assert!(matches!(
        handoff_request(false, "  ", &["telegram"]),
        Handoff::PlatformRequired { .. }
    ));
    assert!(matches!(
        handoff_request(false, "slack", &["telegram"]),
        Handoff::UnknownPlatform { .. }
    ));
    assert_eq!(
        handoff_request(false, " Telegram ", &["telegram"]),
        Handoff::Pending {
            platform: "telegram".into()
        }
    );
}

#[test]
fn demo_queues_and_denies_a_stranger() {
    assert_eq!(demo(), "gateway: queued");
}
