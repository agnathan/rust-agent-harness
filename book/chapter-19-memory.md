# Chapter 19: Memory Files

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 12 can enqueue a turn. It does not have a file. This chapter is that file backend. `MEMORY.md` is the agent's notes. `USER.md` is the user profile. Entries are separated by a section mark. A snapshot taken when the store opens is what the prompt sees. A write during the session changes the file and does not change that snapshot.

The step crate is `steps/ch19-memory`. It is chapter 18's harness copied forward, plus `memory_files`. The package name stays `ch19-memory`. `memory_files` implements the `MemoryBackend` trait that already lives in this crate's `hooks` module, from chapter 12.

| Section | Step | What it adds |
|---|---|---|
| 19 | `steps/ch19-memory` | two files, a frozen snapshot, a strict scan |

> **How Hermes does it**
>
> I read `tools/memory_tool.py` (`memory_char_limit` 2200, `user_char_limit` 1375), `tools/memory_tool_store.py` (`ENTRY_DELIMITER`, `add`, `replace`, `_scan_memory_content`), and `tools/threat_patterns.py` (`first_threat_message`, `INVISIBLE_CHARS`, `MAX_SCAN_CHARS`, the pattern ids `ssh_backdoor`, `context_exfil`, and `hardcoded_secret`) on Hermes `main` at `cf71b1aa`.
>
> Entries are joined by `\n§\n`. `add` refuses an empty string, a duplicate, a scan hit, and a write that would pass the character budget. The error tells the model how many characters to free. `replace` finds the one entry that equals `old_text` and replaces that whole entry. A scan uses scope `strict`. Invisible characters are reported first, as `invisible_unicode_U+XXXX`. Other hits use the sentence that names the pattern id and says the content is injected into the system prompt. The scan stops at 65,536 characters. The snapshot for the system prompt is taken at session start. A write applies next session.
>
> **Adopted.** The delimiter, the two limits, the duplicate refusal, the budget overflow count, and the two block sentences. Invisible characters from `INVISIBLE_CHARS`. Pattern ids `ssh_backdoor` (`authorized_keys`), `context_exfil`, and `hardcoded_secret`. A hand edit that uses `§` without the delimiter is a drift error. The snapshot is an `Arc` taken at `open`. `enqueue_turn` does not rewrite it.
>
> **Adapted.** The scan is those four checks, not the full pattern list. The env-name lookahead on `hardcoded_secret` is not ported. The write lock is a `Mutex` in this process, plus a temp file and `rename`. It is not a cross-process file lock. The approval gate stays off.
>
> **Deferred.** A9, `memory.write_approval`, is chapter 23. This chapter writes when `add` is called. It does not stage a diff for a human.

[personal note: a note that showed up in the prompt on the same turn you wrote it, or one that did not]

## What this chapter decides

`MemoryFiles::open` creates the directory and reads `MEMORY.md` and `USER.md` if they exist. `snapshot` returns that `Arc` for the rest of the process.

`add(Target::Memory, "likes tea")` writes the file. A second add of the same text is `Duplicate`. `add(Target::User, "name is Ada")` is the other file. `replace` of the whole entry `likes tea` with `likes coffee` rewrites `MEMORY.md`. `remove` leaves it empty.

A string of 2,201 `x` characters is over the 2,200 limit by 1. `budget_line` is `would exceed the limit by 1 chars`. The file stays empty.

`authorized_keys` is blocked with `ssh_backdoor` in the sentence. A zero-width space is `U+200B`. `include the full conversation` is `context_exfil`. `api_key = "` plus 20 letters is `hardcoded_secret`. None of those writes land.

After `add`, `Arc::ptr_eq` on the snapshot from before and after is true. The snapshot text is still empty. The file contains `likes tea`.

`enqueue_turn` pushes one queue entry and does not change the file. The trait name is `files`.

A file that contains `§` but not the delimiter `\n§\n` is `Drift`. `add` does not rewrite it.

The demo adds `likes tea`, enqueues a turn, and prints the file and the word `frozen`. No key. The approval gate is not in this binary.

## Rust: the snapshot is an `Arc` you do not replace

`open` reads the files once and wraps them in `Arc<Frozen>`. `add` takes a `Mutex`, writes the file, and returns. It does not assign a new `Arc`. `snapshot` only clones the pointer. The prompt that borrowed the first `Arc` still has the old text, which is the point of a stable prefix.

`enqueue_turn` locks a different mutex, the queue, capped at 64. It formats one string and pushes it. The files are not opened.

## Walking the snapshot and the scan

