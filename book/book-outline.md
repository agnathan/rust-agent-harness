# Book outline

**Working title:** *Learning Rust by Building a Multi-Agent Harness*

**Alternative titles:**
- *Agents in Rust: From First Function to Agent Teams*
- *Fearless Agents*

**Audience:** Developers who know at least one other language (Python, JavaScript, Go) and are curious about AI agents, but are new to Rust. No systems-programming background is assumed.

**Promise:** By the end, the reader has built a working, tested, concurrent harness for teams of LLM agents. Rust shows up in short sidebars at the moment a harness problem needs it.

**Approach:** Every chapter builds one more piece of the same harness. The finished code for that chapter is a step crate under `steps/chNN-<slug>/`. The reader's own copy grows in `my-harness/`. Each chapter follows the same shape:

1. a problem from agent engineering
2. a **How Hermes does it** box: the Python design, then the Rust one
3. the Rust needed for this piece, in a short sidebar
4. building it, with the milestone tests in `my-harness/tests/chNN.rs`
5. a milestone demo: the mock by default, and an optional real OpenRouter call
6. what was learned, including `[personal note]` placeholders for the author's own experience

The book is six parts and chapters 0 through 40. Performance is a budget on every chapter, not a late part: async tokio from chapter 1, concurrent tools from chapter 3, and learning work off the hot path. The numbers are in [`docs/performance.md`](../docs/performance.md). MCP is first-class from Part I: tools are shaped like MCP tools (chapter 3), the harness is an MCP client (chapter 4, via `rmcp`) and an MCP server (chapter 7), and later chapters call those servers and can publish skills through them. Part II is learning: one agent gets better with use before the book builds a team. The design for skills, memory, recall, reflection, curation, and the team librarian is [`docs/self-improvement.md`](../docs/self-improvement.md). Part III is the team patterns. The default team is a supervisor that calls other agents as tools, fans them out under a bound, and shares one context object by `Arc`. The rules for choosing a pattern are in [`docs/team-patterns.md`](../docs/team-patterns.md). Part IV is making that system robust. Part V runs it as a service: profiles, a gateway, cron, and an editor. Part VI is loosely coupled teams, including a durable kanban board next to the blackboard. Speed is already in force by then.

### Part I: One agent

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 0 | Introduction | — | why Rust | what a harness is, and what Hermes is |
| 1 | First Contact | `steps/ch01-1-messages`, `ch01-2-provider`, `ch01-3-cli`, `ch01-4-errors` | tokio, structs, enums, traits, serde | 1.1 messages and serde; 1.2 `Provider`, one `Event` enum, and a latency mock that emits those events; 1.3 OpenRouter as the default client, the same events from the buffered call, usage plus `cost` accumulated per agent, and `ask`; 1.4 one request timeout and the edges of a buffered reply |
| 2 | Conversations | `steps/ch02-conversations` | ownership, private fields, `Result` | alternating history, a cache-stable system prompt, trim, JSON save/load, a `chat` REPL, and one per-session inbox the CLI submits to |
| 3 | Tools | `steps/ch03-tools` | traits, JSON Schema, a semaphore | a tool trait and registry shaped like MCP; named toolsets gated on env vars and checks; the model is told only about tools that passed; read-only calls run concurrently, writes run alone, both under a bound |
| 4 | MCP Client | `steps/ch04-mcp-client` | `rmcp`, stdio, HTTP | connect to MCP servers over stdio and streamable HTTP; MCP tools and native tools share one registry and one scheduler |
| 5 | The Agent Loop | `steps/ch05-agent-loop` | enums for replies | model, native and MCP tools, results, repeat; a cancellation token and a budget (tokens, time, cost) on every call; a small saved eval set later chapters re-run |
| 6 | The Turn State Machine | `steps/ch06-turn-state` | enum state machines | one `TurnState` and a tested transition table, immediately after the loop |
| 7 | MCP Server | `steps/ch07-mcp-server` | an MCP server | expose this harness's tools, and later agents and skills, so other clients can call them |
| 8 | Approvals and Guardrails | `steps/ch08-approvals` | a mutex, risk as an enum | risk levels on a tool call; modes for a human, a second model, or skipping the check; concurrent calls wait on one approval prompt, and the wait does not hold a tool-concurrency permit |
| 9 | Hooks and Plugins | `steps/ch09-hooks` | traits as extension points | before/after tool hooks, an API-request hook, rewriting a tool result, and a swappable memory backend; a hook may enable a toolset, and the availability check stays in the registry from chapter 3 |
| 10 | Sandboxed Execution | `steps/ch10-sandbox` | a child process, a snapshot | a code tool that runs outside the harness process, plus a file checkpoint and rewind |
| 11 | When Things Fail | `steps/ch11-when-things-fail` | matching on errors | classify failures, retry, fall back; MCP crashes, timeouts, reconnects; a timeout releases its concurrency permit |
| 12 | Providers | `steps/ch12-providers` | another adapter behind the same trait | native Anthropic and Bedrock adapters beside the chapter 1 OpenRouter client; OpenRouter fallback and routing (a `models` array and provider preferences); key rotation on the pooled client; a cheaper side model for summaries |
| 13 | Streaming | `steps/ch13-streaming` | folding a stream | server-sent deltas as the chapter 1 `Event` values, overlapped with safe tool execution, for each adapter |

