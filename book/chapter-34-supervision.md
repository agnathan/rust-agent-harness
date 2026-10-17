# Chapter 34: Supervision and Restart

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 33 can run a hundred agents. It does not start a failed one again, and it does not notice when the restarts themselves are the loop. This chapter restarts a failed child once. The decision is bookkeeping. The child's work is a separate call. Three chained restarts trip a breaker and skip auto-resume. A cancellation, or a budget of zero, does not start the child.

The step crate is `steps/ch30-supervision`. The book number is 34. The package is `ch30-supervision`.

| Section | Step | What it adds |
|---|---|---|
| 34 | `steps/ch30-supervision` | one restart, a chain of three, cancel and budget |

> **How Hermes does it**
>
> I read `gateway/restart_loop_guard.py` (`DEFAULT_MAX_RESTARTS`, `DEFAULT_WINDOW_SECONDS`, `DEFAULT_MAX_GAP_SECONDS`, `_chain_ending_at`, `check_and_record`, `_load_boots`) on Hermes `main` at `e31c674f`.
>
> `DEFAULT_MAX_RESTARTS` is 3. `DEFAULT_WINDOW_SECONDS` is 60. `DEFAULT_MAX_GAP_SECONDS` is 300. The gap that still links two boots is the max of those two, so it is 300 seconds. `check_and_record` returns true when the chain length reaches 3, and the caller skips auto-resume. `_load_boots` returns an empty list on `OSError`. The module docstring says an I/O failure fails open.
>
> **Adopted.** The three constants, the chain, the trip at 3, and fail-open when I/O has failed. A cancelled call and a zero budget do not start the child. Those two are already on the call from chapter 9. This crate checks them.
>
> **Adapted.** The log is a `Mutex<Vec<u64>>` plus an `AtomicU32`. There is no file. `now` is an argument, so a test does not sleep 300 seconds. The child is `FakeModel`. One failure retries once. The bench records one boot and does not call the child.
>
> **Deferred.** The persisted `restart_loop.json`, and a real child process. Chapter 35 places a server pool after this restart.

[personal note: a child that failed, and a restart that was only the decision to try again]

## What this chapter decides

`record` walks the boot stamps from newest to oldest. A gap wider than 300 seconds ends the chain. The new stamp is appended. The return is true when the chain's length is at least 3. `fail_io` makes `record` return false and leave the counter at 0.

`restart_child` checks the `AtomicBool` and the budget first. A failure calls `record`. If that trips, the error is `Breaker` and the child is not called again. Otherwise the child runs once more.

`demo` fails once, succeeds on the retry, and returns `restarted: ok`. No key.

## Rust: the atomic is the count

`restarts` is an `AtomicU32`. `fetch_add` is the increment. The stamps live in a mutex because the chain is a vec, and a counter alone cannot see a gap. `cancelled` is an `AtomicBool`. The bench takes the mutex once, on an empty vec.

## Tests

Six tests. A scripted failure then `ok` calls the child twice and records one restart. Stamps at 0, 10, and 20 trip. A second stamp 301 seconds later does not. Three records after `fail_io` do not trip and the counter stays 0. Cancel, and a budget of 0, call the child zero times. The third failure in one supervisor returns `Breaker` after one call, so the success line does not run. The demo is `restarted: ok`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch30-supervision
```

```text
Starting 6 tests across 3 binaries
     Summary [   0.007s] 6 tests run: 6 passed, 0 skipped
```

```bash
cargo run -p ch30-supervision
```

```text
restarted: ok
```

The binary retries one scripted child. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `restart_bookkeeping` | 32.47 ns | restart under 1 ms, excluding the child's work | record one boot on an empty supervisor |

`restart_bookkeeping` uses `iter_batched`. The setup closure calls `Supervisor::new` and is not timed. The timed routine calls `record(0)` with the timestamp passed through `black_box`, then `black_box` of the bool. The chain length is 1, so the breaker does not trip. `FakeModel` is not constructed. `restart_child` is not called. No sleep runs. The 300 second gap is a comparison against the argument `0`, not a wait.

The printed slope on this machine was `time: [32.336 ns 33.040 ns 33.444 ns]`. The table is `mean.point_estimate` from `target/criterion/restart_bookkeeping/new/estimates.json` (32.4736 ns), with the Benchmarks flags `--warm-up-time 1 --measurement-time 2 --sample-size 10`. The workflow mean on ubuntu-latest replaces this number when that run is green.

### Milestone demo

The `cargo run` above. The line is `restarted: ok`.

## Pitfalls

The bench is not the retry. A number that includes `complete` is the child's work, and the budget says to leave that out.

`record` on one supervisor is the chain. Three separate supervisors each recording once do not trip. The test that trips uses one supervisor.

Fail-open is a flag. This crate has no disk. A caller that wanted a missing file to trip the breaker will get the opposite: Hermes does not trip when the read fails, and neither does this.

The second `complete` runs only when the breaker did not trip. On the third failure the success script is never popped.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Restart loop (J6).** Three chained boots, with no gap wider than 300 seconds, skip auto-resume. The window of 60 seconds floors that gap. An I/O failure does not trip.

**Call.** A cancelled flag or a budget of zero returns before the child runs. The retry checks the flag again.

**Once.** A single failure retries one time. The breaker is what stops a sequence of failures, not a hidden third try inside one call.

**Left for later.** The file under the Hermes home, and a real process. E1 and E2 are still absent. The scripted error is not redacted.

## Chapter summary

- **Retry.** One failure, one more call, unless the breaker, the flag, or the budget says no.
- **Chain.** Gap 300 seconds. Length 3 trips. I/O failure stays open.
- **Atomics.** The restart count and the cancel flag.
- **Model.** `FakeModel` is a queue of results. The demo needs no key.
- **Time.** One record measured **32.47 ns**.

[personal note: a restart that was cheaper than the work it was restarting]

## What's next

Chapter 35 is the server track. A tool has a pool of backends. The pick is local: least outstanding, or two random choices.
