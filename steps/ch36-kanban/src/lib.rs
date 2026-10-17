//! Book chapter 42. One JSON board. A lock file is the transaction.
//! The status update is a compare-and-swap: a second owner loses.

use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub parents: Vec<String>,
}

impl Task {
    pub fn ready(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: "ready".into(),
            owner: None,
            result: None,
            parents: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board {
    #[serde(default)]
    pub paused: bool,
    pub tasks: Vec<Task>,
}

impl Board {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            paused: false,
            tasks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Claim { owner: String },
    Complete { owner: String, result: String },
    Review { owner: String, summary: String },
    Block { owner: String, reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError {
    Lost,
    Empty,
    Parents,
    Paused,
    Busy,
    Missing,
    Io,
}

struct Unlock {
    path: PathBuf,
}

impl Drop for Unlock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn board_path(dir: &Path) -> PathBuf {
    dir.join("board.json")
}

fn lock_path(dir: &Path) -> PathBuf {
    dir.join("board.lock")
}

pub fn write_board(dir: &Path, board: &Board) -> Result<(), TxError> {
    fs::create_dir_all(dir).map_err(|_| TxError::Io)?;
    let bytes = serde_json::to_vec(board).map_err(|_| TxError::Io)?;
    let tmp = dir.join("board.json.tmp");
    fs::write(&tmp, bytes).map_err(|_| TxError::Io)?;
    let dest = board_path(dir);
    if fs::rename(&tmp, &dest).is_err() {
        let _ = fs::remove_file(&dest);
        fs::rename(&tmp, &dest).map_err(|_| TxError::Io)?;
    }
    Ok(())
}

fn read_board(dir: &Path) -> Result<Board, TxError> {
    let bytes = fs::read(board_path(dir)).map_err(|_| TxError::Io)?;
    serde_json::from_slice(&bytes).map_err(|_| TxError::Io)
}

fn acquire(path: &Path) -> Result<Unlock, TxError> {
    for _ in 0..2_000 {
        match OpenOptions::new().write(true).create_new(true).open(path) {
            Ok(file) => {
                drop(file);
                return Ok(Unlock {
                    path: path.to_path_buf(),
                });
            }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                thread::sleep(Duration::from_micros(100));
            }
            Err(_) => return Err(TxError::Io),
        }
    }
    Err(TxError::Busy)
}

fn parents_done(board: &Board, task: &Task) -> bool {
    task.parents.iter().all(|parent| {
        board
            .tasks
            .iter()
            .find(|candidate| candidate.id == *parent)
            .is_some_and(|candidate| candidate.status == "done")
    })
}

fn apply(board: &mut Board, id: &str, op: &Op) -> Result<(), TxError> {
    let Some(index) = board.tasks.iter().position(|task| task.id == id) else {
        return Err(TxError::Missing);
    };
    if board.paused && matches!(op, Op::Claim { .. }) {
        return Err(TxError::Paused);
    }
    let parents_ok = parents_done(board, &board.tasks[index]);
    let task = &mut board.tasks[index];
    match op {
        Op::Claim { owner } => {
            if !parents_ok {
                return Err(TxError::Parents);
            }
            if task.status != "ready" || task.owner.is_some() {
                return Err(TxError::Lost);
            }
            task.status = "running".into();
            task.owner = Some(owner.clone());
        }
        Op::Complete { owner, result } => {
            if result.trim().is_empty() {
                return Err(TxError::Empty);
            }
            if task.status != "running" || task.owner.as_deref() != Some(owner) {
                return Err(TxError::Lost);
            }
            task.status = "done".into();
            task.result = Some(result.clone());
            task.owner = None;
        }
        Op::Review { owner, summary } => {
            if summary.trim().is_empty() {
                return Err(TxError::Empty);
            }
            if task.status != "running" || task.owner.as_deref() != Some(owner) {
                return Err(TxError::Lost);
            }
            task.status = "review".into();
            task.result = Some(summary.clone());
            task.owner = None;
        }
        Op::Block { owner, reason } => {
            if reason.trim().is_empty() {
                return Err(TxError::Empty);
            }
            if task.status != "running" || task.owner.as_deref() != Some(owner) {
                return Err(TxError::Lost);
            }
            task.status = "blocked".into();
            task.result = Some(reason.clone());
            task.owner = None;
        }
    }
    Ok(())
}

/// One transaction. The lock file is gone before this returns.
pub fn transact(dir: &Path, id: &str, op: Op) -> Result<String, TxError> {
    let _lock = acquire(&lock_path(dir))?;
    let mut board = read_board(dir)?;
    apply(&mut board, id, &op)?;
    let status = board
        .tasks
        .iter()
        .find(|task| task.id == id)
        .map(|task| task.status.clone())
        .ok_or(TxError::Missing)?;
    write_board(dir, &board)?;
    Ok(status)
}

pub fn task_status(dir: &Path, id: &str) -> Result<Option<String>, TxError> {
    let board = read_board(dir)?;
    Ok(board
        .tasks
        .iter()
        .find(|task| task.id == id)
        .map(|task| task.status.clone()))
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
    let dir = std::env::temp_dir().join(format!("ch36-kanban-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).expect("board");
    transact(
        &dir,
        "t1",
        Op::Claim {
            owner: "worker".into(),
        },
    )
    .expect("claim");
    assert!(!lock_path(&dir).exists());
    let _ = FakeModel { reply: "ok" }.complete("t1");
    let status = transact(
        &dir,
        "t1",
        Op::Complete {
            owner: "worker".into(),
            result: "shipped".into(),
        },
    )
    .expect("complete");
    let _ = fs::remove_dir_all(&dir);
    format!("kanban: {status}")
}
