//! The OpenRouter client, with room for a tool list on the request.
//!
//! This is the same pooled client as chapter 1. A live call is optional.
//! The milestone demo uses `--mock` and does not build this client.

use crate::config::Endpoint;
use crate::event::{events_for_reply, Event, Usage};
use crate::message::Message;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("missing {name}: pass --api-key, or run with --mock")]
    MissingKey { name: &'static str },
    #[error("model id is empty")]
    EmptyModel,
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("API returned {status}: {body}")]
    Api { status: u16, body: String },
    #[error("response had no choices")]
    EmptyResponse,
    #[error("request timed out after {timeout:?}")]
    Timeout { timeout: Duration },
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError>;

    async fn complete_detailed(
        &self,
        messages: &[Message],
    ) -> Result<DetailedCompletion, ProviderError> {
        Ok(DetailedCompletion {
            message: self.complete(messages).await?,
            model: None,
            usage: None,
        })
    }

    async fn complete_events(&self, messages: &[Message]) -> Result<Vec<Event>, ProviderError> {
        let detailed = self.complete_detailed(messages).await?;
        Ok(events_for_reply(
            &detailed.message,
            detailed.usage.as_ref(),
            detailed.model.as_deref(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DetailedCompletion {
    pub message: Message,
    pub model: Option<String>,
    pub usage: Option<Usage>,
}

/// OpenAI-style tool object. The card's JSON Schema is `parameters`.
pub fn openai_tool(name: &str, description: &str, parameters: &Value) -> Value {
    serde_json::json!({
        "type": "function",
        "function": {
            "name": name,
            "description": description,
            "parameters": parameters,
        }
    })
}

pub struct OpenAiCompatible {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
    http_referer: Option<String>,
    title: Option<String>,
    timeout: Duration,
}

impl OpenAiCompatible {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        Self::from_endpoint(&Endpoint {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
            http_referer: None,
            title: None,
        })
    }

    pub fn from_endpoint(endpoint: &Endpoint) -> Self {
        Self::with_timeout(endpoint, Duration::from_secs(60))
    }

    pub fn with_timeout(endpoint: &Endpoint, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(8)
            .http2_adaptive_window(true)
            .timeout(timeout)
            .build()
            .expect("HTTP client");
        Self {
            client,
            base_url: endpoint.base_url.trim_end_matches('/').to_string(),
            api_key: endpoint.api_key.clone(),
            model: endpoint.model.clone(),
            http_referer: endpoint.http_referer.clone(),
            title: endpoint.title.clone(),
            timeout,
        }
    }

    pub async fn complete_with_tools(
        &self,
        messages: &[Message],
        tools: Vec<Value>,
    ) -> Result<Vec<Event>, ProviderError> {
        let detailed = self.send(messages, tools).await?;
        Ok(events_for_reply(
            &detailed.message,
            detailed.usage.as_ref(),
            detailed.model.as_deref(),
        ))
    }

    async fn send(
        &self,
        messages: &[Message],
        tools: Vec<Value>,
    ) -> Result<DetailedCompletion, ProviderError> {
        let url = format!("{}/chat/completions", self.base_url);
        let request = ChatRequest {
            model: &self.model,
            messages,
            tools,
        };
        let mut builder = self.client.post(&url).json(&request);
        if !self.api_key.is_empty() {
            builder = builder.bearer_auth(&self.api_key);
        }
        if let Some(referer) = &self.http_referer {
            builder = builder.header("HTTP-Referer", referer);
        }
        if let Some(title) = &self.title {
            builder = builder.header("X-Title", title);
        }
        let response = builder.send().await.map_err(|err| {
            if err.is_timeout() {
                ProviderError::Timeout {
                    timeout: self.timeout,
                }
            } else {
                ProviderError::Network(err)
            }
        })?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api {
                status: status.as_u16(),
                body,
            });
        }
        let parsed: ChatResponse = response.json().await?;
        let message = parsed
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message)
            .ok_or(ProviderError::EmptyResponse)?;
        Ok(DetailedCompletion {
            message,
            model: parsed.model,
            usage: parsed.usage,
        })
    }
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Value>,
}

#[derive(Deserialize)]
struct ChatResponse {
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[async_trait]
impl Provider for OpenAiCompatible {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        Ok(self.send(messages, Vec::new()).await?.message)
    }

    async fn complete_detailed(
        &self,
        messages: &[Message],
    ) -> Result<DetailedCompletion, ProviderError> {
        self.send(messages, Vec::new()).await
    }
}
