# Chapter 36: Supervision and Restart

A child that goes quiet is the failure this chapter can see. Fifteen idle cycles times 30 seconds is 450 seconds, and inside a tool the cap is 40 cycles, 1200 seconds. Progress, a new iteration, a different tool, or a newer activity timestamp, means the child isn't stale. A slow model that refreshes the timestamp isn't idle.

`restart_child` calls `complete` a second time after `record`. That is one retry of one completion, with bookkeeping. It isn't a new task, and it isn't a restart of chapter 35's actor. A real process restart waits, which is why the title stops at the second call and the stamps around it. The heartbeat that chapter 29 and chapter 31 deferred is the comparison above. The cancellation token and the budget stay the ones chapter 9 put on the call.

A reviewer or a curator that hangs has to be noticed. One retry is the bound so the child's writes aren't run forever. That is what a continuously improving team needs when reflection, a specialist, and a scheduled curator are all children. The restart stays on this harness because it is about the child the supervisor started. A remote tool has its own idea of a dead backend. Mixing the two clocks would restart an agent because a remote tool was slow, or refuse a remote tool because an agent was idle.

The step is a copy of chapter 35 plus `supervision`, at `steps/ch36-supervision`, package `ch36-supervision`. `cargo run -p ch36-supervision --bin ch36-supervision --offline` prints `restarted: ok`. The exercise is `my-harness/tests/ch36.rs`, where an idle child is stale at the short threshold and a child inside a tool uses the longer one.

| Section | Step | What it adds |
|---|---|---|
| 36.1 | `steps/ch36-supervision` | `RestartError`, `ChildFailed`, the three restart constants |
| 36.2 | `steps/ch36-supervision` | `FakeModel` and `Call` |
| 36.3 | `steps/ch36-supervision` | `Supervisor`, `record`, `fail_io` |
| 36.4 | `steps/ch36-supervision` | `restart_child` and `demo` |
| 36.5 | `steps/ch36-supervision` | heartbeat constants, `Pulse`, `is_stale` |

> **How Hermes does it**
>
> I read `tools/delegate_tool.py` and `tools/delegate_tool_child_run.py`, and `gateway/restart_loop_guard.py`, on Hermes `main` at `9e01b44`.
>
> The heartbeat constants sit together in `tools/delegate_tool.py`:
>
> 

```text
> 105  DEFAULT_MAX_ITERATIONS = 250
> 106  _HEARTBEAT_INTERVAL = 30  # seconds between parent activity heartbeats during delegation
> 107  # Stale-heartbeat thresholds (cycles of _HEARTBEAT_INTERVAL with no progress). Progress = iteration, current_tool OR
> 108  # last_activity_ts advancing; an in-flight model wait refreshes last_activity_ts, so slow models are not "idle". Idle
> 109  # stays tight so a truly wedged child doesn't mask the gateway timeout; in-tool is much higher so legitimately long
> 110  # tools can finish.
> 111  _HEARTBEAT_STALE_CYCLES_IDLE = 15  # 450s idle between turns → stale
> 112  _HEARTBEAT_STALE_CYCLES_IN_TOOL = 40  # 1200s stuck on same tool → stale
> ```

>
> `_start_heartbeat` is defined in `tools/delegate_tool_child_run.py` and imported back into `delegate_tool.py` (the import is at lines 27–29). `_run_single_child` calls it at line 363. The comment above that call says the heartbeat keeps the parent's activity timestamp moving, and once the child looks stale it also ends the wait. `_Heartbeat.tick` is where the cycle count is chosen:
>
> 

```text
> 321  last_seen["stale"] += 1
> 322  stale_cycles = _HEARTBEAT_STALE_CYCLES_IN_TOOL if child_tool else _HEARTBEAT_STALE_CYCLES_IDLE
> 323  if last_seen["stale"] >= stale_cycles:
> ```

