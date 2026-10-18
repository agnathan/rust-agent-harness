//! Book chapter 20. session_search over an FTS5 trigram index.
//! Subagent sessions stay out. Cron and worker sessions sort after the user's.

use rusqlite::{params, Connection};
use sessions::Store;

const HIDDEN: &str = "('subagent', 'kanban', 'tool')";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    pub session_id: String,
    pub message_id: i64,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionQuery {
    pub query: Option<String>,
    pub session_id: Option<String>,
    pub around_message_id: Option<i64>,
}

pub fn install_fts(store: &mut Store) -> Result<(), rusqlite::Error> {
    let conn = store.connection_mut();
    conn.execute_batch(
        "CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts_trigram USING fts5(
            content,
            content='messages',
            content_rowid='id',
            tokenize='trigram'
        );",
    )?;
    conn.execute(
        "INSERT INTO messages_fts_trigram(messages_fts_trigram) VALUES('rebuild')",
        [],
    )?;
    Ok(())
}

pub fn session_search(store: &Store, query: &SessionQuery) -> Result<Vec<Hit>, rusqlite::Error> {
    match (
        query.session_id.as_deref(),
        query.around_message_id,
        query.query.as_deref().filter(|text| !text.is_empty()),
    ) {
        (Some(session_id), Some(around), _) => scroll(store.connection(), session_id, around),
        (Some(session_id), None, _) => read(store.connection(), session_id),
        (_, _, Some(text)) => discover(store.connection(), text),
        _ => browse(store.connection()),
    }
}

pub fn insert_message(
    store: &mut Store,
    session_id: &str,
    source: &str,
    content: &str,
) -> Result<i64, rusqlite::Error> {
    let conn = store.connection_mut();
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT OR IGNORE INTO sessions (id, source, parent_id, started_at, title)
         VALUES (?1, ?2, NULL, 1, NULL)",
        params![session_id, source],
    )?;
    tx.execute(
        "INSERT INTO messages (session_id, role, content, ts) VALUES (?1, 'user', ?2, 1)",
        params![session_id, content],
    )?;
    let id = tx.last_insert_rowid();
    tx.commit()?;
    Ok(id)
}

fn discover(conn: &Connection, query: &str) -> Result<Vec<Hit>, rusqlite::Error> {
    let sql = format!(
        "SELECT s.id, m.id, m.content
         FROM messages_fts_trigram
         JOIN messages m ON m.id = messages_fts_trigram.rowid
         JOIN sessions s ON s.id = m.session_id
         WHERE messages_fts_trigram MATCH ?1
           AND s.source NOT IN {HIDDEN}
         ORDER BY CASE WHEN s.source IN ('cron', 'worker') THEN 1 ELSE 0 END, rank"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![query], |row| {
        Ok(Hit {
            session_id: row.get(0)?,
            message_id: row.get(1)?,
            content: row.get(2)?,
        })
    })?;
    let mut hits = Vec::new();
    let mut seen = Vec::new();
    for row in rows {
        let hit = row?;
        if seen.iter().any(|id: &String| id == &hit.session_id) {
            continue;
        }
        seen.push(hit.session_id.clone());
        hits.push(hit);
        if hits.len() == 3 {
            break;
        }
    }
    Ok(hits)
}

fn scroll(conn: &Connection, session_id: &str, around: i64) -> Result<Vec<Hit>, rusqlite::Error> {
    if hidden(conn, session_id)? {
        return Ok(Vec::new());
    }
    let mut stmt = conn.prepare(
        "SELECT session_id, id, content FROM messages
         WHERE session_id = ?1 AND id BETWEEN ?2 AND ?3
         ORDER BY id",
    )?;
    let rows = stmt.query_map(params![session_id, around - 5, around + 5], |row| {
        Ok(Hit {
            session_id: row.get(0)?,
            message_id: row.get(1)?,
            content: row.get(2)?,
        })
    })?;
    rows.collect()
}

fn read(conn: &Connection, session_id: &str) -> Result<Vec<Hit>, rusqlite::Error> {
    if hidden(conn, session_id)? {
        return Ok(Vec::new());
    }
    let mut stmt = conn.prepare(
        "SELECT session_id, id, content FROM messages WHERE session_id = ?1 ORDER BY id",
    )?;
    let rows = stmt.query_map(params![session_id], |row| {
        Ok(Hit {
            session_id: row.get(0)?,
            message_id: row.get(1)?,
            content: row.get(2)?,
        })
    })?;
    rows.collect()
}

fn browse(conn: &Connection) -> Result<Vec<Hit>, rusqlite::Error> {
    let sql = format!(
        "SELECT id, source FROM sessions
         WHERE source NOT IN {HIDDEN}
         ORDER BY CASE WHEN source IN ('cron', 'worker') THEN 1 ELSE 0 END, started_at DESC
         LIMIT 10"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| {
        Ok(Hit {
            session_id: row.get(0)?,
            message_id: 0,
            content: row.get(1)?,
        })
    })?;
    rows.collect()
}

fn hidden(conn: &Connection, session_id: &str) -> Result<bool, rusqlite::Error> {
    let source: Option<String> = conn
        .query_row(
            "SELECT source FROM sessions WHERE id = ?1",
            params![session_id],
            |row| row.get(0),
        )
        .ok();
    Ok(matches!(
        source.as_deref(),
        Some("subagent") | Some("kanban") | Some("tool")
    ))
}

pub fn demo_db(path: &std::path::Path) -> Result<String, rusqlite::Error> {
    let mut store = Store::open(path)?;
    store.migrate()?;
    insert_message(&mut store, "child", "subagent", "zephyrtoken in a child")?;
    insert_message(&mut store, "desk", "user", "zephyrtoken on the desk")?;
    install_fts(&mut store)?;
    let hits = session_search(
        &store,
        &SessionQuery {
            query: Some("zephyrtoken".into()),
            session_id: None,
            around_message_id: None,
        },
    )?;
    Ok(hits
        .first()
        .map(|hit| hit.session_id.clone())
        .unwrap_or_default())
}
