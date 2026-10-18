# Chapter 10: The Turn State Machine

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 9 returns one `TurnOutcome` when a turn ends. The loop that produces it is a nest of checks. This chapter pulls the legal moves out into a table, and then makes that same loop walk the table. The step is not a second demo crate. It is chapter 9's harness (`steps/ch09-10-evals`) copied here, plus `turn`.

The step crate is `steps/ch10-turn-state`. The directory name stays `ch10-turn-state` so it matches the CI filter. The book number is 10. The package is `ch10-turn-state`, library name `harness_core`, under `crates/harness-core`. The CLI package is `ch10-harness-cli`.

| Section | Step | What it adds |
|---|---|---|
| 10 | `steps/ch10-turn-state` | `Phase`, `TurnEvent`, and `step` |

> **How Hermes does it**
>
> I read `gateway/session_state.py` and `agent/conversation_loop.py` on current Hermes `main`. `TurnState` there is a dataclass of pointers for one gateway turn: the running agent, the event, the context, a lease, timestamps. `clear` sets the pointers back to empty. It is not a table of moves. `_LoopState` in `conversation_loop.py` is the bag of locals the phase helpers in `agent/turn_*.py` pass by name. `run_conversation` is still the loop. Neither type refuses an illegal combination at compile time or with a match.
>
> **Adopted.** A turn has a phase, and a finished turn is not the same slot as a running one. Soft stop and a budget wrap-up both pass through a wrap-up, and they do not end as the same phase. That split matches chapter 9: soft stop returns a cancel-shaped outcome, and a spent budget returns `Budget` or `Partial`. `run_loop` now records each of those observations on a `Driver`.
>
> **Adapted.** The table is exhaustive. Every `(Phase, TurnEvent)` pair is either `Ok(next)` or `Illegal`. Hermes stores the fields and lets the loop decide. I did not port `_LoopState`'s fields. Three events are new because the live loop has moves the first table did not name: `NoDone`, `BudgetAbort`, and `SoftWrapIgnoredTools`. Compression attempts, provider overflow, and verification holds stay in the chapters that own them (32 and 14).
>
> **Deferred.** `run_script` is still a fake model: a list of `ScriptStep` values, no HTTP, no key. Empty-response retries and the repetition guard stay in chapter 14. They are not new phases here.

[personal note: a transition you expected to be legal, and the test that said it was not]

## What this chapter decides

`Phase` is `Idle`, `CallingModel`, `Dispatching`, `WrappingUp`, and the terminals `Finished`, `Cancelled`, `SoftStopped`, `Budget`, `Partial`, `InvalidToolCalls`, `Stalled`. A terminal phase accepts no event.

`step` is the whole policy:

| From | Event | To |
|---|---|---|
| `Idle` | `Start` | `CallingModel` |
| `Idle` | `BudgetHit` | `Budget` |
| `CallingModel` | `ModelText` | `Finished` |
| `CallingModel` | `ModelToolCalls` | `Dispatching` |
| `CallingModel` | `HardCancel` | `Cancelled` |
| `CallingModel` | `SoftStop` | `WrappingUp` |
| `CallingModel` | `BudgetHit` | `WrappingUp` |
| `CallingModel` | `Truncated` | `Partial` |
| `CallingModel` | `InvalidStrikeout` | `InvalidToolCalls` |
| `Dispatching` | `ToolsDone` | `CallingModel` |
| `Dispatching` | `HardCancel` | `Cancelled` |
| `Dispatching` | `Stall` | `Stalled` |
| `WrappingUp` | `WrapUpText` | `Finished` |
| `WrappingUp` | `WrapUpToolCalls` | `Partial` |
| `WrappingUp` | `HardCancel` | `Cancelled` |
| `CallingModel` | `NoDone` | `Budget` |
| `WrappingUp` | `NoDone` | `Budget` |
| `CallingModel` | `BudgetAbort` | `Budget` |
| `WrappingUp` | `SoftWrapIgnoredTools` | `SoftStopped` |

