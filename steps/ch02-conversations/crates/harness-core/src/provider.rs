use crate::message::Message;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
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

// ---------- OpenAI-compatible provider ----------

pub struct OpenAiCompatible {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAiCompatible {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [Message],
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
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

// ---------- Mock provider for tests ----------

/// Plays back scripted replies in order, then falls back to a default reply.
/// Records every conversation it was sent.
pub struct MockProvider {
    script: Mutex<VecDeque<Result<String, String>>>,
    default_reply: String,
    latency: Duration,
    pub seen: Mutex<Vec<Vec<Message>>>,
}

impl MockProvider {
    /// Always answers with `reply`.
    pub fn new(reply: impl Into<String>) -> Self {
        Self::with_latency(reply, Duration::ZERO)
    }

    /// `latency` stands in for model time. Overhead benchmarks use zero.
    pub fn with_latency(reply: impl Into<String>, latency: Duration) -> Self {
        Self {
            script: Mutex::new(VecDeque::new()),
            default_reply: reply.into(),
            latency,
            seen: Mutex::new(Vec::new()),
        }
    }

    /// Answers with each reply in turn (then "(mock) no more replies").
    pub fn scripted<I, S>(replies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mock = Self::new("(mock) no more replies");
        mock.script
            .lock()
            .unwrap()
            .extend(replies.into_iter().map(|r| Ok(r.into())));
        mock
    }

    /// Queue a failure, to test error handling.
    pub fn then_fail(self, message: impl Into<String>) -> Self {
        self.script.lock().unwrap().push_back(Err(message.into()));
        self
    }

    pub fn calls(&self) -> usize {
        self.seen.lock().unwrap().len()
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        if !self.latency.is_zero() {
            tokio::time::sleep(self.latency).await;
        }
        self.seen.lock().unwrap().push(messages.to_vec());
        match self.script.lock().unwrap().pop_front() {
            Some(Ok(reply)) => Ok(Message::assistant(reply)),
            Some(Err(message)) => Err(ProviderError::Api {
                status: 500,
                body: message,
            }),
            None => Ok(Message::assistant(self.default_reply.clone())),
        }
    }
}

// A shared reference to a provider is also a provider.
#[async_trait]
impl<P: Provider + ?Sized> Provider for std::sync::Arc<P> {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        (**self).complete(messages).await
    }
}

#[async_trait]
impl<P: Provider + ?Sized> Provider for Box<P> {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        (**self).complete(messages).await
    }
}
