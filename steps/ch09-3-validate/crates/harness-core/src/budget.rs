//! Hard stop is a [`CancellationToken`]. Soft stop is a flag checked between
//! iterations. The budget is the same gate as before, still checked before
//! a model call starts.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio_util::sync::CancellationToken;

use crate::event::AgentUsage;
use crate::inbox::SessionInbox;

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
}

#[derive(Debug, Clone)]
pub struct Budget {
    pub max_iterations: u64,
    pub max_tokens: Option<u64>,
    pub max_cost_usd: Option<f64>,
    pub max_duration: Option<Duration>,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max_iterations: 8,
            max_tokens: None,
            max_cost_usd: None,
            max_duration: None,
        }
    }
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
