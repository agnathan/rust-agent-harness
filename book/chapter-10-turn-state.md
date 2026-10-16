# Chapter 10: The Turn State Machine

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 9 returns one `TurnOutcome` when a turn ends. The loop that produces it is a nest of checks. This chapter pulls the legal moves out into a table a test can exhaust. The table does not call a model, repair a name, or run a tool. Chapter 9 still does that work. This chapter says which phase is allowed to follow which event.

The step crate is `steps/ch07-turn-state`. The directory name stays `ch07-turn-state` so it matches the CI filter. The book number is 10. The package is `ch07-turn-state`.

| Section | Step | What it adds |
|---|---|---|
| 10 | `steps/ch07-turn-state` | `Phase`, `TurnEvent`, and `step` |

> **How Hermes does it**
>
> I read `gateway/session_state.py` and `agent/conversation_loop.py` on current Hermes `main`. `TurnState` there is a dataclass of pointers for one gateway turn: the running agent, the event, the context, a lease, timestamps. `clear` sets the pointers back to empty. It is not a table of moves. `_LoopState` in `conversation_loop.py` is the bag of locals the phase helpers in `agent/turn_*.py` pass by name. `run_conversation` is still the loop. Neither type refuses an illegal combination at compile time or with a match.
>
> **Adopted.** A turn has a phase, and a finished turn is not the same slot as a running one. Soft stop and a budget wrap-up both pass through a wrap-up, and they do not end as the same phase. That split matches chapter 9: soft stop returns a cancel-shaped outcome, and a spent budget returns `Budget` or `Partial`.
>
> **Adapted.** The table is exhaustive. Every `(Phase, TurnEvent)` pair is either `Ok(next)` or `Illegal`. Hermes stores the fields and lets the loop decide. I did not port `_LoopState`'s fields. Compression attempts, provider overflow, and verification holds stay in the chapters that own them (32 and 14).
>
> **Deferred.** Driving chapter 9's `run_loop` through this function is the follow-on. This chapter's `run_script` is a fake model: a list of `ScriptStep` values, no HTTP, no key. Empty-response retries and the repetition guard stay in chapter 14. They are not new phases here.

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

Anything else is `Illegal { from, event }`. A budget that hits before the first model call goes `Idle` to `Budget` and never enters `CallingModel`. That matches chapter 9: a time cap already spent, with zero calls, returns `Budget` and does not call the provider.

`finish_wrap` maps a text wrap-up to `SoftStopped` or `Budget`. `run_script` does that when a wrap-up was the reason for `WrappingUp`. A wrap-up that still wants tools becomes `Partial` and does not call `finish_wrap`. Chapter 9's budget wrap-up that asks for tools returns `Partial`. Same split.

## A scripted turn

`run_script` starts with `step(Idle, Start)`. Each `ScriptStep` is one observation from a fake model.

`[Tools, Tools, Text]` means: the model asked for a tool, the tool finished, the model answered with text. The phase is `Finished`.

`[Tools, Tools, SoftStop, Text]` ends `SoftStopped`. `[Tools, Tools, Budget, Text]` ends `Budget`. `[Tools, Tools, Budget, Tools]` ends `Partial`, because the wrap-up asked for another tool.

The script step `Tools` means `ModelToolCalls` while the phase is `CallingModel`, and `ToolsDone` while it is `Dispatching`. One enum value, two phases. The table, not the script name, decides.

### Rust: the match is the spec

`step` matches `(from, event)` and the last arm is `_`. The test `every_pair_is_in_the_table` loops `Phase::all()` by `TurnEvent::all()` and counts legal and illegal results. A terminal phase that returns `Ok` fails the test. Adding a phase without adding it to `all` would hide it from that test, so `all` is the list the test trusts.

## Tests

Five tests: the full pair grid, a tool-then-text finish, soft stop versus budget versus a wrap-up that still wants tools, stall plus hard cancel plus invalid plus truncated, and `Dispatching` plus `Start` is illegal.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch07-turn-state
```

```text
Starting 5 tests across 3 binaries
     Summary [   0.006s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch07-turn-state
```

```text
phase: Finished
```

The binary runs `[Tools, Tools, Text]`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `turn_transition` | 1.008 ns | under 1 µs | `step(Idle, Start)` |
| `scripted_turn` | 6.694 ns | under 1 ms | the three-step script, no model |

`turn_transition` is `step` and `black_box` of the next phase. Building the enums is the arguments, which are `Copy`. `scripted_turn` runs `run_script` on a three-element slice inside the sample. There is no runtime, no sleep, and no provider. CI section id is `10`.

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

- **Table.** Fifteen legal moves. Every other pair is `Illegal`. The grid test counts both.
- **Script.** A tool call, a tool result, and a text answer end `Finished`. Soft stop ends `SoftStopped`. A budget wrap-up that asks for tools ends `Partial`.
- **Time.** One transition measured **1.008 ns**. The three-step script measured **6.694 ns**.

[personal note: which exit you had been handling inline, and how it looked as a row]

## What's next

Chapter 11 is approvals. The wait for a human has to sit in this table without holding the tool-concurrency permit. The outline already says that. This chapter does not add the wait.
