# Chapter 37: Balancing and Federation

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 36 can restart a child that stopped making progress. That child may be waiting on a tool that lives on another MCP server, and the gateway still treats every backend for a tool as one call. If three upstreams offer `search`, the gateway should pick one that is not already busy, and it should not open a connection to all of them while it decides. This chapter adds the pool and the federation map. It is the server line, not the harness line. The crate is `steps/ch37-balancing`. It vendors chapter 23, library name `balancing`, new module `federation`. It binds no socket. Chapter 38 copies chapter 36. It does not copy this directory.

`Policy` is least-outstanding or power-of-two choices. `Backend` is one upstream in the pool. `pick` chooses without opening a connection, which is the property the bench locks. `call` formats a string and puts the counter back, so a finished call decrements the outstanding count the next pick will see. `federate` and `pool_for` group one tool name to the server names that provide it. `discover` counts overlap and connects at most 4, the discovery concurrency Hermes uses. `0` as "unlimited" stays unimplemented. A live socket to those upstreams stays unimplemented too. The pool is in memory.

The server stays a separate program because the pool is a property of the upstreams, and every harness client should share it. If each agent balanced for itself, two agents would both pick the same idle backend and then both discover it was busy. The gateway is the place that can see every in-flight call. The harness remains the client: it asks for a tool by name, and the server chooses the backend. Chapter 42 vendors this crate and adds routing, drain, and quotas. Chapter 15's breaker still sits under the call. A pool that ignores an open breaker would send traffic at a backend the limits chapter already decided to leave alone.

Nothing here learns a skill. The connection to the self-improving harness is indirect and real: a team of agents calling tools only works if the catalog stays up. Federation is how the catalog grows to include tools the gateway does not contain, which is the same split chapter 4 drew for a single server, now with more than one upstream and a cap on how many you ask at once.

Rust, at the pick: the outstanding count is an integer you increment and decrement. Power of two is "sample two, take the quieter one," a few lines, not a load-balancer framework. The discovery cap is a semaphore of 4. `Result` is a pool that has no backend for that name.

`cargo run -p ch37-balancing --offline` prints the demo line. The exercise is `my-harness/tests/ch37.rs`: a least-outstanding pick does not open a connection.

**In this chapter**

- 37.1 The crate around the new module
- 37.2 `Policy`, `Backend`, and `pick`
- 37.3 `call` formats a string and puts the counter back
- 37.4 `federate` and `pool_for` group one tool
- 37.5 `discover` counts overlap, capped at 4
- 37.6 `FakeModel` and the demo line

| Section | Step | What it adds |
|---|---|---|
| 37.1 | `steps/ch37-balancing` | the vendored chapter 23 crate, and which `demo` runs |
| 37.2 | `steps/ch37-balancing` | `Policy`, `Backend`, `Pool::pick` |
| 37.3 | `steps/ch37-balancing` | `Pool::call` |
| 37.4 | `steps/ch37-balancing` | `Upstream`, `federate`, `pool_for` |
| 37.5 | `steps/ch37-balancing` | `discover` and the cap of 4 |
| 37.6 | `steps/ch37-balancing` | `FakeModel`, `federation::demo`, the balancing tests |

