//! Identical tool calls, and short A,B cycles, stop counting as progress.
//! A byte-identical result is replaced with a stub from the second repeat
//! when it is long enough. The turn halts after a fixed number of repeats.

use std::collections::{HashSet, VecDeque};

pub const STALL_NOTICE_AT: u32 = 3;
pub const STALL_HALT_AFTER: u32 = 4;
pub const STUB_MIN_CHARS: usize = 512;
pub const CYCLE_HALT_LAPS: u32 = 3;
pub const MAX_CYCLE_PERIOD: usize = 4;
const HISTORY_WINDOW: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seen {
    iter: u64,
    tool: String,
    args: String,
    result: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hist {
    iter: u64,
    tool: String,
    args: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StallAction {
    pub text: String,
    pub halt: bool,
}

#[derive(Debug, Default)]
pub struct StallGuard {
    last: Option<Seen>,
    streak: u32,
    history: VecDeque<Hist>,
}

impl StallGuard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn observe(
        &mut self,
        iter: u64,
        tool: &str,
        args: &str,
        result: &str,
        failed: bool,
    ) -> StallAction {
        self.history.push_back(Hist {
            iter,
            tool: tool.to_string(),
            args: args.to_string(),
        });
        if self.history.len() > HISTORY_WINDOW {
            self.history.pop_front();
        }
        let sibling = self.last.as_ref().is_some_and(|prev| prev.iter == iter);
        let identical = !sibling
            && self.last.as_ref().is_some_and(|prev| {
                prev.tool == tool && prev.args == args && prev.result == result
            });
        if !sibling {
            if identical {
                self.streak += 1;
            } else {
                self.streak = 1;
            }
            self.last = Some(Seen {
                iter,
                tool: tool.to_string(),
                args: args.to_string(),
                result: result.to_string(),
            });
        }
        let mut text = if identical && !failed && result.len() >= STUB_MIN_CHARS && self.streak >= 2
        {
            stub_text(tool, args)
        } else {
            result.to_string()
        };
        if self.streak >= STALL_NOTICE_AT {
            text.push_str(&format!(
                "\n[note: identical call repeated {} times this turn]",
                self.streak
            ));
        }
        let mut halt = self.streak >= STALL_HALT_AFTER;
        if let Some(laps) = cycle_laps(&self.history) {
            if laps >= CYCLE_HALT_LAPS {
                halt = true;
                text.push_str(&format!("\n[note: tool-call cycle repeated {laps} laps]"));
            }
        }
        StallAction { text, halt }
    }
}

pub fn stub_text(tool: &str, args_preview: &str) -> String {
    let preview: String = args_preview.chars().take(80).collect();
    format!(
        "[note: this result is byte-identical to the {tool} result earlier this turn. Refer to that result; it has not changed. Args: {preview}]"
    )
}

fn cycle_laps(history: &VecDeque<Hist>) -> Option<u32> {
    let items: Vec<&Hist> = history.iter().collect();
    let len = items.len();
    let mut best = 0u32;
    for period in 2..=MAX_CYCLE_PERIOD {
        if len < period * 2 {
            continue;
        }
        let mut offset = len;
        let pattern_start = len - period;
        let mut laps = 0u32;
        let mut iters = HashSet::new();
        while offset >= period {
            let window = &items[offset - period..offset];
            let base = &items[pattern_start..pattern_start + period];
            let same = window
                .iter()
                .zip(base.iter())
                .all(|(left, right)| left.tool == right.tool && left.args == right.args);
            if !same {
                break;
            }
            laps += 1;
            for item in window {
                iters.insert(item.iter);
            }
            offset -= period;
        }
        if laps >= 2 && iters.len() >= 2 && laps > best {
            best = laps;
        }
    }
    if best >= 2 {
        Some(best)
    } else {
        None
    }
}
