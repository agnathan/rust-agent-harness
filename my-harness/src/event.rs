use crate::message::Message;

/// The one event enum. Section 1.2 fills in how a buffered call produces these.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    TextDelta(String),
    ToolCall {
        id: String,
        name: String,
        arguments: String,
    },
    Usage {
        prompt_tokens: u64,
        completion_tokens: u64,
        total_tokens: u64,
        cost: Option<f64>,
    },
    Done {
        message: Message,
        model: Option<String>,
    },
    Error(String),
}

pub fn buffered_events(reply: &Message) -> Vec<Event> {
    let _ = reply;
    todo!("chapter 01.2")
}
