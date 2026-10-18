use gw_limits::session::{
    page_text, run_rename_task, synthetic, Arm, Load, PluginSet, SessionError, SessionGateway,
    ToolCard, Upstream, MAX_DESCRIBE_NAMES, MAX_QUERIES_PER_CALL, PAGE_CHARS,
};

fn card(name: &str, description: &str, server: &str) -> ToolCard {
    ToolCard {
        name: name.into(),
        description: description.into(),
        server: server.into(),
    }
}

#[test]
fn search_reveals_names_and_describe_is_capped() {
    let gateway = SessionGateway::new();
    gateway
        .reload(Load {
            native: vec![
                card("files_rename_file", "Rename a file on disk.", "files"),
                card("mail_search_message", "Search a mail message.", "mail"),
            ],
            plugins: vec![PluginSet {
                plugin: "notes".into(),
                tools: vec![card("notes_read_note", "Read a note.", "notes")],
            }],
            upstreams: vec![Upstream {
                name: "ops".into(),
                tools: vec![card("ops_list_job", "List a job.", "ops")],
            }],
        })
        .unwrap();
    gateway.open_session("s");
    assert!(gateway.session_tools("s").unwrap().tools.is_empty());
    let listing = gateway
        .search_tools("s", &["rename a file".into()])
        .unwrap();
    assert!(listing.contains("files_rename_file"));
    assert!(!listing.contains("server:"));
    assert!(gateway.take_list_changed("s").unwrap());
    assert!(!gateway.take_list_changed("s").unwrap());
    let full = gateway
        .describe("s", &["files_rename_file".into()])
        .unwrap();
    assert!(full.contains("Rename a file on disk."));
    let too_many = (0..MAX_QUERIES_PER_CALL + 1)
        .map(|index| format!("q{index}"))
        .collect::<Vec<_>>();
    assert!(matches!(
        gateway.search_tools("s", &too_many),
        Err(SessionError::TooManyQueries { .. })
    ));
    let names = (0..MAX_DESCRIBE_NAMES + 1)
        .map(|index| format!("n{index}"))
        .collect::<Vec<_>>();
    assert!(matches!(
        gateway.describe("s", &names),
        Err(SessionError::TooManyNames { .. })
    ));
}

#[test]
fn a_call_must_be_offered_and_paging_splits_long_text() {
    let gateway = SessionGateway::new();
    gateway
        .reload(Load {
            native: vec![card("echo", "Echo text.", "core")],
            plugins: vec![],
            upstreams: vec![],
        })
        .unwrap();
    gateway.open_session("s");
    let err = gateway
        .call("s", "echo", &serde_json::json!({"text": "hi"}))
        .unwrap_err();
    assert!(matches!(err, SessionError::NotOffered { .. }));
    gateway.search_tools("s", &["echo".into()]).unwrap();
    let text = gateway
        .call("s", "echo", &serde_json::json!({"text": "hi"}))
        .unwrap();
    assert_eq!(text, "hi");
    assert!(gateway.offered_unused("s").unwrap().is_empty());
    let long = "x".repeat(PAGE_CHARS + 10);
    let paged = page_text(&long, 0);
    assert!(paged.contains("[page 0 continues]"));
    assert_eq!(page_text(&long, 1).chars().count(), 10);
}

#[test]
fn set_task_prefers_the_files_server() {
    let gateway = SessionGateway::new();
    gateway
        .reload(Load {
            native: synthetic(64),
            plugins: vec![],
            upstreams: vec![],
        })
        .unwrap();
    gateway.open_session("s");
    let picked = gateway.set_task("s", "rename a file").unwrap();
    assert!(!picked.is_empty());
    assert!(picked.iter().all(|name| name.starts_with("files_")));
}

#[test]
fn three_arms_succeed_and_dynamic_sends_fewer_schema_tokens() {
    let every = run_rename_task(Arm::Every, 256);
    let static_arm = run_rename_task(Arm::Static, 256);
    let dynamic = run_rename_task(Arm::Dynamic, 256);
    assert!(every.success && static_arm.success && dynamic.success);
    assert!(dynamic.schema_tokens < static_arm.schema_tokens);
    assert!(static_arm.schema_tokens < every.schema_tokens);
    println!(
        "every={} static={} dynamic={}",
        every.schema_tokens, static_arm.schema_tokens, dynamic.schema_tokens
    );
}
