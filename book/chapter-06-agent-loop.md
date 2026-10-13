# Chapter 6: The Agent Loop

*Draft. Bracketed notes like [personal note] are where my own story goes.*

The pieces from the earlier chapters do not yet take a turn. Chapter 1 can ask a model. Chapter 2 can hold the messages and a session inbox. Chapter 3 can run tools. Chapter 4 can run those tools in another process. Chapter 5 can keep a huge catalog off the prompt. This chapter is the loop that uses them: the model returns text or tool calls, the dispatcher runs the calls, the results become messages, and the model is asked again.

One check sits in front of every model call. A cancellation flag, or a budget of iterations, tokens, time, or cost, means that call does not start. The mock is scripted. It does not invent usage, and it does not call OpenRouter.

This chapter is one step, `steps/ch06-agent-loop`, the way chapter 2 was one step. The loop, the gate, and the saved eval set are one program. Splitting them into copies would hide the fact that the gate wraps the same function the eval set calls.

| Section | Step | What it adds |
|---|---|---|
| 6.1 | `steps/ch06-agent-loop` | the loop: model, native and MCP tools, results, repeat |
| 6.2 | the same step | a cancellation flag and a budget, checked before every model call |
| 6.3 | the same step | a saved eval set the later chapters re-run on the mock |

The crate still sets `rust-version = "1.88"` because it includes the chapter 4 client. `rmcp` 3.5.1 needs that compiler.

> **How Hermes does it**
>
> `agent/conversation_loop.py` is `run_conversation`. The once-per-turn prologue lives in `build_turn_context` in `agent/turn_context.py`: sanitize the user message, restore or build the system prompt, run preflight compression, the `pre_llm_call` hook, an external-memory prefetch, and the persistence that lets a crashed session resume. Then the loop repeats.
>
> Each iteration looks at `agent._interrupt_requested` and at `iteration_budget`. The loop condition also stops when `api_call_count` reaches `max_iterations` or the iteration budget is spent. The model call itself is interruptible (`_interruptible_api_call`, and the streaming twin). An interrupt breaks out of the tool loop before the next model request.
>
> When the model returns several tool calls, Hermes runs them on a thread pool. The developer guide describes a `ThreadPoolExecutor`, with results put back in the original order. A single call stays on the calling thread. A batch that includes an interactive tool such as `clarify` stays sequential. The change that added the pool capped it at 8 workers (`_MAX_TOOL_WORKERS`) and listed tools that must not overlap (`_NEVER_PARALLEL_TOOLS`).
>
> This chapter's prologue is smaller. The inbox pop is the whole setup. There is no compression and no hook yet. The gate is `check_gate`: the flag, then iterations, tokens, cost, and time. Tool batches go through the chapter 3 dispatcher, bound 8. Reads overlap. A write runs alone. A single read still takes a permit. I did not special-case it onto the calling task. Results come back in input order, which the dispatcher already guaranteed.

[personal note: the first time a loop kept calling the model after you had meant to stop it]

## What this chapter decides

The loop does not know whether a tool is native or MCP. Both are `Arc<dyn Tool>`. The tests register `EchoTool` and, separately, the chapter 4 fixture's `echo` over stdio and over streamable HTTP. The same `run_loop` handles all three.

The provider in this chapter is scripted. A `ScriptedReply` is text, or tool calls, and an optional `Usage`. The mock leaves usage empty unless a test sets it. That is the same rule as chapter 1: the mock does not invent a bill.

The gate runs before `complete`, not after. A reply that has already arrived is part of the transcript. The budget can only refuse the next request. A final answer with no tool calls returns `Finished` even if that reply's token count is over the cap, because there is no next request. The tests that expect `Budget` use a tool call as the first reply, so the loop has a second model call to refuse.

Cancellation is an `Arc<AtomicBool>`. A tool can set it while the dispatcher is still awaiting that tool. The next iteration sees the flag and does not call the provider. The test counts provider calls and expects 1.

The saved eval set is three tasks in the library, not a file of transcripts. `echo-hello` must call `echo` and finish with `hello`. `direct` finishes with `ok` and calls nothing. `eight-reads` fans eight delayed reads out in one turn and records a peak in-flight count. A score fails if that peak is above 8. Later chapters re-run `run_saved_evals`. Chapter 41 is where the set gets a real score report. This chapter only checks that the mock passes.

