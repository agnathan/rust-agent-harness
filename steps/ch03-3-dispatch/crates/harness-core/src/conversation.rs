use crate::message::{Message, Role};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, thiserror::Error, PartialEq)]
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

/// A system prompt plus an ordered, alternating history of user/assistant turns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Which role is allowed to speak next.
    pub fn next_role(&self) -> Role {
        match self.history.last() {
            None => Role::User,
            Some(m) if m.role == Role::User => Role::Assistant,
            Some(_) => Role::User,
        }
    }

    /// Append a message, enforcing strict user/assistant alternation.
    pub fn push(&mut self, message: Message) -> Result<(), ConversationError> {
        if message.role == Role::System {
            return Err(ConversationError::SystemInHistory);
        }
        let expected = self.next_role();
        if message.role != expected {
            return Err(ConversationError::OutOfOrder {
                expected,
                got: message.role,
            });
        }
        self.history.push(message);
        Ok(())
    }

    /// Undo a user turn that never got an answer (e.g. the API call failed).
    pub fn pop_last_user(&mut self) -> Option<Message> {
        if self.next_role() == Role::Assistant {
            self.history.pop()
        } else {
            None
        }
    }

    /// Everything the model sees: the system prompt first, then the history.
    pub fn to_messages(&self) -> Vec<Message> {
        let mut out = Vec::with_capacity(self.history.len() + 1);
        out.push(Message::system(self.system.clone()));
        out.extend(self.history.iter().cloned());
        out
    }

    /// Forget the history but keep the (stable) system prompt.
    pub fn reset(&mut self) {
        self.history.clear();
    }

    /// Rough token estimate: about 4 characters per token for English text.
    pub fn approx_tokens(&self) -> usize {
        let chars: usize =
            self.system.len() + self.history.iter().map(|m| m.content.len()).sum::<usize>();
        chars.div_ceil(4)
    }

    /// Drop the oldest user/assistant pairs until we fit in `max_tokens`.
    /// Never touches the system prompt and never drops the newest message.
    /// Returns how many messages were removed.
    pub fn trim_to(&mut self, max_tokens: usize) -> usize {
        let mut removed = 0;
        while self.approx_tokens() > max_tokens && self.history.len() > 2 {
            self.history.drain(0..2);
            removed += 2;
        }
        removed
    }

    pub fn save(&self, path: &Path) -> Result<(), ConversationError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ConversationError::Format(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| ConversationError::Io(e.to_string()))
    }

    /// Load and re-validate: a hand-edited file must still obey the rules.
    pub fn load(path: &Path) -> Result<Self, ConversationError> {
        let text =
            std::fs::read_to_string(path).map_err(|e| ConversationError::Io(e.to_string()))?;
        let raw: Conversation =
            serde_json::from_str(&text).map_err(|e| ConversationError::Format(e.to_string()))?;
        let mut checked = Conversation::new(raw.system);
        for message in raw.history {
            checked.push(message)?;
        }
        Ok(checked)
    }
}
