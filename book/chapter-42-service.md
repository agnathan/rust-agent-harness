# Chapter 42: MCP Gateway Service

A replica can die while a call is still in flight. A drain that dropped those calls would cut a `skill_view` or a search in half, and reflection would then learn from the error. That is why this service matters to a continuously improving harness. `route` remembers which replica holds a session id, and `kill` marks that replica dead. The next lookup fails over to the other id, so failover by session id keeps the offered set. `accept` stops when a drain is on, and `finish` lets the calls already accepted complete.

This is the MCP server program, the same line as chapters 5, 7, 8, 15, 23, and 37, and chapter 51 copies this crate. It isn't the chat gateway, and it isn't the harness, because chapter 43 copies chapter 41 rather than this directory. Mixing a Telegram adapter and a tool catalog in one process would be one failure domain, so the service stays a separate program.

The crate is `steps/ch42-service`. It vendors chapter 37, the library is `gwservice`, and the new module is `service`. The process binds no socket. OAuth issuance stays deferred, and a bearer check stands in for the resource-server decision.

Beside the route table, the service keeps a reloaded `Config`, a per-tenant quota, an audit vec, and span names. `Sampler` caps calls at ten inside sixty seconds. `elicit` is the form-mode approval, and `OsvCache` screens an `npx` or `uvx` launch in memory. Health and readiness are flags on that same service.

The route table is a map from session id to replica id. An `Arc` is what reload swaps, and `accept` is where the drain flag is checked. The error type is `Reject`. The failover test kills the record and looks the session up again.

Running `cargo run -p ch42-service` runs the demo in process and returns `route: a`. The exercise is `my-harness/tests/ch42.rs`, where killing the route record fails the next lookup over to the other replica id.

| Section | What it walks |
|---|---|
| 42.1 | the vendored modules, and `service` beside them |
| 42.2 | `SAMPLING_MAX_RPM`, `Config`, `Reject`, `Service::new` |
| 42.3 | `route`, `place`, `kill`, `alive` |
| 42.4 | `config` and `reload` |
| 42.5 | `accept`, `finish`, `start_drain`, `ready`, `drained` |
| 42.6 | `bearer_ok` |
| 42.7 | `Sampler` and `allow` |
| 42.8 | `Elicit` and `elicit` |
| 42.9 | `OsvVerdict` and `OsvCache::check` |
| 42.10 | `FakeModel` and `demo` |

> **How Hermes does it**
>
> I read `tools/mcp_tool_sampling.py` (`SamplingHandler.max_rpm`, `SamplingHandler.timeout`, `_check_rate_limit`, `ElicitationHandler.timeout`, the URL-mode branch in `ElicitationHandler.__call__`) and `tools/osv_check.py` (`_TIMEOUT`, `_CACHE_TTL_S`, `_ECOSYSTEM_BY_COMMAND`, `check_package_for_malware`, `_query_osv`, `_cache_put`) on Hermes `main` at `9e01b44`. I also read `tools/mcp_oauth.py` (the module docstring and `build_oauth_auth`) and `gateway/drain_control.py` (`DRAIN_REQUEST_MAX_AGE_SECONDS`, `open_restart_wait`).
>
> `SamplingHandler.__init__` sets `max_rpm` from `config.get("max_rpm", 10)` and `timeout` from `config.get("timeout", 30)`. `_check_rate_limit` keeps timestamps newer than `now - 60` seconds. Ten pass. The eleventh inside that window returns false, and the caller logs the limit and returns an error. The sampling LLM call uses that 30 second timeout. `ElicitationHandler` defaults its timeout to 300 seconds and passes `timeout_seconds=int(self.timeout)` into `request_elicitation_consent`. If `params.mode` is `"url"`, `__call__` returns a decline. Form mode goes through approval. A timeout, an exception, or an unexpected answer declines. It does not accept on failure.
>
> `osv_check.py` queries `https://api.osv.dev/v1/query` with `_TIMEOUT = 10` seconds. `_query_osv` keeps vulnerability ids that start with `MAL-` and drops the rest, so a regular CVE is not a block. `_CACHE_TTL_S` is `float(os.getenv("OSV_CHECK_CACHE_TTL", "3600"))`. Clean and blocked verdicts are stored until that expiry. A network exception returns `None` and is not cached. The comment on `_CACHE_TTL_S` says a cached failure could mask a later advisory. `_ECOSYSTEM_BY_COMMAND` maps `npx` and `npx.cmd` to npm, and `uvx`, `uvx.cmd`, `uvx.exe`, `pipx`, and `pipx.exe` to PyPI. `_infer_ecosystem` uses the last path component, lowercased. The cache also has a disk file and a cap of `_CACHE_MAX_ENTRIES = 256`.
>
> `mcp_oauth.py` is an OAuth 2.1 client: browser authorization-code with PKCE, `HermesTokenStorage`, and `build_oauth_auth`. Hermes is not the authorization server for this gateway. `drain_control.py` is the messaging gateway's marker file, `{HERMES_HOME}/.drain_request.json`. `DRAIN_REQUEST_MAX_AGE_SECONDS` is 3600.0. A malformed file still reads as drain-active. That file is the chat gateway's contract.
>
> **Adopted.** The sampling cap of 10 and the 60 second window. Form-mode approval at 300 seconds. URL-mode declined. OSV timeout 10, cache 3600, `MAL-*` only, fail open and uncached on a network error, and a skip when the command is not a launch this check understands. The drain idea is stop accepting and let accepted calls finish.
>
> **Adapted.** Replicas, the session map, quotas, the audit vec, and span names are this book's service. Hot reload is the `Arc` swap already used by the catalog. The bearer check is a non-empty `Bearer` token, which stands in for the resource-server decision. `OsvCache::check` takes `network_ok` and a slice of ids. It does not open a socket and it does not parse argv. The command match is the exact strings `npx` and `uvx`. `FakeModel` in `service.rs` is not consulted by `route`. The 10 second and 300 second values are named. Neither is slept. `SAMPLING` has no 30 second constant in this file. Thirty seconds is Hermes's sampling LLM timeout, and this sampler does not call a model.
>
> **Deferred.** A live listener, OAuth 2.1 token issuance, a real exporter, Redis, and oha. `pipx`, Windows launcher suffixes, path basenames, the OSV disk cache, and the 256-entry cap stay in Hermes. The drain marker file and its 3600 second max age stay in `gateway/drain_control.py`. The route bench is a map lookup.

