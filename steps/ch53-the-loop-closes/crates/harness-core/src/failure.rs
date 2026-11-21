//! Book chapter 14. A failure has a kind. Empty replies retry, then fall
//! back. A timeout drops the tool permit. A repetition loop is not continued.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub const DEFAULT_EMPTY_RETRY_BUDGET: u32 = 3;
pub const REDUCED_EMPTY_RETRY_BUDGET: u32 = 1;
pub const COST_THRESHOLD_USD: f64 = 0.25;
pub const MIN_FRAGMENT_LENGTH: usize = 400;
pub const REPEAT_WINDOW: usize = 60;
pub const MIN_REPEAT_COUNT: usize = 5;
pub const DOMINANCE_RATIO: f64 = 0.5;
pub const PERMIT_BOUND: usize = 8;

pub const REPETITION_NOTE: &str =
    "[the reply degenerated into a repetition loop and was interrupted]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Empty,
    Truncated,
    Repeated,
    Timeout,
    RateLimit,
    Transport,
    Crash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyAttempt {
    pub model: String,
    pub provider: String,
    pub finish_reason: String,
    pub usage_present: bool,
    pub zero_output: bool,
}

impl EmptyAttempt {
    fn signature(&self) -> (String, String, String) {
        (
            self.model.clone(),
            self.provider.clone(),
            self.finish_reason.clone(),
        )
    }
}

pub fn retry_budget(attempt_cost_usd: Option<f64>) -> u32 {
    match attempt_cost_usd {
        Some(cost) if cost > COST_THRESHOLD_USD => REDUCED_EMPTY_RETRY_BUDGET,
        _ => DEFAULT_EMPTY_RETRY_BUDGET,
    }
}

/// Two consecutive empties, usage present, zero output tokens, same signature.
pub fn deterministic_empty(attempts: &[EmptyAttempt]) -> bool {
    let Some(pair) = attempts.windows(2).last() else {
        return false;
    };
    let (prev, last) = (&pair[0], &pair[1]);
    prev.usage_present
        && last.usage_present
        && prev.zero_output
        && last.zero_output
        && prev.signature() == last.signature()
}

