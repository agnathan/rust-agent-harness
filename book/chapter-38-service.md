# Chapter 38: MCP Gateway Service

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 37 admits a person to the chat gateway. This chapter is the other process: the MCP gateway. A session id sticks to a replica. Killing that replica moves the session. A reload swaps an `Arc` and leaves the old config in the caller's hand. A tenant has a concurrent quota. A drain finishes the calls already accepted and refuses new ones. A bearer token is required. Sampling stops at ten requests a minute. URL-mode elicitation is declined. An `npx` or `uvx` launch checks for `MAL-*` advisories and fails open when the network does not answer. Nothing here binds a socket.

The step crate is `steps/gw-service`. The book number is 38. The package is `gw-service`.

| Section | Step | What it adds |
|---|---|---|
| 38 | `steps/gw-service` | session routing, an `Arc` reload, quotas, a drain, sampling, elicitation, an OSV check |

> **How Hermes does it**
>
> I read `tools/mcp_tool_sampling.py` (`max_rpm`, `_check_rate_limit`, the elicitation callback) and `tools/osv_check.py` (`_TIMEOUT`, `_CACHE_TTL_S`, `check_package_for_malware`, `_query_osv`) on Hermes `main` at `e31c674f`.
>
> Sampling `max_rpm` defaults to 10. The window keeps timestamps newer than `now - 60` seconds. The sampling LLM timeout defaults to 30 seconds. Form-mode elicitation goes through approval with a default timeout of 300 seconds. URL-mode is declined. The OSV check looks up `npx` and `uvx` packages, keeps only ids that start with `MAL-`, ignores regular CVEs, times out at 10 seconds, caches a verdict for 3600 seconds, and returns allow on a network error without caching that failure. Hermes is the MCP client. The catalog has no gateway-OAuth row.
>
> **Adopted.** The sampling cap and the 60 second window. Form-mode approval at 300 seconds. URL-mode declined. OSV timeout 10, cache 3600, `MAL-*` only, fail open and uncached on a network error, skip when the command is not `npx` or `uvx`.
>
> **Adapted.** Replicas, the session map, quotas, the audit vec, and span names are this book's service, from the gateway notes. Hot reload is the `Arc` swap from chapter 7. The bearer check is a non-empty `Bearer` token, which is the resource-server decision, not an authorization server. `FakeModel` is unused by routing. No Redis, no axum, no socket. The 10 second and 300 second timeouts are named and not slept.
>
> **Deferred.** A live listener, OAuth 2.1 token issuance, a real exporter, and oha. The route bench is a map lookup.

[personal note: a session that moved when its replica went away, and a call the drain let finish]

## What this chapter decides

`place` keeps a session on its replica while that replica is alive. Otherwise it takes the first live name. `kill` marks the replica dead and places each of its sessions again. `route` is the map lookup.

`reload` replaces the `Arc<Config>`. A caller holding the old `Arc` still sees the old generation.

`accept` requires `Bearer` plus a non-empty token with no space. A drain returns `Draining`. A tenant at its concurrent quota returns `Quota`. No live replica returns `NoReplica`. A success appends `gateway.call {id}` to the span vec and an accept line to the audit vec, and counts one in-flight call. `finish` drops that call and frees the quota slot. `drained` is true when a drain is on and the in-flight list is empty. `ready` is false while draining, or while every replica is dead.

`Sampler::allow` keeps stamps younger than 60 seconds. Ten pass. The eleventh inside the window does not.

`elicit(false)` is an approval with timeout 300. `elicit(true)` is `Declined`.

`OsvCache::check` returns `Skipped` for any command other than `npx` or `uvx`. A cached verdict newer than `now` is returned as stored. A network failure returns `FailOpen` and writes nothing. Otherwise a `MAL-*` id blocks and a CVE-only list is clean. Both verdicts are cached until `now + 3600`.

`demo` places `s1` on `a` and returns `route: a`. No key.

## Rust: the route is a HashMap

`routes` maps a session id to a replica name. `route` borrows that name. The bench places 64 sessions before the sample and times one lookup of `s0`. The `Arc` reload is the same shape as chapter 7: readers keep the old pointer, the service stores the new one.

## Tests

Seven tests. Killing `a` moves `s1` to `b`, and a held `Arc` stays at generation 1 while the service moves to 2. Quota 1 rejects the second concurrent accept. A missing bearer is `BadToken`. The audit line and the span are recorded. Drain rejects a new call, `finish` clears the in-flight list, and `drained` becomes true. A dead pool is not healthy and `accept` returns `NoReplica`. Eleven sampling calls at timestamp 0 refuse the last one, and a call at 60 seconds is allowed. URL mode is declined. `echo` is skipped. A CVE is clean and stays clean from the cache when the network is down. `uvx` with a network error is fail-open and uncached, then `MAL-9` blocks and the cache holds that block. The demo is `route: a`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p gw-service
```

```text
     Summary [   0.007s] 7 tests run: 7 passed, 0 skipped
```

```bash
cargo run -p gw-service
```

```text
route: a
```

The binary places one session on the first replica. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `route_session` | 19.43 ns | a session id picks a replica | one map lookup after 64 placements |

`route_session` builds a service with replicas `a` through `d` and calls `place` for `s0` through `s63` before the bench. The timed closure is `b.iter`. Inside: `route` of `s0` passed through `black_box`, then `black_box` of the `Option`. The `HashMap` get is the sample. `place`, `kill`, `accept`, `Sampler`, `OsvCache`, and `FakeModel` are not called. No socket is opened. oha is not this number.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `38`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `route: a`.

## Pitfalls

The bench is the lookup. Failover walks the sessions of a dead replica and is the test, not this sample. A millisecond here would mean the sample started `kill`.

`accept` counts concurrent calls, not a lifetime total. `finish` has to name the same tenant or the slot stays taken.

The OSV cache returns the stored verdict even if you pass different ids before the TTL. That is the point of the cache. A network error must not be stored, or a blip becomes an hour of fail-open.

The bearer check does not validate a signature. An empty token and a missing `Bearer ` prefix fail closed. Issuing tokens is not this crate.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Sampling (M7).** Ten requests a minute per sampler. The window is 60 seconds. The LLM timeout of 30 seconds is Hermes's sampling call and is not slept here.

**Elicitation (M7).** Form mode is an approval with a 300 second timeout. URL mode is `Declined`. There is no browser.

**Malware check (H4).** `npx` and `uvx` only. `MAL-*` blocks. A CVE does not. Timeout 10 seconds, named and not slept. Cache TTL 3600 seconds. A network error allows the launch and is not cached.

**Resource server.** A call without `Bearer` and a token is `BadToken`. This is not the Hermes client. Hermes does not ship a gateway OAuth row.

**Left for later.** Token issuance, a real trace exporter, Redis, and a listener. E1 and E2 are still absent.

## Chapter summary

- **Route.** A session id, a replica name, and a move when that replica dies.
- **Reload.** A new `Arc`. The old one stays readable.
- **Drain.** Accepted calls finish. New calls wait.
- **Quota.** Concurrent, per tenant.
- **Client checks.** Sampling, elicitation, and `MAL-*`.
- **Model.** `FakeModel` answers `ok`. The demo needs no key.
- **Time.** One route measured **19.43 ns**.

[personal note: a failover that was a map update]

## What's next

Chapter 39 is cron. A job is claimed once. A crash may retry that claim one time. The prompt is scanned before the job runs.
