//! Lesson 16: persistence with SQLite (rusqlite), plus tracing hooks.
//! Hermes: hermes_state.py SessionDB (SQLite WAL + FTS5, append-only messages).
use crate::l02_message::Message;
use crate::l05_json::role_from_str;
use rusqlite::{params, Connection};

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open_in_memory() -> rusqlite::Result<Self> {
        let s = Store {
            conn: Connection::open_in_memory()?,
        };
        s.migrate()?;
        Ok(s)
    }
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        let s = Store { conn };
        s.migrate()?;
        Ok(s)
    }
    /// Create tables if missing:
    /// sessions(id TEXT PRIMARY KEY, title TEXT NOT NULL)
    /// messages(id INTEGER PRIMARY KEY AUTOINCREMENT, session_id TEXT NOT NULL REFERENCES sessions(id),
    ///          role TEXT NOT NULL, content TEXT NOT NULL)
    pub fn migrate(&self) -> rusqlite::Result<()> {
        // Hint: conn.execute_batch("CREATE TABLE IF NOT EXISTS ...; ...")
        todo!("lesson 16")
    }
    pub fn create_session(&self, id: &str, title: &str) -> rusqlite::Result<()> {
        // Hint: INSERT with params![..]
        todo!("lesson 16")
    }
    /// Append-only insert; return the new row id.
    #[tracing::instrument(skip(self, m), fields(role = m.role.as_str()))]
    pub fn append(&self, session_id: &str, m: &Message) -> rusqlite::Result<i64> {
        // Hint: execute then conn.last_insert_rowid()
        todo!("lesson 16")
    }
    /// All messages of a session in insertion order (rows with unknown roles are skipped).
    pub fn load(&self, session_id: &str) -> rusqlite::Result<Vec<Message>> {
        // Hint: prepare + query_map; build Message via Message::user/.. or a struct literal
        todo!("lesson 16")
    }
    /// (session_id, content) for messages whose content contains `needle` (LIKE), ordered by id.
    pub fn search(&self, needle: &str) -> rusqlite::Result<Vec<(String, String)>> {
        // Hint: WHERE content LIKE '%' || ?1 || '%'
        todo!("lesson 16")
    }
}
