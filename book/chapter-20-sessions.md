# Chapter 20: Sessions on Disk

Chapter 2 can save a conversation as one JSON file, and chapter 19 can fold a stream into text worth keeping. Neither of those is a session you can resume, search, or trust with a secret that passed through the transcript. A learning agent that forgets every session can only learn inside one sitting. This chapter puts the session on disk, and it scrubs the text on the way in. The crate is `steps/ch20-sessions`, a copy of chapter 19 plus `wal` and `redact`. The package is `ch20-sessions`. The library is `harness_core`. `lib.rs` re-exports the redaction functions from here on, so later chapters call `crate::redact` instead of growing a second scrub.

`Store` is a SQLite connection in WAL mode. Two tables hold the session and its messages. `append_turn` runs both strings through `redact_sensitive_text` before the insert, so a turn that contains a key is stored without the secret. The mask keeps the prefix through the first `-` or `_`, then stars. The patterns cover the prefix families, environment assignments, `Authorization` headers, and PEM blocks. `redact_for_egress` is that scrub plus a sweep that turns a bare bearer token into `Bearer [redacted]`. A background queue does the write. Full-text search is not built here. The ids are queued. Chapter 22 builds the search on this store.

The scrub is the one later chapters are allowed to call. Chapter 21's memory writes go through it when a secret would otherwise land in the file. Chapter 39's outbound chat, chapter 47's bus delivery, and chapter 49's external-agent call all call `redact_for_egress`. They do not invent a narrower one. Chapter 52's red-team case for persistence calls this module. That is the point of copying the harness forward: the function is still there, under the same name, on the later crate.

This is the start of the learning half of the book. Memory, recall, skills, reflection, and curation all assume a session exists and that a secret in the transcript did not become part of the lesson. Hermes keeps sessions in SQLite and scrubs before text leaves or lands. The same split is here, with JSON from chapter 2 left behind as the export format rather than the store.

Rust, beside the connection: SQLite is a library linked into the process, WAL is the mode that lets a reader proceed while a writer appends, and a prepared statement is the insert. `redact` is plain string replacement driven by a regex set compiled once. `Result` is a disk error, which is different from a model error and should stay that way.

`cargo run -p ch20-sessions` runs the demo. The exercise is `my-harness/tests/ch20.rs`: a turn that contains an `sk-` token and `OPENAI_API_KEY=` is stored without the secret, and egress strips a bare bearer.

**In this chapter**

- 20.1 A file, a connection, and two tables
- 20.2 One transaction, then the queue
- 20.3 The prefix patterns, compiled once
- 20.4 Env assignments, the auth header, and egress

| Section | Step | What it adds |
|---|---|---|
| 20 | `steps/ch20-sessions` | WAL sessions, an append of one turn, a queued FTS id |

> **How Hermes does it**
>
> I read `hermes_state.py` (the writer connection sets journal mode and probes `messages_fts` and `messages_fts_trigram`) and `hermes_state_common.py` (`FTS_TRIGRAM_SQL`, `tokenize='trigram'`) on Hermes `main` at `cf71b1aa`. The table list this book uses is in [`docs/self-improvement.md`](../docs/self-improvement.md): `sessions(id, source, parent_id, started_at, title)` and `messages(id, session_id, role, content, ts)`.
>
> WAL is the journal mode on the writer connection. The FTS tables are separate from the insert. Hermes keeps them in sync with triggers. A trigram index is the expensive one, and Hermes does not put every row in it.
>
> **Adopted.** The two tables. `PRAGMA journal_mode=WAL`. One transaction for the user row and the assistant row. The session row is inserted once (`INSERT OR IGNORE`).
>
> **Adapted.** The FTS virtual table is not created here. `append_turn` pushes the new row ids onto a queue of 64 and drops the oldest. Nothing in this crate runs a `MATCH` query. `rusqlite` is bundled so CI does not need a system SQLite.
>
> **Deferred.** Trigram search, hiding subagent sessions, and demoting cron are chapter 22. The triggers in `FTS_TRIGRAM_SQL` land there, in a smaller form.

## What this chapter decides

`Store::open` opens a file. `migrate` sets WAL and creates the two tables if they are missing.

`journal_mode` is `wal`.

`append_turn("s1", "hi", "pong")` returns 2. `messages("s1")` is the user row and then the assistant row. `pending_fts` is 2. `sqlite_master` has no `messages_fts_trigram`.

A second `append_turn` on `s1` makes four messages and still one session row.

The demo writes `ch20-sessions-demo.db` under the temp directory, appends the fake reply `pong`, prints the row count and the journal mode, and deletes the file. No key.

