# Chapter 22: Recall

Chapter 21 can remember a fact that is true every session. It cannot answer "what did we decide last Tuesday?" The transcript is in the chapter 20 store, and the full-text rows were left for this chapter. When the user refers to a past conversation, the agent should search before it asks them to repeat it. This chapter is that search. The crate is `steps/ch22-recall`, a copy of chapter 21 plus `recall`. The package is `ch22-recall`. The search calls `crate::wal`. It does not take a dependency on a separate sessions crate, and it does not call a model.

`session_search` picks one shape from the arguments. A query is discover: BM25 hits, one row per session lineage, with the matching text on the top hit. A session id plus `around` is scroll, a window of messages. A session id alone is read, the session or its head and tail, with compaction summaries kept out of that view. No arguments is browse, the recent sessions. Subagent sessions are hidden. Automation sessions, cron and workers, are demoted so their repeated vocabulary does not push the user's own sessions out of the results. `insert_message` writes the rows the index rebuilds from. The virtual table is FTS5, created on purpose, with a trigram tokenizer, kept in sync by the insert path.

Discover over a large store has to stay off the hot path except when the model calls the tool. The index update is the background work chapter 20 queued. A turn that always searched would spend the request on recall. A turn that cannot search will ask the user to paste the same context again, and the pasted text will then be what reflection tries to learn from.

Chapter 23, on the server, will use usage and this kind of history to predict which tools to offer. It does not call `session_search`. The prediction stays a catalog decision. Chapter 24's skills are the other store: a procedure for a kind of task, loaded on demand, not a past message. Chapter 26's reviewer reads the current conversation, trimmed, and may look things up only through the tools it is given. Recall is the read path. It is not a write, and it is not a place a poisoned page gets to rewrite memory. That write stays in chapter 21, behind the scan.

Rust, beside the query: FTS5 is a SQLite virtual table, which is a schema you create rather than a search service. The mode is a match on which arguments were present, so an illegal combination is an error instead of a guess. `rusqlite` is the binding, with the bundled SQLite so the tokenizer is the one the tests expect.

`cargo run -p ch22-recall` runs the demo. The exercise is `my-harness/tests/ch22.rs`: discover hides a subagent session and demotes cron.

**In this chapter**

- 22.1 The virtual table is built on purpose
- 22.2 `session_search` picks one shape
- 22.3 Discover walks the index and keeps three sessions
- 22.4 Scroll, read, browse, and a hidden source
- 22.5 `insert_message` writes the rows the index rebuilds from

| Section | Step | What it adds |
|---|---|---|
| 22 | `steps/ch22-recall` | trigram FTS, discover, scroll, read, browse |

> **How Hermes does it**
>
> I read `tools/session_search_tool.py` (`session_search`, `_scroll`, `_HIDDEN_SESSION_SOURCES`) and `hermes_state_common.py` (`FTS_TRIGRAM_SQL`, `FTS_TRIGRAM_EXCLUDED_SOURCES`) on Hermes `main` at `cf71b1aa`.
>
> The tool picks a shape from the arguments. A `query` discovers. `session_id` plus `around_message_id` scrolls a window. The default window in `_scroll` is 5. `session_id` alone reads that session. No arguments browses recent sessions. `_HIDDEN_SESSION_SOURCES` is `kanban`, `subagent`, and `tool`. The trigram table is `messages_fts_trigram`, `tokenize='trigram'`, on an external content table. `FTS_TRIGRAM_EXCLUDED_SOURCES` is `cron` and `subagent`: those rows are not in the trigram index.
>
> **Adopted.** The four shapes. The scroll window of 5 on either side of the anchor. The hidden sources `subagent`, `kanban`, and `tool`. The virtual table name, `tokenize='trigram'`, and `content='messages'`. Discover returns one hit per session. The demo query is answered from the user session, not the child that contains the same token.
>
> **Adapted.** Cron is still in the index, and the `ORDER BY` puts `cron` and `worker` after other sources. Hermes leaves cron out of the trigram index and searches it another way. I did not port that split. There is no `messages_fts` beside the trigram table. Rebuild is one `INSERT ... VALUES('rebuild')` after the rows exist, not the trigger set in `FTS_TRIGRAM_SQL`.
>
> **Deferred.** CJK tokenizer, the rank explanation Hermes returns to the model, and compaction summaries staying out of a read. The queue from chapter 20 is not what feeds this index. `install_fts` rebuilds from `messages`.