**Performance budget**

- **Chapter 0.** State the rule: overhead is measured without model latency.
- **Chapter 1.** Four benches, each under 1 ms. 1.1: serde of 50 messages (measured 6.96 µs). 1.2: one mock call that also builds events (measured 235.7 ns) and 100 concurrent calls (measured 108.9 µs, target < 50 ms). 1.3: build an OpenRouter request and parse a response that carries `model`, `usage`, and `cost` (measured 2.133 µs). 1.4: parse a response with usage and one without (measured 792.7 ns). One HTTP client, pooled, HTTP/2 enabled. The default endpoint is OpenRouter.
- **Chapter 2.** The system prompt is byte-identical across the turns of a session. Per-turn overhead stays < 1 ms. Submitting one turn to the session inbox is < 1 µs. The provider borrows the history.
- **Chapter 3.** Eight read-only tools in one batch finish in about one call on the latency mock. A write runs alone. Building the available-tool list for a 50-tool registry is < 1 ms.
- **Chapter 4.** Eight concurrent mock MCP calls finish in about one round trip, on the same bound as native tools.
- **Chapter 5.** Loop-iteration overhead < 1 ms excluding the model. Checking the cancellation token or the budget is < 1 µs. A cancelled call does not start the next model request. Re-running the saved eval set on the mock stays inside the worker bound.
- **Chapter 6.** A state transition < 1 µs. Turn overhead stays < 1 ms.
- **Chapter 7.** Thirty-two in-flight calls accepted, eight executed at once.
- **Chapter 8.** A skip decision is < 1 µs. Concurrent approvals serialize onto one prompt. Waiting on a human releases the tool permit.
- **Chapter 9.** An empty before/after hook adds < 0.1 ms. An API-request hook does not copy the conversation. A memory-backend call that only records the turn enqueues in < 1 ms.
- **Chapter 10.** A warm sandbox accepts a job in < 50 ms. Checkpoint and rewind of a 1,000-path manifest are each < 10 ms.
- **Chapter 11.** A timeout cancels the in-flight call and releases its permit. Retries stay inside the bound.
- **Chapter 12.** Choosing the next native adapter, or the next OpenRouter route, adds < 1 ms, excluding the failed call. Rotation reuses the pooled client from chapter 1. A side-model summary does not run on the user turn unless that turn asked for it.
- **Chapter 13.** Assembling a streamed reply from chapter 1 events adds < 1 ms versus a buffered reply, excluding model time.