> **How Hermes does it**
>
> I searched the Hermes tree at `9e01b44` for a least-outstanding pick, a power-of-two pick, a backend pool, and federation. I did not find a function that chooses the quietest replica of one MCP tool, and I did not find a power-of-two-choices picker. Other lines that contain those words are not this pool, and I am not going to describe files I did not open. The match I could adopt, and the files I opened for it, are discovery concurrency in `tools/mcp_tool_discovery.py` and the `mcp` default in `hermes_cli/config_defaults.py`.
>
> I read `tools/mcp_tool_discovery.py` and the `mcp` block in `hermes_cli/config_defaults.py`.
>
> ```text
> 25  # Default max concurrent MCP server connections per discovery pass
> 27  # ``mcp.discovery_concurrency`` overrides it, 0 = unlimited (#117373).
> 28  _DISCOVERY_CONNECT_CONCURRENCY = 4
> ```
>
> `_discovery_connect_concurrency` reads `mcp.discovery_concurrency`. A non-integer, a bool, or a negative value warns and returns 4. The comment on the constant says 0 means unlimited. `config_defaults.py` sets that key to 4, in the `mcp` object, and the comment there says the same thing: max servers connected at once per discovery pass, and 0 means unlimited. `_discover_all` builds the bound:
>
> ```text
> 442  # Flat cap for all transports; 0 (unlimited) keeps the original unbounded gather.
> 443  cap = _discovery_connect_concurrency()
> 444  semaphore = asyncio.Semaphore(cap) if cap > 0 else None
> ```
>
> A positive cap is an `asyncio.Semaphore`. Zero leaves `semaphore` as `None`, and `_connect_bounded` then awaits the connect with no semaphore. That is the unlimited setting.
>
> **Adopted.** The discovery cap of 4. Six overlapping connects report a max in flight of 4. The constant in this crate is `DISCOVERY_CONCURRENCY`.
>
> **Adapted.** Least outstanding, and the quieter of two indexes, are this chapter's pool. I did not find them in the MCP client. Federation is a `HashMap` from tool name to server names, built from an in-memory list. Outstanding is an `AtomicUsize`. `pick` only loads it. `call` adds one, formats `reply via server`, and subtracts one. `discover` is threads, a mutex, and a condvar. The test's `hold` is a `thread::sleep` so the workers overlap. It is not a TCP connect. `0` as unlimited is not implemented. The gate always uses 4.
>
> **Deferred.** A live upstream connection, the unlimited override, and session routing. Chapter 42 is the service that routes by session id. The pick stays a local read.

[personal note: a tool that had one backend until a second server offered the same name]

## What this chapter decides

`pick` with `LeastOutstanding` returns the index of the smallest `outstanding` counter. Ties keep the earlier backend. `PowerOfTwo { a, b }` reduces both indexes modulo the pool length and returns the quieter one. An equal pair keeps the left index. An empty pool returns `None`.

`call` picks, adds one to that counter, formats `reply via server`, and subtracts one. The counter is back where it started when `call` returns. The pick does not open a connection. `call` does not open one either. Both return data that was already in the `Backend`.

`federate` builds the map. `pool_for` turns one tool's servers into a pool whose reply string is the tool name and whose counters start at 0.

`discover` starts `n` threads and holds each one inside a gate of 4. The return is the max in flight. The function does not open a socket.

`federation::demo` federates `echo` on `alpha` and `beta` and returns `pick: echo via alpha`. No key. That is the line `main` prints.

## 37.1 The crate around the new module

`src/lib.rs` is the chapter 23 gateway, copied in so the step builds alone. The module docs say a prediction is installed before the request, `session_tools` reads that set, composite and macro names expand to steps, and code mode runs a line script against an allowlist. The new line in that file, beside the older modules, is `pub mod federation`.

The public surface I am not re-teaching is the vendored one. `catalog` is the chapter 7 gateway: native tools, plugin tools, upstream tool cards, and a BM25 index. `session` is the chapter 8 session view over that catalog. `limits` is the chapter 15 gateway: accept and execute permits, an idempotent retry, a breaker, and the caps of 50. `lib.rs` re-exports `Catalog`, `LimitedGateway`, `SessionError`, and `SessionView` from `limits`. On top of that sit the learned types: `Use`, `PredictInput`, `CodeError`, `ScriptedModel`, `LearnedGateway`, `predict`, `SANDBOX_ALLOWED`, and `CODE_MODE_MAX_CALLS` of 50. `LearnedGateway::session_tools` clones the installed offer. `refresh_prediction` is the offline score. `run_code_mode` accepts a line that starts with `call `, refuses a tool that is not in `SANDBOX_ALLOWED` or not in the offer, and stops after 50 calls.

`pub fn demo()` in `lib.rs` opens a session named `desk`, predicts from a usage row for `read_file`, reads `session_tools`, and runs one `call read_file` line. It returns `(offered, bodies)`, a tuple. `main` does not call it. `tests/learned.rs` exercises that path: the prediction test, a zero-score exclusion, composite and macro expansion, the allowlist and the offer, the cap of 50, and the keyless scripted model. Those tests belong to the vendored gateway. The new behavior is `tests/balancing.rs`.

`catalog::Upstream` is a name plus tool cards for the in-memory catalog. `federation::Upstream` is a server name plus a `Vec<String>` of tool names for the pool map. Same word, two types, two modules. The pool in this chapter is not `LimitedGateway`'s permit pool. That one limits how many calls run. This one chooses which backend would run, and then does not run a connection.

