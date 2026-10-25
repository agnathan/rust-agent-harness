# Chapter 37: Balancing and Federation

This chapter is the MCP server program, the same line as chapters 5, 7, 8, 15, and 23, and it continues at 42 and 51. The harness doesn't copy this directory, because chapter 38 copies chapter 36 and this server crate stays on its own line.

The notes record a gap that the pool has to fill on its own. The tree at the commit this chapter cites has no function that chooses the quietest replica. It has no power-of-two picker either, so least outstanding and the quieter of two indexes are written here. The notes also say that other lines containing those words are not this pool. The piece they adopt is the discovery cap.

The reservation is two steps, and they aren't one critical section. `pick` reads the counters and returns an index. `call` then does `fetch_add`, so two threads can both pick the same index before either increment is visible. This chapter shows the policy and leaves that reservation race open. The counters live on the gateway, and `pick` returns the index without opening a connection.

A gateway that sees three upstreams for one tool still has to choose, and it should choose from the counters it already holds. `Policy` is least outstanding, or the quieter of two indexes the caller already named. `Backend` is one upstream in that pool, with a server name, a reply string, and an outstanding count. `federate` and `pool_for` group one tool name to the servers that provide it. `discover` counts how many workers overlap and lets at most 4 through, which is the discovery concurrency Hermes uses. A setting of `0` for unlimited stays unimplemented. The pool lives in memory, and this crate binds no socket.

The pool stays on the server because it is a property of the upstreams, and every harness client should share that view. Chapter 42 vendors this crate and adds routing, drain, and quotas. Chapter 15's breaker still sits under the call. A pool that ignored an open breaker would send traffic at a backend the limits chapter already decided to leave alone.

A continuously improving team can call tools only if the catalog stays up. Federation is how that catalog can include tools the gateway doesn't contain.

At the pick, the outstanding count is an integer you raise and lower. Power of two in this file is a comparison rather than a framework. An empty pool returns `None`, which is how a missing name looks to the caller. The crate is `steps/ch37-balancing`, which vendors chapter 23 under the library name `balancing` and adds the module `federation`. `cargo run -p ch37-balancing --offline` prints `pick: echo via alpha`. The exercise is `my-harness/tests/ch37.rs`, and it checks that a least-outstanding pick returns an index from those counters.

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
> 

```text
> 25  # Default max concurrent MCP server connections per discovery pass
> 27  # ``mcp.discovery_concurrency`` overrides it, 0 = unlimited (#117373).
> 28  _DISCOVERY_CONNECT_CONCURRENCY = 4
> ```

>
> `_discovery_connect_concurrency` reads `mcp.discovery_concurrency`. A non-integer, a bool, or a negative value warns and returns 4. The comment on the constant says 0 means unlimited. `config_defaults.py` sets that key to 4, in the `mcp` object, and the comment there says the same thing: max servers connected at once per discovery pass, and 0 means unlimited. `_discover_all` builds the bound:
>
> 