## What this chapter decides

`install_fts` creates `messages_fts_trigram` if it is missing and rebuilds it from `messages`.

A discover query for `zephyrtoken` hits the user session `desk` first and the cron session `nightly` second. The subagent session `child` has the same token and is not in the list.

Scroll with `session_id` `desk` and `around_message_id` set to the first row returns `alpha` and `beta`. Read of `desk` returns both rows. Read of `child` returns nothing. Browse with no arguments lists `desk` before `nightly` and does not list `child`.

The demo builds a temp database, indexes it, and prints `hit: desk`. No key. No model.

## 22.1 The virtual table is built on purpose

Chapter 20 stores messages and queues their ids. The queue is still just a `VecDeque` in the process. This chapter adds `recall`, which opens with `use crate::wal::Store`. There is no path dependency on the `ch20-sessions` package. The step crate is chapter 21's harness copied forward, plus this module. `lib.rs` adds `pub mod recall`. The WAL module comment in this copy is still the chapter 20 comment: indexing is queued, and "This chapter does not build the FTS table." That sentence stays true of `wal.rs`. The `CREATE VIRTUAL TABLE` lives in `recall::install_fts`.

`recall` reaches the store through four functions. `Store::open` and `Store::migrate` are what `demo_db` calls before any row exists. `connection` and `connection_mut` are how every later statement borrows the same file.

```rust
pub fn connection(&self) -> &Connection {
    &self.conn
}

pub fn connection_mut(&mut self) -> &mut Connection {
    &mut self.conn
}
```

`connection` is a shared borrow. `session_search` can take `&Store` because prepare, query, and `query_row` only need that. `connection_mut` is the mutable borrow. `install_fts` and `insert_message` take `&mut Store` so they can call `transaction()` and the other write statements. Rust will not hand out both borrows at once. A search cannot run on a store while `install_fts` still holds it.

`append_turn` is the writer the inherited tests still call. It is the chapter 20 function, copied forward with the redact calls still in the parameter lists.

```rust
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
        params![session_id, crate::redact::redact_sensitive_text(user), now],
    )?;
    let user_id = tx.last_insert_rowid();
    tx.execute(
        "INSERT INTO messages (session_id, role, content, ts) VALUES (?1, 'assistant', ?2, ?3)",
        params![
            session_id,
            crate::redact::redact_sensitive_text(assistant),
            now
        ],
    )?;
    let assistant_id = tx.last_insert_rowid();
    tx.commit()?;
    self.enqueue(user_id);
    self.enqueue(assistant_id);
    Ok(2)
}
```

Both strings still pass through `redact_sensitive_text` before the insert. The session source is the literal `'user'`. The commit happens, then the two ids are pushed on the in-memory queue, capped at 64. I read this function looking for a virtual table. The statements are the session insert and the two message inserts. There is no `CREATE VIRTUAL TABLE`, and there is no `VALUES('rebuild')`. `enqueue` does not touch SQLite at all. I am leaving that split. `hi` and `pong` still miss every redaction pattern, so the session test can keep expecting those words.

The inherited test is `a_turn_appends_two_rows_and_queues_fts_without_a_virtual_table`. It opens a temp file, migrates, checks that `journal_mode` is `wal`, appends `s1` / `hi` / `pong`, expects the return value 2, expects the two rows in that order, expects `pending_fts()` to be 2, and then runs:

```rust
"SELECT COUNT(*) FROM sqlite_master WHERE name = 'messages_fts_trigram'"
```

It expects the count to be 0. `sqlite_master` is the catalog. A table created inside `append_turn` would make that count 1 and fail the test. The test runs against `wal` before any recall setup. Read it before moving the `CREATE` into the writer.

The same test then drops the store and deletes three paths. Dropping the store closes the connection. WAL mode keeps two siblings beside the database file: the path plus `-wal` is the write-ahead log, and the path plus `-shm` is the shared-memory index SQLite uses to coordinate readers with that log. The cleanup builds those two names with `OsString::push` and removes all three. `let _ =` ignores a missing file. A failed remove must not hide the assertion that already ran. The recall tests use the same cleanup. `install_fts` does not change those names. It adds a virtual table inside the same database file.

