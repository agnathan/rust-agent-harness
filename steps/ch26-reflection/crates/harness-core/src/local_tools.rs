//! Chapter 17. Files, a terminal, `todo`, and `clarify` on the chapter 16 harness.
//!
//! Path checks follow Hermes `tools/file_tools_paths.py`: a path that resolves
//! outside the workspace root is refused. `write_file` of a file this task
//! has not read, or whose mtime moved, is `stale_write_blocked`
//! (`tools/file_state.py`, `tools/file_tools_write_guards.py`). `patch`
//! replaces one `old_string`. The terminal calls `approval::deny_tool` before
//! `sandbox::run_until`, so a hardline command never reaches the worker.
//! `todo` is agent-owned memory, not a file in the workspace. `clarify` pauses
//! for a user reply. A child (`depth >= 1`) may not call it.

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use async_trait::async_trait;
use serde_json::{json, Value};

use crate::approval::{self, Session};
use crate::sandbox::{self, SandboxError, Worker};
use crate::tool::{check_arguments, Tool, ToolError, ToolKind, ToolResult, ToolSchema};

pub const MAX_READ_CHARS: usize = 100_000;
pub const FOREGROUND_MAX_TIMEOUT: Duration = Duration::from_secs(600);
pub const DEFAULT_MAX_BYTES: usize = 50_000;
pub const DEFAULT_MAX_LINES: usize = 2_000;
pub const DEFAULT_MAX_LINE_LENGTH: usize = 2_000;
pub const MAX_TODO_ITEMS: usize = 256;
pub const MAX_TODO_CONTENT_CHARS: usize = 4_000;
pub const MAX_QUESTIONS: usize = 5;
pub const MAX_CHOICES: usize = 4;
pub const MAX_CHOICE_CHARS: usize = 8_000;

/// Hermes `DELEGATE_BLOCKED_TOOLS`. Chapter 29's supervisor keeps the same list.
/// This chapter enforces `clarify` on the tool itself.
pub const BLOCKED_CHILD_TOOLS: &[&str] = &[
    "delegate_task",
    "clarify",
    "memory",
    "send_message",
    "cronjob_manage",
];

pub fn child_may_call(tool: &str, depth: u8) -> Result<(), String> {
    if depth >= 1 && BLOCKED_CHILD_TOOLS.contains(&tool) {
        return Err(format!("BLOCKED: a child agent may not call {tool}"));
    }
    Ok(())
}

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Absolute paths are allowed when they land inside the root. A relative
    /// path is joined to the root and must still be inside it after `..` is resolved.
    pub fn resolve(&self, raw: &str) -> Result<PathBuf, String> {
        let root = fs::canonicalize(&self.root).map_err(|err| err.to_string())?;
        let candidate = {
            let path = Path::new(raw);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                root.join(path)
            }
        };
        let resolved = if candidate.exists() {
            fs::canonicalize(&candidate).map_err(|err| err.to_string())?
        } else {
            let parent = candidate
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .ok_or_else(|| format!("path escapes the workspace root: {raw}"))?;
            let name = candidate
                .file_name()
                .ok_or_else(|| format!("path escapes the workspace root: {raw}"))?;
            fs::canonicalize(parent)
                .map_err(|err| err.to_string())?
                .join(name)
        };
        if !resolved.starts_with(&root) {
            return Err(format!("path escapes the workspace root: {raw}"));
        }
        Ok(resolved)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Stamp {
    mtime: SystemTime,
    len: u64,
}

fn stamp_of(path: &Path) -> Result<Stamp, String> {
    let meta = fs::metadata(path).map_err(|err| err.to_string())?;
    Ok(Stamp {
        mtime: meta.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        len: meta.len(),
    })
}

/// Reads this task has already done. A write compares the stamp.
#[derive(Default)]
pub struct FileState {
    reads: HashMap<PathBuf, Stamp>,
}

impl FileState {
    pub fn new() -> Self {
        Self::default()
    }