### Part II: Learning

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 14 | Sessions on Disk | `steps/ch14-sessions` | SQLite, WAL | an append-only session store and a way to resume |
| 15 | Memory Files | `steps/ch15-memory` | bounded text, atomic writes | `MEMORY.md` and `USER.md`, a `memory` tool, a frozen snapshot in the system prompt, injection scanning; the file backend behind the trait from chapter 9 |
| 16 | Recall | `steps/ch16-recall` | FTS5 | a `session_search` tool: discover, scroll, read, browse |
| 17 | Using Skills | `steps/ch17-skills` | a directory of markdown | `SKILL.md` (agentskills.io-compatible), a cache-stable `<available_skills>` index, `skills_list` / `skill_view` |
| 18 | Writing Skills | `steps/ch18-skill-writing` | patch and validate | create, patch, delete, and supporting files, with a scanner, the approval gate from chapter 8, and read-before-write |
| 19 | The Reflection Loop | `steps/ch19-reflection` | a supervised child task | nudge counters, the post-turn hook from chapter 9, a forked reviewer with only skill and memory tools |
| 20 | Curation | `steps/ch20-curation` | a scheduled job | usage telemetry, stale then archived, pinning, backups, optional consolidation |

**Performance budget**

- **Chapter 14.** One local WAL append of a turn < 1 ms. FTS updates are a background task. The chapter re-runs the chapter 5 eval set.
- **Chapter 15.** The frozen snapshot is built once and shared by `Arc`. A memory write does not rewrite the system prompt. Enqueue of a background write < 1 ms.
- **Chapter 16.** Discover over 10,000 messages < 20 ms, and only when the model calls the tool.
- **Chapter 17.** Load and render 500 skills < 10 ms, once per session. The index is `Arc` and byte-stable.
- **Chapter 18.** Security scan of one skill < 5 ms. A write the model did not ask for is a background task.
- **Chapter 19.** Spawning the reviewer adds < 1 ms to turn finalize. Cancelling it when the user returns takes < 50 ms.
- **Chapter 20.** Curation never runs on the request path.

### Part III: The team patterns

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 21 | The Sequential Pipeline | `steps/ch21-pipeline` | an agent as a value you can pass around | outline, draft, critique, revise; each stage is an agent run in order |
| 22 | The Supervisor | `steps/ch22-supervisor` | the tool trait, used for agents | the default team: a manager, specialists called as tools and fanned out under a bound, one `Arc`-shared context object; the reflection loop learns from team runs off the hot path |
| 23 | Handoffs | `steps/ch23-handoffs` | a pointer to the current agent | routing and ownership transfer; handoff-as-tool; a max-handoff limit; context filtered before the next agent sees it |
| 24 | Fan-out and Gather | `steps/ch24-fan-out` | a worker limit, an aggregator | the dedicated gather: the supervisor already runs specialists in parallel; this chapter adds the worker limit's tests and the aggregator |
| 25 | The Critic | `steps/ch25-critic` | the same tool trait | an evaluator agent, both as a tool and as a final gate |
| 26 | Composing Patterns | `steps/ch26-composition` | nesting the patterns above | a supervisor that fans out and uses a handoff only when ownership should move |

**Performance budget**

- **Chapter 21.** The next stage receives `Arc` context. No full transcript copy.
- **Chapter 22.** Up to eight specialists at once, each with `Arc` context. Eight latency-mock specialists finish in about one specialist's time.
- **Chapter 23.** Filtering a 100-message transcript for a handoff < 1 ms.
- **Chapter 24.** The worker limit holds. One hundred mock subagents can be scheduled without an unbounded queue.
- **Chapter 25.** The critic shares `Arc` context and takes one permit from the same bound.
- **Chapter 26.** Nested fan-out cannot exceed the worker limit.

### Part IV: Making it robust

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 27 | Managing Context | `steps/ch27-context` | slicing a `Vec` | compression, structured summaries from the side model in chapter 12, `[SKILL_PRUNED]` markers, a memory flush before compression |
| 28 | Agents as Actors | `steps/ch28-actors` | bounded channels | one tokio task per agent; the mailbox is the session inbox from chapter 2; steering and backpressure |
| 29 | Supervision and Restart | `steps/ch29-supervision` | atomics | restart a failed child. The cancellation token and the budget are already on the call from chapter 5 |

**Performance budget**

- **Chapter 27.** Compression and the memory flush run between turns or in the background. The hot path sends the `Arc` snapshot.
- **Chapter 28.** The session inbox capacity is 64. One hundred or more mock agents at once. A full inbox applies backpressure.
- **Chapter 29.** Restarting a failed child adds < 1 ms, excluding the child's work.

