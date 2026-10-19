use std::sync::Arc;

use gw_limits::catalog::{
    CatalogError, Gateway, HashEmbedder, Load, Origin, PluginSet, ToolCard, Upstream,
};

fn card(name: &str, description: &str, server: &str) -> ToolCard {
    ToolCard {
        name: name.into(),
        description: description.into(),
        server: server.into(),
    }
}

#[test]
fn reload_swaps_the_arc_and_keeps_the_old_snapshot() {
    let gateway = Gateway::new();
    gateway
        .reload(Load {
            native: vec![card("echo", "Echo text.", "core")],
            plugins: vec![],
            upstreams: vec![],
        })
        .unwrap();
    let held = gateway.snapshot();
    assert_eq!(held.len(), 1);
    gateway
        .reload(Load {
            native: vec![card("files_rename_file", "Rename a file.", "files")],
            plugins: vec![PluginSet {
                plugin: "notes".into(),
                tools: vec![card("notes_read_note", "Read a note.", "notes")],
            }],
            upstreams: vec![Upstream {
                name: "mail".into(),
                tools: vec![card("mail_search_message", "Search mail.", "mail")],
            }],
        })
        .unwrap();
    assert_eq!(held.len(), 1);
    assert!(held.card("echo").is_some());
    assert_eq!(gateway.len(), 3);
    assert_eq!(
        gateway.snapshot().origin("notes_read_note"),
        Some(Origin::Plugin)
    );
    assert_eq!(
        gateway.snapshot().origin("mail_search_message"),
        Some(Origin::Upstream)
    );
    let hits = gateway.search("rename a file", 3);
    assert_eq!(hits[0].name, "files_rename_file");
}

#[test]
fn a_duplicate_name_does_not_replace_the_catalog() {
    let gateway = Gateway::new();
    gateway
        .reload(Load {
            native: vec![card("echo", "Echo text.", "core")],
            plugins: vec![],
            upstreams: vec![],
        })
        .unwrap();
    let err = gateway
        .reload(Load {
            native: vec![card("echo", "Echo text.", "core")],
            plugins: vec![PluginSet {
                plugin: "other".into(),
                tools: vec![card("echo", "Also echo.", "other")],
            }],
            upstreams: vec![],
        })
        .unwrap_err();
    assert_eq!(
        err,
        CatalogError::Duplicate {
            name: "echo".into()
        }
    );
    assert_eq!(gateway.len(), 1);
}

#[test]
fn an_exact_name_ranks_first() {
    let gateway = Gateway::new();
    gateway
        .reload(Load {
            native: vec![
                card("files_rename_file", "Rename a file.", "files"),
                card(
                    "calendar_rename_file",
                    "Rename a calendar file event.",
                    "calendar",
                ),
            ],
            plugins: vec![],
            upstreams: vec![],
        })
        .unwrap();
    let hits = gateway.search("calendar_rename_file", 2);
    assert_eq!(hits[0].name, "calendar_rename_file");
    assert!(hits[0].score.is_infinite());
}

#[test]
fn call_uses_the_snapshot_it_captured() {
    let gateway = Gateway::new();
    gateway
        .reload(Load {
            native: vec![card("echo", "Echo text.", "core")],
            plugins: vec![],
            upstreams: vec![],
        })
        .unwrap();
    let text = gateway
        .call("echo", &serde_json::json!({"text": "hello"}))
        .unwrap();
    assert_eq!(text, "hello");
    gateway.set_embedder(Arc::new(HashEmbedder));
    assert!(gateway.search("echo", 1)[0].score.is_infinite());
}

#[test]
fn ten_thousand_tools_find_rename() {
    let gateway = Gateway::new();
    gateway
        .reload(Load {
            native: gw_limits::catalog::synthetic(10_000),
            plugins: vec![],
            upstreams: vec![],
        })
        .unwrap();
    let hits = gateway.search("rename a file", 5);
    assert_eq!(hits.len(), 5);
    assert!(hits
        .iter()
        .any(|hit| hit.name.contains("rename") && hit.name.contains("file")));
}
