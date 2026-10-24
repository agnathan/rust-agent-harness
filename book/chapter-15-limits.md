# Chapter 15: Limits and Breakers

The harness can now classify a failure and refuse a destructive command. Those are decisions about one agent. A gateway that fronts many tools has a different problem: one tool can be slow, one tool can be called fifty times in a turn, and a backend that is already failing should not be called again while it recovers. This chapter is the server's answer. It is a copy of the gateway, not of the harness. Chapter 16 will copy chapter 14. It will not copy this crate.

The package is `ch15-limits`, the library is `gw_limits`, and the bench is `limits`. Chapter 7's catalog and chapter 8's session gateway are vendored under `src/catalog/` and `src/session.rs`, so this step builds alone. There is no path dependency on `ch07-catalog` or `ch08-session-toolsets`. The public names chapter 23 already calls stay: `Catalog`, `LimitedGateway` (`new`, `catalog`, `open_session`, `session_tools`), and `SessionView`. The crate binds no socket.

`LimitedGateway` wraps the session view with a per-tool gate. `ACCEPT_BOUND` and `EXECUTE_BOUND` are the chapter 5 numbers, now per tool rather than process-wide only. `ToolGate` hands out permits. `Breaker` opens after repeated failures and `breaker_open` stops a new call from reaching the backend. `set_idempotent` marks the tools a retry is allowed to run again. A tool that is not marked idempotent is not retried. Timeouts release the permit. `CallError::Cap` is what the 51st `web_search` receives, and the same cap of 50 applies to a subagent tool. `MAX_WEB_SEARCHES` is that number. Hedging and 429/503 backpressure sit on the same gate. `dispatch` is the path that checks the breaker, the cap, the permit, and the timeout in one order.

The server stays a separate program because these limits protect the backends the gateway fronts, and the other sessions sharing them. The harness already has a bound of 8 on its own dispatcher. That bound does not know that `web_search` has been called fifty times, or that one upstream is down. Putting the cap, the breaker, and the idempotent retry on the server means every client inherits them, including a later harness that only knows the tool name. Chapter 16 will name its search tool `web_search` so this cap applies, and it will not add a second counter. Chapter 23 vendors this crate the same way this crate vendored chapters 7 and 8. Learned tools sit on top of the limit. They do not replace it.

A self-improving agent that can call search, or spawn a subagent, on every reflection will spend the turn on the tool instead of the lesson. The cap is the boring reason the learning loop stays a loop.

Rust, beside the types: a permit is an owned guard, and releasing it is a drop, including on timeout. The breaker is an enum you match on, open or closed. `Result` is `CallError`, and `Cap` is its own variant so a caller can tell a policy refusal from a backend failure.

`cargo run -p ch15-limits` runs the demo in-process. The exercise is `my-harness/tests/ch15.rs`: the 51st `web_search` is the cap error, and an open breaker does not call the backend.

**In this chapter**

- 15.1 One flat package, and the names chapter 23 already calls
- 15.2 `ACCEPT_BOUND`, `EXECUTE_BOUND`, and the cap of 50
- 15.3 `FailureKind`
- 15.4 `Outcome`
- 15.5 `CallOpts`
- 15.6 `CallError`
- 15.7 `Backend`
- 15.8 `Script`
- 15.9 `ToolGate`
- 15.10 `Breaker`
- 15.11 `LimitedGateway` and `new`
- 15.12 `catalog`, `open_session`, and `session_tools`
- 15.13 `set_idempotent`
- 15.14 `is_idempotent`
- 15.15 `breaker_open`
- 15.16 `accept_available` and `execute_available`
- 15.17 `roundtrip_execute`
- 15.18 `hold_accept`
- 15.19 `dispatch`
- 15.20 `retriable`
- 15.21 `block_cap`
- 15.22 `note_cap`
- 15.23 `spawn_count`
- 15.24 `note_failure`
- 15.25 `note_success`
- 15.26 `gate`
- 15.27 `try_accept` and `try_execute`
- 15.28 `run_with_timeout`
- 15.29 `Default` for `LimitedGateway`
- 15.30 `open_breaker` and `permit_release`

| Section | Step | What it adds |
|---|---|---|
| 15 | `steps/ch15-limits` | per-tool permits, idempotent retry, a breaker, the caps of 50 |

> **How Hermes does it**
>
> I read `agent/tool_guardrails.py` (`LoopCapConfig`, `_DEFAULT_MAX_WEB_SEARCHES_PER_TURN`, `_DEFAULT_MAX_SUBAGENTS_PER_TURN`, `_check_loop_cap`, `_subagent_spawn_count`, `loop_web_search_cap`, `loop_subagent_cap`) and `website/docs/user-guide/features/mcp.md` (the connect-cooldown sentence) on Hermes `main` at `cf71b1aa`.
>
> The defaults are 50 web searches and 50 subagent spawns. `_check_loop_cap` blocks before the call once the count is already at the cap, then advances the counter. `web_search` counts as 1. `delegate_task` counts `len(tasks)` for a non-empty batch, otherwise 1. Actions `list`, `steer`, and `stop` count as 0, so they still run after the cap. The block text for search says the turn has already made `{cap}` web searches. The block text for delegation says it has already spawned `{count}` subagents (limit `{cap}`).
>
> The same MCP guide says a server whose first connect failed is retried on a cooldown of 30 seconds, doubling up to 10 minutes. That is a client policy.
>
> **Adopted.** Both caps at 50. The block happens before the backend call. A `tasks` array counts as its length. `list`, `steer`, and `stop` add nothing and still run. The two block sentences, with the count and the cap filled in.
>
> **Adapted.** The breaker, the 32/8 gate, the hedge, and 429/503 handling are this chapter's server policy. A transport error, or a status other than 429 or 503, retries once when the tool is idempotent. 429 and 503 do not retry. Three failed calls open the breaker. A success clears the failure count only while the breaker is still closed. A hedge starts a second call only for an idempotent tool and keeps the first success.
>
> **Deferred.** The 30-second cooldown that doubles to 10 minutes is not copied. A half-open breaker, shared rate-limit state, and the socket stay later (chapters 35 and 38). Chapter 9 still counts a tool named `delegate` against `max_subagent_iterations`. This chapter counts `delegate_task`.

