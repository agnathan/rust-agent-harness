//! Book chapter 11. One pipeline decides a shell command before it runs.
//! Hardline matches are refused even when the session is in YOLO. A human
//! wait takes the tool-concurrency permit back before it blocks.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, Notify, Semaphore};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(300);
pub const DENIAL_BREAKER_THRESHOLD: u32 = 3;
pub const TOOL_PERMIT_BOUND: usize = 8;

pub const STOP_COMMAND: &str = " The user has NOT consented to this action. Do NOT retry this command, do NOT rephrase it, and do NOT attempt the same outcome via a different command. Stop the current workflow and wait for the user to respond before taking any further destructive or irreversible action.";

pub const BREAKER_TEXT: &str = " CIRCUIT BREAKER: consecutive commands were blocked by the security reviewer. STOP attempting variations of this operation. Report the blocked operation to the user and either ask them to run it manually or use /approve.";

/// How risky a call looks before any pattern runs. The pipeline, not this
/// label, decides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Safe,
    Sensitive,
    Destructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `approvals.mode: "off"`. Floors still run.
    Off,
    /// A second model, then a human if it will not approve.
    Smart,
    /// Always ask a human for a dangerous command.
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unattended {
    Deny,
    Approve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    Interactive,
    Cron,
    SingleQuery,
    Unattended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub mode: Mode,
    pub timeout: Duration,
    pub cron_mode: Unattended,
    pub single_query_mode: Unattended,
    pub unattended_mode: Unattended,
    pub denial_breaker_threshold: u32,
    pub yolo: bool,
    pub deny: Vec<String>,
    pub allowlist: Vec<String>,
    pub presence: Presence,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            mode: Mode::Manual,
            timeout: DEFAULT_TIMEOUT,
            cron_mode: Unattended::Deny,
            single_query_mode: Unattended::Deny,
            unattended_mode: Unattended::Deny,
            denial_breaker_threshold: DENIAL_BREAKER_THRESHOLD,
            yolo: false,
            deny: Vec::new(),
            allowlist: Vec::new(),
            presence: Presence::Interactive,
        }
    }
}

impl Policy {
    pub fn manual() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    denials: u32,
}

impl Session {
    pub fn new() -> Self {
        Self { denials: 0 }
    }

    pub fn denials(&self) -> u32 {
        self.denials
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartVote {
    Approve,
    Deny,
    Escalate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Allow { risk: Risk },
    Deny { message: String, hardline: bool },
    Ask { description: String, risk: Risk },
}

impl Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Allow { .. } => "allow",
            Verdict::Deny { .. } => "deny",
            Verdict::Ask { .. } => "ask",
        }
    }

    pub fn is_allow(&self) -> bool {
        matches!(self, Verdict::Allow { .. })
    }
}

/// One scripted tool call from a fake model. No HTTP and no key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptCall {
    pub command: String,
}

/// What a second model would say. The default denies a recursive delete and
/// escalates anything else that reached it.
pub fn fake_smart_review(command: &str) -> SmartVote {
    let normalized = normalize(command);
    if normalized.contains("curl") {
        SmartVote::Deny
    } else {
        SmartVote::Escalate
    }
}

pub fn risk_of(command: &str) -> Risk {
    if hardline(command).is_some() || pipes_password_to_sudo(command) {
        Risk::Destructive
    } else if dangerous(command).is_some() {
        Risk::Sensitive
    } else {
        Risk::Safe
    }
}