> **Guardrails.** Sampling and elicitation are M7. The malware check is H4. Both live in `service.rs` as `SAMPLING_MAX_RPM`, `OSV_TIMEOUT_SECONDS`, and `OsvCache`. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

## What this chapter decides

A placed session stays on its replica while that replica is alive. Otherwise `place` takes the first live name in the `Vec` from `new`. `kill` marks that name dead and places each of its sessions again. `route` is only the map lookup, so a miss stays a miss until something calls `place`.

A reload replaces the `Arc<Config>`, and a caller that already holds the old `Arc` still sees the old generation. `config` clones the pointer rather than the struct.

Calling `accept` requires the word `Bearer`, then a non-empty token with no space in it. A drain returns `Draining`, a tenant at its concurrent quota returns `Quota`, and no live replica returns `NoReplica`. Success appends `gateway.call {id}` to the span vec and an accept line to the audit vec, and it counts one in-flight call. `finish` drops that call and frees the quota slot for the tenant name it is given. `drained` is true when a drain is on and the in-flight list is empty. `ready` is false while draining, or while every replica is dead.

The sampler keeps stamps younger than 60 seconds. Ten calls pass, and the eleventh inside the window is refused.

Calling `elicit(false)` returns an approval whose timeout is 300. Calling `elicit(true)` returns `Declined`.

The malware check returns `Skipped` for any command other than `npx` or `uvx`. A cached verdict whose expiry is still greater than `now` comes back as stored. A network failure returns `FailOpen` and writes nothing into the map. Otherwise a `MAL-*` id blocks, and a CVE-only list is clean. Both of those verdicts are cached until `now + 3600`.

The demo places `s1` on `a` and returns `route: a`. The `FakeModel` reply is discarded, and no key is read.

## 42.1 The crate vendors chapter 37

This crate copies chapter 37's modules in so the step builds alone. `Cargo.toml` has no path dependency on `ch37-balancing`. The crate root is the learned gateway vendored beside `service`. A prediction is installed before the request, session tools read that set, and code mode runs a line script against an allowlist.

The modules it declares are the copied tree plus this chapter.

```rust
pub mod catalog;
pub mod federation;
pub mod limits;
pub mod service;
pub mod session;

pub use limits::{Catalog, LimitedGateway, SessionError, SessionView};
```

The catalog module is the chapter 7 gateway, with native tools, plugin tools, an in-memory upstream list, and BM25 search. The session module is the chapter 8 view. `limits` is the chapter 15 wrapper, with permits, an idempotent retry, a breaker, and the caps of 50. The root of `lib.rs` is that learned gateway: `predict`, `LearnedGateway`, composites, macros, and `run_code_mode`, with `SANDBOX_ALLOWED` and `CODE_MODE_MAX_CALLS` of 50. `federation` is chapter 37, with `Policy`, `Pool::pick`, `pool_for`, and `discover` at `DISCOVERY_CONCURRENCY` of 4. Those functions were walked in those chapters. This chapter leaves them as copied code, and `service` doesn't call them.

The module path is `gwservice::service`, and `main.rs` is three lines.

```rust
use gwservice::service::demo;

fn main() {
    println!("{}", demo());
}
```

The learned `demo` at the bottom of `lib.rs` returns a `(String, String)` pair from code mode. The federation `demo` returns `pick: echo via alpha`. Neither of those is the binary. `main` calls `service::demo`, not the learned demo and not the federation demo.

> **Rust sidebar: `pub mod` and a copied crate**
>
> `pub mod service` makes the module visible as `gwservice::service`. A private `mod service` would hide `Service` from the integration tests, which write `use gwservice::service::Service`. The `pub use` puts `Catalog`, `LimitedGateway`, `SessionError`, and `SessionView` at the crate root so the copied learned tests can keep their imports. Vendoring compiles those types into `ch42-service`. A path dependency would have compiled them in `ch37-balancing` and then linked them, and the step would no longer be a snapshot of its own.
>
> The package depends on `serde_json`, `thiserror`, and `tokio`. `service.rs` uses none of those three, because the copied catalog and the session gateway do. Criterion is a dev-dependency. The bench target is `service`, with `harness = false`.