>
> `child_tool` is `current_tool` from the child's activity summary. A tool name selects 40. No tool name selects 15. Progress, a few lines above, is `child_iter > last_seen["iter"]`, or `child_tool != last_seen["tool"]`, or `last_activity_ts` advancing. On progress the stale counter goes back to 0. `_Heartbeat.start` schedules `tick` every `_HEARTBEAT_INTERVAL` through `agent.periodic_scheduler`. The class docstring says this is not one daemon thread per child.
>
> The restart breaker is a different file. `gateway/restart_loop_guard.py` sets `DEFAULT_MAX_RESTARTS = 3`, `DEFAULT_WINDOW_SECONDS = 60`, and `DEFAULT_MAX_GAP_SECONDS = 300`. `_chain_gap` is the max of the window and the max gap, so with these defaults the link between two boots is 300 seconds. `check_and_record` returns true when the chain length reaches 3, and the docstring says the caller skips auto-resume. `_load_boots` returns an empty list on `OSError`, `ValueError`, or `TypeError`. The module docstring says an I/O failure fails open.
>
> **Adopted.** The interval of 30 seconds. Idle stale at 15 cycles, 450 seconds. In a tool, 40 cycles, 1200 seconds. Progress is an iteration change, a tool change, or `last_activity_ts` moving forward, and that progress means the child is not stale. The restart constants 3, 60, and 300. The chain trips at length 3. An I/O failure records nothing and does not trip. A cancelled call and a budget of zero do not start the child.
>
> **Adapted.** `is_stale` is a pure function. It divides elapsed seconds by 30 and compares. It does not schedule `tick`, it does not sleep, and it does not start a thread. The longer cap is `previous.in_tool`, a bool stored on the pulse, where Hermes uses the truth of `current_tool` on this tick. Iteration progress is `!=`, where Hermes uses `>`. The boot log is a `Mutex<Vec<u64>>` plus an `AtomicU32`. There is no `restart_loop.json`. `now` is an argument. The child is `FakeModel`. One failure retries once. `is_stale` and `restart_child` do not call each other.
>
> **Deferred.** The periodic scheduler, the parent `_touch_activity` touch, the `settled` event, and `_STALE_RESULT_GRACE_SECONDS` of 2.0. The persisted boot file, `_MAX_STORED_BOOTS` of 50, and a real child process. `DEFAULT_MAX_ITERATIONS` of 250 is the line above the heartbeat constants. This module does not store it. Wiring a stale result into `record` waits. Chapter 37 is the server pool, on the other lineage.

## What this chapter decides

`is_stale` returns true when nothing has progressed and the elapsed time covers the cap. Idle uses `HEARTBEAT_STALE_CYCLES_IDLE`, 15, and the product with `HEARTBEAT_INTERVAL_SECS` is 450. Inside a tool the cap is `HEARTBEAT_STALE_CYCLES_IN_TOOL`, 40, and the product is 1200. If the iteration differs, the tool differs, or the activity timestamp is newer, the function returns false before it looks at the cap. It leaves the pulse unchanged.

`record` walks boot stamps from newest to oldest. A gap wider than 300 seconds ends the chain, and the new stamp is appended. The return is true when the chain's length is at least 3. `fail_io` makes `record` return false and leave the counter at 0.

`restart_child` checks the flag and the budget first. A failure calls `record`, and when that trips the error is `Breaker` and the retry is skipped. Otherwise the child runs once more. The function leaves `is_stale` to its caller.

`demo` fails once, succeeds on the retry, and returns `restarted: ok`. No key is read.

## 36.1 The restart constants and the two error types

The restart half of the module opens with three public constants and two error types. The heartbeat constants sit lower in the same file, because a chain of boots and a quiet child are two decisions.

```rust
/// Hermes `DEFAULT_MAX_RESTARTS`.
pub const MAX_RESTARTS: u32 = 3;
/// Hermes `DEFAULT_WINDOW_SECONDS`. The gap is floored by this.
pub const WINDOW_SECONDS: u64 = 60;
/// Hermes `DEFAULT_MAX_GAP_SECONDS`. Boots farther apart are a new chain.
pub const MAX_GAP_SECONDS: u64 = 300;
```

`MAX_RESTARTS` is 3, a `u32`, matching `DEFAULT_MAX_RESTARTS`. `WINDOW_SECONDS` is 60 and `MAX_GAP_SECONDS` is 300, both `u64`, because `record` subtracts timestamps with `saturating_sub` on `u64`. Hermes stores the window and the gap as `int` and computes `_chain_gap` as `float(max(1, window_seconds, max_gap_seconds))`. With these two numbers the max is 300 either way, and the extra `max(1, ...)` in Hermes matters only if both inputs were 0. This file never passes 0. `record` writes the gap as `WINDOW_SECONDS.max(MAX_GAP_SECONDS)`, which is 300.

