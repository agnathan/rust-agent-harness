# Chapter 15: Limits and Breakers

The 51st `web_search` never reaches the backend. The counter is already at 50, `dispatch` returns the block sentence, and the call that would have been the runaway search is the one that stops. Three failures on the same tool open a breaker, so a backend that is already failing isn't asked again while it recovers. Chapter 14 can classify one agent's empty reply and refuse a destructive command. A gateway that fronts many tools has a different problem, and this chapter is that server, a copy of the gateway rather than of the harness.

A continuously improving harness will want to search, and later to spawn a subagent, on the way to a lesson. The cap is the boring reason that loop stays a loop: fifty searches, then the turn has to answer with what it already has. The harness bound of 8 doesn't know that `web_search` has been called fifty times, or that one upstream is down. Putting the cap, the breaker, and the idempotent retry on the server means every client inherits them. Chapter 16 names its search tool `web_search` so this cap applies, and it copies chapter 14, not this crate. Chapter 23 vendors this crate, and learned tools sit on top of the limit.

The package is `ch15-limits`, the library is `gw_limits`, and the bench is `limits`. Chapter 7's catalog and chapter 8's session gateway are vendored under `src/catalog/` and `src/session.rs`, so this step builds alone, with no path dependency on `ch07-catalog` or `ch08-session-toolsets`. The public names chapter 23 already calls stay: `Catalog`, `LimitedGateway` (`new`, `catalog`, `open_session`, `session_tools`), and `SessionView`; the crate binds no socket. `LimitedGateway` wraps the session view with a per-tool gate. `ACCEPT_BOUND` and `EXECUTE_BOUND` are the chapter 5 numbers, 32 and 8, now per tool. `CallError::Cap` is what that 51st search receives, and `dispatch` checks the breaker, the cap, the permit, and the timeout in that order.

A permit is an owned guard, so releasing it is a drop, including when a timeout wins. The breaker here is closed or open, and a half-open probe is the state left for later. `Result` is `CallError`, with `Cap` as its own variant, so a policy refusal stays distinct from a backend failure.

`cargo run -p ch15-limits` prints `result: ok` and then `breaker: open`, two fake calls, no key and no socket. `my-harness/tests/ch15.rs` is the 51st `web_search` as the cap error, and an open breaker that leaves the backend uncalled.

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

`LimitedGateway::new` wraps the chapter 8 session gateway; `catalog`, `open_session`, and `session_tools` are the same methods the smoke test already calls. The catalog is empty until something loads it.

`set_idempotent("echo", true)` marks one tool; the default is false, so a write does not retry.

`dispatch` checks the breaker first. An open breaker returns `CallError::OpenBreaker` and does not call the backend; then it checks the cap. The 51st `web_search` returns the search sentence and does not call the backend. The same is true for `delegate_task`, except `{"action":"list"}` (and `steer`, and `stop`) still run.

A transport failure on an idempotent tool calls the backend a second time. A 429 or a 503 does not, even when the tool is idempotent. A status such as 500 does retry once, and only then. A non-idempotent tool never gets that second call.

`CallOpts { hedge: true }` on an idempotent tool always makes the second call, and the first `Ok` wins. The same option on a write makes one call.

Three `503` results open the breaker for that tool name; a different tool is not open. A success in the middle clears the count, so two failures, one success, and two more failures stay closed.

`hold_accept` takes one of 32 accept permits; the 33rd is `Saturated`. `run_with_timeout` takes one accept permit and one execute permit, waits 30 milliseconds against a 5 second sleep, and drops both. The test finishes in under two seconds, and both counts are full again.

The demo marks `echo` idempotent; the script fails once with a transport error, then returns `ok`; three `503`s on `search` open the breaker. The next `search` prints `breaker: open`.

## Permits and the caps

### 15.1 One flat package, and the names chapter 23 already calls

The file is `steps/ch15-limits/src/lib.rs`; the package name is `ch15-limits`; the library name is `gw_limits`; a test writes `gw_limits::LimitedGateway`. The harness layout puts code under `crates/harness-core`; this server step keeps a flat `src/` tree:.

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