## 42.2 The constants, `Config`, `Reject`, and `Service::new`

The numbers this chapter shares with Hermes sit at the top of `service.rs`.

```rust
pub const SAMPLING_MAX_RPM: usize = 10;
pub const SAMPLING_WINDOW_SECONDS: u64 = 60;
pub const ELICIT_TIMEOUT_SECONDS: u64 = 300;
pub const OSV_TIMEOUT_SECONDS: u64 = 10;
pub const OSV_CACHE_TTL_SECONDS: u64 = 3600;
```

The constant `SAMPLING_MAX_RPM` is Hermes's default `max_rpm`. `SAMPLING_WINDOW_SECONDS` is the `60` inside `_check_rate_limit`. `ELICIT_TIMEOUT_SECONDS` is the elicitation default of 300. `OSV_TIMEOUT_SECONDS` is `_TIMEOUT`, and `OSV_CACHE_TTL_SECONDS` is the default of `OSV_CHECK_CACHE_TTL`. There is no constant here for the sampling LLM timeout of 30, because that number stays on `SamplingHandler.timeout`.

A config value is one generation number, and `Reject` is why `accept` refused.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reject {
    Draining,
    Quota,
    NoReplica,
    BadToken,
}
```

The service owns the replica list, the session map, the config pointer, the per-tenant counters, and the quota. It also owns the audit lines, the span names, the in-flight calls, the next call id, and the drain flag. The fields are private, so tests go through the methods.

```rust
pub struct Service {
    replicas: Vec<(String, bool)>,
    routes: HashMap<String, String>,
    config: Arc<Config>,
    inflight_quota: HashMap<String, u32>,
    quota: u32,
    audit: Vec<String>,
    spans: Vec<String>,
    inflight: Vec<(u64, String)>,
    next_id: u64,
    draining: bool,
}
```

Construction marks every name alive, starts generation 1, and starts the next call id at 1. The quota is the concurrent cap for every tenant, and it is a cap on calls in flight rather than a lifetime total.

```rust
pub fn new(names: &[&str], quota: u32) -> Self {
    Self {
        replicas: names
            .iter()
            .map(|name| ((*name).to_string(), true))
            .collect(),
        routes: HashMap::new(),
        config: Arc::new(Config { generation: 1 }),
        inflight_quota: HashMap::new(),
        quota,
        audit: Vec::new(),
        spans: Vec::new(),
        inflight: Vec::new(),
        next_id: 1,
        draining: false,
    }
}
```

The replica `Vec` keeps construction order, and `place` later walks it from the front. A `HashMap` wouldn't keep that order, which is why the live replica is stored as a list of names rather than a map.

> **Rust sidebar: an enum as the error, and a tuple in a `Vec`**
>
> `Reject` is four units with no payload, and `accept` returns `Result<u64, Reject>`. The `u64` is the call id. Matching on `Reject::Quota` is exhaustive only when every variant is named, and `PartialEq` is why a test can write `Err(Reject::Quota)` and compare.
>
> `Vec<(String, bool)>` is a list of pairs, and the `bool` is liveness. A struct with named fields would read more clearly at the call site, but the pair is what `new` builds and what `alive` matches. The session map is a separate `HashMap<String, String>`, session id to replica name. The replica list is the source of who is alive, and the map is only the assignment.

## 42.3 `route`, `place`, and `kill`

A route lookup borrows the stored name, and it leaves the map unchanged. A session that was never placed is `None`.

```rust
pub fn route(&self, session: &str) -> Option<&str> {
    self.routes.get(session).map(String::as_str)
}
```

Assignment is the job of `place`. If the session already has a name and `alive` says that replica is still up, the same name comes back and the map stays as it was. Otherwise the first live replica is inserted and returned. An empty live set returns `None`. The previous map entry is left alone only while the current replica is alive, and a dead replica is overwritten by `insert`.

```rust
pub fn place(&mut self, session: &str) -> Option<String> {
    if let Some(current) = self.routes.get(session) {
        if self.alive(current) {
            return Some(current.clone());
        }
    }
    let next = self.replicas.iter().find(|(_, alive)| *alive)?.0.clone();
    self.routes.insert(session.to_string(), next.clone());
    Some(next)
}
```

Killing a replica sets its alive flag to false, collects every session currently mapped to it, removes those keys, and calls `place` on each. The collect step is required because `place` mutates `routes`, so the loop can't borrow the map and insert into it in one walk.

```rust
pub fn kill(&mut self, name: &str) {
    for (replica, alive) in &mut self.replicas {
        if replica == name {
            *alive = false;
        }
    }
    let sessions: Vec<String> = self
        .routes
        .iter()
        .filter(|(_, replica)| replica.as_str() == name)
        .map(|(session, _)| session.clone())
        .collect();
    for session in sessions {
        self.routes.remove(&session);
        self.place(&session);
    }
}