`RestartError` is what `restart_child` returns in `Err`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartError {
    Failed,
    Cancelled,
    Budget,
    Breaker,
}
```

`Copy` means a test can compare the error with `assert_eq!` without moving it out of the `Result`. The four variants are the four stops. `Failed` is a second `complete` that also failed. `Cancelled` is the flag, `Budget` is a budget of zero, and `Breaker` is a chain that has reached 3. There is no variant for a stale pulse. A stale pulse is a `bool` from `is_stale`, and the supervisor is what turns that bool into a restart.

`ChildFailed` is the scripted failure inside `FakeModel`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildFailed;
```

A unit struct has no fields, and it exists so `Result<String, ChildFailed>` has an error type. `restart_child` maps that error onto `RestartError`. Callers outside `complete` see `ChildFailed` in the script they hand to `FakeModel::script`, and the `Result` that `restart_child` returns speaks `RestartError` instead.

> **Rust sidebar: a unit struct is a value**
>
> `ChildFailed` is both a type and a value, so `Err(ChildFailed)` builds the error with no parentheses, because there are no fields to fill. `#[derive(Copy)]` is legal here because nothing on the value lives on the heap, and copying the error copies the whole variant. `RestartError` and `ChildFailed` are different types, so a `match` on one rejects the other, and the conversion is the `map_err` and the `Err(...)` lines in `restart_child`.

## 36.2 `FakeModel` scripts a child, and `Call` holds the flag and the budget

`FakeModel` is the child. The module comment says `complete` never opens a socket, and the struct holds a queue and a counter.

```rust
#[derive(Debug)]
pub struct FakeModel {
    results: VecDeque<Result<String, ChildFailed>>,
    pub calls: u32,
}
```

`results` is private, so tests can't push onto it after construction, and they can read `calls` because that field is `pub`. That public counter is how `a_failed_child_is_restarted_once` asserts two calls, and how `the_third_failure_in_one_supervisor_stops_auto_resume` asserts that the success line was never popped.

```rust
impl FakeModel {
    pub fn script(results: impl IntoIterator<Item = Result<String, ChildFailed>>) -> Self {
        Self {
            results: results.into_iter().collect(),
            calls: 0,
        }
    }

    pub fn complete(&mut self) -> Result<String, ChildFailed> {
        self.calls += 1;
        self.results.pop_front().unwrap_or(Err(ChildFailed))
    }
}
```

`script` takes anything that can become an iterator of results, and the tests pass an array, `[Err(ChildFailed), Ok("ok".into())]`. `into_iter` yields the two items, `collect` builds the `VecDeque`, and `calls` starts at 0. `complete` needs `&mut self` because it pops, and the increment happens before the pop. A scripted failure and an empty queue both count as a call. `pop_front` returns `Option`, and `unwrap_or(Err(ChildFailed))` turns `None` into a failure, so an empty queue fails the call and the function keeps running. A script of one `Ok` followed by a second `complete` fails on that second call, because the queue is then empty. `restart_child` calls `complete` at most twice, so a script of two items is the shape the retry uses.

`Call` is the cancellation flag and the budget. It is this module's struct. `restart_child` reads `Call`, and the copied crate still has chapter 9's `CancelFlag` sitting unused.

```rust
pub struct Call {
    cancelled: AtomicBool,
    budget: AtomicU32,
}

impl Call {
    pub fn new(budget: u32) -> Self {
        Self {
            cancelled: AtomicBool::new(false),
            budget: AtomicU32::new(budget),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub fn budget(&self) -> u32 {
        self.budget.load(Ordering::Relaxed)
    }
}
```

`new` stores the budget and clears the flag. `cancel` takes `&self`, not `&mut self`, because an atomic store doesn't need a mutable borrow, so a supervisor can hold `&Call` and still cancel. Nothing in this file decrements the budget. `budget()` loads the value `new` stored, a budget of zero stays zero, and a budget of one stays one, including across the retry. The check in `restart_child` is `budget() == 0`.

> **Rust sidebar: `Ordering::Relaxed`**
>
> `AtomicBool` and `AtomicU32` are shared values, `store` and `load` are the writes and reads, and `Relaxed` means this operation adds no ordering fence for other memory. `cancel` and `cancelled` only care about the flag itself, and the tests call them on one thread and then call `restart_child`, so that thread sees its own store. The boot stamps live in a `Mutex`, which is the next section, and the flag, the budget, and the stamp vec stay three separate pieces of state.

## 36.3 `Supervisor::record` is the chain

