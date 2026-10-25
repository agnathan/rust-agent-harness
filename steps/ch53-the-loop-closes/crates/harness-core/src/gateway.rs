//! Book chapter 39. The chat gateway. A platform message is allowed, paired,
//! or denied, then pushed onto a bounded inbox. This is not the MCP server.

use std::collections::VecDeque;

/// Hermes pairing alphabet: no I, O, 0, or 1. `gateway/pairing.py`.
pub const ALPHABET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
pub const CODE_LENGTH: usize = 8;
pub const CODE_TTL_SECONDS: u64 = 3600;
pub const RATE_LIMIT_SECONDS: u64 = 600;
pub const LOCKOUT_SECONDS: u64 = 3600;
pub const MAX_PENDING_PER_PLATFORM: usize = 3;
pub const MAX_FAILED_ATTEMPTS: u32 = 5;
pub const BOT_MAX_EVENTS: usize = 20;
pub const BOT_WINDOW_SECONDS: u64 = 300;
pub const BOT_COOLDOWN_SECONDS: u64 = 600;
pub const INBOX_CAP: usize = 64;
pub const SESSION_BUSY: i32 = 4009;
pub const PLATFORM_REQUIRED: i32 = 4023;
pub const UNKNOWN_PLATFORM: i32 = 4024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairError {
    RateLimited,
    Locked,
    TooManyPending,
    BadCode,
    Unknown,
}

pub struct Pairing {
    pending: Vec<(String, u64)>,
    last_request: Option<u64>,
    fails: u32,
    lockout_until: u64,
    paired: bool,
}

impl Default for Pairing {
    fn default() -> Self {
        Self::new()
    }
}

impl Pairing {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            last_request: None,
            fails: 0,
            lockout_until: 0,
            paired: false,
        }
    }

    pub fn paired(&self) -> bool {
        self.paired
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    pub fn request(&mut self, now: u64, code: &str) -> Result<(), PairError> {
        if now < self.lockout_until {
            return Err(PairError::Locked);
        }
        if let Some(last) = self.last_request {
            if now.saturating_sub(last) < RATE_LIMIT_SECONDS {
                return Err(PairError::RateLimited);
            }
        }
        self.pending
            .retain(|(_, created)| now.saturating_sub(*created) <= CODE_TTL_SECONDS);
        if self.pending.len() >= MAX_PENDING_PER_PLATFORM {
            return Err(PairError::TooManyPending);
        }
        if !valid_code(code) {
            return Err(PairError::BadCode);
        }
        self.pending.push((code.to_string(), now));
        self.last_request = Some(now);
        Ok(())
    }

    pub fn approve(&mut self, now: u64, code: &str) -> Result<(), PairError> {
        if now < self.lockout_until {
            return Err(PairError::Locked);
        }
        self.pending
            .retain(|(_, created)| now.saturating_sub(*created) <= CODE_TTL_SECONDS);
        let code = code
            .split_whitespace()
            .collect::<String>()
            .to_ascii_uppercase();
        if let Some(index) = self
            .pending
            .iter()
            .position(|(pending, _)| pending == &code)
        {
            self.pending.remove(index);
            self.fails = 0;
            self.paired = true;
            return Ok(());
        }
        self.fails += 1;
        if self.fails >= MAX_FAILED_ATTEMPTS {
            self.lockout_until = now.saturating_add(LOCKOUT_SECONDS);
            self.fails = 0;
        }
        Err(PairError::Unknown)
    }
}

fn valid_code(code: &str) -> bool {
    code.len() == CODE_LENGTH && code.chars().all(|c| ALPHABET.contains(c))
}

/// Allowlist, then an already-paired caller, then deny.
pub fn authorize(allowlist: &[&str], paired: bool, user: &str) -> bool {
    allowlist.contains(&user) || paired
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BotAdmit {
    Ok,
    Tripped,
    Cooldown,
}

pub struct BotGuard {
    events: VecDeque<u64>,
    cooldown_until: u64,
}

impl BotGuard {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            cooldown_until: 0,
        }
    }

    pub fn admit(&mut self, now: u64) -> BotAdmit {
        if self.cooldown_until > now {
            return BotAdmit::Cooldown;
        }
        if let Some(cutoff) = now.checked_sub(BOT_WINDOW_SECONDS) {
            while self.events.front().copied().is_some_and(|t| t <= cutoff) {
                self.events.pop_front();
            }
        }
        if self.events.len() >= BOT_MAX_EVENTS {
            self.cooldown_until = now.saturating_add(BOT_COOLDOWN_SECONDS);
            self.events.clear();
            return BotAdmit::Tripped;
        }
        self.events.push_back(now);
        BotAdmit::Ok
    }
}

