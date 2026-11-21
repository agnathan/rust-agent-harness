use harness_core::approval::Presence;
use harness_core::sandbox::{check_execute_code, CodeDecision, EXECUTE_CODE_DESCRIPTION};

#[test]
fn a_local_script_is_denied_for_cron_and_approved_in_a_container() {
    let cron = check_execute_code("print(1)", "local", Presence::Cron, false, false, false);
    match cron {
        CodeDecision::Denied { message } => {
            assert!(message.starts_with("BLOCKED: execute_code runs arbitrary local Python"));
        }
        other => panic!("expected a denial, got {other:?}"),
    }
    assert_eq!(
        check_execute_code("print(1)", "container", Presence::Cron, false, false, false),
        CodeDecision::Approved
    );
    assert_eq!(
        check_execute_code(
            "print(1)",
            "local",
            Presence::Interactive,
            true,
            false,
            false
        ),
        CodeDecision::Approved
    );
    match check_execute_code(
        "print(1)",
        "local",
        Presence::Interactive,
        false,
        false,
        true,
    ) {
        CodeDecision::Ask { description } => assert_eq!(description, EXECUTE_CODE_DESCRIPTION),
        other => panic!("expected one whole-script ask, got {other:?}"),
    }
}