    fn note(&mut self, path: &Path) -> Result<(), String> {
        let stamp = stamp_of(path)?;
        self.reads.insert(path.to_path_buf(), stamp);
        Ok(())
    }

    pub fn check_stale(&self, path: &Path) -> Result<(), String> {
        if !path.exists() {
            return Ok(());
        }
        let Some(seen) = self.reads.get(path) else {
            return Err("stale_write_blocked: file was not read in this task".into());
        };
        let current = stamp_of(path)?;
        if current != *seen {
            return Err("stale_write_blocked: file changed since it was read".into());
        }
        Ok(())
    }
}

pub fn read_file(
    workspace: &Workspace,
    state: &mut FileState,
    raw: &str,
) -> Result<String, String> {
    let path = workspace.resolve(raw)?;
    let text = fs::read_to_string(&path).map_err(|err| err.to_string())?;
    state.note(&path)?;
    Ok(cap_chars(&text, MAX_READ_CHARS, "read"))
}

pub fn write_file(
    workspace: &Workspace,
    state: &mut FileState,
    raw: &str,
    contents: &str,
) -> Result<(), String> {
    let path = workspace.resolve(raw)?;
    state.check_stale(&path)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    fs::write(&path, contents).map_err(|err| err.to_string())?;
    state.note(&path)?;
    Ok(())
}

/// Replace the single occurrence of `old` with `new`. More than one match is
/// ambiguous, so the file is left untouched.
pub fn patch_file(
    workspace: &Workspace,
    state: &mut FileState,
    raw: &str,
    old: &str,
    new: &str,
) -> Result<(), String> {
    if old.is_empty() {
        return Err("patch old_string is empty".into());
    }
    let path = workspace.resolve(raw)?;
    state.check_stale(&path)?;
    let text = fs::read_to_string(&path).map_err(|err| err.to_string())?;
    let count = text.matches(old).count();
    if count != 1 {
        return Err(format!("patch expected one match, found {count}"));
    }
    let patched = text.replacen(old, new, 1);
    fs::write(&path, patched).map_err(|err| err.to_string())?;
    state.note(&path)?;
    Ok(())
}