impl Default for BotGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    Present,
    Absent,
    Error,
}

/// True if any sentinel exists. A stat error fails safe.
pub fn is_engaged(paths: &[Stat]) -> bool {
    let mut saw_error = false;
    for path in paths {
        match path {
            Stat::Present => return true,
            Stat::Absent => {}
            Stat::Error => saw_error = true,
        }
    }
    saw_error
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Turn(String),
    Steer(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Submit {
    Accepted,
    Full,
}

pub struct Inbox {
    q: VecDeque<Message>,
    cap: usize,
}

impl Inbox {
    pub fn new(cap: usize) -> Self {
        Self {
            q: VecDeque::with_capacity(cap),
            cap,
        }
    }

    pub fn submit(&mut self, message: Message) -> Submit {
        if self.q.len() >= self.cap {
            return Submit::Full;
        }
        self.q.push_back(message);
        Submit::Accepted
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn front(&self) -> Option<&Message> {
        self.q.front()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chunk {
    Text(String),
    Tool(String),
}

pub fn fold(chunks: &[Chunk]) -> String {
    let mut out = String::new();
    for chunk in chunks {
        if let Chunk::Text(text) = chunk {
            out.push_str(text);
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Handoff {
    Busy { code: i32 },
    PlatformRequired { code: i32 },
    UnknownPlatform { code: i32, platform: String },
    Pending { platform: String },
}

pub fn handoff_request(running: bool, platform: &str, enabled: &[&str]) -> Handoff {
    if running {
        return Handoff::Busy { code: SESSION_BUSY };
    }
    let platform = platform.trim().to_ascii_lowercase();
    if platform.is_empty() {
        return Handoff::PlatformRequired {
            code: PLATFORM_REQUIRED,
        };
    }
    if !enabled.contains(&platform.as_str()) {
        return Handoff::UnknownPlatform {
            code: UNKNOWN_PLATFORM,
            platform,
        };
    }
    Handoff::Pending { platform }
}

pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

pub fn demo() -> String {
    let mut inbox = Inbox::new(INBOX_CAP);
    let accepted = inbox.submit(Message::Turn("hi".to_string()));
    let text = fold(&[
        Chunk::Text("hello".to_string()),
        Chunk::Tool("skip".to_string()),
        Chunk::Text(" there".to_string()),
    ]);
    let _ = FakeModel { reply: "ok" }.complete(&text);
    if accepted == Submit::Accepted && !authorize(&["owner"], false, "stranger") {
        return "gateway: queued".to_string();
    }
    "gateway: open".to_string()
}

use std::sync::OnceLock;

use regex::Regex;

/// Refuse a shell command that would restart or stop this gateway.
/// Hermes `gateway_lifecycle_block` only fires for a supervised gateway.
/// An unsupervised process may still restart the gateway, so `supervised`
/// false returns `None`.
pub fn gateway_lifecycle_block(command: &str, supervised: bool) -> Option<&'static str> {
    if !supervised {
        return None;
    }
    if hermes_gateway().is_match(command)
        || systemctl_gateway().is_match(command)
        || launchctl_gateway().is_match(command)
        || kill_gateway().is_match(command)
    {
        return Some(
            "Blocked: command cannot restart, stop, or uninstall the gateway from inside the gateway process.",
        );
    }
    None
}

pub fn admit_shell(command: &str, supervised: bool) -> Result<(), &'static str> {
    match gateway_lifecycle_block(command, supervised) {
        Some(reason) => Err(reason),
        None => Ok(()),
    }
}

/// Outbound platform text. Chapter 20's egress scrub, not a second redactor.
pub fn send_out(text: &str) -> String {
    crate::redact::redact_for_egress(text)
}

fn hermes_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)(?:^|[^/\w.\-])hermes(?:\.(?:exe|cmd|bat|com|ps1))?\s+gateway\s+(?:restart|stop|uninstall)\b",
        )
        .expect("pattern")
    })
}

fn systemctl_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)systemctl\s+(?:-\S+\s+)*(?:restart|stop|start)\b[^\n]*\bhermes[.\-]?gateway",
        )
        .expect("pattern")
    })
}

fn launchctl_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(r"(?i)launchctl\s+(?:bootout|kickstart|unload)\b[^\n]*\bhermes")
            .expect("pattern")
    })
}

fn kill_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)\b(?:p?kill|taskkill|stop-process)\b[^\n]*\bhermes\b[^\n]*\bgateway|\b(?:p?kill|taskkill|stop-process)\b[^\n]*\bgateway\b[^\n]*\bhermes",
        )
        .expect("pattern")
    })
}
