//! A scripted provider. It does not invent usage and it does not call a network.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use async_trait::async_trait;
use serde_json::Value;

use crate::event::{Event, Usage};
use crate::message::{Message, RequestedTool};

#[derive(Debug, Clone)]
pub struct ScriptedReply {
    pub text: String,
    pub tool_calls: Vec<RequestedTool>,
    pub usage: Option<Usage>,
}

impl ScriptedReply {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tool_calls: Vec::new(),
            usage: None,
        }
    }

    pub fn tool(id: impl Into<String>, name: impl Into<String>, arguments: Value) -> Self {
        Self {
            text: String::new(),
            tool_calls: vec![RequestedTool {
                id: id.into(),
                name: name.into(),
                arguments,
                raw_arguments: None,
            }],
            usage: None,
        }
    }

    pub fn tools(calls: Vec<RequestedTool>) -> Self {
        Self {
            text: String::new(),
            tool_calls: calls,
            usage: None,
        }
    }

    pub fn raw_tool(
        id: impl Into<String>,
        name: impl Into<String>,
        raw: impl Into<String>,
    ) -> Self {
        let raw = raw.into();
        let arguments = serde_json::from_str(&raw).unwrap_or(Value::Null);
        Self {
            text: String::new(),
            tool_calls: vec![RequestedTool {
                id: id.into(),
                name: name.into(),
                arguments,
                raw_arguments: Some(raw),
            }],
            usage: None,
        }
    }

    pub fn with_usage(mut self, usage: Usage) -> Self {
        self.usage = Some(usage);
        self
    }

    pub fn events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        if !self.text.is_empty() {
            events.push(Event::TextDelta(self.text.clone()));
        }
        for call in &self.tool_calls {
            let arguments = call
                .raw_arguments
                .clone()
                .unwrap_or_else(|| call.arguments.to_string());
            events.push(Event::ToolCall {
                id: call.id.clone(),
                name: call.name.clone(),
                arguments,
            });
        }
        if let Some(usage) = &self.usage {
            events.push(Event::Usage(usage.clone()));
        }
        events.push(Event::Done {
            message: Message::assistant(self.text.clone(), self.tool_calls.clone()),
            model: None,
        });
        events
    }
}

pub struct ScriptedProvider {
    replies: Mutex<VecDeque<ScriptedReply>>,
    calls: AtomicU64,
}

impl ScriptedProvider {
    pub fn new(replies: Vec<ScriptedReply>) -> Self {
        Self {
            replies: Mutex::new(VecDeque::from(replies)),
            calls: AtomicU64::new(0),
        }
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn complete(&self, messages: &[Message]) -> Vec<Event>;
    fn calls(&self) -> u64;
}

#[async_trait]
impl Provider for ScriptedProvider {
    async fn complete(&self, _messages: &[Message]) -> Vec<Event> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let reply = self
            .replies
            .lock()
            .expect("script")
            .pop_front()
            .unwrap_or_else(|| ScriptedReply::text("script exhausted"));
        reply.events()
    }

    fn calls(&self) -> u64 {
        self.calls.load(Ordering::SeqCst)
    }
}
