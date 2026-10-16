# Chapter 5: A Minimal MCP Server

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 4 can talk to an MCP server. It cannot be one. The harness decides what enters a prompt. Something else has to hold the tool list and run the call. This chapter is that process, in the smallest form that is still a server: a fixed list, a session view of that list, and a gate on how many calls may be inside.

The step crate is `steps/gw-minimal-server`. The package is `gw-minimal-mcp-gateway`. It binds no socket. Streamable HTTP, replicas, and OAuth are chapter 38. The catalog search is chapter 7.

| Section | Step | What it adds |
|---|---|---|
| 5 | `steps/gw-minimal-server` | a fixed tool list, `echo`, and a gate of 32 accepted calls with 8 running |

> **How Hermes does it**
>
> I read `mcp_serve.py` on current Hermes `main`. `create_mcp_server` builds an MCP server named `hermes` and registers a fixed list. The comment above `_TOOL_NAMES` says registration order is `list_tools` order. The names are `conversations_list`, `conversation_get`, `messages_read`, `attachments_fetch`, `events_poll`, `events_wait`, `messages_send`, `channels_list`, `permissions_list_open`, `permissions_respond`. `run_mcp_server` serves that list on stdio. It is a messaging bridge. It does not search a catalog and it does not federate other servers.
>
> **Adopted.** A server has a fixed list, and the list order is the registration order. Every session sees that same list. `echo` is this chapter's one real tool, the way the bridge's handlers are ordinary functions behind the protocol.
>
> **Adapted.** Hermes `register()` in `tools/registry.py` lets a same-name, same-toolset duplicate overwrite. This server returns `GatewayError::Duplicate` and keeps the first tool, which is the rule chapter 3 already uses. Hermes serves stdio. This chapter stays in-process. The book's budget for this row is 32 accepted calls and 8 executing. Those two numbers are not in `mcp_serve.py`. Eight is the same bound as `_MAX_TOOL_WORKERS` in `agent/tool_executor.py`. Thirty-two is the number of accept permits.
>
> **Deferred.** A socket, streamable HTTP, and OAuth 2.1 as a resource server wait for chapter 38. Server-side search waits for chapter 7. Per-session filtering waits for chapter 8.

[personal note: the moment a client and a server stopped being the same process in your head]

## What this chapter decides

`Gateway::new` starts empty. `register` appends a name and a description. `session_tools` returns that list, in registration order, for every session id. There is no per-session filter yet.

`call` looks up the name before it takes a permit. An unknown name does not occupy a slot. `echo` reads the string field `text` and returns it. Any other registered name returns its description. A missing `text` on `echo` is `BadArgs`.

The gate is two semaphores. `ACCEPT_BOUND` is 32. `EXECUTE_BOUND` is 8. `CallGate::run` tries to take an accept permit and refuses when none are free. It then waits for an execute permit. The accept permit is held through that wait and through the tool body, so the 33rd call fails instead of joining a longer queue. The test sleeps 30 ms inside 32 tasks, checks that the peak inside the body is at most 8 and at least 2, and checks that one more `run` returns `Saturated`.

## The gate

`try_acquire_owned` is the refuse path. `acquire_owned` is the wait path. Both permits are dropped when `run` returns, including when the tool body returns `BadArgs`.

```rust
let _accept = self.acquire_accept()?;
let _execute = self.execute.clone().acquire_owned().await.expect("execute semaphore");
Ok(body().await)
```

### Rust: a permit is a value

A `Semaphore` starts with N permits. `acquire_owned` returns a permit value. Dropping that value gives the permit back. Holding it across an `.await` is what caps the overlap. You do not increment a counter by hand, and you do not hold a `Mutex` across the tool body. The mutex would pin a worker. The permit does not.

## Tests

`a_new_gateway_lists_nothing` checks the empty catalog and an empty session view. `echo_returns_text_and_a_duplicate_is_refused` registers `echo`, refuses the second `echo`, returns `hello`, and returns `Unknown` for `nope`. `thirty_two_are_accepted_and_eight_run` is the overlap test above.

