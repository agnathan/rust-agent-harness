//! Book chapter 19. MEMORY.md and USER.md. The snapshot frozen at open
//! does not change when a write lands. The scan blocks a poisoned entry.

use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use hooks::{HookError, MemoryBackend, TurnRecord};
use regex::Regex;

pub const ENTRY_DELIMITER: &str = "\n§\n";
pub const MEMORY_CHAR_LIMIT: usize = 2_200;
pub const USER_CHAR_LIMIT: usize = 1_375;
pub const MAX_SCAN_CHARS: usize = 65_536;
const QUEUE_CAP: usize = 64;

const INVISIBLE: &[char] = &[
    '\u{200b}', '\u{200c}', '\u{200d}', '\u{2060}', '\u{2062}', '\u{2063}', '\u{2064}', '\u{feff}',
    '\u{202a}', '\u{202b}', '\u{202c}', '\u{202d}', '\u{202e}', '\u{2066}', '\u{2067}', '\u{2068}',
    '\u{2069}',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Memory,
    User,
}

impl Target {
    fn file(self) -> &'static str {
        match self {
            Target::Memory => "MEMORY.md",
            Target::User => "USER.md",
        }
    }

    fn limit(self) -> usize {
        match self {
            Target::Memory => MEMORY_CHAR_LIMIT,
            Target::User => USER_CHAR_LIMIT,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frozen {
    pub memory: String,
    pub user: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    Empty,
    Duplicate,
    Drift,
    Budget { over: usize },
    Scan(String),
    Io(String),
}

pub struct MemoryFiles {
    root: PathBuf,
    frozen: Arc<Frozen>,
    lock: Mutex<()>,
    queue: Mutex<VecDeque<String>>,
}

impl MemoryFiles {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, MemoryError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|err| MemoryError::Io(err.to_string()))?;
        let frozen = Arc::new(Frozen {
            memory: read_raw(&root.join("MEMORY.md"))?,
            user: read_raw(&root.join("USER.md"))?,
        });
        Ok(Self {
            root,
            frozen,
            lock: Mutex::new(()),
            queue: Mutex::new(VecDeque::new()),
        })
    }

    pub fn snapshot(&self) -> Arc<Frozen> {
        Arc::clone(&self.frozen)
    }

    pub fn queue_len(&self) -> usize {
        self.queue.lock().expect("queue").len()
    }

    pub fn add(&self, target: Target, content: &str) -> Result<&'static str, MemoryError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(MemoryError::Empty);
        }
        if let Some(message) = first_threat_message(content) {
            return Err(MemoryError::Scan(message));
        }
        let _guard = self.lock.lock().expect("memory");
        let mut entries = load_entries(&self.root.join(target.file()))?;
        if entries.iter().any(|entry| entry == content) {
            return Err(MemoryError::Duplicate);
        }
        entries.push(content.to_string());
        check_budget(target, &entries)?;
        write_entries(&self.root.join(target.file()), &entries)?;
        Ok("Entry added.")
    }

    pub fn replace(
        &self,
        target: Target,
        old_text: &str,
        new_content: &str,
    ) -> Result<&'static str, MemoryError> {
        let new_content = new_content.trim();
        if old_text.trim().is_empty() || new_content.is_empty() {
            return Err(MemoryError::Empty);
        }
        if let Some(message) = first_threat_message(new_content) {
            return Err(MemoryError::Scan(message));
        }
        let _guard = self.lock.lock().expect("memory");
        let mut entries = load_entries(&self.root.join(target.file()))?;
        let hits: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.as_str() == old_text.trim())
            .map(|(index, _)| index)
            .collect();
        if hits.len() != 1 {
            return Err(MemoryError::Empty);
        }
        entries[hits[0]] = new_content.to_string();
        check_budget(target, &entries)?;
        write_entries(&self.root.join(target.file()), &entries)?;
        Ok("Entry replaced.")
    }

    pub fn remove(&self, target: Target, old_text: &str) -> Result<&'static str, MemoryError> {
        let _guard = self.lock.lock().expect("memory");
        let mut entries = load_entries(&self.root.join(target.file()))?;
        let before = entries.len();
        entries.retain(|entry| entry != old_text.trim());
        if entries.len() == before {
            return Err(MemoryError::Empty);
        }
        write_entries(&self.root.join(target.file()), &entries)?;
        Ok("Entry removed.")
    }

    pub fn read(&self, target: Target) -> Result<String, MemoryError> {
        read_raw(&self.root.join(target.file()))
    }
}