```text
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

## What this chapter decides

`pick` with `LeastOutstanding` returns the index of the smallest `outstanding` counter. A tie keeps the earlier backend. `PowerOfTwo { a, b }` reduces both indexes modulo the pool length and returns the quieter one. An equal pair keeps the left index. An empty pool returns `None`, so the caller can tell a missing tool from a quiet backend.

`call` picks, adds one to that counter, formats `reply via server`, and subtracts one, which is why the counter is back where it started when `call` returns. Both functions hand back data that was already on the `Backend`. The pick is the step the bench times.

`federate` builds the map from a tool name to the server names that offer it. `pool_for` turns one tool's servers into a pool whose reply string is the tool name and whose counters start at 0.

`discover` starts `n` threads and holds each one inside a gate of 4, and the value it returns is the max in flight. The cap is 4 because that is the number Hermes uses. The function counts overlap inside the gate.

`federation::demo` federates `echo` on `alpha` and `beta` and returns `pick: echo via alpha`. That string is the line `main` prints, and the run reads no key on the way out.

## 37.1 The crate around the new module

The modules beside the new one are the chapter 7, 8, 15, and 23 gateway, copied in so this step builds alone. `pub mod federation` is the new line in `src/lib.rs`. `main` doesn't call the learned `demo`, which `tests/learned.rs` still covers. `catalog::Upstream` is a name plus tool cards for the in-memory catalog. `federation::Upstream` is a server name plus a `Vec<String>` of tool names for the pool map. So the same word names two types. This pool chooses which backend would run, and it isn't `LimitedGateway`'s permit pool, because that pool limits how many calls run. The allowlist below is part of the copied gateway, and the learned tests are what still exercise it.

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

A reader who opens `lib.rs` still sees those names beside `pub mod federation`, and `main` prints the federation line instead.

> **Rust sidebar: two crates, two names**
>
> Chapter 36's library is `harness_core`, and this package's library is `balancing`, which is why a test says `use balancing::federation::...`. `pub mod federation` makes the module public, so `main.rs` can name `balancing::federation::demo` from the same package. Vendoring means the files live in this crate, with no `path` dependency on `steps/ch23-learned-tools`, so a later edit to chapter 23 leaves this copy alone.

## 37.2 `Policy`, `Backend`, and `pick`

The module starts with the cap and the policy, and both stay small on purpose.

```rust
/// Hermes `mcp.discovery_concurrency` default, from `tools/mcp_tool_discovery.py`.
pub const DISCOVERY_CONCURRENCY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    LeastOutstanding,
    PowerOfTwo { a: usize, b: usize },
}
```

`DISCOVERY_CONCURRENCY` is 4, a `usize`, because it counts threads inside the gate. `Policy` is `Copy`, `LeastOutstanding` carries no fields, and `PowerOfTwo { a, b }` takes the two indexes from the caller. This file draws no random numbers, so this is the comparison step, not a random sample of two backends. A caller who always passes `0` and `1` never looks at the rest of the pool, which is why any randomness stays outside `pick`.

`Backend` is one upstream in the pool, and the struct is only the data `pick` needs.

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

`server` and `reply` are public strings, and `outstanding` is private, so readers call `outstanding()`, which loads the atomic. `new` can start the counter at any `usize`. The bench and the least-outstanding test start four backends at 3, 1, 4, and 2. `pool_for` starts every counter at 0. The struct holds those strings and the counter, with no host, no port, and no client. The reply is the string `call` will format later.

`Pool` owns the vec of backends, and `pick` is the only decision it makes.

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

`len` and `is_empty` read the vec. `pick` takes `&self`, and it can, because the counters are atomics and this function only loads them. An empty pool returns `None` before the `match`, and the `Some(...)` wraps every other result, so a present pool always yields an index.

`LeastOutstanding` enumerates `(index, backend)` and asks `min_by_key` for the smallest `outstanding()`. `min_by_key` keeps the first element when several keys tie. It replaces the current minimum only when the new key is strictly smaller. Outstanding counts of 3, 1, 4, and 2 yield index 1, the backend whose counter is 1. If two backends both held 1, the earlier index would win. The tests use distinct counts, so the tie is the rule in `min_by_key` rather than a row the test asserts. The `unwrap_or(0)` sits behind the empty check. On a non-empty pool `min_by_key` returns `Some`. The `0` is only the value the expression would use if the iterator were empty, which this arm has already refused.

`PowerOfTwo` reduces `a` and `b` with `% self.backends.len()`, so an index past the end wraps into the pool. The test uses indexes that already sit inside the pool, `{ a: 0, b: 1 }` and `{ a: 2, b: 0 }` on a pool of three. The comparison is `<=`, which means the left index wins a tie. `{ a: 0, b: 1 }` with counters 5 and 1 returns 1. `{ a: 2, b: 0 }` with 4 at index 2 and 5 at index 0 returns 2, because 4 is less than 5. Swapping the field order would swap which side a tie prefers, and the test's pairs aren't ties. So that preference stays in the operator.

`pick` returns `Option<usize>`, and that index is the whole result. The function stops before `call`, before a formatted reply. Before any socket, which is the milestone: a least-outstanding pick returns an index and leaves the connection unopened.

The bench's four backends are the same numbers as `least_outstanding_picks_the_quietest_backend`. So the walk is the test. Index 0 is server `a` at outstanding 3, and index 1 is `b` at 1. Index 2 is `c` at 4, and index 3 is `d` at 2. `min_by_key` starts at index 0 with key 3, and index 1 has key 1 and becomes the minimum. Indexes 2 and 3 have keys 4 and 2, which aren't smaller than 1. The result is `Some(1)`. Four loads produce one index, and the servers' reply strings stay unread.

The power-of-two test is a pool of three, with `a` at 5, `b` at 1, and `c` at 4. `PowerOfTwo { a: 0, b: 1 }` compares index 0, which holds 5, with index 1, which holds 1. `5 <= 1` is false, so the result is the right index, 1. `PowerOfTwo { a: 2, b: 0 }` compares index 2, which holds 4, with index 0, which holds 5. `4 <= 5` is true, so the result is the left index, 2. Neither call wraps, because both indexes are already below the length. An index of 5 on this pool would wrap, since `5 % 3` is 2, the same slot as `c`. The test doesn't pass 5. The wrap is what the `%` line does when a caller hands in an index past the end.

> **Rust sidebar: `min_by_key` and `%`**
>
> `enumerate` yields `(0, first)`, then `(1, second)`, and so on. The key function receives `(_, backend)` and returns `backend.outstanding()`, a `usize`, so the smallest key wins and `map` keeps the index. `%` is the remainder, `5 % 3` is 2, and on a pool of length 3 that names the same slot as index 2. A length of 0 never reaches this arm, because the empty check already returned `None`. `Ordering::Relaxed` on the load means the read of the counter doesn't fence other memory, and `pick` doesn't read anything else that would need a fence, because the counter is the whole decision.

## 37.3 `call` formats a string and puts the counter back

The method picks a backend, bumps that counter, and formats the reply from strings already on the struct. It puts the count back before it returns.

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

`?` on `Option` returns `None` when `pick` returns `None`, so an empty pool makes `call` return `None` without touching a counter. On a hit, `fetch_add(1)` raises the chosen counter, the format builds `reply via server`, and `fetch_sub(1)` lowers it again. For the test pool, `call(LeastOutstanding)` returns `Some("echo via b")`, and a second `pick` still returns `Some(1)`. The counter is back at 1. The in-flight bump exists only between the add and the sub. So a caller who samples `outstanding()` after `call` returns sees the idle number.

There is no `await` between the add and the sub, because the reply is a format of two strings that were already stored. The comment on `FakeModel` says `call` doesn't open a socket. The same is true of `pick`, which `call` uses and which the bench calls on its own.

Two threads can both pick index 1 before either `fetch_add` is visible to the other. `pick` and the add aren't one critical section, and each thread would then add. The tests call `pick` and `call` on one thread. The bench calls `pick` only, so its counters stay at 3, 1, 4, and 2 for every sample.

> **Rust sidebar: `?` on `Option`**
>
> `pick` returns `Option<usize>`, and `let index = self.pick(policy)?;` means that a `None` returns `None` from `call` while the rest of the function sees a `usize`. `fetch_add` and `fetch_sub` return the previous value, and `call` ignores those returns because the string is the result. `format!` allocates a new `String`, and that allocation is the work `call` does beyond the atomic updates. `pick` doesn't allocate that string, which is why the bench stays on `pick`.

## 37.4 `federate` and `pool_for` group one tool

The map is built by walking the upstream list once, in the order the caller passed. Each tool name collects the servers that offer it.

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

`Upstream` is public and plain. `federate` walks the slice in order, takes the `HashMap` entry for each tool name, defaults that entry to an empty vec, and pushes the server name. A tool offered by `alpha` and then `beta` gets a vec `["alpha", "beta"]`. A tool offered only by `alpha`, such as `time` in the test, gets `["alpha"]`. The vec order is insertion order, which is the upstream slice order, while `HashMap` iteration order isn't that vec's order. The test reads `map.get("echo")`, one key, and compares the vec.

`entry` clones the tool name because the key is an owned `String`. `or_default` uses `Vec`'s default, an empty vec, when the key is new. `push` clones the server name so the map owns its copy. The input slice is only borrowed, so the caller keeps the upstreams it passed in.

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

`pool_for` federates the slice and then `remove`s one key. `remove` returns `Option<Vec<String>>`, a missing name becomes an empty vec through `unwrap_or_default`. `Pool::new` of that empty vec is an empty pool. `is_empty` is then true, and `pick` returns `None`. The test checks `pool_for("missing", &upstreams).is_empty()`, which is the missing-name path.

Each server becomes a `Backend` whose reply is the tool name and whose counter is 0. The server string is cloned into `new` because `Backend::new` takes `impl Into<String>`. The `map` closure would otherwise move the only copy. For `echo` on `alpha` and then `beta`, the pool length is 2 and both counters are 0. So least outstanding returns index 0 and `call` formats `echo via alpha`. Ties at 0 keep the earlier server, even though neither backend has done any work yet. That earlier server is the demo's pick.

> **Rust sidebar: `entry` and `remove`**
>
> `entry(key)` is one lookup that can insert, `or_default()` inserts the default when the key is absent and returns a mutable reference to the vec, and `push` writes the server onto that vec. `remove` takes the vec out of the map, and the rest of the map drops at the end of the statement, including every other tool's server list. `pool_for` only needed the one key. `HashMap` doesn't promise that `keys()` comes back in upstream order, so the order that matters sits inside the `Vec<String>` for that tool, and `pool_for` preserves it by walking that vec.

## 37.5 `discover` counts overlap, capped at 4

`Gate` is private, and the tests reach it only through `discover`. So the cap is a behavior of that function rather than a knob on the call.

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

`enter` locks `inflight` and waits while the count is already at `DISCOVERY_CONCURRENCY`. The wait is `cv.wait`, which unlocks, parks the thread until a notify, and relocks. The `while` checks again because a notify means some waiter should look, not that this waiter owns a free slot. Then `enter` adds one and keeps the inflight lock while it locks `max_seen`. It stores the max of the old high-water mark and the current count. `leave` subtracts one with `saturating_sub`, so a leave that found zero would stay at zero. `notify_one` wakes a single waiter, and the others stay in `wait` until a later `leave`.

Six workers and a cap of 4, with a hold long enough that they overlap, play out in a fixed order. Workers 1 through 4 pass the `while`, each of them increments. `max_seen` climbs to 4, while workers 5 and 6 block inside `wait`. When worker 1 leaves, inflight drops to 3 and one waiter enters. Inflight returns to 4, and `max_seen` stays 4. The function returns that high-water mark rather than the count at the end. After `thread::scope` joins, inflight is 0, and the test asserts the high-water mark.

Hermes does the same shape on a different runtime. `_discover_all` sets `semaphore = asyncio.Semaphore(cap) if cap > 0 else None`, each connect takes the semaphore when `cap` is positive. A cap of 0 stores `None` and awaits the connect with no semaphore. `discover` has no `cap` argument and no `None` semaphore, so the comparison in `enter` is always `>= 4`.

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

`thread::scope` joins every spawned thread before it returns, and `n` is how many connects the test asked for. Each worker enters, optionally sleeps `hold`, and leaves. The sleep is what makes those workers overlap, and a zero `hold` skips the sleep. Even though a zero hold skips that sleep, the function still refuses a fifth `enter` while four workers are inside. But the threads may not overlap, so `max_seen` can come back below 4. The test passes `Duration::from_millis(40)` and an `n` of 6 so the workers do overlap. The first four enter and sleep, and the fifth and sixth block in `wait`. `max_seen` becomes 4 and stays 4, and the return is 4. The test asserts that the return equals `DISCOVERY_CONCURRENCY`, and it asserts that the constant equals 4.

The connect in this function is the pair of `enter` and `leave`, and nothing in the function opens a socket. Hermes's `_discover_all` really does connect, under an `asyncio.Semaphore` of the configured cap, and a cap of 0 skips the semaphore. This gate has no argument for 0, four is the constant. A caller who wanted unlimited concurrency still sees a max of 4 whenever the workers overlap.

`pick` doesn't call `discover`, the bench doesn't call it, and the demo doesn't call it either. The cap and the pick are two public functions that happen to share one module.

> **Rust sidebar: `thread::scope`**
>
> `scope` lets the spawned closures borrow `gate` from the stack, those closures can't outlive the scope, and when the closure passed to `thread::scope` ends, the workers have finished. `Condvar::wait` takes the mutex guard and returns a new guard, and the `while *inflight >= 4` loop rechecks after every wake, because a wake by itself isn't proof that a slot is free. `Duration::from_millis(40)` is the test's hold, and `is_zero` is how a zero hold skips `thread::sleep`. The production pick path doesn't construct a `Duration`.

## 37.6 `FakeModel` and the demo line

The demo builds a two-server pool for `echo` and prints the pick. The scripted model is only the fallback for an empty pool.

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

`FakeModel` in this module isn't the harness `FakeModel`. It has one public `reply`, and `complete` clones that string. `demo` builds the model as a fallback. `pool_for("echo", ...)` yields two backends, both at outstanding 0, so `call` returns `Some("echo via alpha")`. `unwrap_or_else` doesn't call `complete` when the option is `Some`, and the format is `pick: echo via alpha`. An empty pool would fall through to `model.complete()` and print `pick: echo`, and this input doesn't take that path. The function reads no key, and it opens no socket.

`main` prints that string and exits.

```rust
use balancing::federation::demo;

