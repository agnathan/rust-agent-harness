//! Optional OpenRouter call. The mock stays the default. CI does not set a key.

use std::time::Duration;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::event::{Event, Usage};
use crate::message::{Message, RequestedTool, Role};
use crate::provider::Provider;
use crate::tool::ToolSchema;

pub const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
pub const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";

pub fn missing_live_key(
    api_key_flag: Option<&str>,
    openrouter_api_key: Option<&str>,
) -> Option<String> {
    let key = api_key_flag
        .filter(|value| !value.is_empty())
        .or_else(|| openrouter_api_key.filter(|value| !value.is_empty()));
    if key.is_none() {
        Some("missing OPENROUTER_API_KEY: pass --api-key, or run with --mock".into())
    } else {
        None
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LiveError {
    #[error("timeout after {secs}s")]
    Timeout { secs: u64 },
    #[error("network: {0}")]
    Network(String),
    #[error("api status {status}: {body}")]
    Api { status: u16, body: String },
    #[error("empty response")]
    Empty,
}

pub struct OpenAiCompatible {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
    timeout: Duration,
}

impl OpenAiCompatible {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        let timeout = Duration::from_secs(60);
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(8)
            .http2_adaptive_window(true)
            .timeout(timeout)
            .build()
            .expect("HTTP client");
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
            timeout,
        }
    }

    pub async fn complete_with_tools(
        &self,
        messages: &[Message],
        schemas: &[ToolSchema],
    ) -> Result<Vec<Event>, LiveError> {
        let tools = tool_definitions(schemas);
        let body = json!({
            "model": self.model,
            "messages": messages.iter().map(api_message).collect::<Vec<_>>(),
            "tools": tools,
        });
        let url = format!("{}/chat/completions", self.base_url);
        let mut builder = self.client.post(url).json(&body);
        if !self.api_key.is_empty() {
            builder = builder.bearer_auth(&self.api_key);
        }
        let response = builder.send().await.map_err(|error| {
            if error.is_timeout() {
                LiveError::Timeout {
                    secs: self.timeout.as_secs(),
                }
            } else {
                LiveError::Network(error.to_string())
            }
        })?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(LiveError::Api {
                status: status.as_u16(),
                body,
            });
        }
        let parsed: ChatResponse = response
            .json()
            .await
            .map_err(|error| LiveError::Network(error.to_string()))?;
        let choice = parsed.choices.into_iter().next().ok_or(LiveError::Empty)?;
        Ok(events_from_api(choice.message, parsed.usage, parsed.model))
    }
}

pub fn tool_definitions(schemas: &[ToolSchema]) -> Vec<Value> {
    schemas
        .iter()
        .map(|schema| {
            json!({
                "type": "function",
                "function": {
                    "name": schema.name,
                    "description": schema.description,
                    "parameters": schema.input_schema,
                }
            })
        })
        .collect()
}

pub struct LiveProvider {
    pub client: OpenAiCompatible,
    pub schemas: Vec<ToolSchema>,
}

#[async_trait]
impl Provider for LiveProvider {
    async fn complete(&self, messages: &[Message]) -> Vec<Event> {
        match self
            .client
            .complete_with_tools(messages, &self.schemas)
            .await
        {
            Ok(events) => events,
            Err(error) => vec![Event::Error(error.to_string())],
        }
    }

    fn calls(&self) -> u64 {
        0
    }
}

fn api_message(message: &Message) -> Value {
    let role = match message.role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    };
    let mut value = json!({
        "role": role,
        "content": message.content,
    });
    if let Some(id) = &message.tool_call_id {
        value["tool_call_id"] = json!(id);
    }
    if !message.tool_calls.is_empty() {
        value["tool_calls"] = Value::Array(
            message
                .tool_calls
                .iter()
                .map(|call| {
                    json!({
                        "id": call.id,
                        "type": "function",
                        "function": {
                            "name": call.name,
                            "arguments": call.arguments.to_string(),
                        }
                    })
                })
                .collect(),
        );
    }
    value
}

fn events_from_api(message: ApiMessage, usage: Option<Usage>, model: Option<String>) -> Vec<Event> {
    let tool_calls = message
        .tool_calls
        .into_iter()
        .map(|call| {
            let raw = call.function.arguments;
            let arguments = if raw.trim().is_empty() {
                serde_json::json!({})
            } else {
                serde_json::from_str(&raw).unwrap_or(Value::Null)
            };
            RequestedTool {
                id: call.id,
                name: call.function.name,
                arguments,
                raw_arguments: if raw.trim().is_empty() {
                    None
                } else {
                    Some(raw)
                },
            }
        })
        .collect::<Vec<_>>();
    let text = message.content.unwrap_or_default();
    let mut events = Vec::new();
    if !text.is_empty() {
        events.push(Event::TextDelta(text.clone()));
    }
    for call in &tool_calls {
        events.push(Event::ToolCall {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.to_string(),
        });
    }
    if let Some(usage) = usage {
        events.push(Event::Usage(usage));
    }
    events.push(Event::Done {
        message: Message::assistant(text, tool_calls),
        model,
    });
    events
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    choices: Vec<ApiChoice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct ApiChoice {
    message: ApiMessage,
}

#[derive(Debug, Deserialize)]
struct ApiMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<ApiToolCall>,
}

#[derive(Debug, Deserialize)]
struct ApiToolCall {
    id: String,
    function: ApiFunction,
}

#[derive(Debug, Deserialize)]
struct ApiFunction {
    name: String,
    #[serde(default)]
    arguments: String,
}
