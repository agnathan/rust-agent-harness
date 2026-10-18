//! Book chapter 41. A due job is claimed once. A crash may release that claim
//! one time. The prompt is scanned before the claim. The watchdog is a
//! separate call.

use std::collections::VecDeque;
use std::sync::OnceLock;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    Threat(&'static str),
    Lifecycle(&'static str),
    Denied,
    Lost,
}

pub struct Job {
    id: String,
    due: u64,
    prompt: String,
    claim: Option<u32>,
    crash_retries: u8,
    done: bool,
    blocked: bool,
}

impl Job {
    pub fn new(id: impl Into<String>, due: u64, prompt: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            due,
            prompt: prompt.into(),
            claim: None,
            crash_retries: 0,
            done: false,
            blocked: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn is_due(&self, now: u64) -> bool {
        !self.done && !self.blocked && self.claim.is_none() && self.due <= now
    }

    /// One owner wins. A second owner loses while the claim is held.
    pub fn claim(&mut self, owner: u32) -> bool {
        if self.done || self.blocked || self.claim.is_some() {
            return false;
        }
        self.claim = Some(owner);
        true
    }

    /// Release the claim once. A second crash leaves it held.
    pub fn crash(&mut self) -> bool {
        if self.claim.is_none() || self.crash_retries >= 1 {
            return false;
        }
        self.claim = None;
        self.crash_retries = 1;
        true
    }

    pub fn claimed_by(&self) -> Option<u32> {
        self.claim
    }
}

pub struct Store {
    jobs: Vec<Job>,
    watchdog_runs: u32,
    chained: u32,
}

impl Store {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            watchdog_runs: 0,
            chained: 0,
        }
    }

    pub fn add(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn jobs_mut(&mut self) -> &mut [Job] {
        &mut self.jobs
    }

    /// Indexes of jobs that are due. Does not tick the watchdog or chain a job.
    pub fn scan_due(&self, now: u64) -> Vec<usize> {
        self.jobs
            .iter()
            .enumerate()
            .filter(|(_, job)| job.is_due(now))
            .map(|(index, _)| index)
            .collect()
    }

    pub fn watchdog(&mut self) {
        self.watchdog_runs += 1;
    }

    pub fn chain(&mut self, inbox: &mut VecDeque<String>) {
        self.chained += 1;
        if let Some(job) = self.jobs.first() {
            inbox.push_back(job.id.clone());
        }
    }

    pub fn watchdog_runs(&self) -> u32 {
        self.watchdog_runs
    }

    pub fn chained(&self) -> u32 {
        self.chained
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

pub fn scan_prompt(prompt: &str) -> Option<&'static str> {
    for (pattern, id) in threat_patterns() {
        if pattern.is_match(prompt) {
            return Some(id);
        }
    }
    None
}

pub fn check_lifecycle(text: &str) -> Result<(), &'static str> {
    if hermes_gateway().is_match(text) {
        return Err("hermes_gateway");
    }
    if systemctl_gateway().is_match(text) {
        return Err("systemctl");
    }
    if kill_gateway().is_match(text) {
        return Err("kill_gateway");
    }
    Ok(())
}

/// Chapter 11 owns the approval pipeline. Cron denies this shape before a run.
pub fn unattended_denies(command: &str) -> bool {
    root_rm().is_match(command)
}

pub fn run_job(job: &mut Job, owner: u32) -> Result<(), RunError> {
    if let Some(id) = scan_prompt(&job.prompt) {
        job.blocked = true;
        return Err(RunError::Threat(id));
    }
    if let Err(reason) = check_lifecycle(&job.prompt) {
        job.blocked = true;
        return Err(RunError::Lifecycle(reason));
    }
    if unattended_denies(&job.prompt) {
        job.blocked = true;
        return Err(RunError::Denied);
    }
    if job.claim(owner) {
        Ok(())
    } else {
        Err(RunError::Lost)
    }
}

fn threat_patterns() -> &'static [(Regex, &'static str)] {
    static PATTERNS: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            (
                Regex::new(r"(?i)ignore\s+(?:\w+\s+)*(?:previous|all|above|prior)\s+(?:\w+\s+)*instructions").expect("pattern"),
                "prompt_injection",
            ),
            (
                Regex::new(r"(?i)do\s+not\s+tell\s+the\s+user").expect("pattern"),
                "deception_hide",
            ),
            (
                Regex::new(r"(?i)system\s+prompt\s+override").expect("pattern"),
                "sys_prompt_override",
            ),
            (
                Regex::new(r"(?i)disregard\s+(your|all|any)\s+(instructions|rules|guidelines)").expect("pattern"),
                "disregard_rules",
            ),
            (
                Regex::new(r"(?i)cat\s+[^\n]*(\.env|credentials|\.netrc|\.pgpass|id_rsa|id_ed25519|id_ecdsa)").expect("pattern"),
                "read_secrets",
            ),
            (Regex::new(r"(?i)authorized_keys").expect("pattern"), "ssh_backdoor"),
            (
                Regex::new(r"(?i)/etc/sudoers|visudo").expect("pattern"),
                "sudoers_mod",
            ),
            (Regex::new(r"(?i)rm\s+-rf\s+/").expect("pattern"), "destructive_root_rm"),
            (
                Regex::new(r#"(?i)curl\s+[^\n]*https?://[^\s"'`]*\$\{?\w*(?:KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|API)\w*\}?"#).expect("pattern"),
                "exfil_curl_url",
            ),
        ]
    })
}

fn hermes_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(r"(?i)(?:^|[^/\w.\-])hermes(?:\.(?:exe|cmd|bat|com|ps1))?\s+gateway\s+(?:restart|stop|uninstall)\b").expect("pattern")
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

fn kill_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(r"(?i)\b(?:p?kill|taskkill|stop-process)\b[^\n]*\bhermes\b[^\n]*\bgateway|\b(?:p?kill|taskkill|stop-process)\b[^\n]*\bgateway\b[^\n]*\bhermes").expect("pattern")
    })
}

fn root_rm() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"(?i)rm\s+-rf\s+/").expect("pattern"))
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
    let mut store = Store::new();
    store.add(Job::new("daily", 0, "summarize the inbox"));
    let job = &mut store.jobs_mut()[0];
    run_job(job, 1).expect("clean prompt");
    let _ = FakeModel { reply: "ok" }.complete("summarize the inbox");
    "cron: claimed".to_string()
}
