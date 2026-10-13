# Performance

Speed is a constraint from chapter 1, not a late part of the book. Every chapter has a budget below. The numbers exclude model latency: the mock provider's configurable wait stands in for the model, and the harness is measured with that wait set to zero unless the target says otherwise.

Principles and the way to choose a team shape live next to this page: [team patterns](team-patterns.md), [self-improvement](self-improvement.md).

## Principles

**Async from the first call.** Chapters 1 and 2 run on a tokio runtime. `complete` is async. There is no blocking HTTP client to replace later.

**One HTTP client.** `OpenAiCompatible` owns a single `reqwest::Client` and reuses connections. A new client per request is a bug.

**Tools and MCP calls run together, with a bound.** From chapter 3 through the agent loop in chapter 9, native tools and MCP tools share one scheduler. The default bound is **8** in-flight calls. Safety classes decide who may share a batch:

- **Read-only** calls run concurrently, up to the bound.
- **Writes** run alone. A write never overlaps another call.

**The loop overlaps streaming and safe work.** As soon as a tool call is fully parsed from the token stream, a read-only call starts. The rest of the stream keeps arriving. A write waits for the exclusive section.

**Stable system prompts.** The system prompt, the skill index, and the memory snapshot are fixed for the session so a provider can cache that prefix. Mid-session changes are announced on the next user message. They do not rewrite the prefix.

**Share context with `Arc`.** A specialist, a pipeline stage, or a child agent receives `Arc` to the context object. The hot path does not clone the transcript to hand work off.

**Bounded channels.** Inboxes and bus subscriptions have a capacity (64 on an agent inbox). A slow consumer applies backpressure. Queues do not grow without a limit.

**Fan-out as soon as there is a team.** The supervisor (chapter 27) calls specialists concurrently under the worker limit. Chapter 29 is the dedicated gather, not the first time two agents run at once.

**Learning stays off the hot path.** Reflection, skill writes the model did not ask for, memory writes, index rebuilds, FTS updates, and curation are background tasks. The turn awaits the model and the tools the model actually called. Spawning a reviewer, enqueueing a proposal, or swapping an `Arc` index is the cost on the request path.

**Measure, and do not fail the build on a number yet.** Chapter 1 is four sections, each with a criterion bench: serde of 50 messages, one mock call and 100 concurrent mock calls, request build plus response parse, and the edges of a buffered reply. Chapter 2 adds an inbox-submit bench and a mock chat-turn bench. Chapter 3 adds a registry lookup, an available-tool list, and a concurrent-versus-sequential dispatch bench. Chapter 4 measures a stdio echo round trip, a streamable-HTTP echo round trip, 50 MCP calls through the bound of 8, and a cached `tools/list`. Chapter 6 (crates `steps/ch05-*`) measures schema tokens, search, a lazy tool prompt, routed search, and a cached tool list over thousands of tools. That chapter is the client half of the catalog. Chapter 9, the agent loop, is ten sections whose crates are still `steps/ch06-*` and whose section labels stay 6.1–6.10. It measures a mock loop iteration, the cancellation and budget check, eight delayed reads, and one bench for each later check (repair, stall, the budget notice, truncation, steering, canonical arguments, the outcome label, and the stress repair). Shared bench names in the job summary are the section 6.10 run, because that section runs last. The chapter also starts a saved eval set, plus four stress tasks, that later chapters re-run. Chapter 48 scores that set. Server-track targets below have no measurement yet. On pull requests and on every push to `main`, [Benchmarks](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml) runs those benches on Ubuntu, writes the means to the job summary, and uploads `criterion-summary-<commit>.json`. The same file feeds a chart at `dev/bench` on `gh-pages` when the job is allowed to publish. A regression does not fail the job.

## Performance budget

