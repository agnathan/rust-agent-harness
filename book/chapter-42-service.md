# Chapter 42: MCP Gateway Service

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 39 admits a person to the chat gateway. Chapter 37 can pick a backend for a tool. Neither one keeps a session on a replica, and neither one is the process that would sit in front of those replicas. This chapter is that process, still inside one binary. A session id sticks to a replica name. Killing the replica moves the session. A reload swaps an `Arc` and leaves the old config in the caller's hand. A tenant has a concurrent quota. A drain finishes the calls already accepted and refuses new ones. A bearer token is required. Sampling stops at ten requests a minute. URL-mode elicitation is declined. An `npx` or `uvx` launch checks for `MAL-*` advisories and fails open when the network does not answer. Nothing here binds a socket. OAuth issuance stays deferred.

The step crate is `steps/ch42-service`. The book number is 42. The package is `ch42-service`. The library name is `gwservice`. `Cargo.toml` describes it as chapter 37 vendored, plus routes, a drain, quotas, and a bearer check. Edition is 2021. `rust-version` is 1.85. There is no path dependency on `ch37-balancing`. The sources are copied in. The binary is `src/main.rs`, and it prints `service::demo`.

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

[personal note: a session that moved when its replica went away, and a call the drain let finish]

## What this chapter decides

`place` keeps a session on its replica while that replica is alive. Otherwise it takes the first live name in the `Vec` built by `new`. `kill` marks that name dead and places each of its sessions again. `route` is the map lookup. It does not place.

`reload` replaces the `Arc<Config>`. A caller holding the old `Arc` still sees the old generation. `config` clones the pointer.

`accept` requires `Bearer` plus a non-empty token with no space. A drain returns `Draining`. A tenant at its concurrent quota returns `Quota`. No live replica returns `NoReplica`. A success appends `gateway.call {id}` to the span vec and an accept line to the audit vec, and counts one in-flight call. `finish` drops that call and frees the quota slot for the tenant name it is given. `drained` is true when a drain is on and the in-flight list is empty. `ready` is false while draining, or while every replica is dead.

`Sampler::allow` keeps stamps younger than 60 seconds. Ten pass. The eleventh inside the window does not.

`elicit(false)` is an approval with timeout 300. `elicit(true)` is `Declined`.

`OsvCache::check` returns `Skipped` for any command other than `npx` or `uvx`. A cached verdict whose expiry is still greater than `now` is returned as stored. A network failure returns `FailOpen` and writes nothing. Otherwise a `MAL-*` id blocks and a CVE-only list is clean. Both of those verdicts are cached until `now + 3600`.

`demo` places `s1` on `a` and returns `route: a`. The `FakeModel` reply is discarded. No key.

## 42.1 The crate vendors chapter 37

Chapter 37 used to be the server snapshot a reader had to have on disk beside this step. This chapter copies it, the same way chapter 37 copied the learned gateway. `Cargo.toml` does not list `ch37-balancing`.

`lib.rs` still opens with the chapter 23 comment about prediction. The modules it declares are the copied tree plus this chapter:

```rust
pub mod catalog;
pub mod federation;
pub mod limits;
pub mod service;
pub mod session;

pub use limits::{Catalog, LimitedGateway, SessionError, SessionView};
```

`catalog` is the chapter 7 gateway: native tools, plugin tools, an in-memory upstream list, and BM25 search. `session` is the chapter 8 view. `limits` is the chapter 15 wrapper: permits, an idempotent retry, a breaker, and the caps of 50. The root of `lib.rs` is the chapter 23 learned gateway: `predict`, `LearnedGateway`, composites, macros, and `run_code_mode`, with `SANDBOX_ALLOWED` and `CODE_MODE_MAX_CALLS` of 50. `federation` is chapter 37: `Policy`, `Pool::pick`, `pool_for`, and `discover` with `DISCOVERY_CONCURRENCY` of 4. Those functions were walked in those chapters. This chapter does not reimplement them and does not call them from `service`.

`service.rs` opens with a comment that still says book chapter 40. The book number is 42. The module is `gwservice::service`. `main.rs` is three lines:

```rust
use gwservice::service::demo;

fn main() {
    println!("{}", demo());
}
```

