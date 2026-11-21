//! Book chapter 18. One turn can build an Anthropic body, a Bedrock body,
//! or an OpenRouter route. Rotating a key keeps the same client id.
//! The side model stays off unless the turn asked for a summary.

use serde_json::{json, Map, Value};
use thiserror::Error;

pub const INTERRUPTED_PLACEHOLDER: &str = "[interrupt: no assistant output for this turn]";
pub const CLIENT_ID: u64 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Msg {
    pub role: String,
    pub content: String,
}

impl Msg {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProviderError {
    #[error("max_tokens must be positive")]
    MaxTokens,
    #[error("tls verification disabled without the dev flag")]
    InsecureTls,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlsDecision {
    OsTrust,
    DevInsecure,
}

pub fn resolve_tls(ssl_verify: bool, dev_insecure: bool) -> Result<TlsDecision, ProviderError> {
    if ssl_verify {
        return Ok(TlsDecision::OsTrust);
    }
    if dev_insecure {
        Ok(TlsDecision::DevInsecure)
    } else {
        Err(ProviderError::InsecureTls)
    }
}

/// Anthropic Messages body. `max_tokens` is the output cap and must be positive.
/// The per-model ceiling table in Hermes is not ported.
pub fn anthropic_body(
    model: &str,
    system: &str,
    messages: &[Msg],
    max_tokens: u32,
) -> Result<Value, ProviderError> {
    if max_tokens == 0 {
        return Err(ProviderError::MaxTokens);
    }
    let mut body = json!({
        "model": model,
        "max_tokens": max_tokens,
        "messages": wire_messages(messages),
    });
    if !system.is_empty() {
        body["system"] = json!(system);
    }
    Ok(body)
}

/// Bedrock Converse body. `max_tokens` lands in `inferenceConfig.maxTokens`.
pub fn bedrock_body(
    model: &str,
    system: &str,
    messages: &[Msg],
    max_tokens: u32,
) -> Result<Value, ProviderError> {
    if max_tokens == 0 {
        return Err(ProviderError::MaxTokens);
    }
    let mut body = json!({
        "modelId": model,
        "messages": wire_messages(messages),
        "inferenceConfig": { "maxTokens": max_tokens },
    });
    if !system.is_empty() {
        body["system"] = json!(system);
    }
    Ok(body)
}

fn wire_messages(messages: &[Msg]) -> Vec<Value> {
    messages
        .iter()
        .map(|message| json!({"role": message.role, "content": message.content}))
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct Prefs {
    pub only: Vec<String>,
    pub ignore: Vec<String>,
    pub order: Vec<String>,
    pub sort: Option<String>,
    pub require_parameters: bool,
    pub data_collection: Option<String>,
}

pub fn validated_sort(raw: &str) -> Option<&'static str> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "throughput" => Some("throughput"),
        "latency" => Some("latency"),
        "price" => Some("price"),
        _ => None,
    }
}

pub fn provider_preferences(prefs: &Prefs) -> Map<String, Value> {
    let mut out = Map::new();
    if !prefs.only.is_empty() {
        out.insert("only".into(), json!(prefs.only));
    }
    if !prefs.ignore.is_empty() {
        out.insert("ignore".into(), json!(prefs.ignore));
    }
    if !prefs.order.is_empty() {
        out.insert("order".into(), json!(prefs.order));
    }
    if let Some(sort) = prefs.sort.as_deref().and_then(validated_sort) {
        out.insert("sort".into(), json!(sort));
    }
    if prefs.require_parameters {
        out.insert("require_parameters".into(), json!(true));
    }
    if let Some(policy) = prefs
        .data_collection
        .as_ref()
        .filter(|value| !value.is_empty())
    {
        out.insert("data_collection".into(), json!(policy));
    }
    out
}

pub fn openrouter_route(models: &[String], prefs: &Prefs) -> Value {
    json!({
        "models": models,
        "provider": provider_preferences(prefs),
    })
}

pub fn choose_next<'a>(models: &'a [String], failed: Option<&str>) -> Option<&'a str> {
    let Some(failed) = failed else {
        return models.first().map(String::as_str);
    };
    let index = models.iter().position(|model| model == failed)?;
    models.get(index + 1).map(String::as_str)
}

#[derive(Debug, Clone)]
pub struct Pool {
    client_id: u64,
    keys: Vec<String>,
    cursor: usize,
}

impl Pool {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            client_id: CLIENT_ID,
            keys,
            cursor: 0,
        }
    }

    pub fn client_id(&self) -> u64 {
        self.client_id
    }

    pub fn current(&self) -> &str {
        self.keys.get(self.cursor).map(String::as_str).unwrap_or("")
    }

    pub fn mark_exhausted_and_rotate(&mut self) -> &str {
        if self.keys.len() > 1 {
            self.cursor = (self.cursor + 1) % self.keys.len();
        }
        self.current()
    }
}

pub fn side_model<'a>(asked: bool, primary: &'a str, side: &'a str) -> &'a str {
    if asked {
        side
    } else {
        primary
    }
}

/// Duplicate ids in one batch become `{id}_d2`, then `_d3`, matching
/// `uniquify_tool_call_ids` which starts the suffix at 2.
pub fn uniquify_tool_call_ids(ids: &mut [String]) {
    let mut seen: Vec<String> = Vec::new();
    for id in ids.iter_mut() {
        if id.is_empty() {
            continue;
        }
        if !seen.iter().any(|got| got == id) {
            seen.push(id.clone());
            continue;
        }
        let mut n = 2u32;
        let renamed = loop {
            let candidate = format!("{id}_d{n}");
            if !seen.iter().any(|got| got == &candidate) {
                break candidate;
            }
            n += 1;
        };
        seen.push(renamed.clone());
        *id = renamed;
    }
}

pub fn close_interrupted(messages: &mut Vec<Msg>) -> bool {
    let Some(last) = messages.last() else {
        return false;
    };
    if last.role != "tool" {
        return false;
    }
    messages.push(Msg::new("assistant", INTERRUPTED_PLACEHOLDER));
    true
}

#[derive(Debug, Clone)]
pub struct FakeModel {
    pub name: String,
    pub reply: String,
}

pub fn run_turn(models: &[FakeModel], failed_first: bool) -> (String, &'static str) {
    let names: Vec<String> = models.iter().map(|model| model.name.clone()).collect();
    let failed = if failed_first {
        names.first().map(String::as_str)
    } else {
        None
    };
    let chosen = choose_next(&names, failed).unwrap_or("");
    let reply = models
        .iter()
        .find(|model| model.name == chosen)
        .map(|model| model.reply.clone())
        .unwrap_or_default();
    (reply, chosen_static(chosen))
}

fn chosen_static(chosen: &str) -> &'static str {
    match chosen {
        "side" => "side",
        "fallback" => "fallback",
        _ => "primary",
    }
}