/// The pipeline. Order is the floor (hardline, `sudo -S`, deny list), then
/// YOLO or mode off, then the allowlist, then unattended deny, then smart, then ask.
pub fn check(policy: &Policy, session: &mut Session, command: &str) -> Verdict {
    if policy.deny.is_empty() && fast_safe(command) {
        session.denials = 0;
        return Verdict::Allow { risk: Risk::Safe };
    }
    if let Some(description) = hardline(command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: format!(
                "{}{STOP_COMMAND}{}",
                hardline_message(&description),
                breaker_addendum(session, policy)
            ),
            hardline: true,
        };
    }
    if pipes_password_to_sudo(command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: format!(
                "BLOCKED: password piping to sudo -S. Do not pipe passwords to 'sudo -S'.{STOP_COMMAND}{}",
                breaker_addendum(session, policy)
            ),
            hardline: false,
        };
    }
    if let Some(rule) = deny_match(policy, command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: format!(
                "BLOCKED: user deny rule `{rule}` matched.{STOP_COMMAND}{}",
                breaker_addendum(session, policy)
            ),
            hardline: false,
        };
    }
    if policy.yolo || policy.mode == Mode::Off {
        session.denials = 0;
        return Verdict::Allow {
            risk: risk_of(command),
        };
    }
    let normalized = normalize(&mask_quotes(command));
    if policy
        .allowlist
        .iter()
        .any(|entry| normalize(entry) == normalized)
    {
        session.denials = 0;
        return Verdict::Allow { risk: Risk::Safe };
    }
    let danger = dangerous(command);
    if danger.is_none() {
        session.denials = 0;
        return Verdict::Allow { risk: Risk::Safe };
    }
    let description = danger.expect("checked");
    if let Some(mode) = unattended_mode(policy) {
        if mode == Unattended::Deny {
            session.denials = session.denials.saturating_add(1);
            return Verdict::Deny {
                message: format!(
                    "BLOCKED: dangerous command in an unattended session ({description}).{STOP_COMMAND}{}",
                    breaker_addendum(session, policy)
                ),
                hardline: false,
            };
        }
        session.denials = 0;
        return Verdict::Allow {
            risk: Risk::Sensitive,
        };
    }
    if policy.mode == Mode::Smart {
        match fake_smart_review(command) {
            SmartVote::Approve => {
                session.denials = 0;
                return Verdict::Allow {
                    risk: Risk::Sensitive,
                };
            }
            SmartVote::Deny => {
                session.denials = session.denials.saturating_add(1);
                return Verdict::Deny {
                    message: format!(
                        "BLOCKED: smart approval denied this command ({description}).{STOP_COMMAND}{}",
                        breaker_addendum(session, policy)
                    ),
                    hardline: false,
                };
            }
            SmartVote::Escalate => {}
        }
    }
    Verdict::Ask {
        description,
        risk: Risk::Sensitive,
    }
}

/// Run the fake model until a denial. A denied command is not followed by the
/// next scripted call.
pub fn run_script(policy: &Policy, calls: &[ScriptCall]) -> Vec<Verdict> {
    let mut session = Session::new();
    let mut out = Vec::new();
    for call in calls {
        let verdict = check(policy, &mut session, &call.command);
        let stop = matches!(verdict, Verdict::Deny { .. });
        out.push(verdict);
        if stop {
            break;
        }
    }
    out
}

fn fast_safe(command: &str) -> bool {
    let bytes = command.as_bytes();
    if bytes.iter().any(|byte| {
        matches!(
            byte,
            b'"' | b'\'' | b'$' | b'`' | b'|' | b';' | b'>' | b'(' | b')' | b'&'
        )
    }) {
        return false;
    }
    const WORDS: &[&str] = &[
        "rm", "sudo", "mkfs", "dd", "curl", "kill", "shutdown", "reboot", "halt", "poweroff",
        "python", "python3", "perl", "ruby", "node", "bash", "sh", "eval", "nohup",
    ];
    !WORDS.iter().any(|word| contains_word_ci(command, word))
}

fn contains_word_ci(hay: &str, word: &str) -> bool {
    let h = hay.as_bytes();
    let w = word.as_bytes();
    if w.is_empty() || w.len() > h.len() {
        return false;
    }
    let last = h.len() - w.len();
    for start in 0..=last {
        if start > 0 && is_word_byte(h[start - 1]) {
            continue;
        }
        let end = start + w.len();
        if end < h.len() && is_word_byte(h[end]) {
            continue;
        }
        if h[start..end].eq_ignore_ascii_case(w) {
            return true;
        }
    }
    false
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
}

fn hardline_message(description: &str) -> String {
    format!(
        "BLOCKED (hardline): {description}. This command is on the unconditional blocklist and cannot be executed via the agent — not even with --yolo, /yolo, approvals.mode=off, or cron approve mode. If you genuinely need to run it, run it yourself in a terminal outside the agent."
    )
}

fn breaker_addendum(session: &Session, policy: &Policy) -> String {
    if policy.denial_breaker_threshold == 0 {
        return String::new();
    }
    if session.denials >= policy.denial_breaker_threshold {
        format!(
            " CIRCUIT BREAKER: {} consecutive commands were blocked by the security reviewer. STOP attempting variations of this operation. Report the blocked operation to the user and either ask them to run it manually or use /approve.",
            session.denials
        )
    } else {
        String::new()
    }
}

fn unattended_mode(policy: &Policy) -> Option<Unattended> {
    match policy.presence {
        Presence::Interactive => None,
        Presence::Cron => Some(policy.cron_mode),
        Presence::SingleQuery => Some(policy.single_query_mode),
        Presence::Unattended => Some(policy.unattended_mode),
    }
}

fn deny_match(policy: &Policy, command: &str) -> Option<String> {
    let masked = normalize(&mask_quotes(command));
    policy.deny.iter().find_map(|rule| {
        if masked.contains(rule) {
            Some(rule.clone())
        } else {
            None
        }
    })
}

