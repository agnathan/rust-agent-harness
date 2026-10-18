# Chapter 18: Sessions on Disk

*Draft. Bracketed notes like [personal note] are where my own story goes.*

The turns so far live in memory. A new process starts from nothing. This chapter appends a turn to SQLite in WAL mode and can read it back. The search index is a queue of row ids. The virtual table is chapter 20. The chapter 9 eval set runs again, on the mock, and it still passes.

The step crate is `steps/ch18-sessions`. It is chapter 17's harness copied forward, plus `wal` and `redact`. The package name stays `ch18-sessions`. The module is `wal` because `session.rs` is already the MCP session.

| Section | Step | What it adds |
|---|---|---|
| 18 | `steps/ch18-sessions` | WAL sessions, an append of one turn, a queued FTS id |

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
> **Deferred.** Trigram search, hiding subagent sessions, and demoting cron are chapter 20. The triggers in `FTS_TRIGRAM_SQL` land there, in a smaller form.

[personal note: a session you expected to resume, and what was actually on disk]

## What this chapter decides

`Store::open` opens a file. `migrate` sets WAL and creates the two tables if they are missing.

`journal_mode` is `wal`.

`append_turn("s1", "hi", "pong")` returns 2. `messages("s1")` is the user row and then the assistant row. `pending_fts` is 2. `sqlite_master` has no `messages_fts_trigram`.

A second `append_turn` on `s1` makes four messages and still one session row.

The demo writes `ch18-sessions-demo.db` under the temp directory, appends the fake reply `pong`, prints the row count and the journal mode, and deletes the file. No key.

The third test calls `run_saved_evals` and `run_stress_evals` from chapter 9. Both lists are all `passed`. That work is the mock loop. It does not open this database.

## Rust: one transaction, then the queue

`Connection::transaction` starts the write. Both inserts and the `INSERT OR IGNORE` share it. `commit` is the WAL append. A failure before `commit` leaves the file without that turn.

`last_insert_rowid` is the id SQLite just assigned. Those ids go on a `VecDeque`. At 64, the oldest is dropped. The queue is the background job. It is not an index.

`messages` prepares a `SELECT` and collects rows. The order is `id`, which is the append order.

## Walking `append_turn` and `redact_sensitive_text`

`wal::Store::append_turn` inserts the user text and the assistant text and queues the row ids for FTS. It does not create the virtual table. Chapter 20 does. Before the insert, both strings pass through `crate::redact::redact_sensitive_text`. A session row is a place a key would live forever, so the scrub happens at the write, not at the search.

`redact_sensitive_text` builds three `Regex` values behind `OnceLock`. The first call compiles them. Later calls borrow the same compiled pattern. The prefix alternation is the families actually read from Hermes `_PREFIX_PATTERNS`: `sk-`, `ghp_`, `github_pat_`, `gho_`, `ghu_`, `ghs_`, `ghr_`, `xox[baprs]-`, `AIza`, `AKIA`, `sk_live_`, `sk_test_`, `hf_`, `npm_`, `glpat-`, and a PEM block. A match is replaced by the prefix through the first `-` or `_`, then `***`. `sk-abcdefghijklmnopqrstuvwxyz` becomes `sk-***`.

