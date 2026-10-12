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

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 0 | Introduction: Why I'm learning Rust by building agents | — | why Rust | what a harness is, and what Hermes is |
| 1 | First Contact | `steps/ch01-first-contact` | cargo, structs, enums, traits, serde, a little async | `Message`, `Provider`, OpenAI-compatible and mock providers, `ask` |
| 2 | Conversations | `steps/ch02-conversations` | ownership, private fields, `Result` | alternating history, stable system prompt, trim, JSON save/load, `chat` REPL |
| 3 | Tools | `steps/ch03-tools` | traits, JSON Schema | a tool trait, two tools, a registry the model can be shown |
| 4 | The Agent Loop | `steps/ch04-agent-loop` | enums for replies | model, tool calls, results, repeat, with an iteration budget |
| 5 | Handoffs | `steps/ch05-handoffs` | enums, a pointer to the current agent | Swarm / OpenAI Agents SDK shape: an agent is instructions + tools + handoff targets; a handoff is a tool; a current-agent pointer; a max-handoff limit; context filtered before the next agent sees it |
| 6 | Agents as Tools | `steps/ch06-agents-as-tools` | the same tool trait, used for agents | a manager calls specialist agents as tools and synthesizes their results |
| 7 | When Things Fail | `steps/ch07-when-things-fail` | matching on errors | classify HTTP failures, retry, fall back once |
| 8 | Streaming | `steps/ch08-streaming` | folding a stream | server-sent deltas gathered into one reply |
| 9 | Turn State Machine | `steps/ch09-turn-state` | enum state machines | one `TurnState` and a tested transition table |
| 10 | Parallel Tools and Fan-out/Gather | `steps/ch10-parallel-tools` | `Send`, a semaphore | read-only calls share a batch; writes run alone; fan-out and gather |
| 11 | Managing Context | `steps/ch11-context` | slicing a `Vec` | keep the system prompt and the tail; collapse the middle |
| 12 | Remembering | `steps/ch12-remembering` | SQLite | sessions in a database, notes in memory files |
| 13 | Agents as Actors | `steps/ch13-actors` | channels | one task per agent, an inbox, steering |
| 14 | Cancellation and Supervision | `steps/ch14-supervision` | atomics | cancel a parent and the children stop; restart a failed child |
| 15 | The Blackboard | `steps/ch15-blackboard` | shared state behind a lock | a shared task board agents claim from |
| 16 | The Message Bus | `steps/ch16-message-bus` | channels | topics, peer messages, and team topologies |
| 17 | MCP and A2A | `steps/ch17-mcp-a2a` | traits at the process boundary | MCP for tools, A2A for remote agents |
| 18 | Tracing, Benchmarks, Capstone | `steps/ch18-tracing` | tracing | a span around the turn, a small benchmark, a team demo |

Only chapters 1 and 2 have step crates so far. Later rows are the plan, not empty crates. The same list is `.github/chapters.json`, which CI uses.

**Recurring sidebars:**

- "How Hermes does it": file paths and design notes from NousResearch/hermes-agent.
- Rust, introduced when the harness needs it.
- "[personal note]": the author's own experience while working through the chapter.