## What this chapter decides

`LimitedGateway::new` wraps the chapter 8 session gateway. `catalog`, `open_session`, and `session_tools` are the same methods the smoke test already calls. The catalog is empty until something loads it.

`set_idempotent("echo", true)` marks one tool. The default is false, so a write does not retry.

`dispatch` checks the breaker first. An open breaker returns `CallError::OpenBreaker` and does not call the backend. Then it checks the cap. The 51st `web_search` returns the search sentence and does not call the backend. The same is true for `delegate_task`, except `{"action":"list"}` (and `steer`, and `stop`) still run.

A transport failure on an idempotent tool calls the backend a second time. A 429 or a 503 does not, even when the tool is idempotent. A status such as 500 does retry once, and only then. A non-idempotent tool never gets that second call.

`CallOpts { hedge: true }` on an idempotent tool always makes the second call, and the first `Ok` wins. The same option on a write makes one call.

Three `503` results open the breaker for that tool name. A different tool is not open. A success in the middle clears the count, so two failures, one success, and two more failures stay closed.

`hold_accept` takes one of 32 accept permits. The 33rd is `Saturated`. `run_with_timeout` takes one accept permit and one execute permit, waits 30 milliseconds against a 5 second sleep, and drops both. The test finishes in under two seconds, and both counts are full again.

The demo marks `echo` idempotent. The script fails once with a transport error, then returns `ok`. Three `503`s on `search` open the breaker. The next `search` prints `breaker: open`.

## 15.1 One flat package, and the names chapter 23 already calls

The file is `steps/ch15-limits/src/lib.rs`. The package name is `ch15-limits`. The library name is `gw_limits`. A test writes `gw_limits::LimitedGateway`. The harness layout puts code under `crates/harness-core`. This server step keeps a flat `src/` tree:

```text
steps/ch15-limits/
  Cargo.toml
  src/lib.rs
  src/main.rs
  src/session.rs
  src/catalog/mod.rs
  src/catalog/index.rs
  benches/limits.rs
  tests/limits.rs
  tests/catalog.rs
  tests/session.rs
```

`pub mod catalog` loads `src/catalog/mod.rs`. `pub mod session` loads `src/session.rs`. I copied chapter 7 and chapter 8 into those files so the step builds alone. `Cargo.toml` has no path dependency on the earlier server crates.

The top of `lib.rs` is the module comment, the two modules, the imports, and the re-exports.

```rust
//! Book chapter 15. A tool has its own accept and execute permits.
//! A retry runs only when the tool is idempotent. An open breaker
//! does not call the backend. Web search and delegate_task stop at 50.
//!
//! The chapter 7 catalog and the chapter 8 session gateway are vendored
//! in this crate (`catalog`, `session`) so the step builds alone.

pub mod catalog;
pub mod session;

use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::session::SessionGateway;
use serde_json::Value;
use thiserror::Error;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub use crate::catalog::Gateway as Catalog;
pub use crate::session::{SessionError, SessionView};
```

`pub use crate::catalog::Gateway as Catalog` publishes the chapter 7 gateway as `Catalog`. `catalog::Gateway` and `Catalog` are the same type. Chapter 23 writes `gw_limits::Catalog`, `gw_limits::LimitedGateway`, and `gw_limits::SessionView`. Those three stay. `SessionError` is also at the crate root, because the session module's own methods return it. The methods chapter 23 calls are `new`, `catalog`, `open_session`, and `session_tools`.

`LimitedGateway` owns a `SessionGateway`. That struct owns a `catalog::Gateway`. The limits code calls through `inner`.

```rust
pub struct SessionGateway {
    catalog: Gateway,
    sessions: Mutex<HashMap<String, Session>>,
}

impl SessionGateway {
    pub fn new() -> Self {
        Self {
            catalog: Gateway::new(),
            sessions: Mutex::new(HashMap::new()),
        }
    }
}
```

`Gateway::new` stores a snapshot built from an empty card list, so `is_empty` is true. A fresh `LimitedGateway` stays empty until `reload`. The limits tests never load tools. The copied catalog and session tests do, through `gw_limits::catalog` and `gw_limits::session`.

## 15.2 `ACCEPT_BOUND`, `EXECUTE_BOUND`, and the cap of 50

```rust
/// Calls waiting or running for one tool. The 33rd is refused.
pub const ACCEPT_BOUND: usize = 32;
/// Calls inside one tool body at once.
pub const EXECUTE_BOUND: usize = 8;
pub const BREAKER_THRESHOLD: u32 = 3;
pub const MAX_WEB_SEARCHES: u32 = 50;
pub const MAX_SUBAGENTS: u32 = 50;
```

`ACCEPT_BOUND` is 32. `Semaphore::new` starts the accept counter there. `hold_accept` takes one permit at a time. The 33rd take is `CallError::Saturated`, which is what the comment says.

`EXECUTE_BOUND` is 8. `run_with_timeout` waits for one execute permit before the body runs. Eight bodies can hold one at once. A ninth waits, after it already holds an accept permit.

`Semaphore::new` and `available_permits` use `usize`, so those two constants are `usize`. The streak and the caps live in `AtomicU32`, so the other three constants are `u32`. `web_searches.load(...) >= MAX_WEB_SEARCHES` type-checks because both sides are `u32`.

`BREAKER_THRESHOLD` is 3. `MAX_WEB_SEARCHES` and `MAX_SUBAGENTS` are 50. `block_cap` refuses `web_search` once that counter is already 50, so the 50th search is the last one that runs. A spawning `delegate_task` uses the same line. `list`, `steer`, and `stop` add zero. `format!("{MAX_WEB_SEARCHES}")` prints `50`, and the test names `gw_limits::ACCEPT_BOUND` directly.

