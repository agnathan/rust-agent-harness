# Book outline

**Working title:** *Learning Rust by Building a Multi-Agent Harness*

**Alternative titles:**
- *Agents in Rust: From First Function to Agent Teams*
- *Fearless Agents*

**Audience:** Developers who know at least one other language (Python, JavaScript, Go) and are curious about AI agents, but are new to Rust. No systems-programming background is assumed.

**Promise:** By the end, the reader has built a working, tested, concurrent harness for teams of LLM agents. Along the way they learn Rust's core ideas (ownership, traits, error handling, async) through problems that actually show up in agent systems.

**Approach:** Every chapter matches one course lesson (`lessons/NN-*.md`) and follows the same shape:
1. a problem from agent engineering
2. how Hermes Agent (Nous Research, Python) solves it
3. the Rust concepts needed
4. building it with tests
5. what was learned, including the author's honest notes on what was hard

| Ch | Title | Lesson | Rust concepts | Agent concept |
|---|---|---|---|---|
| 0 | Introduction: Why I'm learning Rust by building agents | — | why Rust | what Hermes is, what a harness is |
| 1 | Hello, Agent | 01 | cargo, functions, strings, tests | slash commands |
| 2 | Speaking in Messages | 02 | structs, enums, impl, match | roles, messages, tool calls |
| 3 | Who Owns the Conversation? | 03 | ownership, borrowing, lifetimes | context trimming |
| 4 | When Things Go Wrong | 04 | Result, `?`, custom errors | command parsing, error classification |
| 5 | The Wire | 05 | serde, generics, From | OpenAI JSON, tool schemas |
| 6 | Tools Are Traits | 06 | traits, trait objects, HashMap | tool registry |
| 7 | The Loop | 07 | trait-based mocks, VecDeque | the agent loop, budgets |
| 8 | Talking to a Real Model | 08 | reqwest, testable boundaries | providers |
| 9 | Waiting Well | 09 | async, tokio, JoinSet, timeouts | I/O-bound agents |
| 10 | Tokens as They Arrive | 10 | channels, let-else | streaming, SSE |
| 11 | Making State Explicit | 11 | enum state machines | the turn lifecycle, retries, compaction |
| 12 | Doing Many Things at Once, Safely | 12 | Arc, Semaphore, async-trait | parallel tool calls |
| 13 | Every Agent an Actor | 13 | mpsc, oneshot, backpressure | inbox, steering |
| 14 | Stopping Things Properly | 14 | CancellationToken, select!, atomics | delegation, budgets, supervision |
| 15 | Agents Talking to Agents | 15 | broadcast, Mutex rules | message bus, team topologies |
| 16 | Remember, Observe, Measure | 16 | rusqlite, tracing, criterion | persistence, observability, performance |
| 17 | Epilogue: What I'd build next | — | — | distributed agents, sandboxes, plugins |

**Recurring sidebars:**
- "How Hermes does it": file paths and design notes.
- "Rust vs Python": what each one makes easy or hard.
- "[personal note]": the author's own experience while working through the lesson.
