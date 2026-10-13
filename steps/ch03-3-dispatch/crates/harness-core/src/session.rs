//! One inbox per chat session.
//!
//! The REPL submits a user line here, then the session pops it and calls the
//! provider. Later the gateway, cron, and the editor submit to this same
//! inbox. They do not grow a second queue. Capacity is 64; a full inbox
//! refuses the new turn instead of growing.

use std::collections::VecDeque;
use std::sync::Mutex;

pub const INBOX_BOUND: usize = 64;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InboxError {
    #[error("session inbox is full ({bound} turns waiting)")]
    Full { bound: usize },
}

#[derive(Debug)]
pub struct SessionInbox {
    queue: Mutex<VecDeque<String>>,
    bound: usize,
}

impl SessionInbox {
    pub fn new(bound: usize) -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(bound.min(64))),
            bound,
        }
    }

    pub fn bound(&self) -> usize {
        self.bound
    }

    pub fn len(&self) -> usize {
        self.queue.lock().expect("inbox").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Enqueue one user turn. This does not call the model.
    pub fn submit(&self, prompt: impl Into<String>) -> Result<(), InboxError> {
        let mut queue = self.queue.lock().expect("inbox");
        if queue.len() >= self.bound {
            return Err(InboxError::Full { bound: self.bound });
        }
        queue.push_back(prompt.into());
        Ok(())
    }

    /// The oldest waiting turn, if any.
    pub fn pop(&self) -> Option<String> {
        self.queue.lock().expect("inbox").pop_front()
    }
}

impl Default for SessionInbox {
    fn default() -> Self {
        Self::new(INBOX_BOUND)
    }
}