Anything else is `Illegal { from, event }`. A budget that hits before the first model call goes `Idle` to `Budget` and never enters `CallingModel`. That matches chapter 9: a time cap already spent, with zero calls, returns `Budget` and does not call the provider.

`finish_wrap` maps a text wrap-up to `SoftStopped` or `Budget`. `run_script` does that when a wrap-up was the reason for `WrappingUp`. A wrap-up that still wants tools becomes `Partial` and does not call `finish_wrap`. Chapter 9's budget wrap-up that asks for tools returns `Partial`. Same split.

## A scripted turn

`run_script` starts with `step(Idle, Start)`. Each `ScriptStep` is one observation from a fake model.

`[Tools, Tools, Text]` means: the model asked for a tool, the tool finished, the model answered with text. The phase is `Finished`.

`[Tools, Tools, SoftStop, Text]` ends `SoftStopped`. `[Tools, Tools, Budget, Text]` ends `Budget`. `[Tools, Tools, Budget, Tools]` ends `Partial`, because the wrap-up asked for another tool.

The script step `Tools` means `ModelToolCalls` while the phase is `CallingModel`, and `ToolsDone` while it is `Dispatching`. One enum value, two phases. The table, not the script name, decides.

### Rust: the match is the spec

`step` matches `(from, event)` and the last arm is `_`. The test `every_pair_is_in_the_table` loops `Phase::all()` by `TurnEvent::all()` and counts legal and illegal results. A terminal phase that returns `Ok` fails the test. Adding a phase without adding it to `all` would hide it from that test, so `all` is the list the test trusts.

## Walking `step`, `Driver::advance`, and `drive_loop`

`step` is a match on `(Phase, event)`. The `_` arm returns `Illegal { from, event }`. There is no default next phase. A pair that is not written down is a bug in the caller, and `note` turns that `Err` into a panic so a test fails instead of the turn quietly continuing.

`Phase::is_terminal` is the complement of the four running phases: `Idle`, `CallingModel`, `Dispatching`, `WrappingUp`. A terminal phase has no row except the ones you add on purpose. `Start` from `Finished` is illegal. That is the property the grid test locks.

`finish_wrap` is a second function because `step(WrappingUp, WrapUpText)` returns `Finished` for both kinds of wrap-up. Soft stop and budget both enter `WrappingUp`. The caller remembers which one it was. `finish_wrap(WrapKind::Soft)` is `SoftStopped`. `finish_wrap(WrapKind::Budget)` is `Budget`. Forgetting the second call reports a soft stop as a normal finish.

`Driver` holds the phase and that `wrap` memory.

```rust
pub fn advance(&mut self, event: TurnEvent) -> Result<Phase, Illegal> {
    if event == TurnEvent::SoftStop {
        self.wrap = Some(WrapKind::Soft);
    } else if event == TurnEvent::BudgetHit && self.phase == Phase::CallingModel {
        self.wrap = Some(WrapKind::Budget);
    }
    let next = step(self.phase, event)?;
    self.phase = next;
    if self.phase == Phase::Finished {
        if let Some(kind) = self.wrap.take() {
            self.phase = finish_wrap(kind);
        }
    }
    Ok(self.phase)
}
```

`BudgetHit` from `Idle` does not set `wrap`. That row goes straight to `Budget` and never passes through `Finished`, so there is nothing for `finish_wrap` to correct. `BudgetHit` from `CallingModel` does set `wrap`, because that row enters `WrappingUp` and a later text answer would otherwise look like a normal finish.

`run_script` is the same walk with no provider. It starts at `step(Idle, Start)`, which is `CallingModel`, then maps each `ScriptStep` to one event. Two `Tools` steps are required for a tool turn: the first is `ModelToolCalls` (`CallingModel` to `Dispatching`), the second is `ToolsDone` (`Dispatching` back to `CallingModel`). One `Tools` ends in `Dispatching`, which is not terminal. The demo prints `Finished` because its script is `[Tools, Tools, Text]` and a text answer from `CallingModel` is `ModelText`.