`pub mod catalog` loads `src/catalog/mod.rs`; `pub mod session` loads `src/session.rs`. I copied chapter 7 and chapter 8 into those files so the step builds alone. `Cargo.toml` has no path dependency on the earlier server crates.

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

`pub use crate::catalog::Gateway as Catalog` publishes the chapter 7 gateway as `Catalog`; `catalog::Gateway` and `Catalog` are the same type. Chapter 23 writes `gw_limits::Catalog`, `gw_limits::LimitedGateway`, and `gw_limits::SessionView`; those three stay. `SessionError` is also at the crate root, because the session module's own methods return it. The methods chapter 23 calls are `new`, `catalog`, `open_session`, and `session_tools`.

`LimitedGateway` owns a `SessionGateway`; that struct owns a `catalog::Gateway`; the limits code calls through `inner`.

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

`Gateway::new` stores a snapshot built from an empty card list, so `is_empty` is true; a fresh `LimitedGateway` stays empty until `reload`. The limits tests never load tools; the copied catalog and session tests do, through `gw_limits::catalog` and `gw_limits::session`.

### 15.2 `ACCEPT_BOUND`, `EXECUTE_BOUND`, and the cap of 50

```rust
/// Calls waiting or running for one tool. The 33rd is refused.
pub const ACCEPT_BOUND: usize = 32;
/// Calls inside one tool body at once.
pub const EXECUTE_BOUND: usize = 8;
pub const BREAKER_THRESHOLD: u32 = 3;
pub const MAX_WEB_SEARCHES: u32 = 50;
pub const MAX_SUBAGENTS: u32 = 50;
```

`ACCEPT_BOUND` is 32; `Semaphore::new` starts the accept counter there; `hold_accept` takes one permit at a time. The 33rd take is `CallError::Saturated`, which is what the comment says.

`EXECUTE_BOUND` is 8; `run_with_timeout` waits for one execute permit before the body runs; eight bodies can hold one at once. A ninth waits, after it already holds an accept permit.

`Semaphore::new` and `available_permits` use `usize`, so those two constants are `usize`. The streak and the caps live in `AtomicU32`, so the other three constants are `u32`. `web_searches.load(...) >= MAX_WEB_SEARCHES` type-checks because both sides are `u32`.

`BREAKER_THRESHOLD` is 3; `MAX_WEB_SEARCHES` and `MAX_SUBAGENTS` are 50. `block_cap` refuses `web_search` once that counter is already 50, so the 50th search is the last one that runs; a spawning `delegate_task` uses the same line. `list`, `steer`, and `stop` add zero; `format!("{MAX_WEB_SEARCHES}")` prints `50`, and the test names `gw_limits::ACCEPT_BOUND` directly.

## What a call returns

### 15.3 `FailureKind`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Transport,
    Status(u16),
}
```

`Transport` carries no data; the empty script queue uses it, and the retry test uses it for a failure that may run again. `Status` holds a `u16`; the tests pass `429` and `503`. `retriable` has a separate arm for every other status, so `Status(500)` is a different answer from `Status(503)`.

> **Rust sidebar: an enum is one variant**
>
> A `FailureKind` is either `Transport`, which carries nothing, or `Status(u16)`, and `retriable` takes it by value while the caller keeps its own. Enums are in [chapter 1](chapter-01-first-contact.md). The binary's panic prints `{other:?}`, which is why `CallError` and the kind inside `Failed` derive `Debug`.

### 15.4 `Outcome`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Ok(String),
    Fail(FailureKind),
}
```

There is no `Copy`; `String` is not `Copy`; moving `Outcome::Ok` moves the body string. `Clone` is still derived, so a caller who needs two copies can ask; `dispatch` moves the outcome.