Search starts when something calls `install_fts`.

```rust
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
```

`install_fts` takes `&mut Store` because it uses `connection_mut`. Chapter 20 exposed that borrow so another module could run its own write statements. `session_search` takes `&Store` and uses `connection`, the shared borrow. A search and an `install_fts` cannot hold the store at the same time. The mutable borrow lasts until `install_fts` returns.

The two calls are different on purpose. `execute_batch` runs the `CREATE`, which is one statement with a trailing semicolon and no parameters. `execute` runs the rebuild, which is one statement and an empty parameter list. The `CREATE` is skipped when the name already exists, because of `IF NOT EXISTS`. The `INSERT` is not skipped. A second `install_fts` on a store that already has the virtual table still rebuilds it from whatever `messages` holds now. That is the only refresh this crate has.

> **Rust sidebar: the statement borrows the connection**
>
> - `prepare` returns a statement that borrows the connection. `query_map` returns an iterator that borrows the statement. `collect` or the `for` loop has to finish before the statement goes out of scope. That is why `stmt` is still alive at the `collect` in scroll, read, and browse, and still alive at the `for` loop in discover.
> - The closure passed to `query_map` returns `Result<Hit, rusqlite::Error>`. `row.get(0)?` is a column that failed to decode, or a column index that is not there. The `?` inside the closure becomes the iterator's `Err` for that row.
> - `rows.collect()` turns an iterator of results into one `Result<Vec<Hit>, _>`. The first bad row is the error. The hits from earlier rows in that same collect are dropped with it. Discover does not use `collect`. It uses `let hit = row?` inside the loop, which returns from `discover` on the first bad row and drops the `hits` vec built so far.
> - `params!` and `[]` are the bound values. `prepare` saw the SQL text. The values arrive later, at `query_map`. Discover's query is bound there. Browse passes `[]` because its SQL has no placeholders.

> **Rust sidebar: the index is a virtual table**
>
> - A normal `CREATE TABLE` stores rows in SQLite's own table format. `CREATE VIRTUAL TABLE ... USING fts5` hands that name to the FTS5 module. The module answers `SELECT` and `INSERT` for `messages_fts_trigram`. The session test looks for that name in `sqlite_master`. Before `install_fts`, the name is absent.
> - `content='messages'` and `content_rowid='id'` tell FTS5 that the text already lives in `messages`, and that the index row id is `messages.id`. The virtual table declares one column, `content`, which is the column it indexes. `MATCH` reads the index. The join in discover reads the message text back from `messages`.
> - `tokenize='trigram'` is the tokenizer argument this SQL passes. A trigram tokenizer indexes overlapping three-character slices, which is why a search for a fragment can find a row without a stemming dictionary. This crate does not implement that tokenizer. It names it.
> - Inserts into `messages` do not update the virtual table by themselves. The triggers chapter 20 deferred are what would keep the index in step with the rows. This chapter does not create those triggers. It runs one rebuild.
> - The bench's 10,000 messages are inserted in the process. There is no network.

The rebuild statement, after the `CREATE`, is:

```sql
INSERT INTO messages_fts_trigram(messages_fts_trigram) VALUES('rebuild')
```

The column name is the table name. That is the FTS5 command form. The value `'rebuild'` tells the module to read `messages` and fill the index from those rows. The parameter list is `[]` because the SQL has no `?` placeholder. The word `rebuild` is a literal in the statement, not user data.

I run this after the rows exist. A message inserted later, including one from `append_turn`, is not in the index until something calls `install_fts` again. Chapter 20's queue is not drained here. `pending_fts` is never read by `recall`. If you append and search in the same process without rebuilding, the new row is missing from `MATCH`. That gap is the one the triggers close in Hermes. This chapter rebuilds on purpose, once, before the tests search and before the bench samples.

The discover bench inserts 10,000 messages into session `desk` inside its own transaction, commits, then calls `install_fts`, then times `session_search`. Message 5,000's content is `zephyrtoken in the middle`. The other rows are `note N`. The timed closure does not insert and does not rebuild. It searches.

## 22.2 `session_search` picks one shape

The arguments are one struct. Every field is optional, and the combination is the shape.

