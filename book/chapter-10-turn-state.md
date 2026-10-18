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

## 10.1 Phase, TurnEvent, and Illegal

A turn needs a type for "where it is" and a type for "what just happened." I made both of them enums. An enum is one value that is exactly one of its variants, so a later `match` either names every variant or admits that it did not.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Idle,
    CallingModel,
    Dispatching,
    WrappingUp,
    Finished,
    Cancelled,
    SoftStopped,
    Budget,
    Partial,
    InvalidToolCalls,
    Stalled,
}
```

`Copy` is the derive I rely on. `Phase` owns no `String` and no `Vec`. Assignment copies the bits. `is_terminal` takes `self` by value, and the `Driver` that called it still holds its phase, because the call received a copy.

```rust
impl Phase {
    pub fn is_terminal(self) -> bool {
        !matches!(
            self,
            Phase::Idle | Phase::CallingModel | Phase::Dispatching | Phase::WrappingUp
        )
    }
```

`matches!` is a `match` that yields a `bool`. The pattern lists the four phases that still have a legal exit: `Idle`, `CallingModel`, `Dispatching`, `WrappingUp`. The `!` flips the bool. `Finished`, `Cancelled`, `SoftStopped`, `Budget`, `Partial`, `InvalidToolCalls`, and `Stalled` are terminal. `Idle` is not terminal. A turn that has not started can still accept `Start`, and it can still accept `BudgetHit`.

> **Rust sidebar: `enum` and `Copy`**
> - An `enum` lists the values that exist. A `match` with no `_` stops compiling when a variant is added and an arm is missing.
> - `Copy` duplicates the bits on assignment and on a by-value call. `Phase` is a tag, so that duplicate is the whole value.
> - `Copy` requires `Clone`. `PartialEq` and `Eq` are what `==` and `assert_eq!` use. `Hash` is derived next to `Eq`, which is what a `HashMap` key needs. The grid test compares phases. It does not hash them.
> - `Driver` does not get this derive. Section 10.4 says why a second copy of the driver would be a second walk.

`all` is the slice the grid test iterates. The reference is `'static`: the array lives in the binary, and the test borrows it instead of allocating a `Vec` per run.

```rust
    pub fn all() -> &'static [Phase] {
        &[
            Phase::Idle,
            Phase::CallingModel,
            Phase::Dispatching,
            Phase::WrappingUp,
            Phase::Finished,
            Phase::Cancelled,
            Phase::SoftStopped,
            Phase::Budget,
            Phase::Partial,
            Phase::InvalidToolCalls,
            Phase::Stalled,
        ]
    }
}
```

Eleven phases. If I add a twelfth and forget to put it in this array, `step` still compiles, because `step` ends in `_`. The new phase would never appear in `every_pair_is_in_the_table`. `all` is the list that test trusts.

`TurnEvent` is the observation. It is `Copy` for the same reason `Phase` is. The loop detects the condition. This enum only names it.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnEvent {
    Start,
    ModelText,
    ModelToolCalls,
    ToolsDone,
    HardCancel,
    SoftStop,
    BudgetHit,
    Truncated,
    InvalidStrikeout,
    Stall,
    WrapUpText,
    WrapUpToolCalls,
    /// The provider batch had no `Done` event. The turn ends `Budget` and does
    /// not start a wrap-up.
    NoDone,
    /// The budget is already spent and the iteration counter is zero, but the
    /// turn has already entered `CallingModel` (an `execute_code` refund can
    /// do that). This is not `BudgetHit`, which would enter `WrappingUp`.
    BudgetAbort,
    /// A soft-stop wrap-up asked for tools, or returned no text. Chapter 9
    /// does not run those tools and the stop reason stays soft stop, so this
    /// is not `WrapUpToolCalls` (that row is the budget partial).
    SoftWrapIgnoredTools,
}
```

`Start` moves the turn into `CallingModel`. The loop emits it once on the way into the provider call, and once on the way out when a hard cancel or a soft stop arrives first. `ModelText` is an assistant whose `tool_calls` list is empty. `ModelToolCalls` is a batch `classify` left runnable. `ToolsDone` is the dispatcher returning without a stall, so the loop can call the model again. `HardCancel` is the cancellation token. `SoftStop` is the soft flag, and that path still owes one `complete`. `BudgetHit` is a spent cap, taken from `Idle` or from `CallingModel`. `Truncated` is an argument string cut off before it parsed. `InvalidStrikeout` is three batches with nothing runnable. `Stall` is `StallGuard::observe` returning `halt`.

`WrapUpText` is a wrap-up completion with no tool calls. `WrapUpToolCalls` is a budget wrap-up that asked for tools. The three comments on the enum are the moves added for the live loop. `NoDone` is a batch `absorb` could not turn into a message, and the turn ends `Budget` without a wrap-up. `BudgetAbort` is a spent budget whose call counter is already zero while the phase is already `CallingModel`. `SoftWrapIgnoredTools` is a soft wrap-up that asked for tools or returned no `Done`, and the stop reason stays soft stop. `TurnEvent::all` is a `'static` slice of all fourteen variants, in this order.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Illegal {
    pub from: Phase,
    pub event: TurnEvent,
}
```

`Illegal` is a struct of the pair that failed. Both fields are `Copy`, so the error is `Copy`. `every_pair_is_in_the_table` asserts `err.from` and `err.event` with `assert_eq!`. The fields are public so that test, which lives in another crate, can read them. Eleven phases times fourteen events is 154 pairs. Nineteen of those pairs are `Ok`. Every other pair is this struct.

## 10.2 Every arm of `step`

`step` is the policy. It takes the phase and the event by value, which is legal because both are `Copy`, and it returns a `Result`.

> **Rust sidebar: `Result`**
> - `Result<Phase, Illegal>` is an enum with two variants, `Ok` and `Err`. The success type and the error type are both named.
> - The caller decides what an `Err` means. `run_script` propagates it with `?`. `note` panics. Those are different policies, and the signature allows both.
> - `Ok` and `Err` are constructed like any other enum. `step` does not print, and it does not change a `Driver`.

```rust
pub fn step(from: Phase, event: TurnEvent) -> Result<Phase, Illegal> {
    let next = match (from, event) {
        (Phase::Idle, TurnEvent::Start) => Phase::CallingModel,
        (Phase::Idle, TurnEvent::BudgetHit) => Phase::Budget,
        (Phase::CallingModel, TurnEvent::ModelText) => Phase::Finished,
        (Phase::CallingModel, TurnEvent::ModelToolCalls) => Phase::Dispatching,
        (Phase::CallingModel, TurnEvent::HardCancel) => Phase::Cancelled,
        (Phase::CallingModel, TurnEvent::SoftStop) => Phase::WrappingUp,
        (Phase::CallingModel, TurnEvent::BudgetHit) => Phase::WrappingUp,
        (Phase::CallingModel, TurnEvent::Truncated) => Phase::Partial,
        (Phase::CallingModel, TurnEvent::InvalidStrikeout) => Phase::InvalidToolCalls,
        (Phase::Dispatching, TurnEvent::ToolsDone) => Phase::CallingModel,
        (Phase::Dispatching, TurnEvent::HardCancel) => Phase::Cancelled,
        (Phase::Dispatching, TurnEvent::Stall) => Phase::Stalled,
        (Phase::WrappingUp, TurnEvent::WrapUpText) => match_wrap_text(from),
        (Phase::WrappingUp, TurnEvent::WrapUpToolCalls) => Phase::Partial,
        (Phase::WrappingUp, TurnEvent::HardCancel) => Phase::Cancelled,
        (Phase::CallingModel, TurnEvent::NoDone) => Phase::Budget,
        (Phase::WrappingUp, TurnEvent::NoDone) => Phase::Budget,
        (Phase::CallingModel, TurnEvent::BudgetAbort) => Phase::Budget,
        (Phase::WrappingUp, TurnEvent::SoftWrapIgnoredTools) => Phase::SoftStopped,
        _ => {
            return Err(Illegal { from, event });
        }
    };
    Ok(next)
}
```

The `match` is on a tuple. Each arm is one legal pair. The value of a successful arm is the next `Phase`, bound to `next`, and the function returns `Ok(next)` after the match. The `_` arm does not produce a `next`. It `return`s. That `return` leaves the function before `Ok`, so an unnamed pair cannot become a phase.

> **Rust sidebar: `match` and `_`**
> - `match` is exhaustive. The compiler accepts this one because `_` covers every pair the named arms do not.
> - `_` throws the pair away. I still have `from` and `event` in scope, so the `Err` can carry them.
> - The arm's block returns from `step`, not from the match. If it fell through with a dummy phase, `Ok` would report a move the table does not list.
> - A refutable pattern such as `(Phase::Idle, TurnEvent::Start)` is one arm. Order does not matter here: the patterns do not overlap.

`Idle` plus `Start` is `CallingModel`. `run_script` takes it before the slice, and `drive_loop` takes it once, when `started_phase` is false and the gate has allowed a provider call. `Idle` plus `BudgetHit` is `Budget`. The turn never enters `CallingModel`. `tests/driven.rs` locks that with `max_iterations: 0`: outcome `Budget`, phase `Budget`, `provider.calls()` 0.

`CallingModel` plus `ModelText` is `Finished`: the assistant had no tool calls, and the loop returns `StopReason::Finished`. `CallingModel` plus `ModelToolCalls` is `Dispatching`. Chapter 11 refuses a command after this arm and before `dispatcher.run_raced`. `CallingModel` plus `HardCancel` is `Cancelled`. `Idle` plus `HardCancel` has no arm, so a cancel before `Start` emits `Start` first, lands in `CallingModel`, and only then notes `HardCancel`. The provider is not called.

`CallingModel` plus `SoftStop` and `CallingModel` plus `BudgetHit` are both `WrappingUp`. The caller stores a `WrapKind` so the endings can differ. `CallingModel` plus `Truncated` is `Partial`: the arguments are unusable, the error tool messages are pushed, and nothing is dispatched. `CallingModel` plus `InvalidStrikeout` is `InvalidToolCalls`, emitted when `invalid_strikes` reaches `MAX_INVALID_STRIKES` (3) on a batch that is not the malformed-JSON retry.

`Dispatching` plus `ToolsDone` is `CallingModel`, so the next iteration can call the model again. `Dispatching` plus `Start` has no arm. `dispatch_cannot_start` asserts that `Err` and asserts `err.from` is `Dispatching`. `Dispatching` plus `HardCancel` is `Cancelled`. `Dispatching` plus `Stall` is `Stalled`, noted after the tool messages are already pushed.

`WrappingUp` plus `WrapUpText` calls `match_wrap_text`, which returns `Finished` for both wrap kinds.

```rust
fn match_wrap_text(from: Phase) -> Phase {
    // A wrap-up that answers in text ends the turn. Soft stop and budget both
    // enter WrappingUp; the caller records which one it was. The table's exit
    // for a text wrap-up is Finished when the caller uses `finish_wrap`.
    let _ = from;
    Phase::Finished
}
```

`from` is named because the arm passes it. The body does not branch on it. `let _ = from` reads the parameter, and the next phase is always `Finished`. `Driver::advance` and `run_script` call `finish_wrap` when they see that `Finished` and still remember a `WrapKind`. A caller that stops at `step` reports a soft stop as a normal finish.

`WrappingUp` plus `WrapUpToolCalls` is `Partial`. That is the budget wrap-up that still wants tools. The next phase is `Partial`, so `finish_wrap` does not run, and the tools are not dispatched. `WrappingUp` plus `HardCancel` is `Cancelled`. The wrap-up `select!` polls the token first and drops the provider future when the token wins.

`CallingModel` plus `NoDone` is `Budget`. `absorb` returned `None`, the loop pushes `Event::Error("provider returned no done event")`, and it does not start a wrap-up. `WrappingUp` plus `NoDone` is also `Budget`: the budget wrap-up's own `complete` had no `Done`. A soft wrap-up uses `SoftWrapIgnoredTools` for that same observation, so the stop stays soft. `CallingModel` plus `BudgetAbort` is `Budget` when the gate says the budget is spent, `usage.calls` is 0, and `started_phase` is already true. `BudgetHit` from `CallingModel` would enter `WrappingUp`. Section 10.5 is the three `if`s that pick the event. `WrappingUp` plus `SoftWrapIgnoredTools` is `SoftStopped`. The tools are not run, and the stop reason is `SoftStop`. `a_soft_wrap_up_that_asks_for_tools_does_not_run_them` locks that: one provider call, no tool-role message.

Every other pair hits `_`. A terminal phase has no successful arm, so `every_pair_is_in_the_table` panics if a terminal `from` returns `Ok`. The test requires `legal + illegal` to equal `Phase::all().len() * TurnEvent::all().len()`, and both counts to be greater than zero. The nineteen arms are the ones above. `all` yields `&Phase`, and the test copies with `*` because `step` takes `self` by value.

## 10.3 `finish_wrap`, `WrapKind`, and `run_script`

`step(WrappingUp, WrapUpText)` returns `Finished` for a soft stop and for a budget wrap-up. The ending the rest of the book shows is `SoftStopped` or `Budget`. That second mapping is `finish_wrap`.

```rust
pub fn finish_wrap(kind: WrapKind) -> Phase {
    match kind {
        WrapKind::Soft => Phase::SoftStopped,
        WrapKind::Budget => Phase::Budget,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapKind {
    Soft,
    Budget,
}
```

`WrapKind` is `Copy`. Two variants, and the `match` has no `_`. A third kind would fail to compile here until I named it. `Soft` becomes `SoftStopped`. `Budget` becomes `Budget`. There is no `Result`. Every kind has an ending, and the function is the whole map.

`ScriptStep` is the fake model's vocabulary. The derive list is `Debug, Clone, PartialEq, Eq`. There is no `Copy`. The script is passed as `&[ScriptStep]`, so `run_script` borrows each step and never needs to move one out of the caller's array.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptStep {
    Text,
    Tools,
    HardCancel,
    SoftStop,
    Budget,
    Stall,
    Truncated,
    Invalid,
}
```

Eight variants. None of them is `Start`, `NoDone`, `BudgetAbort`, or `SoftWrapIgnoredTools`. Those events are produced by the live loop, or by the match below when it translates `Text` and `Tools` according to the current phase.

```rust
pub fn run_script(script: &[ScriptStep]) -> Result<Phase, Illegal> {
    let mut phase = step(Phase::Idle, TurnEvent::Start)?;
    let mut wrap = None;
    for item in script {
        let event = match (phase, item) {
            (Phase::CallingModel, ScriptStep::Text) => TurnEvent::ModelText,
            (Phase::CallingModel, ScriptStep::Tools) => TurnEvent::ModelToolCalls,
            (Phase::CallingModel, ScriptStep::HardCancel) => TurnEvent::HardCancel,
            (Phase::CallingModel, ScriptStep::SoftStop) => {
                wrap = Some(WrapKind::Soft);
                TurnEvent::SoftStop
            }
            (Phase::CallingModel, ScriptStep::Budget) => {
                wrap = Some(WrapKind::Budget);
                TurnEvent::BudgetHit
            }
            (Phase::CallingModel, ScriptStep::Truncated) => TurnEvent::Truncated,
            (Phase::CallingModel, ScriptStep::Invalid) => TurnEvent::InvalidStrikeout,
            (Phase::Dispatching, ScriptStep::Tools) => TurnEvent::ToolsDone,
            (Phase::Dispatching, ScriptStep::HardCancel) => TurnEvent::HardCancel,
            (Phase::Dispatching, ScriptStep::Stall) => TurnEvent::Stall,
            (Phase::WrappingUp, ScriptStep::Text) => TurnEvent::WrapUpText,
            (Phase::WrappingUp, ScriptStep::Tools) => TurnEvent::WrapUpToolCalls,
            (Phase::WrappingUp, ScriptStep::HardCancel) => TurnEvent::HardCancel,
            _ => {
                return Err(Illegal {
                    from: phase,
                    event: TurnEvent::Start,
                });
            }
        };
        phase = step(phase, event)?;
        if phase == Phase::Finished {
            if let Some(kind) = wrap {
                phase = finish_wrap(kind);
            }
        }
        if phase.is_terminal() {
            return Ok(phase);
        }
    }
    Ok(phase)
}
```

The first line is `step(Idle, Start)`. The `?` returns an `Illegal` from `run_script` when `step` returns `Err`, and binds the `Ok` phase otherwise. This pair is legal, so `phase` becomes `CallingModel` before the slice is read.

> **Rust sidebar: `?` and `Option`**
> - `?` on a `Result` returns the `Err` from the enclosing function. The success value is what the `let` binds. `run_script` returns `Result<Phase, Illegal>`, the same error type `step` uses, so `?` is legal.
> - `wrap` starts as `None`. `Some(WrapKind)` remembers which wrap was requested.
> - `if let Some(kind) = wrap` copies `kind` out, because `WrapKind` is `Copy`, and leaves the `Option` as `Some`. `Driver::advance` uses `take`, which writes `None`. `run_script` can leave the `Some` in place because a terminal phase returns on the next line.

`item` is `&ScriptStep`. The patterns say `ScriptStep::Text`. Match ergonomics match the variant behind the reference, and the caller's slice stays borrowed for the whole function.

`CallingModel` plus `Text` becomes `ModelText`, then `Finished`. `wrap` is still `None`, so `finish_wrap` is skipped. `CallingModel` plus `Tools` becomes `ModelToolCalls`, then `Dispatching`, which is not terminal, so the loop continues. The next `Tools` is now paired with `Dispatching` and becomes `ToolsDone`, then `CallingModel`. One `ScriptStep::Tools` is two events. The phase in the pattern picks the event, and `step` picks the next phase. `[Tools, Tools, Text]` is the demo. The binary prints `Finished`.

`CallingModel` plus `HardCancel` becomes `Cancelled`. That is the whole script `[HardCancel]` in `stall_cancel_and_invalid_are_terminal`. `CallingModel` plus `SoftStop` assigns `wrap = Some(Soft)` and the arm's value is `SoftStop`, so `step` enters `WrappingUp`. A following `Text` is `WrapUpText`. `step` returns `Finished`, and the copy into `finish_wrap` yields `SoftStopped`. `[Tools, Tools, SoftStop, Text]` ends `SoftStopped`.

`CallingModel` plus `Budget` assigns `wrap = Some(Budget)` and yields `BudgetHit`, which also enters `WrappingUp`. A following `Text` ends as `Budget` through `finish_wrap`. A following `Tools` is `WrapUpToolCalls`. `step` returns `Partial`, and `finish_wrap` does not run because the phase is `Partial`. `[Tools, Tools, Budget, Text]` ends `Budget`. `[Tools, Tools, Budget, Tools]` ends `Partial`, because the wrap-up asked for another tool.

`CallingModel` plus `Truncated` ends `Partial`. `CallingModel` plus `Invalid` ends `InvalidToolCalls`. `Dispatching` plus `HardCancel` ends `Cancelled`. `Dispatching` plus `Stall` ends `Stalled`. `[Tools, Stall]` uses one `Tools` so the phase is `Dispatching` when `Stall` is observed, and the script never records `ToolsDone`. `[Tools]` alone falls out of the loop and returns `Ok(Dispatching)`. `WrappingUp` plus `HardCancel` ends `Cancelled`. `finish_wrap` does not run, and the `WrapKind` is dropped with the function.

The `_` arm returns `Illegal { from: phase, event: TurnEvent::Start }`. `phase` is the phase the script had reached. The event is `Start` for every unrecognized step: `Text` while `Dispatching`, `Stall` while `CallingModel`, `Budget` while `WrappingUp`, `SoftStop` while `Dispatching`. The table test does not go through `run_script`. A script that stops on `SoftStop` with no following step returns `Ok(WrappingUp)`, which is not terminal. The demo expects `Finished`.

## 10.4 `Driver` remembers which wrap it was

`run_script` keeps `phase` and `wrap` as locals. The live loop needs the same pair across `.await` points, so I put them in a struct.

```rust
#[derive(Debug)]
pub struct Driver {
    phase: Phase,
    wrap: Option<WrapKind>,
}
```

The derive is `Debug` only. Both fields could be `Copy` — `Phase` is, and `Option<WrapKind>` is because `WrapKind` is — and the struct still does not derive `Copy` or `Clone`. A second copy would be a second walk. `advance` takes `&mut self`, so there is one driver per turn.

> **Rust sidebar: `&mut` and why `Driver` is not `Copy`**
> - `&mut self` is the only way `advance` can write `phase` and `wrap`. The caller keeps ownership. The method borrows it for the call.
> - `Phase` is `Copy`, so `step(self.phase, event)` copies the tag out and does not move the field.
> - `Driver` is not `Copy` even though both fields could be. I want a move of the driver to mean "this walk has a new owner," and I want two owners to be a compile error.
> - `phase` and `wrap` are private. Another crate reads the phase through `phase()` and changes it through `advance`.

```rust
impl Driver {
    pub fn new() -> Self {
        Self {
            phase: Phase::Idle,
            wrap: None,
        }
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Record one observation. A text wrap-up then applies `finish_wrap` so a
    /// soft stop and a budget wrap-up do not both land on `Finished`.
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
}
```

`new` starts at `Idle` with `wrap: None`. `phase(&self)` returns the tag by copy, so `run_loop_driven` can read it after the mutable borrow for `drive_loop` ends.

`advance` writes `wrap` before `step`. `SoftStop` stores `Some(Soft)` in whatever phase the driver is in. If `step` then returns `Err`, the wrap has changed and the phase has not, and `note` panics before the loop can continue. `BudgetHit` stores `Some(Budget)` only when `self.phase == Phase::CallingModel`. `BudgetHit` from `Idle` leaves `wrap` as `None`. That row goes to `Budget` and never passes through `Finished`, so `finish_wrap` has nothing to correct. `BudgetHit` from `CallingModel` does store it, because that row enters `WrappingUp` and a later `WrapUpText` becomes `Finished` inside `step`.

The `?` returns the `Illegal` before `self.phase = next`, so a refused move leaves the phase where it was. On `Finished`, `self.wrap.take()` writes `None` and yields the old `WrapKind` for `finish_wrap`. A later `Finished` sees `None`. `run_script` copies the kind out and leaves the `Some` in place, because it returns as soon as the phase is terminal.

`Default` forwards to `new`. The loop calls `Driver::new()`.

```rust
impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
```

## 10.5 How the loop asks the table before it calls the model

`run_loop`, `run_loop_with_inbox`, and `run_loop_driven` all end in `drive_loop`. They differ in what they do with the `Driver`.

```rust
pub async fn run_loop(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
) -> TurnOutcome {
    run_loop_with_inbox(
        messages, provider, registry, dispatcher, budget, cancel, started, None,
    )
    .await
}
```

> **Rust sidebar: `async fn`**
> - `async fn` returns a future. The `.await` inside `run_loop` is the point where this call waits for `run_loop_with_inbox`. The compiler builds the state machine that stores the borrows across that wait.
> - The arguments are borrows: `&mut Vec<Message>`, `&dyn Provider`, `&ToolRegistry`, `&Dispatcher`, `&Budget`, `&CancelFlag`. The caller owns the vec and the services. The future borrows them until it completes, so the caller cannot mutate `messages` on the side while the turn runs.
> - `Instant` is `Copy`, so `started` is a timestamp value, not a borrow of the caller's clock.

`run_loop` passes `None` for the inbox. `run_loop_with_inbox` builds a `Driver` and does not return the phase. `run_loop_driven` returns `(outcome, driver.phase())`, which is what `tests/driven.rs` asserts.

```rust
pub async fn run_loop_driven(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
) -> (TurnOutcome, Phase) {
    let mut driver = Driver::new();
    let outcome = drive_loop(
        messages,
        provider,
        registry,
        dispatcher,
        budget,
        cancel,
        started,
        None,
        &mut driver,
    )
    .await;
    (outcome, driver.phase())
}
```

The mutable borrow of `driver` lasts for the `.await`. After it ends, `phase()` copies the tag into the returned tuple, and the driver is dropped.

`note` is the only place the loop turns an `Illegal` into a panic.

```rust
fn note(driver: &mut Driver, event: TurnEvent) {
    driver.advance(event).unwrap_or_else(|err| {
        panic!(
            "illegal turn move from {:?} on {:?} — the loop and the table disagree",
            err.from, err.event
        )
    });
}
```

> **Rust sidebar: `panic!` and `Result`**
> - `advance` returns `Result<Phase, Illegal>`. `unwrap_or_else` runs the closure only on `Err`. The closure's value would have to be a `Phase` if the closure returned. This closure calls `panic!`, which does not return, so the type checks.
> - `panic!` aborts the task. The turn does not keep going with a stale phase. A test that drives the loop into an unnamed pair fails the suite.
> - `Illegal` is a struct. `{:?}` prints `from` and `event` because both types derive `Debug`. The test crate can also match the fields without parsing that string. `note` itself is private. The public loops either finish or panic.

`drive_loop` owns the counters the table does not know about: `events`, `usage`, `json_retries`, `invalid_strikes`, the `StallGuard`, `iteration`, and `notices`. `started_phase` is a `bool`, not a phase.

```rust
    let mut started_phase = false;
    loop {
        if cancel.is_cancelled() {
            if let Some(inbox) = inbox {
                drop_pending_steer(cancel, inbox);
            }
            if !started_phase {
                note(driver, TurnEvent::Start);
            }
            note(driver, TurnEvent::HardCancel);
            return finish(StopReason::Cancelled, events, usage, last_text(messages));
        }
```

The hard-cancel check is first. `if let Some(inbox)` borrows the inbox only when one was passed. `drop_pending_steer` pops until empty, and only while `is_cancelled` is true, so a soft stop leaves the inbox alone. `hard_stop_drops_a_pending_steer_and_soft_stop_does_not` locks that. The drain runs before the notes.

`Idle` plus `HardCancel` is illegal, so a cancel before `Start` notes `Start` and then `HardCancel`. `Start` moves `Idle` to `CallingModel`, and `HardCancel` from there is `Cancelled`. The function returns without setting `started_phase`. `finish(StopReason::Cancelled)` builds `TurnOutcome::Cancelled { hard: true, .. }`. `last_text` clones the newest assistant content that has no tool calls, or returns an empty string.

Steers and notices are next. They do not note an event.

```rust
        if let Some(inbox) = inbox {
            append_steers(messages, inbox);
        }
        inject_budget_notices(
            messages,
            budget,
            &usage,
            started.elapsed(),
            cancel,
            &mut notices,
        );
```

`append_steers` returns before it pops when no message has `Role::Tool`, so the lines stay queued until a later iteration has a tool message. When one exists, it pops every line, drops the empty ones, joins the rest with newlines, and `append_steer_text` pushes `"\n\n"` plus that string onto the newest tool message. An empty steer, or a reverse scan that finds no tool message, returns false. `inject_budget_notices` may append a warning the same way. Neither call notes an event.

Soft stop is the next flag, and it is checked before the gate so a soft stop still gets its wrap-up when a budget is also spent.

```rust
        if cancel.is_soft_stop() {
            if !started_phase {
                note(driver, TurnEvent::Start);
            }
            note(driver, TurnEvent::SoftStop);
            return soft_wrap_up(messages, provider, events, usage, cancel, driver).await;
        }
```

`Start` first, for the same reason as hard cancel: the phase has to be `CallingModel` before `SoftStop` is legal. `advance` sees `SoftStop` and stores `WrapKind::Soft` before `step` enters `WrappingUp`. `soft_wrap_up` is the one extra `complete`. Section 10.6 walks its arms. `a_soft_stop_before_the_model_ends_soft_stopped` asserts the outcome is `SoftStop` and the phase is `SoftStopped`.

The gate is `check_gate`. It returns `Option<StopReason>`: `Some(Cancelled)` when the token is set, `Some(Budget)` when `Budget::exceeded` is true, and `None` otherwise. The token arm here is a second look. The top of the loop already returned if the token was set before the steers. A cancel that lands during those synchronous calls takes the last arm below.

`exceeded` is four checks, in order. `usage.calls >= max_iterations`, then `total_tokens` over `max_tokens` when that cap is `Some`, then `cost_usd` over `max_cost_usd` when the cost is known, then `started.elapsed() > max_duration` when a duration is set. Any one of them makes the gate return `Budget`.

```rust
        if let Some(stop) = check_gate(cancel, budget, &usage, started) {
            if stop == StopReason::Budget && usage.calls > 0 {
                if !started_phase {
                    note(driver, TurnEvent::Start);
                }
                note(driver, TurnEvent::BudgetHit);
                return budget_wrap_up(messages, provider, events, usage, cancel, driver).await;
            }
            if stop == StopReason::Budget && !started_phase {
                note(driver, TurnEvent::BudgetHit);
                return finish(stop, events, usage, last_text(messages));
            }
            if stop == StopReason::Budget {
                note(driver, TurnEvent::BudgetAbort);
                return finish(stop, events, usage, last_text(messages));
            }
            if !started_phase {
                note(driver, TurnEvent::Start);
            }
            note(driver, TurnEvent::HardCancel);
            return finish(stop, events, usage, last_text(messages));
        }
```

The first three arms return. The order is the policy.

`Budget` with `usage.calls > 0` notes `Start` when the flag is still false, so `BudgetHit` is observed from `CallingModel`. `advance` stores `WrapKind::Budget` only in that phase, `step` enters `WrappingUp`, and `budget_wrap_up` performs the one wrap-up call. Usage starts at 0 and `started_phase` becomes true before the provider call, so a later iteration with `calls > 0` has already started. The `Start` in this arm is what keeps a positive counter from taking the `Idle` row.

`Budget` with `started_phase` still false is the zero-call cap, because the previous arm returned when `calls > 0`. `max_iterations: 0` makes `calls >= max_iterations` true while `calls` is 0. The note is `BudgetHit` from `Idle`. The table goes to `Budget`. There is no `Start`, no `CallingModel`, and no provider call. `finish(stop)` returns `StopReason::Budget`.

The remaining `Budget` has `calls == 0` and `started_phase` true. The note is `BudgetAbort`. `step(CallingModel, BudgetAbort)` is `Budget`, with no wrap-up. `BudgetHit` from `CallingModel` would enter `WrappingUp`, which is the first arm's row. An `execute_code` refund is how `calls` can be 0 after the turn has entered `CallingModel`. `refund_iteration` decrements `calls` when `calls > 0` and leaves tokens, cost, and the clock alone, so `exceeded` can still be true on one of those other caps.

The last arm is the non-budget stop. `check_gate` produces `Cancelled` there. The loop notes `Start` when the flag is false, then `HardCancel`, and returns `StopReason::Cancelled`.

If the gate returns `None`, the loop is allowed to call the model. `Start` is recorded once.

```rust
        if !started_phase {
            note(driver, TurnEvent::Start);
            started_phase = true;
        }
```

The flag is set after the note. A later iteration that emitted `Start` again would call `step(CallingModel, Start)`, which is `Illegal`, and `note` would panic. The normal path emits `Start` once on the way in. Cancel and soft stop emit it once on the way out and return without setting the flag.

## 10.6 The batch, the tools, the wrap-up, and `absorb`

The provider call sits in `tokio::select!`. The token arm is written first, and `biased` makes that order matter when both futures are ready.

```rust
        let token = cancel.token();
        let send = canonical_messages(messages);
        let batch = tokio::select! {
            biased;
            _ = token.cancelled() => {
                note(driver, TurnEvent::HardCancel);
                return finish(StopReason::Cancelled, events, usage, last_text(messages));
            }
            batch = provider.complete(&send) => batch,
        };
```

> **Rust sidebar: `tokio::select!` and `biased`**
> - `select!` waits until one future completes, then drops the others. The provider future is dropped when the token wins.
> - `biased` polls the arms in source order and takes the first one that is ready. The token arm is first, so a cancel that has already fired is recorded as `HardCancel` from `CallingModel`.
> - The `_` pattern ignores the unit value `cancelled()` produces. The loop's answer is the `return`.

`canonical_messages` builds the request before the `select!`, so both arms share one snapshot. `absorb` turns the batch into an assistant message, or into `None` when no `Done` arrived.

```rust
        let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
            events.push(Event::Error("provider returned no done event".into()));
            note(driver, TurnEvent::NoDone);
            return finish(StopReason::Budget, events, usage, last_text(messages));
        };
```

`let Some(assistant) = ... else` returns early on `None`. That arm pushes an error event, notes `NoDone`, and finishes `Budget`. The phase is `CallingModel`, so the turn does not enter `WrappingUp`.

An assistant with an empty `tool_calls` list is the answer.

```rust
        let requested = assistant.tool_calls.clone();
        if requested.is_empty() {
            messages.push(assistant);
            note(driver, TurnEvent::ModelText);
            return finish(StopReason::Finished, events, usage, last_text(messages));
        }
```

The clone happens before `push` moves `assistant`. `ModelText` from `CallingModel` is `Finished`. `a_text_answer_finishes_in_the_table` asserts that phase.

When the list is not empty, `known_names` collects each `schema.name` and `classify` marks truncated or malformed arguments. Truncation is checked first. A truncated batch pushes the assistant and a tool error for every call, notes `Truncated`, and returns `StopReason::Partial`. Nothing is dispatched.

Malformed JSON is the next flag. `MAX_JSON_RETRIES` is 3.

```rust
        if classified.malformed {
            json_retries += 1;
            if json_retries < MAX_JSON_RETRIES {
                continue;
            }
            json_retries = 0;
        } else {
            json_retries = 0;
        }
```

A malformed batch increments the counter. While it is below 3, `continue` returns to the top without pushing the assistant. The phase stays `CallingModel`, so the next provider call does not need another `Start`. On the third malformed batch the counter is 3, the `<` fails, and the counter is reset to 0. A batch that is not malformed clears the counter. The truncated arm already returned, so a batch that is both truncated and malformed never reaches this counter.

Runnable calls are the planned calls whose `error` is `None`.

```rust
        if runnable.is_empty() {
            push_assistant(messages, assistant, &classified);
            push_errors(messages, &classified);
            if classified.malformed {
                continue;
            }
            invalid_strikes += 1;
            if invalid_strikes >= MAX_INVALID_STRIKES {
                note(driver, TurnEvent::InvalidStrikeout);
                return finish(
                    StopReason::InvalidToolCalls,
                    events,
                    usage,
                    last_text(messages),
                );
            }
            continue;
        }
```

`push_assistant` rewrites `tool_calls` from the classified calls, sets each `raw_arguments` to `None`, and pushes the message. `push_errors` pushes one tool message per call, using `call.error` or `"error: invalid tool call"` when the error is `None`. An empty runnable batch that is still malformed `continue`s without touching `invalid_strikes`. Any other empty batch increments the strike counter, notes `InvalidStrikeout` at `MAX_INVALID_STRIKES` (3), and otherwise `continue`s in `CallingModel`.

A non-empty runnable list clears `invalid_strikes`, notes `ModelToolCalls`, and pushes the assistant. The phase is now `Dispatching`. The note is before the dispatcher runs. Chapter 11 inserts the approval check in that gap.

```rust
        invalid_strikes = 0;
        note(driver, TurnEvent::ModelToolCalls);
        push_assistant(messages, assistant, &classified);
        let mut allowed = Vec::new();
        let mut blocked = Vec::new();
        for call in runnable {
            if call.name == "delegate" {
                if usage.subagent_calls >= budget.max_subagent_iterations {
                    blocked.push(call);
                    continue;
                }
                usage.subagent_calls += 1;
            }
            allowed.push(call);
        }
```

The loop moves each `PlannedCall`. A `delegate` at `max_subagent_iterations` goes to `blocked` and `continue`s. A `delegate` under the cap increments `usage.subagent_calls` and falls through into `allowed`. Every other name goes to `allowed`. The cap is a counter, not a phase.

`blocked_ids` clones those ids before `blocked` is finished with. `calls` is a `Vec<ToolCall>` built only from `allowed`. `run_raced` gets that vec, the registry, and `cancel.token()`, and the outputs come back in the same order.

```rust
        let outputs = dispatcher
            .run_raced(registry, &calls, &cancel.token())
            .await;
        let mut ran = allowed.into_iter().zip(outputs);
```

`into_iter` moves `allowed`. `zip` pairs each planned call with the output at the same index. The result walk does not iterate `ran` directly. It walks `classified.calls`, which still contains the error calls and the blocked calls that were never sent.

```rust
        let mut halt = false;
        for planned in &classified.calls {
            if let Some(error) = &planned.error {
                messages.push(Message::tool(planned.id.clone(), error.clone()));
                continue;
            }
            if blocked_ids.iter().any(|id| id == &planned.id) {
                messages.push(Message::tool(
                    planned.id.clone(),
                    "error: subagent iteration cap",
                ));
                continue;
            }
            let Some((_, output)) = ran.next() else {
                messages.push(Message::tool(
                    planned.id.clone(),
                    "error: tool result missing",
                ));
                continue;
            };
```

The order of those three arms is the alignment. An error call was not in `allowed`, so it must `continue` before `ran.next()`. A blocked id was not in `allowed` either. `ran.next()` is only for a call that was dispatched. If the zip is exhausted, the message is `"error: tool result missing"` and the loop continues. Pulling `next` earlier would attach the next tool's output to a call that never ran.

`tool_text` clones the first content block on `Ok`, or `""` when that vec is empty, and formats `"error: {error}"` on `Err`. `truncate_head_tail` cuts the string to `TOOL_OUTPUT_MAX_CHARS`. `failed` is true on `Err`, and also on `Ok` when `result.is_error` is true. That second case is the `matches!` guard. `stall.observe` then sees the iteration, the name, the arguments via `to_string`, the truncated text, and `failed`. `action.halt` sets the local flag, and the tool message uses `action.text`.

```rust
            if planned.name == "execute_code" && !failed {
                usage.refund_iteration();
            }
            messages.push(Message::tool(planned.id.clone(), action.text));
        }
        iteration += 1;
        if halt {
            note(driver, TurnEvent::Stall);
            return finish(StopReason::Stalled, events, usage, last_text(messages));
        }
        note(driver, TurnEvent::ToolsDone);
    }
}
```

The refund runs only for a successful `execute_code`. `refund_iteration` subtracts one from `calls` when `calls > 0`, which is how the next gate can see `calls == 0` with `started_phase` already true and take `BudgetAbort` if another cap is still exceeded. `iteration` then increments. `halt` notes `Stall` from `Dispatching` and returns. Otherwise `ToolsDone` moves the phase back to `CallingModel` and the loop starts again at the hard-cancel check.

`budget_wrap_up` is the single `complete` after `BudgetHit` from `CallingModel`. It does not build a `ToolCall` list and it does not call the dispatcher.

```rust
    let batch = tokio::select! {
        biased;
        _ = token.cancelled() => {
            note(driver, TurnEvent::HardCancel);
            return finish(StopReason::Cancelled, events, usage, last_text(messages));
        }
        batch = provider.complete(&send) => batch,
    };
    let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
        events.push(Event::Error("provider returned no done event".into()));
        note(driver, TurnEvent::NoDone);
        return finish(StopReason::Budget, events, usage, last_text(messages));
    };
    if assistant.tool_calls.is_empty() {
        let text = assistant.content.clone();
        messages.push(assistant);
        note(driver, TurnEvent::WrapUpText);
        return finish(StopReason::Budget, events, usage, text);
    }
    note(driver, TurnEvent::WrapUpToolCalls);
    finish(StopReason::Partial, events, usage, last_text(messages))
```

The phase is `WrappingUp`. `HardCancel` returns `Cancelled` before `absorb`. A missing `Done` notes `NoDone`, and `WrappingUp` plus `NoDone` is `Budget`. An empty tool list clones `content` for the outcome, pushes the assistant, and notes `WrapUpText`. `step` returns `Finished`, `advance` `take`s `WrapKind::Budget`, and `finish_wrap` sets the phase to `Budget` beside `StopReason::Budget`. A non-empty tool list notes `WrapUpToolCalls`. The phase becomes `Partial`, the assistant is not pushed, and the tools are not run. That `finish(StopReason::Partial)` is the tail of the function, so it is the return value.

`soft_wrap_up` has the same `select!` and the same early `HardCancel`. The other two observations are different events, and both endings share one stop reason.

```rust
    let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
        events.push(Event::Error("provider returned no done event".into()));
        note(driver, TurnEvent::SoftWrapIgnoredTools);
        return finish(StopReason::SoftStop, events, usage, last_text(messages));
    };
    if assistant.tool_calls.is_empty() {
        messages.push(assistant);
        note(driver, TurnEvent::WrapUpText);
    } else {
        note(driver, TurnEvent::SoftWrapIgnoredTools);
    }
    finish(StopReason::SoftStop, events, usage, last_text(messages))
```

A missing `Done` notes `SoftWrapIgnoredTools` and returns `StopReason::SoftStop`. `NoDone` from `WrappingUp` would make the phase `Budget`, and a soft stop has to stay a soft stop. An empty tool list pushes the assistant and notes `WrapUpText`, so `advance` applies `finish_wrap(WrapKind::Soft)` and the phase becomes `SoftStopped`. A non-empty tool list notes `SoftWrapIgnoredTools`, does not push the assistant, and does not run the tools. The phase becomes `SoftStopped` from that arm directly. Both arms fall through to `finish(StopReason::SoftStop)`. `finish` builds `TurnOutcome::Cancelled { hard: false, .. }`. `TurnOutcome::stop` maps `hard: false` back to `StopReason::SoftStop`, and the driver's phase is `SoftStopped`. `phase_of` in `tests/driven.rs` maps `StopReason::SoftStop` to `Phase::SoftStopped`, and the soft-stop test asserts both.

`absorb` is the function both wrap-ups and the main call share.

```rust
fn absorb(batch: &[Event], events: &mut Vec<Event>, usage: &mut AgentUsage) -> Option<Message> {
    let mut assistant = None;
    let mut saw_usage = false;
    for event in batch {
        if let Event::Usage(reported) = event {
            usage.record(reported);
            saw_usage = true;
        }
        if let Event::Done { message, .. } = event {
            assistant = Some(message.clone());
        }
        events.push(event.clone());
    }
    if !saw_usage {
        usage.count_call_without_usage();
    }
    assistant
}
```

`batch` is a shared slice. `events` and `usage` are `&mut`, so the caller keeps ownership while `absorb` appends and increments. `assistant` starts as `None`. `Event::Usage` calls `usage.record` and sets `saw_usage`. `Event::Done` clones the message into `assistant`, and a later `Done` overwrites that `Some`. `TextDelta`, `ToolCall`, and `Error` match neither `if let`, and they are still cloned onto `events` because the outcome owns the log after `batch` is dropped. The `Done` clone is the same rule for the `Option<Message>`. With no usage event, `count_call_without_usage` increments `calls` by one. That is the increment the next gate's `calls > 0` arm sees, and the one `refund_iteration` subtracts after a successful `execute_code`. The returned `Option` is `None` only when the batch contained no `Done`.

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
| `turn_transition` | 1.258 ns | under 1 µs | `step(Idle, Start)` |
| `scripted_turn` | 19.08 ns | under 1 ms | the three-step script, no model |

`turn_transition` is `step` and `black_box` of the next phase. Building the enums is the arguments, which are `Copy`. `scripted_turn` runs `run_script` on a three-element slice inside the sample. There is no runtime, no sleep, and no provider.

Those two means are `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `10`, on this pull request's tree. The ids and the function names stay, which is what keeps the dashboard history continuous. Do not paste a local `cargo bench`. The printed Criterion `time:` line is the slope, not this mean.

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
- **Time.** One transition measured **1.258 ns**. The three-step script measured **19.08 ns**.

[personal note: which exit you had been handling inline, and how it looked as a row]

## What you implement

Chapter 10 has no `my-harness/tests/ch10.rs`. The table is the loop the later milestones sit on, and the reference is `steps/ch10-turn-state`. Before you run the binary, name the phase of three scripts. `[Tools, Tools, Text]` is `Finished`. `[Tools, Tools, SoftStop, Text]` is `SoftStopped`. `[Tools, Tools, Budget, Tools]` is `Partial`, because the wrap-up asked for a tool and `finish_wrap` does not run. Then say why a cancel that arrives while the phase is still `Idle` emits `Start` and then `HardCancel`: `Idle` plus `HardCancel` is not a row, and `note` panics on an illegal pair. The grid test in the step crate is the check.

## What's next

Chapter 11 is approvals. The wait for a human has to sit in this table without holding the tool-concurrency permit. The outline already says that. This chapter does not add the wait.
