//! Book chapter 23. A write is scanned, then either saved or staged.
//! The approval gate is off unless it is turned on. A reviewer must
//! have loaded the file it patches.

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use regex::Regex;
use skills::SkillError;

pub const MAX_FILE_BYTES: usize = 1_048_576;
const QUEUE_CAP: usize = 64;

const SUPPORT: &[&str] = &["references", "templates", "scripts", "assets"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    User,
    Agent,
    Reviewer,
    Librarian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trust {
    Builtin,
    Trusted,
    Community,
    AgentCreated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Safe,
    Caution,
    Dangerous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub pattern_id: String,
    pub severity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub verdict: Verdict,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Block,
    Ask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteError {
    Scan(String),
    Staged(String),
    Unread(String),
    Pinned(String),
    Origin(String),
    Missing(String),
    Path(String),
    Io(String),
    Skill(String),
}

struct Rule {
    id: &'static str,
    severity: &'static str,
    regex: Regex,
}

pub struct Writer {
    root: PathBuf,
    approval: Mutex<bool>,
    viewed: Mutex<HashMap<String, u64>>,
    staged: Mutex<Vec<String>>,
    pinned: Mutex<Vec<String>>,
    queue: Mutex<VecDeque<String>>,
}

impl Writer {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, WriteError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|err| WriteError::Io(err.to_string()))?;
        Ok(Self {
            root,
            approval: Mutex::new(false),
            viewed: Mutex::new(HashMap::new()),
            staged: Mutex::new(Vec::new()),
            pinned: Mutex::new(Vec::new()),
            queue: Mutex::new(VecDeque::new()),
        })
    }

    pub fn set_write_approval(&self, on: bool) {
        *self.approval.lock().expect("approval") = on;
    }

    pub fn write_approval(&self) -> bool {
        *self.approval.lock().expect("approval")
    }

    pub fn staged(&self) -> Vec<String> {
        self.staged.lock().expect("staged").clone()
    }

    pub fn pin(&self, name: &str) {
        let mut pinned = self.pinned.lock().expect("pinned");
        if !pinned.iter().any(|got| got == name) {
            pinned.push(name.to_string());
        }
    }

    pub fn enqueue_background(&self, job: &str) {
        let mut queue = self.queue.lock().expect("queue");
        if queue.len() == QUEUE_CAP {
            queue.pop_front();
        }
        queue.push_back(job.to_string());
    }

    pub fn queue_len(&self) -> usize {
        self.queue.lock().expect("queue").len()
    }

    pub fn view(&self, name: &str, file: Option<&str>, actor: Actor) -> Result<String, WriteError> {
        let text = skills::skill_view(&self.root, name, file).map_err(skill_err)?;
        if actor == Actor::Reviewer {
            let key = view_key(name, file);
            self.viewed
                .lock()
                .expect("viewed")
                .insert(key, fnv(text.as_bytes()));
        }
        Ok(text)
    }

    pub fn create(&self, category: &str, raw: &str, actor: Actor) -> Result<(), WriteError> {
        let trust = if actor == Actor::Agent || actor == Actor::Reviewer {
            Trust::AgentCreated
        } else {
            Trust::Community
        };
        self.write_skill(category, raw, trust, false, actor)
    }

    pub fn install(
        &self,
        category: &str,
        raw: &str,
        trust: Trust,
        force: bool,
    ) -> Result<(), WriteError> {
        self.write_skill(category, raw, trust, force, Actor::User)
    }

    pub fn patch(
        &self,
        name: &str,
        file: Option<&str>,
        find: &str,
        replace: &str,
        actor: Actor,
    ) -> Result<(), WriteError> {
        let path = self.target(name, file)?;
        let current = fs::read_to_string(&path).map_err(|err| WriteError::Io(err.to_string()))?;
        self.require_read(name, file, &current, actor)?;
        let count = current.matches(find).count();
        if count != 1 {
            return Err(WriteError::Missing(format!("{find} matched {count} times")));
        }
        let next = current.replacen(find, replace, 1);
        self.scan_or_stop(&next, Trust::AgentCreated, false)?;
        let label = format!("patch {name}");
        if self.hold(&label) {
            return Err(WriteError::Staged(label));
        }
        atomic_write(&path, next.as_bytes())
    }

    pub fn write_file(
        &self,
        name: &str,
        rel: &str,
        bytes: &[u8],
        actor: Actor,
    ) -> Result<(), WriteError> {
        if bytes.len() > MAX_FILE_BYTES {
            return Err(WriteError::Path(format!("{} bytes", bytes.len())));
        }
        let text = std::str::from_utf8(bytes).map_err(|_| WriteError::Path(rel.to_string()))?;
        self.scan_or_stop(text, Trust::AgentCreated, false)?;
        let dir = find_skill_dir(&self.root, name)?;
        let path = support_path(&dir, rel)?;
        self.require_seen(name, Some(rel), actor)?;
        let label = format!("write {name} {rel}");
        if self.hold(&label) {
            return Err(WriteError::Staged(label));
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| WriteError::Io(err.to_string()))?;
        }
        atomic_write(&path, bytes)
    }

    pub fn delete(&self, name: &str, _actor: Actor) -> Result<(), WriteError> {
        if self
            .pinned
            .lock()
            .expect("pinned")
            .iter()
            .any(|got| got == name)
        {
            return Err(WriteError::Pinned(name.to_string()));
        }
        let dir = find_skill_dir(&self.root, name)?;
        let raw = fs::read_to_string(dir.join("SKILL.md"))
            .map_err(|err| WriteError::Io(err.to_string()))?;
        let origin = origin_of(&raw);
        if matches!(origin, "bundled" | "hub" | "team") {
            return Err(WriteError::Origin(origin.to_string()));
        }
        let label = format!("delete {name}");
        if self.hold(&label) {
            return Err(WriteError::Staged(label));
        }
        fs::remove_dir_all(&dir).map_err(|err| WriteError::Io(err.to_string()))
    }

    fn write_skill(
        &self,
        category: &str,
        raw: &str,
        trust: Trust,
        force: bool,
        _actor: Actor,
    ) -> Result<(), WriteError> {
        self.scan_or_stop(raw, trust, force)?;
        let name = front_value(raw, "name").ok_or_else(|| WriteError::Missing("name".into()))?;
        let label = format!("create {name}");
        if self.hold(&label) {
            return Err(WriteError::Staged(label));
        }
        let dir = self.root.join(category).join(&name);
        fs::create_dir_all(&dir).map_err(|err| WriteError::Io(err.to_string()))?;
        atomic_write(&dir.join("SKILL.md"), raw.as_bytes())
    }

    fn scan_or_stop(&self, text: &str, trust: Trust, force: bool) -> Result<(), WriteError> {
        let result = scan_text(text);
        match should_allow_install(&result, trust, force) {
            Decision::Allow => Ok(()),
            Decision::Ask => {
                let label = "ask dangerous agent-created skill".to_string();
                self.staged.lock().expect("staged").push(label.clone());
                Err(WriteError::Staged(label))
            }
            Decision::Block => Err(WriteError::Scan(block_reason(&result, trust, force))),
        }
    }

    fn hold(&self, label: &str) -> bool {
        if !*self.approval.lock().expect("approval") {
            return false;
        }
        self.staged.lock().expect("staged").push(label.to_string());
        true
    }

    fn require_read(
        &self,
        name: &str,
        file: Option<&str>,
        current: &str,
        actor: Actor,
    ) -> Result<(), WriteError> {
        if actor != Actor::Reviewer {
            return Ok(());
        }
        let key = view_key(name, file);
        let seen = self.viewed.lock().expect("viewed").get(&key).copied();
        if seen != Some(fnv(current.as_bytes())) {
            let label = file.unwrap_or("SKILL.md");
            return Err(WriteError::Unread(format!(
                "Refusing background curator patch for skill '{name}': the current {label} content has not been loaded in this review turn. Call skill_view(name) for SKILL.md, or skill_view(name, file_path=...) for a supporting file, then retry the write using the content just returned."
            )));
        }
        Ok(())
    }

    fn require_seen(&self, name: &str, file: Option<&str>, actor: Actor) -> Result<(), WriteError> {
        if actor != Actor::Reviewer {
            return Ok(());
        }
        let key = view_key(name, file);
        if !self.viewed.lock().expect("viewed").contains_key(&key) {
            let label = file.unwrap_or("SKILL.md");
            return Err(WriteError::Unread(format!(
                "Refusing background curator patch for skill '{name}': the current {label} content has not been loaded in this review turn. Call skill_view(name) for SKILL.md, or skill_view(name, file_path=...) for a supporting file, then retry the write using the content just returned."
            )));
        }
        Ok(())
    }

    fn target(&self, name: &str, file: Option<&str>) -> Result<PathBuf, WriteError> {
        let dir = find_skill_dir(&self.root, name)?;
        match file {
            None => Ok(dir.join("SKILL.md")),
            Some(rel) => support_path(&dir, rel),
        }
    }
}

