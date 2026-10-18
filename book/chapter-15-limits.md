# Chapter 15: Limits and Breakers

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 14 names a failure and decides retry or fallback inside the harness. The tool call still arrives at a backend with no limit of its own. This chapter is the gateway. Each tool accepts 32 calls and runs 8. A retry, and a hedge, run only when that tool is marked idempotent. Three failures open a breaker, and the next call does not reach the backend. `web_search` and `delegate_task` stop at 50 for the turn.

The step crate is `steps/ch15-limits`. The book number is 15. The package is `ch15-limits`. Chapter 21's process crate wraps `LimitedGateway`. A new gateway still has an empty catalog.

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

[personal note: a retry you would have allowed, and why the tool was not marked idempotent]

## What this chapter decides

`LimitedGateway::new` wraps the chapter 8 session gateway. `catalog`, `open_session`, and `session_tools` are the same methods the smoke test already calls. The catalog is empty until something loads it.

`set_idempotent("echo", true)` marks one tool. The default is false, so a write does not retry.

`dispatch` checks the breaker first. An open breaker returns `CallError::OpenBreaker` and does not call the backend. Then it checks the cap. The 51st `web_search` returns the search sentence and does not call the backend. The same is true for `delegate_task`, except `{"action":"list"}` (and `steer`, and `stop`) still run.

A transport failure on an idempotent tool calls the backend a second time. A 429 or a 503 does not, even when the tool is idempotent. A status such as 500 does retry once, and only then. A non-idempotent tool never gets that second call.

`CallOpts { hedge: true }` on an idempotent tool always makes the second call, and the first `Ok` wins. The same option on a write makes one call.

Three `503` results open the breaker for that tool name. A different tool is not open. A success in the middle clears the count, so two failures, one success, and two more failures stay closed.

`hold_accept` takes one of 32 accept permits. The 33rd is `Saturated`. `run_with_timeout` takes one accept permit and one execute permit, waits 30 milliseconds against a 5 second sleep, and drops both. The test finishes in under two seconds, and both counts are full again.

The demo marks `echo` idempotent. The script fails once with a transport error, then returns `ok`. Three `503`s on `search` open the breaker. The next `search` prints `breaker: open`.

## Rust: the permit is dropped when the timer wins

Each tool has two `tokio::sync::Semaphore`s. Accept is 32. Execute is 8. `try_acquire_owned` either returns an `OwnedSemaphorePermit` or refuses the call. The permit is not tied to a reference on `self`. `run_with_timeout` moves both permits into the function, races the body against `tokio::time::timeout`, and `drop`s them on both arms. The sleep is cancelled because its future is dropped. The side effect a tool already started is not undone.

The caps are `AtomicU32`. The breaker map is a `Mutex`. The demo does not wait on the semaphore. `dispatch` is the policy. The timeout test is the only place the permits are held across a wait.

## Tests

Eight tests. An idempotent transport error retries once. A write does not retry, and neither do 429 or 503. Three failures open the breaker, and the fourth call does not touch the script. A success clears the count. Fifty web searches run, and the next one is the block sentence. A batch of three plus the rest of the cap blocks `delegate_task`, and `{"action":"list"}` still runs. A hedge on `echo` calls twice and keeps the first body. A hedge on a write calls once. Thirty-two accept permits fill the gate. A timeout returns both permits.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch15-limits
```

```text
Starting 8 tests across 3 binaries
     Summary [   0.038s] 8 tests run: 8 passed, 0 skipped
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
| `open_breaker` | 30.09 ns | under 1 µs | a call that returns before the backend |
| `permit_release` | 58.96 ns | under 1 µs | take one execute permit and drop it |

`open_breaker` records three 503s before the bench. The timed closure is `b.iter`. Inside: `dispatch` of `search` with the tool name passed through `black_box`. The breaker is already open, so the script is not called. Building the gateway and opening the breaker are excluded.

`permit_release` builds the gateway before the bench. Each sample calls `roundtrip_execute`, which `try_acquire_owned`s one permit from the execute semaphore of 8 and drops it. The accept semaphore is not taken. There is no sleep and no backend.

The timeout is not a bench. The test above is the measurement of the cancel: 30 milliseconds against a 5 second sleep, both permit counts full, elapsed under 2 seconds.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38993207081](https://github.com/agnathan/rust-agent-harness/actions/runs/38993207081), section `15`.

### Milestone demo

The `cargo run` above. The result is `ok`. The breaker is open.

## Pitfalls

A 429 on a tool you marked idempotent still does not retry. Idempotent means a transport error or a status other than 429 or 503 may run once more. It does not mean every failure is safe to repeat.

The breaker stays open. This chapter has no half-open probe. The next call is refused until the process is new.

A hedge spends the second call even when the first one already succeeded. That is the race you asked for. Do not turn it on for a write. The flag is ignored for a non-idempotent tool, which is the safe default, and it is easy to miss in a log that only shows the body you kept.

The queue is the accept bound of 32. The 33rd call is refused. Nothing parks past that bound.

`delegate` and `delegate_task` are different names. Chapter 9's loop still counts `delegate`. This gateway counts `delegate_task`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L4.

**I3.** `web_search` stops at 50. `delegate_task` stops at 50. A batch counts as `len(tasks)`. `list`, `steer`, and `stop` do not count and still run. The block strings are the ones in `agent/tool_guardrails.py`.

**Timeout.** `run_with_timeout` drops the accept permit and the execute permit. The 5 second sleep does not finish.

**Retry.** A second call happens only when the tool is idempotent, and only for a transport error or a status other than 429 or 503.

**Backpressure.** 429 and 503 do not retry. They count toward the breaker.

**Hedge.** The second call is started only when the tool is idempotent.

**Breaker.** Three failed calls open it. The next call does not reach the backend.

**Left for later.** The client reconnect cooldown (30 seconds, doubling to 10 minutes) is not this breaker. Replica routing is chapter 35. The socket is chapter 38. Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Permits.** 32 accepted, 8 running, per tool. A timeout gives both back.
- **Retry.** Idempotent only. 429 and 503 wait outside.
- **Breaker.** Three failures, then the backend is not called.
- **Caps.** 50 web searches. 50 subagent spawns. A list action still runs.
- **Time.** The open-breaker check measured **30.09 ns**. The permit round trip measured **58.96 ns**.

[personal note: a breaker that opened on a tool you still wanted to retry]

## What's next

Chapter 16 is the provider. The same turn can build an Anthropic body, a Bedrock body, or an OpenRouter route, rotate a key without building a new client, and leave the side model alone unless the turn asked for a summary.
