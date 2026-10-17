# Chapter 25: Curation

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 24 can fork a reviewer. It does not decide that a skill has gone quiet. This chapter marks an agent-created skill stale after 14 days and archives it after 30. A pin, a cron reference, a bundled skill, and a hub skill stay where they are. The request path only enqueues the pass. The pass snapshots the tree first, then moves directories. An archive is a write, so the chapter 23 gate can hold it. An install is scanned with the chapter 23 rules.

The step crate is `steps/ch21-curation`. The book number is 25. The package is `ch21-curation`.

| Section | Step | What it adds |
|---|---|---|
| 25 | `steps/ch21-curation` | stale, then archived, a snapshot, the gate and the scan off the request path |

> **How Hermes does it**
>
> I read `agent/curator.py` (`apply_automatic_transitions`, `DEFAULT_STALE_AFTER_DAYS`, `DEFAULT_ARCHIVE_AFTER_DAYS`, `DEFAULT_INTERVAL_HOURS`, `DEFAULT_MIN_IDLE_HOURS`, `DEFAULT_CONSOLIDATE`), `agent/curator_backup.py` (`snapshot_skills`, `DEFAULT_KEEP`, `_EXCLUDE_TOP_LEVEL`), and `hermes_cli/config_defaults.py` (the `curator` block, including `prune_builtins`) on Hermes `main` at `8dcf6d5b`.
>
> `apply_automatic_transitions` walks curator-managed rows. Pinned skills are skipped. Cron-referenced skills are treated as pinned. Stale is 14 days. Archive is 30, and the move is into `skills/.archive/`, not a delete. `consolidate` defaults to false, so the inactivity pass does not call a model. `interval_hours` is `24 * 7`. `min_idle_hours` is 2. `prune_builtins` defaults to false. The comment says hub-installed skills are never pruned. `snapshot_skills` writes `skills.tar.gz` and keeps `DEFAULT_KEEP` (2) snapshots. `_EXCLUDE_TOP_LEVEL` includes `.curator_backups`, `.hub`, `.locks`, `.git`, `.archive`, and `.curator_ledger.jsonl`. The config comment says the snapshot runs before a consolidation pass, and the prune-only pass takes none.
>
> **Adopted.** 14 and 30 days, the pin and cron exemptions, bundled skills left in place, hub skills left in place, consolidate off, the seven-day interval, the two-hour idle, keep 2, and those excluded names.
>
> **Adapted.** This pass snapshots before the inactivity moves as well. Hermes takes that tarball only before consolidation. The clock is an `age_days` field on the record, so a test does not sleep. Staging is the in-memory label from chapter 23, not a pending JSON file.
>
> **Deferred.** The aux-model consolidation pass, the ledger, and rollback. Chapter 23's scan is what an install calls here. It is not widened.

[personal note: a skill that should have been archived, and one you pinned so it would not be]

## What this chapter decides

`Curator::due` is true when the agent has been idle at least 2 hours and the last pass was at least 168 hours ago. `enqueue` pushes `curate` onto a queue of 64. It does not call `pass`.

`pass` calls `snapshot`, then walks the records. A pinned or cron-referenced skill is skipped. `Origin::Hub` is skipped. `Origin::Bundled` is skipped because `PRUNE_BUILTINS` is false. An agent skill at 14 days becomes `Stale` and stays in its directory. At 30 days it becomes `Archived` and the directory moves to `.archive/<name>`. `consolidate` false leaves `consolidated` at 0 and does not rewrite `SKILL.md`.

`snapshot` writes `.curator_backups/NNNN-skills.tar.gz`. A path component in the exclude list is omitted, including `.git` and the backup directory itself. After the write, older files are deleted until two remain.

`install` runs `scan_text` and `should_allow_install`. A dangerous community skill is `CuratorError::Scan` even with `force`. When `set_write_approval(true)` is set, an archive and a safe install stage a label and do not move or create the directory.