The third test calls `run_saved_evals` and `run_stress_evals` from chapter 9. Both lists are all `passed`. That work is the mock loop. It does not open this database.

## 20.1 A file, a connection, and two tables

A session has to outlive the process. I put it in a SQLite file. The Rust type that holds the open file is `rusqlite::Connection`. `Store` owns that connection, and it owns the queue of row ids that a later chapter will index.

```rust
pub struct Store {
    conn: Connection,
    pending_fts: VecDeque<i64>,
}
```

`Connection` is the handle. Queries go through it. `pending_fts` is a `VecDeque<i64>`. A `VecDeque` is a buffer with a front and a back. I push new ids on the back and drop old ids from the front. The id is an `i64` because that is what SQLite returns from `last_insert_rowid`. The queue lives in the process. It is not a table. Open the same file in a new process and the queue starts empty. The messages are still in the file.

> **Rust sidebar: SQLite from Rust**
>
> - A database here is a file. `Connection::open` takes a `&Path` and returns `Result<Connection, rusqlite::Error>`. The path is borrowed. The connection owns the open file.
> - `Result` is the error channel. The `?` after a call means: if this is `Err`, return that error from the function; if it is `Ok`, keep the value. `open`, `migrate`, and `append_turn` all use it. A caller gets either the store or a SQLite error. I do not invent a second error enum for this file.
> - SQL in this crate is a string. `?1`, `?2`, and `?3` are placeholders. `params!` builds the list that fills them. The text of a message is data in that list. It is not pasted into the SQL string.
> - `execute` runs one statement. `execute_batch` runs several, separated by semicolons. `query_row` runs a statement that must return one row and maps that row with a closure.

`open` creates the connection and an empty queue. It does not create tables.

```rust
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
```

`Path` is a borrowed path, the way `&str` is a borrowed string. The demo and the tests own a `PathBuf` and lend it. `Connection::open` creates the file if it is missing. A second `open` of the same path sees the rows that were committed.

`busy_timeout` tells this connection to wait up to two seconds when SQLite says the file is busy, instead of failing the next statement at once. The comment in `open` is the reason: parallel tests can touch neighboring files while macOS still reports the new file busy. The wait is on the connection. It is not a lock I take in Rust.

`migrate` is the next call. Every test and the demo call it before a write. `append_turn` assumes the tables exist. I do not call `migrate` from `append_turn`, so a store that was only opened fails the insert with a SQLite error. That split keeps "create the schema" visible.

```rust
pub fn migrate(&self) -> Result<(), rusqlite::Error> {
    self.conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    self.conn.execute_batch(SCHEMA)?;
    Ok(())
}
```

The first batch is one pragma. `journal_mode=WAL` turns on write-ahead logging for this file. SQLite stores that choice in the file, so a later open still reports it. WAL means commits land in a log beside the database. Readers of the main file do not have to wait for a writer the way they do with the older rollback journal. The demo's journal line is this pragma read back.

```rust
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
```

`CREATE TABLE IF NOT EXISTS` is safe to run twice. The second `migrate` sees the tables and leaves the rows. `sessions.id` is text, and it is the primary key. `source` has to be present. `parent_id` and `title` may be null. `started_at` is an integer. `messages.id` is `INTEGER PRIMARY KEY`. In SQLite that column is the row id. I never send an id in the insert. SQLite assigns one, and `last_insert_rowid` reads it back. `session_id`, `role`, and `content` are required text. `ts` is an integer. There is no foreign key in this SQL. A message can name a session id I have not inserted. `append_turn` does not do that. It inserts the session row first.

`journal_mode` reads the pragma back as one string. After `migrate` the tests expect `"wal"`.

```rust
pub fn journal_mode(&self) -> Result<String, rusqlite::Error> {
    self.conn
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
}
```

The empty `[]` is the parameter list. The closure gets one row. `row.get(0)` is column zero, the mode name. `query_row` returns an error if SQLite returns no row. A pragma always returns one, so the tests see the string.

Two borrows of the connection are public because chapter 22's recall code lives in another module and has to run its own SQL.

```rust
pub fn connection(&self) -> &Connection {
    &self.conn
}

pub fn connection_mut(&mut self) -> &mut Connection {
    &mut self.conn
}
```

`connection` is a shared borrow. `journal_mode` and `messages` only need that. `connection_mut` is a mutable borrow. `transaction()` on this version of `rusqlite` takes `&mut Connection`, so a writer needs it. Rust allows many shared borrows, or one mutable borrow, not both at once. A function that holds `connection_mut` is the only user of that store until it returns.