## Build, run, and benchmark

From the repository root.

```bash
cargo nextest run --locked -p gw-minimal-mcp-gateway
```

```text
Starting 3 tests across 3 binaries
     Summary [   0.129s] 3 tests run: 3 passed, 0 skipped
```

The three tests are the empty gateway, echo plus the duplicate, and the 32/8 gate. The gate test is the slow one, about 128 ms, because the tasks sleep.

```bash
cargo run -p gw-minimal-mcp-gateway
```

```text
tools: echo
echo: hello
accept_bound: 32
```

There is no model in this process, so there is no API key and no `--mock` flag. The binary registers `echo` and calls it.

The bench is `benches/gate.rs`. Criterion warm-up is 1 second, measurement is 2 seconds, and the sample size is 10. The mean is `estimates.json` `mean.point_estimate`.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `echo_through_gate` | 79.93 ns | the permit round trip | one `CallGate::run` that returns `hello`, no sleep |
| `eight_concurrent_sleeps` | 2.307 ms | eight 1 ms sleeps on the execute bound | eight tasks, each sleeping 1 ms inside `run` |

`echo_through_gate` builds the runtime (4 workers) and the gate outside the sample. The timed closure takes one accept permit, waits for an execute permit, and returns the string `hello`. `eight_concurrent_sleeps` spawns eight tasks inside the sample. Each sleeps 1 ms while holding an execute permit. Setup of the runtime is excluded. Nothing is a model. The 2.307 ms number is one wave of eight 1 ms sleeps plus scheduling, on this machine.

CI runs the same bench on Ubuntu. The workflow id is `gw5`.

### Milestone demo

The `cargo run` above is the demo. I did not open a socket and I did not call OpenRouter.

## Pitfalls

The accept permit is held while the task waits for an execute permit. A tool that waits on another call into the same gate can fill all 32 slots and then deadlock. This chapter's tools do not call back in.

`call` checks the name before `run`. A burst of unknown names does not trip `Saturated`. That is deliberate. The bound is for work the server accepted.

`Semaphore::new(0)` would refuse every call. The constants are 32 and 8. A test that constructs `CallGate::new(0, 8)` is testing a different server.

## Guardrails in this chapter

Layer L7, only the concurrency cap, plus the duplicate-name refusal from chapter 3. The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Accept and execute bounds.** `ACCEPT_BOUND` is 32 and `EXECUTE_BOUND` is 8. The 33rd `run` returns `Saturated` and does not enter the body. Hermes does not put this gate on `mcp_serve.py`. Chapter 15 is where per-tool queues, breakers, and 429/503 backpressure land. This chapter refuses. It does not queue past 32 and it does not retry.

**Duplicate names.** `register` returns `Duplicate` and leaves the first tool in place.

This process does not authenticate a caller. Default-deny and pairing are chapter 37 for the chat gateway, and OAuth for this MCP process is chapter 38. It does not scrub secrets (E1 and E2, `agent/redact.py`). It does not sandbox the tool body. Chapter 13 is that boundary, and Hermes leaves the sandbox off by default (`terminal.backend: "local"`, D1). The gate is a concurrency limit. It is not a security boundary.

## Chapter summary

- **List.** A new gateway is empty. `register` appends. `session_tools` returns that list for every session.
- **Call.** `echo` returns `text`. A second `echo` is `Duplicate`. An unknown name is `Unknown` and takes no permit.
- **Gate.** 32 calls may be inside. 8 may run the body. One more is `Saturated`. A permit round trip measured **79.93 ns**. Eight 1 ms sleeps measured **2.307 ms**.

[personal note: what was harder, the semaphore or deciding the server is not the harness]

## What's next

Chapter 6 is already written. It is the client half: schema cost, BM25, and a lazy prompt. Chapter 7 puts that index in this process and reloads it by swapping an `Arc`.