`demo` writes `notes`, ages it 14 days, and returns `stale: 1`. No key.

## Rust: the clock is a number you pass in

`age_days` is an `i64` on `Skill`. The pass compares it with `STALE_AFTER_DAYS` and `ARCHIVE_AFTER_DAYS`. Nothing reads the system clock. A test builds a skill at 30 days and the archive happens in the same process.

The snapshot is `flate2`'s `GzEncoder` around `tar::Builder`. `append_path_with_name` stores the path relative to the skill root. `into_inner` gives the encoder back so `finish` can write the gzip trailer.

## Tests

Four tests. A 14-day skill is stale and still in place. A 30-day skill moves under `.archive`. A pin, a cron reference, a bundled skill, and a hub skill stay `Active`. Consolidate stays off and the body still says `Body.` The gate stages `archive old` and leaves the directory. A dangerous community install is `Scan` and creates nothing. A safe install while the gate is on is `Staged`. `enqueue` leaves `.archive` absent. `due` is false one hour short of idle or one hour short of the interval. Three snapshots leave the newest two. The tarball lists `notes/SKILL.md` and does not list `.git`. The demo line is `stale: 1`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch21-curation
```

```text
Starting 4 tests across 3 binaries
     Summary [   0.013s] 4 tests run: 4 passed, 0 skipped
```

```bash
cargo run -p ch21-curation
```

```text
stale: 1
```

The binary writes one skill, ages it 14 days, and runs the pass. No key. The enqueue the request path uses is the benchmark, not this print.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `enqueue_curation` | 29.05 ns | off the request path | record that a curator pass should run |

`enqueue_curation` opens a `Curator` on a temp directory before the bench. The timed closure is `b.iter`. Inside: `enqueue`, then `queue_len` passed through `black_box`. No snapshot is built. No skill is scanned or moved. The queue is capped at 64, so a long sample drops the oldest job.

The table is `mean.point_estimate` from Criterion on this machine, with the Benchmarks flags `--warm-up-time 1 --measurement-time 2 --sample-size 10`. The workflow mean on ubuntu-latest replaces this number when that run is green.

### Milestone demo

The `cargo run` above. One skill is stale. Nothing is archived.

## Pitfalls

Hermes does not snapshot a prune-only pass. This one does, because the pass moves directories and a move is harder to undo than a mark. If you expected the Hermes split, the extra tarball is the difference.

A bundled skill with `prune_builtins` false never ages out here. That flag is a constant, not a setter. Hub skills are never candidates.

`consolidate` true still does not call a model. The count stays 0. The aux-model pass is not in this crate.

The age is whatever the caller stored. A bad record of 30 days archives a skill that was used this morning.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**A9.** An archive and a skill install go through the same flag as chapter 23. Off, the move happens. On, the label is staged and the directory stays. The scan is not the gate. A dangerous install is refused before the gate is consulted.

**H1.** `install` calls `scan_text` and `should_allow_install`. `force` does not save a dangerous community skill. That call sits inside `install` and `pass`, which `enqueue` does not call.

**Off the request path.** `enqueue` is the only function the turn should call. Measured **29.05 ns**. The snapshot, the scan, and the rename run when something later drains the queue.

**Left for later.** E1 and E2 are still absent. The aux-model consolidation and rollback are not in this book yet.

## Chapter summary

- **Clock.** Stale at 14 days. Archive at 30. Idle 2 hours. Interval 7 days.
- **Exempt.** Pin, cron, bundled, hub.
- **Snapshot.** A gzip of the tree, two kept, `.git` left out. Taken before the moves.
- **Gate and scan.** Chapter 23's flag and chapter 23's install scan. Neither runs from `enqueue`.
- **Time.** Enqueue measured **29.05 ns**.

[personal note: a directory you were glad got a tarball before it moved]

## What's next

Chapter 26 runs four agents in order: outline, draft, critique, revise. The next stage receives the same `Arc`. It does not copy the transcript.
