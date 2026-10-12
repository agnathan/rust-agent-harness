# Performance

Speed is a constraint from chapter 1, not a late part of the book. Every chapter has a budget below. The numbers exclude model latency: the mock provider's configurable wait stands in for the model, and the harness is measured with that wait set to zero unless the target says otherwise.

Principles and the way to choose a team shape live next to this page: [team patterns](team-patterns.md), [self-improvement](self-improvement.md).

## Principles

**Async from the first call.** Chapters 1 and 2 run on a tokio runtime. `complete` is async. There is no blocking HTTP client to replace later.

**One HTTP client.** `OpenAiCompatible` owns a single `reqwest::Client` and reuses connections. A new client per request is a bug.

**Tools and MCP calls run together, with a bound.** From chapters 3–5, native tools and MCP tools share one scheduler. The default bound is **8** in-flight calls. Safety classes decide who may share a batch:

- **Read-only** calls run concurrently, up to the bound.
- **Writes** run alone. A write never overlaps another call.

**The loop overlaps streaming and safe work.** As soon as a tool call is fully parsed from the token stream, a read-only call starts. The rest of the stream keeps arriving. A write waits for the exclusive section.

**Stable system prompts.** The system prompt, the skill index, and the memory snapshot are fixed for the session so a provider can cache that prefix. Mid-session changes are announced on the next user message. They do not rewrite the prefix.

**Share context with `Arc`.** A specialist, a pipeline stage, or a child agent receives `Arc` to the context object. The hot path does not clone the transcript to hand work off.

**Bounded channels.** Inboxes and bus subscriptions have a capacity (64 on an agent inbox). A slow consumer applies backpressure. Queues do not grow without a limit.

**Fan-out as soon as there is a team.** The supervisor (chapter 17) calls specialists concurrently under the worker limit. Chapter 19 is the dedicated gather, not the first time two agents run at once.

**Learning stays off the hot path.** Reflection, skill writes the model did not ask for, memory writes, index rebuilds, FTS updates, and curation are background tasks. The turn awaits the model and the tools the model actually called. Spawning a reviewer, enqueueing a proposal, or swapping an `Arc` index is the cost on the request path.

**Measure, and do not fail the build on a number yet.** Chapter 1 is three sections, each with a criterion bench: serde of 50 messages, one mock call and 100 concurrent mock calls, and request build plus response parse. Chapter 5 extends that idea to the agent loop. On pull requests and on every push to `main`, [Benchmarks](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml) runs those benches on Ubuntu and writes the means to the job summary. A regression does not fail the job.

## Performance budget

| Ch | Chapter | Target |
|---|---|---|
| 0 | Introduction | State the rule: overhead is measured without model latency. |
| 1 | First Contact | 1.1: serde of 50 messages **< 1 ms** (measured 6.96 µs). 1.2: one mock call **< 1 ms** (measured 135 ns) and 100 concurrent calls **< 50 ms** (measured 105 µs). 1.3: build and parse **< 1 ms** (measured 1.74 µs). One pooled client, HTTP/2 enabled. |
| 2 | Conversations | System-prompt bytes identical across the turns of a session. Per-turn overhead still **< 1 ms**. The model is called with a borrow, not a clone of the history. |
| 3 | Tools | **8** read-only tools in one batch finish in about one call, not eight, on the latency mock. A write runs alone. |
| 4 | MCP Client | MCP calls use that same bound. **8** concurrent mock MCP calls finish in about one round trip. |
| 5 | The Agent Loop | Loop-iteration overhead **< 1 ms** excluding the model. A safe tool starts while tokens are still arriving. The criterion bench covers both. |
| 6 | MCP Server | **32** in-flight calls accepted; **8** executed at once. |
| 7 | When Things Fail | A timeout cancels the in-flight call and releases its permit. Retries stay inside the bound. |
| 8 | Streaming | Assembling a streamed reply adds **< 1 ms** versus a buffered reply, excluding model time. |
| 9 | Sessions on Disk | One local WAL append of a turn **< 1 ms**. FTS updates run in the background. |
| 10 | Memory Files | The frozen snapshot is built once and shared by `Arc`. A memory write does not rewrite the system prompt. Enqueue of a background write **< 1 ms**. |
| 11 | Recall | Discover over **10,000** messages **< 20 ms**. Search runs only when the model calls the tool. |
| 12 | Using Skills | Load and render **500** skills **< 10 ms**, once per session. The rendered index is `Arc` and byte-stable. |
| 13 | Writing Skills | Security scan of one skill **< 5 ms**. A write the model did not ask for is a background task. |
| 14 | The Reflection Loop | Spawning the reviewer adds **< 1 ms** to turn finalize. Cancelling it when the user returns takes **< 50 ms**. |
| 15 | Curation | The curator never runs on the request path. |
| 16 | The Sequential Pipeline | The next stage receives `Arc` context. No full transcript copy. |
| 17 | The Supervisor | Up to **8** specialists at once, each with `Arc` context. Eight latency-mock specialists finish in about one specialist's time. |
| 18 | Handoffs | Filtering a **100**-message transcript for a handoff **< 1 ms**. |
| 19 | Fan-out and Gather | The worker limit holds. **100** mock subagents can be scheduled without an unbounded queue. |
| 20 | The Critic | The critic shares `Arc` context and takes one permit from the same bound. |
| 21 | Composing Patterns | Nested fan-out cannot exceed the worker limit. |
| 22 | The Turn State Machine | A transition **< 1 µs**. Turn overhead still **< 1 ms**. |
| 23 | Managing Context | Compression and the memory flush run between turns or in the background. The hot path sends the `Arc` snapshot. |
| 24 | Agents as Actors | Inbox capacity **64**. **100+** mock agents at once. A full inbox applies backpressure. |
| 25 | Cancellation and Supervision | Cancel the root and **100** mock children stop in **< 50 ms**. |
| 26 | The Blackboard | A claim **< 1 ms** with **100** waiters. The lock is not held across a model call. |
| 27 | The Message Bus | Publish of one small envelope **< 1 ms** at **100** agents. Slow subscribers hit a bounded channel. |
| 28 | Shared Team Skills | `skill_propose` enqueues in **< 1 ms**. The index swap is an `Arc` swap. Workers do not wait on the librarian. |
| 29 | A2A for External Agents | Remote calls use the HTTP pool and the same bound. Cancel reaches an in-flight call in **< 50 ms**. |
| 30 | Seeing Inside | A span per turn adds **< 0.1 ms**. The chapter 1 bench still meets its budget with tracing on. The capstone runs **100+** concurrent mock agents. |