`Supervisor` keeps the stamps, the restart counter, and the I/O flag.

```rust
#[derive(Debug)]
pub struct Supervisor {
    boots: Mutex<Vec<u64>>,
    restarts: AtomicU32,
    io_failed: AtomicBool,
}
```

`new` starts with an empty vec, a counter of 0, and `io_failed` false. `fail_io` stores true on the flag, `restarts` loads the counter, and the method that implements the breaker is `record`.

```rust
pub fn record(&self, now: u64) -> bool {
    if self.io_failed.load(Ordering::Relaxed) {
        return false;
    }
    let gap = WINDOW_SECONDS.max(MAX_GAP_SECONDS);
    let mut boots = self.boots.lock().expect("boots");
    let mut chain = Vec::new();
    let mut prev = now;
    for stamp in boots.iter().rev() {
        if prev.saturating_sub(*stamp) > gap {
            break;
        }
        chain.push(*stamp);
        prev = *stamp;
    }
    chain.reverse();
    chain.push(now);
    *boots = chain;
    self.restarts.fetch_add(1, Ordering::Relaxed);
    boots.len() as u32 >= MAX_RESTARTS
}
```

The first branch is fail-open. If `fail_io` has been called, `record` returns false immediately, and it skips the lock, the append, and the `fetch_add`. Three calls after `fail_io` leave `restarts()` at 0. Hermes's `_load_boots` returns `[]` when the read fails, and the module docstring says an I/O failure fails open, never wedging. This flag is that branch, without a file. A caller who wanted a missing file to trip the breaker gets the open result, which is the one the Python returns.

The catalog describes the breaker as 3 restarts in 60 seconds. The file chains on `DEFAULT_MAX_GAP_SECONDS` of 300, and the gap this file uses is `max(60, 300)`, so 300. That is the moment the catalog and the file disagree. `gap` is that max, and the loop walks `boots` from the back, newest first, because each successful `record` appends `now` at the end. `prev` starts as the new timestamp. `saturating_sub` subtracts and stops at 0 when the stamp is ahead of `prev`. A difference greater than 300 breaks, so older stamps stay out of `chain`. A difference of 300 doesn't break, because the test is `>`, matching Hermes's `prev - t > gap` in `_chain_ending_at`.

The walk pushes onto `chain` newest-first, `reverse` puts them oldest-first, and `push(now)` appends the new boot. `*boots = chain` replaces the vec. Stamps that fell off the gap are dropped, and they aren't kept beside the chain. Hermes keeps a forensic log of up to `_MAX_STORED_BOOTS` (50) and returns the chain for the verdict. This vec becomes the chain itself, with no second buffer of 50.

`fetch_add(1)` counts the record even when a wide gap reset the chain. The return value is separate, `boots.len() >= 3`, and the length includes `now`. The counter and the length answer different questions. `restarts()` is how many times `record` passed the I/O check, and the bool is whether this chain is long enough to skip auto-resume.

One supervisor, with the gap at 300, produces these worked stamps.

| Call | `now` | Vec after the call | Return | `restarts()` |
|---|---|---|---|---|
| 1 | 0 | `[0]` | false | 1 |
| 2 | 10 | `[0, 10]` | false | 2 |
| 3 | 20 | `[0, 10, 20]` | true | 3 |

The gaps are 10 and 10, both inside 300, so the third call is the trip. That row is `three_chained_restarts_trip_and_a_wide_gap_does_not`'s first supervisor.

A second supervisor, with a wide gap, produces these stamps.

| Call | `now` | Vec after the call | Return | `restarts()` |
|---|---|---|---|---|
| 1 | 0 | `[0]` | false | 1 |
| 2 | 301 | `[301]` | false | 2 |

`301 - 0` is 301, and `301 > 300`, so the walk breaks before pushing `0`. The vec holds only the new stamp, and the length of 1 makes the bool false. `restarts()` is 2 because both calls incremented, and the test asserts that counter. Three separate supervisors, each recording once, also leave the breaker quiet, because the chain is the vec inside one `Supervisor`.

`lock().expect("boots")` panics if the mutex is poisoned, and poison means a previous holder panicked while the guard was alive. The tests don't panic inside the lock. `record` takes `&self`, and the mutex is how a shared supervisor can record without `&mut self`.

