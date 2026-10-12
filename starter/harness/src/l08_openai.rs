//! Lesson 08: a real OpenAI-compatible provider with reqwest (blocking for now).
use crate::l02_message::{Message, ToolCall};
use crate::l05_json::{ToolSpec, WireMessage};
use crate::l07_loop::{Model, ModelReply};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAiConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

impl OpenAiConfig {
    /// Read config through a lookup fn (so tests don't touch real env vars).
    /// OPENAI_API_KEY required (Err("OPENAI_API_KEY not set")),
    /// OPENAI_BASE_URL default "https://api.openai.com/v1" (strip a trailing '/'),
    /// OPENAI_MODEL default "gpt-4o-mini".
    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        // Hint: get("..").ok_or(..)? and unwrap_or_else for defaults
        todo!("lesson 08")
    }
    pub fn from_env() -> Result<Self, String> {
        Self::from_lookup(|k| std::env::var(k).ok())
    }
}

/// {"model":..,"messages":[WireMessage..]} plus "tools":[spec.to_openai()..] only if tools is non-empty.
pub fn build_request_body(model: &str, messages: &[Message], tools: &[ToolSpec]) -> Value {
    // Hint: map messages through WireMessage::from; serde_json::to_value
    todo!("lesson 08")
}

/// Read choices[0].message. Non-empty tool_calls -> ToolCalls, else Text(content or "").
/// Missing choices -> Err("no choices"). If body has "error":{"message":m} -> Err(m).
pub fn parse_chat_response(body: &Value) -> Result<ModelReply, String> {
    // Hint: body["choices"][0]["message"] indexing returns Value::Null when missing
    todo!("lesson 08")
}

pub struct OpenAiModel {
    pub cfg: OpenAiConfig,
    client: reqwest::blocking::Client,
}

impl OpenAiModel {
    pub fn new(cfg: OpenAiConfig) -> Self {
        OpenAiModel {
            cfg,
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Model for OpenAiModel {
    /// POST {base_url}/chat/completions with bearer auth; parse with parse_chat_response.
    /// (Not unit-tested: run examples in the lesson against a real or local endpoint.)
    fn complete(&mut self, messages: &[Message], tools: &[ToolSpec]) -> Result<ModelReply, String> {
        // Hint: client.post(url).bearer_auth(..).json(&body).send() ; .json::<Value>()
        todo!("lesson 08")
    }
}