`Ok` on this enum is my variant; `Result`'s `Ok` is a different type; `backend.call` returns `Outcome`; `dispatch` returns `Result<String, CallError>`. The last match in `dispatch` turns `Outcome::Ok(body)` into `Ok(body)`, and `Outcome::Fail(kind)` into `Err(CallError::Failed(kind))`.

`Outcome::Ok("ok".into())` allocates when the script is built; `"ok".into()` uses `From<&str> for String`.

### 15.5 `CallOpts`

```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct CallOpts {
    pub hedge: bool,
}
```

`hedge` is public; the hedge test builds `CallOpts { hedge: true }`. Every other call in the tests and the binary passes `CallOpts::default()`. `Default` on the struct is `Default` on each field, and `bool`'s default is `false`; a default call does not hedge.

`Copy` means `dispatch` receives a copy of the struct. The function reads `opts.hedge` twice: once inside the retry guard, and once as the hedge branch condition.

### 15.6 `CallError`

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
>
> `Result<String, CallError>` holds either the body or one `CallError`, and `try_accept(tool)?` either binds the permit or returns `Err(CallError::Saturated)`. `#[derive(Error)]` from `thiserror` is the `Display` text: `OpenBreaker` prints `open`, `Cap`'s `{0}` prints the block sentence, and `Failed` prints `failed`, while `assert_eq!` still sees `FailureKind::Status(429)` through `PartialEq`. `Result` is in [chapter 1](chapter-01-first-contact.md).

`dispatch` returns `OpenBreaker`, `Cap`, and `Failed`; the permit helpers return `Saturated` from `map_err`. `run_with_timeout` returns `Timeout` when the timer wins.

## The scripted backend

### 15.7 `Backend`

```rust
pub trait Backend: Send + Sync {
    fn call(&self, tool: &str) -> Outcome;
}
```

> **Rust sidebar: a trait, and `dyn`**
>
> A trait is the list of methods a type promises, which is why `Script` can implement `Backend` and `dispatch` can take `&dyn Backend` without naming `Script`. `Send + Sync` is the same bound chapter 3 put on `Tool`. `call` returns `Outcome` directly, and the only `async` function in this file is `run_with_timeout`, which never calls the backend. Traits are in [chapter 3](chapter-03-tools.md).

`Script::call` ignores `tool`; the gateway has already branched on the name: the breaker, the cap, and the idempotent flag are all keyed by that string.

### 15.8 `Script`

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

`new` takes the `Vec`; `into()` builds a `VecDeque`, and `pop_front` removes the oldest outcome. The first value I pushed is the first `call` returns.

`calls` starts at 0, and `fetch_add(1, Ordering::SeqCst)` adds one as a single step. The old value is unused, as the sidebar just above says, and `calls()` loads the same atomic. `lock().expect("script")` panics if the mutex is poisoned, which means a thread panicked while holding it. `pop_front` then `unwrap_or` yields `Fail(Transport)` on an empty deque. The open-breaker bench's idle script is that empty deque, and the open breaker returns before `call` pops it. `_tool` is required by the trait and unread here.

> **Rust sidebar: `AtomicU32`, `SeqCst`, and `Mutex`**
>
> An `AtomicU32` is a `u32` several tasks can update without a `Mutex`, and `fetch_add` adds one. I ignore the previous value `fetch_add` returns, here and in `note_cap` later, because the next `load` is the number that matters. Every atomic in this file uses `Ordering::SeqCst`, so that later load sees the earlier add. `call` takes `&self` and can't assign a plain field, so the deque sits behind a `Mutex` whose guard unlocks on drop. Both types are `Sync`, which is why `Script` can implement `Backend`.

## The gate and the breaker

### 15.9 `ToolGate`

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

The struct is private; `Semaphore::new(32)` and `Semaphore::new(8)` are different counters; filling accept leaves execute at 8.

