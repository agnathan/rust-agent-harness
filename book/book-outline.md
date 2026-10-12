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
| 3 | Tools | — | traits, JSON Schema | a tool trait, two tools, a registry the model can be shown |
| 4 | The Agent Loop | — | enums for replies | model, tool calls, results, repeat, with an iteration budget |
| 5 | When the Call Fails | — | matching on errors | classify HTTP failures, retry, fall back once |
| 6 | Streaming | — | folding a stream | server-sent deltas gathered into one reply |
| 7 | An Explicit Turn | — | enum state machines | one `TurnState` and a tested transition table |
| 8 | Parallel Tools | — | `Send`, a semaphore | read-only calls share a batch; writes run alone |
| 9 | Context Compression | — | slicing a `Vec` | keep the system prompt and the tail; collapse the middle |
| 10 | Sessions and Memory | — | SQLite | sessions in a database, notes in a memory file |
| 11 | The Agent as an Actor | — | channels | one task per agent, an inbox, steering |
| 12 | Cancellation and Supervision | — | atomics | cancel a parent and the children stop; restart a failed child |
| 13 | Delegation | — | — | a child with a smaller toolbox returns only a summary |
| 14 | A Message Bus | — | — | topics for the team, and a direct message to one peer |
| 15 | Teams | — | — | a shared task board and round-robin assignment |
| 16 | Observe, Measure, Capstone | — | tracing | a span around the turn, a small benchmark, a team demo |
| 17 | Epilogue: What I'd build next | — | — | sandboxes, plugins, more than one machine |

Only chapter 1 has a step crate so far. Later rows are the plan, not empty crates.

**Recurring sidebars:**

- "How Hermes does it": file paths and design notes from NousResearch/hermes-agent.
- Rust, introduced when the harness needs it.
- "[personal note]": the author's own experience while working through the chapter.
