use crate::config::Endpoint;
use crate::event::{events_for_reply, Event, Usage};
use crate::message::{ChatRequest, Message};
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Mutex;
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
}

#[async_trait]
pub trait Provider: Send + Sync {
    /// Send the conversation, get back the assistant's reply.
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError>;

    /// Same call, plus the model id and token counts the server reported.
    /// The default has neither: a mock did not call a server.
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

    /// The same call, as events. The default mock path has no usage and no model id.
    async fn complete_events(&self, messages: &[Message]) -> Result<Vec<Event>, ProviderError> {
        let detailed = self.complete_detailed(messages).await?;
        Ok(events_for_reply(
            &detailed.message,
            detailed.usage.as_ref(),
            detailed.model.as_deref(),
        ))
    }
}

/// What one buffered (non-streaming) completion came back as.
#[derive(Debug, Clone, PartialEq)]
pub struct DetailedCompletion {
    pub message: Message,
    pub model: Option<String>,
    pub usage: Option<Usage>,
}

/// One shared `reqwest::Client`: connections are pooled, and HTTP/2 is enabled
/// for servers that negotiate it (ALPN on TLS, or prior knowledge).
pub struct OpenAiCompatible {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
    http_referer: Option<String>,
    title: Option<String>,
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
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(8)
            .http2_adaptive_window(true)
            .build()
            .expect("HTTP client");
        Self {
            client,
            base_url: endpoint.base_url.trim_end_matches('/').to_string(),
            api_key: endpoint.api_key.clone(),
            model: endpoint.model.clone(),
            http_referer: endpoint.http_referer.clone(),
            title: endpoint.title.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[async_trait]
impl Provider for OpenAiCompatible {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        Ok(self.complete_detailed(messages).await?.message)
    }

    async fn complete_detailed(
        &self,
        messages: &[Message],
    ) -> Result<DetailedCompletion, ProviderError> {
        let url = format!("{}/chat/completions", self.base_url);
        let request = ChatRequest {
            model: &self.model,
            messages,
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

        let response = builder.send().await?;
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

/// Returns a canned reply after an optional wait, and records every conversation.
///
/// `latency` stands in for model time. Overhead benchmarks use [`Duration::ZERO`].
pub struct MockProvider {
    reply: String,
    latency: Duration,
    pub seen: Mutex<Vec<Vec<Message>>>,
}

impl MockProvider {
    pub fn new(reply: impl Into<String>) -> Self {
        Self::with_latency(reply, Duration::ZERO)
    }

    pub fn with_latency(reply: impl Into<String>, latency: Duration) -> Self {
        Self {
            reply: reply.into(),
            latency,
            seen: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        if !self.latency.is_zero() {
            tokio::time::sleep(self.latency).await;
        }
        self.seen.lock().expect("seen").push(messages.to_vec());
        Ok(Message::assistant(self.reply.clone()))
    }
}

#[async_trait]
impl<P: Provider + ?Sized> Provider for std::sync::Arc<P> {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        (**self).complete(messages).await
    }

    async fn complete_detailed(
        &self,
        messages: &[Message],
    ) -> Result<DetailedCompletion, ProviderError> {
        (**self).complete_detailed(messages).await
    }

    async fn complete_events(&self, messages: &[Message]) -> Result<Vec<Event>, ProviderError> {
        (**self).complete_events(messages).await
    }
}