```rust
pub struct SessionQuery {
    pub query: Option<String>,
    pub session_id: Option<String>,
    pub around_message_id: Option<i64>,
}
```

`Hit` is the row I hand back. All four shapes use it, even browse, which does not have a message.

```rust
pub struct Hit {
    pub session_id: String,
    pub message_id: i64,
    pub content: String,
}
```

```rust
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
```

`as_deref` on `Option<String>` turns it into `Option<&str>`, a borrow of the string inside. `around_message_id` is already an `Option<i64>`, and `i64` is `Copy`, so the match copies the number. The query goes through `filter`. `None` stays `None`. `Some("")` becomes `None` because the closure rejects an empty string. A query of only whitespace is not empty, so it is kept. I do not trim it.

The arm order is the policy.

A `session_id` and an `around_message_id` together are scroll. The third field is `_`. A query string on that same struct is ignored. The window wins.

A `session_id` and no anchor are read. A query string on that struct is ignored too. Naming a session reads it. It does not search inside it.

Discover is the third arm. It needs `Some(text)` from that filtered query. The first two arms have already taken every `Some(session_id)`. So discover runs when the session id is `None` and the query is a non-empty string. An anchor without a session id does not select scroll. The pair `(None, Some(around), Some(text))` fails the first arm, fails the second, and takes discover. The anchor is ignored. The pair `(None, Some(around), None)` fails the first three arms and browses. The anchor alone does not scroll.

Browse is the last arm. No session id, and no non-empty query. The demo and the discover test set `session_id: None` and `around_message_id: None` and a `Some` query, so they take discover. The browse test sets all three to `None`.

The tests pass the structs the match actually sees. Discover is:

```rust
SessionQuery {
    query: Some("zephyrtoken".into()),
    session_id: None,
    around_message_id: None,
}
```

The triple is `(None, None, Some("zephyrtoken"))`. The first two arms need a session id. The third arm takes the text.

Scroll in `scroll_read_and_browse_are_the_other_three_shapes` sets `query: None`, `session_id: Some("desk".into())`, and `around_message_id: Some(anchor)`. The triple is `(Some("desk"), Some(anchor), None)`. The first arm matches. The query filter is never consulted, because `_` already accepted that position.

Read of `desk` uses the same session id, `query: None`, and `around_message_id: None`. The triple is `(Some("desk"), None, None)`. The first arm fails on the missing anchor. The second arm matches. Read of `child` is the same shape with `Some("child")`. The match does not know that `child` is hidden. `read` finds that out afterward.

Browse is `query: None`, `session_id: None`, `around_message_id: None`. The triple is `(None, None, None)`. Every `Some` arm fails. The last arm runs.

An empty session id is still `Some("")`. It is not filtered. `Some("")` with no anchor takes read of the id `""`. `Some("")` with an anchor takes scroll of that id. Only the query field treats `""` as absent. A query of `Some("   ")` is not empty, so with no session id it is discover, and FTS5 receives three spaces.

The tool returns `Vec<Hit>`. It does not call a model. A SQLite error from the shape that ran is the error the caller sees.

## 22.3 Discover walks the index and keeps three sessions

Discover is the shape the bench times. The SQL is built with `format!` because the hidden-source list is spliced in as text. The query string is not spliced in. It is bound as `?1`.

```rust
const HIDDEN: &str = "('subagent', 'kanban', 'tool')";
```

```rust
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
```

`HIDDEN` is already a parenthesized list of SQL string literals. It is a constant in this file, not a value from the caller. After `format!` the statement text is this:

```sql
SELECT s.id, m.id, m.content
FROM messages_fts_trigram
JOIN messages m ON m.id = messages_fts_trigram.rowid
JOIN sessions s ON s.id = m.session_id
WHERE messages_fts_trigram MATCH ?1
  AND s.source NOT IN ('subagent', 'kanban', 'tool')
ORDER BY CASE WHEN s.source IN ('cron', 'worker') THEN 1 ELSE 0 END, rank
```

That is the string `prepare` compiles. I did not add a column, a trigger, or a second virtual table. `?1` is still a placeholder. A subagent session, a kanban session, and a tool session fail the `NOT IN` and drop out. Cron is not in that list. Worker is not in that list. They stay searchable.