fn main() {
    println!("{}", demo());
}
```

## Tests

`tests/balancing.rs` is the new file, and `tests/learned.rs` is the vendored chapter 23 suite. The assertions below are the balancing ones, kept separate from that older file.

`least_outstanding_picks_the_quietest_backend` builds counters 3, 1, 4, and 2. `pick(LeastOutstanding)` is `Some(1)`, `call` is `Some("echo via b")`, and a second `pick` is still `Some(1)`.

`power_of_two_picks_the_quieter_of_the_pair` uses counters 5, 1, and 4. `PowerOfTwo { a: 0, b: 1 }` is `Some(1)`, and `PowerOfTwo { a: 2, b: 0 }` is `Some(2)`. The same pool's least-outstanding pick is `Some`, and an empty `Pool` picks `None`.

`federation_groups_a_tool_across_upstreams` puts `echo` and `time` on `alpha`, and `echo` on `beta`. `federate` returns `["alpha", "beta"]` for `echo`, `pool_for("echo")` has length 2 and `call` returns `echo via alpha`, and `pool_for("missing")` is empty.

`discovery_stays_at_four` calls `discover(6, Duration::from_millis(40))` and expects 4, and it expects the constant to be 4.

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

The package has one binary, so the run doesn't need `--bin`. The printed line is the federation demo, `pick: echo via alpha`, and the run prints no key.

```bash
cargo bench -p ch37-balancing --bench balancing --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `pick_least` | 3.74 ns | a local pick among a pool | the quietest of four backends |