## 15.3 `FailureKind`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Transport,
    Status(u16),
}
```

`Transport` carries no data. The empty script queue uses it, and the retry test uses it for a failure that may run again. `Status` holds a `u16`. The tests pass `429` and `503`. `retriable` has a separate arm for every other status, so `Status(500)` is a different answer from `Status(503)`.

> **Rust sidebar: an enum is one variant**
> - A struct always has the same fields. An enum is a choice. A `FailureKind` is either `Transport` or `Status(u16)`, and it is exactly one of those.
> - You get the `u16` back by matching. `Transport` has nothing to get back.
> - `Debug` is `{:?}`. The binary's panic prints `{other:?}`, which needs `Debug` on `CallError` and on the `FailureKind` inside `Failed`.
> - `Copy` means a value is copied on assignment. Both variants are `Copy` because `u16` is `Copy`. `retriable` takes a `FailureKind` by value, and the caller still has its own.
> - `PartialEq` and `Eq` are `==`. Tests compare `CallError::Failed(FailureKind::Transport)` by value.

## 15.4 `Outcome`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Ok(String),
    Fail(FailureKind),
}
```

There is no `Copy`. `String` is not `Copy`. Moving `Outcome::Ok` moves the body string. `Clone` is still derived, so a caller who needs two copies can ask. `dispatch` moves the outcome.

`Ok` on this enum is my variant. `Result`'s `Ok` is a different type. `backend.call` returns `Outcome`. `dispatch` returns `Result<String, CallError>`. The last match in `dispatch` turns `Outcome::Ok(body)` into `Ok(body)`, and `Outcome::Fail(kind)` into `Err(CallError::Failed(kind))`.

`Outcome::Ok("ok".into())` allocates when the script is built. `"ok".into()` uses `From<&str> for String`.

## 15.5 `CallOpts`

```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct CallOpts {
    pub hedge: bool,
}
```

`hedge` is public. The hedge test builds `CallOpts { hedge: true }`. Every other call in the tests and the binary passes `CallOpts::default()`. `Default` on the struct is `Default` on each field, and `bool`'s default is `false`. A default call does not hedge.

`Copy` means `dispatch` receives a copy of the struct. I read `opts.hedge` twice: once inside the retry guard, and once as the hedge branch condition.

## 15.6 `CallError`

```rust
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CallError {
    #[error("open")]
    OpenBreaker,
    #[error("saturated")]
    Saturated,
    #[error("timeout")]
    Timeout,
    #[error("{0}")]
    Cap(String),
    #[error("failed")]
    Failed(FailureKind),
}
```

> **Rust sidebar: `Result` and `?`**
> - `Result<String, CallError>` has two variants. `Ok` holds the tool body. `Err` holds one `CallError`.
> - `?` returns early on `Err`. `try_accept(tool)?` inside `run_with_timeout` either binds the permit or returns `Err(CallError::Saturated)`. The error type already matches, so `?` does not convert it.
> - `#[derive(Error)]` comes from `thiserror`. `#[error("open")]` is the `Display` text for `OpenBreaker`. `Cap`'s `{0}` prints the `String`, which is the block sentence. `Failed` prints `failed`. The kind is not in that sentence. `assert_eq!` uses `PartialEq`, so tests still see `FailureKind::Status(429)`.
> - `OpenBreaker`, `Saturated`, and `Timeout` carry no fields. `Cap` owns the message. `Failed` owns a `Copy` kind.

`dispatch` returns `OpenBreaker`, `Cap`, and `Failed`. The permit helpers return `Saturated` from `map_err`. `run_with_timeout` returns `Timeout` when the timer wins.

## 15.7 `Backend`

```rust
pub trait Backend: Send + Sync {
    fn call(&self, tool: &str) -> Outcome;
}
```

> **Rust sidebar: a trait, and `dyn`**
> - A trait is a list of methods a type promises to have. `Script` implements `Backend`.
> - `&dyn Backend` is a fat pointer: the value, and a table of its methods. `dispatch` does not name `Script` in its signature.
> - `Send` means the value can move to another thread. `Sync` means a shared reference can be used from several threads. The trait bound puts both on every implementor.
> - `call` takes `&self` and returns `Outcome` directly. It is not `async`. `dispatch` is synchronous for the same reason. The only `async` function in this file is `run_with_timeout`, and that function does not call the backend.

`Script::call` ignores `tool`. The gateway has already branched on the name: the breaker, the cap, and the idempotent flag are all keyed by that string.

## 15.8 `Script`

```rust
#[derive(Debug)]
pub struct Script {
    steps: Mutex<VecDeque<Outcome>>,
    calls: AtomicU32,
}

impl Script {
    pub fn new(steps: Vec<Outcome>) -> Self {
        Self {
            steps: Mutex::new(steps.into()),
            calls: AtomicU32::new(0),
        }
    }

    pub fn calls(&self) -> u32 {
        self.calls.load(Ordering::SeqCst)
    }
}

impl Backend for Script {
    fn call(&self, _tool: &str) -> Outcome {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.steps
            .lock()
            .expect("script")
            .pop_front()
            .unwrap_or(Outcome::Fail(FailureKind::Transport))
    }
}
```

`new` takes the `Vec`. `into()` builds a `VecDeque`, and `pop_front` removes the oldest outcome. The first value I pushed is the first `call` returns.

`calls` starts at 0. `fetch_add(1, Ordering::SeqCst)` adds one as a single step. I ignore the old value it returns. `calls()` loads that same atomic. `lock().expect("script")` panics if the mutex is poisoned, which means a thread panicked while holding it. `pop_front` then `unwrap_or` yields `Fail(Transport)` on an empty deque. The open-breaker bench's idle script is that empty deque, and the open breaker returns before `call` pops it. `_tool` is required by the trait and unread here.

> **Rust sidebar: `AtomicU32`, `SeqCst`, and `Mutex`**
> - An `AtomicU32` is a `u32` several tasks can update without a `Mutex`. `fetch_add` adds one. `call` ignores the old value it returns.
> - Every atomic in this file uses `Ordering::SeqCst`, so a later `load` sees an earlier `fetch_add`.
> - `call` takes `&self`, so it cannot assign a plain field. `lock()` yields a guard that acts as `&mut VecDeque<Outcome>` and unlocks when that guard drops. `Mutex` and `AtomicU32` are `Sync`, which is why `Script` can implement `Backend`.

## 15.9 `ToolGate`

```rust
#[derive(Clone)]
struct ToolGate {
    accept: Arc<Semaphore>,
    execute: Arc<Semaphore>,
}

impl ToolGate {
    fn new() -> Self {
        Self {
            accept: Arc::new(Semaphore::new(ACCEPT_BOUND)),
            execute: Arc::new(Semaphore::new(EXECUTE_BOUND)),
        }
    }
}
```