fn alive(&self, name: &str) -> bool {
    self.replicas
        .iter()
        .any(|(replica, alive)| replica == name && *alive)
}
```

Liveness stays private. A name that was never passed to `new` is treated as down. `kill` of a name that isn't in the list changes no flag and moves no session, because the filter matches nothing.

Building with `Service::new(&["a", "b"], 4)` and then `place("s1")` returns `Some("a")`, since `a` is the first live name. A second `place("s1")` hits the early return and stays on `a`. `kill("a")` clears the flag, removes `s1`, and `place` finds `b`, so `route("s1")` is then `Some("b")`. That is the failover the milestone asks for. If `b` is also dead, `place` returns `None` after the remove, and `route` is `None` as well.

Health is true when any replica is live, and the drain flag is outside that question.

```rust
pub fn healthy(&self) -> bool {
    self.replicas.iter().any(|(_, alive)| *alive)
}
```

> **Rust sidebar: `Option`, `?`, and collecting before mutate**
>
> `find` returns `Option<&(String, bool)>`. The `?` on that option turns "no live replica" into `None` from `place`, and `.0.clone()` runs only when a pair was found. `route` uses `map(String::as_str)` so the caller borrows the `String` inside the map, and that borrow can't outlive the `&self` borrow. `place` returns an owned `String` because the caller, and `kill`, need the name after further mutation.
>
> The `sessions` vector owns the keys. After `collect`, the immutable borrow of `routes` has ended, so `remove` and `place` can take `&mut self`. Skipping the vector would be a borrow-checker error, not a style choice.
>
> `HashMap` iteration order isn't the replica order. With one session the order doesn't matter. With several sessions on a dead replica, each is placed onto the same first live name, and they don't fan out across the remaining replicas.

## 42.4 `config` and `reload`

```rust
pub fn config(&self) -> Arc<Config> {
    Arc::clone(&self.config)
}

pub fn reload(&mut self, generation: u64) -> Arc<Config> {
    self.config = Arc::new(Config { generation });
    Arc::clone(&self.config)
}
```

Construction stores generation 1. `config` bumps the strong count and returns that pointer, while `reload` allocates a new `Config` and stores it. The previous `Arc` stays valid for whoever cloned it. The test holds the pointer, reloads generation 2, and checks that `held.generation` is still 1 while `reloaded.generation` is 2. `Arc::strong_count(&held)` is 1, because the service dropped its clone when it stored the new pointer. The reloaded pointer is held by the service and by `reloaded`, so its count is 2, and the test doesn't assert that second count.

Readers of the old `Arc` keep seeing the generation they cloned. There is no lock around the swap, because `reload` takes `&mut self` and the swap in this type is exclusive. The `Arc` matters for the caller who already cloned, not for two threads calling `reload` together. `Service` isn't shared behind a mutex in this file.

> **Rust sidebar: `Arc` is the snapshot**
>
> `Arc::clone` clones the pointer, not the `Config`, and `Arc::new` allocates a fresh one. After `reload`, the two pointers are different allocations and their `generation` fields differ. Dropping the service drops the new pointer, while the caller's old pointer still reads generation 1 until the caller drops it. That is the same shape as the catalog reload in chapter 7, where the in-flight reader keeps the snapshot it loaded.

## 42.5 `accept`, `finish`, and the drain

Readiness is health together with the absence of a drain. A dead pool fails readiness, and a live pool that has started a drain fails readiness too.

```rust
pub fn ready(&self) -> bool {
    self.healthy() && !self.draining
}
```

Each accept checks the header, then the drain, then the quota, then a replica. The first failure returns, and the later checks don't run.

```rust
pub fn accept(&mut self, tenant: &str, session: &str, header: &str) -> Result<u64, Reject> {
    if !bearer_ok(header) {
        self.audit.push(format!("deny {tenant}"));
        return Err(Reject::BadToken);
    }
    if self.draining {
        self.audit.push(format!("drain {tenant}"));
        return Err(Reject::Draining);
    }
    let used = *self.inflight_quota.entry(tenant.to_string()).or_insert(0);
    if used >= self.quota {
        self.audit.push(format!("quota {tenant}"));
        return Err(Reject::Quota);
    }
    if self.place(session).is_none() {
        return Err(Reject::NoReplica);
    }
    self.inflight_quota
        .insert(tenant.to_string(), used.saturating_add(1));
    let id = self.next_id;
    self.next_id += 1;
    self.inflight.push((id, session.to_string()));
    self.spans.push(format!("gateway.call {id}"));
    self.audit.push(format!("accept {tenant} {session}"));
    Ok(id)
}
```

A bad token writes `deny {tenant}` and stops before `place`. A drain writes `drain {tenant}` and also stops before `place`. A quota miss writes `quota {tenant}` and stops in the same way. `NoReplica` writes nothing, so the audit vec stays as it was. That branch is the one the dead-pool test hits after `kill` of the only replica: `healthy` is false, and `accept` is `Err(Reject::NoReplica)`.

The quota read uses `entry`. A tenant seen for the first time gets a zero counter, and `used >= self.quota` refuses before the increment. A quota of 1 allows one accept, and the second concurrent accept for that tenant is `Quota`, even when the session id differs. A different tenant has its own counter. The test doesn't accept a second tenant before the drain. After the first success the counter is `used + 1`, stored with `saturating_add`, and a `u32` at the top of its range doesn't wrap on that add.

The success path places the session, stores the call as `(id, session)`, pushes `gateway.call {id}`, and pushes `accept {tenant} {session}`. The first id is 1 because `next_id` starts at 1 and is incremented after the copy. The span test expects exactly `["gateway.call 1"]`.

Finishing a call finds the id, removes that pair, and decrements the named tenant. A missing id returns false and leaves the counter alone.

```rust
pub fn finish(&mut self, id: u64, tenant: &str) -> bool {
    let Some(index) = self.inflight.iter().position(|(call, _)| *call == id) else {
        return false;
    };
    self.inflight.remove(index);
    if let Some(used) = self.inflight_quota.get_mut(tenant) {
        *used = used.saturating_sub(1);
    }
    true
}
```

The tenant string is the caller's responsibility. `finish(id, "other")` removes the call and looks up `other`, and if `other` has no counter the original tenant's slot stays taken. A later `accept` for the original tenant still sees the old count.

```rust
pub fn start_drain(&mut self) {
    self.draining = true;
}

