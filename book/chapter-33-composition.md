# Chapter 33: Composing Patterns

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 32 can judge one reply. It takes one permit, locks the notes, and returns. It does not place a fan-out on that same pool, it does not move ownership, and it does not call the critic again. A width that would put more workers in flight than the pool allows has nowhere to be refused. A `Wait` verdict has no session, no pid, and no number of seconds. A shell gate that must pass before the judge runs is still a comment in Hermes.

This chapter composes those patterns. `nested_fanout` refuses when `held` plus `width` would pass `pool.limit()`, and it refuses before any task is scheduled. A width that fits is gathered on the same `Pool`. `dispatch` calls the handoff board only for `Route::Move`. `run_continuation` speaks, runs the gates, and only then calls `as_tool`. A wait with a target parks. Twenty turns stop the loop. The constants 30 and 300 are stored. Nothing in this file sleeps them.

Context files, a shared snapshot, and an actor inbox wait. Chapter 34 scans and freezes context. Chapter 35 gives each agent a mailbox. Chapter 36 restarts a failed child.

The step crate is `steps/ch33-composition`. The package is `ch33-composition`. The library name is `harness_core`. This crate is chapter 32's harness copied forward, plus `composition`. `lib.rs` adds `pub mod composition`. The top comment in `lib.rs` still says chapter 10. `Cargo.toml` describes the package as the previous harness plus this chapter. The `[dependencies]` table lists `tokio`, `regex`, `rusqlite`, and the rest of the copied stack. It does not list `ch32-critic`. There is no path dependency on the previous crate. The pool, the fan-out board, the handoff board, and the critic in this tree are the copies that already live in this crate.

| Section | Step | What it adds |
|---|---|---|
| 33.1 | `steps/ch33-composition` | the four constants, `ComposeError`, `Route`, `WaitTarget`, `LoopStop` |
| 33.2 | `steps/ch33-composition` | `Gate` and `FakeModel` |
| 33.3 | `steps/ch33-composition` | `nested_fanout` refuses past the pool limit |
| 33.4 | `steps/ch33-composition` | `dispatch` hands off only on `Route::Move` |
| 33.5 | `steps/ch33-composition` | `wait_target` prefers a session, then a pid, then seconds |
| 33.6 | `steps/ch33-composition` | `run_continuation` runs gates before `as_tool` |
| 33.7 | `steps/ch33-composition` | `limits`, the demo, and `nested_eight` |

> **How Hermes does it**
>
> I read `hermes_cli/goals.py` (`DEFAULT_MAX_TURNS`, `DEFAULT_JUDGE_TIMEOUT`, `DEFAULT_GATE_TIMEOUT_SECONDS`, `DEFAULT_GATE_MAX_RETRIES`, `_parse_judge_response`) and `tools/delegate_tool_config.py` (`MAX_DEPTH`, `_DEFAULT_MAX_CONCURRENT_CHILDREN`, `_get_max_concurrent_children`, `_get_max_async_children`) on Hermes `main` at `9e01b44`.
>
> `DEFAULT_MAX_TURNS` is 20. `DEFAULT_JUDGE_TIMEOUT` is 30.0. `DEFAULT_GATE_TIMEOUT_SECONDS` is 300. `DEFAULT_GATE_MAX_RETRIES` is 3. The comment above the gate constants says a failed gate short-circuits the judge. `_parse_judge_response` prefers a session, then a pid, then seconds. The session keys it reads are `wait_on_session`, then `session_id`, then `wait_session`. The pid keys are `wait_on_pid`, `pid`, `wait_pid`. The seconds keys are `wait_for_seconds`, `seconds`, `wait_seconds`. A `wait` with none of those returns `"continue"`. `MAX_DEPTH` is 1. `_DEFAULT_MAX_CONCURRENT_CHILDREN` is 10. `_get_max_concurrent_children` reads `delegation.max_concurrent_children`, then the env var `DELEGATION_MAX_CONCURRENT_CHILDREN`, then 10. The floor is 1. The docstring says there is no ceiling. `_get_max_async_children` uses that same cap, and its docstring says a new async dispatch at capacity is rejected.
>
> **Adopted.** The turn cap of 20. The gate retry count of 3. The gate timeout constant of 300 and the judge timeout constant of 30. Gates run before the judge may accept. The wait-target order is session, then pid, then seconds. A wait with no target does not park.
>
> **Adapted.** The nested bound is `pool.limit()`. The tests build that pool with `WORKER_BOUND`, which is 8 in the copied supervisor. Hermes's default concurrent-children cap is 10. `held.saturating_add(width)` is compared with the limit before `FanBoard::new`. A refusal returns `ComposeError::ExceedsBound` and leaves `max_seen` at 0. The gate is a `VecDeque` of scripted exit codes. This file does not spawn a shell and does not sleep 300 or 30. `wait_target` scans the three names `wait_on_session`, `wait_on_pid`, and `wait_for_seconds`. `FakeModel::complete` pops a scripted line. `dispatch` returns `Ok(None)` for `Route::Stay`.
>
> **Deferred.** The alias keys `session_id`, `wait_session`, `pid`, `wait_pid`, `seconds`, and `wait_seconds`. A JSON object parse. Spawning the gate command. Heartbeats, which stay in chapter 36. Context-file scan, truncation, and the untrusted wrap, which are chapter 34. `MAX_DEPTH` stays the copied supervisor's depth check. This chapter does not call `child_call`.