The learned `demo` at the bottom of `lib.rs` returns a `(String, String)` pair from code mode. The federation `demo` returns `pick: echo via alpha`. Neither is the binary. The binary calls `service::demo`.

### Rust: `pub mod` and a copied crate

`pub mod service` makes the module visible as `gwservice::service`. A private `mod service` would hide `Service` from the integration tests, which write `use gwservice::service::Service`. `pub use` puts `Catalog`, `LimitedGateway`, `SessionError`, and `SessionView` at the crate root so the copied learned tests can keep their imports. Vendoring means those types are compiled into `ch42-service`. A path dependency would have compiled them in `ch37-balancing` and linked them. The step would not be a snapshot.

The package depends on `serde_json`, `thiserror`, and `tokio`. `service.rs` uses none of them. The copied catalog and the session gateway do. Criterion is a dev-dependency. The bench target is `service`, `harness = false`.

## 42.2 The constants, `Config`, `Reject`, and `Service::new`

The numbers this chapter shares with Hermes sit at the top of `service.rs`.

```rust
pub const SAMPLING_MAX_RPM: usize = 10;
pub const SAMPLING_WINDOW_SECONDS: u64 = 60;
pub const ELICIT_TIMEOUT_SECONDS: u64 = 300;
pub const OSV_TIMEOUT_SECONDS: u64 = 10;
pub const OSV_CACHE_TTL_SECONDS: u64 = 3600;
```

`SAMPLING_MAX_RPM` is Hermes's default `max_rpm`. `SAMPLING_WINDOW_SECONDS` is the `60` inside `_check_rate_limit`. `ELICIT_TIMEOUT_SECONDS` is the elicitation default of 300. `OSV_TIMEOUT_SECONDS` is `_TIMEOUT`. `OSV_CACHE_TTL_SECONDS` is the default of `OSV_CHECK_CACHE_TTL`. There is no constant here for the sampling LLM timeout of 30. That number stays on `SamplingHandler.timeout`.

`Config` is one generation number. `Reject` is why `accept` refused.

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

`Service` owns the replica list, the session map, the config pointer, the per-tenant counters, the quota, the audit lines, the span names, the in-flight calls, the next call id, and the drain flag. The fields are private. Tests go through the methods.

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

`new` marks every name alive, starts generation 1, and starts the next call id at 1. The quota is the concurrent cap for every tenant. It is not a lifetime total.

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

The replica `Vec` keeps construction order. `place` later walks it from the front. `HashMap` does not keep that order, which is why the live replica is not stored as a map of names.

### Rust: an enum as the error, and a tuple in a `Vec`

`Reject` is four units with no payload. `accept` returns `Result<u64, Reject>`. The `u64` is the call id. Matching on `Reject::Quota` is exhaustive only if every variant is named. `PartialEq` is why a test can write `Err(Reject::Quota)` and compare.

`Vec<(String, bool)>` is a list of pairs. The `bool` is liveness. A struct with named fields would read more clearly at the call site. The pair is what `new` builds, and `alive` matches on it. The session map is a separate `HashMap<String, String>`: session id to replica name. The replica list is the source of who is alive. The map is only the assignment.

## 42.3 `route`, `place`, and `kill`

`route` borrows the stored name. It does not insert. A session that was never placed is `None`.

```rust
pub fn route(&self, session: &str) -> Option<&str> {
    self.routes.get(session).map(String::as_str)
}
```

`place` is the assignment. If the session already has a name and `alive` says that replica is still up, the same name comes back and the map is unchanged. Otherwise the first live replica is inserted and returned. An empty live set returns `None`. The previous map entry is left in place only when the current replica is alive. When it is dead, `insert` overwrites.

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

`kill` sets that replica's flag to false, collects every session currently mapped to it, removes those keys, and calls `place` on each. The collect step is required. `place` mutates `routes`, so the loop cannot borrow the map and insert into it in one walk.

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

`alive` is private. A name that was never passed to `new` is not alive. `kill` of a name that is not in the list changes no flag and moves no session, because the filter matches nothing.