fn cap_chars(text: &str, max_chars: usize, kind: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_chars {
        return text.to_string();
    }
    let kept: String = chars[..max_chars].iter().collect();
    format!("{kept}\n\n[TRUNCATED {kind} at {max_chars} chars]")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalOutput {
    pub denied: bool,
    pub text: String,
}

/// Approval first. The worker runs only when `deny_tool` returns `None`.
/// The timeout cannot exceed [`FOREGROUND_MAX_TIMEOUT`]. Output is cut to
/// [`DEFAULT_MAX_BYTES`] and [`DEFAULT_MAX_LINES`].
pub fn run_terminal(
    session: &mut Session,
    command: &str,
    timeout: Duration,
    worker: &Path,
) -> TerminalOutput {
    let arguments = json!({"command": command});
    if let Some(message) = approval::deny_tool(session, "terminal", &arguments) {
        return TerminalOutput {
            denied: true,
            text: message,
        };
    }
    let timeout = if timeout > FOREGROUND_MAX_TIMEOUT {
        FOREGROUND_MAX_TIMEOUT
    } else {
        timeout
    };
    // `sleep-ms` is the worker's slow job. `run_until` kills the child when
    // the timeout fires. Every other command is one line in and one line out
    // on the same long-lived worker `Worker::accept` uses.
    let text = if command.starts_with("sleep-ms ") {
        match sandbox::run_until(worker, command, timeout) {
            Ok(line) => line,
            Err(SandboxError::Timeout) => "timeout".into(),
            Err(SandboxError::Io(err)) => err,
        }
    } else {
        match Worker::spawn(worker).and_then(|mut child| {
            let line = child.accept(command)?;
            let _ = child.quit();
            Ok(line)
        }) {
            Ok(line) => line,
            Err(err) => err.to_string(),
        }
    };
    TerminalOutput {
        denied: false,
        text: truncate_terminal(&text),
    }
}

pub fn truncate_terminal(text: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in text.lines() {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() > DEFAULT_MAX_LINE_LENGTH {
            let kept: String = chars[..DEFAULT_MAX_LINE_LENGTH].iter().collect();
            lines.push(format!("{kept}…"));
        } else {
            lines.push(line.to_string());
        }
    }
    if lines.len() > DEFAULT_MAX_LINES {
        let head = DEFAULT_MAX_LINES / 2;
        let tail = DEFAULT_MAX_LINES - head;
        let omitted = lines.len() - head - tail;
        let mut kept = lines[..head].to_vec();
        kept.push(format!("… [{omitted} lines omitted] …"));
        kept.extend(lines[lines.len() - tail..].iter().cloned());
        lines = kept;
    }
    let mut joined = lines.join("\n");
    let bytes = joined.len();
    if bytes > DEFAULT_MAX_BYTES {
        let head = floor_char_boundary(&joined, DEFAULT_MAX_BYTES / 2);
        let tail_start = ceil_char_boundary(&joined, bytes - (DEFAULT_MAX_BYTES - head));
        let omitted = tail_start.saturating_sub(head);
        joined = format!(
            "{}\n… [{omitted} bytes omitted] …\n{}",
            &joined[..head],
            &joined[tail_start..]
        );
    }
    joined
}

fn floor_char_boundary(text: &str, mut index: usize) -> usize {
    if index > text.len() {
        return text.len();
    }
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceil_char_boundary(text: &str, mut index: usize) -> usize {
    if index > text.len() {
        return text.len();
    }
    while index < text.len() && !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: String,
}

#[derive(Debug, Default)]
pub struct TodoStore {
    items: Vec<TodoItem>,
    revision: u64,
}

impl TodoStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn items(&self) -> &[TodoItem] {
        &self.items
    }

    /// `merge` updates by id and appends unknown ids. A mismatched
    /// `expected_revision` is refused and does not bump the counter.
    pub fn apply(
        &mut self,
        incoming: Vec<TodoItem>,
        merge: bool,
        expected_revision: Option<u64>,
    ) -> Result<u64, String> {
        if let Some(expected) = expected_revision {
            if expected != self.revision {
                return Err(format!(
                    "stale todo revision: expected {expected}, have {}",
                    self.revision
                ));
            }
        }
        let incoming: Vec<TodoItem> = incoming.into_iter().map(clip_item).collect();
        if merge {
            for item in incoming {
                if let Some(existing) = self.items.iter_mut().find(|row| row.id == item.id) {
                    *existing = item;
                } else {
                    self.items.push(item);
                }
            }
        } else {
            self.items = incoming;
        }
        if self.items.len() > MAX_TODO_ITEMS {
            self.items.truncate(MAX_TODO_ITEMS);
        }
        self.revision = self.revision.saturating_add(1);
        Ok(self.revision)
    }
}

fn clip_item(mut item: TodoItem) -> TodoItem {
    if item.content.chars().count() > MAX_TODO_CONTENT_CHARS {
        item.content = item.content.chars().take(MAX_TODO_CONTENT_CHARS).collect();
    }
    item
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub text: String,
    pub choices: Vec<String>,
}

pub fn parse_questions(value: &Value) -> Result<Vec<Question>, String> {
    let list = value
        .get("questions")
        .and_then(Value::as_array)
        .ok_or_else(|| "questions must be an array".to_string())?;
    if list.is_empty() {
        return Err("questions supports at least 1 item".into());
    }
    if list.len() > MAX_QUESTIONS {
        return Err(format!("questions supports at most {MAX_QUESTIONS} items"));
    }
    let mut questions = Vec::with_capacity(list.len());
    for entry in list {
        let text = entry
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let mut choices = Vec::new();
        if let Some(raw) = entry.get("choices").and_then(Value::as_array) {
            for choice in raw.iter().filter_map(Value::as_str).take(MAX_CHOICES) {
                let clipped: String = choice.chars().take(MAX_CHOICE_CHARS).collect();
                if !clipped.is_empty() {
                    choices.push(clipped);
                }
            }
        }
        questions.push(Question { text, choices });
    }
    Ok(questions)
}

/// Pauses the tool call until `answers` yields one reply. `depth >= 1` returns
/// before the queue is touched.
pub struct ClarifyTool {
    depth: u8,
    answers: Mutex<VecDeque<String>>,
    asked: AtomicUsize,
}

impl ClarifyTool {
    pub fn new(depth: u8, answers: Vec<String>) -> Self {
        Self {
            depth,
            answers: Mutex::new(answers.into()),
            asked: AtomicUsize::new(0),
        }
    }

    pub fn asked(&self) -> usize {
        self.asked.load(Ordering::SeqCst)
    }

    pub fn pending(&self) -> usize {
        self.answers.lock().expect("answers").len()
    }

    pub fn schema_value() -> ToolSchema {
        ToolSchema {
            name: "clarify".into(),
            description: "Pause and ask the user. A child agent cannot call this.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "questions": { "type": "array" }
                },
                "required": ["questions"],
                "additionalProperties": false
            }),
        }
    }
}