pub fn scan_text(text: &str) -> ScanResult {
    let mut findings = Vec::new();
    for rule in rules() {
        if rule.regex.is_match(text) {
            findings.push(Finding {
                pattern_id: rule.id.to_string(),
                severity: rule.severity.to_string(),
            });
        }
    }
    let verdict = if findings.iter().any(|item| item.severity == "critical") {
        Verdict::Dangerous
    } else if findings.iter().any(|item| item.severity == "high") {
        Verdict::Caution
    } else {
        Verdict::Safe
    };
    ScanResult { verdict, findings }
}

pub fn should_allow_install(result: &ScanResult, trust: Trust, force: bool) -> Decision {
    let decision = policy(trust, result.verdict);
    let hard_block =
        result.verdict == Verdict::Dangerous && matches!(trust, Trust::Community | Trust::Trusted);
    if decision == Decision::Allow {
        return Decision::Allow;
    }
    if force && !hard_block {
        return Decision::Allow;
    }
    decision
}

pub fn demo(root: &Path) -> Result<String, WriteError> {
    let writer = Writer::open(root)?;
    let raw = "---\nname: rename\ndescription: Rename a file\norigin: agent\n---\nRename a file by its path.\n";
    let scanned = scan_text(raw);
    writer.create("files", raw, Actor::Agent)?;
    let _ = skills::skill_view(root, "rename", None).map_err(skill_err)?;
    match scanned.verdict {
        Verdict::Safe => Ok("safe".to_string()),
        other => Err(WriteError::Scan(format!("{other:?}"))),
    }
}