impl MemoryBackend for MemoryFiles {
    fn name(&self) -> &str {
        "files"
    }

    fn enqueue_turn(&self, record: &TurnRecord<'_>) -> Result<(), HookError> {
        let mut queue = self.queue.lock().expect("queue");
        if queue.len() == QUEUE_CAP {
            queue.pop_front();
        }
        queue.push_back(format!("{} {}", record.user, record.assistant));
        Ok(())
    }
}

pub fn first_threat_message(content: &str) -> Option<String> {
    let mut end = MAX_SCAN_CHARS.min(content.len());
    while !content.is_char_boundary(end) {
        end -= 1;
    }
    let content = &content[..end];
    if let Some(ch) = content.chars().find(|ch| INVISIBLE.contains(ch)) {
        return Some(format!(
            "Blocked: content contains invisible unicode character U+{:04X} (possible injection).",
            ch as u32
        ));
    }
    if content.contains("authorized_keys") {
        return Some(block("ssh_backdoor"));
    }
    if context_exfil(content) {
        return Some(block("context_exfil"));
    }
    if hardcoded_secret(content) {
        return Some(block("hardcoded_secret"));
    }
    None
}

fn block(pid: &str) -> String {
    format!(
        "Blocked: content matches threat pattern '{pid}'. Content is injected into the system prompt and must not contain injection or exfiltration payloads."
    )
}

fn context_exfil(content: &str) -> bool {
    Regex::new(
        r"(?i)(include|output|print|share)\s+(?:\w+\s+){0,8}(conversation|chat\s+history|previous\s+messages|full\s+context|entire\s+context)",
    )
    .expect("context")
    .is_match(content)
}

fn hardcoded_secret(content: &str) -> bool {
    Regex::new(r#"(?i)(?:api[_-]?key|token|secret|password)\s*[=:]\s*["'][A-Za-z0-9+/=_-]{20,}"#)
        .expect("secret")
        .is_match(content)
}

fn check_budget(target: Target, entries: &[String]) -> Result<(), MemoryError> {
    let total = entries.join(ENTRY_DELIMITER).len();
    let limit = target.limit();
    if total > limit {
        Err(MemoryError::Budget {
            over: total - limit,
        })
    } else {
        Ok(())
    }
}

fn load_entries(path: &Path) -> Result<Vec<String>, MemoryError> {
    parse_entries(&read_raw(path)?)
}

fn read_raw(path: &Path) -> Result<String, MemoryError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(MemoryError::Io(err.to_string())),
    }
}

fn parse_entries(raw: &str) -> Result<Vec<String>, MemoryError> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    if raw.contains('§') && !raw.contains(ENTRY_DELIMITER) {
        return Err(MemoryError::Drift);
    }
    Ok(raw
        .split(ENTRY_DELIMITER)
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect())
}

fn write_entries(path: &Path, entries: &[String]) -> Result<(), MemoryError> {
    let body = entries.join(ENTRY_DELIMITER);
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, &body).map_err(|err| MemoryError::Io(err.to_string()))?;
    if fs::rename(&tmp, path).is_err() {
        let _ = fs::remove_file(path);
        fs::rename(&tmp, path).map_err(|err| MemoryError::Io(err.to_string()))?;
    }
    Ok(())
}

pub fn budget_line(over: usize) -> String {
    format!("would exceed the limit by {over} chars")
}