The vendored constants and the learned types are what `tests/learned.rs` calls. They are in the crate so a reader who opens `lib.rs` can see them beside `pub mod federation`. They are not what `main` prints.

```rust
pub const SANDBOX_ALLOWED: &[&str] = &[
    "web_search",
    "web_extract",
    "read_file",
    "write_file",
    "search_files",
    "patch",
    "terminal",
];

pub const CODE_MODE_MAX_CALLS: usize = 50;
```

`SANDBOX_ALLOWED` is seven names. `run_code_mode` rejects any other tool with `CodeError::NotAllowed`. A name that is allowed and still missing from the offered slice is `CodeError::NotOffered`. A line that does not start with `call `, or that has no whitespace after the tool, is `CodeError::BadLine`. More than 50 calls is `CodeError::TooManyCalls`. The function parses every line into a local vec first. After each accepted line it checks `lines.len() > CODE_MODE_MAX_CALLS`. The 51st accepted line returns that error immediately. The callback runs only after the whole script has parsed, in the `Ok` arm, so a script that trips the cap invokes the callback zero times. Blank lines and lines whose first non-whitespace character is `#` are skipped before that parse.

`Use` is one row of history: a task string and a tool string. `PredictInput` borrows the task, the usage slice, the memory text, a session note, and the catalog names. `predict` scores each catalog name. A usage row whose tool equals the name and whose task shares an alphanumeric token of length at least 3 with the new task adds 1. The memory text or the session note containing the tool name as a whole word adds 3. A score of 0 drops the name. The rest sort by score descending, then by name. `refresh_prediction` calls `predict`, stores the `Arc<Vec<String>>` on the gateway, sets `pending_change`, and increments `predict_calls`. `session_tools` clones that `Arc`'s vec into the view when the offer is non-empty, and swaps `pending_change` to false. It does not call `predict`. The comment on `offered_arc` says the request path clones the installed `Arc` and does not score usage.

`ScriptedModel` holds a reply and a call count. `complete` increments the count and clones the reply. The learned `demo` does not construct it. `federation::demo` constructs a different `FakeModel`, the one in `federation.rs`. `LearnedGateway::new` starts from `LimitedGateway::new`, an empty offer, `pending_change` false, `predict_calls` at 0, and two empty `BTreeMap`s for composites and macros. `save_composite` and `save_macro` insert a name and a step list. `expand` looks at composites first and macros second. `Default` calls `new`.

`limits.rs`, which I read at the top of the file, is the chapter 15 copy inside this crate. `ACCEPT_BOUND` is 32. `EXECUTE_BOUND` is 8. `BREAKER_THRESHOLD` is 3. `MAX_WEB_SEARCHES` and `MAX_SUBAGENTS` are 50. `Backend` there is a trait with `fn call(&self, tool: &str) -> Outcome`. That trait is not `federation::Backend`. The federation struct has no `call` method of its own. `Pool::call` is the method. The trait lives on the vendored gateway, and the federation pool does not implement it.

Tokio is a dependency because that vendored gateway uses `tokio::sync::Semaphore`. The features on the dependency are `macros`, `rt-multi-thread`, `sync`, and `time`. `federation.rs` does not import tokio. `discover` uses `std::thread`. The pick uses `std::sync::atomic`. A reader who searches the new module for `TcpStream` will not find it.

> **Rust sidebar: two crates, two names**
>
> - Chapter 36's library is `harness_core`. This package's library is `balancing`. A test says `use balancing::federation::...`.
> - `pub mod federation` makes the module public. `main.rs` can name `balancing::federation::demo` because the binary is the same package and the module is `pub`.
> - Vendoring means the files live in this crate. There is no `path` dependency on `steps/ch23-learned-tools`. Editing chapter 23 later does not change this copy.

## 37.2 `Policy`, `Backend`, and `pick`

The module starts with the cap and the policy.

```rust
/// Hermes `mcp.discovery_concurrency` default, from `tools/mcp_tool_discovery.py`.
pub const DISCOVERY_CONCURRENCY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    LeastOutstanding,
    PowerOfTwo { a: usize, b: usize },
}
```

`DISCOVERY_CONCURRENCY` is 4, a `usize`, because it counts threads inside the gate. `Policy` is `Copy`. `LeastOutstanding` has no fields. `PowerOfTwo` carries the two indexes the caller already chose. This crate does not draw random numbers. A caller who always passes `0` and `1` never looks at the rest of the pool. The randomness, if there is any, stays outside `pick`.