pub fn inflight(&self) -> usize {
    self.inflight.len()
}

pub fn drained(&self) -> bool {
    self.draining && self.inflight.is_empty()
}

pub fn audit(&self) -> &[String] {
    &self.audit
}

pub fn spans(&self) -> &[String] {
    &self.spans
}
```

Starting a drain flips the flag, and it leaves the in-flight calls and the session map in place. `drained` stays false while a call is still in the vec, and it also stays false before `start_drain` even when the vec is empty. The timeline in the test is accept, then `start_drain`, then `finish`. One call is accepted, the drain starts, `ready` goes false, and a new accept returns `Draining`. `drained` is still false at that moment. `finish` of the accepted id is what makes `drained` true, with `inflight` at 0. The call already accepted is allowed to complete, and the new call is refused. That drain is a bool, not `.drain_request.json`, and it never reads `DRAIN_REQUEST_MAX_AGE_SECONDS`.

> **Rust sidebar: `entry`, `let`/`else`, and `saturating_add`**
>
> `entry` returns a handle to the occupied or vacant slot. `or_insert(0)` writes zero only when the key is new, then yields `&mut u32`. The `*` copies the number out so the later `insert` can borrow the map again, because holding the `Entry` across the `insert` would keep the map borrowed.
>
> A let-else binds `index` when `position` finds the id, and it returns false when the id is missing. `position` walks the vec from the front and returns the first match.
>
> `saturating_add(1)` and `saturating_sub(1)` stop at the ends of `u32`. A zero counter doesn't wrap to `u32::MAX` if `finish` runs once too often, and a counter at `u32::MAX` doesn't wrap to zero on another accept. The quota test uses 1, so the saturate path isn't what that test shows, though the methods still use it.

## 42.6 `bearer_ok`

```rust
pub fn bearer_ok(header: &str) -> bool {
    let Some(token) = header.strip_prefix("Bearer ") else {
        return false;
    };
    !token.is_empty() && !token.contains(' ')
}
```

The prefix is the six letters, the space, and nothing else. `Bearer token` passes. A bare `token` fails, `Bearer ` fails because the remainder is empty, and `Bearer token extra` fails because the remainder contains a space. The function stops at that space check. It doesn't decode base64, and it doesn't verify a signature. `accept` calls it first. Issuing the token isn't this crate. `tools/mcp_oauth.py` is the Hermes client that would talk to an authorization server, and this service doesn't call that module.

> **Rust sidebar: `strip_prefix`**
>
> `strip_prefix` returns `Option<&str>` borrowing the tail of `header`. The pattern is a literal prefix, not a regex. `Bearer` without the trailing space misses, and case matters, so `bearer token` fails the prefix and `accept` returns `BadToken`.

## 42.7 `Sampler`

```rust
pub struct Sampler {
    stamps: VecDeque<u64>,
}

impl Sampler {
    pub fn new() -> Self {
        Self {
            stamps: VecDeque::new(),
        }
    }

