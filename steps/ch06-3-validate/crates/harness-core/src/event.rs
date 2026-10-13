//! The chapter 1 event enum. The loop emits the same variants.

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
    #[serde(default)]
    pub cost: Option<f64>,
}

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

    pub fn count_call_without_usage(&mut self) {
        self.calls += 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    TextDelta(String),
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

pub fn format_cost(cost: f64) -> String {
    let text = format!("{cost:.8}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
