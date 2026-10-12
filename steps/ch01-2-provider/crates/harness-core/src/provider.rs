use crate::message::Message;
use async_trait::async_trait;
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
}

#[async_trait]
pub trait Provider: Send + Sync {
    /// Send the conversation, get back the assistant's reply.
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError>;
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