The loop's `clone` is a move, not a style choice. `hit.session_id` is an owned `String`. `seen` needs its own copy so the comparison on the next row still has the id. `hits.push(hit)` then moves the whole `Hit`, including the original session id, the message id, and the content. If I pushed first, the id would be gone and the `clone` would not compile. `continue` skips that push. The session id is not added to `seen` on a skip, because it is already there. `break` leaves the function to return the three hits it has. The iterator is dropped with the rest of the locals. Unread rows are not an error.

`MATCH ?1` is the trigram query. `params![query]` binds the caller's string. A quote in the query is data. It is not a broken SQL string, because it never becomes part of the statement text. The code does not check the length of `query`. A one-character string is still bound and handed to FTS5. Trigram `MATCH` wants at least three characters. The short query is FTS5's problem. This function does not special-case it.

The first join uses `messages_fts_trigram.rowid`, which `content_rowid='id'` defined as `messages.id`. The second join loads the session so the source can be filtered and sorted. The selected columns are the session id, the message id, and the message content. Those are the three fields of `Hit`, in that order. `row.get` reads them by index.

The `ORDER BY` has two keys. The first is a `CASE`. `cron` and `worker` sort as 1. Every other source sorts as 0. Zero comes first, so a user session sorts before a cron session that also matched. The second key is `rank`, with no `DESC`, so it is ascending. `rank` is the column this FTS5 table exposes on a `MATCH`. The SQL does not compute a score in Rust.

The SQL can return several messages from one session. The row that survives is the first one in that `ORDER BY`, because the loop keeps the first id it has not seen. A query that matches ten user sessions returns three. The hide-and-demote test matches two visible sessions, so the cap of three is not what decides that result.

`discover_hides_a_subagent_and_demotes_cron` inserts three messages before `install_fts`. `child` is source `subagent` and content `zephyrtoken hidden`. `nightly` is source `cron` and content `zephyrtoken cron`. `desk` is source `user` and content `zephyrtoken desk`. The query is `zephyrtoken`. The first hit's session id is `desk`. No hit has session id `child`. The second hit is `nightly`. The `NOT IN` removed the child. The `CASE` put the user session ahead of cron. The token is long enough for a trigram match, and it sits in all three contents, so the filter and the sort are what the test is locking, not the question of whether the word exists.

The demo builds the same kind of split with two sessions. `demo_db` inserts the child first and the desk second, then installs the index, then searches. Insertion order is not the hit order. The child is hidden. The first hit is `desk`.

## 22.4 Scroll, read, browse, and a hidden source

Scroll and read share a helper the SQL in discover does not use. Discover filters in the query. These two ask one session for its source, and they return no rows when the source is hidden.

```rust
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
```

`query_row` returns an error when the id is missing, and it returns an error when the statement fails. `.ok()` turns both into `None`. A missing session is then `source == None`, and `matches!` is false, so `hidden` returns `Ok(false)`. The following `SELECT` on `messages` returns no rows. A real SQLite error on the source lookup is also swallowed here. The function reports "not hidden" and lets the next statement be the one that fails, if it fails.

`matches!` is true only for the three sources. `user`, `cron`, and `worker` are not hidden. Cron is demoted in discover and browse. It is not removed. A scroll or a read of a cron id returns that session's messages.

```rust
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
```

The window is the anchor id minus 5 through the anchor id plus 5. `BETWEEN` is inclusive. The subtraction happens in Rust, in `params![session_id, around - 5, around + 5]`, and the two integers are bound as `?2` and `?3`. SQL does not see the expression `around - 5`. It sees the numbers. An anchor of 1 binds `-4` and `6`. This schema's row ids start at 1, so the negative lower bound adds no rows, and ids 1 through 6 are inside. An id of 7 is outside even if it is the next message after id 1, which is what a gap looks like. The 5 is written in the parameter list, not taken from `SessionQuery`. There is no row limit besides that range. The order is `id`. A hidden session returns an empty vec before the `SELECT`. The `?` on `hidden` still passes a real error through, except that `hidden` itself turned lookup errors into `Ok(false)`.

The scroll test inserts `alpha` into `desk` and uses that row id as the anchor, then inserts `beta` into the same session. Those two ids are adjacent and both fall inside the window. The test expects both contents. It does not insert enough rows to fall outside the window. The SQL is still the window above.

```rust
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
```