`Backend` is one upstream in the pool.

```rust
pub struct Backend {
    pub server: String,
    pub reply: String,
    outstanding: AtomicUsize,
}

impl Backend {
    pub fn new(server: impl Into<String>, reply: impl Into<String>, outstanding: usize) -> Self {
        Self {
            server: server.into(),
            reply: reply.into(),
            outstanding: AtomicUsize::new(outstanding),
        }
    }

    pub fn outstanding(&self) -> usize {
        self.outstanding.load(Ordering::Relaxed)
    }
}
```

`server` and `reply` are public strings. `outstanding` is private. Readers use `outstanding()`, which loads the atomic. `new` can start the counter at any `usize`. The bench and the least-outstanding test start four backends at 3, 1, 4, and 2. `pool_for` starts every counter at 0. There is no connection field. There is no host, no port, and no client. The reply is the string `call` will format.

`Pool` owns the vec.

```rust
pub struct Pool {
    backends: Vec<Backend>,
}

impl Pool {
    pub fn new(backends: Vec<Backend>) -> Self {
        Self { backends }
    }

    pub fn len(&self) -> usize {
        self.backends.len()
    }

    pub fn is_empty(&self) -> bool {
        self.backends.is_empty()
    }

    pub fn pick(&self, policy: Policy) -> Option<usize> {
        if self.backends.is_empty() {
            return None;
        }
        Some(match policy {
            Policy::LeastOutstanding => self
                .backends
                .iter()
                .enumerate()
                .min_by_key(|(_, backend)| backend.outstanding())
                .map(|(index, _)| index)
                .unwrap_or(0),
            Policy::PowerOfTwo { a, b } => {
                let left = a % self.backends.len();
                let right = b % self.backends.len();
                if self.backends[left].outstanding() <= self.backends[right].outstanding() {
                    left
                } else {
                    right
                }
            }
        })
    }
}
```

`len` and `is_empty` read the vec. `pick` takes `&self`. It does not need `&mut self`, because the counters are atomics and this function only loads them. An empty pool returns `None` before the `match`. The `Some(...)` wraps every other result.

`LeastOutstanding` enumerates `(index, backend)` and asks `min_by_key` for the smallest `outstanding()`. `min_by_key` keeps the first element when several keys tie, because it replaces the current minimum only when the new key is strictly smaller. Outstanding counts 3, 1, 4, 2 yield index 1, the backend whose counter is 1. If two backends both held 1, the earlier index would win. The tests use distinct counts, so the tie is the rule in `min_by_key`, not a row in the test. The `unwrap_or(0)` sits behind the empty check. On a non-empty pool `min_by_key` returns `Some`. The `0` is the value the expression would use if the iterator were empty, which this arm has already refused.

`PowerOfTwo` reduces `a` and `b` with `% self.backends.len()`. An index past the end wraps. The test uses indexes inside the pool: `{ a: 0, b: 1 }` and `{ a: 2, b: 0 }` on a pool of three. The comparison is `<=`. The left index wins a tie. `{ a: 0, b: 1 }` with counters 5 and 1 returns 1. `{ a: 2, b: 0 }` with counters 4 at index 2 and 5 at index 0 returns 2, because 4 is less than 5. Swapping the field order would swap which side a tie prefers. The test's pairs are not ties.

`pick` does not call `call`. It does not format a reply. It does not read `server` except through the index it returns. It does not open a socket. The milestone is that fact: a least-outstanding pick does not open a connection. The value that comes back is `Option<usize>`.

The bench's four backends are the same numbers as `least_outstanding_picks_the_quietest_backend`. Walk them once. Index 0 is server `a`, outstanding 3. Index 1 is `b`, outstanding 1. Index 2 is `c`, outstanding 4. Index 3 is `d`, outstanding 2. `min_by_key` starts at index 0 with key 3. Index 1 has key 1, which is smaller, so the minimum becomes 1. Index 2 has key 4, which is not smaller. Index 3 has key 2, which is not smaller than 1. The result is `Some(1)`. Four loads, one index. The servers' reply strings are never read.

The power-of-two test is a pool of three: `a` at 5, `b` at 1, `c` at 4. `PowerOfTwo { a: 0, b: 1 }` compares index 0 (5) with index 1 (1). `5 <= 1` is false, so the result is the right index, 1. `PowerOfTwo { a: 2, b: 0 }` compares index 2 (4) with index 0 (5). `4 <= 5` is true, so the result is the left index, 2. Neither call wraps, because both indexes are already below the length. An index of 5 on this pool would wrap: `5 % 3` is 2, the same slot as `c`. The test does not pass 5. The wrap is what the `%` line does when a caller does.

