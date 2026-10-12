use crate::message::Message;
use async_trait::async_trait;
use serde::de::{Deserialize, Deserializer};
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("network error: {0}")]
    Network(String),
    #[error("API returned {status}: {body}")]
    Api { status: u16, body: String },
    #[error("response had no choices")]
    EmptyResponse,
    #[error("{0}")]
    Mock(String),
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError>;
}

#[derive(Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [Message],
}

pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

pub struct Choice {
    pub message: Message,
}

impl<'de> Deserialize<'de> for ChatResponse {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        // Ignore unknown fields such as id, usage, and finish_reason.
        todo!("chapter 01")
    }
}

/// Records every conversation it is sent. `script` is the playback queue.
pub struct MockProvider {
    script: Mutex<VecDeque<Result<String, String>>>,
    default_reply: String,
    latency: Duration,
    pub seen: Mutex<Vec<Vec<Message>>>,
}

impl MockProvider {
    pub fn new(reply: impl Into<String>) -> Self {
        Self::with_latency(reply, Duration::ZERO)
    }

    pub fn with_latency(reply: impl Into<String>, latency: Duration) -> Self {
        Self {
            script: Mutex::new(VecDeque::new()),
            default_reply: reply.into(),
            latency,
            seen: Mutex::new(Vec::new()),
        }
    }

    pub fn scripted<I, S>(replies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mock = Self::new("(mock) no more replies");
        mock.script
            .lock()
            .expect("script")
            .extend(replies.into_iter().map(|reply| Ok(reply.into())));
        mock
    }

    pub fn then_fail(self, message: impl Into<String>) -> Self {
        self.script
            .lock()
            .expect("script")
            .push_back(Err(message.into()));
        self
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        // Record `messages` in `seen`, then pop the script.
        // Ok(text) -> assistant message. Err(text) -> ProviderError whose Display contains text.
        // Empty script -> assistant message with `default_reply`.
        let _ = messages;
        todo!("chapter 01")
    }
}

/// OpenAI-compatible client. Section 1.3 fills this in.
pub struct OpenAiCompatible {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

impl OpenAiCompatible {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        let _ = (base_url, api_key, model);
        todo!("chapter 01.3")
    }
}

#[async_trait]
impl Provider for OpenAiCompatible {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        let _ = messages;
        todo!("chapter 01.3")
    }
}

#[async_trait]
impl<P: Provider + ?Sized> Provider for std::sync::Arc<P> {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        (**self).complete(messages).await
    }
}