The struct is private. `Semaphore::new(32)` and `Semaphore::new(8)` are different counters. Filling accept leaves execute at 8.

> **Rust sidebar: `Arc` and a semaphore**
> - `Arc` is a reference-counted pointer. `clone` copies the pointer and adds one to the count. The last drop frees the heap value. Cloning a `ToolGate` clones both `Arc`s, so both gates share the same two semaphores.
> - A semaphore is a counter of permits. Taking one subtracts. Dropping an `OwnedSemaphorePermit` adds one back. That `Drop` is the release. The calls in this file do not use a separate give-back method.
> - `try_acquire_owned` and `acquire_owned` take `Arc<Semaphore>` by value. The call site clones the `Arc`. The permit keeps that clone. The `ToolGate` in the map keeps its own, so the semaphore outlives the permit.
> - The first lookup of a tool name inserts one gate. A second tool name gets a second pair. A full `echo` gate leaves `search` at 32 and 8.

## 15.10 `Breaker`

```rust
#[derive(Default)]
struct Breaker {
    failures: u32,
    open: bool,
}
```

`Default` is `failures: 0` and `open: false`. `or_default()` builds that on the first failure, then adds one. Tests see `open` through `breaker_open`. They see a cleared streak by watching later failures stay closed. Once `open` is true, `note_success` leaves both fields alone.

## 15.11 `LimitedGateway` and `new`

```rust
pub struct LimitedGateway {
    inner: SessionGateway,
    idempotent: Mutex<HashMap<String, bool>>,
    gates: Mutex<HashMap<String, ToolGate>>,
    breakers: Mutex<HashMap<String, Breaker>>,
    web_searches: AtomicU32,
    subagents: AtomicU32,
}

impl LimitedGateway {
    pub fn new() -> Self {
        Self {
            inner: SessionGateway::new(),
            idempotent: Mutex::new(HashMap::new()),
            gates: Mutex::new(HashMap::new()),
            breakers: Mutex::new(HashMap::new()),
            web_searches: AtomicU32::new(0),
            subagents: AtomicU32::new(0),
        }
    }
```

The `impl` continues through the methods below. `inner` is `SessionGateway::new()`, which is `Gateway::new()`. The catalog starts empty. The three maps start empty. A tool name is inserted on first use: `set_idempotent` inserts a bool, `gate` inserts a `ToolGate`, `note_failure` inserts a `Breaker`. A tool that has only succeeded has no breaker row. `breaker_open` treats a missing row as closed.

`web_searches` and `subagents` are one counter each, shared by every session on this gateway. This file never stores 0 back into either atomic. `new` is the reset. The fields are private.

## 15.12 `catalog`, `open_session`, and `session_tools`

```rust
pub fn catalog(&self) -> &Catalog {
    self.inner.catalog()
}

pub fn open_session(&self, session_id: impl Into<String>) {
    self.inner.open_session(session_id);
}

pub fn session_tools(&self, session_id: &str) -> Option<SessionView> {
    self.inner.session_tools(session_id)
}
```

`catalog` returns `&Catalog`. The inner method returns `&Gateway`. The alias makes them the same reference, borrowed for as long as `&self`. `gateway.catalog().is_empty()` is true on a new gateway.

`open_session` forwards the id. The inner method locks the session map, calls `session_id.into()`, and `entry(...).or_insert_with(Session::new)` inserts only when the id is new.

> **Rust sidebar: `impl Into<String>`**
> - `Into<String>` turns the argument into an owned `String`. A literal `&str` allocates. An existing `String` moves.
> - `impl Into<String>` means any one type that can become a `String`. The test calls `open_session("session-1")`.
> - `entry(...).or_insert_with(Session::new)` inserts only when the id is new. A second open of the same id leaves the session in place. `Session::new` starts with an empty revealed list and `list_changed` false.

`session_tools` locks that map. `get` plus `?` returns `None` for an id that was never opened. `Some` copies `session_id`, clones `revealed` into `tools`, and copies `list_changed`.

> **Rust sidebar: `Option`**
> - `Option<SessionView>` is `Some(view)` or `None`. `get` is `None` when the id was never opened. `?` in a function that returns `Option` returns that `None`.
> - `Some` clones the revealed names and copies `list_changed`. The view does not borrow the session. The lock ends when the function returns.
> - The limits test opens `"session-1"` and expects `tools` to be empty. `revealed` is still empty. `SessionView`'s fields are `session_id`, `tools`, and `list_changed`. Chapter 23 reads `tools`. This layer returns the inner view unchanged.

## 15.13 `set_idempotent`

```rust
pub fn set_idempotent(&self, tool: &str, idempotent: bool) {
    self.idempotent
        .lock()
        .expect("policies")
        .insert(tool.to_string(), idempotent);
}
```

`to_string()` allocates the map key. `insert` replaces a previous bool for the same key. The hedge test sets `echo` to true and `write` to false. Those are two keys. `expect("policies")` is the poison message. The guard drops at the end of the statement, so the lock is gone before `dispatch` calls the backend.

`set_idempotent("write", false)` writes the key even though a missing key is already treated as false. `is_idempotent` then finds `Some(false)`.

## 15.14 `is_idempotent`

```rust
fn is_idempotent(&self, tool: &str) -> bool {
    self.idempotent
        .lock()
        .expect("policies")
        .get(tool)
        .copied()
        .unwrap_or(false)
}
```

`get` returns `Option<&bool>`. `copied()` turns that into `Option<bool>` because `bool` is `Copy`. `unwrap_or(false)` is the answer for a name nobody inserted. `search` in the breaker test was never marked, so this returns false, and a 503 on `search` does not retry.

The bool is copied out before the lock ends. `dispatch` holds that local across `backend.call`.

## 15.15 `breaker_open`

```rust
pub fn breaker_open(&self, tool: &str) -> bool {
    self.breakers
        .lock()
        .expect("breakers")
        .get(tool)
        .is_some_and(|state| state.open)
}
```

`is_some_and` is true when the option is `Some` and the closure returns true. A missing tool is false. A row whose `open` is false is false. The true answer is a row with `open` set. After three failures, `breaker_open("search")` is true and `breaker_open("echo")` is false, because echo has no row. The lock ends inside this function. `dispatch` does not hold it across the backend call.