> **Rust sidebar: `Arc` and a semaphore**
>
> `Arc` is a reference-counted pointer, so cloning a `ToolGate` clones both pointers and both gates share the same two semaphores. A semaphore counts permits: taking one subtracts, and dropping an `OwnedSemaphorePermit` adds one back, which is the release, with no separate give-back method. `try_acquire_owned` and `acquire_owned` take `Arc<Semaphore>` by value, the call site clones the `Arc`, and the permit keeps that clone while the map keeps its own. The first lookup of a tool name inserts one gate, a second name gets a second pair, and a full `echo` gate leaves `search` at 32 and 8. `Arc` is in [chapter 7](chapter-07-dynamic-catalog.md), where a reload swaps one.

### 15.10 `Breaker`

```rust
#[derive(Default)]
struct Breaker {
    failures: u32,
    open: bool,
}
```

`Default` is `failures: 0` and `open: false`; `or_default()` builds that on the first failure, then adds one; tests see `open` through `breaker_open`. They see a cleared streak by watching later failures stay closed. Once `open` is true, `note_success` leaves both fields alone.

## The surface chapter 23 calls

### 15.11 `LimitedGateway` and `new`

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

The `impl` continues through the methods below; `inner` is `SessionGateway::new()`, which is `Gateway::new()`; the catalog starts empty. The three maps start empty. A tool name is inserted on first use: `set_idempotent` inserts a bool, `gate` inserts a `ToolGate`, `note_failure` inserts a `Breaker`. A tool that has only succeeded has no breaker row. `breaker_open` treats a missing row as closed.

`web_searches` and `subagents` are one counter each, shared by every session on this gateway. This file never stores 0 back into either atomic; `new` is the reset; the fields are private.

### 15.12 `catalog`, `open_session`, and `session_tools`

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

`catalog` returns `&Catalog`; the inner method returns `&Gateway`; the alias makes them the same reference, borrowed for as long as `&self`. `gateway.catalog().is_empty()` is true on a new gateway.

`open_session` forwards the id. The inner method locks the session map, calls `session_id.into()`, and `entry(...).or_insert_with(Session::new)` inserts only when the id is new.

> **Rust sidebar: `impl Into<String>`**
>
> `impl Into<String>` means any one type that can become an owned `String`, so a literal allocates and an existing `String` moves. The test calls `open_session("session-1")`. `entry(...).or_insert_with(Session::new)` inserts only when the id is new, and a second open of the same id leaves the session in place. Same idea as `Msg::new` will use in [chapter 18](chapter-18-providers.md).

`session_tools` locks that map. `get` plus `?` returns `None` for an id that was never opened. `Some` copies `session_id`, clones `revealed` into `tools`, and copies `list_changed`.

> **Rust sidebar: `Option`**
>
> `Option<SessionView>` is `Some(view)` or `None`, and `get` is `None` when the id was never opened (see [chapter 1](chapter-01-first-contact.md)). `Some` clones the revealed names and copies `list_changed`. The limits test opens `"session-1"` and expects `tools` to be empty, because `revealed` is still empty. Chapter 23 reads `tools`.

### 15.13 `set_idempotent`

```rust
pub fn set_idempotent(&self, tool: &str, idempotent: bool) {
    self.idempotent
        .lock()
        .expect("policies")
        .insert(tool.to_string(), idempotent);
}
```

`to_string()` allocates the map key; `insert` replaces a previous bool for the same key. The hedge test sets `echo` to true and `write` to false; those are two keys. `expect("policies")` is the poison message. The guard drops at the end of the statement, so the lock is gone before `dispatch` calls the backend.

`set_idempotent("write", false)` writes the key even though a missing key is already treated as false; `is_idempotent` then finds `Some(false)`.

### 15.14 `is_idempotent`

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

`get` returns `Option<&bool>`; `copied()` turns that into `Option<bool>` because `bool` is `Copy`. `unwrap_or(false)` is the answer for a name nobody inserted. `search` in the breaker test was never marked, so this returns false, and a 503 on `search` does not retry.

The bool is copied out before the lock ends. `dispatch` holds that local across `backend.call`.

### 15.15 `breaker_open`

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

### 15.16 `accept_available` and `execute_available`

