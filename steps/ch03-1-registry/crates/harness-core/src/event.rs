//! One event shape for a model call.
//!
//! A buffered client emits these too: one text delta for the whole reply,
//! then usage when the server sent it, then done. Streaming, the gateway,
//! tracing, and evals fold the same enum later. They do not invent a second one.

use crate::message::Message;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Usage {
    #[serde(default)]
    pub prompt_tokens: u64,
    #[serde(default)]
    pub completion_tokens: u64,
    #[serde(default)]
    pub total_tokens: u64,
    /// USD, when the server reports it. OpenRouter sends `cost`. Many other
    /// OpenAI-compatible servers omit it.
    #[serde(default)]
    pub cost: Option<f64>,
}

/// Token counts and cost summed across the calls one agent has made.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AgentUsage {
    pub calls: u64,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    pub cost_usd: f64,
    pub cost_known: bool,
}

impl AgentUsage {
    pub fn record(&mut self, usage: &Usage) {
        self.calls += 1;
        self.prompt_tokens += usage.prompt_tokens;
        self.completion_tokens += usage.completion_tokens;
        self.total_tokens += usage.total_tokens;
        if let Some(cost) = usage.cost {
            self.cost_usd += cost;
            self.cost_known = true;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Assistant text. A buffered call emits one delta, the whole reply.
    TextDelta(String),
    /// A tool the model asked to run. Chapter 2 has no tools, so nothing emits this yet.
    ToolCall {
        id: String,
        name: String,
        arguments: String,
    },
    Usage(Usage),
    Done {
        message: Message,
        model: Option<String>,
    },
    Error(String),
}

impl Event {
    pub fn error(err: impl ToString) -> Self {
        Event::Error(err.to_string())
    }

    /// One line for the CLI and the milestone demo.
    pub fn report_line(&self) -> String {
        match self {
            Event::TextDelta(text) => format!("event: text-delta {text}"),
            Event::ToolCall {
                id,
                name,
                arguments,
            } => format!("event: tool-call {name} id={id} args={arguments}"),
            Event::Usage(usage) => {
                let cost = match usage.cost {
                    Some(cost) => format!(" cost={}", format_cost(cost)),
                    None => String::new(),
                };
                format!(
                    "event: usage prompt={} completion={} total={}{cost}",
                    usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
                )
            }
            Event::Done { .. } => "event: done".to_string(),
            Event::Error(text) => format!("event: error {text}"),
        }
    }
}

/// Events a non-streaming call emits. `usage` and `model` are omitted when the
/// server (or the mock) did not report them.
pub fn events_for_reply(
    message: &Message,
    usage: Option<&Usage>,
    model: Option<&str>,
) -> Vec<Event> {
    let mut events = vec![Event::TextDelta(message.content.clone())];
    if let Some(usage) = usage {
        events.push(Event::Usage(usage.clone()));
    }
    events.push(Event::Done {
        message: message.clone(),
        model: model.map(str::to_string),
    });
    events
}

pub fn format_cost(cost: f64) -> String {
    let text = format!("{cost:.8}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