```rust
pub fn pending_fts(&self) -> usize {
    self.pending_fts.len()
}
```

This is the length of the in-memory queue, not a count of indexed rows. After one turn it is 2. It never grows past 64.

`unix_secs` is the clock both rows share.

```rust
fn unix_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}
```

`SystemTime::now` is the machine clock. `duration_since(UNIX_EPOCH)` is the time since 1970, or an error if the clock is set earlier than that. The `map` keeps the whole seconds and casts them to `i64`. `unwrap_or(0)` is the branch for a clock before the epoch. I would rather store a zero than refuse the turn. Both message rows and, on the first insert, `sessions.started_at` use this one number. A second turn on the same session does not change `started_at`, because that insert is ignored.

```rust
pub fn demo_reply() -> &'static str {
    "pong"
}
```

`&'static str` is a string baked into the binary. The demo prints it and passes it as the assistant text. There is no model in this binary.

`lib.rs` declares the modules and re-exports the scrub functions. `Store` stays behind the module. Tests construct `harness_core::wal::Store`. The redact tests import the three functions from the crate root.

```rust
pub mod redact;
pub mod wal;

pub use redact::{redact_for_egress, redact_sensitive_text, redacting_formatter};
```

## 20.2 One transaction, then the queue

`append_turn` writes one user row and one assistant row, then remembers both ids. The virtual table is not in this function. I want that to stay obvious, because chapter 22's copy of the session test still looks for the table and still expects it to be missing.

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

The signature takes `&mut self` because the transaction borrows the connection mutably, and because the queue push needs to change `pending_fts`. `session_id`, `user`, and `assistant` are borrowed strings. The function does not keep them. It stores copies inside SQLite.

`now` is computed once, before the transaction. The session row and both message rows carry the same timestamp.

> **Rust sidebar: a transaction**
>
> - `self.conn.transaction()` starts a transaction and returns a `Transaction` value. The statements I run on `tx` are part of that transaction.
> - `commit` writes the transaction. In WAL mode that commit is the append to the log. A reader that starts after the commit sees both message rows.
> - `tx` owns the transaction. If an `?` returns before `commit`, `tx` is dropped at the end of that early return. Dropping an uncommitted `Transaction` rolls it back. A failure on the assistant insert does not leave the user row behind, and it does not leave a new session row behind either.
> - The queue push is after `commit`. A rollback never enqueues an id. A commit that succeeds and then a panic before `enqueue` would leave rows with no queued id. The queue is a hint for a background job. The file is the record.

The session insert is `INSERT OR IGNORE`. The primary key is `id`. The first turn for `"s1"` inserts one sessions row: source `'user'`, `parent_id` null, `started_at` set to `now`, `title` null. The source string is fixed in the SQL. This function cannot record a cron session or a subagent session. Chapter 22 has a different insert for those. A second `append_turn` for `"s1"` hits the same primary key. `OR IGNORE` skips that insert. It does not update `source`, `parent_id`, `started_at`, or `title`. The test that appends twice expects `SELECT COUNT(*) FROM sessions` to be 1.

I read `last_insert_rowid` after each message insert, not after the session insert. SQLite remembers the last row id on the connection. The message insert is the statement that sets the id I care about. On a resumed session the ignored session insert does not change that, and I still read the id only after the message statement.

Both message values pass through `crate::redact::redact_sensitive_text` in the `params!` list. That call runs before `execute` sends the row. The user string is scrubbed. The assistant string is scrubbed. The session id is not scrubbed. The role is the literal `'user'` or `'assistant'` in the SQL, not a parameter I could forget to scrub. `ts` is the integer `now`.

`Ok(2)` is the literal at the bottom. It is not `changes()`, and it is not the number of session rows. Both inserts have already succeeded, or the function would have returned on `?`. One turn reports two message rows. The resume test does not use that return for the total. It reads `messages` and expects four pairs.

`enqueue` is the cap.

```rust
fn enqueue(&mut self, id: i64) {
    if self.pending_fts.len() == PENDING_CAP {
        self.pending_fts.pop_front();
    }
    self.pending_fts.push_back(id);
}
```

`PENDING_CAP` is 64. The check is `==`, not `>`. At 64 ids, the oldest is removed, then the new id is pushed, and the length stays 64. Under 64, the length grows by one. One turn calls `enqueue` twice, user id then assistant id, so the assistant id is the newer one. A long bench of `append_turn` fills the queue and then forgets the oldest id on every further push. Those forgotten ids are still rows in `messages`. Nothing in this crate drains the queue into an index. Chapter 22 rebuilds from the table. It does not read `pending_fts`.

