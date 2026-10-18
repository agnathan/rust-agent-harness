//! A cancellation flag and a budget, checked before every model call.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::event::AgentUsage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    Finished,
    Cancelled,
    Budget,
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

#[derive(Debug, Clone)]
pub struct CancelFlag {
    inner: Arc<AtomicBool>,
}

impl CancelFlag {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.inner.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.inner.load(Ordering::SeqCst)
    }
}

impl Default for CancelFlag {
    fn default() -> Self {
        Self::new()
    }
}

/// The check that sits in front of every model call. A hit means that call
/// does not start.
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
