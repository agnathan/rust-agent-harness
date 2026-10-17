# Chapter 42: The Kanban Board

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 41 shares a fact inside one process. A crash takes the board with it, and a second process cannot see it. Hermes keeps a kanban card in SQLite so a worker in another process can claim it. This chapter is that card: one JSON file, one lock file, and a compare-and-swap on the status.

The step crate is `steps/ch36-kanban`. The book number is 42. The package is `ch36-kanban`.

| Section | Step | What it adds |
|---|---|---|
| 42 | `steps/ch36-kanban` | claim, complete, review, or block as one transaction |

> **How Hermes does it**
>
> I read `hermes_cli/kanban_db.py` (the module docstring, `claim_task`, `_claim_and_open_run`, `complete_task`, `request_review`, `block_task`) and `hermes_cli/kanban_workflow.py` (`DEFAULT_WORKFLOW`) on Hermes `main` at `e31c674f`.
>
> The module docstring says concurrency is WAL plus `BEGIN IMMEDIATE` plus a compare-and-swap on `tasks.status` and `claim_lock`. SQLite serializes writers. One claimer wins. Losers see zero rows. `_claim_and_open_run` updates `ready` (or `review`) to `running` only when `claim_lock` is null, and returns none when `rowcount` is not 1. `complete_task` refuses an empty result and refuses to close a live claim without proof of ownership. `request_review` moves `running` to `review`. `block_task` moves a card to `blocked`. `DEFAULT_WORKFLOW` names the columns `triage`, `todo`, `scheduled`, `ready`, `running`, `blocked`, `review`, and `done`.
>
> **Adopted.** A claim is one transaction that loses when the status is no longer `ready`. Complete, review, and block require the owner. An empty result is refused. A parent that is not `done` blocks the claim. Pause refuses a new claim.
>
> **Adapted.** The board is one JSON file. The transaction is a `create_new` lock file, not SQLite. The columns in this crate are `ready`, `running`, `review`, `blocked`, and `done`. The model runs after the lock file is removed.
>
> **Deferred.** WAL, worker processes, worktrees, and the pull-request acceptance gate. The catalog has no separate kanban id. N2 is the pause.

[personal note: a second process that had to lose]

## What this chapter decides

`transact` creates `board.lock` with `create_new`, reads `board.json`, applies one op, writes the file back, and deletes the lock in a `Drop`. The handle is closed before the delete so Windows can unlink it.

`Claim` requires status `ready`, no owner, parents `done`, and a board that is not paused. It sets `running` and the owner.

`Complete` requires `running`, the same owner, and a non-empty result. It sets `done` and clears the owner. Whitespace is `Empty`.

`Review` requires the owner and a non-empty summary. It sets `review` and clears the owner. `Block` does the same on the way to `blocked`.

A second claim of a `running` card returns `Lost`. Two processes are the binary `claim` subcommand. One prints `claimed`. The other prints `lost`.

`demo` uses a temp directory, claims `t1`, completes it, and prints `kanban: done`. No key. `FakeModel` runs only after `board.lock` is gone.

## Rust: create_new is the swap

`OpenOptions::create_new(true)` fails with `AlreadyExists` when the lock file is already there. That is the compare-and-swap. The loser sleeps 100 microseconds and retries, up to 2,000 times, then returns `Busy`. The status check inside the lock is the second half: even after the lock is acquired, a card that is not `ready` is `Lost`. The JSON write happens before the function returns, so a reader that runs after `transact` sees the new status. The bench does not start a second process.

## Tests

Seven tests. Claim then review, claim then block, and claim then complete each land on that status. An empty result is `Empty` and a stranger's complete is `Lost`; the card stays `running`. A child whose parent is not `done` is `Parents`. A paused board refuses a claim. Two processes spawned together produce one `claimed` and one `lost`, and the card is `running`. The lock file is absent before `FakeModel`. The demo is `kanban: done`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch36-kanban
```

```text
     Summary [   0.008s] 7 tests run: 7 passed, 0 skipped
```

```bash
cargo run -p ch36-kanban
```

```text
kanban: done
```

The binary writes a temp board, claims it, and completes it. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `claim_transaction` | 81.55 µs | one claim transaction under 5 ms | lock file, read, CAS, write |

`claim_transaction` uses `iter_batched`. The setup closure makes a fresh temp directory, writes one `ready` task `t1`, and is not timed. The timed routine calls `transact` with `Op::Claim` and owner `worker`, then deletes the directory. The `create_new`, the JSON read, the status swap, and the JSON write are inside the sample. The directory creation and the second process are not. `FakeModel` is not called. Complete, review, and block are the tests, not this sample.

The printed slope on this machine was `time: [80.600 µs 81.719 µs 82.635 µs]`. The table is `mean.point_estimate` from `target/criterion/claim_transaction/new/estimates.json` (81548.6431 ns), with the Benchmarks flags `--warm-up-time 1 --measurement-time 2 --sample-size 10`. The workflow mean on ubuntu-latest replaces this number when that run is green.

### Milestone demo

The `cargo run` above. The line is `kanban: done`.

## Pitfalls

The bench is one uncontended claim. A number that includes two processes and the 100 microsecond retry is the race test.

The lock file is removed in `Drop`. A crash between `create_new` and that drop leaves `board.lock` behind, and the next caller spins until `Busy`. This crate does not reclaim a stale lock. Hermes reclaims an expired `claim_lock`. That reclaim is not here.

Complete from a process that is not the owner is `Lost`. Clearing the owner on review is what lets a reviewer claim next. Leaving the owner set would stuck the card.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**One owner.** The status update matches `ready` and an empty owner, the same shape as `_claim_and_open_run` requiring `claim_lock IS NULL` and `rowcount == 1`. Two processes cannot both observe `running` under their own name.

**Empty completion.** `complete_task` raises when the evidence is empty. This crate returns `Empty` and does not write `done`.

**Parents.** A claim whose parent is not `done` returns `Parents`. Hermes demotes that card. This crate leaves it `ready` and refuses.

**Pause (N2).** A paused board refuses `Claim`. Complete of a card already `running` is a different op and is not refused by the pause check.

**Left for later.** The jobs file, worker pids, and the GitHub check gate. E1 and E2 are still absent.

## Chapter summary

- **File.** One JSON board and a lock file.
- **Verbs.** Claim, complete, review, block. Each is one transaction.
- **Race.** Two processes. One `claimed`, one `lost`.
- **Model.** `FakeModel` runs after the lock file is gone. The demo needs no key.
- **Time.** One claim measured **81.55 µs**.

[personal note: the lock file that had to disappear before the reply]

## What's next

Chapter 43 is the bus. Peers publish a small envelope. A slow subscriber hits a bounded channel.