`messages` is the read path the tests use.

```rust
pub fn messages(&self, session_id: &str) -> Result<Vec<(String, String)>, rusqlite::Error> {
    let mut stmt = self
        .conn
        .prepare("SELECT role, content FROM messages WHERE session_id = ?1 ORDER BY id")?;
    let rows = stmt.query_map(params![session_id], |row| Ok((row.get(0)?, row.get(1)?)))?;
    rows.collect()
}
```

`prepare` compiles the statement. `query_map` binds the session id and runs the closure on each row. The closure returns `Ok((role, content))`. `row.get` fails if the column is the wrong type, and `?` turns that into the closure's `Err`. `query_map` itself returns an iterator of results. `collect` on that iterator is a `Result<Vec<_>, _>`: the first row error stops the collect, and a clean run is the whole vec. `ORDER BY id` is append order, because the ids are the row ids SQLite assigned. A session with no rows is an empty vec, not an error. A missing table is an error, which is what you get if you skip `migrate`.

The first session test calls `append_turn("s1", "hi", "pong")` and expects exactly those two strings back, in that order, with `pending_fts() == 2`. It also runs this count:

```rust
"SELECT COUNT(*) FROM sqlite_master WHERE name = 'messages_fts_trigram'"
```

The count is 0. `sqlite_master` is SQLite's catalog of tables. `append_turn` never creates `messages_fts_trigram`, so the name is absent. I checked the strings against the scrub. `hi` and `pong` contain no prefix family, no `=`, and no `Authorization` header. `redact_sensitive_text` still runs on both. It allocates a new `String` and the characters are unchanged. That is why the expectation can stay `hi` and `pong`. Redaction is a no-op on those two words. The resume test uses `one`, `a`, `two`, and `b`, which miss the patterns the same way, and it expects four messages and one session row.

The third test calls `run_saved_evals` and `run_stress_evals`. Those functions drive the chapter 9 mock loop. They do not call `Store::open`. A pass there means the copied loop still passes.

The demo binary opens `ch20-sessions-demo.db` under the temp directory, calls `migrate`, appends session `"demo"` with user `"hi"` and `demo_reply()`, prints the returned count and `journal_mode`, drops the store, and deletes the file. Dropping the store closes the connection. The delete happens after that, so the remove is not racing an open handle in this binary.

## 20.3 The prefix patterns, compiled once

A session row is a place a key would live forever. I scrub at the write. `redact.rs` is that scrub. The module says the patterns are the prefix families read from `_PREFIX_PATTERNS`, plus env assignments, `Authorization` headers, and a PEM block. Plugin-registered patterns and the vault are not copied.

`MASK` is the three stars every replacement uses.

```rust
const MASK: &str = "***";
```

Four functions each hide one `Regex` in an `OnceLock`. `patterns` is the prefix alternation.

> **Rust sidebar: `OnceLock`**
>
> - `OnceLock<Regex>` is a cell that starts empty. `OnceLock::new()` is a constant, so it can sit in a `static`.
> - A `static` inside a function is still one value for the whole process. The next call does not build another cell. The name `RE` is local. The storage is not.
> - `get_or_init` runs the closure only on the first call. If two threads race, one runs the closure, the other waits, and both then receive the same `&Regex`. The patterns live for the process.
> - If the closure panics, the panic spreads and the cell stays empty, so a later call can try again. These closures only panic when `Regex::new` rejects the pattern. The pattern is a fixed string, so a bad one fails the first scrub in the process.
> - `patterns`, `env_assign`, `auth_header`, and `bearer_residue` each have their own lock. Calling `redact_sensitive_text` fills the first three. It does not fill the Bearer lock. `redact_for_egress` fills that fourth lock because it calls the Bearer sweep after the scrub.

```rust
fn patterns() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(concat!(
            r"sk-[A-Za-z0-9_-](?:\.?[A-Za-z0-9_-]){9,}",
            r"|ghp_[A-Za-z0-9]{10,}",
            r"|github_pat_[A-Za-z0-9_]{10,}",
            r"|gho_[A-Za-z0-9]{10,}",
            r"|ghu_[A-Za-z0-9]{10,}",
            r"|ghs_[A-Za-z0-9]{10,}",
            r"|ghr_[A-Za-z0-9]{10,}",
            r"|xox[baprs]-[A-Za-z0-9-]{10,}",
            r"|AIza[A-Za-z0-9_-]{30,}",
            r"|AKIA[A-Z0-9]{16}",
            r"|sk_live_[A-Za-z0-9]{10,}",
            r"|sk_test_[A-Za-z0-9]{10,}",
            r"|hf_[A-Za-z0-9]{10,}",
            r"|npm_[A-Za-z0-9]{10,}",
            r"|glpat-[A-Za-z0-9_\-]{10,}",
            r"|-----BEGIN [A-Z ]*PRIVATE KEY-----[\s\S]*?-----END [A-Z ]*PRIVATE KEY-----",
        ))
        .expect("redaction pattern")
    })
}
```