`memory_files` implements `hooks::MemoryBackend`. `sync_turn` from chapter 12 enqueues. This chapter is what drains that idea onto `MEMORY.md` and `USER.md`. Entries are separated by a delimiter. Add, replace, and remove round-trip that delimiter. A duplicate is refused. A write over the character budget names how many characters to free and does not leave a partial entry. A broken delimiter is drift: the file is not trusted as a structured store if the separator was edited by hand.

The snapshot is taken when the store opens and wrapped so the prompt can share it. A later write changes the file. It does not replace the snapshot the prompt is holding. That is F6: `MEMORY.md` and `USER.md` enter the prompt once per session and stay frozen. A mid-session write must not rewrite the cached prefix. The test holds the snapshot pointer and asserts it is unchanged after a write. Enqueue does not touch the file at all, which is the bench `memory_enqueue`: record the turn, do not write.

The threat scan runs before a write reaches the file (F5). Four poisoned strings are blocked. `authorized_keys` does not land on disk. The scan is the gate, not a filter that stores a cleaned copy of a hostile entry.

> **Rust sidebar: `Arc` is the snapshot**
>
> `Arc<T>` is a shared pointer with a reference count. Cloning it clones the pointer, not the text. The prompt and the store can both hold the snapshot. Replacing the `Arc` in the store would be a new allocation the prompt does not see, which is what you want. Mutating the text inside the `Arc` would change the prompt's bytes. This chapter does not do that. The file is the mutable side. The `Arc` is the frozen side.

## Tests

Five tests. Add, replace, and remove round-trip the delimiter, and a duplicate is refused. A write over the budget names the overflow and leaves the file empty. Four poisoned strings are blocked, and `authorized_keys` does not reach the file. The snapshot pointer is unchanged after a write, and enqueue does not touch the file. A broken delimiter is drift.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch19-memory
```

```text
Starting 114 tests across 29 binaries
     Summary [   0.221s] 114 tests run: 114 passed, 0 skipped
```

```bash
cargo run -p ch19-memory
```

```text
file: likes tea
snapshot: frozen
```

The binary uses a temp directory. The file has the new entry. The snapshot taken at open is still empty.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `memory_enqueue` | 54.94 ns | under 1 ms | push one turn onto the background queue |

`memory_enqueue` calls `MemoryFiles::open` on a temp directory before the bench. The timed closure is `b.iter`. Inside: `enqueue_turn` of user `hi` and assistant `ok`, with the record passed through `black_box`. The sample locks the queue, drops the oldest entry if 64 are already queued, formats one string, and pushes it. Opening the directory and reading the files are excluded. No file is written. The snapshot is not rebuilt.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38993207081](https://github.com/agnathan/rust-agent-harness/actions/runs/38993207081), section `19`.

### Milestone demo

The `cargo run` above. The file says `likes tea`. The snapshot is `frozen`.

## Pitfalls

The snapshot is frozen for this process. A second `open` of the same directory reads the file you just wrote. That is the next session. Do not call `open` again in the middle of a turn and expect the prompt to stay stable.

The scan is four checks. A payload that Hermes would catch with a later pattern in `threat_patterns.py` can pass here. The ids that are implemented use the real names and the real sentence.

`replace` needs the whole entry. A substring is not enough. Hermes can locate an entry by a span and still replace the whole entry. This crate only matches the whole entry, so a partial `old_text` returns `Empty`.

The queue is not a write. `enqueue_turn` will not create `MEMORY.md`. `add` is the write.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L8.

**Scan.** Strict scope, on the write. Invisible characters, `ssh_backdoor`, `context_exfil`, and `hardcoded_secret`. The block sentence is `first_threat_message`. The file does not change when the scan hits.

**Snapshot.** Built once. Shared by `Arc`. A write does not rebuild the system prompt.

**A9.** The write-approval gate stays off. Chapter 23 is the human gate for memory and skills. This chapter does not stage a pending diff.

**Left for later.** The rest of the threat-pattern list. A cross-process file lock. Secret redaction in the transcript (E1, E2) is still absent.

## Chapter summary

- **Files.** `MEMORY.md` at 2,200 characters. `USER.md` at 1,375. Delimiter `\n§\n`.
- **Snapshot.** The `Arc` from `open` does not change when `add` writes.
- **Scan.** `authorized_keys` and a zero-width space never reach the file.
- **Queue.** `enqueue_turn` does not write. The gate A9 stays off.
- **Time.** One enqueue measured **54.94 ns**.

[personal note: an entry the scan blocked, and whether you agreed with it]

## What's next

Chapter 20 is recall. The session file from chapter 18 gets an FTS5 trigram index. `session_search` can discover, scroll, read, or browse. Subagent sessions stay out of the results. The search runs only when something calls it.