#[async_trait]
impl Tool for ClarifyTool {
    fn schema(&self) -> ToolSchema {
        Self::schema_value()
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
        let schema = self.schema();
        check_arguments(&schema, &arguments)?;
        if let Err(message) = child_may_call("clarify", self.depth) {
            return Err(ToolError::Failed { message });
        }
        let questions =
            parse_questions(&arguments).map_err(|message| ToolError::Failed { message })?;
        self.asked.fetch_add(1, Ordering::SeqCst);
        let answer = self
            .answers
            .lock()
            .expect("answers")
            .pop_front()
            .unwrap_or_else(|| "(no user reply)".into());
        let rendered = questions
            .iter()
            .map(|question| question.text.clone())
            .collect::<Vec<_>>()
            .join(" | ");
        Ok(ToolResult::text(format!("{rendered} => {answer}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_relative_escape_is_refused_and_an_unread_write_is_stale() {
        let root = std::env::temp_dir().join(format!("harness-local-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("inside")).unwrap();
        fs::write(root.join("inside/note.txt"), "one").unwrap();
        let workspace = Workspace::new(&root);
        assert!(workspace.resolve("../outside.txt").is_err());
        assert!(workspace.resolve("inside/note.txt").is_ok());
        let mut state = FileState::new();
        let err = write_file(&workspace, &mut state, "inside/note.txt", "two").unwrap_err();
        assert!(err.contains("stale_write_blocked"));
        let text = read_file(&workspace, &mut state, "inside/note.txt").unwrap();
        assert_eq!(text, "one");
        write_file(&workspace, &mut state, "inside/note.txt", "two").unwrap();
        patch_file(&workspace, &mut state, "inside/note.txt", "two", "three").unwrap();
        assert_eq!(
            fs::read_to_string(root.join("inside/note.txt")).unwrap(),
            "three"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn todo_merge_updates_one_item_and_a_stale_revision_does_not() {
        let mut store = TodoStore::new();
        store
            .apply(
                vec![TodoItem {
                    id: "a".into(),
                    content: "first".into(),
                    status: "pending".into(),
                }],
                false,
                Some(0),
            )
            .unwrap();
        let err = store
            .apply(
                vec![TodoItem {
                    id: "a".into(),
                    content: "nope".into(),
                    status: "pending".into(),
                }],
                true,
                Some(0),
            )
            .unwrap_err();
        assert!(err.contains("stale todo revision"));
        assert_eq!(store.items()[0].content, "first");
        store
            .apply(
                vec![TodoItem {
                    id: "a".into(),
                    content: "second".into(),
                    status: "done".into(),
                }],
                true,
                Some(store.revision()),
            )
            .unwrap();
        assert_eq!(store.items().len(), 1);
        assert_eq!(store.items()[0].content, "second");
        assert_eq!(store.revision(), 2);
    }
}