> **Rust sidebar: the mutex is the vec, the atomic is the count**
>
> `Vec<u64>` isn't an atomic, so updating it means `lock`, then the change, then the guard drop at the end of `record`, which unlocks. `fetch_add` returns the old value, this function ignores that old value, and the tests read the counter later with `restarts()`. `boots.len() as u32` casts the length, the chain is a handful of stamps well inside `u32`, and the comparison is `>= MAX_RESTARTS`, so length 3 trips and length 2 doesn't.

## 36.4 `restart_child` retries once, and `demo` prints the retry

`restart_child` is the function the bench leaves uncalled. The module comment says the record is the bookkeeping and `complete` is the work.

```rust
pub fn restart_child(
    child: &mut FakeModel,
    call: &Call,
    supervisor: &Supervisor,
    now: u64,
) -> Result<String, RestartError> {
    if call.cancelled() || call.budget() == 0 {
        return Err(if call.cancelled() {
            RestartError::Cancelled
        } else {
            RestartError::Budget
        });
    }
    match child.complete() {
        Ok(text) => Ok(text),
        Err(ChildFailed) => {
            if supervisor.record(now) {
                return Err(RestartError::Breaker);
            }
            if call.cancelled() {
                return Err(RestartError::Cancelled);
            }
            child.complete().map_err(|_| RestartError::Failed)
        }
    }
}
```

The gate is first. A cancelled flag, or a budget of zero, returns before `complete`. If both are true, the inner `if call.cancelled()` wins, so the error is `Cancelled`. The budget arm runs when the flag is clear and the budget is zero. `cancel_and_an_empty_budget_do_not_start_the_child` covers the two cases on two children, and both leave `calls` at 0. `record` stays off this path, so a cancelled call doesn't grow the chain.

A first `complete` that returns `Ok` is returned as-is, and `record` stays quiet, because a successful child is a finished call. The tests script a failure first, so this arm is the retry's success, reached from the bottom of the function. It is also the path a healthy first call would take.

A first `Err(ChildFailed)` records one boot at `now`. If `record` returns true, the function returns `Breaker` and skips the second `complete`. That is the third failure in `the_third_failure_in_one_supervisor_stops_auto_resume`. Two earlier `restart_child` calls have each failed once and then succeeded, recording stamps 0 and 1. The third child's first `complete` fails, `record(2)` sees a chain of three, and the `Ok("should-not-run")` still sitting in the queue is never popped. `calls` is 1, and the success string is the assertion that the retry didn't run.

If `record` returns false, the function checks the flag again. A cancel that arrives after the first failure and before the retry returns `Cancelled` and skips the second `complete`. The tests cancel before the first call. This second check is the branch the source keeps for a cancel that lands between the two calls. The retry itself is `child.complete().map_err(|_| RestartError::Failed)`. `Ok` passes through, and `Err` becomes `Failed`, whether the script said `ChildFailed` or the queue was empty. There is no third `complete` in this function. One failure gets one more call, unless the breaker or the second cancel check says otherwise.

`demo` builds that one-failure script.

```rust
pub fn demo() -> String {
    let mut child = FakeModel::script([Err(ChildFailed), Ok("ok".into())]);
    let call = Call::new(1);
    let supervisor = Supervisor::new();
    match restart_child(&mut child, &call, &supervisor, 0) {
        Ok(text) => format!("restarted: {text}"),
        Err(_) => "failed".into(),
    }
}
```

Budget 1, a clear flag, an empty supervisor, and `now` of 0 are the demo's inputs. The first `complete` fails. `record(0)` stores `[0]`, the length is 1, and the return is false, so the second `complete` returns `"ok"` and the format string is `restarted: ok`. An `Err` would print `failed`, and this input doesn't take that arm. No key is read, and `is_stale` stays uncalled.

> **Rust sidebar: `map_err` keeps the `Ok`**
>
> `complete` returns `Result<String, ChildFailed>`, and `map_err` replaces the error while leaving `Ok(String)` alone, so the expression matches `Result<String, RestartError>`. `format!("restarted: {text}")` captures `text` by name, the `{text}` slot is the `Ok` string, and `demo` leaves `calls` and the boot stamp out of that line. `Err(_)` discards which `RestartError` it was, so `demo` collapses cancel, budget, breaker, and a second failure into the same word, while the tests match the variant.

## 36.5 `Pulse` and `is_stale` are the heartbeat

The constants at the bottom of `supervision.rs` are the ones from `delegate_tool.py`.

