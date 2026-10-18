//! One inbox per chat session. Capacity is 64.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub const INBOX_BOUND: usize = 64;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InboxError {
    #[error("session inbox is full ({bound} turns waiting)")]
    Full { bound: usize },
}

#[derive(Debug, Clone)]
pub struct SessionInbox {
    queue: Arc<Mutex<VecDeque<String>>>,
    bound: usize,
}

impl SessionInbox {
    pub fn new(bound: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(bound.min(64)))),
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

    pub fn submit(&self, prompt: impl Into<String>) -> Result<(), InboxError> {
        let mut queue = self.queue.lock().expect("inbox");
        if queue.len() >= self.bound {
            return Err(InboxError::Full { bound: self.bound });
        }
        queue.push_back(prompt.into());
        Ok(())
    }

    pub fn pop(&self) -> Option<String> {
        self.queue.lock().expect("inbox").pop_front()
    }
}

impl Default for SessionInbox {
    fn default() -> Self {
        Self::new(INBOX_BOUND)
    }
}
