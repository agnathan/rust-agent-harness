//! Book chapter 18. A session is an append-only SQLite file in WAL mode.
//! Indexing is queued. This chapter does not build the FTS table.

use std::collections::VecDeque;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

const PENDING_CAP: usize = 64;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL,
    parent_id TEXT,
    started_at INTEGER NOT NULL,
    title TEXT
);
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    ts INTEGER NOT NULL
);
";

pub struct Store {
    conn: Connection,
    pending_fts: VecDeque<i64>,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        // Parallel tests can open neighboring files while macOS still reports
        // the new file busy. Wait briefly instead of failing the migrate.
        conn.busy_timeout(Duration::from_secs(2))?;
        Ok(Self {
            conn,
            pending_fts: VecDeque::new(),
        })
    }

    pub fn migrate(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        self.conn.execute_batch(SCHEMA)?;
        Ok(())
    }

    pub fn journal_mode(&self) -> Result<String, rusqlite::Error> {
        self.conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }

    pub fn pending_fts(&self) -> usize {
        self.pending_fts.len()
    }

    /// One transaction: the user row and the assistant row. FTS ids are queued.
    pub fn append_turn(
        &mut self,
        session_id: &str,
        user: &str,
        assistant: &str,
    ) -> Result<usize, rusqlite::Error> {
        let now = unix_secs();
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT OR IGNORE INTO sessions (id, source, parent_id, started_at, title)
             VALUES (?1, 'user', NULL, ?2, NULL)",
            params![session_id, now],
        )?;
        tx.execute(
            "INSERT INTO messages (session_id, role, content, ts) VALUES (?1, 'user', ?2, ?3)",
            params![session_id, user, now],
        )?;
        let user_id = tx.last_insert_rowid();
        tx.execute(
            "INSERT INTO messages (session_id, role, content, ts) VALUES (?1, 'assistant', ?2, ?3)",
            params![session_id, assistant, now],
        )?;
        let assistant_id = tx.last_insert_rowid();
        tx.commit()?;
        self.enqueue(user_id);
        self.enqueue(assistant_id);
        Ok(2)
    }

    pub fn messages(&self, session_id: &str) -> Result<Vec<(String, String)>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT role, content FROM messages WHERE session_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![session_id], |row| Ok((row.get(0)?, row.get(1)?)))?;
        rows.collect()
    }

    fn enqueue(&mut self, id: i64) {
        if self.pending_fts.len() == PENDING_CAP {
            self.pending_fts.pop_front();
        }
        self.pending_fts.push_back(id);
    }
}

fn unix_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

pub fn demo_reply() -> &'static str {
    "pong"
}