Streaming still waits until the buffered reply is in hand, then overlaps the safe calls. Parsing a tool call out of a token stream and starting it early is chapter 14. The dispatcher is already the scheduler that chapter will call.

## 6.1 Model, tools, results

`run_loop` takes the message vec, a provider, the registry, the dispatcher, the budget, the flag, and the instant the turn started.

```rust
if let Some(stop) = check_gate(cancel, budget, &usage, started) {
    return TurnOutcome { events, usage, stop };
}
let batch = provider.complete(messages).await;
```

The batch is the chapter 1 `Event` list: text, tool calls, usage if the script included it, then `Done`. The assistant message on `Done` carries `tool_calls`. An empty list means the turn is over. A non-empty list becomes `ToolCall` values for the dispatcher. Each output is appended as a `Role::Tool` message with the tool-call id, and the loop asks the model again.

> **Rust sidebar: the flag the tool can see**
> - `CancelFlag` holds `Arc<AtomicBool>`. Cloning it clones the pointer, so the tool and the loop share one flag.
> - The tool stores `true` from inside `call`. The loop loads it at the top of the next iteration.
> - `SeqCst` is more ordering than a flag needs. The test is "the next `complete` did not run", and the atomic makes that obvious.

The session inbox is the chapter 2 queue, bound 64. `submit` enqueues a string and does not call the model. `turn_from_inbox` pops that string, pushes a user message, and runs the loop. The CLI does those two steps for `ask`.

`EchoTool` is the native tool. The fixture's `echo` is the MCP tool. A duplicate name is still `ToolError::Duplicate`, so the demo registers the remote tools and does not also register the native echo.

### Build, run, and benchmark

```bash
cargo nextest run -p ch06-harness-core --locked
cargo run -p ch06-harness-core --example demo --quiet
cargo run -p ch06-harness-cli -- --mock ask "say hello"
cargo bench -p ch06-harness-core --bench loop_iter -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Library, 9 tests. The CLI has no tests of its own. The missing-key path is the same binary.

```text
     Summary [   0.040s] 9 tests run: 9 passed, 0 skipped
```

The example submits `say hello`, the script calls the fixture's `echo`, and the second reply is the text `hello`. The mock emits no usage line.

```text
event: tool-call echo id=call-1 args={"text":"hello"}
event: done
event: text-delta hello
event: done
tool-result: echo hello
stop: Finished
```

One loop iteration with a scripted final answer, and no tool call, is the overhead bench. Eight reads that each sleep 1 ms share the bound. The `time:` lines are Criterion's slopes. The table uses the means from `estimates.json`.

```text
loop_iteration          time:   [259.67 ns 263.54 ns 269.93 ns]
loop_8_reads            time:   [2.3780 ms 2.3995 ms 2.4184 ms]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `loop_iteration` | 263.9 ns | < 1 ms, excluding the model | 4 vCPU, sample size 10, warm-up 1 s, measurement 2 s |
| `loop_8_reads` | 2.392 ms | eight 1 ms reads in one turn, bound 8 | same machine |

2.392 ms is one wave of 1 ms sleeps plus the second scripted reply and the dispatcher. It is not eight milliseconds. The 30 ms test in the same file is the overlap check: eight reads finish in under 200 ms, and the peak in-flight count is greater than 1 and at most 8.

### Milestone demo

`--mock` is the demo. The model replies are the script. The tool is the fixture.

```bash
cargo run -p ch06-harness-cli -- --mock ask "say hello"
```

```text
event: tool-call echo id=call-1 args={"text":"hello"}
event: done
event: text-delta hello
event: done
tool-result: echo hello
```

Without `--mock`, and with `OPENROUTER_API_KEY` unset, the process stops before it opens the fixture and before any model call:

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

`--api-key` overrides the variable for that check. A key that is present still does not place a live OpenRouter call. The process prints `this demo does not place a live OpenRouter call` and exits 1.

## 6.2 The gate

`check_gate` is a plain function. The bench calls it in a loop with a clear flag and an empty budget.

```rust
pub fn check_gate(
    cancel: &CancelFlag,
    budget: &Budget,
    usage: &AgentUsage,
    started: Instant,
) -> Option<StopReason> {
    if cancel.is_cancelled() {
        return Some(StopReason::Cancelled);
    }
    if budget.exceeded(usage, started) {
        return Some(StopReason::Budget);
    }
    None
}
```

