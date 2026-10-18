//! Book chapter 32. The critic judges a finished reply. It takes one
//! permit from the supervisor pool and reads the shared context. It does
//! not write the answer. A missing verdict rejects.

use supervisor::Pool;

pub const DEFAULT_MAX_TURNS: u32 = 20;
pub const DEFAULT_JUDGE_TIMEOUT_SECS: u64 = 30;
pub const SNIPPET_CHARS: usize = 4_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Done,
    Blocked,
    Wait,
    Continue,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub accepted: bool,
    pub verdict: Verdict,
    pub snippet: String,
}

pub fn limits() -> (u32, u64) {
    (DEFAULT_MAX_TURNS, DEFAULT_JUDGE_TIMEOUT_SECS)
}

pub fn parse_verdict(raw: &str) -> Verdict {
    let lower = raw.to_ascii_lowercase();
    let Some(pos) = lower.find("verdict") else {
        return Verdict::Reject;
    };
    let rest = lower[pos + "verdict".len()..].trim_start_matches(|ch: char| {
        ch == ':' || ch == '"' || ch == ' ' || ch == '=' || ch == '\''
    });
    if rest.starts_with("done") {
        Verdict::Done
    } else if rest.starts_with("blocked") {
        Verdict::Blocked
    } else if rest.starts_with("wait") {
        Verdict::Wait
    } else if rest.starts_with("continue") {
        Verdict::Continue
    } else {
        Verdict::Reject
    }
}

fn snippet_of(raw: &str) -> String {
    let mut end = SNIPPET_CHARS.min(raw.len());
    while !raw.is_char_boundary(end) {
        end -= 1;
    }
    raw[..end].to_string()
}

fn judge(pool: &Pool, raw: &str) -> Outcome {
    let _permit = pool.acquire();
    let context = pool.context();
    let _notes = context.notes.lock().expect("notes");
    let verdict = parse_verdict(raw);
    Outcome {
        accepted: verdict == Verdict::Done,
        verdict,
        snippet: snippet_of(raw),
    }
}

pub fn as_tool(pool: &Pool, raw: &str) -> Outcome {
    judge(pool, raw)
}

pub fn final_gate(pool: &Pool, raw: &str) -> Result<String, Verdict> {
    let outcome = judge(pool, raw);
    if outcome.accepted {
        Ok(outcome.snippet)
    } else {
        Err(outcome.verdict)
    }
}

pub fn demo() -> String {
    let pool = Pool::new(supervisor::WORKER_BOUND);
    let outcome = as_tool(&pool, "verdict: done\nThe note is short enough.");
    if outcome.accepted {
        "accepted: done".to_string()
    } else {
        "rejected".to_string()
    }
}
