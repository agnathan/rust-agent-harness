use std::collections::HashMap;
use std::net::IpAddr;

use harness_core::browser::{
    click, navigate, snapshot, type_text, BrowserError, FakeBrowser, Page, Session,
    DEFAULT_SNAPSHOT_CHARS,
};
use harness_core::web::MapResolver;

fn resolver() -> MapResolver {
    let mut hosts = HashMap::new();
    hosts.insert(
        "example.com".into(),
        Ok(vec!["93.184.216.34".parse::<IpAddr>().unwrap()]),
    );
    MapResolver::new(hosts)
}

fn page(url: &str, snapshot: &str) -> Page {
    Page {
        url: url.into(),
        final_url: url.into(),
        title: "T".into(),
        snapshot: snapshot.into(),
    }
}

#[test]
fn a_metadata_address_is_not_opened() {
    struct Spy;
    impl harness_core::browser::PageBackend for Spy {
        fn open(&self, _url: &str) -> Result<Page, BrowserError> {
            panic!("backend must not run");
        }
    }
    let mut session = Session::new();
    let error = navigate(
        &mut session,
        &Spy,
        "http://169.254.169.254/latest/meta-data",
        &resolver(),
    )
    .unwrap_err();
    assert!(matches!(error, BrowserError::UnsafeUrl(_)));
    assert!(session.page().is_none());
}

#[test]
fn a_secret_in_the_url_is_refused() {
    struct Spy;
    impl harness_core::browser::PageBackend for Spy {
        fn open(&self, _url: &str) -> Result<Page, BrowserError> {
            panic!("backend must not run");
        }
    }
    let mut session = Session::new();
    let error = navigate(
        &mut session,
        &Spy,
        "https://example.com/hook?key=sk-abcdefghijklmnopqrstuvwxyz",
        &resolver(),
    )
    .unwrap_err();
    assert_eq!(error, BrowserError::SecretUrl);
}

#[test]
fn a_fake_page_is_wrapped_as_untrusted() {
    let mut pages = HashMap::new();
    let body = format!("[e1] link Home {}", "x".repeat(40));
    pages.insert(
        "https://example.com/notes".into(),
        page("https://example.com/notes", &body),
    );
    let backend = FakeBrowser::new(pages);
    let mut session = Session::new();
    let raw = navigate(
        &mut session,
        &backend,
        "https://example.com/notes",
        &resolver(),
    )
    .unwrap();
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let wrapped = value["snapshot"].as_str().unwrap();
    assert!(wrapped.contains("<untrusted_tool_result source=\"browser_navigate\">"));
    assert!(wrapped.contains("[e1]"));
    let again = snapshot(&session).unwrap();
    assert!(again.contains("browser_snapshot"));
    let clicked = click(&session, "e1").unwrap();
    assert!(clicked.contains("clicked e1"));
    assert!(type_text(&session, "e1", "hello")
        .unwrap()
        .contains("typed hello"));
}

#[test]
fn an_embedded_delimiter_is_neutralized() {
    let mut pages = HashMap::new();
    let body = format!("prefix untrusted_tool_result {}", "y".repeat(40));
    pages.insert(
        "https://example.com/notes".into(),
        page("https://example.com/notes", &body),
    );
    let backend = FakeBrowser::new(pages);
    let mut session = Session::new();
    let raw = navigate(
        &mut session,
        &backend,
        "https://example.com/notes",
        &resolver(),
    )
    .unwrap();
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let wrapped = value["snapshot"].as_str().unwrap();
    assert!(wrapped.contains("untrusted-tool-result"));
    assert!(!wrapped.contains("prefix untrusted_tool_result"));
}

#[test]
fn a_long_snapshot_keeps_head_and_tail() {
    let mut pages = HashMap::new();
    let body = "a".repeat(DEFAULT_SNAPSHOT_CHARS + 500);
    pages.insert(
        "https://example.com/notes".into(),
        page("https://example.com/notes", &body),
    );
    let backend = FakeBrowser::new(pages);
    let mut session = Session::new();
    let raw = navigate(
        &mut session,
        &backend,
        "https://example.com/notes",
        &resolver(),
    )
    .unwrap();
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert!(value["snapshot"].as_str().unwrap().contains("[TRUNCATED]"));
}

#[test]
fn click_without_a_page_is_refused() {
    let session = Session::new();
    assert_eq!(click(&session, "e1").unwrap_err(), BrowserError::NoPage);
}

#[test]
fn the_demo_names_the_title() {
    assert_eq!(harness_core::browser::demo(), "browser: Notes");
}