`Service::new(&["a", "b"], 4)` then `place("s1")` returns `Some("a")`. `a` is the first live name. A second `place("s1")` hits the early return and stays on `a`. `kill("a")` clears the flag, removes `s1`, and `place` finds `b`. `route("s1")` is then `Some("b")`. That is the failover the milestone asks for. If `b` is also dead, `place` returns `None` after the remove, and `route` is `None`.

`healthy` is any live replica. It does not look at the drain flag.

```rust
pub fn healthy(&self) -> bool {
    self.replicas.iter().any(|(_, alive)| *alive)
}
```

### Rust: `Option`, `?`, and collecting before mutate

`find` returns `Option<&(String, bool)>`. The `?` on that option turns "no live replica" into `None` from `place`. The `.0.clone()` runs only when a pair was found. `route` uses `map(String::as_str)` so the caller borrows the `String` inside the map. The borrow cannot outlive the `&self` borrow. `place` returns an owned `String` because the caller, and `kill`, need the name after further mutation.

The `sessions` vector owns the keys. After `collect`, the immutable borrow of `routes` has ended. `remove` and `place` can take `&mut self`. Skipping the vector would be a borrow-checker error, not a style choice.

`HashMap` iteration order is not the replica order. With one session the order does not matter. With several sessions on a dead replica, each is placed onto the same first live name. They do not fan out across the remaining replicas.

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

`new` stores generation 1. `config` bumps the strong count and returns that pointer. `reload` allocates a new `Config` and stores it. The previous `Arc` stays valid for whoever cloned it. The test holds the pointer, reloads generation 2, and checks `held.generation` is still 1 while `reloaded.generation` is 2. `Arc::strong_count(&held)` is 1, because the service dropped its clone when it stored the new pointer. The reloaded pointer is held by the service and by `reloaded`, so its count is 2. The test does not assert that second count.

Readers of the old `Arc` do not see the new generation. There is no lock around the swap. `reload` takes `&mut self`, so in this type the swap is exclusive. The `Arc` matters for the caller who already cloned, not for two threads calling `reload`. `Service` is not shared behind a mutex in this file.

### Rust: `Arc` is the snapshot

`Arc::clone` clones the pointer, not the `Config`. `Arc::new` allocates a fresh one. After `reload`, the two pointers are not equal as allocations, and their `generation` fields differ. Dropping the service drops the new pointer. The caller's old pointer still reads generation 1 until the caller drops it. That is the same shape as the catalog reload in chapter 7: the in-flight reader keeps the snapshot it loaded.

## 42.5 `accept`, `finish`, and the drain

`ready` is health and the absence of a drain. A dead pool is not ready. A live pool that has started a drain is not ready either.

```rust
pub fn ready(&self) -> bool {
    self.healthy() && !self.draining
}
```

`accept` checks the header, then the drain, then the quota, then a replica. The first failure returns. Later checks do not run.

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

A bad token writes `deny {tenant}` and does not place the session. A drain writes `drain {tenant}` and does not place. A quota miss writes `quota {tenant}` and does not place. `NoReplica` writes nothing. The audit vec stays as it was. That branch is the one the dead-pool test hits after `kill` of the only replica: `healthy` is false, and `accept` is `Err(Reject::NoReplica)`.

The quota read uses `entry`. A tenant seen for the first time gets a zero counter. `used >= self.quota` refuses before the increment. A quota of 1 allows one accept. The second concurrent accept for that tenant is `Quota`, even for a different session. A different tenant has its own counter. The test does not accept a second tenant before the drain. After the first success the counter is `used + 1`, stored with `saturating_add`. With a `u32` cap that add does not wrap.

The success path places the session, stores the call as `(id, session)`, pushes `gateway.call {id}`, and pushes `accept {tenant} {session}`. The first id is 1 because `next_id` starts at 1 and is incremented after the copy. The span test expects exactly `["gateway.call 1"]`.

`finish` finds the id, removes that pair, and decrements the named tenant. A missing id returns false and does not touch the counter.

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

The tenant string is the caller's responsibility. `finish(id, "other")` removes the call and looks up `other`. If `other` has no counter, the original tenant's slot stays taken. A later `accept` for the original tenant still sees the old count.

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

