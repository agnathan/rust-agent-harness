# Chapter 5: A Minimal MCP Server

The client from the last chapter can list tools and call them. Someone still has to hold the list and run the body, and if that someone is the harness, every agent binary is also the catalog. A change to who gets in would be a change to the prompt builder. This chapter is the other program, small enough to be a server and nothing else. The step crate is `steps/ch05-minimal-server`, package `ch05-minimal-server`, and the types worth knowing up front are `Gateway`, `ToolCard`, `SessionView`, `CallGate`, and `GatewayError`.

What it offers is a fixed list, one real tool named `echo`, and a gate of 32 accepted calls with 8 running, and `Gateway::new` starts empty. A second registration of the same name is `GatewayError::Duplicate`, the same refusal chapter 3 already uses. There is no per-session filter yet, and an unknown name is refused before it takes a slot. The interesting part is the gate, and the way it can stall itself, which the pitfalls walk through once the permits are on the page.

Hermes's own `mcp_serve.py` is a small stdio server with a fixed list, separate from the agent loop that calls other servers. This chapter keeps the fixed list and the registration order, and it stays in-process: it binds no socket. Streamable HTTP, replicas, and a bearer check are chapter 42; search over a changing catalog is chapter 7. A session-sized view is chapter 8. Limits, retries, and a breaker wrap this gate in chapter 15. The process crate `mcp-gateway` will re-export the finished server, not this first slice, and it will still bind no socket.

A permit is a value. Dropping it gives the slot back, including when the body returns `BadArgs`, which is why the gate can be two semaphores instead of a counter someone has to remember to decrement. `cargo run -p ch05-minimal-server` runs the demo, and nothing listens on a port.

I expected the interesting part to be a socket, because the word server usually means something is listening. The file has no listener. `Gateway` is a struct in this process, `register` pushes a card onto a `Vec`, and `call` runs a closure. The harness from chapters 1 through 4 stays a different crate. It will talk to a catalog later. This crate stays the catalog, and it grows neither a prompt, nor a memory file, nor a skill. That split is the one the rest of the server chapters keep. A continuously improving harness still needs a place that can say which tools exist. Putting that place inside the agent would make every lesson also a change to the tool list, and the other way around.

The types are small, and it helps to know what each one is for before the gate gets all the attention. `ToolCard` is a name and a description. `SessionView` is a session id plus a clone of that vec. `session_tools` builds one for any id you pass, and the comment on the function says the list order is the registration order. Two sessions, two views, one vec of cards underneath. `GatewayError` is the refusal: `Duplicate` when the name is already there, `Unknown` when `call` cannot find it, `Saturated` when the accept pool is empty, `BadArgs` when `echo` has no `text` string. `CallGate` is the two semaphores. `Gateway` owns the vec and the gate.

`register` walks the vec. The first time a name appears, it pushes the card and returns `Ok`. The second time it returns `Duplicate` and leaves the vec alone, so the first description wins. I expected a second `echo` to overwrite, because a registry that "updates" a tool is a familiar shape. Chapter 3 already refused that, and this server uses the same refusal. The first card stays. A later reload, in chapter 7, will make the same choice for a whole snapshot.

Nothing in this chapter is the learning loop. There is no score, no skill, and no second attempt. What it protects is the other half of the promise from the introduction: the harness can keep what it learned only if the catalog it calls is a program with its own rules. The rules here are the fixed list, the duplicate refusal, and the two bounds.

**In this chapter**

- What this chapter decides
- The gate
- Tests
- Build, run, and benchmark
- Pitfalls
- Guardrails in this chapter
- Chapter summary
- What's next

| Section | Step | What it adds |
|---|---|---|
| 5 | `steps/ch05-minimal-server` | a fixed tool list, `echo`, and a gate of 32 accepted calls with 8 running |

