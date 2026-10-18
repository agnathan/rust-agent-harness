# Chapter 41: The Blackboard

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 40 ends a turn. It does not share a fact with a second agent. A supervisor still decides who runs. A blackboard is the other shape: the fact is the coordination, and a thin scheduler notices which waiter was waiting for it.

The step crate is `steps/ch41-blackboard`. The book number is 41. The package is `ch41-blackboard`.

| Section | Step | What it adds |
|---|---|---|
| 41 | `steps/ch41-blackboard` | one board, a condition, a claim that drops the lock before the model |

> **How Hermes does it**
>
> I read `hermes_cli/kanban_db_dispatch.py` (`dispatch_once`), `agent/estop.py` (`is_engaged`, `check_paused`), and `tools/threat_patterns.py` (the `ignore previous instructions` pattern) on Hermes `main` at `e31c674f`.
>
> Hermes does not ship a blackboard type. The dispatcher tick is the thin scheduler: `dispatch_once` holds the board lock for the tick, then the comment after the lock says the tick observer fires outside that section so a slow subscriber cannot stall the next tick. `is_engaged` is true when an ESTOP sentinel exists, and it fails safe on a stat error. `check_paused` is how cron, kanban, and new turns skip work. In-flight work is not killed. The threat pattern matches "ignore" plus a filler plus "previous|all|above|prior" plus "instructions".
>
> **Adopted.** The scheduler claims under the lock and returns before `FakeModel`. A paused board claims nothing. A fact that contains one of those four phrases is not stored.
>
> **Adapted.** The board is a `Mutex<HashMap>` and a `Condvar`, in this process. Waiters are records, not OS processes. The injection check is the four phrases with no filler words. A waiter stops after 8 wakes.
>
> **Deferred.** The durable compare-and-swap, and two processes, are chapter 42. SQLite WAL is not this chapter.

[personal note: the moment the lock had to end before the fake reply]

## What this chapter decides

`post` stores a key and a value, or returns `Threat` and stores nothing.

`tick` locks, and `claim_one` wakes the first waiter whose key is present and whose wake count is under 8. The fact is removed. The `MutexGuard` ends when `tick` returns.

`answer` calls `try_lock` before `FakeModel::complete`. If the scheduler still held the lock, that call would fail.

`pause` makes the next `tick` return `Paused`. `resume` allows a claim again. A value already returned can still be passed to the model. That is the ESTOP split: new work stops, the call already taken does not.

`demo` posts `notes` for the waiter `researcher`, claims it, and prints `blackboard: researcher ok`. No key.

## Rust: the guard is the lock

`tick` binds `let mut board = lock.lock()`. That binding is a `MutexGuard`. It unlocks when it goes out of scope, which is the end of `tick`, not the end of `answer`. The model is the next function. The condvar is the other half: `Shared::post` inserts under the same mutex and then `notify_all`. A waiter that already holds the guard sees the fact in the `while` check. A waiter inside `wait` is woken. Either way the model runs after `drop(guard)`.

## Tests

Seven tests. One claim wakes `researcher` and the second claim is `Idle`. `answer` runs while `try_lock` succeeds. A paused board returns `Paused` until `resume`. An injection is not stored. The ninth wake of the same waiter is `Idle` and the fact stays. A spawned thread waits on the condvar, claims, and only then calls `FakeModel`. The demo is `blackboard: researcher ok`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch41-blackboard
```

```text
     Summary [   0.006s] 7 tests run: 7 passed, 0 skipped
```

```bash
cargo run -p ch41-blackboard
```

```text
blackboard: researcher ok
```

The binary posts one fact and claims it. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `claim_with_waiters` | 4.97 µs | a claim under 1 ms with 100 waiters | lock, scan, remove the fact |

`claim_with_waiters` uses `iter_batched`. The setup closure builds 100 `Waiter` values, posts `notes` under the key `ready` on the last waiter, wraps the board in a `Mutex`, and is not timed. The timed routine calls `tick` with that mutex passed through `black_box`, then `black_box` of the `Claim`. The lock, the scan of 100 waiters, the wake increment, and the removal of the fact are inside the sample. The matching waiter is last, so the scan walks the other 99. Building the waiters is excluded. `FakeModel`, `answer`, `pause`, and the `Condvar` are not called. The 100 waiters are records, not 100 threads.

The table is `mean.point_estimate` from Benchmarks run [38991844877](https://github.com/agnathan/rust-agent-harness/actions/runs/38991844877) on ubuntu-latest (`bench-run.json`, 4973.257594361747 ns). The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `blackboard: researcher ok`.

## Pitfalls

The bench is the claim. A number that spawns 100 threads and waits on a condvar is a different program. The condvar test is the one that sleeps inside `wait`.

The lock has to end before the model. Holding it across `complete` would make every other waiter wait for a provider.

`post` of an injection returns `Threat` and leaves the map unchanged. A later claim cannot wake on a fact that was refused.

Eight wakes is the cap. The ninth post still stores the fact. `tick` leaves it there and returns `Idle`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Pause (N2).** `is_engaged` skips new work. `tick` on a paused board returns `Paused`. The model call that already received a `Claim::Woke` still runs. In-flight work is not cancelled here.

**Injection on the board (F1).** The four phrases from the prompt-injection pattern are refused at `post`. Filler words between "ignore" and "instructions" are not matched. That gap stays named.

**Wake cap.** A waiter stops after 8 claims. The board does not keep scheduling the same agent because the fact is still there. The fact stays on the ninth miss so a different waiter can be added later.

**Left for later.** Two processes and the durable card are chapter 42. E1 and E2 are still absent. The full threat scanner, with filler words, is not copied.

## Chapter summary

- **Board.** Facts and waiters behind one mutex.
- **Scheduler.** `tick` claims one waiter and drops the guard.
- **Model.** `FakeModel` runs after the claim. The demo needs no key.
- **Pause.** New claims stop. A claim already taken does not.
- **Time.** One claim with 100 waiters measured **4.97 µs**.

[personal note: a guard that ended in the right function]

## What's next

Chapter 42 puts the card on disk. Claim, complete, review, and block are one transaction, and two processes cannot hold the same task.