[personal note: a width that looked small until I added the workers I already held]

## What this chapter decides

`nested_fanout` returns `Err(ComposeError::ExceedsBound { held, asked: width, limit })` when `held.saturating_add(width) > pool.limit()`. That return happens before `FanBoard::new` and before `schedule`. A width that fits is queued and then `gather_holding` runs it on the caller's `Pool`.

`dispatch` matches `Route`. `Stay` returns `Ok(None)` and does not call `handoff`. `Move` calls `board.handoff` and wraps the filtered messages in `Some`.

`wait_target` returns a session when `wait_on_session` has a value, otherwise a pid greater than 0, otherwise seconds greater than 0, otherwise `None`.

`run_continuation` stops with `LoopStop::Turns` when `turns >= DEFAULT_MAX_TURNS`, which is 20, and it stops before the next `complete`. After each line, `check_gates` runs. A non-zero exit retries or, once `attempts` passes 3, returns `LoopStop::GatePaused`. `as_tool` runs only on `Gates::Clear`. `Verdict::Done`, `Blocked`, and `Reject` return. `Verdict::Wait` with a target returns `LoopStop::Parked` and does not increment `turns`. `Verdict::Wait` with no target, and `Verdict::Continue`, increment `turns`.

`limits` returns `(20, 30, 300, 3)`. `demo` returns `fan n0 | n1; owner supervisor; done`.

## 33.1 Four constants, and the enums that carry a stop

```rust
pub const DEFAULT_MAX_TURNS: u32 = 20;
pub const DEFAULT_JUDGE_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_GATE_TIMEOUT_SECONDS: u32 = 300;
pub const DEFAULT_GATE_MAX_RETRIES: u32 = 3;
```

These four names are the Hermes constants from `goals.py`, stored as integers. `DEFAULT_MAX_TURNS` is 20, a `u32`. `DEFAULT_JUDGE_TIMEOUT_SECS` is 30, a `u64`, where Hermes stores `DEFAULT_JUDGE_TIMEOUT` as 30.0. `DEFAULT_GATE_TIMEOUT_SECONDS` is 300, a `u32`. `DEFAULT_GATE_MAX_RETRIES` is 3, a `u32`. Chapter 32's `critic::limits` still returns the pair `(20, 30)`. This module's `limits` returns all four. The critic module in this crate is the copy. It still does not loop. The loop that reads 20 lives here.

`Duration` is imported at the top of `composition.rs`. The continuation loop does not construct a `Duration` from 30 or from 300. The only `Duration` this file builds is `Duration::ZERO`, passed by `nested_fanout` into `nested_fanout_holding`. A caller who wants the tasks to overlap passes a non-zero hold into `nested_fanout_holding`. That hold is a sleep inside the copied `gather_holding`, per task, after that task's `acquire`. It is not the judge timeout and it is not the gate timeout.

> **Rust sidebar: `enum`**
>
> - An `enum` is one value that is exactly one of its variants. `Route::Stay` and `Route::Move` cannot both be true.
> - `ComposeError` has one variant, `ExceedsBound`, and that variant holds three `usize` fields: `held`, `asked`, and `limit`.
> - `LoopStop` has variants with no data (`Done`, `Blocked`, `Reject`, `Turns`) and variants with data (`Parked(WaitTarget)`, `GatePaused { command: String }`).
> - `#[derive(Debug, Clone, PartialEq, Eq)]` is what `assert_eq!` and `matches!` in the tests use. `Route` also derives `Copy`, so a route is copied when it is matched. `WaitTarget` does not derive `Copy`, because `Session` owns a `String`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeError {
    ExceedsBound {
        held: usize,
        asked: usize,
        limit: usize,
    },
}
```

`asked` is the width the caller wanted. `held` is the count the caller says it already owns. `limit` is `pool.limit()` at the moment of the check. There is no second error variant. A schedule failure inside a width that passed the check is an `expect` panic, described in 33.3, because the fan-out queue cap is 100 and the pool the tests build has limit 8.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Stay,
    Move,
}
```

`Stay` means the specialist is a tool and the supervisor keeps the conversation. `Move` means ownership should move, and the handoff board filters, then accepts. The comments on the variants are the contract `dispatch` implements.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaitTarget {
    Session(String),
    Pid(u32),
    Seconds(u32),
}
```

`Session` owns the id string. `Pid` and `Seconds` are `u32`. A zero pid and a zero seconds value are not built by `wait_target`. The enum can still hold a zero if a caller constructs `WaitTarget::Pid(0)` directly. The parser does not.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopStop {
    Done,
    Blocked,
    Reject,
    Parked(WaitTarget),
    Turns,
    GatePaused { command: String },
}
```

`Done`, `Blocked`, and `Reject` are the critic verdicts that stop the loop. `Parked` carries the target that was found. `Turns` means the cap of 20 was already reached at the top of the loop. `GatePaused` carries the command string of the gate that ran out of retries. The command is data on the stop value. This function does not execute it.

## 33.2 `Gate` records exits, and `FakeModel` pops a line

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    pub command: String,
    pub exits: VecDeque<i32>,
    pub attempts: u32,
}

