use harness_core::suite::{
    append_event, bound_refuses_the_ninth, demo, record_saved, run_red_team, run_saved, run_stress,
    run_trajectories, saved_names, stress_names, FAIL_THRESHOLD, WORKER_BOUND,
};

#[test]
fn the_saved_set_and_the_stress_set_pass_on_the_mock() {
    let saved = run_saved();
    assert_eq!(
        saved.iter().map(|score| score.name).collect::<Vec<_>>(),
        saved_names()
    );
    assert!(saved.iter().all(|score| score.passed), "{saved:?}");
    let stress = run_stress();
    assert_eq!(
        stress.iter().map(|score| score.name).collect::<Vec<_>>(),
        stress_names()
    );
    assert!(stress.iter().all(|score| score.passed), "{stress:?}");
}

#[test]
fn one_red_team_case_per_layer_and_no_threshold() {
    let scores = run_red_team();
    let names: Vec<_> = scores.iter().map(|score| score.name).collect();
    assert_eq!(
        names,
        vec!["L1", "L2", "L3", "L4", "L5", "L6", "L7", "L8", "L9", "X"]
    );
    assert!(scores.iter().all(|score| score.passed), "{scores:?}");
    assert!(FAIL_THRESHOLD.is_none());
}

#[test]
fn one_hundred_trajectories_stay_inside_the_worker_bound() {
    let (count, peak) = run_trajectories(100);
    assert_eq!(count, 100);
    assert!(peak > 0 && peak <= WORKER_BOUND, "peak {peak}");
    assert!(bound_refuses_the_ninth());
}

#[test]
fn an_appended_event_is_one_json_object_in_saved_order() {
    let mut log = Vec::new();
    append_event(&mut log, "echo-hello", true);
    assert_eq!(
        log,
        vec![r#"{"name":"echo-hello","passed":true}"#.to_string()]
    );
    let mut recorded = Vec::new();
    let scores = record_saved(&mut recorded);
    assert!(scores.iter().all(|score| score.passed), "{scores:?}");
    assert_eq!(recorded.len(), scores.len());
    assert!(recorded[0].contains("echo-hello"), "{recorded:?}");
    assert!(recorded[1].contains("direct"), "{recorded:?}");
    assert!(recorded[2].contains("eight-reads"), "{recorded:?}");
}

#[test]
fn demo_lists_passes_and_the_peak() {
    let text = demo();
    assert!(text.contains("eval: echo-hello pass"), "{text}");
    assert!(text.contains("eval: unknown-tool-repair pass"), "{text}");
    assert!(text.contains("eval: L5 pass"), "{text}");
    assert!(text.contains("eval: X pass"), "{text}");
    assert!(text.contains("threshold: None"), "{text}");
    assert!(
        text.contains("trajectories: 100 within_bound: true"),
        "{text}"
    );
}