Read is every message in the session, in id order, with no window. A hidden session is an empty vec. The test reads `desk` and expects length 2, `alpha` and `beta`. It reads `child` and expects an empty vec. `child` is source `subagent`, so `hidden` returns true and `read` never runs the `SELECT`. An unknown id also comes back empty, by the missing-row path above, not by the hidden path.

Browse does not look at `messages` or at the virtual table. It lists sessions.

```rust
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
```

After `format!` the browse statement is:

```sql
SELECT id, source FROM sessions
WHERE source NOT IN ('subagent', 'kanban', 'tool')
ORDER BY CASE WHEN source IN ('cron', 'worker') THEN 1 ELSE 0 END, started_at DESC
LIMIT 10
```

`query_map` is called with `[]`. There is no `?` to bind, and there is no `MATCH`. The `NOT IN` list is the same constant discover uses. The `CASE` sorts `cron` and `worker` after everyone else. `started_at DESC` is the second key, so a later start comes first inside the same group. `LIMIT 10` caps the list. An eleventh visible session is not returned. The tests insert two visible sessions, so the limit is not what cuts the result.

The `Hit` is a session dressed as a hit. `session_id` is `sessions.id`. `message_id` is the literal `0`, not a message row. `content` is the source string, not message text. A browse hit for `desk` has content `user`. A browse hit for `nightly` has content `cron`. Callers that print `content` and expect a message will see the source instead. That is what this function stores.

The browse test inserts `desk` (`user`, `alpha`), another row in `desk` (`beta`), `nightly` (`cron`, `gamma`), and `child` (`subagent`, `delta`). All four session rows go through `insert_message`, which writes `started_at` as the constant `1`. The `CASE` is what puts `desk` first. `started_at DESC` does not separate them, because every inserted session has the same start. The test expects the first session id to be `desk`, expects `nightly` to be present, and expects `child` to be absent. It does not assert a total order beyond that. `worker` is in the `CASE` and not in this test. A worker session would sort with cron. `kanban` and `tool` are in `HIDDEN` and in `hidden`, and neither test inserts them. A session with one of those sources is dropped by browse and comes back empty from scroll and read.

`rows.collect()` is the same collect as chapter 20's `messages`. The iterator yields `Result<Hit, _>`. The first error stops it. A clean run is the whole vec, which for browse is at most ten hits.

## 22.5 `insert_message` writes the rows the index rebuilds from

The recall tests need a cron session and a subagent session. `append_turn` cannot make those. Its session insert hardcodes source `'user'`. `insert_message` takes the source as an argument. It also does not redact. A key passed here is stored as itself and, after `install_fts`, it is indexed as itself. The tests pass `zephyrtoken`, `alpha`, `beta`, `gamma`, and `delta`. None of those is a key. The write path that scrubs is still `append_turn`.

```rust
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
```

This is one transaction, the same shape as `append_turn`. The session row is `INSERT OR IGNORE` on the text primary key. A second message in `desk` does not change `source` or `started_at`. `parent_id` and `title` are null. `started_at` is the literal `1`, not `unix_secs()`. Every session created through this function shares that start time, which is why browse's time sort does not reorder the test data. The message role is the literal `'user'`. `ts` is the literal `1`. The content is bound as `?2` with no call to `redact_sensitive_text`.

`last_insert_rowid` is read after the message insert, so a resumed session still returns the new message id. The ignored session insert is not the value we keep. An `OR IGNORE` that inserts nothing does not wipe the row id the next statement writes. `Ok(id)` is that row id. `append_turn` returns the count `2`. This function returns the id the window will use. The caller binds it or ignores it. Scroll's test passes it as `around_message_id`. The first call in that test is the anchor, and the second call, `beta` in the same `desk` session, returns the next id. `commit` makes both rows visible. An error before `commit` drops the transaction and rolls it back, so a failed message insert does not leave a new session row either. Unlike `append_turn`, this function does not enqueue the id. `pending_fts` stays whatever `append_turn` last did. The index learns the row only from the later rebuild.

`demo_db` is the binary's whole search.

```rust
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
```

