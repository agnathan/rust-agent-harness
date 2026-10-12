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

The book is five parts and thirty chapters. MCP is first-class from Part I: tools are shaped like MCP tools, the harness is both an MCP client and an MCP server, and later chapters call those servers and can publish skills through them. Part II is learning: one agent gets better with use before the book builds a team. The design for skills, memory, recall, reflection, curation, and the team librarian is [`docs/self-improvement.md`](../docs/self-improvement.md). Part III is the team patterns. The default team is a supervisor that calls other agents as tools and shares one context object. The rules for choosing a pattern are in [`docs/team-patterns.md`](../docs/team-patterns.md).

### Part I: One agent

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 0 | Introduction | — | why Rust | what a harness is, and what Hermes is |
| 1 | First Contact | `steps/ch01-first-contact` | cargo, structs, enums, traits, serde, a little async | `Message`, `Provider`, OpenAI-compatible and mock providers, `ask` |
| 2 | Conversations | `steps/ch02-conversations` | ownership, private fields, `Result` | alternating history, stable system prompt, trim, JSON save/load, `chat` REPL |
| 3 | Tools | `steps/ch03-tools` | traits, JSON Schema | a tool trait and registry shaped like MCP: name, description, input schema, content results |
| 4 | MCP Client | `steps/ch04-mcp-client` | `rmcp`, stdio, HTTP | connect to MCP servers over stdio and streamable HTTP; MCP tools and native tools share one registry |
| 5 | The Agent Loop | `steps/ch05-agent-loop` | enums for replies | model, native and MCP tools, results, repeat, with an iteration budget |
| 6 | MCP Server | `steps/ch06-mcp-server` | an MCP server | expose this harness's tools, and later agents and skills, so other clients can call them |
| 7 | When Things Fail | `steps/ch07-when-things-fail` | matching on errors | classify failures, retry, fall back; MCP crashes, timeouts, reconnects |
| 8 | Streaming | `steps/ch08-streaming` | folding a stream | server-sent deltas gathered into one reply |

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

### Part III: The team patterns

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 16 | The Sequential Pipeline | `steps/ch16-pipeline` | an agent as a value you can pass around | outline, draft, critique, revise; each stage is an agent run in order |
| 17 | The Supervisor | `steps/ch17-supervisor` | the tool trait, used for agents | the default team: a manager, specialists called as tools, one shared context object; the same reflection loop learns from team runs |
| 18 | Handoffs | `steps/ch18-handoffs` | a pointer to the current agent | routing and ownership transfer; handoff-as-tool; a max-handoff limit; context filtered before the next agent sees it |
| 19 | Fan-out and Gather | `steps/ch19-fan-out` | tokio, a worker limit | parallel specialists, then an aggregator |
| 20 | The Critic | `steps/ch20-critic` | the same tool trait | an evaluator agent, both as a tool and as a final gate |
| 21 | Composing Patterns | `steps/ch21-composition` | nesting the patterns above | a supervisor that fans out and uses a handoff only when ownership should move |

### Part IV: Making it robust and fast

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 22 | The Turn State Machine | `steps/ch22-turn-state` | enum state machines | one `TurnState` and a tested transition table |
| 23 | Managing Context | `steps/ch23-context` | slicing a `Vec` | compression, structured summaries, `[SKILL_PRUNED]` markers, a memory flush before compression |
| 24 | Agents as Actors | `steps/ch24-actors` | channels | one tokio task per agent, an inbox, steering |
| 25 | Cancellation and Supervision | `steps/ch25-supervision` | atomics | cancel a parent and the children stop; restart a failed child |

### Part V: Loosely coupled teams

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 26 | The Blackboard | `steps/ch26-blackboard` | shared state behind a lock | shared state, agents woken by a condition, a thin scheduler |
| 27 | The Message Bus | `steps/ch27-message-bus` | channels | peer-to-peer messaging |
| 28 | Shared Team Skills | `steps/ch28-team-skills` | a single writer | a layered skill store, `skill_propose` on the bus, a librarian agent, `skills.updated`, a versioned library, `TEAM.md` |
| 29 | A2A for External Agents | `steps/ch29-a2a` | traits at the process boundary | A2A only for agents outside this process; MCP already covers tools, agents, and skills we expose |
| 30 | Seeing Inside | `steps/ch30-tracing` | tracing | spans, a benchmark, a capstone research team that learns across runs |

Only chapters 1 and 2 have step crates so far. Later rows are the plan, not empty crates. The same list is `.github/chapters.json`, which CI uses. Draft rows in `book/SUMMARY.md` are the chapters that are not written yet.

**Recurring sidebars:**

- "How Hermes does it": file paths and design notes from NousResearch/hermes-agent.
- Rust, introduced when the harness needs it.
- "[personal note]": the author's own experience while working through the chapter.