`concat!` joins those string literals at compile time into one pattern. Each piece after the first starts with `|`, which is regex alternation: try this, or that, or that. `Regex::new` compiles the combination the first time `patterns` is called. `expect("redaction pattern")` is the panic message if the compile fails. The function returns `&'static Regex`, a borrow that lasts as long as the process.

The replacement for every one of these matches is the same three lines, in `redact_sensitive_text`:

```rust
let raw = caps.get(0).map(|m| m.as_str()).unwrap_or("");
let prefix_len = raw.find(['-', '_']).map(|i| i + 1).unwrap_or(0);
format!("{}{MASK}", &raw[..prefix_len])
```

`caps.get(0)` is the whole match. `find(['-', '_'])` returns the byte index of the first `-` or the first `_`, whichever sits further left. The `map` adds one, so the prefix includes that separator. `unwrap_or(0)` is the branch where neither character exists: the prefix length is zero, and the whole match is replaced by `***`. `format!` builds a new `String`. The body of the key is not copied. A second call of the scrub sees the prefix and the stars, and that text is too short and too punctuated to match the same alternative again.

I walk the alternatives in the order `concat!` lists them.

`sk-[A-Za-z0-9_-](?:\.?[A-Za-z0-9_-]){9,}` is the OpenAI-style key. After `sk-` it wants one character from that class, then at least nine more groups of an optional dot and one class character. `sk-abcdefghijklmnopqrstuvwxyz` matches. The first separator is the hyphen in `sk-`, so the stored text is `sk-***`. `sk-abc` has only three characters after the hyphen. It does not match, and it is left as written. `sk-abc.def.ghij` does match, because the dot is allowed inside the repeated group, and the mask is still `sk-***`.

`ghp_[A-Za-z0-9]{10,}` is a classic GitHub personal token. Ten or more letters and digits after `ghp_`. The first separator is that underscore, so `ghp_` plus ten `a`s becomes `ghp_***`.

`github_pat_[A-Za-z0-9_]{10,}` is the fine-grained GitHub token. The mask does not keep `github_pat_`. `find` stops at the first underscore, the one after `github`, so `github_pat_` plus ten `a`s becomes `github_***`. The rest of the family name is inside the match, and the match is replaced as a whole.

`gho_`, `ghu_`, `ghs_`, and `ghr_` are the four GitHub token shapes that share one rule: the literal prefix, then at least ten letters or digits. OAuth, user-to-server, server-to-server, and refresh. Each mask keeps its own three letters and the underscore: `gho_***`, `ghu_***`, `ghs_***`, `ghr_***`. A token with only nine characters after the underscore does not match.

`xox[baprs]-[A-Za-z0-9-]{10,}` is the Slack family. The character class is the single letter `b`, `a`, `p`, `r`, or `s`, so the literals are `xoxb-`, `xoxa-`, `xoxp-`, `xoxr-`, and `xoxs-`. Ten or more letters, digits, or hyphens follow. The first separator is that hyphen, so `xoxb-` plus ten `a`s becomes `xoxb-***`. `xoxc-` is not in the class. It is left alone.

`AIza[A-Za-z0-9_-]{30,}` is the Google key. Thirty or more characters from that class. Twenty-nine does not match. The match has no `-` and no `_` when the body is only letters, and it has none in the literal `AIza` either. If a hyphen shows up inside the body, `find` stops there and the prefix includes that hyphen. A body of thirty `a`s has neither character, `prefix_len` is 0, and the whole match becomes `***`.

`AKIA[A-Z0-9]{16}` is the AWS access-key id. The `{16}` is exact, not "16 or more". Sixteen uppercase letters or digits after `AKIA` match. Fifteen do not. The literal and those sixteen characters contain no hyphen and no underscore, so the match becomes `***`. A seventeenth trailing character is outside the match. `replace_all` leaves it in the string. The stars replace only the twenty characters the pattern consumed.

`sk_live_[A-Za-z0-9]{10,}` and `sk_test_[A-Za-z0-9]{10,}` are the Stripe live and test keys. Both contain an underscore at index 2, before `live` or `test`. The mask keeps `sk_` and nothing more. `sk_live_` plus ten letters becomes `sk_***`. The same three characters replace a test key. A later reader cannot tell live from test by the stored prefix. They also cannot recover the key.