```rust
pub const HEARTBEAT_INTERVAL_SECS: u64 = 30;
pub const HEARTBEAT_STALE_CYCLES_IDLE: u64 = 15;
pub const HEARTBEAT_STALE_CYCLES_IN_TOOL: u64 = 40;
```

The three numbers are 30, 15, and 40. The comments in Hermes spell the products, 15 cycles as 450 seconds and 40 cycles as 1200 seconds. This file stores the cycle counts and the interval, the tests multiply them, `15 * 30` is 450, and `40 * 30` is 1200. Those products aren't a second set of constants.

`Pulse` is one observation of a child.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pulse {
    pub iteration: u32,
    pub tool: Option<String>,
    pub last_activity_ts: u64,
    pub in_tool: bool,
}
```

Every field is public. `iteration` is the child's iteration counter. `tool` is the current tool name, or `None` when there is no tool. `last_activity_ts` is the activity timestamp, in the same units as the `now` argument of `is_stale`, which is seconds in the tests. `in_tool` selects the cap. Hermes stores no separate bool, because `tick` uses `if child_tool`, the current tool name on this observation. Here the cap is the flag on the previous pulse, and the `tool` argument of `is_stale` is only the progress comparison. A caller can pass `Some("terminal")` while `previous.in_tool` is false, and the idle cap still applies. The tests set the flag and the tool name together.

```rust
pub fn is_stale(
    previous: &Pulse,
    now: u64,
    iteration: u32,
    tool: Option<&str>,
    activity: u64,
) -> bool {
    let progressed = iteration != previous.iteration
        || tool != previous.tool.as_deref()
        || activity > previous.last_activity_ts;
    if progressed {
        return false;
    }
    let cycles = now.saturating_sub(previous.last_activity_ts) / HEARTBEAT_INTERVAL_SECS;
    let limit = if previous.in_tool {
        HEARTBEAT_STALE_CYCLES_IN_TOOL
    } else {
        HEARTBEAT_STALE_CYCLES_IDLE
    };
    cycles >= limit
}
```

`progressed` is the comment in `delegate_tool.py`, where progress is iteration, `current_tool`, or `last_activity_ts`, and the Rust form is three comparisons. `iteration != previous.iteration` is any change, including a decrease, while Hermes's tick uses `child_iter > last_seen["iter"]`, a strictly newer count. A lower iteration counts as progress here, and it would leave Hermes's counter alone. `tool != previous.tool.as_deref()` compares `Option<&str>` with `Option<&str>`, and `as_deref` borrows the `String` inside `previous.tool` as `&str` without cloning. `Some("terminal")` against `Some("terminal")` is equal, and `Some("terminal")` against `None` or against `Some("read_file")` is progress. `activity > previous.last_activity_ts` is a newer timestamp, and an equal timestamp isn't progress. Hermes also treats a missing previous timestamp as advanced. This function has no missing timestamp, because `last_activity_ts` is a `u64` and the caller always has one.

If any of the three is true, the function returns false, the child isn't stale, and the rest of the body doesn't run. That is the slow-model case the Hermes comment describes, where an in-flight model wait refreshes `last_activity_ts`, so a slow model isn't idle. `a_moved_activity_timestamp_is_not_idle` passes `activity` equal to `now`, and `now` is `30 * 40`, far past both caps, with the iteration and the tool unchanged. The newer timestamp makes `progressed` true, and the return is false.

When nothing has progressed, the elapsed time is `now.saturating_sub(previous.last_activity_ts)`. If `now` is behind the stored timestamp, the difference is 0, and integer division by `HEARTBEAT_INTERVAL_SECS` (30) is the cycle count. `449 / 30` is 14, and `450 / 30` is 15. The function is that division, so it neither loops nor adds one per imagined tick. One call with a `now` of 450 is 15 cycles. Hermes increments `last_seen["stale"]` once per scheduled tick and compares with `>=`. The boundary matches. Fifteen frozen cycles, or 450 seconds of a frozen timestamp at a 30 second interval, is stale, and the implementation is the division.

`limit` is 40 when `previous.in_tool` is true and 15 otherwise, and `cycles >= limit` is the verdict. At exactly 15 idle cycles the child is stale, and at 14 it isn't. At 15 cycles inside a tool the child isn't stale, because 15 is below 40. At 40 cycles inside a tool the child is stale. The idle test uses `HEARTBEAT_INTERVAL_SECS * (HEARTBEAT_STALE_CYCLES_IDLE - 1)` as the not-yet time and the product without the `- 1` as the due time. The in-tool test uses the idle product first and expects false, then the in-tool product and expects true.

The function returns that comparison and leaves `previous` unchanged, so it never calls `record`, `restart_child`, `thread::sleep`, or `std::thread::spawn`. A reader who treats the constants as a deadline this function waits out will find the numbers in the comparison and no wait. `_Heartbeat.start` in Hermes is the schedule, and this file stops at the predicate `tick` computes. The only bench is `restart_bookkeeping` at 29.8 ns, because this check is a comparison. A bench of it would measure an integer divide, and `449 / 30` is 14 while `450 / 30` is 15.

> **Rust sidebar: `Option<&str>` and integer division**
>
> `tool` is `Option<&str>` so the caller can pass `Some("terminal")` or `None` without allocating, while `previous.tool` is `Option<String>` because the pulse owns the name it stored, and `as_deref()` is the borrow that makes the two options comparable. `/` on `u64` truncates toward zero, so a remainder of 29 seconds doesn't round up to another cycle, and the tests use exact multiples, which leaves the remainder at 0. `||` stops at the first true clause, so a changed iteration skips the tool and the timestamp, and the return is still false.

## Tests

`tests/supervision.rs` is the new file. The package carries the earlier harness suites, because the crate is a copy, and the assertions for this chapter are the functions in that file.

`a_failed_child_is_restarted_once` scripts `Err(ChildFailed)` then `Ok("ok")`, with budget 1 and `now` 0, and the result is `"ok"`, with `calls` at 2 and `restarts()` at 1. The second complete is the retry, the chain length is 1, and the breaker doesn't trip.

`three_chained_restarts_trip_and_a_wide_gap_does_not` records 0, 10, and 20 on one supervisor, and the third returns true. A fresh supervisor records 0 and then `MAX_GAP_SECONDS + 1`, which is 301, the second returns false, and `restarts()` on that fresh supervisor is 2.

`an_io_failure_does_not_trip` calls `fail_io` and then `record` three times, each call returns false, and `restarts()` stays 0.

`cancel_and_an_empty_budget_do_not_start_the_child` cancels a budget of 1 and expects `RestartError::Cancelled` with `calls` still 0, and a second child with `Call::new(0)` expects `RestartError::Budget` with `calls` still 0. The same supervisor is reused, neither call records, and the supervisor's counter is unchanged by this test.

`the_third_failure_in_one_supervisor_stops_auto_resume` runs two successful retries, at `now` 0 and `now` 1, then a third child whose queue is `Err(ChildFailed)` followed by `Ok("should-not-run")`. The third result is `Err(RestartError::Breaker)`, `calls` is 1, and the success string stays in the queue.

`the_demo_restarts` asserts `demo()` equals `restarted: ok`.

`an_idle_child_is_stale_at_fifteen_cycles` builds a pulse with iteration 1, no tool, `last_activity_ts` 0, and `in_tool` false. At `30 * 14` seconds `is_stale` is false. At `30 * 15` seconds, with the same iteration, no tool, and activity still 0, `is_stale` is true.

`a_child_inside_a_tool_uses_the_longer_threshold` uses iteration 3, tool `terminal`, and `in_tool` true. At the idle product, `30 * 15`, the result is false. At `30 * 40`, with the same iteration, the same tool, and activity still 0, the result is true.

`a_moved_activity_timestamp_is_not_idle` uses the idle pulse and a `now` of `30 * 40`, and passes that same number as `activity`. The result is false, because the timestamp moved, and the long cap never applies once progress returns.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch36-supervision
```