```rust
pub fn accept_available(&self, tool: &str) -> usize {
    self.gate(tool).accept.available_permits()
}

pub fn execute_available(&self, tool: &str) -> usize {
    self.gate(tool).execute.available_permits()
}
```

`gate` inserts a gate on first sight and returns a clone. `available_permits` reads the counter. It does not take a permit. A new tool reports 32 and 8. The timeout test calls both after the timer and expects `ACCEPT_BOUND` and `EXECUTE_BOUND` again.

### 15.17 `roundtrip_execute`

```rust
/// Take one execute permit and drop it. The bench times this pair.
pub fn roundtrip_execute(&self, tool: &str) -> Result<(), CallError> {
    let permit = self.try_execute(tool)?;
    drop(permit);
    Ok(())
}
```

`try_execute` returns an `OwnedSemaphorePermit` or `Err(Saturated)`. `?` returns the error. `drop(permit)` runs `Drop` now and adds the execute permit back. `Ok(())` carries no body. Accept is untouched. The bench loops this on `"echo"`, and each sample finds a free permit because the previous drop already ran.

### 15.18 `hold_accept`

```rust
pub fn hold_accept(&self, tool: &str) -> Result<OwnedSemaphorePermit, CallError> {
    self.try_accept(tool)
}
```

The caller owns the permit. The test stores 32 of them in a `Vec`. The 33rd call is `Err(Saturated)`. `drop(held)` drops each permit, and each `Drop` returns a slot, so `accept_available("echo")` is 32 again. The permit owns an `Arc<Semaphore>`, so the test can call `hold_accept` again while the vec still holds the earlier ones.

## Dispatch

### 15.19 `dispatch`

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

`args` is the raw JSON `spawn_count` parses, `opts` is copied in, and `backend` is `&dyn Backend`. The function is one order of refusals, then at most two calls, then one note.

While the breaker is closed, `breaker_open` is false and the function continues. After three failures it is open, and the first line returns `Err(CallError::OpenBreaker)`, so `block_cap`, `note_cap`, and `backend.call` stay uncalled. The fourth `search` in the breaker test takes this arm: `script.calls()` stays 3, and `Outcome::Ok("late")` stays in the deque. People often describe a breaker as three states, closed, then half-open for one probe, then open again if that probe fails. This struct has the closed streak and the open bool, and the half-open probe is the state left for later, which is why the next call is refused until the process is new. `breaker_open` locks and unlocks before the rest of the function, so on a closed path the backend runs and `note_failure` locks the map again afterward. The mutex isn't held across the backend.

The cap is the next refusal. `if let Some(message)` returns `Err(CallError::Cap(message))` before `note_cap` and before `backend.call`, so a blocked search doesn't add to `web_searches` and a blocked spawn doesn't add to `subagents`. `web_search` blocks once the counter is `>= 50`. `delegate_task` blocks when `spawn_count(args) > 0` and `subagents >= 50`. `list`, `steer`, and `stop` return a spawn count of 0, the guard fails, and the backend still runs.

`note_cap` then runs for every call that passed both checks, including a `web_search` whose backend later fails. `is_idempotent` copies the bool, and `backend.call` runs. That is the first real call.

What happens next depends on the tool and on `opts`. `matches!` tests `&first`, so `kind` is `&FailureKind`, and the retry guard is `idempotent && retriable(*kind) && !opts.hedge`. A write has `idempotent == false`, so a transport error is final and `calls()` stays 1. `retriable` is true for `Transport` and for any status other than 429 or 503, including 500, and false for 429 and 503, so an idempotent tool that returns either of those still calls the backend once. `!opts.hedge` keeps the hedge path from also setting this flag. An `Ok` fails the `Fail` pattern, so `retry` is false.

