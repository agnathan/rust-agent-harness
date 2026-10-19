//! Book chapter 26. Nudge counters decide when a turn forks a reviewer.
//! The fork's tool list is skills, plus memory only when the memory nudge
//! fired. Cancelling sets a flag. The turn does not wait out a timeout.

use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub const MEMORY_NUDGE_INTERVAL: u32 = 10;
pub const SKILL_NUDGE_INTERVAL: u32 = 10;
/// Hermes waits this long for a review thread to notice cancel. This crate
/// does not sleep for it. The flag is the cancel.
pub const HERMES_CANCEL_TIMEOUT: Duration = Duration::from_secs(2);

const MEMORY_TOOLS: &[&str] = &[
    "memory",
    "skill_view",
    "skill_manage",
    "read_file",
    "search_files",
];
const SKILL_TOOLS: &[&str] = &["skill_view", "skill_manage", "read_file", "search_files"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewError {
    NotAllowed(String),
    Cancelled,
    Skill(String),
}

pub struct Loop {
    user_turns: u32,
    tool_iters: u32,
    memory_due: bool,
    skill_due: bool,
}

pub struct Fork {
    pub tools: Vec<&'static str>,
    pub memory_nudge_interval: u32,
    pub skill_nudge_interval: u32,
    pub review_memory: bool,
    cancel: Arc<AtomicBool>,
}

pub struct ScriptedModel {
    lines: Vec<String>,
    calls: AtomicUsize,
}

pub struct ReviewRun {
    cancel: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Loop {
    pub fn new() -> Self {
        Self {
            user_turns: 0,
            tool_iters: 0,
            memory_due: false,
            skill_due: false,
        }
    }

    pub fn note_user_turn(&mut self) {
        self.user_turns += 1;
        if self.user_turns.is_multiple_of(MEMORY_NUDGE_INTERVAL) {
            self.memory_due = true;
        }
    }

    pub fn note_tool_iter(&mut self) {
        self.tool_iters += 1;
        if self.tool_iters.is_multiple_of(SKILL_NUDGE_INTERVAL) {
            self.skill_due = true;
        }
    }

    pub fn user_turns(&self) -> u32 {
        self.user_turns
    }

    /// Record the fork when a nudge is due. Returns `None` when this turn
    /// has nothing to review. Building the fork is the work added to
    /// turn finalize.
    pub fn finalize(&mut self) -> Option<Fork> {
        let review_memory = self.memory_due;
        let review_skill = self.skill_due;
        self.memory_due = false;
        self.skill_due = false;
        if !review_memory && !review_skill {
            return None;
        }
        Some(spawn_reviewer(review_memory))
    }
}

impl Default for Loop {
    fn default() -> Self {
        Self::new()
    }
}

impl Fork {
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Acquire)
    }
}

impl ScriptedModel {
    pub fn new(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            lines: lines.into_iter().map(Into::into).collect(),
            calls: AtomicUsize::new(0),
        }
    }

    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::Relaxed)
    }

    fn next(&self) -> Option<String> {
        let index = self.calls.fetch_add(1, Ordering::Relaxed);
        self.lines.get(index).cloned()
    }
}

impl ReviewRun {
    /// A worker that sleeps in 1 ms steps until cancel. The spawn is the
    /// setup. `cancel_and_join` is the return path.
    pub fn spawn_spinner() -> Self {
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&cancel);
        let handle = thread::spawn(move || {
            while !flag.load(Ordering::Acquire) {
                thread::sleep(Duration::from_millis(1));
            }
        });
        Self {
            cancel,
            handle: Some(handle),
        }
    }

    pub fn cancel_and_join(mut self) {
        self.cancel.store(true, Ordering::Release);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

pub fn spawn_reviewer(review_memory: bool) -> Fork {
    let tools = if review_memory {
        MEMORY_TOOLS
    } else {
        SKILL_TOOLS
    };
    Fork {
        tools: tools.to_vec(),
        memory_nudge_interval: 0,
        skill_nudge_interval: 0,
        review_memory,
        cancel: Arc::new(AtomicBool::new(false)),
    }
}

pub fn execute(
    fork: &Fork,
    model: &ScriptedModel,
    notes: &mut Vec<String>,
    skills_root: Option<&Path>,
) -> Result<Vec<String>, ReviewError> {
    let mut done = Vec::new();
    while let Some(line) = model.next() {
        if fork.is_cancelled() {
            return Err(ReviewError::Cancelled);
        }
        let mut parts = line.splitn(2, ' ');
        let tool = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("").trim();
        if !fork.tools.contains(&tool) {
            return Err(ReviewError::NotAllowed(tool.to_string()));
        }
        let result = match tool {
            "memory" => {
                notes.push(arg.to_string());
                arg.to_string()
            }
            "skill_view" => {
                let root = skills_root.ok_or_else(|| ReviewError::Skill("no root".into()))?;
                crate::skills::skill_view(root, arg, None)
                    .map_err(|err| ReviewError::Skill(format!("{err:?}")))?
            }
            "read_file" | "search_files" | "skill_manage" => format!("{tool} {arg}"),
            other => return Err(ReviewError::NotAllowed(other.to_string())),
        };
        done.push(result);
    }
    Ok(done)
}

pub fn demo() -> String {
    let mut loop_ = Loop::new();
    for _ in 0..MEMORY_NUDGE_INTERVAL {
        loop_.note_user_turn();
    }
    let fork = loop_.finalize().expect("fork");
    let model = ScriptedModel::new(["memory prefer short notes"]);
    let mut notes = Vec::new();
    execute(&fork, &model, &mut notes, None).expect("review");
    notes.pop().unwrap_or_default()
}