`start_drain` does not finish calls and does not clear the map. `drained` is false while a call is still in the vec, and it is also false before `start_drain` even when the vec is empty. The test accepts one call, starts the drain, sees `ready` false, sees a new accept return `Draining`, sees `drained` false, finishes the id, and then sees `drained` true and `inflight` 0. The finished call is allowed to complete. The new call is not accepted. That is the drain. It is a bool, not `.drain_request.json`, and it does not consult `DRAIN_REQUEST_MAX_AGE_SECONDS`.

### Rust: `entry`, `let`/`else`, and `saturating_add`

`entry` returns a handle to the occupied or vacant slot. `or_insert(0)` writes zero only when the key is new, then yields `&mut u32`. The `*` copies the number out so the later `insert` can borrow the map again. Holding the `Entry` across the `insert` would keep the map borrowed.

`let Some(index) = ... else { return false }` is a let-else. The `None` path leaves the function. The `Some` path binds `index` for the remove. `position` walks the vec from the front and returns the first matching id.

`saturating_add(1)` and `saturating_sub(1)` stop at the ends of `u32`. A zero counter does not wrap to `u32::MAX` if `finish` runs once too often. A counter at `u32::MAX` does not wrap to zero on another accept. The quota test uses 1, so the saturate path is not what that test shows. The methods still use it.

## 42.6 `bearer_ok`

```rust
pub fn bearer_ok(header: &str) -> bool {
    let Some(token) = header.strip_prefix("Bearer ") else {
        return false;
    };
    !token.is_empty() && !token.contains(' ')
}
```

The prefix is the six letters, the space, and nothing else. `Bearer token` passes. `token` fails. `Bearer ` fails because the remainder is empty. `Bearer token extra` fails because the remainder contains a space. The function does not split on more than that space check, does not decode base64, and does not verify a signature. `accept` calls it first. Issuing the token is not this crate. `tools/mcp_oauth.py` is the Hermes client that would talk to an authorization server. This service does not call it.

### Rust: `strip_prefix`

`strip_prefix` returns `Option<&str>` borrowing the tail of `header`. The pattern is not a regex. `Bearer` without the trailing space does not match. Case matters. `bearer token` fails the prefix and `accept` returns `BadToken`.

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

`Default` calls `new`. `allow` drops stamps whose age is not strictly less than 60. A stamp equal to `now` has age 0 and stays. A stamp from exactly 60 seconds ago has age 60, and `60 < 60` is false, so it is dropped. Hermes uses `t > now - 60`, which is the same cut: a timestamp exactly 60 seconds in the past is not kept.

The test calls `allow(0)` ten times. Each call sees a length under 10, appends 0, and returns true. The eleventh call at timestamp 0 sees length 10 and returns false without appending. `allow(60)` drops every 0, sees an empty deque, appends 60, and returns true. The refused eleventh call did not occupy a slot. A clock that goes backwards is handled by `saturating_sub`: if `now` is less than a stored stamp, the age is 0 and the stamp stays.

The sampler is not inside `Service`. `accept` does not call `allow`. A caller who wants the limit owns a `Sampler` and asks it. One sampler is one window. Hermes keeps `_rate_timestamps` on the `SamplingHandler` for one MCP server. Two `Sampler` values do not share stamps.

The function does not sleep, and it does not read `SamplingHandler.timeout`. The 30 second sampling budget is Hermes's `asyncio.wait_for` around the LLM call. This `allow` returns a bool.

### Rust: `VecDeque` and `retain`

`VecDeque` is a ring buffer. `push_back` adds at the end. `retain` keeps elements for which the closure returns true and drops the rest, preserving order. A `Vec` would do the same job at this length. The deque matches a window that drops from the old side. The length check uses `>= SAMPLING_MAX_RPM`, so the cap is ten stored stamps, not eleven.

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

`elicit(false)` is the form-mode path: an approval whose timeout is 300. `elicit(true)` is URL mode: `Declined`. Hermes reads `getattr(params, "mode", "form")` and treats `"url"` as the decline. Anything else, including the default `"form"`, goes to consent. This function takes a bool the caller has already decided. It does not parse a mode string, does not call `request_elicitation_consent`, and does not sleep for 300 seconds. The outer grace of `_OUTER_TIMEOUT_GRACE_SECONDS = 5` is Hermes's asyncio safety net. It is not added here.