> **How Hermes does it**
>
> I read `mcp_serve.py` on current Hermes `main`. `create_mcp_server` builds an MCP server named `hermes` and registers a fixed list. The comment above `_TOOL_NAMES` says registration order is `list_tools` order. The names are `conversations_list`, `conversation_get`, `messages_read`, `attachments_fetch`, `events_poll`, `events_wait`, `messages_send`, `channels_list`, `permissions_list_open`, `permissions_respond`. `run_mcp_server` serves that list on stdio. It is a messaging bridge. It does not search a catalog and it does not federate other servers.
>
> **Adopted.** A server has a fixed list, and the list order is the registration order. Every session sees that same list. `echo` is this chapter's one real tool, the way the bridge's handlers are ordinary functions behind the protocol.
>
> **Adapted.** Hermes `register()` in `tools/registry.py` lets a same-name, same-toolset duplicate overwrite. This server returns `GatewayError::Duplicate` and keeps the first tool, which is the rule chapter 3 already uses. Hermes serves stdio. This chapter stays in-process. The book's budget for this row is 32 accepted calls and 8 executing. Those two numbers are not in `mcp_serve.py`. Eight is the same bound as `_MAX_TOOL_WORKERS` in `agent/tool_executor.py`. Thirty-two is the number of accept permits.
>
> **Deferred.** A socket, streamable HTTP, and OAuth 2.1 as a resource server wait for chapter 42. Server-side search waits for chapter 7. Per-session filtering waits for chapter 8.

## What this chapter decides

`Gateway::new` starts empty, `register` appends a name and a description, and `session_tools` returns that list, in registration order, for every session id. There is no per-session filter yet.

`call` looks up the name before it takes a permit, so an unknown name never occupies a slot. `echo` reads the string field `text` and returns it. Any other registered name returns its description, and a missing `text` on `echo` is `BadArgs`.

The gate is two semaphores, `ACCEPT_BOUND` 32 and `EXECUTE_BOUND` 8. `CallGate::run` tries to take an accept permit and refuses when none are free, then waits for an execute permit. The accept permit is held through that wait and through the tool body, so the 33rd call fails instead of joining a longer queue. The test sleeps 30 ms inside 32 tasks, checks that the peak inside the body is at most 8 and at least 2, and checks that one more `run` returns `Saturated`.

## The gate

`try_acquire_owned` is the refuse path; `acquire_owned` is the wait path. Both permits are dropped when `run` returns, including when the tool body returns `BadArgs`.

```rust
let _accept = self.acquire_accept()?;
let _execute = self.execute.clone().acquire_owned().await.expect("execute semaphore");
Ok(body().await)
```

> **Rust sidebar: a permit is a value**
>
> A `Semaphore` starts with N permits. `acquire_owned` returns a permit value, and dropping that value gives the permit back. Holding it across an `.await` is what caps the overlap. A hand-counted integer would lie the first time a task panicked between the increment and the decrement. A `Mutex` held across the tool body would pin a worker for the whole call. The permit doesn't.

## One echo, one unknown, one extra

Take the demo's call and walk it without the binary. `register("echo", ...)` finds an empty vec, pushes the card, and returns. `session_tools` clones that one card into a view. The demo prints `tools: echo` from that view. Then `call("echo", {"text": "hello"})` looks for the name before it touches the gate. The name is there, so `run` tries the accept semaphore. A free permit exists, because nothing else is inside. It then waits on the execute semaphore, which also has room. The body reads `text`, finds the string `hello`, and returns it. Both permits drop as `run` returns. The demo prints `echo: hello` and `accept_bound: 32`.

Now change the name to `nope`, which is the unknown the echo test asks for. The `any` check fails. `call` returns `Unknown` and never enters `run`. I expected an unknown name to occupy a slot until the error came back, the way a queue might hold a request it cannot serve. The check is before the permit, so a burst of bad names leaves `Saturated` alone. The bound is for work the server accepted. The pitfalls say the same thing, and the order in `call` is why.

A second `register` of `echo` returns `Duplicate` with that name and pushes nothing. The vec length stays 1. `call` on the first card still returns `hello`. The refusal and the successful call are the same test, `echo_returns_text_and_a_duplicate_is_refused`, because the point is that the failure leaves the tool that was already there.

`BadArgs` is the path that does take the permits and then fails. `echo` with no `text` field, or with a `text` that is not a string, goes through `run`. The body returns the error. The accept permit and the execute permit are both dropped on the way out, including on that error, because they are values in local bindings. A counter you increment on the way in and decrement on the success path would leak the slot here. The permit cannot forget.

The overlap test is the other shape. Thirty-two tasks enter `run`, each sleeps 30 ms inside the body, and the peak observed inside the body is at most 8 and at least 2. The accept pool let all 32 in. The execute pool let 8 run. A thirty-third `run` returns `Saturated` and stays out of the body. That is the whole gate. There is no queue past 32, and there is no retry. Chapter 15 is where a per-tool limit, an idempotent retry, and a breaker wrap this idea. This chapter only refuses.

