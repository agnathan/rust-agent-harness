use providers::run_turn;
use providers::{
    anthropic_body, bedrock_body, choose_next, close_interrupted, openrouter_route,
    provider_preferences, resolve_tls, side_model, uniquify_tool_call_ids, FakeModel, Msg, Pool,
    Prefs, ProviderError, TlsDecision, CLIENT_ID, INTERRUPTED_PLACEHOLDER,
};
use serde_json::json;

fn hello() -> Vec<Msg> {
    vec![Msg::new("user", "hi")]
}

#[test]
fn anthropic_and_bedrock_bodies_carry_the_fields_their_apis_require() {
    let anthropic = anthropic_body("claude", "be brief", &hello(), 128).expect("anthropic");
    assert_eq!(anthropic["model"], "claude");
    assert_eq!(anthropic["max_tokens"], 128);
    assert_eq!(anthropic["system"], "be brief");
    assert_eq!(anthropic["messages"][0]["role"], "user");
    assert!(anthropic_body("claude", "", &hello(), 0).is_err());

    let bedrock = bedrock_body("anthropic.claude", "", &hello(), 4096).expect("bedrock");
    assert_eq!(bedrock["modelId"], "anthropic.claude");
    assert_eq!(bedrock["inferenceConfig"]["maxTokens"], 4096);
    assert!(bedrock.get("system").is_none());
}

#[test]
fn an_invalid_sort_is_dropped_and_the_next_model_is_the_fallback() {
    let prefs = Prefs {
        only: vec!["anthropic".into()],
        ignore: vec![],
        order: vec!["anthropic".into(), "amazon".into()],
        sort: Some("Price".into()),
        require_parameters: false,
        data_collection: Some("deny".into()),
    };
    let map = provider_preferences(&prefs);
    assert_eq!(
        map.get("sort").map(|value| value.as_str()),
        Some(Some("price"))
    );
    assert!(map.get("require_parameters").is_none());
    assert_eq!(map["data_collection"], "deny");

    let bad = Prefs {
        sort: Some("fastest".into()),
        ..Prefs::default()
    };
    assert!(provider_preferences(&bad).get("sort").is_none());

    let models = vec!["primary".into(), "fallback".into()];
    let route = openrouter_route(&models, &prefs);
    assert_eq!(route["models"], json!(models));
    assert_eq!(choose_next(&models, Some("primary")), Some("fallback"));
    assert_eq!(choose_next(&models, None), Some("primary"));
}

#[test]
fn rotation_keeps_the_client_and_the_side_model_stays_off() {
    let mut pool = Pool::new(vec!["sk-one".into(), "sk-two".into()]);
    assert_eq!(pool.client_id(), CLIENT_ID);
    assert_eq!(pool.mark_exhausted_and_rotate(), "sk-two");
    assert_eq!(pool.client_id(), CLIENT_ID);
    assert_eq!(side_model(false, "primary", "side"), "primary");
    assert_eq!(side_model(true, "primary", "side"), "side");
}

#[test]
fn duplicate_tool_ids_and_a_dangling_tool_row_are_repaired() {
    let mut ids = vec!["call_0".into(), "call_0".into(), "call_1".into()];
    uniquify_tool_call_ids(&mut ids);
    assert_eq!(ids, vec!["call_0", "call_0_d2", "call_1"]);

    let mut messages = vec![Msg::new("assistant", ""), Msg::new("tool", "partial")];
    assert!(close_interrupted(&mut messages));
    assert_eq!(
        messages.last().expect("row").content,
        INTERRUPTED_PLACEHOLDER
    );
    assert!(!close_interrupted(&mut messages));
}

#[test]
fn tls_stays_on_the_os_store_unless_the_dev_flag_is_set() {
    assert_eq!(resolve_tls(true, false).expect("os"), TlsDecision::OsTrust);
    assert_eq!(
        resolve_tls(false, false).expect_err("refused"),
        ProviderError::InsecureTls
    );
    assert_eq!(
        resolve_tls(false, true).expect("dev"),
        TlsDecision::DevInsecure
    );
}

#[test]
fn the_fake_turn_uses_the_fallback_model() {
    let (reply, route) = run_turn(
        &[
            FakeModel {
                name: "primary".into(),
                reply: "no".into(),
            },
            FakeModel {
                name: "fallback".into(),
                reply: "pong".into(),
            },
        ],
        true,
    );
    assert_eq!(reply, "pong");
    assert_eq!(route, "fallback");
}