`Copy` is derived because both variants are a bool's worth of data plus a `u64`. A test compares the approval with `==`.

### Rust: a struct-like variant

`Elicit::Approval { timeout_seconds }` is a variant with a field. Matching it binds the number. `Elicit::Declined` has no field. Putting the timeout on the approval variant means a declined result cannot be read as 300 seconds by mistake. The number is still the constant. The function does not take a caller-supplied timeout.

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

`Clean` is a cached or fresh verdict with no `MAL-*` id. `Blocked` owns the id. `FailOpen` is a network miss that was not stored. `Skipped` is a command this check does not handle. The map value is `(expiry, malware id)`. `None` means clean. `Some` means blocked. `len` and `is_empty` report the map. `Default` calls `new`.

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

`let _ = OSV_TIMEOUT_SECONDS` uses the constant so the timeout Hermes budgets is named in this function. The function does not sleep and does not call `urlopen`. `network_ok` is the stand-in for that call. `ids` is the stand-in for the filtered vulnerability list `_query_osv` would return.

The command test is exact. `echo` is `Skipped`, and the test checks `is_empty` afterward. `npx.cmd`, `uvx.exe`, `pipx`, and `/usr/bin/npx` are also `Skipped`, because none of them is the string `npx` or the string `uvx`. Hermes would take the basename and would also map `pipx`. Those spellings are deferred with the live query.

The key is `command:package`. `npx:leftpad` and `uvx:leftpad` are different entries. Version is not part of the key. Hermes keys `(ecosystem, package, version)`.

A hit requires `expiry > now`. Equal to `now` is stale, and the function falls through. The stored verdict is returned even if the caller passes different ids. That is the cache. The test stores a clean CVE for `npx` / `leftpad` at time 0, then calls again at time 10 with `network_ok` false and `MAL-1`. The result is still `Clean`, because the expiry is 3600 and the network flag is never read on a fresh hit.

A miss with `network_ok` false returns `FailOpen` and does not insert. The test checks `uvx` / `bad` that way and then asserts `len` is still 1, the clean `npx` entry. A later call with `network_ok` true and `["MAL-9", "CVE-1"]` finds `MAL-9` first, inserts it, and returns `Blocked("MAL-9")`. A call at `OSV_CACHE_TTL_SECONDS - 1` with only a CVE still returns `Blocked("MAL-9")`, because the cached id is what comes back. The CVE in that later slice is ignored for the same reason.

`find` stops at the first id that starts with `MAL-`. A list of only `CVE-2020-1` stores `None` and returns `Clean`. Hermes's `_query_osv` already drops non-`MAL-` rows before the caller sees them. This function filters again so a mixed slice still blocks on the malware id.

### Rust: `impl AsRef<str>` in the slice

`ids: &[impl AsRef<str>]` accepts `&[&str]` and `&[String]`. `as_ref` borrows each id as `&str`, and the map owns a `String` only for the `MAL-*` hit it keeps. The cache clones that `String` into the verdict on the way out, so the map and the caller each own their copy. `saturating_add` on the expiry avoids a wrap if `now` is near `u64::MAX`. The tests use small timestamps.

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

`complete` ignores the prompt and returns the stored reply. `demo` places `s1`, which is `a`, passes that name into `complete`, discards `ok`, and formats `route: a`. The quota of 4 is unused because `demo` does not call `accept`. `expect` panics if `place` returns `None`. With `a` and `b` both starting alive, it does not panic. There is no API key and no model call.

`federation::FakeModel` is a different type. Its `complete` takes no prompt. `ScriptedModel` in `lib.rs` counts calls for the learned gateway. The binary does not construct those.

### Rust: `expect` and a discarded binding

`Option::expect` turns `None` into a panic with the message `"a live replica"`. `let _ =` drops the `String` from `complete`. The format string uses the route, not the model reply. That is why the printed line is `route: a` even though the model was asked.

## Tests

