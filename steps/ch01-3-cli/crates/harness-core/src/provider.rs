use crate::message::{ChatRequest, Message};
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
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
}

/// One shared `reqwest::Client`: connections are pooled, and HTTP/2 is enabled
/// for servers that negotiate it (ALPN on TLS, or prior knowledge).
pub struct OpenAiCompatible {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAiCompatible {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(8)
            .http2_adaptive_window(true)
            .build()
            .expect("HTTP client");
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[async_trait]
impl Provider for OpenAiCompatible {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        let url = format!("{}/chat/completions", self.base_url);
        let request = ChatRequest {
            model: &self.model,
            messages,
        };

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api {
                status: status.as_u16(),
                body,
            });
        }

        let parsed: ChatResponse = response.json().await?;
        parsed
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message)
            .ok_or(ProviderError::EmptyResponse)
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
}