`hf_[A-Za-z0-9]{10,}` is the Hugging Face token. Ten letters or digits. The mask is `hf_***`.

`npm_[A-Za-z0-9]{10,}` is the npm token. The mask is `npm_***`.

`glpat-[A-Za-z0-9_\-]{10,}` is the GitLab token. The class allows letters, digits, underscore, and hyphen. In the raw string, `\-` is a hyphen the regex engine treats as a literal hyphen inside the class. Ten of those characters after `glpat-`. The first separator is the hyphen in `glpat-`, so the mask is `glpat-***`.

The last alternative is a PEM block.

```rust
r"|-----BEGIN [A-Z ]*PRIVATE KEY-----[\s\S]*?-----END [A-Z ]*PRIVATE KEY-----"
```

It starts at `-----BEGIN`, allows uppercase letters and spaces, requires the words `PRIVATE KEY`, and stops at the matching `-----END ... PRIVATE KEY-----` line. `[\s\S]*?` is any character, including newlines, and the `?` makes it stop at the first END line rather than the last. `-----BEGIN PRIVATE KEY-----` through `-----END PRIVATE KEY-----` matches, and so does a block whose label is `RSA PRIVATE KEY`. The first character of the match is `-`. `prefix_len` is 1. The entire block, body included, is replaced by `-***`.

`replace_all` walks non-overlapping matches of this whole alternation. Text that matches none of them is copied through. That is the path `hi` and `pong` take.

## 20.4 Env assignments, the auth header, and egress

The prefix pass is the first of three in `redact_sensitive_text`. The env assignment is the second.

```rust
fn env_assign() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)\b([A-Z0-9_]*(?:API_?KEY|TOKEN|SECRET|PASSWORD|PASSWD|CREDENTIAL)[A-Z0-9_]*)\s*=\s*(\S+)",
        )
        .expect("env pattern")
    })
}
```

`(?i)` makes the match case-insensitive. `\b` is a word boundary, so the name has to start at a boundary. Group 1 is the name. It may start and end with letters, digits, or underscores. The middle has to be one of `API_KEY`, `APIKEY` (the `_?` is an optional underscore), `TOKEN`, `SECRET`, `PASSWORD`, `PASSWD`, or `CREDENTIAL`. Under `(?i)`, a lowercase `password` matches `PASSWORD`. `NOT_A_NAME=zz` has none of those words, so it stays. `\s*=\s*` allows spaces around the equals sign. Group 2 is `\S+`, one or more characters that are not whitespace. The value stops at the first space. `MY_TOKEN=abc def` masks `abc` and leaves the space and `def`.

The replacement keeps group 1 and writes `=***`. Group 2 is discarded.

```rust
format!("{}={MASK}", caps.get(1).map(|m| m.as_str()).unwrap_or(""))
```

`OPENAI_API_KEY=super-secret-value` becomes `OPENAI_API_KEY=***`. `openai_api_key=super-secret-value` keeps the lowercase name and the same mask. `APIKEY=abc` and `API_KEY=abc` both match, because of `_?`. `PASSWD = x` matches across the spaces. The name is captured as the caller wrote it. I do not uppercase it.

The third pass is the `Authorization` header.

```rust
fn auth_header() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)((?:Proxy-)?Authorization:\s*)([A-Za-z][\w.+-]*\s+)?([^\s"']+)"#)
            .expect("auth header")
    })
}
```

> **Rust sidebar: a raw string**
>
> - A normal string `"..."` treats `\"` as a quote and `\n` as a newline.
> - A raw string `r"..."` does not. The backslash is just a backslash. `\"` does not escape inside `r"..."`. The quote ends the string, and the backslash sits in front of that end.
> - This pattern needs a real `"` inside the character class `[^\s"']`. I cannot write that with `r"..."` and a backslash.
> - `r#"..."#` is a raw string whose delimiter is `"#` instead of `"`. The quote in `[^\s"']` is then an ordinary character. Extra `#` characters on both sides are how you nest a raw string that itself contains `"#`.
> - `(?i)` is still the regex flag. The raw string does not change regex meaning. It only changes how Rust reads the source.

Group 1 is the header word, an optional `Proxy-` prefix, the colon, and the whitespace after it. Group 2 is optional. It is a scheme: a letter, then word characters or `.` `+` `-`, then whitespace. `Bearer ` matches. `Basic ` matches. Group 3 is the credential, one or more characters that are not whitespace and not a quote. The replacement is group 1 plus `***`. The scheme and the credential are dropped.

