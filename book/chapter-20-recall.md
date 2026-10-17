# Chapter 20: Recall

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 18 stores the messages and queues their ids. It cannot search them. This chapter builds an FTS5 trigram index on that file and adds `session_search`. The tool returns rows from the database. It does not call a model. A subagent session stays out of the results. A cron session sorts after the user's own sessions.

The step crate is `steps/ch17-recall`. The book number is 20. The package is `ch17-recall`. It uses the store from `steps/ch15-sessions`.

| Section | Step | What it adds |
|---|---|---|
| 20 | `steps/ch17-recall` | trigram FTS, discover, scroll, read, browse |

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
> **Deferred.** CJK tokenizer, the rank explanation Hermes returns to the model, and compaction summaries staying out of a read. The queue from chapter 18 is not what feeds this index. `install_fts` rebuilds from `messages`.

[personal note: a search that returned a cron log instead of the conversation you meant]

## What this chapter decides

`install_fts` creates `messages_fts_trigram` if it is missing and rebuilds it from `messages`.

A discover query for `zephyrtoken` hits the user session `desk` first and the cron session `nightly` second. The subagent session `child` has the same token and is not in the list.

Scroll with `session_id` `desk` and `around_message_id` set to the first row returns `alpha` and `beta`. Read of `desk` returns both rows. Read of `child` returns nothing. Browse with no arguments lists `desk` before `nightly` and does not list `child`.

The demo builds a temp database, indexes it, and prints `hit: desk`. No key. No model.

## Rust: the index is a virtual table

`CREATE VIRTUAL TABLE ... USING fts5` does not store a second copy when `content='messages'`. The rowid is `messages.id`. `MATCH` reads the index and the join reads the message text back from `messages`.

`INSERT INTO messages_fts_trigram(messages_fts_trigram) VALUES('rebuild')` is the FTS5 rebuild command. It runs once, before any search in the tests and before the bench samples. A later `append_turn` from chapter 18 does not update this index until something rebuilds again. That is the gap the triggers close in Hermes. This chapter rebuilds on purpose.

The hidden check is a `NOT IN` on `sessions.source`. Scroll and read use the same list, so naming a subagent id returns no rows.

## Tests

Three tests. Discover hides the subagent and lists the user session before cron. Scroll, read, and browse are the other three shapes, and a hidden session reads as empty. The demo hit is `desk`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch17-recall
```

```text
Starting 3 tests across 3 binaries
     Summary [   0.010s] 3 tests run: 3 passed, 0 skipped
```

```bash
cargo run -p ch17-recall
```

```text
mode: discover
hit: desk
```

The binary is one temp database. A child session and a user session both contain `zephyrtoken`. The hit is the user session.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `discover_10k` | 59.35 µs | under 20 ms | one trigram query over 10,000 messages |

`discover_10k` opens a temp database, migrates, and inserts 10,000 messages into session `desk` before the bench. Message 5,000 contains `zephyrtoken`. The other 9,999 are `note N`. `install_fts` rebuilds the trigram index before the bench. The timed closure is `b.iter`. Inside: `session_search` with the query `zephyrtoken`, no session id, and no anchor, both the store and the query passed through `black_box`. The inserts and the rebuild are excluded. No model is called. The result is one hit in `desk`.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38993207081](https://github.com/agnathan/rust-agent-harness/actions/runs/38993207081), section `20`.

### Milestone demo

The `cargo run` above. The mode is `discover`. The hit is `desk`.

## Pitfalls

A message inserted after `install_fts` is not in the index until the next rebuild. Chapter 18's queue is not drained here. If you append and search in the same process without rebuilding, the new row is missing.

Cron is searchable. It sorts after a user session that also matched. A query that only the cron session contains will still return that session. Hiding cron would drop a log you might have wanted. Demoting it is the choice this chapter made.

The scroll window is ids, not "five messages of text." If ids have gaps, the window is still `anchor - 5` through `anchor + 5`.

Trigram `MATCH` wants at least three characters. A one-letter query is the wrong tool.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Recall is persistence, layer L8, and it is not a write.

**What landed.** Search reads the session file. It does not call a model. Subagent, kanban, and tool sessions are hidden, which is `_HIDDEN_SESSION_SOURCES`. Cron and worker sessions sort after the others so a repeated automation log does not take the first row when the user's session also matched.

**What did not.** The index is not updated on every insert. The full trigger set in `FTS_TRIGRAM_SQL` is not ported. Cron is not excluded from the trigram table the way `FTS_TRIGRAM_EXCLUDED_SOURCES` excludes it.

**Left for later.** Secret redaction (E1, E2) is still absent. A poisoned memory file is chapter 19's scan, not this search.

## Chapter summary

- **Index.** `messages_fts_trigram`, trigram tokenizer, rebuilt from `messages`.
- **Discover.** One hit per session. `desk` before `nightly`. `child` is absent.
- **Other shapes.** Scroll is a window of 5. Read is the session. Browse is recent sessions, with cron after the user.
- **Time.** One discover over 10,000 messages measured **59.35 µs**.

[personal note: a past session the search found, or one it hid]

## What's next

Chapter 21 is the server again: learned tools. The harness side of part II continues with skills. The session file and the memory files are what those later chapters read.
