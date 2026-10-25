//! Book chapter 42. The learned gateway is vendored here: a prediction is installed before the request.
//! `session_tools` only reads that set. Composite and macro tools expand
//! to names. Code mode runs a line script against an allowlist.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub mod catalog;
pub mod federation;
pub mod limits;
pub mod service;
pub mod session;

pub use limits::{Catalog, LimitedGateway, SessionError, SessionView};

/// Tools a code-mode script may call. Same names as Hermes
/// `SANDBOX_ALLOWED_TOOLS`.
pub const SANDBOX_ALLOWED: &[&str] = &[
    "web_search",
    "web_extract",
    "read_file",
    "write_file",
    "search_files",
    "patch",
    "terminal",
];

/// One script stops at this many tool calls.
pub const CODE_MODE_MAX_CALLS: usize = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Use {
    pub task: String,
    pub tool: String,
}

#[derive(Debug, Clone)]
pub struct PredictInput<'a> {
    pub task: &'a str,
    pub usage: &'a [Use],
    pub memory: &'a str,
    pub session_note: &'a str,
    pub catalog: &'a [String],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeError {
    TooManyCalls,
    NotAllowed(String),
    NotOffered(String),
    BadLine(String),
}

/// A keyless stand-in. The prediction path does not call it.
#[derive(Debug)]
pub struct ScriptedModel {
    reply: String,
    calls: u32,
}

impl ScriptedModel {
    pub fn new(reply: impl Into<String>) -> Self {
        Self {
            reply: reply.into(),
            calls: 0,
        }
    }

    pub fn calls(&self) -> u32 {
        self.calls
    }

    pub fn complete(&mut self, _prompt: &str) -> String {
        self.calls += 1;
        self.reply.clone()
    }
}

pub struct LearnedGateway {
    inner: LimitedGateway,
    offered: Mutex<Arc<Vec<String>>>,
    pending_change: AtomicBool,
    predict_calls: AtomicU64,
    composites: Mutex<BTreeMap<String, Vec<String>>>,
    macros: Mutex<BTreeMap<String, Vec<String>>>,
}