`Authorization: Bearer abcdefghijklmnopqrstuvwxyz012345` becomes `Authorization: ***`. The captured groups are the header word with its space, the word `Bearer`, and the token. Only the first group is written back. `Proxy-Authorization: Basic abcdef` becomes `Proxy-Authorization: ***`. A credential that starts with a quote does not match group 3, so `Authorization: "secret"` is unchanged by this pass. The class refuses the quote on purpose. I do not have a second pattern for a quoted header value.

`redact_sensitive_text` runs the three passes in that order and returns an owned `String`.

```rust
pub fn redact_sensitive_text(text: &str) -> String {
    let prefixed = patterns().replace_all(text, |caps: &regex::Captures| {
        let raw = caps.get(0).map(|m| m.as_str()).unwrap_or("");
        let prefix_len = raw.find(['-', '_']).map(|i| i + 1).unwrap_or(0);
        format!("{}{MASK}", &raw[..prefix_len])
    });
    let assigned = env_assign().replace_all(&prefixed, |caps: &regex::Captures| {
        format!("{}={MASK}", caps.get(1).map(|m| m.as_str()).unwrap_or(""))
    });
    auth_header()
        .replace_all(&assigned, |caps: &regex::Captures| {
            format!("{}{MASK}", caps.get(1).map(|m| m.as_str()).unwrap_or(""))
        })
        .into_owned()
}
```

`replace_all` returns a `Cow<str>`. A `Cow` is either a borrow of the input or an owned string that holds the replacements. `into_owned` turns either one into a `String`, which is what the signature promises. The env pass borrows the prefix result. The header pass borrows the env result. The last `into_owned` is the value the caller keeps. The input `&str` is never written in place. `append_turn` stores that new string.

Order matters when one token could hit two patterns. The prefix pass runs first. `Bearer sk-abcdefghijklmnopqrstuvwxyz` becomes `Bearer sk-***` before the header pass and before the env pass look at it. There is no `Authorization:` and no `=`, so those two passes leave it. The Bearer sweep later also leaves `Bearer sk-***`, because the residue pattern wants a bracketed blob or a token of at least twenty characters from its own class, and `sk-***` is neither. The key is already gone. The word `Bearer` remains.

The test string is `"openai sk-abcdefghijklmnopqrstuvwxyz and OPENAI_API_KEY=super-secret-value"`. The prefix pass turns the `sk-` key into `sk-***`. The env pass turns the assignment into `OPENAI_API_KEY=***`. The header pass sees nothing to do. The result contains `sk-***` and `OPENAI_API_KEY=***`, and it does not contain `super-secret-value`.

`redacting_formatter` is the log path. This crate does not take a logging facade. Call it on the line you are about to log.

```rust
pub fn redacting_formatter(line: &str) -> String {
    redact_sensitive_text(line)
}
```

It does not call the Bearer sweep. A bare `Bearer` token in a log line is scrubbed only if one of the three passes matches it. The test does not rely on that. It passes `Authorization: Bearer abcdefghijklmnopqrstuvwxyz012345` and expects `Authorization: ***`, with the token absent.

Egress is the stricter function. It scrubs, then it sweeps a leftover `Bearer`.

```rust
fn bearer_residue() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\bBearer\s+(?:\[[^\]]+\]|[A-Za-z0-9._~+/-]{20,}=*)").expect("bearer")
    })
}

pub fn redact_for_egress(text: &str) -> String {
    let scrubbed = redact_sensitive_text(text);
    bearer_residue()
        .replace_all(&scrubbed, "Bearer [redacted]")
        .into_owned()
}
```

The residue pattern is case-insensitive. `\bBearer\s+` is the word, then whitespace. The group after that is one of two shapes. `\[[^\]]+\]` is a bracketed residue: `Bearer [already-masked]` and `Bearer [redacted]` both match, and both are written back as `Bearer [redacted]`. The other shape is a token of at least twenty characters from letters, digits, and `._~+/-`, with optional `=` signs after it. `see Bearer abcdefghijklmnopqrstuvwxyz012345 now` becomes `see Bearer [redacted] now`. `Bearer short` is under twenty characters and is not in brackets, so the sweep leaves it.

The test calls `redact_for_egress` on a bare `Bearer abcdefghijklmnopqrstuvwxyz012345` with no `Authorization:` prefix. The header rule does not eat it first. The sensitive-text scrub leaves the bearer line alone, and the sweep produces `Bearer [redacted]`. The token characters are gone. If the same token had been written as `Authorization: Bearer ...`, the header pass would already have turned the line into `Authorization: ***`, and the sweep would find no `Bearer` left to replace.