    /// Sliding window of 60 seconds. The 11th request inside the window is refused.
    pub fn allow(&mut self, now: u64) -> bool {
        self.stamps
            .retain(|stamp| now.saturating_sub(*stamp) < SAMPLING_WINDOW_SECONDS);
        if self.stamps.len() >= SAMPLING_MAX_RPM {
            return false;
        }
        self.stamps.push_back(now);
        true
    }
}
```

The default constructor calls `new`. From there, `allow` drops stamps whose age isn't strictly less than 60. A stamp equal to `now` has age 0 and stays, while a stamp from exactly 60 seconds ago has age 60. `60 < 60` is false, so that stamp is dropped. Hermes uses `t > now - 60`, which is the same cut, and a timestamp exactly 60 seconds in the past is dropped there too.

The test calls `allow(0)` ten times. Each call sees a length under 10, appends 0, and returns true. The eleventh call at timestamp 0 sees length 10 and returns false without appending, so the refused call adds no slot. `allow(60)` drops every 0, sees an empty deque, appends 60, and returns true. A clock that goes backwards is handled by `saturating_sub`: if `now` is less than a stored stamp, the age is 0 and the stamp stays.

The sampler sits outside `Service`, and `accept` doesn't call `allow`. A caller who wants the limit owns a `Sampler` and asks it. One sampler is one window. Hermes keeps `_rate_timestamps` on the `SamplingHandler` for one MCP server, and two `Sampler` values don't share stamps.

The function returns a bool and sleeps for nothing. It also leaves `SamplingHandler.timeout` unread. The 30 second sampling budget is Hermes's `asyncio.wait_for` around the LLM call, and this `allow` has no such wait.

> **Rust sidebar: `VecDeque` and `retain`**
>
> `VecDeque` is a ring buffer. `push_back` adds at the end, and `retain` keeps elements for which the closure returns true, dropping the rest and preserving order. A `Vec` would do the same job at this length, but the deque matches a window that drops from the old side. The length check uses `>= SAMPLING_MAX_RPM`, so the cap is ten stored stamps, not eleven.

## 42.8 `elicit`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elicit {
    Approval { timeout_seconds: u64 },
    Declined,
}

pub fn elicit(url_mode: bool) -> Elicit {
    if url_mode {
        Elicit::Declined
    } else {
        Elicit::Approval {
            timeout_seconds: ELICIT_TIMEOUT_SECONDS,
        }
    }
}
```

Form mode is what `elicit(false)` returns, an approval whose timeout is 300. `elicit(true)` is URL mode, and the result is `Declined`. Hermes reads `getattr(params, "mode", "form")` and treats `"url"` as the decline, while anything else, including the default `"form"`, goes to consent. This function takes a bool the caller has already decided. It doesn't parse a mode string, it doesn't call `request_elicitation_consent`, and it doesn't sleep for 300 seconds. The outer grace of `_OUTER_TIMEOUT_GRACE_SECONDS = 5` is Hermes's asyncio safety net, and that grace isn't added here.

Deriving `Copy` works because both variants are a bool's worth of data plus a `u64`. A test compares that approval with `==` rather than matching field by field.

> **Rust sidebar: a struct-like variant**
>
> `Elicit::Approval { timeout_seconds }` carries the number on the approval arm, and `Elicit::Declined` has no field. Putting the timeout on the approval means a declined result can't be read as 300 seconds by mistake. The number is still the constant, and the function doesn't take a caller-supplied timeout.

## 42.9 `OsvCache`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsvVerdict {
    Clean,
    Blocked(String),
    FailOpen,
    Skipped,
}

pub struct OsvCache {
    entries: HashMap<String, (u64, Option<String>)>,
}
```

A clean result is a cached or fresh verdict with no `MAL-*` id, and `Blocked` owns the id it found. `FailOpen` is a network miss that wasn't stored. `Skipped` is a command this check doesn't handle. The map value is `(expiry, malware id)`, where `None` means clean and `Some` means blocked. `len` and `is_empty` report the map, and `Default` calls `new`.

```rust
pub fn check(
    &mut self,
    command: &str,
    package: &str,
    now: u64,
    network_ok: bool,
    ids: &[impl AsRef<str>],
) -> OsvVerdict {
    let _ = OSV_TIMEOUT_SECONDS;
    if command != "npx" && command != "uvx" {
        return OsvVerdict::Skipped;
    }
    let key = format!("{command}:{package}");
    if let Some((expiry, cached)) = self.entries.get(&key) {
        if *expiry > now {
            return match cached {
                Some(id) => OsvVerdict::Blocked(id.clone()),
                None => OsvVerdict::Clean,
            };
        }
    }
    if !network_ok {
        return OsvVerdict::FailOpen;
    }
    let malware = ids
        .iter()
        .map(|id| id.as_ref().to_string())
        .find(|id| id.starts_with("MAL-"));
    self.entries.insert(
        key,
        (now.saturating_add(OSV_CACHE_TTL_SECONDS), malware.clone()),
    );
    match malware {
        Some(id) => OsvVerdict::Blocked(id),
        None => OsvVerdict::Clean,
    }
}
```

The line `let _ = OSV_TIMEOUT_SECONDS` puts Hermes's timeout in this function by name. It doesn't sleep, and it doesn't call `urlopen`. `network_ok` stands in for that call, and `ids` stands in for the filtered vulnerability list `_query_osv` would return.

The command test is exact. `echo` is `Skipped`, and the test checks `is_empty` afterward. `npx.cmd`, `uvx.exe`, `pipx`, and `/usr/bin/npx` are also `Skipped`, because none of them is the string `npx` or the string `uvx`. Hermes would take the basename and would also map `pipx`, and those spellings stay deferred with the live query.

The key is `command:package`, so `npx:leftpad` and `uvx:leftpad` are different entries. Version isn't part of the key. Hermes keys the triple `(ecosystem, package, version)`.

A hit requires `expiry > now`. A stamp equal to `now` is stale, and the function falls through to the network stand-in. The stored verdict is returned even when the caller passes different ids, and that reuse is the cache. The test stores a clean CVE for `npx` and `leftpad` at time 0, then calls again at time 10 with `network_ok` false and `MAL-1`. The result is still `Clean`, because the expiry is 3600 and a fresh hit never reads the network flag.

A miss with `network_ok` false returns `FailOpen` and inserts nothing. The test checks `uvx` and `bad` that way, then asserts `len` is still 1, the clean `npx` entry. A later call with `network_ok` true and `["MAL-9", "CVE-1"]` finds `MAL-9` first, inserts it, and returns `Blocked("MAL-9")`. A call at `OSV_CACHE_TTL_SECONDS - 1` with only a CVE still returns `Blocked("MAL-9")`, because the cached id is what comes back and the CVE in that later slice is ignored.

Searching with `find` stops at the first id that starts with `MAL-`. A list of only `CVE-2020-1` stores `None` and returns `Clean`. Hermes's `_query_osv` already drops non-`MAL-` rows before the caller sees them, and this function filters again so a mixed slice still blocks on the malware id.

> **Rust sidebar: `impl AsRef<str>` in the slice**
>
> `ids: &[impl AsRef<str>]` accepts `&[&str]` and `&[String]`. `as_ref` borrows each id as `&str`, and the map owns a `String` only for the `MAL-*` hit it keeps. The cache clones that `String` into the verdict on the way out, so the map and the caller each own a copy. `saturating_add` on the expiry avoids a wrap if `now` is near `u64::MAX`, and the tests use small timestamps.

## 42.10 `FakeModel` and `demo`

```rust
pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