impl LearnedGateway {
    pub fn new() -> Self {
        Self {
            inner: LimitedGateway::new(),
            offered: Mutex::new(Arc::new(Vec::new())),
            pending_change: AtomicBool::new(false),
            predict_calls: AtomicU64::new(0),
            composites: Mutex::new(BTreeMap::new()),
            macros: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn catalog(&self) -> &Catalog {
        self.inner.catalog()
    }

    pub fn open_session(&self, session_id: impl Into<String>) {
        self.inner.open_session(session_id);
    }

    pub fn predict_calls(&self) -> u64 {
        self.predict_calls.load(Ordering::SeqCst)
    }

    /// The request path. Clones the installed `Arc`. Does not score usage.
    pub fn offered_arc(&self) -> Arc<Vec<String>> {
        Arc::clone(&self.offered.lock().expect("offered"))
    }

    pub fn session_tools(&self, session_id: &str) -> Option<SessionView> {
        let mut view = self.inner.session_tools(session_id)?;
        let offered = self.offered_arc();
        if !offered.is_empty() {
            view.tools = offered.as_ref().clone();
            view.list_changed = self.pending_change.swap(false, Ordering::SeqCst);
        }
        Some(view)
    }

    /// Offline. Scores the catalog from the usage log, memory, and a session note.
    pub fn refresh_prediction(&self, input: &PredictInput<'_>) -> Arc<Vec<String>> {
        self.predict_calls.fetch_add(1, Ordering::SeqCst);
        let names = predict(input);
        let arc = Arc::new(names);
        *self.offered.lock().expect("offered") = Arc::clone(&arc);
        self.pending_change.store(true, Ordering::SeqCst);
        arc
    }

    pub fn save_composite(&self, name: &str, steps: Vec<String>) {
        self.composites
            .lock()
            .expect("composites")
            .insert(name.to_string(), steps);
    }

    pub fn save_macro(&self, name: &str, steps: Vec<String>) {
        self.macros
            .lock()
            .expect("macros")
            .insert(name.to_string(), steps);
    }

    pub fn expand(&self, name: &str) -> Option<Vec<String>> {
        self.composites
            .lock()
            .expect("composites")
            .get(name)
            .cloned()
            .or_else(|| self.macros.lock().expect("macros").get(name).cloned())
    }

    pub fn run_code_mode(
        &self,
        script: &str,
        offered: &[String],
        mut call: impl FnMut(&str, &str) -> String,
    ) -> Result<Vec<String>, CodeError> {
        let offered_set: BTreeSet<&str> = offered.iter().map(String::as_str).collect();
        let mut lines = Vec::new();
        for raw in script.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let rest = line
                .strip_prefix("call ")
                .ok_or_else(|| CodeError::BadLine(line.to_string()))?;
            let (tool, args) = rest
                .split_once(char::is_whitespace)
                .ok_or_else(|| CodeError::BadLine(line.to_string()))?;
            if !SANDBOX_ALLOWED.contains(&tool) {
                return Err(CodeError::NotAllowed(tool.to_string()));
            }
            if !offered_set.contains(tool) {
                return Err(CodeError::NotOffered(tool.to_string()));
            }
            lines.push((tool.to_string(), args.trim().to_string()));
            if lines.len() > CODE_MODE_MAX_CALLS {
                return Err(CodeError::TooManyCalls);
            }
        }
        Ok(lines
            .into_iter()
            .map(|(tool, args)| call(&tool, &args))
            .collect())
    }
}

impl Default for LearnedGateway {
    fn default() -> Self {
        Self::new()
    }
}

pub fn predict(input: &PredictInput<'_>) -> Vec<String> {
    let task_tokens = tokens(input.task);
    let memory = input.memory.to_ascii_lowercase();
    let note = input.session_note.to_ascii_lowercase();
    let mut scored: Vec<(i32, &str)> = input
        .catalog
        .iter()
        .map(|name| {
            let mut score = 0;
            for row in input.usage {
                if row.tool == *name && shares_token(&task_tokens, &row.task) {
                    score += 1;
                }
            }
            if contains_name(&memory, name) || contains_name(&note, name) {
                score += 3;
            }
            (score, name.as_str())
        })
        .filter(|(score, _)| *score > 0)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
    scored
        .into_iter()
        .map(|(_, name)| name.to_string())
        .collect()
}

fn tokens(text: &str) -> BTreeSet<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| word.len() >= 3)
        .map(|word| word.to_ascii_lowercase())
        .collect()
}

fn shares_token(task: &BTreeSet<String>, other: &str) -> bool {
    tokens(other).iter().any(|token| task.contains(token))
}

fn contains_name(haystack: &str, name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    haystack.match_indices(&name).any(|(at, _)| {
        let before_ok = at == 0 || !is_name_char(haystack.as_bytes()[at - 1]);
        let after = at + name.len();
        let after_ok = after >= haystack.len() || !is_name_char(haystack.as_bytes()[after]);
        before_ok && after_ok
    })
}

fn is_name_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

pub fn demo() -> (String, String) {
    let gateway = LearnedGateway::new();
    gateway.open_session("desk");
    let catalog = vec!["read_file".to_string(), "web_search".to_string()];
    let usage = vec![Use {
        task: "read the note".into(),
        tool: "read_file".into(),
    }];
    let input = PredictInput {
        task: "read the note",
        usage: &usage,
        memory: "prefer read_file for notes",
        session_note: "",
        catalog: &catalog,
    };
    gateway.refresh_prediction(&input);
    let before = gateway.predict_calls();
    let view = gateway.session_tools("desk").expect("session");
    let after = gateway.predict_calls();
    assert_eq!(before, after);
    let offered = view.tools.join(",");
    let bodies = gateway
        .run_code_mode(
            "call read_file {\"path\":\"note\"}\n",
            &view.tools,
            |_tool, _args| "ok".into(),
        )
        .expect("code");
    (offered, bodies.join(","))
}
