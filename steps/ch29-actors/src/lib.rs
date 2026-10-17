//! Book chapter 33. One tokio task per agent. The mailbox is a session
//! inbox with room for 64 messages. A full inbox refuses the next send.
//! A steer waits for the turn. An interrupt drops that turn before the model.

use std::collections::VecDeque;

use tokio::sync::mpsc::{self, error::TrySendError};
use tokio::task::JoinSet;

pub const INBOX_CAP: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mail {
    Turn(String),
    Steer(String),
    Interrupt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboxError {
    Full,
    Closed,
}

/// A scripted model. `complete` never opens a socket.
#[derive(Debug, Clone)]
pub struct FakeModel {
    reply: String,
    pub calls: u32,
}

impl FakeModel {
    pub fn new(reply: impl Into<String>) -> Self {
        Self {
            reply: reply.into(),
            calls: 0,
        }
    }

    pub fn complete(&mut self, _prompt: &str) -> String {
        self.calls += 1;
        self.reply.clone()
    }
}

#[derive(Debug)]
pub struct Inbox {
    queue: VecDeque<Mail>,
    steer: String,
    interrupted: bool,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            steer: String::new(),
            interrupted: false,
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn try_submit(&mut self, mail: Mail) -> Result<(), InboxError> {
        if self.queue.len() == INBOX_CAP {
            return Err(InboxError::Full);
        }
        self.queue.push_back(mail);
        Ok(())
    }

    /// Apply one queued message. A steer is stored. An interrupt marks the next
    /// turn. A turn calls the model unless it was interrupted.
    pub fn step(&mut self, model: &mut FakeModel) -> Option<String> {
        let mail = self.queue.pop_front()?;
        match mail {
            Mail::Steer(text) => {
                let cleaned = text.trim();
                if !cleaned.is_empty() {
                    if self.steer.is_empty() {
                        self.steer = cleaned.to_string();
                    } else {
                        self.steer.push('\n');
                        self.steer.push_str(cleaned);
                    }
                }
                None
            }
            Mail::Interrupt => {
                self.interrupted = true;
                None
            }
            Mail::Turn(text) => {
                if self.interrupted {
                    self.interrupted = false;
                    self.steer.clear();
                    return Some("interrupted".to_string());
                }
                let mut reply = model.complete(&text);
                if !self.steer.is_empty() {
                    reply.push('\n');
                    reply.push_str(&self.steer);
                    self.steer.clear();
                }
                Some(reply)
            }
        }
    }
}

impl Default for Inbox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn submit(tx: &mpsc::Sender<Mail>, mail: Mail) -> Result<(), InboxError> {
    match tx.try_send(mail) {
        Ok(()) => Ok(()),
        Err(TrySendError::Full(_)) => Err(InboxError::Full),
        Err(TrySendError::Closed(_)) => Err(InboxError::Closed),
    }
}

pub async fn serve(mut rx: mpsc::Receiver<Mail>, model: &mut FakeModel) -> Vec<String> {
    let mut inbox = Inbox::new();
    let mut replies = Vec::new();
    while let Some(mail) = rx.recv().await {
        inbox
            .try_submit(mail)
            .expect("the channel cap is the inbox cap");
        if let Some(reply) = inbox.step(model) {
            replies.push(reply);
        }
    }
    replies
}

pub async fn fan(n: usize) -> Vec<String> {
    let mut set = JoinSet::new();
    for index in 0..n {
        set.spawn(async move {
            let (tx, rx) = mpsc::channel(INBOX_CAP);
            submit(&tx, Mail::Turn(format!("job {index}"))).expect("empty inbox");
            drop(tx);
            let mut model = FakeModel::new(format!("ok {index}"));
            let replies = serve(rx, &mut model).await;
            replies.into_iter().next().unwrap_or_default()
        });
    }
    let mut replies = Vec::with_capacity(n);
    while let Some(joined) = set.join_next().await {
        replies.push(joined.expect("agent"));
    }
    replies.sort();
    replies
}

pub fn demo() -> String {
    let mut inbox = Inbox::new();
    inbox.try_submit(Mail::Turn("hello".into())).expect("empty");
    let mut model = FakeModel::new("ok");
    let reply = inbox.step(&mut model).expect("a turn");
    format!("agent: {reply}")
}