`append_turn` stores the sensitive-text scrub. It does not call `redact_for_egress`. A bare Bearer token in a message is stored as itself unless a prefix, an env assignment, or an `Authorization` header also matches. The egress function is for text that is about to leave the process. Chapters 39, 47, and 49 call `redact_for_egress` instead of writing a narrower scrub.

`tests/redact.rs` is the new test. One function checks all three. `redact_sensitive_text` on the openai sentence must contain `sk-***` and `OPENAI_API_KEY=***` and must not contain `super-secret-value`. `redacting_formatter` on the Authorization line must contain `Authorization: ***` and must not contain the token. `redact_for_egress` on the bare Bearer sentence must contain `Bearer [redacted]` and must not contain the token. The three session tests are unchanged in what they expect to read back. `hi` and `pong` still round-trip. The journal mode is still `wal`. The trigram table still does not exist.

## Tests

Three tests. A turn appends two rows, the journal is `wal`, and the trigram table does not exist. A second turn resumes the same session. The chapter 9 saved evals and the four stress evals pass.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch20-sessions
```

```text
Starting 109 tests across 28 binaries
     Summary [   0.192s] 109 tests run: 109 passed, 0 skipped
```

```bash
cargo run -p ch20-sessions
```

```text
reply: pong
rows: 2 journal: wal
```

The binary is one fake reply in a temp database. The file is removed when the process exits.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `wal_append` | 139.2 µs | under 1 ms | one transaction, two message rows |

The reply is `pong`. Two rows. The journal is `wal`.

`wal_append` opens a temp database and runs `migrate` before the bench. The timed closure is `b.iter`.

The means are `mean.point_estimate` from [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `20`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The queue is not the index. A reader who `MATCH`es this file will fail, because the virtual table is not there yet. That is chapter 22.

`pending_fts` forgets ids past 64. A long bench, or a long session that never drains the queue, drops the oldest. Chapter 22 is what drains it. Do not treat the queue as a durable log.

`INSERT OR IGNORE` does not update `source` or `title` on a session that already exists. A later chapter that wants a title writes a separate update.

The eval test does not touch this store. A green eval run means the chapter 9 loop still passes. It does not mean the new tables were in that loop.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L8 is persistence. The memory and skill gates are not this chapter.

**E1 and E2 are this chapter.** `redact_sensitive_text` and `redacting_formatter` scrub stored and logged text. `redact_for_egress` does that scrub and then replaces a leftover `Bearer <token>` with `Bearer [redacted]`. `append_turn` stores the E1 scrub of both strings. The patterns are the prefix families read from `agent/redact.py` `_PREFIX_PATTERNS` (OpenAI-style `sk-`, GitHub `ghp_` / `github_pat_` / `gho_` / `ghu_` / `ghs_` / `ghr_`, Slack `xox[baprs]-`, Google `AIza`, AWS `AKIA`, Stripe `sk_live_` / `sk_test_`, Hugging Face `hf_`, npm `npm_`, GitLab `glpat-`), plus an env assignment whose name contains `API_KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `PASSWD`, or `CREDENTIAL`, an `Authorization` header, and a PEM block. Plugin-registered patterns and the vault are not copied. The mask keeps the prefix through the first `-` or `_` and then writes `***`, so a second pass does not see a new key. Chapters 39, 47, and 49 call `redact_for_egress` instead of writing a narrower scrub.

**What landed.** The session file is append-only for messages. A turn is one transaction. Search is not on the request path. The ids wait on a queue.

**What did not.** Subagent sessions are stored if you pass that source. Hiding them is chapter 22. Memory writes and the approval gate (A9) are chapter 21 and chapter 25. This chapter does not scan message text.

**Left for later.** Plugin-registered patterns and the vault. `append_turn` stores the E1 scrub and does not call `redact_for_egress`.

## Chapter summary

- **File.** `sessions` and `messages`. Journal mode `wal`.
- **Turn.** Two rows, one commit. The session row is inserted once.
- **Search.** Two ids queued. No virtual table.
- **Evals.** The chapter 9 mock set still passes.
- **Time.** One WAL append measured **139.2 µs**.

## What you implement

`my-harness/tests/ch20.rs` is the milestone. It compiles, then panics in `todo!("chapter 20")` until `chapter_20_stored_line("sk-abcdefghijklmnopqrstuvwxyz")` contains `sk-***` and does not contain the rest of that key. The store redacts before the insert. 

## What's next

Chapter 21 is `MEMORY.md` and `USER.md`. Entries are separated by a section mark. A write during the session does not change the snapshot already in the prompt. The background write is an enqueue.