/// Strip `sudo`, `nohup`, `command`, `time`, and `env NAME=value` wrappers.
pub fn normalize(command: &str) -> String {
    let mut parts: Vec<&str> = command.split_whitespace().collect();
    while let Some(first) = parts.first().copied() {
        if matches!(first, "sudo" | "nohup" | "command" | "time") {
            parts.remove(0);
            continue;
        }
        if first == "env" {
            parts.remove(0);
            while parts.first().is_some_and(|token| token.contains('=')) {
                parts.remove(0);
            }
            continue;
        }
        break;
    }
    parts.join(" ")
}

/// Replace quoted spans with spaces so `echo "rm -rf /"` is not a hardline hit.
pub fn mask_quotes(command: &str) -> String {
    let mut out = String::with_capacity(command.len());
    let mut chars = command.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '"' || ch == '\'' {
            let quote = ch;
            out.push(' ');
            for inner in chars.by_ref() {
                if inner == quote {
                    break;
                }
                out.push(' ');
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn variants(command: &str) -> Vec<String> {
    let mut found = vec![normalize(&mask_quotes(command))];
    if let Some(payload) = shell_c_payload(command) {
        found.push(normalize(&payload));
    }
    found
}

fn shell_c_payload(command: &str) -> Option<String> {
    let lowered = command.to_ascii_lowercase();
    let marker = ["sh -c ", "bash -c ", "eval "]
        .into_iter()
        .find(|marker| lowered.contains(marker))?;
    let index = lowered.find(marker)? + marker.len();
    let rest = command[index..].trim();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return Some(rest.to_string());
    }
    let inner = &rest[quote.len_utf8()..];
    Some(inner.split(quote).next().unwrap_or(inner).to_string())
}

fn hardline(command: &str) -> Option<String> {
    for text in variants(command) {
        if let Some(description) = hardline_text(&text) {
            return Some(description);
        }
    }
    None
}

fn hardline_text(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    if recursive_delete_of_root(&lower) {
        return Some("recursive delete of root filesystem".to_string());
    }
    if [
        "/etc", "/etc/*", "/boot", "/boot/*", "/usr", "/usr/*", "~", "~/", "~/*", "$home",
    ]
    .into_iter()
    .any(|path| recursive_delete_of(&lower, path))
    {
        return Some("recursive delete of system directory".to_string());
    }
    if command_word(&lower, "mkfs") {
        return Some("format filesystem (mkfs)".to_string());
    }
    if lower.contains("dd")
        && (lower.contains("of=/dev/sd") || lower.contains("of=/dev/nvme"))
        && command_word(&lower, "dd")
    {
        return Some("dd to raw block device".to_string());
    }
    if lower.contains(":(){") && lower.contains(":|:&") {
        return Some("fork bomb".to_string());
    }
    if kill_all(&lower) {
        return Some("kill all processes".to_string());
    }
    for word in ["shutdown", "reboot", "halt", "poweroff"] {
        if command_word(&lower, word) {
            return Some("system shutdown/reboot".to_string());
        }
    }
    None
}

fn recursive_delete_of_root(lower: &str) -> bool {
    if !is_rm_recursive(lower) {
        return false;
    }
    lower.split_whitespace().any(|token| {
        token == "/" || token == "/*" || token == "/." || token == "/./" || token == "//"
    })
}

fn recursive_delete_of(lower: &str, path: &str) -> bool {
    is_rm_recursive(lower) && lower.split_whitespace().any(|token| token == path)
}

fn is_rm_recursive(lower: &str) -> bool {
    let mut saw_rm = false;
    for token in lower.split_whitespace() {
        if token == "rm" {
            saw_rm = true;
            continue;
        }
        if saw_rm && token.starts_with('-') && token.contains('r') {
            return true;
        }
        if saw_rm && !token.starts_with('-') {
            saw_rm = false;
        }
    }
    false
}

fn command_word(lower: &str, word: &str) -> bool {
    lower.split_whitespace().any(|token| token == word)
}

fn kill_all(lower: &str) -> bool {
    let tokens: Vec<&str> = lower.split_whitespace().collect();
    tokens
        .windows(2)
        .any(|pair| pair[0] == "kill" && pair[1] == "-1")
}

fn pipes_password_to_sudo(command: &str) -> bool {
    let masked = mask_quotes(command);
    masked.contains("-S") && masked.to_ascii_lowercase().contains("sudo")
}

fn dangerous(command: &str) -> Option<String> {
    for text in variants(command) {
        if let Some(description) = dangerous_text(&text) {
            return Some(description);
        }
    }
    None
}

fn dangerous_text(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    if is_rm_recursive(&lower) {
        return Some("recursive delete".to_string());
    }
    if lower.split_whitespace().any(|token| {
        token.starts_with("/etc") || token.contains("/.ssh") || token.contains("/.env")
    }) {
        return Some("write to a sensitive path".to_string());
    }
    if lower.contains("curl") && (lower.contains("--upload-file") || lower.contains(" -t ")) {
        return Some("curl upload".to_string());
    }
    for word in ["python", "python3", "perl", "ruby", "node"] {
        if command_word(&lower, word)
            && lower
                .split_whitespace()
                .any(|token| token == "-c" || token == "-e")
        {
            return Some("interpreter inline code".to_string());
        }
    }
    None
}

/// One human prompt. Concurrent callers take the mutex in turn.
pub struct Prompt {
    lock: Mutex<()>,
    inside: AtomicUsize,
    peak: AtomicUsize,
    permits_seen: AtomicUsize,
    answer: Notify,
    answered: AtomicUsize,
    /// `0` wait, `1` allow, `2` deny.
    script: AtomicUsize,
}

impl Prompt {
    pub fn allow() -> Self {
        Self::scripted(1)
    }

    pub fn deny() -> Self {
        Self::scripted(2)
    }

    /// Never signals. The wait hits the timeout.
    pub fn silent() -> Self {
        Self::scripted(0)
    }

    fn scripted(script: usize) -> Self {
        Self {
            lock: Mutex::new(()),
            inside: AtomicUsize::new(0),
            peak: AtomicUsize::new(0),
            permits_seen: AtomicUsize::new(0),
            answer: Notify::new(),
            answered: AtomicUsize::new(0),
            script: AtomicUsize::new(script),
        }
    }

    pub fn peak(&self) -> usize {
        self.peak.load(Ordering::SeqCst)
    }

    pub fn permits_seen_free(&self) -> bool {
        self.permits_seen.load(Ordering::SeqCst) > 0
    }

    pub fn release(&self) {
        self.answered.store(1, Ordering::SeqCst);
        self.answer.notify_waiters();
    }

    async fn wait(&self, timeout: Duration, permits_free: usize) -> bool {
        let _guard = self.lock.lock().await;
        let now = self.inside.fetch_add(1, Ordering::SeqCst) + 1;
        self.peak.fetch_max(now, Ordering::SeqCst);
        if permits_free > 0 {
            self.permits_seen.fetch_add(1, Ordering::SeqCst);
        }
        let script = self.script.load(Ordering::SeqCst);
        let allowed = if script == 0 {
            timeout_or_release(&self.answer, &self.answered, timeout).await
        } else {
            script == 1
        };
        self.inside.fetch_sub(1, Ordering::SeqCst);
        allowed
    }
}

async fn timeout_or_release(answer: &Notify, answered: &AtomicUsize, timeout: Duration) -> bool {
    if answered.load(Ordering::SeqCst) == 1 {
        return true;
    }
    match tokio::time::timeout(timeout, answer.notified()).await {
        Ok(()) => answered.load(Ordering::SeqCst) == 1,
        Err(_) => false,
    }
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
}

/// Run one command. If a human has to answer, the permit is dropped first.
pub async fn run_call(
    policy: &Policy,
    session: &Mutex<Session>,
    prompt: &Prompt,
    permits: &ToolPermits,
    command: &str,
) -> Verdict {
    let permit = permits
        .sem
        .clone()
        .acquire_owned()
        .await
        .expect("semaphore open");
    let verdict = {
        let mut guard = session.lock().await;
        check(policy, &mut guard, command)
    };
    match verdict {
        Verdict::Ask { risk, .. } => {
            drop(permit);
            let free = permits.available();
            let allowed = prompt.wait(policy.timeout, free).await;
            if allowed {
                {
                    let mut guard = session.lock().await;
                    guard.denials = 0;
                }
                let _permit = permits
                    .sem
                    .clone()
                    .acquire_owned()
                    .await
                    .expect("semaphore open");
                Verdict::Allow { risk }
            } else {
                let mut guard = session.lock().await;
                guard.denials = guard.denials.saturating_add(1);
                let extra = breaker_addendum(&guard, policy);
                Verdict::Deny {
                    message: format!(
                        "BLOCKED: Command timed out without user response.{STOP_COMMAND} Silence is not consent.{extra}"
                    ),
                    hardline: false,
                }
            }
        }
        other => other,
    }
}

/// A skip of a safe command. The bench calls this.
pub fn skip_decision(command: &str) -> Verdict {
    let mut session = Session::new();
    check(&Policy::manual(), &mut session, command)
}
