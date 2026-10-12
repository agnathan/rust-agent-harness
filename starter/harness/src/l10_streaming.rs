//! Lesson 10: streaming. Parse OpenAI SSE lines into deltas and accumulate them.
use crate::l02_message::ToolCall;
use crate::l07_loop::ModelReply;
use serde_json::Value;
use tokio::sync::mpsc;

#[derive(Debug, Clone, PartialEq)]
pub enum Delta {
    Text(String),
    ToolCallStart {
        index: usize,
        id: String,
        name: String,
    },
    ToolCallArgs {
        index: usize,
        fragment: String,
    },
    Done,
}

/// One SSE line -> deltas.
/// - blank lines / lines not starting with "data:" -> Ok(None)
/// - "data: [DONE]" -> Ok(Some(vec![Done]))
/// - otherwise parse JSON (Err(msg) if invalid); from choices[0].delta emit:
///   Text(content) if content is a non-empty string;
///   for each tool_calls[i]: ToolCallStart if it has an "id" (name from function.name),
///   then ToolCallArgs if function.arguments is a non-empty string.
pub fn parse_sse_line(line: &str) -> Result<Option<Vec<Delta>>, String> {
    // Hint: strip_prefix("data:"), trim, serde_json::from_str::<Value>
    todo!("lesson 10")
}

#[derive(Debug, Default)]
pub struct Accumulator {
    pub text: String,
    pub calls: Vec<ToolCall>,
}

impl Accumulator {
    /// Text appends; ToolCallStart pushes a call (index == position); ToolCallArgs appends to calls[index].arguments.
    pub fn apply(&mut self, d: &Delta) {
        // Hint: match d { ... }
        todo!("lesson 10")
    }
    /// ToolCalls if any calls, else Text.
    pub fn finish(self) -> ModelReply {
        // Hint: consumes self
        todo!("lesson 10")
    }
}

/// Drain a channel of deltas until Done or channel close; call on_text for every Text delta
/// (this is how a UI would render tokens live). Return the finished reply.
pub async fn consume(mut rx: mpsc::Receiver<Delta>, mut on_text: impl FnMut(&str)) -> ModelReply {
    // Hint: while let Some(d) = rx.recv().await
    todo!("lesson 10")
}