## 15.16 `accept_available` and `execute_available`

```rust
pub fn accept_available(&self, tool: &str) -> usize {
    self.gate(tool).accept.available_permits()
}

pub fn execute_available(&self, tool: &str) -> usize {
    self.gate(tool).execute.available_permits()
}
```

`gate` inserts a gate on first sight and returns a clone. `available_permits` reads the counter. It does not take a permit. A new tool reports 32 and 8. The timeout test calls both after the timer and expects `ACCEPT_BOUND` and `EXECUTE_BOUND` again.

## 15.17 `roundtrip_execute`

```rust
/// Take one execute permit and drop it. The bench times this pair.
pub fn roundtrip_execute(&self, tool: &str) -> Result<(), CallError> {
    let permit = self.try_execute(tool)?;
    drop(permit);
    Ok(())
}
```

`try_execute` returns an `OwnedSemaphorePermit` or `Err(Saturated)`. `?` returns the error. `drop(permit)` runs `Drop` now and adds the execute permit back. `Ok(())` carries no body. Accept is untouched. The bench loops this on `"echo"`, and each sample finds a free permit because the previous drop already ran.

## 15.18 `hold_accept`

```rust
pub fn hold_accept(&self, tool: &str) -> Result<OwnedSemaphorePermit, CallError> {
    self.try_accept(tool)
}
```

The caller owns the permit. The test stores 32 of them in a `Vec`. The 33rd call is `Err(Saturated)`. `drop(held)` drops each permit, and each `Drop` returns a slot, so `accept_available("echo")` is 32 again. The permit owns an `Arc<Semaphore>`, so the test can call `hold_accept` again while the vec still holds the earlier ones.

## 15.19 `dispatch`

`dispatch` is the policy. It is synchronous. The first returns are the ones that leave the backend uncalled.

```rust
pub fn dispatch(
    &self,
    tool: &str,
    args: &str,
    opts: CallOpts,
    backend: &dyn Backend,
) -> Result<String, CallError> {
    if self.breaker_open(tool) {
        return Err(CallError::OpenBreaker);
    }
    if let Some(message) = self.block_cap(tool, args) {
        return Err(CallError::Cap(message));
    }
    self.note_cap(tool, args);
    let idempotent = self.is_idempotent(tool);
    let first = backend.call(tool);
    let retry = matches!(
        &first,
        Outcome::Fail(kind) if idempotent && retriable(*kind) && !opts.hedge
    );
    let outcome = if opts.hedge && idempotent {
        let second = backend.call(tool);
        match first {
            Outcome::Ok(body) => Outcome::Ok(body),
            Outcome::Fail(_) => second,
        }
    } else if retry {
        backend.call(tool)
    } else {
        first
    };
    match outcome {
        Outcome::Ok(body) => {
            self.note_success(tool);
            Ok(body)
        }
        Outcome::Fail(kind) => {
            self.note_failure(tool);
            Err(CallError::Failed(kind))
        }
    }
}
```

`args` is the raw JSON `spawn_count` parses. `opts` is copied in. `backend` is `&dyn Backend`.

**Open breaker.** A true `breaker_open` returns `Err(CallError::OpenBreaker)` on the first line. `block_cap`, `note_cap`, and `backend.call` stay uncalled. The fourth `search` in the breaker test takes this arm: `script.calls()` stays 3, and `Outcome::Ok("late")` stays in the deque. `breaker_open` locks and unlocks before the rest of the function. On a closed path, `backend.call` runs, and `note_failure` locks the map again afterward. The mutex is not held across the backend.

**Cap.** `if let Some(message)` returns `Err(CallError::Cap(message))` before `note_cap` and before `backend.call`. A blocked search does not add to `web_searches`. A blocked spawn does not add to `subagents`. `web_search` blocks once the counter is `>= 50`. `delegate_task` blocks when `spawn_count(args) > 0` and `subagents >= 50`. `list`, `steer`, and `stop` return a spawn count of 0, the guard fails, and the backend still runs.

**Count, then call.** `note_cap` runs for every call that passed both checks, including a `web_search` whose backend later fails. `is_idempotent` copies the bool. Then `backend.call` runs. This is the first real call.

**Retry flag.** `matches!` tests `&first`, so `kind` is `&FailureKind`. The guard is `idempotent && retriable(*kind) && !opts.hedge`. A write has `idempotent == false`, so a transport error is final and `calls()` stays 1. `retriable` is true for `Transport` and for any status other than 429 or 503, including 500. It is false for 429 and 503, so an idempotent tool that returns either of those still calls the backend once. `!opts.hedge` keeps the hedge path from also setting this flag. An `Ok` fails the `Fail` pattern, so `retry` is false.

**Hedge.** `opts.hedge && idempotent` always makes a second `backend.call`, then matches `first`. `Outcome::Ok(body)` keeps that body and drops `second`, including its string. The test expects `"first"` and `calls() == 2`. `Outcome::Fail(_)` moves `second` in. One `note_success` or one `note_failure` records the winner. A hedge on a non-idempotent tool fails the condition, `retry` is false, and the `else` keeps `first`. That write sees `calls() == 1` and the body `"once"`.

**Retry arm.** `else if retry` calls the backend once more and uses that second outcome. `first` drops at the end of the function. Echo scripts `Fail(Transport)` then `Ok("ok")`, returns `"ok"`, and `calls()` is 2. There is no loop. A second failure is the one failure `note_failure` records.

**Else.** `first` is the outcome. One `backend.call` happened. That is the write, the 429, the 503, a plain success, or a hedge on a write.

**Record.** `Outcome::Ok(body)` calls `note_success` and returns `Ok(body)`. The caller owns the string. `Outcome::Fail(kind)` calls `note_failure` and returns `Err(CallError::Failed(kind))`. One dispatch notes once, even when two backend calls ran. The kind on the error is the kind that won.

> **Rust sidebar: `match` and `matches!`**
> - `match` runs exactly one arm. The last match has `Ok` and `Fail`, which covers the enum.
> - Naming a field moves it. `Outcome::Ok(body)` moves the `String` out.
> - `matches!` returns `bool` and does not leave `kind` bound afterward.
> - A guard is the `if` on a pattern. `*kind` copies the `FailureKind` because the match is on a reference and the enum is `Copy`.
> - `if let Some(message)` is a one-arm match. `None` skips the block.