```text
    Starting 190 tests across 43 binaries
     Summary [   0.393s] 190 tests run: 190 passed, 0 skipped
```

```bash
cargo run -p ch36-supervision --bin ch36-supervision --offline
```

```text
restarted: ok
```

The package has more than one binary, so the run names `ch36-supervision`. That binary is `src/bin/chapter_demo.rs`, and it prints the retry. No key is read.

```bash
cargo bench -p ch36-supervision --bench supervision --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `restart_bookkeeping` | 29.8 ns | the record, excluding the child's work | one `record(0)` on an empty supervisor |

`restart_bookkeeping` is `benches/supervision.rs`, and it uses `iter_batched`. The heartbeat check is a comparison, so it isn't this bench. A bench of it would measure an integer divide, where `449 / 30` is 14 and `450 / 30` is 15. The only bench is `restart_bookkeeping` at 29.8 ns, which times one `record(0)` and leaves `complete` and `is_stale` outside the sample.

The means are `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `36`, with bench ids `36/restart_bookkeeping` and `34/restart_bookkeeping`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The bench is the bookkeeping, so a number that includes `complete`, or includes `is_stale`, is a different function. `is_stale` is covered by the tests, and `restart_bookkeeping` leaves it out.

`is_stale` doesn't tick. Calling it once at 450 seconds is the idle verdict. A loop of 15 calls isn't inside the function, and sleeping 450 seconds isn't inside it either.