> **Rust sidebar: `min_by_key` and `%`**
>
> - `enumerate` yields `(0, first)`, then `(1, second)`, and so on. The key function receives `(_, backend)` and returns `backend.outstanding()`, a `usize`. The smallest key wins. The index is what `map` keeps.
> - `%` is the remainder. `5 % 3` is 2. On a pool of length 3, index 5 names the same slot as index 2. A length of 0 never reaches this arm, because the empty check returned `None`. Dividing by zero is not this function's path.
> - `Ordering::Relaxed` on the load means the read of the counter does not fence other memory. `pick` does not read anything else that would need a fence. The counter is the whole decision.

## 37.3 `call` formats a string and puts the counter back

```rust
/// Pick, count one outstanding request, return the scripted reply, then drop the count.
pub fn call(&self, policy: Policy) -> Option<String> {
    let index = self.pick(policy)?;
    let backend = &self.backends[index];
    backend.outstanding.fetch_add(1, Ordering::Relaxed);
    let reply = format!("{} via {}", backend.reply, backend.server);
    backend.outstanding.fetch_sub(1, Ordering::Relaxed);
    Some(reply)
}
```

`?` on `Option` returns `None` when `pick` returns `None`. An empty pool makes `call` return `None` without touching a counter. On a hit, `fetch_add(1)` raises the chosen counter, the format builds `reply via server`, and `fetch_sub(1)` lowers it again. For the test pool, `call(LeastOutstanding)` returns `Some("echo via b")`. A second `pick` still returns `Some(1)`, because the counter is back at 1. The in-flight bump exists only between the add and the sub. A caller who samples `outstanding()` after `call` returns sees the idle number.

There is no `await` between the add and the sub. The reply is a format of two strings that were already on the struct. No `TcpStream`, no `std::net`, no process. The comment on `FakeModel` says `call` does not open a socket. The same is true of `pick`, which `call` uses and which the bench calls alone.

Two threads can both `pick` index 1 before either `fetch_add` is visible to the other, because `pick` and the add are not one critical section. Each would then add. The tests call `pick` and `call` on one thread. The bench calls `pick` only, so its counters stay at 3, 1, 4, and 2 for every sample.

> **Rust sidebar: `?` on `Option`**
>
> - `pick` returns `Option<usize>`. `let index = self.pick(policy)?;` means "if this is `None`, return `None` from `call`". The rest of the function sees a `usize`.
> - `fetch_add` and `fetch_sub` return the previous value. `call` ignores those returns. The string is the result.
> - `format!` allocates a new `String`. That allocation is the work `call` does beyond the atomic updates. `pick` does not allocate that string. The bench stays on `pick`.

## 37.4 `federate` and `pool_for` group one tool

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upstream {
    pub server: String,
    pub tools: Vec<String>,
}

