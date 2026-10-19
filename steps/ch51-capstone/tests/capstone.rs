use gwservice::capstone::{
    admit, compare, demo, load, p99, sample_catalog, schema_tokens, task_arms, CALL_BOUND,
    CATALOG_LEN, STATIC_LEN,
};

#[test]
fn three_arms_share_a_success_and_not_a_token_count() {
    let catalog = sample_catalog();
    assert_eq!(catalog.len(), CATALOG_LEN);
    let one = schema_tokens(&catalog[0].schema);
    assert!(one > 0);
    let arms = compare(&catalog, "tool_07", None);
    assert_eq!(arms[0].name, "every");
    assert_eq!(arms[1].name, "static");
    assert_eq!(arms[2].name, "dynamic");
    assert_eq!(arms[0].offered, CATALOG_LEN);
    assert_eq!(arms[1].offered, STATIC_LEN);
    assert_eq!(arms[2].offered, 1);
    assert!(arms.iter().all(|arm| arm.success));
    assert!(arms[0].tokens > arms[1].tokens);
    assert!(arms[1].tokens > arms[2].tokens);
    assert_eq!(arms[0].tokens, one * CATALOG_LEN as u64);
    assert_eq!(arms[2].tokens, one);
    let later = compare(&catalog, "tool_31", None);
    assert!(later[0].success);
    assert!(!later[1].success);
    assert!(later[2].success);
    let absent = compare(&catalog, "missing", None);
    assert!(absent.iter().all(|arm| !arm.success));
}

#[test]
fn the_session_gateway_arms_each_report_tokens_and_success() {
    let tasks = task_arms(CATALOG_LEN);
    assert!(
        tasks
            .iter()
            .all(|task| task.success && task.schema_tokens > 0),
        "{tasks:?}"
    );
    assert!(tasks[0].schema_tokens >= tasks[2].schema_tokens);
}

#[test]
fn a_tight_deadline_fails_the_wide_arm_only() {
    let catalog = sample_catalog();
    let dynamic = compare(&catalog, "tool_07", None)[2].tokens;
    let arms = compare(&catalog, "tool_07", Some(dynamic));
    assert!(!arms[0].success);
    assert!(!arms[1].success);
    assert!(arms[2].success);
}

#[test]
fn the_load_stays_inside_the_bound_and_ranks_p99() {
    assert!(admit(CALL_BOUND - 1));
    assert!(!admit(CALL_BOUND));
    let samples: Vec<u128> = (1..=100).map(|n| n as u128).collect();
    assert_eq!(p99(samples), 99);
    let report = load(100);
    assert_eq!(report.successes, 100);
    assert_eq!(report.requests, 100);
    assert_eq!(report.waves, 13);
    assert!(report.p99_ns > 0);
}

#[test]
fn demo_reports_three_successes() {
    let line = demo();
    assert!(line.contains("success=1/1/1"), "{line}");
    assert!(line.contains("task=1/1/1"), "{line}");
    assert!(line.contains("load=100/100"), "{line}");
    assert!(line.contains("waves=13"), "{line}");
}
