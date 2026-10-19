# Chapter 33: Composing Patterns

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 32 can accept or reject one reply. It does not run the next turn, and it does not decide whether the work was a tool call or a change of owner. This chapter nests the patterns. The outer loop is a supervisor. An inner fan-out uses that same pool. A handoff runs only when ownership should move. The critic loops, a wait names a target, and a quality gate can pause the loop before the judge is asked.

The step crate is `steps/ch33-composition`. The package is `ch33-composition`. It depends on `ch32-critic` only. That crate's library is `harness_core`, and the copied modules `supervisor`, `handoffs`, `fanout`, and `critic` are already in it. Four path dependencies would be four crates with the same library name.

| Section | Step | What it adds |
|---|---|---|
| 33 | `steps/ch33-composition` | nested fan-out under one pool, a handoff only when ownership moves, the critic loop |

> **How Hermes does it**
>
> I read `hermes_cli/goals.py` (`DEFAULT_MAX_TURNS`, `DEFAULT_JUDGE_TIMEOUT`, `DEFAULT_GATE_TIMEOUT_SECONDS`, `DEFAULT_GATE_MAX_RETRIES`, `_parse_judge_response`, `CONTINUATION_PROMPT_GATE_FAILED_TEMPLATE`, `_check_gates`) and `tools/delegate_tool_config.py` (`MAX_DEPTH`, `_DEFAULT_MAX_CONCURRENT_CHILDREN`) on Hermes `main` at `e31c674f`.
>
> `DEFAULT_MAX_TURNS` is 20. `DEFAULT_JUDGE_TIMEOUT` is 30 seconds. `DEFAULT_GATE_TIMEOUT_SECONDS` is 300. `DEFAULT_GATE_MAX_RETRIES` is 3. `_parse_judge_response` prefers `wait_on_session`, then `wait_on_pid`, then `wait_for_seconds`. A `wait` with none of those is downgraded to `continue`. A failing quality gate does not call the judge. After `attempts` passes `max_retries`, the goal pauses. `MAX_DEPTH` is 1. The default concurrent-children cap is 10. Deeper spawn is `delegation.max_spawn_depth`, floored at 1.
>
> **Adopted.** The four limits, the wait-target order, the downgrade of a wait that names nothing, and a gate that is checked before the judge. `done` is still the only accept, from chapter 32.
>
> **Adapted.** Nested fan-out is one pool of 8, not a deeper delegate tree and not Hermes's cap of 10. `held + width` past that limit is refused before any task is scheduled. A handoff runs only for `Route::Move`. The gate's exit codes are a script. This crate does not spawn a shell and does not sleep the 30 second or 300 second timeouts. `FakeModel` returns the next scripted line. No key.
>
> **Deferred.** The continuation prompt text (`CONTINUATION_PROMPT_GATE_FAILED_TEMPLATE`) is not assembled into the next user message. A live shell for the gate, and a real pid or session that the wait parks on, stay out. Hermes's `max_spawn_depth` above 1 is not a second pool.

[personal note: a team that kept handing the job sideways when it only needed two specialists at once]

## What this chapter decides

`nested_fanout` takes the permits the parent already holds and the width it wants. If `held + width` is greater than `pool.limit()`, the error is `ExceedsBound` and nothing is scheduled. Otherwise the tasks go on a chapter 31 board and `gather` runs them on the chapter 29 pool. `nested_fanout_holding` is the same path with a sleep inside each worker, so a test can see `max_seen` reach 8. The bench uses a zero hold.

`dispatch` with `Route::Stay` returns `None` and does not touch the handoff board. `Route::Move` calls chapter 30's `handoff`, which still drops `scratch` and still stops at four.

`run_continuation` reads one line from `FakeModel`. Then the gates run. A non-zero exit increments `attempts` and, while that is at most 3, the loop continues without `as_tool`. The fourth failure is `GatePaused`. When the gates are clear, chapter 32's `as_tool` judges the line. `Wait` with a session, a pid, or a number of seconds returns `Parked` and does not burn a turn. `Wait` with no target burns a turn and continues. Twenty continues return `Turns`. A missing verdict is still `Reject`.

`limits` returns `(20, 30, 300, 3)`. Nothing in this crate sleeps.

`demo` fans out two children, stays with the supervisor, judges `verdict: done`, and returns `fan n0 | n1; owner supervisor; done`.