The new tests are `steps/ch42-service/tests/service.rs`. The package also builds the copied `tests/balancing.rs` and `tests/learned.rs`. The nextest line in the next section is the package run. I am not typing a pass count for it.

`kill_moves_the_session_to_the_next_replica` places `s1` on `a` with replicas `a` and `b` and quota 4. It holds `config`, reloads generation 2, and expects the held generation to stay 1, the reloaded generation to be 2, and the held strong count to be 1. `kill("a")` then leaves `route("s1")` as `Some("b")`. `healthy` and `ready` stay true because `b` is alive and the drain flag is still false.

`quota_audit_and_drain` checks `bearer_ok("Bearer token")`, rejects `"token"`, and rejects `"Bearer "`. Quota 1 accepts `tenant` / `s1` / `Bearer token`. The next accept of `s2` for the same tenant is `Err(Reject::Quota)`. A header of `nope` is `Err(Reject::BadToken)`. Some audit line starts with `accept `. The span vec equals `["gateway.call 1"]`. `start_drain` makes `ready` false. An accept for `other` is `Err(Reject::Draining)`. `drained` is false until `finish` of that id, then true, and `inflight` is 0. `FakeModel { reply: "ok" }` completes `"a"` and the result is dropped.

`a_dead_pool_refuses_the_call` kills the only replica `a` and expects `healthy` false and `accept` to return `Err(Reject::NoReplica)`.

`sampling_stops_at_ten_per_minute` allows `SAMPLING_MAX_RPM` calls at timestamp 0, refuses the next at 0, and allows one at 60.

`url_elicitation_is_declined` expects `elicit(false)` to be `Elicit::Approval { timeout_seconds: ELICIT_TIMEOUT_SECONDS }` and `elicit(true)` to be `Elicit::Declined`.

`osv_blocks_malware_and_fails_open_on_the_network` skips `echo` / `leftpad` even with `MAL-1`, and the cache stays empty. `npx` / `leftpad` with `CVE-2020-1` is `Clean`. The same key at time 10 with the network down and `MAL-1` is still `Clean`. `uvx` / `bad` with the network down is `FailOpen`, and `len` is 1. The same `uvx` key with the network up and `MAL-9` plus `CVE-1` is `Blocked("MAL-9")`. At `OSV_CACHE_TTL_SECONDS - 1`, a CVE-only slice is still `Blocked("MAL-9")`.

`demo_routes_to_the_first_replica` expects `demo()` to equal `route: a`.

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

The binary places one session on the first replica. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `route_session` | @@MEAN:42/route_session@@ | a session id picks a replica | one map lookup after 64 placements |

The function name is `route_session`. The bench id is `42/route_session`. The old ids `40/route_session` and `38/route_session` are aliases in `benchmarks/descriptions.json`. The function name did not change when the id moved. I did not rewrite gh-pages history.

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

`Service::new` builds replicas `a` through `d` with quota 8 before the timer. `place` runs for `s0` through `s63`. Each lands on `a`, the first live name. The timed closure is `b.iter`. Inside it, `route` of `s0` is passed through `black_box`, then the `Option` is passed through `black_box`. The `HashMap` get is the sample. `place`, `kill`, `accept`, `Sampler`, `OsvCache`, and `FakeModel` are not called. No socket is opened. oha is not this number. The descriptions file says the result is `Some(a)`.

`.github/scripts/run_bench_section.sh` runs `cargo bench -p ch42-service --bench service --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10` for section `42`. The Mean cell is `mean.point_estimate` from that workflow's `estimates.json`. Criterion also prints a `time:` line. That line is the slope. It is not this cell. I am not pasting a nanosecond figure in its place.

### Milestone demo

The `cargo run` above. The line is `route: a`.

## Pitfalls

The bench is the lookup. Failover walks the sessions of a dead replica, clones their ids, and calls `place`. That work is the test `kill_moves_the_session_to_the_next_replica`. It is not this sample. A sample that started `kill` would be a different measurement.

`accept` counts concurrent calls. `finish` has to name the same tenant or the slot stays taken. A missing id returns false and leaves the quota alone. `NoReplica` does not append an audit line. The other three refusals do.

`drained` is false until `start_drain` has been called. An empty in-flight list on a service that is still accepting is not drained. `ready` goes false as soon as the flag is set, while the accepted call is still in the vec.