impl Gate {
    pub fn scripted(command: impl Into<String>, exits: impl IntoIterator<Item = i32>) -> Self {
        Self {
            command: command.into(),
            exits: exits.into_iter().collect(),
            attempts: 0,
        }
    }
}
```

> **Rust sidebar: `VecDeque`**
>
> - A `VecDeque<i32>` is a queue of exit codes. `pop_front` takes the oldest code. `push_back` would add at the other end. `scripted` only collects into the queue.
> - `impl Into<String>` lets the caller pass a `String` or a `&str`. `command.into()` produces the owned `String` stored on the struct.
> - `impl IntoIterator<Item = i32>` lets the caller pass an array such as `[1, 0]` or an iterator such as `std::iter::repeat_n(1, 4)`. Both become the `exits` queue.
> - `attempts` starts at 0. `check_gates` is the only function that writes it.

`command` is the label that `GatePaused` will clone. The tests use `"cargo test"`. The string is not passed to a process API. I read `composition.rs`. `std::process` is not imported. `Gate` holds the name and the scripted codes.

```rust
pub struct FakeModel {
    lines: VecDeque<String>,
}

impl FakeModel {
    pub fn script(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            lines: lines.into_iter().map(Into::into).collect(),
        }
    }

    pub fn complete(&mut self) -> String {
        self.lines.pop_front().unwrap_or_default()
    }
}
```

`script` collects each item into an owned `String`. `complete` needs `&mut self` because `pop_front` removes a line. The first call returns the first scripted line. The next call returns the next. When the queue is empty, `unwrap_or_default` returns `String::new()`, which is `""`. An empty reply still goes through the gates and, if the gates are clear, through `as_tool`. The copied `parse_verdict` returns `Verdict::Reject` when the lowercased text has no `verdict` substring. A model that has run out of lines does not spin forever on empty strings once the judge is reached. The judge rejects that empty line and the loop returns `LoopStop::Reject`.

`FakeModel` in this module is not the provider in `provider.rs`. It has no socket, no HTTP client, and no API key. `complete` returns the next owned line or `""`.

## 33.3 `nested_fanout` refuses before `schedule`

```rust
pub fn nested_fanout(pool: &Pool, held: usize, width: usize) -> Result<String, ComposeError> {
    nested_fanout_holding(pool, held, width, Duration::ZERO)
}

