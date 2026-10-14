use crate::config::Endpoint;
use crate::event::{events_for_reply, Event, Usage};
use crate::message::Message;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
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
    #[error("request timed out after {timeout:?}")]
    Timeout { timeout: Duration },
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

    /// `timeout` covers the whole request. Retries wait for a later chapter.
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
}

#[derive(Debug, Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [Message],
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

/// Plays back scripted replies in order, then falls back to a default reply.
/// Records every conversation it was sent.
///
/// `latency` stands in for model time. Overhead benchmarks use [`Duration::ZERO`].
/// `usage` is optional and scripted: the mock does not invent a server bill.
pub struct MockProvider {
    script: Mutex<VecDeque<Result<String, String>>>,
    default_reply: String,
    latency: Duration,
    usage: Option<Usage>,
    answered_model: Option<String>,
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
            usage: None,
            answered_model: None,
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
            .expect("script")
            .extend(replies.into_iter().map(|reply| Ok(reply.into())));
        mock
    }

    /// Queue a failure, to test error handling.
    pub fn then_fail(self, message: impl Into<String>) -> Self {
        self.script
            .lock()
            .expect("script")
            .push_back(Err(message.into()));
        self
    }

    /// Attach a scripted model id and usage to every successful reply.
    /// This is not a live OpenRouter response.
    pub fn with_usage(mut self, model: impl Into<String>, usage: Usage) -> Self {
        self.answered_model = Some(model.into());
        self.usage = Some(usage);
        self
    }

    pub fn calls(&self) -> usize {
        self.seen.lock().expect("seen").len()
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        if !self.latency.is_zero() {
            tokio::time::sleep(self.latency).await;
        }
        self.seen.lock().expect("seen").push(messages.to_vec());
        match self.script.lock().expect("script").pop_front() {
            Some(Ok(reply)) => Ok(Message::assistant(reply)),
            Some(Err(message)) => Err(ProviderError::Api {
                status: 500,
                body: message,
            }),
            None => Ok(Message::assistant(self.default_reply.clone())),
        }
    }

    async fn complete_detailed(
        &self,
        messages: &[Message],
    ) -> Result<DetailedCompletion, ProviderError> {
        let message = self.complete(messages).await?;
        Ok(DetailedCompletion {
            message,
            model: self.answered_model.clone(),
            usage: self.usage.clone(),
        })
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

#[async_trait]
impl<P: Provider + ?Sized> Provider for Box<P> {
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
