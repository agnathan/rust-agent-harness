//! Book chapter 45. Agents share one board. A condition wakes a waiter.
//! The scheduler claims under the lock, then drops it before the model.

use std::collections::HashMap;
use std::sync::{Condvar, Mutex, MutexGuard};

/// A waiter that has already fired this many times is left asleep.
pub const MAX_WAKES: u32 = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Waiter {
    pub name: String,
    pub key: String,
    pub wakes: u32,
}

impl Waiter {
    pub fn new(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            key: key.into(),
            wakes: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Threat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Claim {
    Woke { name: String, value: String },
    Idle,
    Paused,
}

#[derive(Debug)]
pub struct Board {
    facts: HashMap<String, String>,
    waiters: Vec<Waiter>,
    paused: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
            waiters: Vec::new(),
            paused: false,
        }
    }

    pub fn with_waiters(waiters: Vec<Waiter>) -> Self {
        Self {
            facts: HashMap::new(),
            waiters,
            paused: false,
        }
    }

    pub fn waiter_count(&self) -> usize {
        self.waiters.len()
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn has_fact(&self, key: &str) -> bool {
        self.facts.contains_key(key)
    }

    /// Refuse a value that carries an injection phrase. The fact is not stored.
    pub fn post(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<(), Threat> {
        let value = value.into();
        if is_injection(&value) {
            return Err(Threat);
        }
        self.facts.insert(key.into(), value);
        Ok(())
    }

    /// One waiter whose key is on the board, or idle. The fact leaves with the claim.
    pub fn claim_one(&mut self) -> Claim {
        if self.paused {
            return Claim::Paused;
        }
        let index = self
            .waiters
            .iter()
            .position(|waiter| waiter.wakes < MAX_WAKES && self.facts.contains_key(&waiter.key));
        let Some(index) = index else {
            return Claim::Idle;
        };
        let waiter = &mut self.waiters[index];
        waiter.wakes += 1;
        let name = waiter.name.clone();
        let key = waiter.key.clone();
        let value = self.facts.remove(&key).unwrap_or_default();
        Claim::Woke { name, value }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Shared {
    pub board: Mutex<Board>,
    pub cv: Condvar,
}

impl Shared {
    pub fn new(waiters: Vec<Waiter>) -> Self {
        Self {
            board: Mutex::new(Board::with_waiters(waiters)),
            cv: Condvar::new(),
        }
    }

    pub fn post(&self, key: impl Into<String>, value: impl Into<String>) -> Result<(), Threat> {
        let mut board = self.board.lock().expect("blackboard");
        board.post(key, value)?;
        self.cv.notify_all();
        Ok(())
    }
}

/// The thin scheduler. The guard ends before this function returns.
pub fn tick(lock: &Mutex<Board>) -> Claim {
    let mut board = lock.lock().expect("blackboard");
    board.claim_one()
}

/// Phrases adapted from the Hermes prompt-injection pattern. Filler words are not matched.
pub fn is_injection(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "ignore previous instructions",
        "ignore all instructions",
        "ignore above instructions",
        "ignore prior instructions",
    ]
    .iter()
    .any(|phrase| lower.contains(phrase))
}

pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

/// Run the model only after `tick` has returned, so the lock is not held.
pub fn answer(lock: &Mutex<Board>, claim: &Claim, model: &FakeModel) -> String {
    let _free: MutexGuard<'_, Board> = lock.try_lock().expect("lock held across the model");
    drop(_free);
    match claim {
        Claim::Woke { value, .. } => model.complete(value),
        Claim::Idle | Claim::Paused => model.complete(""),
    }
}

pub fn demo() -> String {
    let board = Mutex::new(Board::with_waiters(vec![Waiter::new(
        "researcher",
        "ready",
    )]));
    board
        .lock()
        .expect("blackboard")
        .post("ready", "notes")
        .expect("clean fact");
    let claim = tick(&board);
    let reply = answer(&board, &claim, &FakeModel { reply: "ok" });
    let name = match &claim {
        Claim::Woke { name, .. } => name.as_str(),
        Claim::Idle => "idle",
        Claim::Paused => "paused",
    };
    format!("blackboard: {name} {reply}")
}
