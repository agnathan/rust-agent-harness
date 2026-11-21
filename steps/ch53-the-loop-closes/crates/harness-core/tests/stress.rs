use harness_core::{prepare_live_evals, run_stress_evals, stress_eval_names};

#[tokio::test]
async fn the_stress_set_passes_on_the_mock() {
    let scores = run_stress_evals().await;
    assert_eq!(scores.len(), stress_eval_names().len());
    assert!(scores.iter().all(|score| score.passed), "{scores:?}");
    let names: Vec<_> = scores.iter().map(|score| score.name).collect();
    assert_eq!(
        names,
        vec![
            "tool-fails-then-recovers",
            "hits-iteration-cap",
            "unknown-tool-repair",
            "stall"
        ]
    );
}

#[test]
fn live_evals_without_a_key_do_not_build_a_client() {
    let error = prepare_live_evals(None).unwrap_err();
    assert_eq!(
        error,
        "missing OPENROUTER_API_KEY: pass --api-key, or run with --mock"
    );
    assert!(prepare_live_evals(Some("")).is_err());
    assert!(prepare_live_evals(Some("present")).is_ok());
}