## 15.20 `retriable`

```rust
fn retriable(kind: FailureKind) -> bool {
    match kind {
        FailureKind::Transport => true,
        FailureKind::Status(429 | 503) => false,
        FailureKind::Status(_) => true,
    }
}
```

`Transport` returns true. That is the arm an idempotent echo retries on.

`Status(429 | 503)` returns false. The `|` matches either code. 429 and 503 are backpressure, so another call would pile onto a backend that has already asked for less traffic. An idempotent fetch still sees `calls()` of 1 for each of them. `dispatch` requires both `idempotent` and `retriable`.

`Status(_)` returns true for every other `u16`, including 500. The function takes `kind` by value. It does not read the idempotent map.

## 15.21 `block_cap`

```rust
fn block_cap(&self, tool: &str, args: &str) -> Option<String> {
    match tool {
        "web_search" if self.web_searches.load(Ordering::SeqCst) >= MAX_WEB_SEARCHES => {
            Some(format!(
                "Blocked web_search: this turn has already made {MAX_WEB_SEARCHES} web searches, the per-turn limit. This looks like a runaway search loop. Work with the results you already have and give the user your answer."
            ))
        }
        "delegate_task"
            if spawn_count(args) > 0
                && self.subagents.load(Ordering::SeqCst) >= MAX_SUBAGENTS =>
        {
            Some(format!(
                "Blocked delegate_task: this turn has already spawned {MAX_SUBAGENTS} subagents (limit {MAX_SUBAGENTS}). This looks like a runaway delegation loop. Finish the work with the results you have and answer the user."
            ))
        }
        _ => None,
    }
}
```

A failed guard does not run the arm. Matching continues at the next pattern.

**`web_search`.** The guard loads the counter and compares with `>= 50`. 49 skips the arm. `_` then returns `None`, and `note_cap` adds one. 50 returns `Some`. `dispatch` wraps it in `CallError::Cap`. The format string fills the constant, so the text contains `already made 50 web searches`. Arguments are ignored. Every `web_search` counts as one, including `""`.

**`delegate_task`.** The guard requires `spawn_count(args) > 0` and `subagents >= 50`. `list`, `steer`, and `stop` return 0, so they skip the arm with the cap already full, and `dispatch` still calls the backend. The format string fills both braces with `MAX_SUBAGENTS`, so the text says `already spawned 50 subagents` and `(limit 50)`.

A batch is a spawn: `{"tasks":[1,2,3]}` has `spawn_count` 3. The guard looks at the counter before the add. A batch that starts under 50 is allowed, and `note_cap` adds the whole length. The test starts with that batch on a fresh gateway, then single spawns, until the next spawn is the block.

**`_`.** `None` is every other tool, `web_search` under the cap, and `delegate_task` when the guard fails. `dispatch` continues.

`chapter_15_web_search_allowed(49)` is this guard with 49 already made, so the 50th search still runs. `50` makes the guard true.

## 15.22 `note_cap`

```rust
fn note_cap(&self, tool: &str, args: &str) {
    match tool {
        "web_search" => {
            self.web_searches.fetch_add(1, Ordering::SeqCst);
        }
        "delegate_task" => {
            self.subagents
                .fetch_add(spawn_count(args), Ordering::SeqCst);
        }
        _ => {}
    }
}
```

`web_search` adds 1. I ignore the previous value `fetch_add` returns. `delegate_task` adds `spawn_count`: 0 for `list`, `steer`, and `stop`, 3 for three tasks, and 1 for `""` because that string is not JSON. The delegate test uses `""` for each single spawn. `_ => {}` leaves `echo` and `search` alone.

`load` in `block_cap` and `fetch_add` here are two operations. On one thread the add from call N is what call N+1 loads. A blocked call never reaches this function, so single searches stop at 50. A batch allowed under the cap adds its full length, and that add can land past 50.

## 15.23 `spawn_count`

```rust
fn spawn_count(args: &str) -> u32 {
    let Ok(value) = serde_json::from_str::<Value>(args) else {
        return 1;
    };
    if let Some(action) = value.get("action").and_then(Value::as_str) {
        if matches!(
            action.to_ascii_lowercase().as_str(),
            "list" | "steer" | "stop"
        ) {
            return 0;
        }
    }
    match value.get("tasks") {
        Some(Value::Array(items)) if !items.is_empty() => items.len() as u32,
        _ => 1,
    }
}
```

**Bad JSON returns 1.** `from_str::<Value>` parses the string. The `let`-`else` returns 1 on `Err`. `""` takes that path, so an empty argument counts as one spawn. The singles in the delegate test depend on it.

> **Rust sidebar: `let`-`else`**
> - `let Ok(value) = result else { return 1; }` binds `value` on `Ok`. On `Err` the `else` block runs, and it must diverge. `return` diverges.
> - After the `let`, `value` is a parsed `serde_json::Value`. `::<Value>` picks the type `from_str` produces.

**Control actions return 0.** `get("action").and_then(Value::as_str)` is `Some` only for a string action. `to_ascii_lowercase` makes `"LIST"` match `"list" | "steer" | "stop"`. This return is before the `tasks` match, so a list action with a `tasks` array still counts as 0. Any other action, and a missing action, fall through. `{"tasks":[1,2,3]}` has no action, so it continues.

**A non-empty array returns its length.** `Some(Value::Array(items)) if !items.is_empty()` rejects `[]`. `items.len() as u32` turns the `usize` length into the atomic's type. Three items count as three subagents on one `dispatch`.

**Otherwise 1.** `_` is a missing `tasks`, a non-array, or an empty array. A non-empty batch used the length. Every other spawn that did not return 0 counts as one.

## 15.24 `note_failure`

```rust
fn note_failure(&self, tool: &str) {
    let mut breakers = self.breakers.lock().expect("breakers");
    let state = breakers.entry(tool.to_string()).or_default();
    if state.open {
        return;
    }
    state.failures += 1;
    if state.failures >= BREAKER_THRESHOLD {
        state.open = true;
    }
}
```