`pick_least` times that local read, the quietest of four backends, and the mean in the table is the measurement. `discover(6, Duration::from_millis(40))` sleeps 40 ms so six workers overlap inside a gate of 4, and that sleep isn't in the pick bench.

The milestone is the pick itself. Least outstanding returns an index, and the index is a `usize` into a vec of strings and atomics.

`pick_least` is `benches/balancing.rs`. It builds four backends before the timed loop, with outstanding counts 3, 1, 4, and 2. Every sample reads the same quiet backend.

The means are `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `37`. The bench ids are `37/pick_least` and `35/pick_least`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`call` drops the outstanding count before it returns, so a metric that samples after `call` sees the idle number. The bump is visible only between `fetch_add` and `fetch_sub`, which for a scripted reply is the time it takes to format one string.

`pick` and the add are separate steps, so two overlapping calls can choose the same index. The tests run on one thread, and the bench never adds, so neither of those runs shows the race the opening left open.

Power of two is only as random as the indexes the caller passes in, and this crate doesn't draw those indexes itself.

A discovery run with a zero hold can show a max below 4, because the threads may finish before they overlap. The test holds for 40 milliseconds so the six workers do overlap, the cap stays at 4 while they do. The gate doesn't let the count climb past 4.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Pick.** Least outstanding, or the quieter of two indexes. The choice loads atomics. It does not call an upstream. It does not open a socket. That is the milestone.

**Discovery.** At most 4 connects in flight. That is `_DISCOVERY_CONNECT_CONCURRENCY` and the default `mcp.discovery_concurrency` of 4. Hermes's `0` means unlimited, through a missing semaphore. This chapter does not implement that override.

**Federation.** A tool's pool is the list of servers that offer it, in the order the upstreams were given. A missing name is an empty pool, and an empty pool picks `None`.

**Vendored limits.** The copied chapter 15 gateway still has its own permits, breaker, and caps of 50. Those guard a call into `LimitedGateway`. They are not the outstanding counters on `Backend`.

**Left for later.** Live sockets, the unlimited discovery setting, and session routing are chapter 42. The scripted reply is not redacted.

## What you implement

`my-harness/tests/ch37.rs` is the milestone, and the stub stays `todo!` until the reader fills it in.

```rust
use my_harness::chapter_37_pick_opens_connection;