The cap follows `previous.in_tool`, and the `tool` argument is the progress check. Hermes picks 40 when `current_tool` on this tick is set. A pulse with `in_tool: false` and a tool name of `terminal` still uses 15 cycles here, and the in-tool test sets the flag.

`record` on one supervisor is the chain, and the counter is a different number. A wide gap leaves `restarts()` at 2 while the vec holds one stamp and the bool is false.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Heartbeat.** Interval 30 seconds. Idle stale at 15 cycles, 450 seconds. In a tool, 40 cycles, 1200 seconds. Progress is a new iteration, a different tool, or a newer `last_activity_ts`, and that progress returns false from `is_stale`. The numbers are `_HEARTBEAT_INTERVAL`, `_HEARTBEAT_STALE_CYCLES_IDLE`, and `_HEARTBEAT_STALE_CYCLES_IN_TOOL` in `tools/delegate_tool.py`. The choice of 40 versus 15 is the `stale_cycles` line in `tools/delegate_tool_child_run.py`. This function does not sleep and does not start a thread.

**Restart loop (J6).** Three chained boots, with no gap wider than 300 seconds, skip auto-resume. `WINDOW_SECONDS` is 60 and floors that gap through `max`. The catalog row describes the breaker as 3 restarts in 60 seconds. The file I opened chains on `DEFAULT_MAX_GAP_SECONDS` of 300, and `_chain_gap` takes the max with the window. An I/O failure does not trip.

**Call.** A cancelled flag or a budget of zero returns before the child runs. The retry checks the flag again. The budget does not decrease.

**Once.** A single failure retries one time. The breaker is what stops a sequence of failures.

**Left for later.** The scheduler, the parent activity touch, the grace poll of 2 seconds, the JSON boot log, and a real process. `is_stale` is not yet an input to `record`. E1 and E2 are the redaction module elsewhere in this copied crate. `supervision.rs` does not call it. The scripted error string is not redacted.

## What you implement

`my-harness/tests/ch36.rs` is the milestone, and the stub stays `todo!`.

```rust
use my_harness::chapter_36_idle_is_stale;

#[test]
fn idle_stale_uses_15_cycles_and_a_tool_uses_40() {
    assert!(chapter_36_idle_is_stale(15, false));
    assert!(!chapter_36_idle_is_stale(15, true));
    assert!(chapter_36_idle_is_stale(40, true));
}
```

Fifteen cycles with `in_tool` false is stale. Fifteen cycles with `in_tool` true isn't stale, and forty cycles with `in_tool` true is stale. That pair is the idle threshold and the longer in-tool threshold.

The stub panics.

```rust
pub fn chapter_36_idle_is_stale(cycles: u64, in_tool: bool) -> bool {
    let _ = (cycles, in_tool);
    todo!("chapter 36")
}
```

`todo!("chapter 36")` is the panic. The arguments are named so the stub compiles, filling the function in is the reader's work, and copying `steps/ch36-supervision` into `my-harness/` isn't the milestone.

## Chapter summary

- **Heartbeat.** `is_stale` compares elapsed time with the cap. Idle is 15 cycles and 450 seconds, a tool uses 40 cycles and 1200 seconds, and the interval is 30 seconds. Progress returns false.
- **Retry.** One failure gets one more `complete`, unless the breaker, the flag, or the budget stops it. That second call is the whole restart.
- **Chain.** The gap is 300 seconds, length 3 trips, and an I/O failure stays open.
- **Split.** `record` leaves the child uncalled, `is_stale` records no boot, and the bench times `record(0)` at 29.8 ns.
- **Demo.** The printed line is `restarted: ok`, and no key is read.

## What's next

Chapter 37 is the server track, a pool of backends, not a copy of this harness. Chapter 38 copies this harness and adds profiles.