| Ch | Chapter | Target |
|---|---|---|
| 0 | Introduction | State the rule: overhead is measured without model latency. |
| 1 | First Contact | 1.1: serde of 50 messages **< 1 ms** (measured 6.96 µs). 1.2: one mock call that builds events **< 1 ms** (measured 235.7 ns) and 100 concurrent calls **< 50 ms** (measured 108.9 µs). 1.3: build an OpenRouter request and parse `model`, `usage`, and `cost` **< 1 ms** (measured 2.133 µs). 1.4: parse a reply with usage and one without **< 1 ms** (measured 792.7 ns). One pooled client, HTTP/2 enabled. Default endpoint is OpenRouter. |
| 2 | Conversations | System-prompt bytes identical across the turns of a session. Per-turn overhead still **< 1 ms** (measured 628.2 ns). Submitting one turn to the session inbox is **< 1 µs** (measured 32.78 ns). The model is called with a borrow, not a clone of the history. |
| 3 | Tools | Registry lookup in a **50**-tool map is **< 1 µs** (measured 13.96 ns). The available-tool list for **50** tools is **< 1 ms** (measured 21.89 µs). **8** read-only tools that each sleep 1 ms finish together in **2.362 ms**, against **17.55 ms** one at a time. A write runs alone. |
| 4 | MCP Client | MCP calls use the chapter 3 bound of **8**. Eight remote reads that each sleep 40 ms overlap (under 200 ms together, over 250 ms one at a time). A stdio `echo` round trip is **71.29 µs**. The same call over streamable HTTP on `127.0.0.1` is **215.7 µs**. Fifty zero-delay `echo` calls are **1.908 ms** concurrent and **3.603 ms** sequential. A cold `tools/list` is **82.87 µs**. The cache hit is **168.0 ns**. |
| 5 | A Minimal MCP Server *(server)* | **32** in-flight calls accepted; **8** executed at once. The skeleton does not accept calls yet. |
| 6 | Thousands of Tools | Schema tokens for **10,000** tools **< 50 ms** (measured 21.06 ms). Top-8 search **< 1 ms** (measured 146.8 µs). The lazy tool prompt **< 1 ms** (measured 12.26 µs), and its core prefix stays byte-identical across reveal and eviction. A cached `tools/list` hit **< 1 µs** (measured 17.56 ns). Filtering one agent's servers out of **10,000** tools **< 1 ms** (measured 77.00 µs). Client half. Crates stay `ch05-*`. |
| 7 | Dynamic Catalog *(server)* | Search over **10,000** tools on the server. A catalog reload swaps an `Arc` and does not block an in-flight call. No server measurement yet. |
| 8 | Session Toolsets *(server)* | Tokens per completed task, for every tool versus a static set versus a dynamic set, reported with task success. No measurement yet. |
| 9 | The Agent Loop | Ten sections, crates `ch06-*`, labels 6.1–6.10. One scripted iteration on the finished loop is **< 1 ms** excluding the model (measured **608.0 ns**; the same bench on section 6.1, before the later checks, measured **248.5 ns**). `check_gate` is **< 1 µs** (measured **11.92 ns** on the finished loop). A cancelled call does not start the next model request. Eight 1 ms reads in one turn measured **2.421 ms** on the bound of 8. Name repair **188.8 ns**, stall observe **100.0 ns**, a budget notice **58.20 ns**, truncating a long tool result **3.174 µs**, appending a steer **47.12 ns**, canonicalizing one arguments object **771.0 ns**, labeling an outcome **5.606 ns**, and the unknown-tool-repair eval **20.80 µs**. The saved eval set and the four stress evals pass on the mock. |
| 10 | The Turn State Machine | A transition **< 1 µs**. Turn overhead still **< 1 ms**. |
| 11 | Approvals and Guardrails | A skip decision is **< 1 µs**. Concurrent approvals serialize onto one prompt. Waiting on a human releases the tool permit. |
| 12 | Hooks and Plugins | An empty before/after hook adds **< 0.1 ms**. An API-request hook does not copy the conversation. A memory-backend record enqueues in **< 1 ms**. |
| 13 | Sandboxed Execution | A warm sandbox accepts a job in **< 50 ms**. Checkpoint and rewind of a **1,000**-path manifest are each **< 10 ms**. |
| 14 | When Things Fail | A timeout cancels the in-flight call and releases its permit. Retries stay inside the bound. Empty-response retries, length continuation, and the repetition guard, deferred from chapter 9, land here. |
| 15 | Limits and Breakers *(server)* | A timeout releases that tool's permit. A retry runs only when the tool is marked idempotent. An open breaker stops new calls to that backend. |
| 16 | Providers | Choosing the next native adapter, or the next OpenRouter route, adds **< 1 ms**, excluding the failed call. Rotation reuses the pooled client from chapter 1. A side-model summary stays off the user turn unless that turn asked for it. |
| 17 | Streaming | Assembling a streamed reply from chapter 1 events adds **< 1 ms** versus a buffered reply, excluding model time. |
| 18 | Sessions on Disk | One local WAL append of a turn **< 1 ms**. FTS updates run in the background. The chapter re-runs the chapter 9 eval set. |
| 19 | Memory Files | The frozen snapshot is built once and shared by `Arc`. A memory write does not rewrite the system prompt. Enqueue of a background write **< 1 ms**. |
| 20 | Recall | Discover over **10,000** messages **< 20 ms**. Search runs only when the model calls the tool. |
| 21 | Learned Tools *(server)* | Prediction stays off the request path except for the offered set. No measurement yet. |
| 22 | Using Skills | Load and render **500** skills **< 10 ms**, once per session. The rendered index is `Arc` and byte-stable. |
| 23 | Writing Skills | Security scan of one skill **< 5 ms**. A write the model did not ask for is a background task. |
| 24 | The Reflection Loop | Spawning the reviewer adds **< 1 ms** to turn finalize. Cancelling it when the user returns takes **< 50 ms**. |
| 25 | Curation | The curator never runs on the request path. |
| 26 | The Sequential Pipeline | The next stage receives `Arc` context. No full transcript copy. |
| 27 | The Supervisor | Up to **8** specialists at once, each with `Arc` context. Eight latency-mock specialists finish in about one specialist's time. |
| 28 | Handoffs | Filtering a **100**-message transcript for a handoff **< 1 ms**. |
| 29 | Fan-out and Gather | The worker limit holds. **100** mock subagents can be scheduled without an unbounded queue. |
| 30 | The Critic | The critic shares `Arc` context and takes one permit from the same bound. |
| 31 | Composing Patterns | Nested fan-out cannot exceed the worker limit. |
| 32 | Managing Context | Compression and the memory flush run between turns or in the background. The hot path sends the `Arc` snapshot. Preflight compression, deferred from chapter 9, runs before the model call. |
| 33 | Agents as Actors | Session inbox capacity **64**. **100+** mock agents at once. A full inbox applies backpressure. |
| 34 | Supervision and Restart | Restarting a failed child adds **< 1 ms**, excluding the child's work. |
| 35 | Balancing and Federation *(server)* | A per-tool pool picks a backend by least outstanding requests, or by power of two choices. Federated upstream MCP servers share that pool. No measurement yet. |
| 36 | Profiles | Resolving a profile and its secrets is **< 1 ms**. Secrets are not copied into the prompt. |
| 37 | The Gateway | Enqueue onto the session inbox is **< 1 ms**. A steer is a push onto that inbox. Streaming folds chapter 1 events into one message. This is the chat gateway, not the MCP server. |
| 38 | MCP Gateway Service *(server)* | Throughput and p99 under load (oha or something like it). Killing one replica fails over by session id. Drain finishes in-flight calls. No measurement yet. |
| 39 | Cron | A crash retries a claimed job at most once. Scanning **1,000** due jobs is **< 10 ms**. The watchdog and a chained job stay off the request path. |
| 40 | Editor Integration | Translating an ACP update onto the session queue is **< 1 ms**. |
| 41 | The Blackboard | A claim **< 1 ms** with **100** waiters. The lock is not held across a model call. |
| 42 | The Kanban Board | Claim, complete, review, or block is one transaction **< 5 ms**. Two processes cannot hold the same task. |
| 43 | The Message Bus | Publish of one small envelope **< 1 ms** at **100** agents. Slow subscribers hit a bounded channel. |
| 44 | Shared Team Skills | `skill_propose` enqueues in **< 1 ms**. The index swap is an `Arc` swap. Workers do not wait on the librarian. |
| 45 | A2A for External Agents | Remote calls use the HTTP pool and the same bound. Cancel reaches an in-flight call in **< 50 ms**. |
| 46 | Seeing Inside | A span per chapter 1 event adds **< 0.1 ms**, and spans cross the harness and the gateway. The chapter 1 bench still meets its budget with tracing on. The capstone runs **100+** concurrent mock agents. |
| 47 | Load and Token Capstone *(server)* | The load test and the token comparison (every tool, a static set, a dynamic set) with task success. No measurement yet. |
| 48 | Evals | Re-running the saved eval set, and **100** mock trajectories, stays inside the worker bound. Appending one trajectory event is **< 1 ms**. |