#[test]
fn least_outstanding_pick_does_not_open_a_connection() {
    assert!(!chapter_37_pick_opens_connection());
}
```

The test wants `chapter_37_pick_opens_connection()` to be false. A least-outstanding pick returns an index, and the connection stays closed for the whole call.

The stub is the panic the reader replaces.

```rust
pub fn chapter_37_pick_opens_connection() -> bool {
    todo!("chapter 37")
}
```

`todo!("chapter 37")` panics when the test calls it. Filling the function in is the reader's work, and copying `steps/ch37-balancing` into `my-harness/` isn't the milestone the assertion is checking.

## Chapter summary

- **Crate.** This is the server line, the library is `balancing`, chapter 23 is vendored into the crate. The new module is `federation`, so `main` prints `federation::demo()`.
- **Pick.** Least outstanding, or the quieter of two caller-supplied indexes, keeps the earlier backend on a tie, and an empty pool returns `None`.
- **Call.** The method adds one, formats `reply via server`, and subtracts one before it returns the string.
- **Federation.** The map runs from a tool name to server names. `echo` on `alpha` and `beta` picks `alpha` when both counters are 0.
- **Discovery.** Four workers in flight is the cap Hermes uses, and an unlimited `0` stays unimplemented in this gate.
- **Demo.** The printed line is `pick: echo via alpha`, and the bench times `pick` rather than the format inside `call`.

## What's next

Chapter 38 returns to the harness, where profiles put several personas in one process. This server continues at chapter 42, which is where routing meets the pool.