`entry` stores the new `String` key when the tool is new. `or_default()` inserts `failures: 0, open: false`. `state` is `&mut Breaker`. An open row returns immediately, so the count stops. Otherwise `failures` grows by one, and `>= BREAKER_THRESHOLD` sets `open` on the increment that lands on 3. The guard unlocks when the function returns. `state` borrows the map, so the write finishes first.

Three 503s on an unmarked `search` are three calls and three notes. 503 is not retriable, so each `dispatch` is one `call`. The third note opens `search`. `echo` is a different key, and `breaker_open("echo")` stays false.

## 15.25 `note_success`

```rust
fn note_success(&self, tool: &str) {
    let mut breakers = self.breakers.lock().expect("breakers");
    if let Some(state) = breakers.get_mut(tool) {
        if !state.open {
            state.failures = 0;
        }
    }
}
```

`get_mut` is `None` when the tool has never failed, which is the echo retry: the final `Ok` finds no row. When the row exists and `open` is false, `failures` becomes 0. The clear test is two 503s, one `Ok`, two more 503s. The `Ok` resets a count of 2, and the last pair ends at 2, under the threshold of 3. When `open` is already true the inner `if` skips the write. A success leaves an open breaker open.

## 15.26 `gate`

```rust
fn gate(&self, tool: &str) -> ToolGate {
    let mut gates = self.gates.lock().expect("gates");
    gates
        .entry(tool.to_string())
        .or_insert_with(ToolGate::new)
        .clone()
}
```

`or_insert_with(ToolGate::new)` calls `new` only when the key is vacant. The first `hold_accept("echo")` builds the 32-permit semaphore and the 8-permit semaphore. The next lookup clones that same pair. Calling `ToolGate::new` on every lookup would reset the counters and forget the permits already taken.

`.clone()` copies the two `Arc`s into the returned `ToolGate`. The guard drops when `gate` returns, so the mutex unlocks before the caller awaits. `run_with_timeout` waits on the cloned `Arc`. If it waited while this lock was still held, every other tool's `gate` lookup would sit behind that timeout.

## 15.27 `try_accept` and `try_execute`

```rust
fn try_accept(&self, tool: &str) -> Result<OwnedSemaphorePermit, CallError> {
    self.gate(tool)
        .accept
        .clone()
        .try_acquire_owned()
        .map_err(|_| CallError::Saturated)
}

fn try_execute(&self, tool: &str) -> Result<OwnedSemaphorePermit, CallError> {
    self.gate(tool)
        .execute
        .clone()
        .try_acquire_owned()
        .map_err(|_| CallError::Saturated)
}
```

`.accept.clone()` is a second `Arc`, and `try_acquire_owned` consumes it. The temporary `ToolGate` drops at the end of the statement. The map keeps its `Arc`. The permit keeps the clone until the permit drops. `try_acquire_owned` either subtracts one now or returns `Err` when the counter is zero. `map_err` turns every tokio error into `Saturated`. This file never closes a semaphore, so the 33rd accept is the empty counter. `hold_accept` is `try_accept`. `roundtrip_execute` is `try_execute` plus `drop`. `run_with_timeout` tries accept, then waits on execute.

> **Rust sidebar: `try_acquire_owned` and `acquire_owned().await`**
> - `try_acquire_owned` returns now. `Ok` is a permit. `Err` means the counter was zero, or the semaphore was closed. Accept uses this. The 33rd caller receives `Saturated` and is not queued.
> - `acquire_owned().await` waits. While the counter is zero, `.await` pauses the task. Dropping some other permit wakes it, and the future completes with a permit.
> - Both consume an `Arc<Semaphore>` and return an `OwnedSemaphorePermit`. That owned value is what `hold_accept` returns, and what `run_with_timeout` holds across `.await`. A permit that borrowed `&Semaphore` would be tied to that borrow.
> - `try_execute` is the non-waiting take the bench samples. `run_with_timeout` waits, so a call that already holds an accept permit can wait for one of eight runners.

## 15.28 `run_with_timeout`

```rust
pub async fn run_with_timeout<F, Fut>(
    &self,
    tool: &str,
    budget: Duration,
    body: F,
) -> Result<String, CallError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = String>,
{
    let accept = self.try_accept(tool)?;
    let execute = self
        .gate(tool)
        .execute
        .clone()
        .acquire_owned()
        .await
        .expect("execute semaphore");
    let result = match tokio::time::timeout(budget, body()).await {
        Ok(value) => Ok(value),
        Err(_elapsed) => Err(CallError::Timeout),
    };
    drop(execute);
    drop(accept);
    result
}
```

> **Rust sidebar: `async`, `Future`, and `FnOnce`**
> - `async fn` returns a future. The test builds a current-thread runtime and `block_on`s this function. `dispatch` needs no runtime.
> - `F: FnOnce() -> Fut` is a closure I call once. The test passes `|| async { ... }`. Calling it returns the inner future.
> - `Fut: Future<Output = String>` means that future resolves to a `String`. The `match` turns tokio's timeout error into `CallError::Timeout`.
> - `.await` is the pause. This function may pause on `acquire_owned`, and again on `timeout`.

**Accept refuses.** `try_accept` runs first. A full accept semaphore returns `Saturated` through `?`, and execute is never awaited. The test's tool `"slow"` is a new name, so this line gets a permit.

**Execute waits.** The next lines clone the execute `Arc` and await `acquire_owned`. `"slow"` still has 8 permits, so the future completes on the first poll. `expect("execute semaphore")` panics if the semaphore is closed. An empty execute semaphore waits. Accept returns `Saturated`. Execute pauses. Both locals are `OwnedSemaphorePermit` values, and neither is moved into the body closure.

**The race.** `body()` starts the caller's future. `tokio::time::timeout` races it against `budget`. The test passes 30 milliseconds and a body that sleeps five seconds and would then return `"late"`. `Ok(value)` is the `String`, wrapped in my own `Ok`. `Err(_elapsed)` means the budget fired. That arm stores `Err(CallError::Timeout)` and does not leave the function. Tokio then drops the inner future. The sleep is that future, so the drop cancels it. `"late"` is never built. The test clock finishes under two seconds. This function returns `Timeout` and does not read that clock.

Dropping the body leaves `accept` and `execute` where they are. They live in `run_with_timeout`, outside `body()`.