pub fn demo() -> String {
    let mut service = Service::new(&["a", "b"], 4);
    let route = service.place("s1").expect("a live replica");
    let _ = FakeModel { reply: "ok" }.complete(&route);
    format!("route: {route}")
}
```

Completion ignores the prompt and returns the stored reply. `demo` places `s1`, which lands on `a`, passes that name into `complete`, discards `ok`, and formats `route: a`. The quota of 4 goes unused because `demo` doesn't call `accept`. `expect` panics if `place` returns `None`, but with `a` and `b` both starting alive the demo doesn't hit that panic. There is no API key on this path, and there is no model call either.

Over in `federation`, `FakeModel` is a different type, and its `complete` takes no prompt. `ScriptedModel` in `lib.rs` counts calls for the learned gateway. The binary doesn't construct either of those.

> **Rust sidebar: `expect` and a discarded binding**
>
> `Option::expect` turns `None` into a panic with the message `"a live replica"`. `let _ =` drops the `String` from `complete`, and the format string uses the route rather than the model reply. That is why the printed line is `route: a` even though the model was asked.

## Tests

The new tests live in `steps/ch42-service/tests/service.rs`. The package also builds the copied `tests/balancing.rs` and `tests/learned.rs`, and the nextest block below is the package run.

The test `kill_moves_the_session_to_the_next_replica` places `s1` on `a` with replicas `a` and `b` and quota 4. It holds `config`, reloads generation 2, and expects the held generation to stay 1, the reloaded generation to be 2, and the held strong count to be 1. `kill("a")` then leaves `route("s1")` as `Some("b")`. `healthy` and `ready` stay true because `b` is alive and the drain flag is still false.

The test `quota_audit_and_drain` checks `bearer_ok("Bearer token")`, rejects `"token"`, and rejects `"Bearer "`. Quota 1 accepts `tenant` on `s1` with `Bearer token`, and the next accept of `s2` for the same tenant is `Err(Reject::Quota)`. A header of `nope` is `Err(Reject::BadToken)`. Some audit line starts with `accept `, and the span vec equals `["gateway.call 1"]`. `start_drain` makes `ready` false, an accept for `other` is `Err(Reject::Draining)`, and `drained` stays false until `finish` of that id. After that finish, `drained` is true and `inflight` is 0. `FakeModel { reply: "ok" }` completes `"a"`, and the result is dropped.

The test `a_dead_pool_refuses_the_call` kills the only replica `a` and expects `healthy` false, with `accept` returning `Err(Reject::NoReplica)`.

The test `sampling_stops_at_ten_per_minute` allows `SAMPLING_MAX_RPM` calls at timestamp 0, refuses the next call at 0, and allows one call at 60.

The test `url_elicitation_is_declined` expects `elicit(false)` to be `Elicit::Approval { timeout_seconds: ELICIT_TIMEOUT_SECONDS }` and `elicit(true)` to be `Elicit::Declined`.

The test `osv_blocks_malware_and_fails_open_on_the_network` skips `echo` and `leftpad` even with `MAL-1`, and the cache stays empty. `npx` and `leftpad` with `CVE-2020-1` is `Clean`. The same key at time 10 with the network down and `MAL-1` is still `Clean`. `uvx` and `bad` with the network down is `FailOpen`, and `len` is 1. The same `uvx` key with the network up and `MAL-9` plus `CVE-1` is `Blocked("MAL-9")`. At `OSV_CACHE_TTL_SECONDS - 1`, a CVE-only slice is still `Blocked("MAL-9")`.

The test `demo_routes_to_the_first_replica` expects `demo()` to equal `route: a`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch42-service
```

```text
    Starting 18 tests across 5 binaries
     Summary [   0.085s] 18 tests run: 18 passed, 0 skipped
```