pub fn is_repetition_dominated(text: &str) -> bool {
    let n = text.len();
    if n < MIN_FRAGMENT_LENGTH {
        return false;
    }
    let needed =
        MIN_REPEAT_COUNT.max((n as f64 * DOMINANCE_RATIO / REPEAT_WINDOW as f64).ceil() as usize);
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut start = 0;
    while start + REPEAT_WINDOW <= n {
        if text.is_char_boundary(start) && text.is_char_boundary(start + REPEAT_WINDOW) {
            let window = &text[start..start + REPEAT_WINDOW];
            let count = counts.entry(window).or_insert(0);
            *count += 1;
            if *count >= needed {
                return true;
            }
        }
        start += 1;
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthVerdict {
    Continue { kept: String },
    StopRepeated,
    Finished,
}

pub fn length_verdict(finish_reason: &str, text: &str) -> LengthVerdict {
    if finish_reason != "length" {
        return LengthVerdict::Finished;
    }
    if is_repetition_dominated(text) {
        LengthVerdict::StopRepeated
    } else {
        LengthVerdict::Continue {
            kept: text.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    Empty {
        cost: Option<OrderedF64>,
        usage_present: bool,
        zero_output: bool,
        finish_reason: String,
    },
    Text(String),
    Length(String),
    Fail(Kind),
}

/// `f64` cannot be `Eq`. The fake model stores cents so tests can match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderedF64 {
    pub micros: i64,
}

impl OrderedF64 {
    pub fn usd(dollars: f64) -> Self {
        Self {
            micros: (dollars * 1_000_000.0).round() as i64,
        }
    }

    pub fn as_f64(self) -> f64 {
        self.micros as f64 / 1_000_000.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    pub name: String,
    pub provider: String,
    pub replies: Vec<Reply>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    pub text: String,
    pub kind: Option<Kind>,
    pub attempts: u32,
    pub fallbacks: u32,
}

pub fn run_models(models: &[Model]) -> Turn {
    let mut fallbacks = 0;
    for (index, model) in models.iter().enumerate() {
        if index > 0 {
            fallbacks += 1;
        }
        if let Some(turn) = run_one(model, fallbacks) {
            return turn;
        }
    }
    Turn {
        text: String::new(),
        kind: Some(Kind::Empty),
        attempts: 0,
        fallbacks,
    }
}

fn run_one(model: &Model, fallbacks: u32) -> Option<Turn> {
    let mut attempts_log = Vec::new();
    let mut empties = 0u32;
    let mut budget = DEFAULT_EMPTY_RETRY_BUDGET;
    for (index, reply) in model.replies.iter().enumerate() {
        let attempts = index as u32 + 1;
        match reply {
            Reply::Text(text) => {
                return Some(Turn {
                    text: text.clone(),
                    kind: None,
                    attempts,
                    fallbacks,
                });
            }
            Reply::Length(text) => {
                return Some(match length_verdict("length", text) {
                    LengthVerdict::StopRepeated => Turn {
                        text: REPETITION_NOTE.into(),
                        kind: Some(Kind::Repeated),
                        attempts,
                        fallbacks,
                    },
                    LengthVerdict::Continue { kept } => Turn {
                        text: kept,
                        kind: Some(Kind::Truncated),
                        attempts,
                        fallbacks,
                    },
                    LengthVerdict::Finished => Turn {
                        text: text.clone(),
                        kind: None,
                        attempts,
                        fallbacks,
                    },
                });
            }
            Reply::Fail(kind) => {
                return Some(Turn {
                    text: String::new(),
                    kind: Some(*kind),
                    attempts,
                    fallbacks,
                });
            }
            Reply::Empty {
                cost,
                usage_present,
                zero_output,
                finish_reason,
            } => {
                if empties == 0 {
                    budget = retry_budget(cost.map(OrderedF64::as_f64));
                }
                empties += 1;
                attempts_log.push(EmptyAttempt {
                    model: model.name.clone(),
                    provider: model.provider.clone(),
                    finish_reason: finish_reason.clone(),
                    usage_present: *usage_present,
                    zero_output: *zero_output,
                });
                if deterministic_empty(&attempts_log) || empties > budget {
                    return None;
                }
            }
        }
    }
    None
}

pub struct ToolPermits {
    sem: Arc<Semaphore>,
}

impl ToolPermits {
    pub fn new(bound: usize) -> Self {
        Self {
            sem: Arc::new(Semaphore::new(bound)),
        }
    }

    pub fn available(&self) -> usize {
        self.sem.available_permits()
    }

    pub async fn acquire(&self) -> OwnedSemaphorePermit {
        self.sem
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore open")
    }
}

/// Hold one permit for `work`. A timeout drops it before returning.
pub async fn call_with_timeout<F, Fut>(
    permits: &ToolPermits,
    timeout: Duration,
    work: F,
) -> Result<String, Kind>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = String>,
{
    let permit = permits.acquire().await;
    let result = tokio::time::timeout(timeout, work()).await;
    drop(permit);
    match result {
        Ok(text) => Ok(text),
        Err(_) => Err(Kind::Timeout),
    }
}

/// Retries reuse the same permit. `peak` is the most permits held at once.
pub async fn retry_in_bound(permits: &ToolPermits, times: u32, peak: &AtomicUsize) -> u32 {
    let mut done = 0;
    for _ in 0..times {
        let permit = permits.acquire().await;
        let now = PERMIT_BOUND - permits.available();
        peak.fetch_max(now, Ordering::SeqCst);
        drop(permit);
        done += 1;
    }
    done
}

#[derive(Debug)]
pub struct FakeMcp {
    crashes_left: u32,
    reconnects: u32,
}

impl FakeMcp {
    pub fn new(crashes_left: u32) -> Self {
        Self {
            crashes_left,
            reconnects: 0,
        }
    }

    pub fn reconnects(&self) -> u32 {
        self.reconnects
    }

    pub fn call(&mut self) -> Result<&'static str, Kind> {
        if self.crashes_left > 0 {
            self.crashes_left -= 1;
            self.reconnects += 1;
            return Err(Kind::Crash);
        }
        Ok("pong")
    }

    pub fn call_until_up(&mut self) -> Result<&'static str, Kind> {
        loop {
            match self.call() {
                Err(Kind::Crash) if self.reconnects < 3 => continue,
                other => return other,
            }
        }
    }
}

pub fn repeated_fragment() -> String {
    "abcdefghij".repeat(80)
}

pub fn repetition_probe(text: &str) -> bool {
    is_repetition_dominated(text)
}

pub fn demo_turn() -> Turn {
    let primary = Model {
        name: "primary".into(),
        provider: "openrouter".into(),
        replies: vec![
            Reply::Empty {
                cost: Some(OrderedF64::usd(0.30)),
                usage_present: true,
                zero_output: true,
                finish_reason: "stop".into(),
            },
            Reply::Empty {
                cost: Some(OrderedF64::usd(0.30)),
                usage_present: true,
                zero_output: true,
                finish_reason: "stop".into(),
            },
        ],
    };
    let side = Model {
        name: "side".into(),
        provider: "openrouter".into(),
        replies: vec![Reply::Text("hello".into())],
    };
    run_models(&[primary, side])
}

const GIT_MUTATIONS: &[&str] = &[
    "checkout",
    "switch",
    "rebase",
    "merge",
    "pull",
    "restore",
    "clean",
    "cherry-pick",
    "revert",
    "bisect",
];
const RESET_MODES: &[&str] = &["--hard", "--merge", "--keep"];
const DELETING: &[&str] = &["rm", "rmdir", "rd", "del", "erase", "remove-item", "ri"];

/// `detect_self_repo_git_mutation` in `tools/self_repo_guard.py`.
/// Adapted: global `git -C`, the wrappers `sudo` / `command` / `env`, and the
/// worktree-mutation subcommands. Alias lookup and heredocs are not parsed.
pub fn self_repo_git_mutation(command: &str, source_root: &str) -> bool {
    if command.is_empty() || source_root.is_empty() {
        return false;
    }
    for segment in command.split([';', '\n']) {
        let words = strip_wrappers(segment);
        let Some((git_at, dash_c)) = locate_git(&words) else {
            continue;
        };
        let rest = &words[git_at + 1..];
        let targets =
            dash_c == Some(source_root) || words.iter().any(|word| word_hits(word, source_root));
        if !targets {
            continue;
        }
        let Some(sub) = rest.iter().copied().find(|word| {
            !word.starts_with('-') && !word_hits(word, source_root) && Some(*word) != dash_c
        }) else {
            continue;
        };
        if sub == "reset" {
            if rest.iter().any(|word| RESET_MODES.contains(word)) {
                return true;
            }
            continue;
        }
        if GIT_MUTATIONS.contains(&sub) {
            return true;
        }
    }
    false
}

fn word_hits(word: &str, root: &str) -> bool {
    word == root
        || word.starts_with(&format!("{root}/"))
        || word.starts_with(&format!("{root}\\\\"))
}

fn locate_git<'a>(words: &[&'a str]) -> Option<(usize, Option<&'a str>)> {
    let mut index = 0;
    let mut dash_c = None;
    let mut git_at = None;
    while index < words.len() {
        if words[index] == "-C" && index + 1 < words.len() {
            dash_c = Some(words[index + 1]);
            index += 2;
            continue;
        }
        let base = words[index]
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(words[index]);
        if base == "git" && git_at.is_none() {
            git_at = Some(index);
        }
        index += 1;
    }
    git_at.map(|at| (at, dash_c))
}

