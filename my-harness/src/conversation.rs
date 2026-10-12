use crate::message::{Message, Role};
use std::path::Path;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConversationError {
    #[error("expected a {expected:?} message next, got {got:?}")]
    OutOfOrder { expected: Role, got: Role },
    #[error("system messages only go at the start")]
    SystemInHistory,
    #[error("could not read or write file: {0}")]
    Io(String),
    #[error("invalid conversation file: {0}")]
    Format(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversation {
    system: String,
    history: Vec<Message>,
}

impl Conversation {
    pub fn new(system: impl Into<String>) -> Self {
        Self {
            system: system.into(),
            history: Vec::new(),
        }
    }

    pub fn system(&self) -> &str {
        &self.system
    }

    pub fn history(&self) -> &[Message] {
        &self.history
    }

    pub fn next_role(&self) -> Role {
        todo!("chapter 02")
    }

    pub fn push(&mut self, message: Message) -> Result<(), ConversationError> {
        // Reject Role::System (SystemInHistory). Reject a role other than next_role
        // (OutOfOrder). Otherwise append.
        let _ = message;
        todo!("chapter 02")
    }

    pub fn pop_last_user(&mut self) -> Option<Message> {
        todo!("chapter 02")
    }

    pub fn to_messages(&self) -> Vec<Message> {
        // System prompt first, then history.
        todo!("chapter 02")
    }

    pub fn reset(&mut self) {
        todo!("chapter 02")
    }

    pub fn approx_tokens(&self) -> usize {
        todo!("chapter 02")
    }

    pub fn trim_to(&mut self, max_tokens: usize) -> usize {
        // Drop oldest user/assistant pairs. Never drop the system prompt or the newest message.
        let _ = max_tokens;
        todo!("chapter 02")
    }

    pub fn save(&self, path: &Path) -> Result<(), ConversationError> {
        let _ = path;
        todo!("chapter 02")
    }

    pub fn load(path: &Path) -> Result<Self, ConversationError> {
        // Re-validate through push so a hand-edited file can still be OutOfOrder.
        let _ = path;
        todo!("chapter 02")
    }
}