A hedge is the race the pitfalls name. `opts.hedge && idempotent` always makes a second `backend.call`, then matches `first`. `Outcome::Ok(body)` keeps that body and drops `second`, including its string, and the test expects `"first"` with `calls() == 2`. `Outcome::Fail(_)` moves `second` in. One `note_success` or one `note_failure` records the winner. A hedge on a non-idempotent tool fails the condition, `retry` is false, and the `else` keeps `first`, so that write sees `calls() == 1` and the body `"once"`.

When the retry flag is the one that fired, the function calls the backend once more and uses that second outcome. `first` drops at the end of the function. Echo scripts `Fail(Transport)` then `Ok("ok")`, returns `"ok"`, and `calls()` is 2. There is no loop. A second failure is the one failure `note_failure` records. Otherwise `first` is the outcome and one `backend.call` happened: the write, the 429, the 503, a plain success, or a hedge on a write.

The record is one note even when two backend calls ran. `Outcome::Ok(body)` calls `note_success` and returns `Ok(body)`, and the caller owns the string. `Outcome::Fail(kind)` calls `note_failure` and returns `Err(CallError::Failed(kind))`. The kind on the error is the kind that won.

> **Rust sidebar: `match` and `matches!`**
>
> `match` runs exactly one arm, and the last match covers `Ok` and `Fail`. Naming a field moves it, so `Outcome::Ok(body)` moves the `String` out. `matches!` returns a `bool` and doesn't leave `kind` bound afterward. A guard is the `if` on a pattern, and `*kind` copies the `FailureKind` because the match is on a reference and the enum is `Copy`. `if let Some(message)` is a one-arm match, and `None` skips the block. The same `match` shows up on phases in [chapter 10](chapter-10-turn-state.md).

### 15.20 `retriable`

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

## The caps of 50

### 15.21 `block_cap`

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

### 15.22 `note_cap`

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

`web_search` adds 1, and the old value `fetch_add` returns is unused, same as `Script::call`. `delegate_task` adds `spawn_count`: 0 for `list`, `steer`, and `stop`, 3 for three tasks, and 1 for `""` because that string isn't JSON. The delegate test uses `""` for each single spawn. `_ => {}` leaves `echo` and `search` alone.

`load` in `block_cap` and `fetch_add` here are two operations. On one thread the add from call N is what call N+1 loads. A blocked call never reaches this function, so single searches stop at 50. A batch allowed under the cap adds its full length, and that add can land past 50.

### 15.23 `spawn_count`

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
>
> `let Ok(value) = result else { return 1; }` binds `value` on `Ok`, and on `Err` the `else` block has to diverge, which `return` does. After the `let`, `value` is a parsed `serde_json::Value`. The same early-return form is in [chapter 14](chapter-14-when-things-fail.md), on `deterministic_empty`.

**Control actions return 0.** `get("action").and_then(Value::as_str)` is `Some` only for a string action. `to_ascii_lowercase` makes `"LIST"` match `"list" | "steer" | "stop"`. This return is before the `tasks` match, so a list action with a `tasks` array still counts as 0. Any other action, and a missing action, fall through. `{"tasks":[1,2,3]}` has no action, so it continues.

**A non-empty array returns its length.** `Some(Value::Array(items)) if !items.is_empty()` rejects `[]`. `items.len() as u32` turns the `usize` length into the atomic's type. Three items count as three subagents on one `dispatch`.

**Otherwise 1.** `_` is a missing `tasks`, a non-array, or an empty array. A non-empty batch used the length. Every other spawn that did not return 0 counts as one.

## Opening the breaker

### 15.24 `note_failure`

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

### 15.25 `note_success`

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

### 15.26 `gate`

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

## Taking and dropping a permit

### 15.27 `try_accept` and `try_execute`

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
>
> `try_acquire_owned` returns now: `Ok` is a permit, and `Err` means the counter was zero or the semaphore was closed, which is why the 33rd accept receives `Saturated` and isn't queued. `acquire_owned().await` waits, and dropping some other permit wakes it. Both consume an `Arc<Semaphore>` and return an `OwnedSemaphorePermit`, the owned value `hold_accept` returns and `run_with_timeout` holds across `.await`. A permit that borrowed `&Semaphore` would be tied to that borrow. `try_execute` is the non-waiting take the bench samples, and `run_with_timeout` waits, so a call that already holds an accept permit can wait for one of eight runners.