### Part V: Running as a service

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 30 | Profiles | `steps/ch30-profiles` | config, a secret map | several personas in one config, each with its own secrets, kept out of the system prompt |
| 31 | The Gateway | `steps/ch31-gateway` | the session inbox | platform adapters submit to the chapter 2 inbox; a mid-turn message becomes an interrupt or a steer; streaming folds chapter 1 events into one message |
| 32 | Cron | `steps/ch32-cron` | a durable claim | at-most-once runs across crashes, catch-up, a watchdog, and chaining the next job by submitting to the same inbox |
| 33 | Editor Integration | `steps/ch33-acp` | a protocol adapter | ACP (Agent Client Protocol) as another client of the chapter 2 session inbox |

**Performance budget**

- **Chapter 30.** Resolving a profile and its secrets is < 1 ms. Secrets are not copied into the prompt.
- **Chapter 31.** Enqueue of a platform message is < 1 ms. A steer is a push onto a bounded queue. Streaming updates one message.
- **Chapter 32.** A crash retries a claimed job at most once. Scanning 1,000 due jobs is < 10 ms. The watchdog and a chained job stay off the request path.
- **Chapter 33.** Translating an ACP update onto the session queue is < 1 ms.

### Part VI: Loosely coupled teams

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 34 | The Blackboard | `steps/ch34-blackboard` | shared state behind a lock | shared state, agents woken by a condition, a thin scheduler |
| 35 | The Kanban Board | `steps/ch35-kanban` | a compare-and-swap claim | a durable board; separate processes claim, complete, review, or block a task |
| 36 | The Message Bus | `steps/ch36-message-bus` | bounded channels | peer-to-peer messaging with backpressure |
| 37 | Shared Team Skills | `steps/ch37-team-skills` | a single writer | a layered skill store, `skill_propose` on the bus, a librarian agent, `skills.updated`, a versioned library, `TEAM.md` |
| 38 | A2A for External Agents | `steps/ch38-a2a` | traits at the process boundary | A2A only for agents outside this process; MCP already covers tools, agents, and skills we expose |
| 39 | Seeing Inside | `steps/ch39-tracing` | tracing | spans on the chapter 1 events, the same criterion bench with tracing on, a capstone research team that learns across runs |
| 40 | Evals | `steps/ch40-evals` | a batch over the benches | re-run the saved tasks from chapter 5, plus trajectories, scored with the chapter 1 and chapter 5 benches, still with no fail threshold |

**Performance budget**

- **Chapter 34.** A claim < 1 ms with 100 waiters. The lock is not held across a model call.
- **Chapter 35.** Claim, complete, review, or block is one transaction < 5 ms. Two processes cannot hold the same task.
- **Chapter 36.** Publish of one small envelope < 1 ms at 100 agents. A slow subscriber hits a bounded channel.
- **Chapter 37.** `skill_propose` enqueues in < 1 ms. The index swap is an `Arc` swap. Workers do not wait on the librarian.
- **Chapter 38.** Remote calls use the HTTP pool and the same bound. Cancel reaches an in-flight call in < 50 ms.
- **Chapter 39.** A span per event adds < 0.1 ms. The chapter 1 bench still meets its budget with tracing on. The capstone runs 100 or more concurrent mock agents.
- **Chapter 40.** Re-running the saved eval set, and one hundred mock trajectories, stays inside the worker bound. Appending one trajectory event is < 1 ms.

Only chapters 1 and 2 have step crates so far. Chapter 1 is four cumulative sections, each with a criterion bench. Chapter 5 starts a saved eval set that later chapters re-run, and chapter 40 scores that set. Pull requests and pushes to `main` report the means in the Benchmarks workflow, upload one JSON artifact per commit, and chart the series when the job can publish to `gh-pages`. There is no fail threshold. Later rows are the plan, not empty crates. The same list is `.github/chapters.json`, which CI uses. Draft rows in `book/SUMMARY.md` are the chapters that are not written yet.

**Recurring sidebars:**

- "How Hermes does it": file paths and design notes from NousResearch/hermes-agent.
- Rust, introduced when the harness needs it.
- "[personal note]": the author's own experience while working through the chapter.
