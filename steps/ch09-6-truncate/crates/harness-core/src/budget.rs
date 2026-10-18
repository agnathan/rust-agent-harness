//! Hard stop is a [`CancellationToken`]. Soft stop is a flag checked between
//! iterations. The budget is the same gate as before, still checked before
//! a model call starts.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio_util::sync::CancellationToken;

use crate::event::AgentUsage;
use crate::inbox::SessionInbox;
use crate::message::{Message, Role};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    Finished,
    Cancelled,
    SoftStop,
    Budget,
    /// A truncated tool-call argument, or a wrap-up that still wanted tools.
    Partial,
    /// Three batches in a row contained no runnable tool call.
    InvalidToolCalls,
    /// The same call, or a short cycle of calls, stopped making progress.
    Stalled,
}

pub const ITERATION_WARNING_PREFIX: &str = "[SYSTEM NOTICE — iteration budget checkpoint]";
pub const RUN_WRAPUP_NOTICE: &str = "[SYSTEM NOTICE — run time budget nearly exhausted] Run time budget nearly exhausted. Stop new discovery/verification work now. Produce the required final deliverable (answer/JSON/summary) from the state you already have, completing only mandatory writes.";
pub const DEFAULT_SUBAGENT_ITERATIONS: u64 = 50;
pub const DEFAULT_WARNING_RATIO: f64 = 0.5;

#[derive(Debug, Clone)]
pub struct Budget {
    pub max_iterations: u64,
    pub max_tokens: Option<u64>,
    pub max_cost_usd: Option<f64>,
    pub max_duration: Option<Duration>,
    /// Hermes leaves this unset, which turns the warning off. This chapter
    /// turns it on at one half.
    pub warning_ratio: Option<f64>,
    pub max_subagent_iterations: u64,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max_iterations: 8,
            max_tokens: None,
            max_cost_usd: None,
            max_duration: None,
            warning_ratio: Some(DEFAULT_WARNING_RATIO),
            max_subagent_iterations: DEFAULT_SUBAGENT_ITERATIONS,
        }
    }
}

/// A ratio that is not finite and strictly between 0 and 1 turns the warning off.
pub fn normalize_warning_ratio(ratio: f64) -> Option<f64> {
    if ratio.is_finite() && ratio > 0.0 && ratio < 1.0 {
        Some(ratio)
    } else {
        None
    }
}

pub fn iteration_warning(used: u64, maximum: u64) -> String {
    format!(
        "{ITERATION_WARNING_PREFIX} You have used {used} of {maximum} iterations. Checkpoint durable progress now, then continue the task; do not stop solely because of this warning."
    )
}

#[derive(Debug, Default)]
pub struct NoticeLatch {
    pub warning_sent: bool,
    pub wrapup_sent: bool,
}

pub fn inject_budget_notices(
    messages: &mut [Message],
    budget: &Budget,
    usage: &AgentUsage,
    elapsed: Duration,
    cancel: &CancelFlag,
    latch: &mut NoticeLatch,
) {
    if cancel.is_cancelled() {
        return;
    }
    if let Some(ratio) = budget.warning_ratio.and_then(normalize_warning_ratio) {
        if !latch.warning_sent && budget.max_iterations > 1 {
            let scaled = (ratio * budget.max_iterations as f64).floor() as u64;
            let threshold = scaled.min(budget.max_iterations.saturating_sub(1));
            if usage.calls >= threshold {
                let notice = iteration_warning(usage.calls, budget.max_iterations);
                if append_to_newest_tool(messages, &notice) {
                    latch.warning_sent = true;
                }
            }
        }
    }
    if let Some(max) = budget.max_duration {
        if !latch.wrapup_sent
            && !max.is_zero()
            && elapsed >= max.mul_f64(0.8)
            && append_to_newest_tool(messages, RUN_WRAPUP_NOTICE)
        {
            latch.wrapup_sent = true;
        }
    }
}

fn append_to_newest_tool(messages: &mut [Message], notice: &str) -> bool {
    let Some(tool) = messages
        .iter_mut()
        .rev()
        .find(|message| message.role == Role::Tool)
    else {
        return false;
    };
    if tool.content.contains(notice) {
        return false;
    }
    tool.content.push_str("\n\n");
    tool.content.push_str(notice);
    true
}

impl Budget {
    pub fn exceeded(&self, usage: &AgentUsage, started: Instant) -> bool {
        if usage.calls >= self.max_iterations {
            return true;
        }
        if let Some(max) = self.max_tokens {
            if usage.total_tokens > max {
                return true;
            }
        }
        if let Some(max) = self.max_cost_usd {
            if usage.cost_known && usage.cost_usd > max {
                return true;
            }
        }
        if let Some(max) = self.max_duration {
            if started.elapsed() > max {
                return true;
            }
        }
        false
    }
}

/// Hard stop cancels the token. Soft stop sets a flag and leaves the token
/// alone, so an in-flight tool is not killed to deliver the stop.
#[derive(Debug, Clone)]
pub struct CancelFlag {
    token: CancellationToken,
    soft: Arc<AtomicBool>,
}

impl CancelFlag {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
            soft: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Hard stop. Same as [`Self::hard_stop`].
    pub fn cancel(&self) {
        self.hard_stop();
    }

    pub fn hard_stop(&self) {
        self.token.cancel();
    }

    pub fn soft_stop(&self) {
        self.soft.store(true, Ordering::SeqCst);
    }

    /// Clears the soft flag. A pending steer is not this flag's to drop.
    pub fn clear_soft(&self) {
        self.soft.store(false, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }

    pub fn is_soft_stop(&self) -> bool {
        self.soft.load(Ordering::SeqCst)
    }

    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }
}

impl Default for CancelFlag {
    fn default() -> Self {
        Self::new()
    }
}

/// Hard stop drops queued steer text. Soft stop does not, even when this
/// function is called, because the token is still clear.
pub fn drop_pending_steer(cancel: &CancelFlag, inbox: &SessionInbox) {
    if cancel.is_cancelled() {
        while inbox.pop().is_some() {}
    }
}

/// The check that sits in front of every model call. A hit means that call
/// does not start. Soft stop is handled by the loop, which still owes the
/// turn one wrap-up call.
pub fn check_gate(
    cancel: &CancelFlag,
    budget: &Budget,
    usage: &AgentUsage,
    started: Instant,
) -> Option<StopReason> {
    if cancel.is_cancelled() {
        return Some(StopReason::Cancelled);
    }
    if budget.exceeded(usage, started) {
        return Some(StopReason::Budget);
    }
    None
}