## Rust: the check before the threads

`saturating_add` is the whole guard. The pool's `acquire` waits when it is full. A nested request that asks for more than the limit must not wait. It returns before `schedule`. `max_seen` stays 0.

`FakeModel` is a `VecDeque` of strings. `complete` pops one. An empty deque returns `""`. That empty line is what the critic rejects.

## Tests

Eight tests. Width 9 on a pool of 8 is `ExceedsBound`, and so is holding 1 and asking for 8. `max_seen` stays 0. Eight children with a 40 ms hold finish as `n0` through `n7` and `max_seen` is 8. The same eight with no hold return the same summary and do not pass the bound. `Stay` leaves the owner at `supervisor`. `Move` drops the scratch message and the owner becomes `billing`. A continue then a done accepts. A wait on session `sess-9` parks. Pid 44 and 12 seconds parse. A wait with no target is not a target, and the next line can still be done. A gate that fails once and then exits 0 accepts on the second line, and the judge ran once. Four failures pause on `cargo test` and the fresh pool's `max_seen` is 0. Twenty continues stop the loop. `looks finished` is `Reject`. The demo line matches.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch33-composition
```

```text
Starting 8 tests across 3 binaries
     Summary [   0.049s] 8 tests run: 8 passed, 0 skipped
```

```bash
cargo run -p ch33-composition
```

```text
fan n0 | n1; owner supervisor; done
```

The binary fans out two scripted children, does not hand off, and accepts one scripted verdict. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `nested_eight` | 270.8 µs | nested fan-out stays inside the worker limit | schedule and gather 8 children on one pool |

`nested_eight` builds `Pool::new(8)` before the bench. The timed closure is `b.iter`. Inside: `nested_fanout` with `held` 0 and width 8, both passed through `black_box`, then `black_box` of the summary. The bound check passes. Eight tasks are scheduled and gathered. Each worker takes one permit, copies a short reply, and drops the permit. The hold is zero, so the sample does not sleep. No model is called. The 30 second judge timeout and the 300 second gate timeout are not slept. Width 9 is not this sample. That refusal is the test, and it does not start threads.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `31`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `fan n0 | n1; owner supervisor; done`.

## Pitfalls

Hermes nests by raising `max_spawn_depth`. This crate does not. A child that wants its own pool of 8 while the parent still holds a permit is refused. That is the bound. It is not a second team.

`Route::Stay` is the default shape from the team-patterns page. `Route::Move` is the handoff. Calling `Move` on every specialist turns the supervisor back into chapter 30.

A failing gate consumes the worker's line and does not judge it. The script has to put the `verdict: done` line after the red exit, or the judge never sees it.

`Wait` parks in the return value. It does not sleep, and it does not look up a pid. A caller that expected the process to block for `wait_for_seconds` will return immediately.

The gate command is a string. Exit codes come from the script. `cargo test` in the test is not executed.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Bound (M6).** Nested fan-out cannot exceed the worker limit of 8. Hermes's default is depth 1 and 10 concurrent children. This chapter keeps the harness pool. A request past the limit is an error, not a queue.

**Gate.** A non-zero exit skips the judge. After 3 retries the next failure pauses. The 300 second gate timeout is named and not slept. The shell is not spawned.

**Wait.** A session, a pid, or a number of seconds parks the loop and does not burn a turn. A wait with no target continues. The 30 second judge timeout is not slept.

**Turns.** Twenty continues stop the loop. `done` is still the only accept. A missing verdict is `Reject`.

**Left for later.** The continuation prompt text, a live shell, and a real wait on a pid or a session. E1 and E2 are still absent. The scripted lines are not redacted.

## Chapter summary

- **Nesting.** One pool of 8. `held + width` past 8 is refused before `schedule`.
- **Handoff.** Only `Route::Move`. `Stay` leaves the owner where it was.
- **Loop.** Gates, then the critic. Wait with a target parks. Twenty continues stop.
- **Model.** `FakeModel` is a queue of strings. The demo does not need a key.
- **Time.** Eight nested children measured **270.8 µs**.

[personal note: a bound you wanted to hit in the test, not in a run]

## What's next

Chapter 34 manages context. Compression, a memory flush, and the scan of files such as `AGENTS.md` sit in front of the model call. The hot path still sends the snapshot it already built.