/// Tool name to the servers that offer it. Order follows the upstream list.
pub fn federate(upstreams: &[Upstream]) -> HashMap<String, Vec<String>> {
    let mut pools: HashMap<String, Vec<String>> = HashMap::new();
    for upstream in upstreams {
        for tool in &upstream.tools {
            pools
                .entry(tool.clone())
                .or_default()
                .push(upstream.server.clone());
        }
    }
    pools
}
```

`Upstream` is public and plain. `federate` walks the slice in order. For each tool name it takes the `HashMap` entry, defaulting to an empty vec, and pushes the server name. A tool offered by `alpha` and then `beta` gets a vec `["alpha", "beta"]`. A tool offered only by `alpha`, such as `time` in the test, gets `["alpha"]`. The vec order is insertion order, which is the upstream slice order. `HashMap` iteration order is not that vec's order. The test reads `map.get("echo")`, which is one key, and compares the vec.

`entry` clones the tool name because the key is an owned `String`. `or_default` uses `Vec`'s default, an empty vec, when the key is new. `push` clones the server name so the map owns it. The input slice is only borrowed.

```rust
pub fn pool_for(tool: &str, upstreams: &[Upstream]) -> Pool {
    let servers = federate(upstreams).remove(tool).unwrap_or_default();
    Pool::new(
        servers
            .into_iter()
            .map(|server| Backend::new(server.clone(), tool, 0))
            .collect(),
    )
}
```

`pool_for` federates and then `remove`s one key. `remove` returns `Option<Vec<String>>`. A missing name becomes an empty vec through `unwrap_or_default`, and `Pool::new` of an empty vec is an empty pool. `is_empty` is then true, and `pick` returns `None`. The test checks `pool_for("missing", &upstreams).is_empty()`.

Each server becomes a `Backend` whose reply is the tool name and whose counter is 0. The server string is cloned into `new` because `Backend::new` takes `impl Into<String>` and the `map` closure would otherwise move the only copy. For `echo` on `alpha` then `beta`, the pool length is 2, both counters are 0, and least outstanding returns index 0. `call` formats `echo via alpha`. Ties at 0 keep the earlier server. That is the demo's pick.

> **Rust sidebar: `entry` and `remove`**
>
> - `entry(key)` is one lookup that can insert. `or_default()` inserts the default if the key is absent and returns a mutable reference to the vec. `push` writes the server onto that vec.
> - `remove` takes the vec out of the map. The rest of the map is dropped at the end of the statement, including every other tool's server list. `pool_for` only needed one key.
> - `HashMap` does not promise that `keys()` comes back in upstream order. The order that matters is inside the `Vec<String>` for that tool, and `pool_for` preserves it by walking that vec.

## 37.5 `discover` counts overlap, capped at 4

`Gate` is private. Tests reach it only through `discover`.

```rust
struct Gate {
    inflight: Mutex<usize>,
    max_seen: Mutex<usize>,
    cv: Condvar,
}

impl Gate {
    fn enter(&self) {
        let mut inflight = self.inflight.lock().expect("inflight");
        while *inflight >= DISCOVERY_CONCURRENCY {
            inflight = self.cv.wait(inflight).expect("inflight");
        }
        *inflight += 1;
        let mut max_seen = self.max_seen.lock().expect("max");
        *max_seen = (*max_seen).max(*inflight);
    }

    fn leave(&self) {
        let mut inflight = self.inflight.lock().expect("inflight");
        *inflight = inflight.saturating_sub(1);
        self.cv.notify_one();
    }
}
```

`enter` locks `inflight` and waits while the count is already at `DISCOVERY_CONCURRENCY`. The wait is `cv.wait`, which unlocks, parks the thread until a notify, and relocks. The `while` checks again, because a notify means some waiter should look, not that this waiter owns a free slot. Then `enter` adds one. It keeps the inflight lock while it locks `max_seen`, and stores the max of the old high-water mark and the current count. `leave` subtracts one with `saturating_sub`, so a leave at zero would stay at zero, and `notify_one` wakes a single waiter. The others stay in `wait` until a later `leave`.

Six workers and a cap of 4, with a hold long enough that they overlap, look like this. Workers 1 through 4 pass the `while`, each increment, and `max_seen` climbs to 4. Workers 5 and 6 block inside `wait`. When worker 1 leaves, inflight drops to 3 and one waiter enters, inflight returns to 4, and `max_seen` stays 4. The function returns that high-water mark, not the count at the end. At the end, after `thread::scope` joins, inflight is 0. The test asserts the high-water mark.

Hermes does the same shape with a different runtime. `_discover_all` sets `semaphore = asyncio.Semaphore(cap) if cap > 0 else None`. Each connect takes the semaphore when `cap` is positive. A cap of 0 stores `None` and awaits the connect with no semaphore. `discover` has no `cap` argument and no `None` semaphore. The comparison in `enter` is always `>= 4`.

```rust
pub fn discover(n: usize, hold: Duration) -> usize {
    let gate = Gate {
        inflight: Mutex::new(0),
        max_seen: Mutex::new(0),
        cv: Condvar::new(),
    };
    thread::scope(|scope| {
        for _ in 0..n {
            scope.spawn(|| {
                gate.enter();
                if !hold.is_zero() {
                    thread::sleep(hold);
                }
                gate.leave();
            });
        }
    });
    let seen = *gate.max_seen.lock().expect("max");
    seen
}
```

`thread::scope` joins every spawned thread before it returns. `n` is how many connects the test asked for. Each one enters, optionally sleeps `hold`, and leaves. The sleep is the overlap. A zero `hold` skips it. The function still refuses a fifth `enter` while four are inside. With a zero hold the threads may not overlap, so `max_seen` can come back below 4. The test passes `Duration::from_millis(40)` and `n` of 6 so they do overlap. The first four enter and sleep. The fifth and sixth block in `wait`. `max_seen` becomes 4 and stays 4. The return is 4. The test asserts that return equals `DISCOVERY_CONCURRENCY`, and asserts the constant equals 4.

Nothing in this function opens a socket. The "connect" is `enter` and `leave`. Hermes's `_discover_all` really does connect, under an `asyncio.Semaphore` of the configured cap, and a cap of 0 skips the semaphore. This gate has no argument for 0. Four is the constant. A caller who wanted unlimited concurrency will still see a max of 4 whenever the workers overlap.

`pick` does not call `discover`. The bench does not call `discover`. The demo does not call `discover`. The cap and the pick are two public functions in one module.

> **Rust sidebar: `thread::scope`**
>
> - `scope` lets the spawned closures borrow `gate` from the stack. They cannot outlive the scope. When the closure passed to `thread::scope` ends, the workers have finished.
> - `Condvar::wait` takes the mutex guard and returns a new guard. The `while *inflight >= 4` loop rechecks after every wake. A wake is not by itself proof that a slot is free.
> - `Duration::from_millis(40)` is the test's hold. `is_zero` is how a zero hold skips `thread::sleep`. The production pick path does not construct a `Duration`.

## 37.6 `FakeModel` and the demo line

```rust
/// Scripted upstream replies. `call` does not open a socket.
pub struct FakeModel {
    pub reply: String,
}