fn policy(trust: Trust, verdict: Verdict) -> Decision {
    match (trust, verdict) {
        (Trust::Builtin, _) => Decision::Allow,
        (Trust::Trusted, Verdict::Dangerous) => Decision::Block,
        (Trust::Trusted, _) => Decision::Allow,
        (Trust::Community, Verdict::Safe) => Decision::Allow,
        (Trust::Community, _) => Decision::Block,
        (Trust::AgentCreated, Verdict::Dangerous) => Decision::Ask,
        (Trust::AgentCreated, _) => Decision::Allow,
    }
}

fn block_reason(result: &ScanResult, trust: Trust, force: bool) -> String {
    let n = result.findings.len();
    let hard =
        result.verdict == Verdict::Dangerous && matches!(trust, Trust::Community | Trust::Trusted);
    let tail = if hard {
        "--force does not override a dangerous verdict."
    } else if force {
        "blocked"
    } else {
        "Use --force to override."
    };
    format!(
        "Blocked ({trust:?} source + {:?} verdict, {n} findings). {tail}",
        result.verdict
    )
}

fn rules() -> &'static [Rule] {
    static RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    RULES.get_or_init(|| {
        let compiled = [
            (
                "env_exfil_curl",
                "critical",
                r"curl\s+[^\n]*\$\{?\w*(?:KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL)S?\b",
            ),
            (
                "prompt_injection_ignore",
                "critical",
                r"ignore\s+(?:\w+\s+)*(previous|all|above|prior)\s+instructions",
            ),
            (
                "disregard_rules",
                "critical",
                r"disregard\s+(?:\w+\s+)*(your|all|any)\s+(?:\w+\s+)*(instructions|rules|guidelines)",
            ),
            ("ssh_dir_access", "high", r"\$HOME/\.ssh|~/\.ssh"),
            ("role_hijack", "high", r"you\s+are\s+(?:\w+\s+)*now\s+"),
        ];
        compiled
            .into_iter()
            .map(|(id, severity, pattern)| Rule {
                id,
                severity,
                regex: Regex::new(pattern).expect(id),
            })
            .collect()
    })
}