`Budget::default` allows 8 iterations and sets no token, cost, or time cap. `exceeded` is true when `usage.calls` has reached `max_iterations`, when `total_tokens` is over `max_tokens`, when a known cost is over `max_cost_usd`, or when `started.elapsed()` is over `max_duration`. A missing cost does not trip the cost cap. The mock's empty usage is why.

The token test's first reply is a tool call with `total_tokens: 100` and `cost: Some(0.25)`. `max_tokens` is 50. The tool runs. The next iteration stops with `Budget`, and `provider.calls()` is 1. The cost test uses the same shape with `max_cost_usd` of 0.1. `0.25` is the cost the earlier chapters already treat as exact. The time test sets `max_duration` to zero and a start instant 5 ms in the past, so the first model call never starts and `provider.calls()` is 0.

```text
gate_check              time:   [2.5840 ns 2.6480 ns 2.7556 ns]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `gate_check` | 2.711 ns | < 1 µs | same machine as 6.1 |

### The gate tests

`cancel_before_the_next_model_call` registers a tool whose `call` sets the flag. The script has a second reply waiting. The outcome is `Cancelled` and the provider was asked once.

`a_token_budget_stops_before_the_next_call` and `a_cost_budget_and_a_time_budget_stop_the_same_way` are the three budget shapes. `the_gate_is_a_plain_check` calls `check_gate` without a runtime: `None`, then `Some(Cancelled)` after `cancel()`.

## 6.3 The saved set

`saved_evals` is the list. `run_saved_evals` runs each one on a fresh script, a registry that holds `EchoTool` and `DelayRead`, and a dispatcher whose bound is 8. `DelayRead` increments a counter around a 20 ms sleep and records the peak. The eight-read task's script asks for eight of those calls, then the text `done`.

`the_saved_eval_set_passes_on_the_mock` asserts every score passed. That is the re-run. It stays inside the worker bound because a failing peak fails the score, and the score passed.

Chapter 15 and chapter 41 call this same function again. They are not written yet. The set is here so they have something stable to point at.

## Pitfalls

The gate cannot unsend a reply. Put the check before `complete`. A test that puts a final answer first, and then expects `Budget`, is testing the wrong branch: the loop returns `Finished` because the model asked for nothing else.

The mock will not trip a token or cost cap unless the script sets `Usage`. Forgetting that looks like a broken budget.

`max_iterations` counts model calls. Hermes refunds an iteration on some failed paths. This chapter does not. A provider error that still produced `Done` consumes the iteration.

A single tool call still takes a dispatcher permit. Hermes runs that one call on the calling thread and skips the pool. The difference shows up only when the permit is already held by something else. Nothing in this chapter holds it across the model call.

Eight 1 ms reads measured 2.392 ms, not 1 ms. The second scripted reply and the scheduling sit in that number. Quoting it as "one round trip" would be the 40 ms overlap test from chapter 4, which is a different bench.

The HTTP and stdio fixtures are the chapter 4 server, included again so this crate can spawn `mcp_fixture`. Dropping the session drops the child. `shutdown` still needs the session by value, and the loop holds an `Arc`.

## Chapter summary

- **Loop.** A scripted model call, then the chapter 3 dispatcher, then tool messages, then another call. Native `EchoTool` and the fixture's `echo`, over stdio and over streamable HTTP, share that path. One iteration with no tool call measured **263.9 ns**.
- **Overlap.** Eight reads that each sleep 1 ms, in one turn, measured **2.392 ms**. A 30 ms test stays under 200 ms and keeps the peak at or below 8.
- **Gate.** `check_gate` measured **2.711 ns**. A cancelled tool does not start the next model request (`provider.calls()` stays 1). A token cap, a cost cap, and a time cap stop the same way. A time cap that is already spent does not start the first request either.
- **Evals.** Three scripted tasks, `echo-hello`, `direct`, and `eight-reads`, pass on the mock. The eight-read task fails the score if the peak exceeds the worker bound.

[personal note: whether you trust a loop that stops, or a loop that always asks one more time]

## What's next

Chapter 7 is the turn state machine: one explicit state for this loop, and a tested table of which transitions are legal. The loop in this chapter returns `Finished`, `Cancelled`, or `Budget`. The state machine is where those become states you can match on, instead of a flag checked at the top of a `loop`.