impl FakeModel {
    pub fn complete(&self) -> String {
        self.reply.clone()
    }
}

pub fn demo() -> String {
    let upstreams = [
        Upstream {
            server: "alpha".into(),
            tools: vec!["echo".into()],
        },
        Upstream {
            server: "beta".into(),
            tools: vec!["echo".into()],
        },
    ];
    let pool = pool_for("echo", &upstreams);
    let model = FakeModel {
        reply: "echo".into(),
    };
    let picked = pool
        .call(Policy::LeastOutstanding)
        .unwrap_or_else(|| model.complete());
    format!("pick: {picked}")
}
```

`FakeModel` in this module is not the harness `FakeModel`. It has one public `reply` and `complete` clones it. `demo` builds it as a fallback. `pool_for("echo", ...)` yields two backends, both at outstanding 0, so `call` returns `Some("echo via alpha")`. `unwrap_or_else` does not call `complete` when the option is `Some`. The format is `pick: echo via alpha`. An empty pool would fall through to `model.complete()` and print `pick: echo`. This input does not. No key is read. No socket is opened.

`main` prints that string and exits.

```rust
use balancing::federation::demo;

fn main() {
    println!("{}", demo());
}
```

## Tests

`tests/balancing.rs` is the new file. `tests/learned.rs` is the vendored chapter 23 suite. I am not going to invent a combined total.

`least_outstanding_picks_the_quietest_backend` builds counters 3, 1, 4, and 2. `pick(LeastOutstanding)` is `Some(1)`. `call` is `Some("echo via b")`. A second `pick` is still `Some(1)`.

`power_of_two_picks_the_quieter_of_the_pair` uses counters 5, 1, and 4. `PowerOfTwo { a: 0, b: 1 }` is `Some(1)`. `PowerOfTwo { a: 2, b: 0 }` is `Some(2)`. The same pool's least-outstanding pick is `Some`. An empty `Pool` picks `None`.

`federation_groups_a_tool_across_upstreams` puts `echo` and `time` on `alpha` and `echo` on `beta`. `federate` returns `["alpha", "beta"]` for `echo`. `pool_for("echo")` has length 2 and `call` returns `echo via alpha`. `pool_for("missing")` is empty.

`discovery_stays_at_four` calls `discover(6, Duration::from_millis(40))` and expects 4, and expects the constant to be 4.

`the_demo_picks_alpha` expects `balancing::federation::demo()` to equal `pick: echo via alpha`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch37-balancing
```

```text
    Starting 11 tests across 4 binaries
     Summary [   0.085s] 11 tests run: 11 passed, 0 skipped
```

```bash
cargo run -p ch37-balancing --offline
```

```text
pick: echo via alpha
```

The package has one binary, so the run does not need `--bin`. The line is the federation demo. No key.

