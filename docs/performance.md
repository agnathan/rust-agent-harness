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

**Fan-out as soon as there is a team.** The supervisor (chapter 22) calls specialists concurrently under the worker limit. Chapter 24 is the dedicated gather, not the first time two agents run at once.

**Learning stays off the hot path.** Reflection, skill writes the model did not ask for, memory writes, index rebuilds, FTS updates, and curation are background tasks. The turn awaits the model and the tools the model actually called. Spawning a reviewer, enqueueing a proposal, or swapping an `Arc` index is the cost on the request path.

**Measure, and do not fail the build on a number yet.** Chapter 1 is four sections, each with a criterion bench: serde of 50 messages, one mock call and 100 concurrent mock calls, request build plus response parse, and the edges of a buffered reply. Chapter 5 extends that idea to the agent loop and starts a saved eval set that later chapters re-run. Chapter 40 scores that set. On pull requests and on every push to `main`, [Benchmarks](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml) runs the chapter 1 benches on Ubuntu, writes the means to the job summary, and uploads `criterion-summary-<commit>.json`. The same file feeds a chart at `dev/bench` on `gh-pages` when the job is allowed to publish. A regression does not fail the job.

## Performance budget

| Ch | Chapter | Target |
|---|---|---|
| 0 | Introduction | State the rule: overhead is measured without model latency. |
| 1 | First Contact | 1.1: serde of 50 messages **< 1 ms** (measured 6.96 µs). 1.2: one mock call that builds events **< 1 ms** (measured 235.7 ns) and 100 concurrent calls **< 50 ms** (measured 108.9 µs). 1.3: build an OpenRouter request and parse `model`, `usage`, and `cost` **< 1 ms** (measured 2.133 µs). 1.4: parse a reply with usage and one without **< 1 ms** (measured 792.7 ns). One pooled client, HTTP/2 enabled. Default endpoint is OpenRouter. |
| 2 | Conversations | System-prompt bytes identical across the turns of a session. Per-turn overhead still **< 1 ms**. Submitting one turn to the session inbox is **< 1 µs**. The model is called with a borrow, not a clone of the history. |
| 3 | Tools | **8** read-only tools in one batch finish in about one call, not eight, on the latency mock. A write runs alone. The available-tool list for a **50**-tool registry is **< 1 ms**. |
| 4 | MCP Client | MCP calls use that same bound. **8** concurrent mock MCP calls finish in about one round trip. |
| 5 | The Agent Loop | Loop-iteration overhead **< 1 ms** excluding the model. Checking the cancellation token or the budget is **< 1 µs**. A cancelled call does not start the next model request. |
| 6 | The Turn State Machine | A transition **< 1 µs**. Turn overhead still **< 1 ms**. |
| 7 | MCP Server | **32** in-flight calls accepted; **8** executed at once. |
| 8 | Approvals and Guardrails | A skip decision is **< 1 µs**. Concurrent approvals serialize onto one prompt. Waiting on a human releases the tool permit. |
| 9 | Hooks and Plugins | An empty before/after hook adds **< 0.1 ms**. An API-request hook does not copy the conversation. A memory-backend record enqueues in **< 1 ms**. |
| 10 | Sandboxed Execution | A warm sandbox accepts a job in **< 50 ms**. Checkpoint and rewind of a **1,000**-path manifest are each **< 10 ms**. |
| 11 | When Things Fail | A timeout cancels the in-flight call and releases its permit. Retries stay inside the bound. |
| 12 | Providers | Choosing the next native adapter, or the next OpenRouter route, adds **< 1 ms**, excluding the failed call. Rotation reuses the pooled client from chapter 1. A side-model summary stays off the user turn unless that turn asked for it. |
| 13 | Streaming | Assembling a streamed reply from chapter 1 events adds **< 1 ms** versus a buffered reply, excluding model time. |
| 14 | Sessions on Disk | One local WAL append of a turn **< 1 ms**. FTS updates run in the background. The chapter re-runs the chapter 5 eval set. |
| 15 | Memory Files | The frozen snapshot is built once and shared by `Arc`. A memory write does not rewrite the system prompt. Enqueue of a background write **< 1 ms**. |
| 16 | Recall | Discover over **10,000** messages **< 20 ms**. Search runs only when the model calls the tool. |
| 17 | Using Skills | Load and render **500** skills **< 10 ms**, once per session. The rendered index is `Arc` and byte-stable. |
| 18 | Writing Skills | Security scan of one skill **< 5 ms**. A write the model did not ask for is a background task. |
| 19 | The Reflection Loop | Spawning the reviewer adds **< 1 ms** to turn finalize. Cancelling it when the user returns takes **< 50 ms**. |
| 20 | Curation | The curator never runs on the request path. |
| 21 | The Sequential Pipeline | The next stage receives `Arc` context. No full transcript copy. |
| 22 | The Supervisor | Up to **8** specialists at once, each with `Arc` context. Eight latency-mock specialists finish in about one specialist's time. |
| 23 | Handoffs | Filtering a **100**-message transcript for a handoff **< 1 ms**. |
| 24 | Fan-out and Gather | The worker limit holds. **100** mock subagents can be scheduled without an unbounded queue. |
| 25 | The Critic | The critic shares `Arc` context and takes one permit from the same bound. |
| 26 | Composing Patterns | Nested fan-out cannot exceed the worker limit. |
| 27 | Managing Context | Compression and the memory flush run between turns or in the background. The hot path sends the `Arc` snapshot. |
| 28 | Agents as Actors | Session inbox capacity **64**. **100+** mock agents at once. A full inbox applies backpressure. |
| 29 | Supervision and Restart | Restarting a failed child adds **< 1 ms**, excluding the child's work. |
| 30 | Profiles | Resolving a profile and its secrets is **< 1 ms**. Secrets are not copied into the prompt. |
| 31 | The Gateway | Enqueue onto the session inbox is **< 1 ms**. A steer is a push onto that inbox. Streaming folds chapter 1 events into one message. |
| 32 | Cron | A crash retries a claimed job at most once. Scanning **1,000** due jobs is **< 10 ms**. The watchdog and a chained job stay off the request path. |
| 33 | Editor Integration | Translating an ACP update onto the session queue is **< 1 ms**. |
| 34 | The Blackboard | A claim **< 1 ms** with **100** waiters. The lock is not held across a model call. |
| 35 | The Kanban Board | Claim, complete, review, or block is one transaction **< 5 ms**. Two processes cannot hold the same task. |
| 36 | The Message Bus | Publish of one small envelope **< 1 ms** at **100** agents. Slow subscribers hit a bounded channel. |
| 37 | Shared Team Skills | `skill_propose` enqueues in **< 1 ms**. The index swap is an `Arc` swap. Workers do not wait on the librarian. |
| 38 | A2A for External Agents | Remote calls use the HTTP pool and the same bound. Cancel reaches an in-flight call in **< 50 ms**. |
| 39 | Seeing Inside | A span per chapter 1 event adds **< 0.1 ms**. The chapter 1 bench still meets its budget with tracing on. The capstone runs **100+** concurrent mock agents. |
| 40 | Evals | Re-running the saved eval set, and **100** mock trajectories, stays inside the worker bound. Appending one trajectory event is **< 1 ms**. |
