//! Book chapter 31. Patterns nest. The outer loop is a supervisor. An inner
//! fan-out uses the same worker pool and is refused when it would pass the
//! bound. A handoff runs only when ownership should move. The critic's
//! continuation loop, the wait target, and the quality gate live here.

use std::collections::VecDeque;
use std::time::Duration;

use critic::{as_tool, Verdict};
use fanout::{Board as FanBoard, Task};
use handoffs::{Board as HandoffBoard, HandoffError, Message};
use supervisor::{Pool, WORKER_BOUND};

/// Hermes `DEFAULT_MAX_TURNS` in `hermes_cli/goals.py`.
pub const DEFAULT_MAX_TURNS: u32 = 20;
/// Hermes `DEFAULT_JUDGE_TIMEOUT`. This crate does not sleep it.
pub const DEFAULT_JUDGE_TIMEOUT_SECS: u64 = 30;
/// Hermes `DEFAULT_GATE_TIMEOUT_SECONDS`. This crate does not sleep it.
pub const DEFAULT_GATE_TIMEOUT_SECONDS: u32 = 300;
/// Hermes `DEFAULT_GATE_MAX_RETRIES`.
pub const DEFAULT_GATE_MAX_RETRIES: u32 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeError {
    ExceedsBound {
        held: usize,
        asked: usize,
        limit: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    /// The specialist is a tool. The supervisor keeps the conversation.
    Stay,
    /// Ownership should move. The handoff board filters, then accepts.
    Move,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaitTarget {
    Session(String),
    Pid(u32),
    Seconds(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopStop {
    Done,
    Blocked,
    Reject,
    Parked(WaitTarget),
    Turns,
    GatePaused { command: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    pub command: String,
    pub exits: VecDeque<i32>,
    pub attempts: u32,
}

impl Gate {
    pub fn scripted(command: impl Into<String>, exits: impl IntoIterator<Item = i32>) -> Self {
        Self {
            command: command.into(),
            exits: exits.into_iter().collect(),
            attempts: 0,
        }
    }
}

/// A scripted model. `complete` returns the next line and never opens a socket.
pub struct FakeModel {
    lines: VecDeque<String>,
}

impl FakeModel {
    pub fn script(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            lines: lines.into_iter().map(Into::into).collect(),
        }
    }

    pub fn complete(&mut self) -> String {
        self.lines.pop_front().unwrap_or_default()
    }
}

/// Refuse a nested fan-out that would put more workers in flight than the pool allows.
/// `held` is the permits the parent already owns. On success the tasks run on `pool`.
pub fn nested_fanout(pool: &Pool, held: usize, width: usize) -> Result<String, ComposeError> {
    nested_fanout_holding(pool, held, width, Duration::ZERO)
}

pub fn nested_fanout_holding(
    pool: &Pool,
    held: usize,
    width: usize,
    hold: Duration,
) -> Result<String, ComposeError> {
    if held.saturating_add(width) > pool.limit() {
        return Err(ComposeError::ExceedsBound {
            held,
            asked: width,
            limit: pool.limit(),
        });
    }
    let board = FanBoard::new();
    for index in 0..width {
        board
            .schedule(Task {
                task_index: index,
                reply: format!("n{index}"),
                tool_trace: String::new(),
            })
            .expect("width is at most the worker bound, under the queue of 100");
    }
    Ok(board.gather_holding(pool, hold))
}

/// A handoff runs only for `Route::Move`. `Stay` does not touch the board.
pub fn dispatch(
    route: Route,
    board: &HandoffBoard,
    next: &str,
    messages: &[Message],
) -> Result<Option<Vec<Message>>, HandoffError> {
    match route {
        Route::Stay => Ok(None),
        Route::Move => board.handoff(next, messages).map(Some),
    }
}

fn field<'a>(raw: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{key}\"");
    let rest = raw.split_once(&needle)?.1.trim_start();
    let rest = rest.trim_start_matches([':', ' ', '"', '=']);
    let end = rest
        .find(|ch: char| ch == '"' || ch == ',' || ch == '}' || ch.is_whitespace())
        .unwrap_or(rest.len());
    let value = rest[..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

/// Hermes prefers a session, then a pid, then seconds. A wait with none of them is not a target.
pub fn wait_target(raw: &str) -> Option<WaitTarget> {
    if let Some(session) = field(raw, "wait_on_session") {
        return Some(WaitTarget::Session(session.to_string()));
    }
    if let Some(pid) = field(raw, "wait_on_pid").and_then(|text| text.parse().ok()) {
        if pid > 0 {
            return Some(WaitTarget::Pid(pid));
        }
    }
    if let Some(seconds) = field(raw, "wait_for_seconds").and_then(|text| text.parse().ok()) {
        if seconds > 0 {
            return Some(WaitTarget::Seconds(seconds));
        }
    }
    None
}

enum Gates {
    Clear,
    Retry,
    Paused(String),
}

fn check_gates(gates: &mut [Gate]) -> Gates {
    for gate in gates.iter_mut() {
        let code = gate.exits.pop_front().unwrap_or(0);
        if code == 0 {
            gate.attempts = 0;
            continue;
        }
        gate.attempts += 1;
        if gate.attempts > DEFAULT_GATE_MAX_RETRIES {
            return Gates::Paused(gate.command.clone());
        }
        return Gates::Retry;
    }
    Gates::Clear
}

/// Run the critic in a loop. The worker speaks, then the gates run. A failing
/// gate does not call the judge. `Wait` with a target parks and does not burn
/// a turn. This function does not sleep and does not spawn a shell.
pub fn run_continuation(model: &mut FakeModel, pool: &Pool, gates: &mut [Gate]) -> LoopStop {
    let mut turns = 0u32;
    loop {
        if turns >= DEFAULT_MAX_TURNS {
            return LoopStop::Turns;
        }
        let reply = model.complete();
        match check_gates(gates) {
            Gates::Paused(command) => return LoopStop::GatePaused { command },
            Gates::Retry => {
                turns += 1;
                continue;
            }
            Gates::Clear => {}
        }
        let outcome = as_tool(pool, &reply);
        match outcome.verdict {
            Verdict::Done => return LoopStop::Done,
            Verdict::Blocked => return LoopStop::Blocked,
            Verdict::Reject => return LoopStop::Reject,
            Verdict::Wait => match wait_target(&reply) {
                Some(target) => return LoopStop::Parked(target),
                None => turns += 1,
            },
            Verdict::Continue => turns += 1,
        }
    }
}

pub fn limits() -> (u32, u64, u32, u32) {
    (
        DEFAULT_MAX_TURNS,
        DEFAULT_JUDGE_TIMEOUT_SECS,
        DEFAULT_GATE_TIMEOUT_SECONDS,
        DEFAULT_GATE_MAX_RETRIES,
    )
}

pub fn demo() -> String {
    let pool = Pool::new(WORKER_BOUND);
    let gathered = nested_fanout(&pool, 0, 2)
        .expect("two children fit")
        .replace('\n', " | ");
    let board = HandoffBoard::new("supervisor");
    let messages = [Message {
        role: "user".into(),
        text: "ship the note".into(),
    }];
    let stayed = dispatch(Route::Stay, &board, "billing", &messages).expect("stay");
    assert!(stayed.is_none());
    let mut model = FakeModel::script(["verdict: done"]);
    let stop = run_continuation(&mut model, &pool, &mut []);
    let label = match stop {
        LoopStop::Done => "done",
        _ => "not-done",
    };
    format!("fan {gathered}; owner {}; {label}", board.owner())
}