The live loop is `drive_loop` in `agent.rs`. `run_loop` and `run_loop_with_inbox` both call it. `run_loop_driven` returns the `TurnOutcome` and the `Driver`'s phase, which is what `tests/driven.rs` asserts. `note` calls `advance` and panics with the illegal pair if the table refuses it.

The order inside one iteration matches chapter 9, and each branch names an event:

1. **Hard cancel.** If the phase is still `Idle`, `note` emits `Start` first. `Idle` + `HardCancel` is illegal, and a cancel that arrives before the model has been called is still a turn that started. Then `HardCancel`. The function returns. It does not mark `started_phase` on that path, because it is leaving.
2. **Steers and notices.** Unchanged from chapter 9. They do not move the phase.
3. **Soft stop.** `Start` if still idle, then `SoftStop`, then `soft_wrap_up`. A text reply is `WrapUpText`, and `advance` then applies `finish_wrap` so the phase is `SoftStopped`. A reply that asks for tools, or a batch with no `Done`, is `SoftWrapIgnoredTools`. The tools are not run. The stop reason stays soft stop. Using `WrapUpToolCalls` here would make the phase `Partial`, which is the budget row, and chapter 9's test `a_soft_wrap_up_that_asks_for_tools_does_not_run_them` would fail.
4. **The gate.** `check_gate` can return `Budget` after at least one model call. That path emits `Start` if needed, then `BudgetHit`, then `budget_wrap_up`. A text wrap-up becomes `Budget` via `finish_wrap`. A wrap-up that asks for tools is `WrapUpToolCalls` and the phase is `Partial`. A budget hit when the turn has not entered `CallingModel` is `BudgetHit` from `Idle` and returns immediately. A budget hit that finds the counter already at zero after the turn has started is `BudgetAbort`: `CallingModel` to `Budget`, no wrap-up. An `execute_code` refund can put the counter back at zero while the phase is already `CallingModel`. `BudgetHit` in that spot would try to enter `WrappingUp`, which is the wrong row.
5. **Any other stop** from the gate is `Start` if needed, then `HardCancel`.
6. **The model call.** `Start` once, and `started_phase` becomes true. The `select!` is chapter 9's: the token is polled first because of `biased`. A cancel that wins is `HardCancel`. A batch with no `Done` is `NoDone` and the phase is `Budget`. Empty tool calls are `ModelText` and the phase is `Finished`. Truncated arguments are `Truncated` and the phase is `Partial`. Three invalid batches are `InvalidStrikeout`. A runnable batch is `ModelToolCalls`, which enters `Dispatching`, and that is the moment chapter 11 will refuse a command before the dispatcher runs.
7. **After the tools.** A stall is `Stall`. Otherwise `ToolsDone` returns to `CallingModel` and the loop iterates.

`budget_wrap_up` and `soft_wrap_up` are the two single `complete` calls from chapter 9. They do not dispatch. Each one `note`s the event that matches what the batch actually was: hard cancel, no `Done`, text, or tools. The soft path maps both "no done" and "asked for tools" onto `SoftWrapIgnoredTools`. The budget path maps "asked for tools" onto `WrapUpToolCalls`.

> **Rust sidebar: why `note` panics**
>
> `step` returns `Result<Phase, Illegal>`. The loop could ignore the error and keep going. Then the table would be documentation. `note` uses `expect`-style panic (it formats the pair and `panic!`s) so an outcome chapter 9 can produce that the table cannot name fails the test suite. `Illegal` is a struct, not a string, so the test can match on `from` and `event` without parsing text.
>
> `Phase` and `TurnEvent` are `Copy`. Passing them to `step` does not move the driver's state. `Driver` is not `Copy` because it owns the `wrap` memory, and a copy would let two walks diverge.
>
> `started_phase` is a `bool` local, not a phase. The phase can be `CallingModel` while the flag is still false only in the sense that `Start` has not been recorded yet. The flag exists so a cancel before the first model call emits `Start` exactly once on the way out, and the normal path emits it exactly once on the way in.

## Tests