```bash
cargo bench -p ch37-balancing --bench balancing --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `pick_least` | 3.74 ns | a local pick among a pool | the quietest of four backends |

`pick_least` is `benches/balancing.rs`. It builds four backends before the bench, with outstanding counts 3, 1, 4, and 2. The timed closure is `b.iter`. Inside: `pick(Policy::LeastOutstanding)` passed through `black_box`, then `black_box` of the index. Each sample loads four atomics and returns index 1. `call` is not used, so the counters do not change and the `via` string is not allocated. No thread is started. `discover` is not called. No socket is opened. `FakeModel` is not constructed. `LearnedGateway` is not constructed. The prediction path is not on this bench.

The mean is `3.74 ns`. It is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `37`. The id is `37/pick_least`. The old id `35/pick_least` is an alias in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `pick: echo via alpha`. The milestone is the pick itself: least outstanding does not open a connection. The index is a `usize` into a vec of strings and atomics.

## Pitfalls

`call` drops the outstanding count before it returns. A metric that samples after `call` sees the idle number. The bump is visible only between `fetch_add` and `fetch_sub`, which for a scripted reply is the format of one string.

`pick` and the add are separate. Two overlapping `call`s can choose the same index. The tests are single-threaded. The bench never adds.

Power of two is only as random as the indexes the caller passes. This crate does not draw them.

Discovery with a zero hold can show a max below 4, because the threads may not overlap. The test holds for 40 milliseconds so they do. The cap is still 4 when they overlap. It does not raise the count above 4.

Hermes treats `discovery_concurrency: 0` as unlimited, by skipping the semaphore. This gate does not. Four is the constant. There is no parameter to pass 0.

`lib.rs`'s `demo` and `federation::demo` are different functions. The binary prints the federation line. A test that calls `balancing::demo()` is on the learned-tools tuple, not on `pick: echo via alpha`.

`catalog::Upstream` will not compile in a `federate` call. The federation struct is `balancing::federation::Upstream`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Pick.** Least outstanding, or the quieter of two indexes. The choice loads atomics. It does not call an upstream. It does not open a socket. That is the milestone.

**Discovery.** At most 4 connects in flight. That is `_DISCOVERY_CONNECT_CONCURRENCY` and the default `mcp.discovery_concurrency` of 4. Hermes's `0` means unlimited, through a missing semaphore. This chapter does not implement that override.

**Federation.** A tool's pool is the list of servers that offer it, in the order the upstreams were given. A missing name is an empty pool, and an empty pool picks `None`.

**Vendored limits.** The copied chapter 15 gateway still has its own permits, breaker, and caps of 50. Those guard a call into `LimitedGateway`. They are not the outstanding counters on `Backend`.

**Left for later.** Live sockets, the unlimited discovery setting, and session routing are chapter 42. The scripted reply is not redacted.

## What you implement

`my-harness/tests/ch37.rs` is the milestone. The step crate is the reference. The stub stays `todo!`.

```rust
use my_harness::chapter_37_pick_opens_connection;

#[test]
fn least_outstanding_pick_does_not_open_a_connection() {
    assert!(!chapter_37_pick_opens_connection());
}
```

The test wants `chapter_37_pick_opens_connection()` to be false. A least-outstanding pick returns an index. It does not open a connection.

The stub panics.

```rust
pub fn chapter_37_pick_opens_connection() -> bool {
    todo!("chapter 37")
}
```

`todo!("chapter 37")` is the panic. Filling the function in is the reader's work. Copying `steps/ch37-balancing` into `my-harness/` is not the milestone.

## Chapter summary

- **Crate.** Server lineage. Library `balancing`. Chapter 23 is vendored. The new module is `federation`. `main` prints `federation::demo()`.
- **Pick.** Least outstanding, or the quieter of two indexes. Ties on least outstanding keep the earlier backend. An empty pool is `None`. No socket.
- **Call.** Add one, format `reply via server`, subtract one.
- **Federation.** A map from tool name to server names. `echo` on `alpha` and `beta` picks `alpha` when both counters are 0.
- **Discovery.** Four at a time. Unlimited `0` stays unimplemented.
- **Demo.** `pick: echo via alpha`. The bench is `pick`, not `call`.

[personal note: four atomic loads, and a name that came back without a handshake]

## What's next

Chapter 38 returns to the harness. It copies chapter 36 and skips this server crate. Several personas share one process. Each profile has its own secret map. In multiplex mode, a secret read with no scope installed fails closed, and the secret does not go into the prompt.