Hold the accept permit in your head while the task waits for an execute permit. That overlap is the deadlock the pitfalls describe, and it is worth one more pass because the code looks innocent. Thirty-two tasks have each taken an accept permit and are parked on `acquire_owned` for an execute permit. Eight bodies are running. If one of those bodies calls back into the same gate, it needs an accept permit, the pool is full, and it waits. It will wait until some body finishes and drops an execute permit, but the bodies that could finish are the ones blocked on the inner call. The tools in this chapter return a string and stop, so the tests never trip it. A later tool that calls another tool on this same gate will trip it the first time the outer calls fill the accept pool. Seeing the shape now is cheaper than debugging it when the tool is a search that calls a catalog.

`session_tools` ignores the session id except to store it on the view. Pass `"a"` or `"b"` and you get the same cards. I expected a session id to filter the list, because chapter 8 will do exactly that. This chapter's comment says every session sees the same fixed list, and the function clones `self.tools` with no further test. The filter is a later chapter on purpose. If it were here, the gate tests would be about visibility instead of about permits.

## Tests

`a_new_gateway_lists_nothing` checks the empty catalog and an empty session view. `echo_returns_text_and_a_duplicate_is_refused` registers `echo`, refuses the second `echo`, returns `hello`, and returns `Unknown` for `nope`. `thirty_two_are_accepted_and_eight_run` is the overlap test above.

## Build, run, and benchmark

From the repository root.

```bash
cargo nextest run --locked -p ch05-minimal-server
```

```text
Starting 3 tests across 3 binaries
     Summary [   0.129s] 3 tests run: 3 passed, 0 skipped
```

The three tests are the empty gateway, echo plus the duplicate, and the 32/8 gate. The gate test is the slow one, about 128 ms, because the tasks sleep.

```bash
cargo run -p ch05-minimal-server
```

```text
tools: echo
echo: hello
accept_bound: 32
```

The binary registers `echo` and calls it, and there is no model in this process, so there is no API key and no `--mock` flag.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `echo_through_gate` | 45.53 ns | the permit round trip | one `CallGate::run` that returns `hello`, no sleep |
| `eight_concurrent_sleeps` | 2.129 ms | eight 1 ms sleeps on the execute bound | eight tasks, each sleeping 1 ms inside `run` |

`echo_through_gate` builds the runtime (4 workers) and the gate outside the sample. The timed closure takes one accept permit, waits for an execute permit, and returns the string `hello`. `eight_concurrent_sleeps` spawns eight tasks inside the sample. Each sleeps 1 ms while holding an execute permit; setup of the runtime is excluded. The 2.129 ms number is one wave of eight 1 ms sleeps plus scheduling.

