//! Browser tools behind a `PageBackend`. Tests use `FakeBrowser`.
//! Page text is untrusted. URLs go through chapter 16's secret and SSRF floors.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;

use serde_json::{json, Value};

use crate::web::{
    is_safe_url, url_has_secret, wrap_untrusted, HostResolver, MapResolver, WebError,
};

pub const DEFAULT_SNAPSHOT_CHARS: usize = 15_000;
pub const MIN_SNAPSHOT_CHARS: usize = 1_000;
pub const MAX_STORED_SNAPSHOT_CHARS: usize = 2_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserError {
    SecretUrl,
    UnsafeUrl(String),
    Redirect(String),
    NoPage,
    UnknownRef(String),
    Backend(String),
}

impl std::fmt::Display for BrowserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SecretUrl => f.write_str(
                "Blocked: URL contains what appears to be an API key or token. Secrets must not be sent in URLs.",
            ),
            Self::UnsafeUrl(text) => f.write_str(text),
            Self::Redirect(text) => write!(f, "Blocked: redirect landed on {text}"),
            Self::NoPage => f.write_str("browser_navigate first"),
            Self::UnknownRef(refer) => write!(f, "unknown ref {refer}"),
            Self::Backend(text) => f.write_str(text),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub url: String,
    pub final_url: String,
    pub title: String,
    pub snapshot: String,
}

pub trait PageBackend: Send + Sync {
    fn open(&self, url: &str) -> Result<Page, BrowserError>;
}

/// Canned pages. `open` never dials.
#[derive(Clone, Default)]
pub struct FakeBrowser {
    pages: HashMap<String, Page>,
}

impl FakeBrowser {
    pub fn new(pages: HashMap<String, Page>) -> Self {
        Self { pages }
    }

    pub fn example() -> Self {
        let mut pages = HashMap::new();
        pages.insert(
            "https://example.com/notes".into(),
            Page {
                url: "https://example.com/notes".into(),
                final_url: "https://example.com/notes".into(),
                title: "Notes".into(),
                snapshot: "heading Notes\n[e1] link Home\n[e2] button Save\n".into(),
            },
        );
        Self { pages }
    }
}

impl PageBackend for FakeBrowser {
    fn open(&self, url: &str) -> Result<Page, BrowserError> {
        self.pages
            .get(url)
            .cloned()
            .ok_or_else(|| BrowserError::Backend(format!("no canned page for {url}")))
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    page: Option<Page>,
}

impl Session {
    pub fn new() -> Self {
        Self { page: None }
    }

    pub fn page(&self) -> Option<&Page> {
        self.page.as_ref()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

pub fn public_resolver() -> MapResolver {
    let mut hosts = HashMap::new();
    hosts.insert(
        "example.com".into(),
        Ok(vec!["93.184.216.34".parse::<IpAddr>().expect("ip")]),
    );
    MapResolver::new(hosts)
}

fn check_url(url: &str, resolver: &dyn HostResolver) -> Result<(), BrowserError> {
    if url_has_secret(url) {
        return Err(BrowserError::SecretUrl);
    }
    is_safe_url(url, false, resolver).map_err(|error| match error {
        WebError::Message(text) => BrowserError::UnsafeUrl(text),
    })
}

fn clip_snapshot(text: &str) -> String {
    let mut stored = text.to_string();
    if stored.chars().count() > MAX_STORED_SNAPSHOT_CHARS {
        stored = stored.chars().take(MAX_STORED_SNAPSHOT_CHARS).collect();
    }
    if stored.chars().count() <= DEFAULT_SNAPSHOT_CHARS {
        return stored;
    }
    let head = (DEFAULT_SNAPSHOT_CHARS as f64 * 0.75) as usize;
    let tail = DEFAULT_SNAPSHOT_CHARS - head;
    let chars: Vec<char> = stored.chars().collect();
    let mut out: String = chars[..head].iter().collect();
    out.push_str("\n[TRUNCATED]\n");
    out.extend(chars[chars.len() - tail..].iter());
    out
}

/// Navigate, then wrap the snapshot. A redirect is checked again.
/// Metadata hosts stay blocked. The backend is not called for a blocked URL.
pub fn navigate(
    session: &mut Session,
    backend: &dyn PageBackend,
    url: &str,
    resolver: &dyn HostResolver,
) -> Result<String, BrowserError> {
    check_url(url, resolver)?;
    let page = backend.open(url)?;
    if page.final_url != url {
        check_url(&page.final_url, resolver).map_err(|_| {
            BrowserError::Redirect("a cloud metadata endpoint or a private address".into())
        })?;
    }
    let snapshot = clip_snapshot(&page.snapshot);
    let wrapped = wrap_untrusted("browser_navigate", &snapshot);
    let body = json!({
        "success": true,
        "url": page.final_url,
        "title": page.title,
        "snapshot": wrapped,
    });
    session.page = Some(page);
    Ok(body.to_string())
}

fn require_page(session: &Session) -> Result<&Page, BrowserError> {
    session.page.as_ref().ok_or(BrowserError::NoPage)
}

pub fn snapshot(session: &Session) -> Result<String, BrowserError> {
    let page = require_page(session)?;
    let text = clip_snapshot(&page.snapshot);
    Ok(wrap_untrusted("browser_snapshot", &text))
}

pub fn click(session: &Session, refer: &str) -> Result<String, BrowserError> {
    let page = require_page(session)?;
    let needle = format!("[{refer}]");
    if !page.snapshot.contains(&needle)
        && !page
            .snapshot
            .contains(&format!("[{}]", refer.trim_start_matches('@')))
    {
        return Err(BrowserError::UnknownRef(refer.to_string()));
    }
    let note = format!("clicked {refer} on {}", page.final_url);
    Ok(wrap_untrusted("browser_click", &note))
}

pub fn type_text(session: &Session, refer: &str, text: &str) -> Result<String, BrowserError> {
    let _ = require_page(session)?;
    click(session, refer)?;
    let note = format!("typed {text} into {refer}");
    Ok(wrap_untrusted("browser_type", &note))
}

pub fn scroll(session: &Session, direction: &str) -> Result<String, BrowserError> {
    let page = require_page(session)?;
    let note = format!("scrolled {direction} on {}", page.final_url);
    Ok(wrap_untrusted("browser_scroll", &note))
}

pub fn demo() -> String {
    let backend = FakeBrowser::example();
    let mut session = Session::new();
    let raw = navigate(
        &mut session,
        &backend,
        "https://example.com/notes",
        &public_resolver(),
    )
    .expect("example page");
    let value: Value = serde_json::from_str(&raw).expect("json");
    let title = value["title"].as_str().unwrap_or("");
    format!("browser: {title}")
}

/// Shared resolver for benches. Constructing it is outside the sample.
pub fn bench_backend() -> Arc<FakeBrowser> {
    Arc::new(FakeBrowser::example())
}
