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

**Fan-out as soon as there is a team.** The supervisor (chapter 29) calls specialists concurrently under the worker limit. Chapter 31 is the dedicated gather, not the first time two agents run at once.

**Learning stays off the hot path.** Reflection, skill writes the model did not ask for, memory writes, index rebuilds, FTS updates, and curation are background tasks. The turn awaits the model and the tools the model actually called. Spawning a reviewer, enqueueing a proposal, or swapping an `Arc` index is the cost on the request path.

**Measure, and do not fail the build on a number yet.** Chapter 1 is four sections, each with a criterion bench: serde of 50 messages, one mock call and 100 concurrent mock calls, request build plus response parse, and the edges of a buffered reply. Chapter 2 adds an inbox-submit bench and a mock chat-turn bench. Chapter 3 adds a registry lookup, an available-tool list, and a concurrent-versus-sequential dispatch bench. Chapter 4 measures a stdio echo round trip, a streamable-HTTP echo round trip, 50 MCP calls through the bound of 8, and a cached `tools/list`. Chapter 6 (crates `steps/ch06-*`) measures schema tokens, search, a lazy tool prompt, routed search, and a cached tool list over thousands of tools. That chapter is the client half of the catalog. Chapter 9, the agent loop, is ten sections whose crates are still `steps/ch09-*` and whose section labels stay 9.1–9.10. It measures a mock loop iteration, the cancellation and budget check, eight delayed reads, and one bench for each later check (repair, stall, the budget notice, truncation, steering, canonical arguments, the outcome label, and the stress repair). Each section is recorded on its own, so section 9.1's `loop_iteration` is not overwritten by section 9.10. Older artifacts stored one mean per function name, and those points are seeded on section 9.10. The chapter also starts a saved eval set, plus four stress tasks, that later chapters re-run. Chapter 52 scores that set. Chapters 5, 7, 8, and 10 quote `mean.point_estimate` from Benchmarks run 38863938100 on ubuntu-latest. Later server chapters are still unmeasured. On pull requests and on every push to `main`, [Benchmarks](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml) runs those benches on Ubuntu, writes the median, low, and high to the job summary, and uploads `criterion-summary-<commit>.json`. A push to `main` appends the run to the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/). The same job still updates `dev/bench` on `gh-pages` when it is allowed to publish. A regression does not fail the job.

## Performance budget