The means are `mean.point_estimate` from [run 38863938100](https://github.com/agnathan/rust-agent-harness/actions/runs/38863938100), section `5`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

Picture thirty-two calls that have each taken an accept permit and are waiting for an execute permit, while the eight bodies that are running try to make one more call into the same gate. The accept pool is full, so that inner call can't get a permit, the bodies never finish, and the execute permits never come back. The accept permit is held while the task waits for an execute permit, which is how a tool that calls back in can fill all 32 slots and then deadlock. The tools in this chapter return a string and stop, so the tests never trip it, but the shape is still worth seeing before a later tool starts calling another tool.

`call` checks the name before `run`, so a burst of unknown names leaves `Saturated` alone. The bound is for work the server accepted.

`Semaphore::new(0)` would refuse every call. The constants are 32 and 8. A test that constructs `CallGate::new(0, 8)` is testing a different server.

## Guardrails in this chapter

Layer L7, only the concurrency cap, plus the duplicate-name refusal from chapter 3. The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Accept and execute bounds.** `ACCEPT_BOUND` is 32 and `EXECUTE_BOUND` is 8. The 33rd `run` returns `Saturated` and does not enter the body. Hermes does not put this gate on `mcp_serve.py`. Chapter 15 is where per-tool queues, breakers, and 429/503 backpressure land. This chapter refuses. It does not queue past 32 and it does not retry.

**Duplicate names.** `register` returns `Duplicate` and leaves the first tool in place.

This process does not authenticate a caller. Default-deny and pairing are chapter 39 for the chat gateway, and OAuth for this MCP process is chapter 42. It does not scrub secrets (E1 and E2, `agent/redact.py`). It does not sandbox the tool body. Chapter 13 is that boundary, and Hermes leaves the sandbox off by default (`terminal.backend: "local"`, D1). The gate is a concurrency limit. It is not a security boundary.

The bench numbers are easy to over-read, so the setup is worth a paragraph of its own. `echo_through_gate` builds the runtime, four workers, and the gate outside the sample. The timed closure takes one accept permit, waits for an execute permit, and returns `hello`. 45.53 ns is that round trip, with no sleep. `eight_concurrent_sleeps` spawns eight tasks inside the sample. Each sleeps 1 ms while it holds an execute permit. The runtime setup is excluded. 2.129 ms is one wave of those sleeps plus the scheduling, which is why it sits a little above the 1 ms the bodies asked for. Both means are `mean.point_estimate` from the Benchmarks run named in the table, section 5. The appendix says how to read a Criterion `time:` line. I am not adding a third number here. The demo's three printed lines are the behavior. The table is the time.

Eight as the execute bound is the same width chapter 3 uses for in-flight tool work, and the Hermes box ties it to `_MAX_TOOL_WORKERS`. Thirty-two is this book's accept budget. `mcp_serve.py` doesn't carry that number. The consequence is the one the overlap test checks: 32 may be inside, 8 may run, and the next `run` is `Saturated`. A harness that learns a skill and then calls a tool still has to pass through some bound like this, or a reflection fork and a user turn will pile calls onto the same catalog with no backpressure. The bound in this chapter is local and in-process. Chapter 15 puts a finer one on individual tools. The idea starts here, with two semaphores and a test that sleeps.

`cargo run` from the repository root is enough. The binary registers `echo`, calls it with `hello`, and prints the three lines. There is no model, so there is no API key and no `--mock` flag. If you go looking for a port, you will not find one, and that absence is the design. The process crate `mcp-gateway` re-exports a later copy of this server. It still binds no socket. Streamable HTTP and a bearer check are chapter 42. Until then, "server" in this track means a type another crate can call, with a list and a gate, living outside the harness on purpose.

## Chapter summary

- **List.** A new gateway is empty. `register` appends. `session_tools` returns that list for every session.
- **Call.** `echo` returns `text`. A second `echo` is `Duplicate`. An unknown name is `Unknown` and takes no permit.
- **Gate.** 32 calls may be inside. 8 may run the body. One more is `Saturated`. A permit round trip measured **45.53 ns**. Eight 1 ms sleeps measured **2.129 ms**.

Hold the three refusals next to each other before you leave, because they are easy to merge into one "error" and the file keeps them apart. `Duplicate` happens at `register`, before any call, and the first card stays. `Unknown` happens at the start of `call`, before a permit, so a bad name cannot fill the gate. `Saturated` happens inside `run`, when `try_acquire_owned` finds the accept pool empty, and the body never starts. `BadArgs` is the odd one. It happens inside the body, after both permits were taken, and the permits still come back because dropping them is what `run` does on every return. If you only remember `Saturated`, you will misread a missing `text` field as a capacity problem. The test names keep the cases separate. The empty gateway is one test. Echo plus the duplicate plus the unknown is the second. The 32/8 overlap, including the extra `run` that must be `Saturated`, is the third. Nextest's summary in this chapter is three tests, three passed. The gate test is the slow one, about 128 ms, because the tasks sleep 30 ms.

That sleep is also why the peak check asks for at least 2 and at most 8. At most 8 is the execute bound. At least 2 is the evidence that the tasks actually overlapped, rather than running one after another while the assert still passed. A gate that accidentally serialized every call would still show a peak of 1, and the test would fail the "at least 2" half. I expected a concurrency test to only check the ceiling. The floor is what proves the bound is a cap on overlap and not an accident of scheduling on an empty machine.

`SessionView` is worth one last look, because the name sounds like chapter 8 and the function is a clone of the whole list. It stores the session id you passed and a clone of the whole vec. `"a"` and `"b"` see the same cards, in registration order. The clone is the list the demo prints. It is also the reason a later chapter can add a revealed set without changing `ToolCard`: the view is already a separate value from the gateway's vec. Today that value happens to contain everything. Chapter 8 is where "everything" stops being the default.

## What's next

Chapter 6 is the client half: schema cost, BM25, and a lazy prompt. Chapter 7 puts that index in this process and reloads it by swapping an `Arc`. The gate you just built stays under both. Chapter 8 will filter what a session may call, and it will still take an accept permit and an execute permit on the way through `catalog.call`. The deadlock shape in the pitfalls is the reason that later call drops its session lock first. You have seen the permits. The next two server chapters add the index and the filter. The harness loop waits until chapter 9, and it will call tools through a scheduler that already knows a bound of 8. Same width as this execute bound, and it lives in the harness crate rather than in this one. Keep the two eights distinct as you read.
