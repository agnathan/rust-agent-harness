//! Book chapter 8. One session sees a filtered toolset. `search_tools` and
//! `set_task` reveal names. Schemas stay off the list until `describe`.
//! The catalog underneath is the chapter 7 gateway.

use std::collections::{BTreeSet, HashMap};
use std::sync::Mutex;

use gw_catalog::{tokenize, Gateway, Hit};
use serde_json::Value;
use thiserror::Error;

pub const MAX_QUERIES_PER_CALL: usize = 7;
pub const MAX_DESCRIBE_NAMES: usize = 10;
pub const SEARCH_LIMIT: usize = 5;
pub const STARTER_LIMIT: usize = 8;
pub const PAGE_CHARS: usize = 2_000;
pub const CHARS_PER_TOKEN: usize = 4;

pub use gw_catalog::{synthetic, CatalogError, Load, PluginSet, ToolCard, Upstream};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SessionError {
    #[error("unknown session {0}")]
    UnknownSession(String),
    #[error("too many queries ({got} > {MAX_QUERIES_PER_CALL})")]
    TooManyQueries { got: usize },
    #[error("too many names ({got} > {MAX_DESCRIBE_NAMES})")]
    TooManyNames { got: usize },
    #[error("tool {name} is not in this session")]
    NotOffered { name: String },
    #[error(transparent)]
    Catalog(#[from] CatalogError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionView {
    pub session_id: String,
    pub tools: Vec<String>,
    pub list_changed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Telemetry {
    pub schema_tokens: u64,
    pub call_tokens: u64,
    pub result_tokens: u64,
}

struct Session {
    revealed: Vec<String>,
    list_changed: bool,
    schema_tokens: u64,
    call_tokens: u64,
    result_tokens: u64,
    called: BTreeSet<String>,
}

impl Session {
    fn new() -> Self {
        Self {
            revealed: Vec::new(),
            list_changed: false,
            schema_tokens: 0,
            call_tokens: 0,
            result_tokens: 0,
            called: BTreeSet::new(),
        }
    }

    fn reveal(&mut self, name: &str) -> bool {
        if self.revealed.iter().any(|got| got == name) {
            return false;
        }
        self.revealed.push(name.to_string());
        self.list_changed = true;
        true
    }
}

pub struct SessionGateway {
    catalog: Gateway,
    sessions: Mutex<HashMap<String, Session>>,
}

impl SessionGateway {
    pub fn new() -> Self {
        Self {
            catalog: Gateway::new(),
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn catalog(&self) -> &Gateway {
        &self.catalog
    }

    pub fn reload(&self, load: Load) -> Result<(), CatalogError> {
        self.catalog.reload(load)
    }

    pub fn open_session(&self, session_id: impl Into<String>) {
        self.sessions
            .lock()
            .expect("sessions")
            .entry(session_id.into())
            .or_insert_with(Session::new);
    }

    pub fn session_tools(&self, session_id: &str) -> Option<SessionView> {
        let sessions = self.sessions.lock().expect("sessions");
        let session = sessions.get(session_id)?;
        Some(SessionView {
            session_id: session_id.to_string(),
            tools: session.revealed.clone(),
            list_changed: session.list_changed,
        })
    }

    pub fn take_list_changed(&self, session_id: &str) -> Result<bool, SessionError> {
        let mut sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        Ok(std::mem::take(&mut session.list_changed))
    }

    /// Reveal up to [`STARTER_LIMIT`] tools from the server that best matches `task`.
    pub fn set_task(&self, session_id: &str, task: &str) -> Result<Vec<String>, SessionError> {
        let snapshot = self.catalog.snapshot();
        let tokens = tokenize(task);
        let mut scores: HashMap<&str, u32> = HashMap::new();
        for card in snapshot.cards() {
            let mut score = 0u32;
            for token in &tokens {
                if card.name.to_ascii_lowercase().contains(token)
                    || card.description.to_ascii_lowercase().contains(token)
                {
                    score += 1;
                }
                if card.server.starts_with(token.as_str()) || token.starts_with(&card.server) {
                    score += 2;
                }
            }
            if score > 0 {
                *scores.entry(card.server.as_str()).or_insert(0) += score;
            }
        }
        let server = scores
            .into_iter()
            .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(left.0)))
            .map(|(server, _)| server.to_string());
        let mut picked = Vec::new();
        if let Some(server) = server {
            let mut names: Vec<&ToolCard> = snapshot
                .cards()
                .iter()
                .filter(|card| card.server == server)
                .collect();
            names.sort_by(|left, right| {
                name_token_hits(&right.name, &tokens)
                    .cmp(&name_token_hits(&left.name, &tokens))
                    .then_with(|| left.name.cmp(&right.name))
            });
            for card in names.into_iter().take(STARTER_LIMIT) {
                picked.push(card.name.clone());
            }
        }
        let mut sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        for name in &picked {
            session.reveal(name);
        }
        Ok(picked)
    }

    pub fn search_tools(
        &self,
        session_id: &str,
        queries: &[String],
    ) -> Result<String, SessionError> {
        if queries.len() > MAX_QUERIES_PER_CALL {
            return Err(SessionError::TooManyQueries { got: queries.len() });
        }
        let mut hits: Vec<Hit> = Vec::new();
        for query in queries {
            for hit in self.catalog.search(query, SEARCH_LIMIT) {
                if !hits.iter().any(|got| got.name == hit.name) {
                    hits.push(hit);
                }
            }
        }
        let snapshot = self.catalog.snapshot();
        let mut lines = Vec::new();
        let mut sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        for hit in &hits {
            session.reveal(&hit.name);
            let blurb = snapshot
                .card(&hit.name)
                .map(|card| trim_chars(&card.description, 80))
                .unwrap_or_default();
            lines.push(format!("{} — {blurb}", hit.name));
        }
        let listing = if lines.is_empty() {
            "no tools".to_string()
        } else {
            lines.join("\n")
        };
        session.schema_tokens += approx_tokens(&listing);
        Ok(listing)
    }

    pub fn describe(&self, session_id: &str, names: &[String]) -> Result<String, SessionError> {
        if names.len() > MAX_DESCRIBE_NAMES {
            return Err(SessionError::TooManyNames { got: names.len() });
        }
        let snapshot = self.catalog.snapshot();
        let mut blocks = Vec::new();
        for name in names {
            let Some(card) = snapshot.card(name) else {
                blocks.push(format!("{name}: unknown"));
                continue;
            };
            blocks.push(format!(
                "{name}\nserver: {}\n{}",
                card.server, card.description
            ));
        }
        let text = blocks.join("\n\n");
        let mut sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        session.schema_tokens += approx_tokens(&text);
        Ok(text)
    }

    pub fn list_compact(&self, session_id: &str) -> Result<String, SessionError> {
        let snapshot = self.catalog.snapshot();
        let sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        let mut lines = Vec::new();
        for name in &session.revealed {
            let blurb = snapshot
                .card(name)
                .map(|card| trim_chars(&card.description, 80))
                .unwrap_or_default();
            lines.push(format!("{name} — {blurb}"));
        }
        Ok(lines.join("\n"))
    }

    pub fn call(
        &self,
        session_id: &str,
        name: &str,
        arguments: &Value,
    ) -> Result<String, SessionError> {
        let args_text = arguments.to_string();
        let mut sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        if !session.revealed.iter().any(|got| got == name) {
            return Err(SessionError::NotOffered {
                name: name.to_string(),
            });
        }
        drop(sessions);
        let result = self.catalog.call(name, arguments)?;
        let mut sessions = self.sessions.lock().expect("sessions");
        let session = sessions.get_mut(session_id).expect("session");
        session.called.insert(name.to_string());
        session.call_tokens += approx_tokens(&args_text);
        session.result_tokens += approx_tokens(&result);
        Ok(result)
    }

    pub fn telemetry(&self, session_id: &str) -> Result<Telemetry, SessionError> {
        let sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        Ok(Telemetry {
            schema_tokens: session.schema_tokens,
            call_tokens: session.call_tokens,
            result_tokens: session.result_tokens,
        })
    }

    pub fn offered_unused(&self, session_id: &str) -> Result<Vec<String>, SessionError> {
        let sessions = self.sessions.lock().expect("sessions");
        let session = sessions
            .get(session_id)
            .ok_or_else(|| SessionError::UnknownSession(session_id.to_string()))?;
        Ok(session
            .revealed
            .iter()
            .filter(|name| !session.called.contains(*name))
            .cloned()
            .collect())
    }
}

impl Default for SessionGateway {
    fn default() -> Self {
        Self::new()
    }
}

fn name_token_hits(name: &str, tokens: &[String]) -> usize {
    let parts = tokenize(name);
    tokens
        .iter()
        .filter(|token| parts.iter().any(|part| part == *token))
        .count()
}

fn schema_blob(cards: &[ToolCard]) -> String {
    cards
        .iter()
        .map(|card| {
            format!(
                "{}\nserver: {}\n{}",
                card.name, card.server, card.description
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn approx_tokens(text: &str) -> u64 {
    (text.len().div_ceil(CHARS_PER_TOKEN)) as u64
}

pub fn trim_chars(text: &str, max_chars: usize) -> String {
    let count = text.chars().count();
    if count <= max_chars {
        return text.to_string();
    }
    text.chars().take(max_chars).collect()
}

pub fn page_text(text: &str, page: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let start = page.saturating_mul(PAGE_CHARS).min(chars.len());
    let end = (start + PAGE_CHARS).min(chars.len());
    let body: String = chars[start..end].iter().collect();
    if end < chars.len() {
        format!("{body}\n[page {page} continues]")
    } else {
        body
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm {
    Every,
    Static,
    Dynamic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskReport {
    pub success: bool,
    pub schema_tokens: u64,
    pub call_tokens: u64,
    pub result_tokens: u64,
}

/// A fake model renames a file. No network. `count` is the synthetic catalog size.
pub fn run_rename_task(arm: Arm, count: usize) -> TaskReport {
    let gateway = SessionGateway::new();
    let cards = synthetic(count);
    gateway
        .reload(Load {
            native: cards.clone(),
            plugins: vec![],
            upstreams: vec![],
        })
        .expect("reload");
    gateway.open_session("task");
    let target = cards
        .iter()
        .find(|card| card.server == "files" && card.name.contains("rename_file"))
        .expect("rename tool")
        .name
        .clone();
    match arm {
        Arm::Every => {
            let mut sessions = gateway.sessions.lock().expect("sessions");
            let session = sessions.get_mut("task").expect("session");
            for card in &cards {
                session.reveal(&card.name);
            }
            session.schema_tokens = approx_tokens(&schema_blob(&cards));
            drop(sessions);
            let result = gateway
                .call("task", &target, &serde_json::json!({"text": "a.txt"}))
                .expect("call");
            report(&gateway, result.contains("rename") || !result.is_empty())
        }
        Arm::Static => {
            let mut sessions = gateway.sessions.lock().expect("sessions");
            let session = sessions.get_mut("task").expect("session");
            session.reveal(&target);
            for card in cards
                .iter()
                .filter(|card| card.server == "files" && card.name != target)
                .take(STARTER_LIMIT - 1)
            {
                session.reveal(&card.name);
            }
            let revealed = sessions.get("task").expect("session").revealed.clone();
            let shown: Vec<ToolCard> = cards
                .iter()
                .filter(|card| revealed.iter().any(|name| name == &card.name))
                .cloned()
                .collect();
            sessions.get_mut("task").expect("session").schema_tokens =
                approx_tokens(&schema_blob(&shown));
            drop(sessions);
            let result = gateway
                .call("task", &target, &serde_json::json!({"to": "b.txt"}))
                .expect("call");
            report(
                &gateway,
                result.contains("rename") || result.contains("file"),
            )
        }
        Arm::Dynamic => {
            let listing = gateway
                .search_tools("task", &["rename a file".into()])
                .expect("search");
            let name = listing
                .lines()
                .next()
                .and_then(|line| line.split(" — ").next())
                .unwrap_or("")
                .to_string();
            if !name.is_empty() {
                let _ = gateway.describe("task", std::slice::from_ref(&name));
                let result = gateway
                    .call("task", &name, &serde_json::json!({"to": "b.txt"}))
                    .expect("call");
                let success = name.contains("rename") && name.contains("file");
                let _ = result;
                return report(&gateway, success);
            }
            report(&gateway, false)
        }
    }
}

fn report(gateway: &SessionGateway, success: bool) -> TaskReport {
    let telemetry = gateway.telemetry("task").expect("telemetry");
    TaskReport {
        success,
        schema_tokens: telemetry.schema_tokens,
        call_tokens: telemetry.call_tokens,
        result_tokens: telemetry.result_tokens,
    }
}