### 15.28 `run_with_timeout`

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
>
> `async fn` returns a future. The test builds a current-thread runtime and `block_on`s this function, while `dispatch` needs no runtime. `F: FnOnce() -> Fut` is a closure called once, the test passes `|| async { ... }`, and `Fut: Future<Output = String>` means that future resolves to a `String`. The `match` turns tokio's timeout error into `CallError::Timeout`. `.await` is the pause, on `acquire_owned` and again on `timeout`. `async` itself is in [chapter 3](chapter-03-tools.md), on `Tool::call`.

**Accept refuses.** `try_accept` runs first. A full accept semaphore returns `Saturated` through `?`, and execute is never awaited. The test's tool `"slow"` is a new name, so this line gets a permit.

**Execute waits.** The next lines clone the execute `Arc` and await `acquire_owned`. `"slow"` still has 8 permits, so the future completes on the first poll. `expect("execute semaphore")` panics if the semaphore is closed. An empty execute semaphore waits. Accept returns `Saturated`. Execute pauses. Both locals are `OwnedSemaphorePermit` values, and neither is moved into the body closure.

**The race.** `body()` starts the caller's future. `tokio::time::timeout` races it against `budget`. The test passes 30 milliseconds and a body that sleeps five seconds and would then return `"late"`. `Ok(value)` is the `String`, wrapped in my own `Ok`. `Err(_elapsed)` means the budget fired. That arm stores `Err(CallError::Timeout)` and does not leave the function. Tokio then drops the inner future. The sleep is that future, so the drop cancels it. `"late"` is never built. The test clock finishes under two seconds. This function returns `Timeout` and does not read that clock.

Dropping the body leaves `accept` and `execute` where they are. They live in `run_with_timeout`, outside `body()`.

**Why `drop` releases the permit.** `drop(execute)` and `drop(accept)` sit after the match, so they run when the body finishes and when the timer wins. `drop` takes the permit by value and runs `Drop` immediately. `OwnedSemaphorePermit`'s `Drop` adds one permit back to the semaphore held by the permit's `Arc`. Accept is 32 again. Execute is 8 again. The test reads both counters.

`result` is the return value. The `?` on a failed `try_accept` holds nothing. Once both permits exist, a winning timer does not cancel `run_with_timeout`: `timeout` returns `Err`, and this function drops the permits before it returns. `dispatch` never calls this function.

### 15.29 `Default` for `LimitedGateway`

```rust
impl Default for LimitedGateway {
    fn default() -> Self {
        Self::new()
    }
}
```

`default()` is `new()`. The catalog is empty, the maps are empty, and both counters are 0. `CallOpts` and `Breaker` derive `Default`. `LimitedGateway` writes the impl so callers share `new`. `SessionGateway` and the catalog `Gateway` do the same: `default` calls `new`.

### 15.30 `open_breaker` and `permit_release`

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

`open_breaker` times `dispatch` of `search` after three 503s, so the script is not called. `permit_release` takes one execute permit and drops it. Setup is outside the sample. The timeout stays a test: 30 milliseconds against a 5 second sleep.

The means are `mean.point_estimate` from [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `15`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

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

`my-harness/tests/ch15.rs` is the milestone. It compiles, then panics in `todo!("chapter 15")` until `chapter_15_web_search_allowed(49)` is true and `chapter_15_web_search_allowed(50)` is false. The fiftieth search is the last one allowed. 

## What's next

The harness line resumes at chapter 16, with web search and extract on a copy of chapter 14. This server track continues at chapter 23, where learned tools sit on these limits. Chapter 18, when the harness reaches it, is the provider: the same turn can build an Anthropic body, a Bedrock body, or an OpenRouter route, rotate a key without building a new client, and leave the side model alone unless the turn asked for a summary.