The table tests are still the pair grid, a tool-then-text finish, soft stop versus budget versus a wrap-up that still wants tools, stall plus hard cancel plus invalid plus truncated, and `Dispatching` plus `Start` is illegal. The rest of the 52 are chapter 9's loop, cancel, stall, steer, truncation, validation, budget, canonical, and stress tests, plus `tests/driven.rs`: a text answer finishes, a tool then text makes two provider calls, `max_iterations` 0 returns `Budget` with zero provider calls, and a soft stop before the model returns `SoftStopped`. "Across 15 binaries" is nextest's count of test targets (the library, the bins, the example, and each integration-test file), not three mysterious binaries.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch10-turn-state
```

```text
Starting 52 tests across 15 binaries
     Summary [   0.105s] 52 tests run: 52 passed, 0 skipped
```

```bash
cargo run -p ch10-turn-state
```

```text
phase: Finished
```

The binary runs `[Tools, Tools, Text]`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `turn_transition` | 1.103 ns | under 1 µs | `step(Idle, Start)` |
| `scripted_turn` | 7.803 ns | under 1 ms | the three-step script, no model |

`turn_transition` is `step` and `black_box` of the next phase. Building the enums is the arguments, which are `Copy`. `scripted_turn` runs `run_script` on a three-element slice inside the sample. There is no runtime, no sleep, and no provider.

Those two means are the last figures published for bench ids `10/turn_transition` and `10/scripted_turn`, from [run 38863938100](https://github.com/agnathan/rust-agent-harness/actions/runs/38863938100), section `10`. They are dashboard history. This chapter's crate is now the whole harness, so the Benchmarks workflow on the pull request that introduced the driver is the measurement of this tree. The ids and the function names stay, which is what keeps the history continuous. Do not paste a local `cargo bench`.

<!-- PR-BENCH
10/turn_transition
10/scripted_turn
-->

### Milestone demo

The `cargo run` above. The phase is `Finished`.

## Pitfalls

`run_script` uses `ScriptStep::Tools` for both the model's request and the tool finishing. A script of one `Tools` ends in `Dispatching`, which is not terminal. The demo uses two, then text.

`finish_wrap` is a second function. `step(WrappingUp, WrapUpText)` returns `Finished` either way. Forgetting to call `finish_wrap` reports a soft stop as `Finished`. The script helper does it. A caller that uses `step` alone has to do it too.

An unmatched script step returns `Illegal` with `event: Start`, which is the wrong event. The table test does not go through `run_script`. A script the match does not recognize is a bug in the script, and the error's event field is not a real `Start`. I would rather the helper returned the phase it was in. The tests in this chapter only use scripts the match knows.

## Guardrails in this chapter

The table is a guardrail on the loop's shape, not a new detector. [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L4.

**Illegal moves fail.** A stalled turn cannot start another model call through `step`. A finished turn cannot accept `Start`. The detectors that decide to emit `Stall`, `BudgetHit`, or `InvalidStrikeout` are chapter 9 (K1, J2, I1, I2). This chapter does not reimplement them.

**Wrap-up is not a second agent.** Soft stop and budget enter `WrappingUp` once. A text answer leaves. A tool request becomes `Partial`. Empty-response retries (J4) and the repetition guard (J3) are chapter 14. They are not rows in this table.

Secret redaction is still absent (E1, E2). Approvals are chapter 11. The table will grow a phase when that chapter has a real wait. It does not have one now.

## Chapter summary

- **Table.** Nineteen legal moves, including `NoDone`, `BudgetAbort`, and `SoftWrapIgnoredTools`. Every other pair is `Illegal`. The grid test counts both.
- **Loop.** `drive_loop` holds a `Driver` and panics if a move is illegal. A zero-iteration budget never enters `CallingModel`.
- **Script.** A tool call, a tool result, and a text answer end `Finished`. Soft stop ends `SoftStopped`. A budget wrap-up that asks for tools ends `Partial`.
- **Time.** One transition measured **1.103 ns**. The three-step script measured **7.803 ns**.

[personal note: which exit you had been handling inline, and how it looked as a row]

## What's next

Chapter 11 is approvals. The wait for a human has to sit in this table without holding the tool-concurrency permit. The outline already says that. This chapter does not add the wait.
