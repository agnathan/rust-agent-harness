# Chapter 35: Balancing and Federation

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 34 can restart a child in the harness. The gateway still has one backend per tool. This chapter gives a tool a pool. The pick is local: the backend with the fewest outstanding requests, or the quieter of two indexes. Upstream servers that offer the same tool share that pool. Discovery connects at most four of them at once. Nothing here opens a socket.

The step crate is `steps/gw-balancing`. The book number is 35. The package is `gw-balancing`.

| Section | Step | What it adds |
|---|---|---|
| 35 | `steps/gw-balancing` | least outstanding, power of two choices, federation, a discovery cap of 4 |

> **How Hermes does it**
>
> I read `tools/mcp_tool_discovery.py` (`_DISCOVERY_CONNECT_CONCURRENCY`, `_discovery_connect_concurrency`) and `hermes_cli/config_defaults.py` (`mcp.discovery_concurrency`) on Hermes `main` at `e31c674f`.
>
> `_DISCOVERY_CONNECT_CONCURRENCY` is 4. `mcp.discovery_concurrency` overrides it. `0` means unlimited. A non-integer or a negative value falls back to 4. I did not find a least-outstanding pick or a power-of-two-choices pick in the MCP client.
>
> **Adopted.** The discovery cap of 4. Six upstreams with a 40 ms hold never show more than 4 in flight.
>
> **Adapted.** The pick is this book's pool, from the gateway notes: least outstanding, or the quieter of two indexes. Federation is a map from tool name to server names, in memory. `FakeModel` is the fallback reply. No socket, so `0` as unlimited is not implemented. Outstanding is an `AtomicUsize` that `call` increments and then drops.
>
> **Deferred.** A live upstream connection, and the unlimited setting. Chapter 38 is the service that routes by session id. The pick stays local.

[personal note: a tool that had one backend until the second server offered the same name]

## What this chapter decides

`pick` with `LeastOutstanding` returns the index of the smallest `outstanding` counter. Ties keep the earlier backend. `PowerOfTwo { a, b }` reduces both indexes modulo the pool length and returns the quieter one. An empty pool returns `None`.

`call` picks, adds one to that counter, formats `reply via server`, and subtracts one. The counter is back where it started when `call` returns. A caller that wanted to see in-flight load reads `outstanding` on a backend it has not called through, or holds the add itself. This `call` is the finished request.

`federate` builds a map. `pool_for` turns one tool's servers into a pool of backends whose reply is the tool name and whose counters start at 0.

`discover` starts `n` threads and holds each one inside a gate of 4. The return is the max in flight.

`demo` federates `echo` on `alpha` and `beta` and returns `pick: echo via alpha`. No key.

## Rust: the counter is an atomic

`outstanding` is an `AtomicUsize`. `pick` only loads it. Two picks can run together. `call` uses `fetch_add` and `fetch_sub`, so a finished call does not leave the counter raised. The bench calls `pick`, not `call`.

The discovery gate is a mutex and a condvar. `enter` waits while four connects are inside. The bench does not take that lock.

## Tests

Five tests. Outstanding counts 3, 1, 4, 2 pick index 1, and the reply is `echo via b`. After `call` the pick is still 1. Power of two on indexes 0 and 1 picks 1. Indexes 2 and 0 pick 2, because 4 is less than 5. An empty pool picks nothing. `echo` federates to `alpha` then `beta`. The pool length is 2, and least outstanding returns `echo via alpha`. A missing tool is an empty pool. Six connects with a 40 ms hold see a max of 4. The demo line matches.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p gw-balancing
```

```text
Starting 5 tests across 3 binaries
     Summary [   0.085s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p gw-balancing
```

```text
pick: echo via alpha
```

The binary picks between two scripted upstreams. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `pick_least` | 3.739 ns | a local pick among a pool | choose the quietest of four backends |

`pick_least` builds four backends before the bench, with outstanding counts 3, 1, 4, and 2. The timed closure is `b.iter`. Inside: `pick(Policy::LeastOutstanding)` passed through `black_box`, then `black_box` of the index. Each sample loads four atomics and returns index 1. `call` is not used, so the counters do not change. No thread is started. `discover` is not called. No socket is opened. `FakeModel` is not constructed.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `35`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `pick: echo via alpha`.

## Pitfalls

`call` drops the outstanding count before it returns. A metric that samples after `call` sees the idle number. The in-flight count is visible only while `call` is inside the add, which for a scripted reply is the format of one string.

Power of two is only as random as the indexes the caller passes. This crate does not draw them. A caller that always passes `0` and `1` never looks at the rest of the pool.

Discovery with a zero hold can show a max below 4, because the threads may not overlap. The test holds for 40 ms so they do. The cap is still 4 either way. It does not raise the count above 4.

Hermes treats `discovery_concurrency: 0` as unlimited. This gate does not. Four is the constant.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Pick.** Least outstanding, or the quieter of two indexes. The choice reads atomics. It does not call the upstream.

**Discovery.** At most 4 connects in flight. That is `_DISCOVERY_CONNECT_CONCURRENCY`. The unlimited override is not implemented.

**Federation.** A tool's pool is the list of servers that offer it, in the order the upstreams were given. A missing name is an empty pool, not a guess.

**Left for later.** Live sockets, session routing, and OAuth are chapter 38. E1 and E2 are still absent. The scripted reply is not redacted.

## Chapter summary

- **Pool.** One tool, several backends, outstanding counts.
- **Pick.** Least outstanding, or the quieter of two indexes.
- **Federation.** A map from tool name to server names. No socket.
- **Discovery.** Four at a time.
- **Time.** One least-outstanding pick measured **3.739 ns**.

[personal note: a choice that was four atomic loads]

## What's next

Chapter 36 is profiles. Several personas in one config, each with its own secrets, and those secrets stay out of the prompt.