**Why `drop` releases the permit.** `drop(execute)` and `drop(accept)` sit after the match, so they run when the body finishes and when the timer wins. `drop` takes the permit by value and runs `Drop` immediately. `OwnedSemaphorePermit`'s `Drop` adds one permit back to the semaphore held by the permit's `Arc`. Accept is 32 again. Execute is 8 again. The test reads both counters.

`result` is the return value. The `?` on a failed `try_accept` holds nothing. Once both permits exist, a winning timer does not cancel `run_with_timeout`: `timeout` returns `Err`, and this function drops the permits before it returns. `dispatch` never calls this function.

## 15.29 `Default` for `LimitedGateway`

```rust
impl Default for LimitedGateway {
    fn default() -> Self {
        Self::new()
    }
}
```

`default()` is `new()`. The catalog is empty, the maps are empty, and both counters are 0. `CallOpts` and `Breaker` derive `Default`. `LimitedGateway` writes the impl so callers share `new`. `SessionGateway` and the catalog `Gateway` do the same: `default` calls `new`.

## 15.30 `open_breaker` and `permit_release`

The bench file is `benches/limits.rs`. The ids are the `bench_function` names. The means are the table in the build section.

```rust
fn open_breaker(c: &mut Criterion) {
    let gateway = LimitedGateway::new();
    let script = Script::new(vec![
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
    ]);
    for _ in 0..3 {
        let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    }
    let idle = Script::new(vec![]);
    c.bench_function("open_breaker", |b| {
        b.iter(|| {
            let err = gateway.dispatch(black_box("search"), "", CallOpts::default(), &idle);
            let _ = black_box(err);
        });
    });
}

fn permit_release(c: &mut Criterion) {
    let gateway = LimitedGateway::new();
    c.bench_function("permit_release", |b| {
        b.iter(|| {
            gateway
                .roundtrip_execute(black_box("echo"))
                .expect("permit");
        });
    });
}
```

`open_breaker` records three 503s before `b.iter`. The sample dispatches `search` on an empty script, and the open breaker returns before `idle.call`. `permit_release` builds one gateway and each sample is `roundtrip_execute`: one execute permit, then `drop`. `criterion_group!` registers both ids. The means are the table below. The timeout stays a test.

## Tests

Eight tests. An idempotent transport error retries once. A write does not retry, and neither do 429 or 503. Three failures open the breaker, and the fourth call does not touch the script. A success clears the count. Fifty web searches run, and the next one is the block sentence. A batch of three plus the rest of the cap blocks `delegate_task`, and `{"action":"list"}` still runs. A hedge on `echo` calls twice and keeps the first body. A hedge on a write calls once. Thirty-two accept permits fill the gate. A timeout returns both permits.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch15-limits
```

```text
Starting 17 tests across 5 binaries
     Summary [   0.145s] 17 tests run: 17 passed, 0 skipped
```

```bash
cargo run -p ch15-limits
```

```text
result: ok
breaker: open
```

The binary is two fake calls. `echo` retries a transport error. `search` is not idempotent. Three 503s open its breaker. No key. No socket.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `open_breaker` | 31.97 ns | under 1 µs | a call that returns before the backend |
| `permit_release` | 57.73 ns | under 1 µs | take one execute permit and drop it |

`open_breaker` records three 503s before the bench. The timed closure is `b.iter`. Inside: `dispatch` of `search` with the tool name passed through `black_box`. The breaker is already open, so the script is not called. Building the gateway and opening the breaker are excluded.

`permit_release` builds the gateway before the bench. Each sample calls `roundtrip_execute`, which `try_acquire_owned`s one permit from the execute semaphore of 8 and drops it. The accept semaphore is not taken. There is no sleep and no backend.

The timeout is not a bench. The test above is the measurement of the cancel: 30 milliseconds against a 5 second sleep, both permit counts full, elapsed under 2 seconds.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `15`.

### Milestone demo

The `cargo run` above. The result is `ok`. The breaker is open.

## Pitfalls

A 429 on a tool you marked idempotent still does not retry. Idempotent means a transport error or a status other than 429 or 503 may run once more. It does not mean every failure is safe to repeat.

The breaker stays open. This chapter has no half-open probe. The next call is refused until the process is new.

A hedge spends the second call even when the first one already succeeded. That is the race you asked for. Do not turn it on for a write. The flag is ignored for a non-idempotent tool, which is the safe default, and it is easy to miss in a log that only shows the body you kept.

The queue is the accept bound of 32. The 33rd call is refused. Nothing parks past that bound.

`delegate` and `delegate_task` are different names. Chapter 9's loop still counts `delegate`. This gateway counts `delegate_task`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L4.

**I3.** `web_search` stops at 50. `delegate_task` stops at 50. A batch counts as `len(tasks)`. `list`, `steer`, and `stop` do not count and still run. The block strings are the ones in `agent/tool_guardrails.py`.

**Timeout.** `run_with_timeout` drops the accept permit and the execute permit. The 5 second sleep does not finish.

**Retry.** A second call happens only when the tool is idempotent, and only for a transport error or a status other than 429 or 503.

**Backpressure.** 429 and 503 do not retry. They count toward the breaker.

**Hedge.** The second call is started only when the tool is idempotent.

**Breaker.** Three failed calls open it. The next call does not reach the backend.

**Left for later.** The client reconnect cooldown (30 seconds, doubling to 10 minutes) is not this breaker. Replica routing is chapter 37. The socket is chapter 42. Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Permits.** 32 accepted, 8 running, per tool. A timeout gives both back.
- **Retry.** Idempotent only. 429 and 503 wait outside.
- **Breaker.** Three failures, then the backend is not called.
- **Caps.** 50 web searches. 50 subagent spawns. A list action still runs.
- **Time.** The open-breaker check measured **31.97 ns**. The permit round trip measured **57.73 ns**.

## What you implement

`my-harness/tests/ch15.rs` is the milestone. It compiles, then panics in `todo!("chapter 15")` until `chapter_15_web_search_allowed(49)` is true and `chapter_15_web_search_allowed(50)` is false. The fiftieth search is the last one allowed. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 18 is the provider. The same turn can build an Anthropic body, a Bedrock body, or an OpenRouter route, rotate a key without building a new client, and leave the side model alone unless the turn asked for a summary.