pub fn nested_fanout_holding(
    pool: &Pool,
    held: usize,
    width: usize,
    hold: Duration,
) -> Result<String, ComposeError> {
    if held.saturating_add(width) > pool.limit() {
        return Err(ComposeError::ExceedsBound {
            held,
            asked: width,
            limit: pool.limit(),
        });
    }
    let board = FanBoard::new();
    for index in 0..width {
        board
            .schedule(Task {
                task_index: index,
                reply: format!("n{index}"),
                tool_trace: String::new(),
            })
            .expect("width is at most the worker bound, under the queue of 100");
    }
    Ok(board.gather_holding(pool, hold))
}
```

> **Rust sidebar: `Result`**
>
> - `Result<String, ComposeError>` is either `Ok(summary)` or `Err(ComposeError)`.
> - `return Err(...)` leaves the function immediately. The lines after the `if` do not run.
> - The tests use `unwrap_err()` on a refusal and `unwrap()` on a width that fits. `unwrap` panics on `Err`. `unwrap_err` panics on `Ok`.
> - `?` is not used here. The `schedule` call uses `expect`, which panics with that string if the queue returns `FanError::Full`.

> **Rust sidebar: `saturating_add`**
>
> - `usize` is an unsigned size. Adding past `usize::MAX` would wrap to a small number with the ordinary `+` operator in release builds, and a wrapped sum could look smaller than `pool.limit()`.
> - `saturating_add` returns the sum, or `usize::MAX` when the sum would not fit. `usize::MAX` is greater than any pool limit this harness builds, so an overflowing pair still takes the `Err` branch.
> - The tests use small numbers: `0 + 9`, `1 + 8`, `0 + 8`, `0 + 2`. Those sums do not overflow. The method is still the comparison the function runs.

`pool.limit()` is the limit stored in `Pool::new`. `Pool::new` keeps `limit.max(1)`, so a constructed pool has a limit of at least 1. `WORKER_BOUND` in the copied supervisor is 8. `Pool::new(WORKER_BOUND)` therefore has `limit()` of 8. `held` is an argument. The function does not read `inflight` and does not call `acquire` on the refusal path. A parent that already owns one permit passes `held: 1`. The pool object can still show `max_seen` of 0, because this check is arithmetic on the arguments and the limit. It is the caller's report of what it holds.

The refusal test builds `Pool::new(WORKER_BOUND)` and calls `nested_fanout(&pool, 0, WORKER_BOUND + 1)`. Width 9 with held 0 is `9 > 8`. The error is `ExceedsBound { held: 0, asked: 9, limit: 8 }`. `pool.max_seen()` is 0. The same pool then receives `nested_fanout(&pool, 1, WORKER_BOUND)`, which is `1 + 8 > 8`. That error is `ExceedsBound { held: 1, asked: 8, .. }`. `max_seen` is still 0. `FanBoard::new` is under the `if`, so a refused call does not allocate the board, does not call `schedule`, and does not call `gather_holding`. Tasks have not started.

When the sum is within the limit, the board is new and empty. The loop `for index in 0..width` schedules `width` tasks. Task `index` gets reply `n{index}` from `format!`, and `tool_trace` is an empty `String`. The copied `schedule` returns `Err(FanError::Full)` when the queue length is already `QUEUE_CAP`, and `QUEUE_CAP` in `fanout.rs` is 100. A width that passed a pool limit of 8 is at most 8, which is under 100, so `expect` does not fire for the pools the tests build. `Pool::new(200)` would pass `held: 0` and `width: 200` through this check, and the 101st `schedule` would panic on `expect`. The tests and the demo use `WORKER_BOUND`.

`gather_holding` drains the queue and spawns one scoped thread per task. Each thread calls `pool.acquire()`, sleeps `hold` when `hold` is not zero, and returns `(task_index, reply)`. The replies are sorted by `task_index` and joined with `\n`. `tool_trace` is not in that string. `nested_fanout` then wraps the string in `Ok`.

The overlap test calls `nested_fanout_holding(&pool, 0, WORKER_BOUND, Duration::from_millis(40))`. The summary is `n0\nn1\nn2\nn3\nn4\nn5\nn6\nn7`. `pool.max_seen()` equals `WORKER_BOUND`, 8. The 40 milliseconds are long enough that the eight acquires overlap and the high-water mark reaches 8. A second pool, called through `nested_fanout` so the hold is `Duration::ZERO`, returns the same summary string. That assertion is `quiet.max_seen() <= WORKER_BOUND`. The zero hold still acquires inside `gather_holding`. The test allows the high-water mark to be 8 or lower, because a task can finish and drop its permit before the next one is observed. Both calls use the pool they were given. Neither builds a second pool type.

`acquire` on the copied pool waits on a `Condvar` while `inflight >= limit`. The pre-check exists so a nested width that would pass the bound returns `Err` instead of entering that wait. Hermes's `_get_max_async_children` docstring says a new async dispatch at capacity is rejected. The shape here is the `Result` above. The number compared is this pool's limit.

## 33.4 `dispatch` moves ownership only for `Route::Move`

```rust
pub fn dispatch(
    route: Route,
    board: &HandoffBoard,
    next: &str,
    messages: &[Message],
) -> Result<Option<Vec<Message>>, HandoffError> {
    match route {
        Route::Stay => Ok(None),
        Route::Move => board.handoff(next, messages).map(Some),
    }
}
```

> **Rust sidebar: `match`**
>
> - `match` must name every variant of `Route`. Adding a third variant later is a compile error until this match handles it.
> - `Ok(None)` is a successful `Result` whose inner `Option` is `None`. Success and "no handoff" are different layers. `None` means the board was not asked. `Err` would mean `handoff` failed.
> - `.map(Some)` converts `Result<Vec<Message>, HandoffError>` into `Result<Option<Vec<Message>>, HandoffError>`. `Ok(messages)` becomes `Ok(Some(messages))`. `Err` stays `Err`.

`Route::Stay` returns before `board.handoff`. The copied `handoff` filters the transcript, rejects an empty filtered list with `HandoffError::Empty`, increments a counter, rejects a fifth move with `HandoffError::Limit` (`MAX_HANDOFFS` is 4), and on success sets the owner to `next`. `Stay` does none of that. The test builds `HandoffBoard::new("supervisor")` with a user message and a scratch message. `dispatch(Route::Stay, &board, "billing", &messages)` is `Ok(None)`. `board.owner()` is still `"supervisor"`. `board.count()` is 0.

`Route::Move` on the same board and the same messages returns `Ok(Some(moved))`. `moved.len()` is 1. `filter_transcript` drops `role == "scratch"`, so the private scratch line is not in the vec the next owner receives. `board.owner()` is `"billing"`. The supervisor kept the conversation on `Stay`. Ownership changed on `Move`.

`dispatch` does not catch `HandoffError`. A `Move` whose messages are all scratch returns `Err(Empty)` from `handoff`, still as `Err` after `map`. The fifth `Move` on one board returns `Err(Limit)`, because `current >= MAX_HANDOFFS` after four accepted handoffs. The feature test stops at one `Move`. The error variants are the ones `handoff` already returns. This function adds the `Stay` path that never calls it.

## 33.5 `wait_target` reads a session, then a pid, then seconds

`field` is private. `wait_target` is the public parser.

```rust
fn field<'a>(raw: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{key}\"");
    let rest = raw.split_once(&needle)?.1.trim_start();
    let rest = rest.trim_start_matches([':', ' ', '"', '=']);
    let end = rest
        .find(|ch: char| ch == '"' || ch == ',' || ch == '}' || ch.is_whitespace())
        .unwrap_or(rest.len());
    let value = rest[..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub fn wait_target(raw: &str) -> Option<WaitTarget> {
    if let Some(session) = field(raw, "wait_on_session") {
        return Some(WaitTarget::Session(session.to_string()));
    }
    if let Some(pid) = field(raw, "wait_on_pid").and_then(|text| text.parse().ok()) {
        if pid > 0 {
            return Some(WaitTarget::Pid(pid));
        }
    }
    if let Some(seconds) = field(raw, "wait_for_seconds").and_then(|text| text.parse().ok()) {
        if seconds > 0 {
            return Some(WaitTarget::Seconds(seconds));
        }
    }
    None
}
```

> **Rust sidebar: `Option`**
>
> - `Option<&str>` is `Some(slice)` or `None`. `split_once` returns `None` when the needle is absent, and `?` then returns `None` from `field`.
> - `if let Some(session) = ...` runs the block only when the value is present. A missing session falls through to the pid check.
> - `and_then` chains another `Option`. `text.parse().ok()` is `Some(number)` when the slice parses as the inferred integer type, and `None` when it does not. A failed parse skips that arm.
> - The lifetime `'a` on `field` means the returned `&str` points into `raw`. `wait_target` calls `session.to_string()` before returning, so `WaitTarget::Session` owns its id after `raw` is gone.

The needle is a quoted key: `"wait_on_session"`, with the quote characters included. `split_once` splits on the first match. The suffix is trimmed of leading whitespace, then of a prefix made of `:`, space, `"`, and `=`. The value ends at the next `"`, comma, `}`, or whitespace. An empty slice becomes `None`.

`{"verdict": "wait", "wait_on_session": "sess-9"}` yields `WaitTarget::Session("sess-9")`. The function returns inside the first `if let`. A pid or a seconds field later in the same text is not read once a session value exists.

`{"verdict": "wait", "wait_on_pid": 44}` has no session field. `field` returns `"44"`. `parse` succeeds. `44 > 0`, so the result is `WaitTarget::Pid(44)`. `{"verdict": "wait", "wait_for_seconds": 12}` is `WaitTarget::Seconds(12)`. `{"verdict": "wait"}` matches none of the three names. `wait_target` returns `None`.

A pid of 0 parses and fails `pid > 0`. That arm does not return. Execution continues to the seconds check. A pid that does not parse fails the `if let` and also continues. The same shape applies to seconds: zero or a bad parse falls through to `None`. Hermes's `_first_int` likewise keeps a value only when `iv > 0`, and it tries alias keys this function does not look up. `session_id`, `wait_session`, `pid`, `wait_pid`, `seconds`, and `wait_seconds` are absent from `field`'s call sites. A reply that uses only those aliases is `None` here.

This scan is not `json`. A key without quotes in the text does not match the needle. The order of the three `if` blocks is the order in `_parse_judge_response`: session, then pid, then seconds.

## 33.6 The loop runs the gates, then the critic

`Gates` and `check_gates` are private. `run_continuation` is the public loop.

```rust
enum Gates {
    Clear,
    Retry,
    Paused(String),
}

fn check_gates(gates: &mut [Gate]) -> Gates {
    for gate in gates.iter_mut() {
        let code = gate.exits.pop_front().unwrap_or(0);
        if code == 0 {
            gate.attempts = 0;
            continue;
        }
        gate.attempts += 1;
        if gate.attempts > DEFAULT_GATE_MAX_RETRIES {
            return Gates::Paused(gate.command.clone());
        }
        return Gates::Retry;
    }
    Gates::Clear
}

pub fn run_continuation(model: &mut FakeModel, pool: &Pool, gates: &mut [Gate]) -> LoopStop {
    let mut turns = 0u32;
    loop {
        if turns >= DEFAULT_MAX_TURNS {
            return LoopStop::Turns;
        }
        let reply = model.complete();
        match check_gates(gates) {
            Gates::Paused(command) => return LoopStop::GatePaused { command },
            Gates::Retry => {
                turns += 1;
                continue;
            }
            Gates::Clear => {}
        }
        let outcome = as_tool(pool, &reply);
        match outcome.verdict {
            Verdict::Done => return LoopStop::Done,
            Verdict::Blocked => return LoopStop::Blocked,
            Verdict::Reject => return LoopStop::Reject,
            Verdict::Wait => match wait_target(&reply) {
                Some(target) => return LoopStop::Parked(target),
                None => turns += 1,
            },
            Verdict::Continue => turns += 1,
        }
    }
}
```

`check_gates` walks the slice from front to back. Each gate pops one exit code. An empty `exits` queue uses `unwrap_or(0)`, so a gate with no remaining codes is treated as success. A code of 0 sets `attempts` back to 0 and `continue`s to the next gate. A non-zero code increments `attempts`. When `attempts` is 1, 2, or 3, `attempts > 3` is false, and the function returns `Gates::Retry` immediately. Later gates in the slice are not popped on that call. When `attempts` becomes 4, `4 > DEFAULT_GATE_MAX_RETRIES` is true, and the function returns `Gates::Paused` with `command` cloned. Three failures are retries. The fourth failure pauses. The test name says three retries, then a pause. The script length is `DEFAULT_GATE_MAX_RETRIES as usize + 1`, which is 4.

An empty gate slice never enters the `for` loop. The result is `Gates::Clear`.

`run_continuation` starts `turns` at 0. The first statement in the loop compares `turns` with 20. The model is not called when that comparison is already true. Then `complete` pops one line. Then the gates run. `Gates::Paused` returns `LoopStop::GatePaused` before `as_tool`. `Gates::Retry` increments `turns`, `continue`s, and also skips `as_tool`. The popped reply is dropped. The comment on `run_continuation` says a failing gate does not call the judge. The copied `as_tool` calls `judge`, and `judge` calls `pool.acquire`. Skipping `as_tool` means the refusal path of a red gate does not take a permit.

On `Gates::Clear`, `as_tool(pool, &reply)` runs. `Verdict::Done` returns `LoopStop::Done`. `Blocked` returns `LoopStop::Blocked`. `Reject` returns `LoopStop::Reject`. Those three do not increment `turns`. The cap is checked at the start of the next iteration, which does not happen after a return.

`Verdict::Wait` calls `wait_target(&reply)`. `Some(target)` returns `LoopStop::Parked(target)` and does not execute `turns += 1`. A parked wait consumed a model line and a judge permit, and it did not spend a turn against the cap of 20. `None` falls into `turns += 1` and the loop goes around. Hermes downgrades a target-less `wait` to `"continue"` inside `_parse_judge_response`. This loop leaves the verdict as `Wait` and spends a turn. The next scripted line is judged on the next iteration. `Verdict::Continue` spends a turn the same way.

The accept-and-park test checks `limits()` first: 20, 30, 300, 3. A model scripted with `verdict: continue` then `verdict: done` returns `LoopStop::Done`. The continue spends one turn. The done returns. A model scripted with `{"verdict": "wait", "wait_on_session": "sess-9"}` returns `LoopStop::Parked(WaitTarget::Session("sess-9"))`. The same test calls `wait_target` on the pid line and the seconds line and on `{"verdict": "wait"}`, which is `None`. A model scripted with `verdict: wait` then `verdict: done` returns `LoopStop::Done`. The bare wait has no target, so it spends a turn, and the next line is the accept. `parse_verdict` sees the substring `verdict` and a suffix that starts with `wait`, so the first line is `Verdict::Wait`, not `Reject`.

The gate test uses `Gate::scripted("cargo test", [1, 0])` and a model with `ignored while the gate is red` then `verdict: done`. The first code is 1, so `attempts` becomes 1 and the loop retries. The red line is consumed and never judged. The second iteration pops 0, resets `attempts` to 0, sees `Clear`, and `as_tool` accepts `verdict: done`. The result is `LoopStop::Done`. `gates[0].attempts` is 0 because the successful pop cleared it. `pool.max_seen()` is 1 because the accepting `as_tool` acquired one permit. A fresh pool with four failures of `cargo test`, and a model of four `verdict: done` lines, returns `LoopStop::GatePaused { command: "cargo test" }`. `fresh.max_seen()` is 0. The four done-lines were popped by `complete` and discarded. The judge never ran. After three retries, `turns` is 3, which is under 20, and the fourth failure pauses inside `check_gates`.

`twenty_continues_stop_the_loop` scripts 21 copies of `verdict: continue`. Each clear continue calls `as_tool` and then increments `turns`. After the twentieth continue, `turns` is 20. The next loop iteration hits `turns >= DEFAULT_MAX_TURNS` and returns `LoopStop::Turns` before `complete` pops the twenty-first line. The cap is the check at the top, not a special case inside the `Continue` arm.

`a_missing_verdict_rejects` scripts `looks finished`. The gates slice is empty, so `as_tool` runs. `parse_verdict` finds no `verdict` and returns `Reject`. The loop returns `LoopStop::Reject`.

`Blocked` is a return arm in the same `match`. The feature tests reach `Done`, `Parked`, `Turns`, `GatePaused`, and `Reject`. The `Blocked` arm is the same shape as `Done` and `Reject`: return that stop, do not increment `turns`.

## 33.7 `limits` returns the four numbers, and the demo stays

```rust
pub fn limits() -> (u32, u64, u32, u32) {
    (
        DEFAULT_MAX_TURNS,
        DEFAULT_JUDGE_TIMEOUT_SECS,
        DEFAULT_GATE_TIMEOUT_SECONDS,
        DEFAULT_GATE_MAX_RETRIES,
    )
}

pub fn demo() -> String {
    let pool = Pool::new(WORKER_BOUND);
    let gathered = nested_fanout(&pool, 0, 2)
        .expect("two children fit")
        .replace('\n', " | ");
    let board = HandoffBoard::new("supervisor");
    let messages = [Message {
        role: "user".into(),
        text: "ship the note".into(),
    }];
    let stayed = dispatch(Route::Stay, &board, "billing", &messages).expect("stay");
    assert!(stayed.is_none());
    let mut model = FakeModel::script(["verdict: done"]);
    let stop = run_continuation(&mut model, &pool, &mut []);
    let label = match stop {
        LoopStop::Done => "done",
        _ => "not-done",
    };
    format!("fan {gathered}; owner {}; {label}", board.owner())
}
```

`limits` is the tuple `(20, 30, 300, 3)`. The test asserts each field. Nothing in `limits` starts a timer.

`demo` builds a pool of 8. `nested_fanout(&pool, 0, 2)` fits: `0 + 2` is not greater than 8. The gathered string is `n0\nn1`. `replace('\n', " | ")` makes `n0 | n1`. The handoff board starts owned by `supervisor`. The only message is a user line, `ship the note`. `Route::Stay` returns `None`, and `assert!` checks that. `expect("stay")` would panic if `dispatch` returned `Err`. `Stay` returns `Ok(None)`, so `expect` yields `None`. The owner is unchanged because `handoff` was not called.

The model has one line, `verdict: done`. The gate slice is empty. `run_continuation` judges that line, `as_tool` accepts it, and the stop is `LoopStop::Done`. The label is `done`. Any other `LoopStop` would label `not-done`. This input takes the `Done` arm. `format!` reads `board.owner()`, which is still `supervisor`. The returned string is `fan n0 | n1; owner supervisor; done`.

```rust
#[test]
fn the_demo_stays_with_the_supervisor() {
    assert_eq!(harness_core::composition::demo(), "fan n0 | n1; owner supervisor; done");
}
```

The binary prints that line.

```rust
use harness_core::composition::demo;

fn main() {
    println!("{}", demo());
}
```

I ran `cargo run -p ch33-composition --bin ch33-composition --offline`. The process printed `fan n0 | n1; owner supervisor; done`. The package also builds `ch33_mcp_fixture` and `sandbox-worker`, so a `cargo run -p ch33-composition --offline` without `--bin` stops and asks which binary. The chapter line is the `ch33-composition` bin. The example `demo` is the copied eval runner. It calls `run_saved_evals` and `run_stress_evals`. It does not call `composition::demo`.

The bench is `benches/composition.rs`. `Cargo.toml` names it `composition` with `harness = false`.

```rust
fn nested_eight(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    c.bench_function("nested_eight", |b| {
        b.iter(|| {
            let summary = nested_fanout(black_box(&pool), black_box(0), black_box(WORKER_BOUND))
                .expect("eight fit");
            black_box(summary);
        });
    });
}
```

The pool is built before `b.iter`. Each sample calls `nested_fanout` with held 0 and width 8. `0 + 8` equals the limit, and the `>` check is false, so the call schedules eight tasks and gathers them with `Duration::ZERO`. `black_box` keeps the pool borrow, the numbers, and the summary in the sample. The bench does not pass width 9. The refusal is the unit test. The bench does not call `run_continuation`, so it does not sleep 30 seconds and it does not run a gate.

## Tests

Eight tests in `tests/composition.rs`.

`a_nested_fan_out_past_the_bound_is_refused` refuses width 9 on a pool of 8 with held 0, then refuses width 8 with held 1. Both errors are `ComposeError::ExceedsBound`. `max_seen` stays 0.

`eight_nested_children_use_the_same_pool` gathers `n0` through `n7` with a 40 millisecond hold and sees `max_seen` of 8. A zero-hold gather returns the same summary, and `max_seen` is at most 8.

`a_handoff_runs_only_when_ownership_moves` sees `Stay` leave the owner as `supervisor` and the count at 0. `Move` returns one message and the owner becomes `billing`.

`the_loop_accepts_done_and_parks_a_wait_that_names_a_target` asserts `limits` as 20, 30, 300, and 3. Continue-then-done is `Done`. A session target parks as `sess-9`. Pid 44 and 12 seconds parse. A wait object with no target is `None`. A bare `verdict: wait` followed by `verdict: done` is `Done`.

`a_gate_failure_does_not_ask_the_judge_and_three_retries_then_pause` accepts after a red code then a green code, with `max_seen` of 1 and `attempts` of 0. Four red codes pause on `cargo test` and leave `max_seen` at 0.

`twenty_continues_stop_the_loop` returns `LoopStop::Turns`.

`a_missing_verdict_rejects` returns `LoopStop::Reject` for `looks finished`.

`the_demo_stays_with_the_supervisor` expects `fan n0 | n1; owner supervisor; done`.

The package also runs the tests copied forward from chapter 32. Nextest runs the package.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch33-composition
```

```text
    Starting 171 tests across 40 binaries
     Summary [   0.385s] 171 tests run: 171 passed, 0 skipped
```

```bash
cargo run -p ch33-composition --bin ch33-composition --offline
```

```text
fan n0 | n1; owner supervisor; done
```

The binary fans out two children, stays with the supervisor, and judges one scripted `verdict: done`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `nested_eight` | @@MEAN:33/nested_eight@@ | eight children on the pool of 8 | `nested_fanout` with held 0 and width 8 |

The pool is built before `b.iter`. The timed closure schedules eight `n{index}` tasks and gathers them on that pool with a zero hold. Width 9 is not in the sample. The Mean cell is `mean.point_estimate` for id `33/nested_eight` from this PR's Benchmarks run. The printed Criterion `time:` line is the slope.

### Milestone demo

The `cargo run` above. The line is `fan n0 | n1; owner supervisor; done`.

## Pitfalls

`held` is the caller's number. `nested_fanout` does not read `inflight`. A call with `held: 0` and `width: 8` on a pool that another thread has already filled will pass the arithmetic check and then wait inside `acquire`. The tests use a fresh pool. The refusal the tests observe is the `>` check, and `max_seen` stays 0 because `gather_holding` never runs.

`saturating_add` compares the sum with `pool.limit()`. A pool built with `WORKER_BOUND` has limit 8. `WORKER_BOUND + 1` is the width 9 the refusal test asks for. `Pool::new` can be given another limit. The check uses whatever `limit()` returns.

`schedule`'s `expect` panics if the fan-out queue is already at 100. The bound check does not know `QUEUE_CAP`. It knows `pool.limit()`. The tests stay at 8.

`Route::Stay` returns `Ok(None)`. A reader who treats `None` as a failure will mis-read a successful stay. `Err` is `HandoffError`, and `Stay` does not produce it. `Move` can produce `Empty` or `Limit` from the copied board.

`wait_target` returns on the first session hit. A later pid in the same string is not parsed. The alias keys in `_parse_judge_response` are not needles in `field`. A zero pid fails `pid > 0` and the function continues to seconds.

A red gate consumes the model line and skips `as_tool`. The line can say `verdict: done` and still be ignored. `attempts` resets to 0 on a later exit code of 0, so the field after a recovered gate is 0, not the number of earlier failures. Pause happens when `attempts` becomes 4, because the comparison is `>` 3. Three failures return `Retry`.

`LoopStop::Turns` is decided at the top of the loop. The twentieth `Continue` is judged. The next iteration returns before another `complete`. `Parked` does not increment `turns`. A `Wait` that parks still called `as_tool` when the gates were clear, so that path takes one permit. The pause path does not.

`DEFAULT_JUDGE_TIMEOUT_SECS` and `DEFAULT_GATE_TIMEOUT_SECONDS` are returned by `limits`. `thread::sleep` is not called with them in `run_continuation`. The 40 millisecond hold in the overlap test is an argument to `nested_fanout_holding`, and the sleep is in the copied `gather_holding`.

`demo` uses `assert!` on the stay result. A `Stay` that returned `Some` would panic inside `demo` before the string was built. The test expects the string, which is only built when that assertion passes.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Nested bound.** `held.saturating_add(width) > pool.limit()` returns `ComposeError::ExceedsBound` before `FanBoard::new` and before `schedule`. Width 9 on a pool of 8, and width 8 with `held` of 1, leave `max_seen` at 0. The pool the tests build has limit 8, from `WORKER_BOUND`.

**Quality gates before the judge.** `check_gates` runs after `complete` and before `as_tool`. `Retry` and `Paused` return without `as_tool`. Four failures of `cargo test` pause with `max_seen` of 0. A failure then a success reaches the judge once, and `max_seen` is 1. The command string is stored. A shell is not spawned. `DEFAULT_GATE_MAX_RETRIES` is 3. `DEFAULT_GATE_TIMEOUT_SECONDS` is 300 and is not slept.

**Wait target order.** `wait_target` reads `wait_on_session`, then `wait_on_pid` when the parsed pid is greater than 0, then `wait_for_seconds` when the parsed seconds are greater than 0. A wait with none of them is `None`, and the loop spends a turn. A wait with a target returns `LoopStop::Parked` and does not increment `turns`.

**Twenty-turn cap.** `turns >= DEFAULT_MAX_TURNS` returns `LoopStop::Turns` before the next `complete`. `DEFAULT_MAX_TURNS` is 20. Twenty `verdict: continue` lines stop the loop. The twenty-first scripted line is still in the model.

**Left for later.** Chapter 34 scans context files, truncates them, wraps untrusted tool output, and shares a snapshot `Arc`. Chapter 36 is supervision. Heartbeats are still there. This file does not beat and does not restart a child.

## What you implement

`my-harness/tests/ch33.rs` is the milestone. Nested width 9 on a pool of 8 is refused before tasks start.

```rust
use my_harness::chapter_33_refuses_width_nine_before_tasks_start;

#[test]
fn nested_width_nine_on_a_pool_of_eight_is_refused_before_tasks_start() {
    assert!(chapter_33_refuses_width_nine_before_tasks_start());
}
```

The stub panics.

```rust
pub fn chapter_33_refuses_width_nine_before_tasks_start() -> bool {
    todo!("chapter 33")
}
```

`todo!("chapter 33")` is the panic. The test wants the function to return true only when a nested fan-out of width 9 against a pool whose limit is 8 is refused before any task is scheduled. The step crate is the reference. Do not copy it into `my-harness/`.

## Chapter summary

- **Bound.** `held` plus `width` past `pool.limit()` is `ComposeError::ExceedsBound` before `schedule`. Width 9 on a pool of 8 leaves `max_seen` at 0.
- **Same pool.** Eight children gather on the `Pool` the caller passed. The overlap test sees `max_seen` of 8.
- **Route.** `Stay` returns `None` and leaves the owner. `Move` calls `handoff`.
- **Wait.** Session, then pid, then seconds. A target parks. A missing target spends a turn.
- **Gates.** A failing gate does not call `as_tool`. Three retries, then `GatePaused`. The timeouts 30 and 300 are stored and are not slept in the loop.
- **Cap.** Twenty turns return `LoopStop::Turns`.
- **Demo.** `fan n0 | n1; owner supervisor; done`.
- **Time.** Bench id `33/nested_eight`. The Mean column is `mean.point_estimate` from this PR's Benchmarks run. The Criterion `time:` line is the slope.

[personal note: the moment the ninth child was the bug]

## What's next

Chapter 34 manages context. A project file that matches an injection pattern is replaced. A long file keeps a head and a tail. High-risk tool output is wrapped, and an embedded closer is neutralized. Preflight compression runs when the transcript is over the cap, and the hot path shares the snapshot with `Arc::clone`.
