//! Book chapter 29. A supervisor runs specialists as tools, at most eight
//! at once, on one shared context. A child cannot call the blocked tools.
//! Depth is 1. A worktree is opt-in and is not created here.

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub const WORKER_BOUND: usize = 8;
/// Hermes `delegation.max_concurrent_children`. The pool uses `WORKER_BOUND`.
pub const HERMES_MAX_CONCURRENT_CHILDREN: usize = 10;
pub const MAX_DEPTH: u8 = 1;
pub const BLOCKED_TOOLS: &[&str] = &[
    "delegate_task",
    "clarify",
    "memory",
    "send_message",
    "cronjob_manage",
];
const QUEUE_CAP: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamError {
    Blocked(String),
}

pub struct Context {
    pub notes: Mutex<Vec<String>>,
}

pub struct Job {
    pub name: &'static str,
    pub reply: String,
    pub hold: Option<Duration>,
}

struct State {
    inflight: usize,
    max_seen: usize,
    reflection: VecDeque<String>,
}

pub struct Pool {
    limit: usize,
    state: Mutex<State>,
    cv: Condvar,
    context: Arc<Context>,
}

pub struct Permit<'a> {
    pool: &'a Pool,
}

impl Pool {
    pub fn new(limit: usize) -> Self {
        Self {
            limit: limit.max(1),
            state: Mutex::new(State {
                inflight: 0,
                max_seen: 0,
                reflection: VecDeque::new(),
            }),
            cv: Condvar::new(),
            context: Arc::new(Context {
                notes: Mutex::new(Vec::new()),
            }),
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn context(&self) -> Arc<Context> {
        Arc::clone(&self.context)
    }

    pub fn max_seen(&self) -> usize {
        self.state.lock().expect("state").max_seen
    }

    pub fn reflection_len(&self) -> usize {
        self.state.lock().expect("state").reflection.len()
    }

    pub fn enqueue_reflection(&self, note: &str) {
        let mut state = self.state.lock().expect("state");
        if state.reflection.len() == QUEUE_CAP {
            state.reflection.pop_front();
        }
        state.reflection.push_back(note.to_string());
    }

    pub fn acquire(&self) -> Permit<'_> {
        let mut state = self.state.lock().expect("state");
        while state.inflight >= self.limit {
            state = self.cv.wait(state).expect("state");
        }
        state.inflight += 1;
        state.max_seen = state.max_seen.max(state.inflight);
        Permit { pool: self }
    }
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let mut state = self.pool.state.lock().expect("state");
        state.inflight = state.inflight.saturating_sub(1);
        self.pool.cv.notify_one();
    }
}

pub fn run_specialists(pool: &Pool, jobs: &[Job]) -> Vec<String> {
    thread::scope(|scope| {
        let handles: Vec<_> = jobs
            .iter()
            .map(|job| {
                scope.spawn(|| {
                    let _permit = pool.acquire();
                    if let Some(hold) = job.hold {
                        thread::sleep(hold);
                    }
                    let _shared = pool.context();
                    job.reply.clone()
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("specialist"))
            .collect()
    })
}

pub fn child_call(tool: &str, depth: u8) -> Result<(), TeamError> {
    if depth >= MAX_DEPTH && BLOCKED_TOOLS.contains(&tool) {
        return Err(TeamError::Blocked(tool.to_string()));
    }
    if depth > MAX_DEPTH {
        return Err(TeamError::Blocked(tool.to_string()));
    }
    Ok(())
}

/// `delegation.worktree_isolation` defaults to false. Opt-in returns a
/// marker. This function does not create a git worktree.
pub fn worktree(opt_in: bool) -> Option<&'static str> {
    if opt_in {
        Some("opt-in")
    } else {
        None
    }
}

pub fn demo() -> String {
    let pool = Pool::new(WORKER_BOUND);
    let jobs = [
        Job {
            name: "outline",
            reply: "three steps".into(),
            hold: None,
        },
        Job {
            name: "draft",
            reply: "the draft".into(),
            hold: None,
        },
    ];
    let replies = run_specialists(&pool, &jobs);
    pool.enqueue_reflection("team run finished");
    format!("{} | {}", replies[0], replies[1])
}