The OSV cache returns the stored verdict when `expiry > now`, including when the new call passes different ids or says the network is down. A network error must not be stored. The `uvx` test would see `len` grow if `FailOpen` were inserted, and a later success would have to fight that entry. Hermes's `_cache_put` is also skipped in the `except` path of `check_package_for_malware`.

`allow` refuses without pushing when the deque is already at 10. The refused stamp is not in the window. At age 60 the old stamps drop because the comparison is strict `<`.

The bearer check does not validate a signature. An empty token and a missing `Bearer ` prefix fail closed. Issuing tokens is `tools/mcp_oauth.py` on the client side, and it is not this crate.

`kill` collects keys because `place` mutates the map. `HashMap` order is not replica order. Every moved session goes to the first remaining live name.

`command` is compared to `npx` and `uvx` only. A path or `pipx` is `Skipped` and is not a fail-open. Fail-open is the network flag after the command has matched.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Sampling (M7).** Ten requests a minute per sampler. The window is 60 seconds. `SAMPLING_MAX_RPM` and `SAMPLING_WINDOW_SECONDS` are the constants. Hermes's sampling LLM timeout of 30 seconds is `SamplingHandler.timeout`. It is not a constant in this file and it is not slept.

**Elicitation (M7).** Form mode is `Elicit::Approval` with `ELICIT_TIMEOUT_SECONDS` of 300. URL mode is `Declined`. There is no browser and no call to `request_elicitation_consent`. Hermes declines URL mode in `ElicitationHandler.__call__` because it needs a browser and `elicitation/complete`.

**Malware check (H4).** `npx` and `uvx` only, as exact command strings. `MAL-*` blocks. A CVE does not. `OSV_TIMEOUT_SECONDS` is 10, named with `let _ =` and not slept. Cache TTL is 3600 seconds. A network error returns `FailOpen` and is not cached. Hermes's `_TIMEOUT`, `_CACHE_TTL_S`, and the uncached `except` in `check_package_for_malware` are the source of those three decisions.

**Resource server.** A call without `Bearer` and a token is `BadToken`. This process does not issue tokens. `tools/mcp_oauth.py` is the Hermes OAuth 2.1 client.

**Drain.** Accepted calls finish through `finish`. New calls get `Draining`. The marker file in `gateway/drain_control.py` is the messaging gateway. This bool does not read it.

**Left for later.** Token issuance, a real trace exporter, Redis, a listener, the OSV HTTP call, `pipx`, and path basenames. E1 and E2 are still absent. oha is not measured, because this step does not bind a socket.

## What you implement

`my-harness/tests/ch42.rs` is the milestone.

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

`todo!("chapter 42")` is the panic. The test wants the route, after `killed` is removed, to be `Some("b")` when the other replica is `b`. In the step crate that is `place`, then `kill`, then `route`. The stub stays `todo!`. Do not paste `Service` into `my-harness/`.

## Chapter summary

- **Route.** A session id, a replica name, and a move when that replica dies. The lookup is `route`. The move is `kill` plus `place`.
- **Reload.** A new `Arc`. The old one stays readable at the generation the caller cloned.
- **Drain.** `start_drain` refuses new accepts. `finish` clears the call that was already accepted. `drained` is both.
- **Quota.** Concurrent, per tenant, checked before `place` on the success path.
- **Bearer.** `Bearer ` plus a non-empty token with no space. Issuance stays deferred.
- **Client checks.** Sampling at 10 per 60 seconds. Form elicitation at 300 seconds. URL mode declined. `MAL-*` cached for 3600 seconds. Network failure uncached.
- **Model.** `FakeModel` answers `ok` and `demo` drops it. The line is `route: a`.
- **Time.** `route_session` is `42/route_session`. The mean is @@MEAN:42/route_session@@.

[personal note: a failover that was a map update]

## What's next

Chapter 43 is cron, on the chapter 41 harness. A job is claimed by one owner. A crash may release that claim one time. The prompt is scanned before the claim. A lifecycle command in the job is blocked here, in `check_lifecycle`. That check does not move to the chapter 39 gateway.
