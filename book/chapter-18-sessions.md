# Chapter 18: Sessions on Disk

*Draft. Bracketed notes like [personal note] are where my own story goes.*

The turns so far live in memory. A new process starts from nothing. This chapter appends a turn to SQLite in WAL mode and can read it back. The search index is a queue of row ids. The virtual table is chapter 20. The chapter 9 eval set runs again, on the mock, and it still passes.

The step crate is `steps/ch15-sessions`. The book number is 18. The package is `ch15-sessions`.

| Section | Step | What it adds |
|---|---|---|
| 18 | `steps/ch15-sessions` | WAL sessions, an append of one turn, a queued FTS id |

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

The demo writes `ch15-sessions-demo.db` under the temp directory, appends the fake reply `pong`, prints the row count and the journal mode, and deletes the file. No key.

The third test calls `run_saved_evals` and `run_stress_evals` from chapter 9. Both lists are all `passed`. That work is the mock loop. It does not open this database.

## Rust: one transaction, then the queue

`Connection::transaction` starts the write. Both inserts and the `INSERT OR IGNORE` share it. `commit` is the WAL append. A failure before `commit` leaves the file without that turn.

`last_insert_rowid` is the id SQLite just assigned. Those ids go on a `VecDeque`. At 64, the oldest is dropped. The queue is the background job. It is not an index.

`messages` prepares a `SELECT` and collects rows. The order is `id`, which is the append order.

## Tests

Three tests. A turn appends two rows, the journal is `wal`, and the trigram table does not exist. A second turn resumes the same session. The chapter 9 saved evals and the four stress evals pass.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch15-sessions
```

```text
Starting 3 tests across 3 binaries
     Summary [   0.027s] 3 tests run: 3 passed, 0 skipped
```

```bash
cargo run -p ch15-sessions
```

```text
reply: pong
rows: 2 journal: wal
```

The binary is one fake reply in a temp database. The file is removed when the process exits.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `wal_append` | 143.9 µs | under 1 ms | one transaction, two message rows |

`wal_append` opens a temp database and runs `migrate` before the bench. The timed closure is `b.iter`. Inside: `append_turn` of session `bench`, user `hi`, assistant `pong`, with each argument passed through `black_box`. The sample is the transaction and the queue push. Opening the file and creating the tables are excluded. No FTS insert runs. The queue is capped at 64, so later samples drop the oldest id instead of growing without a bound.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38967738024](https://github.com/agnathan/rust-agent-harness/actions/runs/38967738024), section `18`.

### Milestone demo

The `cargo run` above. The reply is `pong`. Two rows. The journal is `wal`.

## Pitfalls

The queue is not the index. A reader who `MATCH`es this file will fail, because the virtual table is not there yet. That is chapter 20.

`pending_fts` forgets ids past 64. A long bench, or a long session that never drains the queue, drops the oldest. Chapter 20 is what drains it. Do not treat the queue as a durable log.

`INSERT OR IGNORE` does not update `source` or `title` on a session that already exists. A later chapter that wants a title writes a separate update.

The eval test does not touch this store. A green eval run means the chapter 9 loop still passes. It does not mean the new tables were in that loop.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L8 is persistence. The memory and skill gates are not this chapter.

**What landed.** The session file is append-only for messages. A turn is one transaction. Search is not on the request path. The ids wait on a queue.

**What did not.** Subagent sessions are stored if you pass that source. Hiding them is chapter 20. Memory writes and the approval gate (A9) are chapter 19 and chapter 23. This chapter does not scan message text.

**Left for later.** Secret redaction (E1, E2) is still absent.

## Chapter summary

- **File.** `sessions` and `messages`. Journal mode `wal`.
- **Turn.** Two rows, one commit. The session row is inserted once.
- **Search.** Two ids queued. No virtual table.
- **Evals.** The chapter 9 mock set still passes.
- **Time.** One WAL append measured **143.9 µs**.

[personal note: a resume that came back empty, and which file you had opened]

## What's next

Chapter 19 is `MEMORY.md` and `USER.md`. Entries are separated by a section mark. A write during the session does not change the snapshot already in the prompt. The background write is an enqueue.