```bash
cargo run -p ch42-service
```

```text
route: a
```

The binary places one session on the first replica, and it does that with no key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `route_session` | 19.7 ns | a session id picks a replica | one map lookup after 64 placements |

```rust
fn route_session(c: &mut Criterion) {
    let mut service = Service::new(&["a", "b", "c", "d"], 8);
    for n in 0..64 {
        service.place(&format!("s{n}"));
    }
    c.bench_function("route_session", |b| {
        b.iter(|| {
            let route = service.route(black_box("s0"));
            black_box(route);
        });
    });
}
```

Setup calls `Service::new` for replicas `a` through `d` with quota 8 before the timer starts. `place` runs for `s0` through `s63`, and the timed loop only calls `route` on `s0`. The number for `42/route_session` is deliberately a `HashMap` get, not `kill` plus replacing sessions, and not an HTTP load generator.

The means are `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `42`. Bench ids: `42/route_session`, `40/route_session`, `38/route_session`. Command: `cargo bench -p ch42-service --bench service --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

Concurrent calls are what `accept` counts, so `finish` has to name the same tenant or the slot stays taken. A missing id returns false and leaves the quota alone. `NoReplica` appends no audit line, while the other three refusals each append one.

A service stays undrained until `start_drain` has been called. An empty in-flight list on a service that is still accepting hasn't drained, and `ready` goes false as soon as the flag is set, while the accepted call is still in the vec.

The OSV cache returns the stored verdict when `expiry > now`, including when the new call passes different ids or says the network is down. A network error has to stay out of the map. The `uvx` test would see `len` grow if `FailOpen` were inserted, and a later success would then have to fight that entry. Hermes's `_cache_put` is also skipped on the `except` path of `check_package_for_malware`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Sampling (M7).** Ten requests a minute per sampler. The window is 60 seconds. `SAMPLING_MAX_RPM` and `SAMPLING_WINDOW_SECONDS` are the constants. Hermes's sampling LLM timeout of 30 seconds is `SamplingHandler.timeout`. It is not a constant in this file and it is not slept.

**Elicitation (M7).** Form mode is `Elicit::Approval` with `ELICIT_TIMEOUT_SECONDS` of 300. URL mode is `Declined`. There is no browser and no call to `request_elicitation_consent`. Hermes declines URL mode in `ElicitationHandler.__call__` because it needs a browser and `elicitation/complete`.

**Malware check (H4).** `npx` and `uvx` only, as exact command strings. `MAL-*` blocks. A CVE does not. `OSV_TIMEOUT_SECONDS` is 10, named with `let _ =` and not slept. Cache TTL is 3600 seconds. A network error returns `FailOpen` and is not cached. Hermes's `_TIMEOUT`, `_CACHE_TTL_S`, and the uncached `except` in `check_package_for_malware` are the source of those three decisions.

**Resource server.** A call without `Bearer` and a token is `BadToken`. This process does not issue tokens. `tools/mcp_oauth.py` is the Hermes OAuth 2.1 client.

**Drain.** Accepted calls finish through `finish`. New calls get `Draining`. The marker file in `gateway/drain_control.py` is the messaging gateway. This bool does not read it.

**Left for later.** Token issuance, a real trace exporter, Redis, a listener, the OSV HTTP call, `pipx`, and path basenames. E1 and E2 live on the harness in chapter 20. This server crate does not call them. oha is not measured, because this step does not bind a socket.

## What you implement

The milestone lives in `my-harness/tests/ch42.rs`.

```rust
use my_harness::chapter_42_route_after_kill;

#[test]
fn killing_the_route_fails_over_to_the_other_replica() {
    assert_eq!(
        chapter_42_route_after_kill("a", "b").as_deref(),
        Some("b")
    );
}
```

The stub in `my-harness/src/milestones.rs` panics.

```rust
pub fn chapter_42_route_after_kill(killed: &str, other: &str) -> Option<String> {
    let _ = (killed, other);
    todo!("chapter 42")
}
```

That panic is the line `todo!("chapter 42")`. The test wants the route, after `killed` is removed, to be `Some("b")` when the other replica is `b`. In the step crate that is `place`, then `kill`, then `route`, and the stub stays `todo!`.

## Chapter summary

- A session id sticks to a replica name, and `kill` plus `place` moves it when that replica dies, while `route` is the lookup.
- Reload stores a new `Arc`, and the old one stays readable at the generation the caller cloned.
- `start_drain` refuses new accepts, `finish` clears the call that was already accepted, and `drained` needs both the flag and an empty list.
- The quota is concurrent and per tenant, and the success path checks it before `place`.
- A bearer is the prefix `Bearer ` plus a non-empty token with no space, and issuance stays deferred.
- Sampling allows 10 calls per 60 seconds, form elicitation waits up to 300 seconds, URL mode is declined, a `MAL-*` id is cached for 3600 seconds, and a network failure stays uncached.
- `FakeModel` answers `ok`, `demo` drops that reply, and the printed line is `route: a`.
- `route_session` has mean `19.7 ns`, which is `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).

## What's next

Chapter 43 is the harness again, where cron runs a job the model wrote. Chapter 51 copies this service later, when the token arms arrive.
