//! Book chapter 30. Ownership moves to the next agent only after the
//! transcript is filtered. A turn may hand off at most four times.
//! That cap is this harness. Hermes does not count handoffs.

use std::sync::atomic::{AtomicU32, Ordering};

/// Harness cap. Not a Hermes constant.
pub const MAX_HANDOFFS: u32 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffError {
    Limit,
    Empty,
}

pub struct Board {
    count: AtomicU32,
    owner: std::sync::Mutex<String>,
}

impl Board {
    pub fn new(owner: impl Into<String>) -> Self {
        Self {
            count: AtomicU32::new(0),
            owner: std::sync::Mutex::new(owner.into()),
        }
    }

    pub fn owner(&self) -> String {
        self.owner.lock().expect("owner").clone()
    }

    pub fn count(&self) -> u32 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn handoff(&self, next: &str, messages: &[Message]) -> Result<Vec<Message>, HandoffError> {
        let filtered = filter_transcript(messages);
        if filtered.is_empty() {
            return Err(HandoffError::Empty);
        }
        let current = self.count.fetch_add(1, Ordering::Relaxed);
        if current >= MAX_HANDOFFS {
            self.count.fetch_sub(1, Ordering::Relaxed);
            return Err(HandoffError::Limit);
        }
        *self.owner.lock().expect("owner") = next.to_string();
        Ok(filtered)
    }
}

pub fn filter_transcript(messages: &[Message]) -> Vec<Message> {
    messages
        .iter()
        .filter(|message| message.role != "scratch")
        .cloned()
        .collect()
}

pub fn demo() -> String {
    let board = Board::new("outline");
    let messages = vec![
        Message {
            role: "user".into(),
            text: "Write a note.".into(),
        },
        Message {
            role: "scratch".into(),
            text: "draft junk".into(),
        },
        Message {
            role: "assistant".into(),
            text: "three steps".into(),
        },
    ];
    let filtered = board.handoff("draft", &messages).expect("handoff");
    format!("{} sees {}", board.owner(), filtered.len())
}