The env-assignment pattern is case-insensitive and matches a name containing `API_KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `PASSWD`, or `CREDENTIAL`, then `=`, then a non-space value. The replacement keeps the name and writes `=***`. `OPENAI_API_KEY=super-secret-value` loses the value. The `Authorization` header pattern keeps the header word and masks the credential. `redacting_formatter` is `redact_sensitive_text` under a function name, because this crate does not take a logging facade. Call it on the line you are about to log. Hermes's `RedactingFormatter.format` does the same scrub inside the logging stack.

`redact_for_egress` runs `redact_sensitive_text` and then a Bearer sweep. A leftover `Bearer` plus a long token, or a `Bearer [already-masked]` residue, becomes `Bearer [redacted]`. The test uses a bare `Bearer abcdefghijklmnopqrstuvwxyz012345` with no `Authorization:` prefix, so the header rule does not eat it first, and asserts the egress form.

`tests/redact.rs` is the new test. The three session tests still expect the words `hi` and `pong`, which do not match any pattern, so redaction is a no-op on them. The journal mode is still `wal`. The trigram table still does not exist.

> **Rust sidebar: `OnceLock` and a raw string**
>
> `OnceLock<Regex>` is a cell that starts empty and is filled once. `get_or_init` runs the closure only on the first call, even if two threads race: one wins, the other waits and then sees the value. The patterns live for the process.
>
> A Rust raw string `r"..."` does not treat `\"` as an escaped quote. The auth-header pattern is written `r#"(?i)...([^\s"']+)"#` so the quote can sit inside the pattern. The `#` delimiters are how you put a `"` in a raw string.

## Tests

Three tests. A turn appends two rows, the journal is `wal`, and the trigram table does not exist. A second turn resumes the same session. The chapter 9 saved evals and the four stress evals pass.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch18-sessions
```

```text
Starting 109 tests across 28 binaries
     Summary [   0.192s] 109 tests run: 109 passed, 0 skipped
```

```bash
cargo run -p ch18-sessions
```

```text
reply: pong
rows: 2 journal: wal
```

The binary is one fake reply in a temp database. The file is removed when the process exits.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `wal_append` | 142.9 µs | under 1 ms | one transaction, two message rows |

`wal_append` opens a temp database and runs `migrate` before the bench. The timed closure is `b.iter`. Inside: `append_turn` of session `bench`, user `hi`, assistant `pong`, with each argument passed through `black_box`. The sample is the transaction and the queue push. Opening the file and creating the tables are excluded. No FTS insert runs. The queue is capped at 64, so later samples drop the oldest id instead of growing without a bound.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38993207081](https://github.com/agnathan/rust-agent-harness/actions/runs/38993207081), section `18`.

### Milestone demo

The `cargo run` above. The reply is `pong`. Two rows. The journal is `wal`.

## Pitfalls

The queue is not the index. A reader who `MATCH`es this file will fail, because the virtual table is not there yet. That is chapter 20.

`pending_fts` forgets ids past 64. A long bench, or a long session that never drains the queue, drops the oldest. Chapter 20 is what drains it. Do not treat the queue as a durable log.

`INSERT OR IGNORE` does not update `source` or `title` on a session that already exists. A later chapter that wants a title writes a separate update.

The eval test does not touch this store. A green eval run means the chapter 9 loop still passes. It does not mean the new tables were in that loop.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L8 is persistence. The memory and skill gates are not this chapter.

**E1 and E2 are this chapter.** `redact_sensitive_text` and `redacting_formatter` scrub stored and logged text. `redact_for_egress` does that scrub and then replaces a leftover `Bearer <token>` with `Bearer [redacted]`. `append_turn` stores the E1 scrub of both strings. The patterns are the prefix families read from `agent/redact.py` `_PREFIX_PATTERNS` (OpenAI-style `sk-`, GitHub `ghp_` / `github_pat_` / `gho_` / `ghu_` / `ghs_` / `ghr_`, Slack `xox[baprs]-`, Google `AIza`, AWS `AKIA`, Stripe `sk_live_` / `sk_test_`, Hugging Face `hf_`, npm `npm_`, GitLab `glpat-`), plus an env assignment whose name contains `API_KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `PASSWD`, or `CREDENTIAL`, an `Authorization` header, and a PEM block. Plugin-registered patterns and the vault are not copied. The mask keeps the prefix through the first `-` or `_` and then writes `***`, so a second pass does not see a new key. Chapters 37, 43, and 45 call `redact_for_egress` instead of writing a narrower scrub.

**What landed.** The session file is append-only for messages. A turn is one transaction. Search is not on the request path. The ids wait on a queue.

**What did not.** Subagent sessions are stored if you pass that source. Hiding them is chapter 20. Memory writes and the approval gate (A9) are chapter 19 and chapter 23. This chapter does not scan message text.

**Left for later.** Secret redaction (E1, E2) is still absent.

## Chapter summary

- **File.** `sessions` and `messages`. Journal mode `wal`.
- **Turn.** Two rows, one commit. The session row is inserted once.
- **Search.** Two ids queued. No virtual table.
- **Evals.** The chapter 9 mock set still passes.
- **Time.** One WAL append measured **142.9 µs**.

[personal note: a resume that came back empty, and which file you had opened]

## What's next

Chapter 19 is `MEMORY.md` and `USER.md`. Entries are separated by a section mark. A write during the session does not change the snapshot already in the prompt. The background write is an enqueue.