`Store::open` and `migrate` are the chapter 20 pair: the connection, the two-second busy timeout, `PRAGMA journal_mode=WAL`, and the two `CREATE TABLE IF NOT EXISTS` statements. Then two messages, then the virtual table and the rebuild, then a discover query. `hits.first()` is `Option<&Hit>`. The `map` clones `session_id` so the owned `String` does not borrow the vec. `unwrap_or_default` on that `Option<String>` is `""` when `first` is `None`, which is an empty hit list. The function's `Result` is still `Ok`. An empty search is not a SQLite error. The test `the_demo_hit_is_the_user_session` expects `"desk"`. The child session contains the same token and is source `subagent`, so discover drops it. The binary prints `mode: discover` and `hit: ` plus that string. It deletes the temp file after `demo_db` returns. The mode line is a constant in `main`. The function above always searches with a query and no session id, which is the discover arm.

`hi` and `pong` still round-trip through `append_turn` in the inherited session tests. Those words miss every redaction pattern, the journal is still `wal`, and `messages_fts_trigram` is still absent until `install_fts` runs. The recall tests call `install_fts` themselves. The session tests do not, and they are how I keep the `CREATE` out of `append_turn`.

## Tests

Three tests. Discover hides the subagent and lists the user session before cron. Scroll, read, and browse are the other three shapes, and a hidden session reads as empty. The demo hit is `desk`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch22-recall
```

```text
Starting 117 tests across 30 binaries
     Summary [   0.228s] 117 tests run: 117 passed, 0 skipped
```

```bash
cargo run -p ch22-recall
```

```text
mode: discover
hit: desk
```

The binary is one temp database. A child session and a user session both contain `zephyrtoken`. The hit is the user session.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `discover_10k` | 60.80 µs | under 20 ms | one trigram query over 10,000 messages |

`discover_10k` opens a temp database, migrates, and inserts 10,000 messages into session `desk` before the bench. Message 5,000 contains `zephyrtoken`. The other 9,999 are `note N`. `install_fts` rebuilds the trigram index before the bench. The timed closure is `b.iter`. Inside: `session_search` with the query `zephyrtoken`, no session id, and no anchor, both the store and the query passed through `black_box`. The inserts and the rebuild are excluded. No model is called. The result is one hit in `desk`.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `22`.

### Milestone demo

The `cargo run` above. The mode is `discover`. The hit is `desk`.

## Pitfalls

A message inserted after `install_fts` is not in the index until the next rebuild. Chapter 20's queue is not drained here. If you append and search in the same process without rebuilding, the new row is missing.

Cron is searchable. It sorts after a user session that also matched. A query that only the cron session contains will still return that session. Hiding cron would drop a log you might have wanted. Demoting it is the choice this chapter made.

The scroll window is ids, not "five messages of text." If ids have gaps, the window is still `anchor - 5` through `anchor + 5`.

Trigram `MATCH` wants at least three characters. A one-letter query is the wrong tool.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Recall is persistence, layer L8, and it is not a write.

**What landed.** Search reads the session file. It does not call a model. Subagent, kanban, and tool sessions are hidden, which is `_HIDDEN_SESSION_SOURCES`. Cron and worker sessions sort after the others so a repeated automation log does not take the first row when the user's session also matched.

**What did not.** The index is not updated on every insert. The full trigger set in `FTS_TRIGRAM_SQL` is not ported. Cron is not excluded from the trigram table the way `FTS_TRIGRAM_EXCLUDED_SOURCES` excludes it.

**Left for later.** A poisoned memory file is chapter 21's scan, not this search. Transcript redaction (E1, E2) is chapter 20: `append_turn` scrubs before the insert. This search does not scrub again.

## Chapter summary

- **Index.** `messages_fts_trigram`, trigram tokenizer, rebuilt from `messages`.
- **Discover.** One hit per session. `desk` before `nightly`. `child` is absent.
- **Other shapes.** Scroll is a window of 5. Read is the session. Browse is recent sessions, with cron after the user.
- **Time.** One discover over 10,000 messages measured **60.80 µs**.

## What you implement

`my-harness/tests/ch22.rs` is the milestone. It compiles, then panics in `todo!("chapter 22")` until `chapter_22_discover_hides_subagent("subagent")` is true and `chapter_22_discover_hides_subagent("user")` is false. Discover hides a subagent session and keeps a user session. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 23 is the server again: learned tools. The harness side of part II continues with skills. The session file and the memory files are what those later chapters read.
