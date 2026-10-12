use crate::message::Message;
use async_trait::async_trait;
use serde::de::{Deserialize, Deserializer};
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::Mutex;

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
    pub seen: Mutex<Vec<Vec<Message>>>,
}

impl MockProvider {
    pub fn new(reply: impl Into<String>) -> Self {
        Self {
            script: Mutex::new(VecDeque::new()),
            default_reply: reply.into(),
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

#[async_trait]
impl<P: Provider + ?Sized> Provider for std::sync::Arc<P> {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        (**self).complete(messages).await
    }
}