fn origin_of(raw: &str) -> &'static str {
    match front_value(raw, "origin").as_deref() {
        Some("bundled") => "bundled",
        Some("hub") => "hub",
        Some("team") => "team",
        _ => "agent",
    }
}

fn front_value(raw: &str, key: &str) -> Option<String> {
    for line in raw.lines() {
        let Some((got, value)) = line.split_once(':') else {
            continue;
        };
        if got.trim() == key {
            let value = value.trim().to_string();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

fn find_skill_dir(root: &Path, name: &str) -> Result<PathBuf, WriteError> {
    skills::skill_view(root, name, None).map_err(skill_err)?;
    let mut hit = None;
    for category in read_dirs(root)? {
        for skill_dir in read_dirs(&category)? {
            let path = skill_dir.join("SKILL.md");
            if !path.is_file() {
                continue;
            }
            let text = fs::read_to_string(&path).map_err(|err| WriteError::Io(err.to_string()))?;
            if front_value(&text, "name").as_deref() == Some(name) {
                hit = Some(skill_dir);
            }
        }
    }
    hit.ok_or_else(|| WriteError::Missing(name.to_string()))
}

fn support_path(dir: &Path, rel: &str) -> Result<PathBuf, WriteError> {
    if rel.contains("..") || Path::new(rel).is_absolute() {
        return Err(WriteError::Path(rel.to_string()));
    }
    let path = Path::new(rel);
    let first = path
        .components()
        .next()
        .and_then(|part| part.as_os_str().to_str())
        .unwrap_or("");
    if !SUPPORT.contains(&first) {
        return Err(WriteError::Path(rel.to_string()));
    }
    Ok(dir.join(path))
}

fn read_dirs(root: &Path) -> Result<Vec<PathBuf>, WriteError> {
    let mut dirs = Vec::new();
    if !root.exists() {
        return Ok(dirs);
    }
    for entry in fs::read_dir(root).map_err(|err| WriteError::Io(err.to_string()))? {
        let entry = entry.map_err(|err| WriteError::Io(err.to_string()))?;
        if entry.path().is_dir() {
            dirs.push(entry.path());
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), WriteError> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, bytes).map_err(|err| WriteError::Io(err.to_string()))?;
    if fs::rename(&tmp, path).is_err() {
        let _ = fs::remove_file(path);
        fs::rename(&tmp, path).map_err(|err| WriteError::Io(err.to_string()))?;
    }
    Ok(())
}

fn view_key(name: &str, file: Option<&str>) -> String {
    format!("{name}:{}", file.unwrap_or("SKILL.md"))
}

fn fnv(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn skill_err(err: SkillError) -> WriteError {
    WriteError::Skill(format!("{err:?}"))
}
