# Book outline

**Working title:** *Learning Rust by Building a Multi-Agent Harness*

**Alternative titles:**
- *Agents in Rust: From First Function to Agent Teams*
- *Fearless Agents*

**Audience:** Developers who know at least one other language (Python, JavaScript, Go) and are curious about AI agents, but are new to Rust. No systems-programming background is assumed.

**Promise:** By the end, the reader has built a working, tested, concurrent harness for teams of LLM agents. Rust shows up in short sidebars at the moment a harness problem needs it.

**Approach:** Every chapter builds one more piece of the same harness. The finished code for that chapter is a step crate under `steps/chNN-<slug>/`. The reader's own copy grows in `my-harness/`. Each chapter follows the same shape:

1. a problem from agent engineering
2. how Hermes Agent (Nous Research, Python) solves it
3. the Rust needed for this piece, in a short sidebar
4. building it, with the milestone tests in `my-harness/tests/chNN.rs`
5. what was learned, including `[personal note]` placeholders for the author's own experience

The book is five parts and thirty chapters. Performance is a budget on every chapter, not a late part: async tokio from chapter 1, concurrent tools from chapter 3, and learning work off the hot path. The numbers are in [`docs/performance.md`](../docs/performance.md). MCP is first-class from Part I: tools are shaped like MCP tools, the harness is both an MCP client and an MCP server, and later chapters call those servers and can publish skills through them. Part II is learning: one agent gets better with use before the book builds a team. The design for skills, memory, recall, reflection, curation, and the team librarian is [`docs/self-improvement.md`](../docs/self-improvement.md). Part III is the team patterns. The default team is a supervisor that calls other agents as tools, fans them out under a bound, and shares one context object by `Arc`. The rules for choosing a pattern are in [`docs/team-patterns.md`](../docs/team-patterns.md). Part IV is making that system robust. Speed is already in force by then.

### Part I: One agent

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 0 | Introduction | — | why Rust | what a harness is, and what Hermes is |
| 1 | First Contact | `steps/ch01-first-contact` | tokio, structs, enums, traits, serde | `Message`, `Provider`, one reused HTTP client, a mock with configurable latency, a criterion bench, `ask` |
| 2 | Conversations | `steps/ch02-conversations` | ownership, private fields, `Result` | alternating history, a system prompt that stays byte-stable for prompt caching, trim, JSON save/load, `chat` REPL |
| 3 | Tools | `steps/ch03-tools` | traits, JSON Schema, a semaphore | a tool trait and registry shaped like MCP; read-only calls run concurrently, writes run alone, both under a bound |
| 4 | MCP Client | `steps/ch04-mcp-client` | `rmcp`, stdio, HTTP | connect to MCP servers over stdio and streamable HTTP; MCP tools and native tools share one registry and one scheduler |
| 5 | The Agent Loop | `steps/ch05-agent-loop` | enums for replies | model, native and MCP tools, results, repeat, with an iteration budget; streaming overlaps safe tool calls; the criterion bench grows |
| 6 | MCP Server | `steps/ch06-mcp-server` | an MCP server | expose this harness's tools, and later agents and skills, so other clients can call them |
| 7 | When Things Fail | `steps/ch07-when-things-fail` | matching on errors | classify failures, retry, fall back; MCP crashes, timeouts, reconnects; a timeout releases its concurrency permit |
| 8 | Streaming | `steps/ch08-streaming` | folding a stream | server-sent deltas gathered into one reply, overlapped with safe tool execution |

**Performance budget**

- **Chapter 0.** State the rule: overhead is measured without model latency.
- **Chapter 1.** Mean harness overhead < 1 ms per `complete` on the zero-latency mock. One HTTP client per provider. Mock latency is configurable and is not counted in that number.
- **Chapter 2.** The system prompt is byte-identical across the turns of a session. Per-turn overhead stays < 1 ms. The provider borrows the history.
- **Chapter 3.** Eight read-only tools in one batch finish in about one call on the latency mock. A write runs alone.
- **Chapter 4.** Eight concurrent mock MCP calls finish in about one round trip, on the same bound as native tools.
- **Chapter 5.** Loop-iteration overhead < 1 ms excluding the model. A safe tool starts while tokens are still arriving.
- **Chapter 6.** Thirty-two in-flight calls accepted, eight executed at once.
- **Chapter 7.** A timeout cancels the in-flight call and releases its permit. Retries stay inside the bound.
- **Chapter 8.** Assembling a streamed reply adds < 1 ms versus a buffered reply, excluding model time.

### Part II: Learning

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 9 | Sessions on Disk | `steps/ch09-sessions` | SQLite, WAL | an append-only session store and a way to resume |
| 10 | Memory Files | `steps/ch10-memory` | bounded text, atomic writes | `MEMORY.md` and `USER.md`, a `memory` tool, a frozen snapshot in the system prompt, injection scanning |
| 11 | Recall | `steps/ch11-recall` | FTS5 | a `session_search` tool: discover, scroll, read, browse |
| 12 | Using Skills | `steps/ch12-skills` | a directory of markdown | `SKILL.md` (agentskills.io-compatible), a cache-stable `<available_skills>` index, `skills_list` / `skill_view` |
| 13 | Writing Skills | `steps/ch13-skill-writing` | patch and validate | create, patch, delete, and supporting files, with a scanner, an approval gate, and read-before-write |
| 14 | The Reflection Loop | `steps/ch14-reflection` | a supervised child task | nudge counters, a post-turn `TurnHook`, a forked reviewer with only skill and memory tools |
| 15 | Curation | `steps/ch15-curation` | a scheduled job | usage telemetry, stale then archived, pinning, backups, optional consolidation |

