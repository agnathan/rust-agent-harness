use gw_learned::{
    demo, predict, CodeError, LearnedGateway, PredictInput, ScriptedModel, Use,
    CODE_MODE_MAX_CALLS, SANDBOX_ALLOWED,
};

fn catalog() -> Vec<String> {
    ["read_file", "web_search", "write_file", "terminal"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

#[test]
fn prediction_changes_the_offered_set_and_is_not_on_the_read() {
    let gateway = LearnedGateway::new();
    gateway.open_session("desk");
    let empty = gateway.session_tools("desk").expect("session");
    assert!(empty.tools.is_empty());
    assert!(!empty.list_changed);

    let usage = vec![
        Use {
            task: "read the note".into(),
            tool: "read_file".into(),
        },
        Use {
            task: "read the note".into(),
            tool: "read_file".into(),
        },
    ];
    let names = catalog();
    let input = PredictInput {
        task: "read the note",
        usage: &usage,
        memory: "the user keeps notes in files",
        session_note: "last time we called web_search",
        catalog: &names,
    };
    let ranked = gateway.refresh_prediction(&input);
    assert_eq!(ranked.as_slice(), ["web_search", "read_file"]);
    let calls = gateway.predict_calls();
    let view = gateway.session_tools("desk").expect("session");
    assert_eq!(
        view.tools,
        ["web_search".to_string(), "read_file".to_string()]
    );
    assert!(view.list_changed);
    assert_eq!(gateway.predict_calls(), calls);
    let again = gateway.session_tools("desk").expect("session");
    assert!(!again.list_changed);
    assert_eq!(again.tools, view.tools);
}

#[test]
fn a_zero_score_tool_is_left_out() {
    let names = catalog();
    let ranked = predict(&PredictInput {
        task: "unrelated",
        usage: &[],
        memory: "",
        session_note: "",
        catalog: &names,
    });
    assert!(ranked.is_empty());
}

#[test]
fn composite_and_macro_expand_to_steps() {
    let gateway = LearnedGateway::new();
    gateway.save_composite("rename_flow", vec!["read_file".into(), "write_file".into()]);
    gateway.save_macro("morning", vec!["web_search".into(), "read_file".into()]);
    assert_eq!(
        gateway.expand("rename_flow").expect("composite"),
        ["read_file", "write_file"]
    );
    assert_eq!(
        gateway.expand("morning").expect("macro"),
        ["web_search", "read_file"]
    );
    assert!(gateway.expand("missing").is_none());
}

#[test]
fn code_mode_refuses_a_tool_outside_the_allowlist_or_the_offer() {
    let gateway = LearnedGateway::new();
    let offered = vec!["read_file".to_string()];
    let denied = gateway.run_code_mode("call memory {}\n", &offered, |_, _| "no".into());
    assert_eq!(denied, Err(CodeError::NotAllowed("memory".into())));
    let hidden = gateway.run_code_mode("call web_search {}\n", &offered, |_, _| "no".into());
    assert_eq!(hidden, Err(CodeError::NotOffered("web_search".into())));
    let mut calls = 0;
    let ok = gateway
        .run_code_mode(
            "call read_file {\"path\":\"a\"}\n",
            &offered,
            |tool, args| {
                calls += 1;
                assert_eq!(tool, "read_file");
                assert_eq!(args, "{\"path\":\"a\"}");
                "ok".into()
            },
        )
        .expect("run");
    assert_eq!(ok, ["ok"]);
    assert_eq!(calls, 1);
    assert!(SANDBOX_ALLOWED.contains(&"terminal"));
    assert!(!SANDBOX_ALLOWED.contains(&"memory"));
}

#[test]
fn code_mode_stops_at_fifty_calls() {
    let gateway = LearnedGateway::new();
    let offered = vec!["read_file".to_string()];
    let script = "call read_file {}\n".repeat(CODE_MODE_MAX_CALLS + 1);
    let result = gateway.run_code_mode(&script, &offered, |_, _| "ok".into());
    assert_eq!(result, Err(CodeError::TooManyCalls));
}

#[test]
fn the_scripted_model_is_optional_and_keyless() {
    let mut model = ScriptedModel::new("unused");
    assert_eq!(model.calls(), 0);
    let (offered, body) = demo();
    assert_eq!(offered, "read_file");
    assert_eq!(body, "ok");
    assert_eq!(model.complete("predict"), "unused");
    assert_eq!(model.calls(), 1);
}