fn strip_wrappers(segment: &str) -> Vec<&str> {
    let mut words: Vec<&str> = segment.split_whitespace().collect();
    while let Some(first) = words.first().copied() {
        let base = first.rsplit(['/', '\\']).next().unwrap_or(first);
        if matches!(base, "sudo" | "command" | "time" | "nohup") {
            words.remove(0);
            continue;
        }
        if base == "env" {
            words.remove(0);
            while words.first().is_some_and(|word| word.contains('=')) {
                words.remove(0);
            }
            continue;
        }
        break;
    }
    words
}

fn path_hits(word: &str, runtime_path: &str) -> bool {
    let word = word.trim_matches(|ch| ch == '"' || ch == '\'');
    word == runtime_path
        || runtime_path.starts_with(&format!("{word}/"))
        || runtime_path.starts_with(&format!("{word}\\"))
        || word.starts_with(&format!("{runtime_path}/"))
        || word.starts_with(&format!("{runtime_path}\\"))
}

/// `command_deletes_runtime` in `agent/runtime_self_protection.py`.
/// Adapted: the caller names the runtime path (there is no `sys.executable`).
/// Covers `rm` / `rmdir` / `rd` / `del` / `erase` / `remove-item` / `ri`,
/// `find <path> -delete` and `find <path> -exec rm`, and `uv python uninstall`
/// when the spec is `--all` or appears in the runtime path as `cpython-<spec>`.
pub fn command_deletes_runtime(command: &str, runtime_path: &str) -> bool {
    if command.is_empty() || runtime_path.is_empty() {
        return false;
    }
    let normalized = command.replace("&&", "\n").replace("||", "\n");
    for segment in normalized.split([';', '\n', '|']) {
        let words = strip_wrappers(segment);
        if words.is_empty() {
            continue;
        }
        let name = words[0]
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(words[0])
            .to_ascii_lowercase();
        if name == "uv" && uv_uninstalls_runtime(&words, runtime_path) {
            return true;
        }
        if name == "find"
            && (segment.contains("-delete") || segment.contains("-exec rm"))
            && words
                .iter()
                .skip(1)
                .any(|word| !word.starts_with('-') && path_hits(word, runtime_path))
        {
            return true;
        }
        if !DELETING.iter().any(|item| *item == name) {
            continue;
        }
        if words
            .iter()
            .skip(1)
            .any(|word| !word.starts_with('-') && path_hits(word, runtime_path))
        {
            return true;
        }
    }
    false
}

fn uv_uninstalls_runtime(words: &[&str], runtime_path: &str) -> bool {
    let lower: Vec<String> = words.iter().map(|word| word.to_ascii_lowercase()).collect();
    let Some(python_at) = lower.iter().position(|word| word == "python") else {
        return false;
    };
    let Some(offset) = lower
        .iter()
        .skip(python_at + 1)
        .position(|word| word == "uninstall")
    else {
        return false;
    };
    let uninstall_at = python_at + 1 + offset;
    for spec in words.iter().skip(uninstall_at + 1) {
        let spec = spec.trim_matches(|ch| ch == '"' || ch == '\'');
        if spec == "--all" || spec == "-a" || spec.eq_ignore_ascii_case("all") {
            return true;
        }
        if spec.starts_with('-') || spec.len() < 2 {
            continue;
        }
        if runtime_path.contains(&format!("cpython-{spec}")) || runtime_path.ends_with(spec) {
            return true;
        }
    }
    false
}