| Ch | Chapter | Target |
|---|---|---|
| 0 | Introduction | State the rule: overhead is measured without model latency. |
| 1 | First Contact | 1.1: serde of 50 messages **< 1 ms** (measured 9.96 µs). 1.2: one mock call that builds events **< 1 ms** (measured 235.7 ns) and 100 concurrent calls **< 50 ms** (measured 108.9 µs). 1.3: build an OpenRouter request and parse `model`, `usage`, and `cost` **< 1 ms** (measured 2.133 µs). 1.4: parse a reply with usage and one without **< 1 ms** (measured 792.7 ns). One pooled client, HTTP/2 enabled. Default endpoint is OpenRouter. |
| 2 | Conversations | System-prompt bytes identical across the turns of a session. Per-turn overhead still **< 1 ms** (measured 628.2 ns). Submitting one turn to the session inbox is **< 1 µs** (measured 32.78 ns). The model is called with a borrow, not a clone of the history. |
| 3 | Tools | Registry lookup in a **50**-tool map is **< 1 µs** (measured 13.96 ns). The available-tool list for **50** tools is **< 1 ms** (measured 21.89 µs). **8** read-only tools that each sleep 1 ms finish together in **2.362 ms**, against **17.55 ms** one at a time. A write runs alone. |
| 4 | MCP Client | MCP calls use the chapter 3 bound of **8**. Eight remote reads that each sleep 40 ms overlap (under 200 ms together, over 250 ms one at a time). A stdio `echo` round trip is **71.29 µs**. The same call over streamable HTTP on `127.0.0.1` is **215.7 µs**. Fifty zero-delay `echo` calls are **1.908 ms** concurrent and **3.603 ms** sequential. A cold `tools/list` is **82.87 µs**. The cache hit is **168.0 ns**. |
| 5 | A Minimal MCP Server *(server)* | **32** in-flight calls accepted; **8** executed at once. One permit round trip measured **45.53 ns**. Eight 1 ms sleeps on the execute bound measured **2.129 ms**. |
| 6 | Thousands of Tools | Schema tokens for **10,000** tools **< 50 ms** (measured 21.06 ms). Top-8 search **< 1 ms** (measured 146.8 µs). The lazy tool prompt **< 1 ms** (measured 12.26 µs), and its core prefix stays byte-identical across reveal and eviction. A cached `tools/list` hit **< 1 µs** (measured 17.56 ns). Filtering one agent's servers out of **10,000** tools **< 1 ms** (measured 77.00 µs). Client half. Crates stay `ch06-*`. |
| 7 | Dynamic Catalog *(server)* | Search over **10,000** tools measured **299.0 µs**. Rebuilding that index measured **27.42 ms**. A one-tool `Arc` swap while the old snapshot is held measured **2.467 µs**. |
| 8 | Session Toolsets *(server)* | On **2,000** tools, schema-token estimates were **45,104** (every tool), **153** (static), and **135** (dynamic). All three arms succeeded. Dynamic measured **5.687 ms**. Every-tool measured **8.355 ms**. |
| 9 | The Agent Loop | Ten sections, crates `ch09-*`, labels 9.1–9.10. One scripted iteration on the finished loop is **< 1 ms** excluding the model (measured **608.0 ns**; the same bench on section 9.1, before the later checks, measured **248.5 ns**). `check_gate` is **< 1 µs** (measured **11.92 ns** on the finished loop). A cancelled call does not start the next model request. Eight 1 ms reads in one turn measured **2.421 ms** on the bound of 8. Name repair **188.8 ns**, stall observe **100.0 ns**, a budget notice **58.20 ns**, truncating a long tool result **3.174 µs**, appending a steer **47.12 ns**, canonicalizing one arguments object **771.0 ns**, labeling an outcome **5.606 ns**, and the unknown-tool-repair eval **20.80 µs**. The saved eval set and the four stress evals pass on the mock. |
| 10 | The Turn State Machine | One `step` measured **1.258 ns**. A three-step scripted turn measured **19.08 ns**. |
| 11 | Approvals and Guardrails | A skip decision is **< 1 µs** (measured **249.6 ns**). Concurrent approvals serialize onto one prompt. Waiting on a human releases the tool permit. |
| 12 | Hooks and Plugins | An empty before/after hook adds **< 0.1 ms** (measured **10.90 ns**). An API-request hook does not copy the conversation (measured **2.182 ns** on 1,000 messages). A memory-backend record enqueues in **< 1 ms** (measured **61.70 ns**). |
| 13 | Sandboxed Execution | A warm sandbox accepts a job in **< 50 ms** (measured **30.49 µs**). Checkpoint and rewind of a **1,000**-path manifest are each **< 10 ms** (measured **50.37 µs** and **83.79 µs**). |
| 14 | When Things Fail | A timeout cancels the in-flight call and releases its permit. Retries stay inside the bound. The empty-response budget check measured **0.9046 ns**. The repetition guard on an 800-character loop measured **2.77 µs**. |
| 15 | Limits and Breakers *(server)* | A timeout releases that tool's permit. A retry runs only when the tool is marked idempotent. An open breaker stops new calls to that backend (measured **31.97 ns**). One execute-permit round trip measured **57.73 ns**. |
| 16 | Built-in Tools I: Web Search and Extract | A fake search measured **5.023 µs**. Blocking the metadata address measured **335.1 ns**. No network and no API key. |
| 17 | Built-in Tools II: Files, Terminal, Todo, and Clarify | Applying one patch measured **255.1 µs**. Merging one todo measured **131.2 ns**. No network. |
| 18 | Providers | Choosing the next native adapter, or the next OpenRouter route, adds **< 1 ms**, excluding the failed call (measured **5.990 ns**). Rotation reuses the pooled client from chapter 1 (measured **3.465 ns**). A side-model summary stays off the user turn unless that turn asked for it. |
| 19 | Streaming | Assembling a streamed reply from chapter 1 events adds **< 1 ms** versus a buffered reply, excluding model time. Folding 100 plain chunks measured **2.67 µs**. Joining the same chunks measured **689.3 ns**. |
| 20 | Sessions on Disk | One local WAL append of a turn **< 1 ms** (measured **139.2 µs**). FTS updates run in the background. The chapter re-runs the chapter 9 eval set. |
| 21 | Memory Files | The frozen snapshot is built once and shared by `Arc`. A memory write does not rewrite the system prompt. Enqueue of a background write **< 1 ms** (measured **54.59 ns**). |
| 22 | Recall | Discover over **10,000** messages **< 20 ms** (measured **60.80 µs**). Search runs only when the model calls the tool. |
| 23 | Learned Tools *(server)* | Prediction stays off the request path except for the offered set. Applying a 32-name offer measured **79.01 ns**. Scoring 1,000 usage rows against 64 names measured **153.1 µs**, and that function is not called from `session_tools`. |
| 24 | Using Skills | Load and render **500** skills **< 10 ms**, once per session (measured **2.162 ms**). The rendered index is `Arc` and byte-stable. |
| 25 | Writing Skills | Security scan of one skill **< 5 ms** (measured **242.5 ns**). A write the model did not ask for is a background task (enqueue measured **17.96 ns**). |
| 26 | The Reflection Loop | Spawning the reviewer adds **< 1 ms** to turn finalize (measured **60.16 ns**). Cancelling it when the user returns takes **< 50 ms** (measured **819.8 µs**). |
| 27 | Curation | The curator never runs on the request path. Enqueue measured **27.80 ns**. |
| 28 | The Sequential Pipeline | The next stage receives `Arc` context (measured **10.46 ns**). Copying 100 messages measured **6.568 µs** and is not the hand-off. |
| 29 | The Supervisor | Up to **8** specialists at once, each with `Arc` context. Eight specialists that each slept 40 ms finished in **0.044 s** in nextest. Scheduling eight with no sleep measured **148.1 µs**. A reflection note measured **20.81 ns**. |
| 30 | Handoffs | Filtering a **100**-message transcript for a handoff **< 1 ms** (measured **2.664 µs**). |
| 31 | Fan-out and Gather | The worker limit holds. **100** mock subagents can be scheduled without an unbounded queue (scheduling 100 measured **4.285 µs**). The 101st is refused. |
| 32 | The Critic | The critic shares `Arc` context and takes one permit from the same bound (one judgment measured **196.6 ns**). |
| 33 | Composing Patterns | Nested fan-out cannot exceed the worker limit. Eight children on the shared pool measured **271.87 µs**. Width 9 is refused before any task is scheduled. |
| 34 | Managing Context | Compression and the memory flush run between turns or in the background. The hot path sends the `Arc` snapshot (measured **4.86 ns**). Preflight compression, deferred from chapter 9, runs before the model call. |
| 35 | Agents as Actors | Session inbox capacity **64**. **100+** mock agents at once. A full inbox applies backpressure. One submit measured **250.1 ns**. |
| 36 | Supervision and Restart | Restarting a failed child adds **< 1 ms**, excluding the child's work (one record measured **29.8 ns**). |
| 37 | Balancing and Federation *(server)* | A per-tool pool picks a backend by least outstanding requests, or by power of two choices. Federated upstream MCP servers share that pool. One least-outstanding pick measured **3.74 ns**. Discovery connects at most **4** at once. |
| 38 | Profiles | Resolving a profile and its secrets is **< 1 ms** (one resolve measured **96.7 ns**). Secrets are not copied into the prompt. |
| 39 | The Gateway | Enqueue onto the session inbox is **< 1 ms** (one enqueue measured **152.0 ns**). A steer is a push onto that inbox. Streaming folds chapter 1 events into one message. This is the chat gateway, not the MCP server. |
| 40 | Browser Tools | A canned navigation checks the URL, then reads a fake page (measured **1.97 µs**). No browser process. |
| 41 | Vision | A 1×1 PNG is sniffed in process (measured **11.2 ns**). An oversized buffer or a bad type is refused. No vision API. |
| 42 | MCP Gateway Service *(server)* | Killing one replica fails over by session id. Drain finishes in-flight calls. One route lookup measured **19.7 ns**. oha throughput is not measured: this step does not bind a socket. |
| 43 | Cron | A crash retries a claimed job at most once. Scanning **1,000** due jobs is **< 10 ms** (measured **1.75 µs**). The watchdog and a chained job stay off the request path. |
| 44 | Editor Integration | Translating an ACP update onto the session queue is **< 1 ms** (one translate measured **132.4 ns**). |
| 45 | The Blackboard | A claim **< 1 ms** with **100** waiters (measured **4.96 µs**). The lock is not held across a model call. |
| 46 | The Kanban Board | Claim, complete, review, or block is one transaction **< 5 ms** (one claim measured **328.02 µs**). Two processes cannot hold the same task. |
| 47 | The Message Bus | Publish of one small envelope **< 1 ms** at **100** agents (measured **47.98 µs**). The sample includes `redact_for_egress`. Slow subscribers hit a bounded channel. |
| 48 | Shared Team Skills | `skill_propose` enqueues in **< 1 ms** (measured **80.1 ns**). The index swap is an `Arc` swap (measured **200.3 ns**). Workers do not wait on the librarian. |
| 49 | A2A for External Agents | Cancel reaches an in-flight wait in **< 50 ms** (measured **46.13 µs**). The sample is the flag and the join. |
| 50 | Seeing Inside | A span per chapter 1 event adds **< 0.1 ms** (a bare complete measured **18.1 ns**; the spanned turn measured **236.4 ns**). The chapter 1 budget of **1 ms** still holds with tracing on. The research team runs **100** mock agents, then one more. |
| 51 | Load and Token Capstone *(server)* | Dynamic schema tokens are **5**, the static eight are **40**, and every tool is **160**, each with task success for `tool_07`. The dynamic arm measured **264.8 ns**. A load of **100** in-memory gateway calls measured **122.77 µs**. oha is not measured: this step does not bind a socket. |
| 52 | Evals | Re-running the saved eval set measured **21.26 ms**, including the **20 ms** `delay_read` sleep inside `eight-reads`. **100** mock trajectories stay inside the worker bound. Appending one trajectory event is **< 1 ms** (measured **82.2 ns**). There is no fail threshold. |
| 53 | The Loop Closes | The fake-model task fails once (0 tool calls, 1 model call) and passes once (1 tool call, 2 model calls). `53/close_once` measured **701.61 µs** (`mean.point_estimate` from [run 40163536226](https://github.com/agnathan/rust-agent-harness/actions/runs/40163536226)). |