**Performance budget**

- **Chapter 9.** One local WAL append of a turn < 1 ms. FTS updates are a background task.
- **Chapter 10.** The frozen snapshot is built once and shared by `Arc`. A memory write does not rewrite the system prompt. Enqueue of a background write < 1 ms.
- **Chapter 11.** Discover over 10,000 messages < 20 ms, and only when the model calls the tool.
- **Chapter 12.** Load and render 500 skills < 10 ms, once per session. The index is `Arc` and byte-stable.
- **Chapter 13.** Security scan of one skill < 5 ms. A write the model did not ask for is a background task.
- **Chapter 14.** Spawning the reviewer adds < 1 ms to turn finalize. Cancelling it when the user returns takes < 50 ms.
- **Chapter 15.** Curation never runs on the request path.

### Part III: The team patterns

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 16 | The Sequential Pipeline | `steps/ch16-pipeline` | an agent as a value you can pass around | outline, draft, critique, revise; each stage is an agent run in order |
| 17 | The Supervisor | `steps/ch17-supervisor` | the tool trait, used for agents | the default team: a manager, specialists called as tools and fanned out under a bound, one `Arc`-shared context object; the reflection loop learns from team runs off the hot path |
| 18 | Handoffs | `steps/ch18-handoffs` | a pointer to the current agent | routing and ownership transfer; handoff-as-tool; a max-handoff limit; context filtered before the next agent sees it |
| 19 | Fan-out and Gather | `steps/ch19-fan-out` | a worker limit, an aggregator | the dedicated gather: the supervisor already runs specialists in parallel; this chapter adds the worker limit's tests and the aggregator |
| 20 | The Critic | `steps/ch20-critic` | the same tool trait | an evaluator agent, both as a tool and as a final gate |
| 21 | Composing Patterns | `steps/ch21-composition` | nesting the patterns above | a supervisor that fans out and uses a handoff only when ownership should move |

**Performance budget**

- **Chapter 16.** The next stage receives `Arc` context. No full transcript copy.
- **Chapter 17.** Up to eight specialists at once, each with `Arc` context. Eight latency-mock specialists finish in about one specialist's time.
- **Chapter 18.** Filtering a 100-message transcript for a handoff < 1 ms.
- **Chapter 19.** The worker limit holds. One hundred mock subagents can be scheduled without an unbounded queue.
- **Chapter 20.** The critic shares `Arc` context and takes one permit from the same bound.
- **Chapter 21.** Nested fan-out cannot exceed the worker limit.

### Part IV: Making it robust

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 22 | The Turn State Machine | `steps/ch22-turn-state` | enum state machines | one `TurnState` and a tested transition table |
| 23 | Managing Context | `steps/ch23-context` | slicing a `Vec` | compression, structured summaries, `[SKILL_PRUNED]` markers, a memory flush before compression |
| 24 | Agents as Actors | `steps/ch24-actors` | bounded channels | one tokio task per agent, a bounded inbox, steering, backpressure |
| 25 | Cancellation and Supervision | `steps/ch25-supervision` | atomics | cancel a parent and the children stop; restart a failed child |

**Performance budget**

- **Chapter 22.** A state transition < 1 µs. Turn overhead stays < 1 ms.
- **Chapter 23.** Compression and the memory flush run between turns or in the background. The hot path sends the `Arc` snapshot.
- **Chapter 24.** Inbox capacity 64. One hundred or more mock agents at once. A full inbox applies backpressure.
- **Chapter 25.** Cancel the root and 100 mock children stop in < 50 ms.

### Part V: Loosely coupled teams

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 26 | The Blackboard | `steps/ch26-blackboard` | shared state behind a lock | shared state, agents woken by a condition, a thin scheduler |
| 27 | The Message Bus | `steps/ch27-message-bus` | bounded channels | peer-to-peer messaging with backpressure |
| 28 | Shared Team Skills | `steps/ch28-team-skills` | a single writer | a layered skill store, `skill_propose` on the bus, a librarian agent, `skills.updated`, a versioned library, `TEAM.md` |
| 29 | A2A for External Agents | `steps/ch29-a2a` | traits at the process boundary | A2A only for agents outside this process; MCP already covers tools, agents, and skills we expose |
| 30 | Seeing Inside | `steps/ch30-tracing` | tracing | spans, the same criterion bench with tracing on, a capstone research team that learns across runs |

**Performance budget**

- **Chapter 26.** A claim < 1 ms with 100 waiters. The lock is not held across a model call.
- **Chapter 27.** Publish of one small envelope < 1 ms at 100 agents. A slow subscriber hits a bounded channel.
- **Chapter 28.** `skill_propose` enqueues in < 1 ms. The index swap is an `Arc` swap. Workers do not wait on the librarian.
- **Chapter 29.** Remote calls use the HTTP pool and the same bound. Cancel reaches an in-flight call in < 50 ms.
- **Chapter 30.** A span per turn adds < 0.1 ms. The chapter 1 bench still meets its budget with tracing on. The capstone runs 100 or more concurrent mock agents.

Only chapters 1 and 2 have step crates so far. Chapter 1 already carries the criterion bench (`cargo bench -p ch01-harness-core --bench turn_overhead`). Chapter 5 extends it. Pushes to `main` report that bench in the Benchmarks workflow, with no fail threshold. Later rows are the plan, not empty crates. The same list is `.github/chapters.json`, which CI uses. Draft rows in `book/SUMMARY.md` are the chapters that are not written yet.

**Recurring sidebars:**

- "How Hermes does it": file paths and design notes from NousResearch/hermes-agent.
- Rust, introduced when the harness needs it.
- "[personal note]": the author's own experience while working through the chapter.
