# rust-agent-harness

[![CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml/badge.svg)](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml)

An AI agent harness in Rust, with a book on how to build it step by step.

This is a hands-on course for people **new to Rust**. You learn the language by growing one harness, not by a tour of the language. The design is informed by Nous Research's [Hermes Agent](https://github.com/NousResearch/hermes-agent) (Python). Each chapter points at the Hermes files that solve the same problem.

## Layout

```
steps/chNN-<slug>/   finished harness at the end of that chapter (cumulative)
my-harness/          your crate; milestone tests start out failing
book/                mdBook source and social drafts
```

A step is the code as of that chapter, library plus CLI, with its tests. Later chapters are new copies under `steps/`, so chapter 1 still builds after chapter 2 exists. They share one cargo workspace at the repo root. Package names carry the section (`ch01-1-harness-core`, `ch01-3-harness-cli`, `ch02-harness-cli`) and the binaries are `ch01-harness` and `ch02-harness`, so two steps can be built together. The library each test imports is still `harness_core`.

`my-harness/` is where the implementation happens. `tests/ch01_1.rs`, `ch01_2.rs`, `ch01_3.rs`, and `tests/ch02.rs` are the milestones. They compile, then panic with `todo!` until you replace the stubs. Finished snapshots stay in `steps/`. There is no solutions directory.

## How to start

1. Install Rust with [rustup](https://rustup.rs). You need 1.85 or newer (`rustc --version`).
2. Add the usual components: `rustup component add rustfmt clippy`.
3. Read [Chapter 1](book/chapter-01-first-contact.md), then make the milestone fail for the right reason and replace the stubs:

   ```bash
   cargo test -p my-harness --test ch01_1 --test ch01_2 --test ch01_3
   ```

4. Run the finished chapter if you want to see the behavior:

   ```bash
   cargo test -p ch01-1-harness-core
   cargo test -p ch01-2-harness-core
   cargo test -p ch01-3-harness-core -p ch01-3-harness-cli
   cargo run -p ch01-3-harness-cli -- --mock ask "Say hello"
   ```

5. Chapter 2 is the same shape: `cargo test -p my-harness --test ch02`, and `cargo run -p ch02-harness-cli -- --mock chat`.

`OPENAI_API_KEY`, `OPENAI_BASE_URL` (default `https://api.openai.com/v1`), and `HARNESS_MODEL` (default `gpt-4o-mini`) configure a real server. `--mock` uses the offline provider.

Thirty chapters in five parts. Performance is a budget on every chapter, starting with tokio in chapter 1. The targets are [docs/performance.md](docs/performance.md). MCP is first-class: tools are MCP-shaped (chapter 3), the harness is an MCP client (chapter 4, via `rmcp`) and an MCP server (chapter 6). Part II is learning, and that work stays off the hot path: sessions, memory, recall, skills, reflection, and curation. The design is [docs/self-improvement.md](docs/self-improvement.md). Which team pattern to use is [docs/team-patterns.md](docs/team-patterns.md). The default is a supervisor that calls specialists as tools and fans them out under a bound.

**Part I: One agent**

| Ch | Title | Step | You add |
|---|---|---|---|
| 0 | Introduction | — | why a harness, and what Hermes is |
| 1 | First Contact | `steps/ch01-1-messages`, `ch01-2-provider`, `ch01-3-cli` | messages, a latency mock on tokio, one pooled HTTP client, `ask` |
| 2 | Conversations | `steps/ch02-conversations` | a real conversation, a cache-stable system prompt, and a `chat` REPL |
| 3 | Tools | `steps/ch03-tools` | an MCP-shaped tool trait; read-only calls run concurrently, writes run alone |
| 4 | MCP Client | `steps/ch04-mcp-client` | `rmcp` over stdio and streamable HTTP; MCP and native tools share one scheduler |
| 5 | The Agent Loop | `steps/ch05-agent-loop` | model, tools, results, repeat; streaming overlaps safe tool calls |
| 6 | MCP Server | `steps/ch06-mcp-server` | expose this harness's tools, and later agents and skills, to other clients |
| 7 | When Things Fail | `steps/ch07-when-things-fail` | classify failures, retry, fall back, including MCP crashes and reconnects |
| 8 | Streaming | `steps/ch08-streaming` | tokens as they arrive |

**Part II: Learning**

| Ch | Title | Step | You add |
|---|---|---|---|
| 9 | Sessions on Disk | `steps/ch09-sessions` | SQLite sessions (append-only, WAL) and resume |
| 10 | Memory Files | `steps/ch10-memory` | `MEMORY.md` / `USER.md`, the `memory` tool, a frozen system-prompt snapshot |
| 11 | Recall | `steps/ch11-recall` | FTS5 `session_search`: discover, scroll, read, browse |
| 12 | Using Skills | `steps/ch12-skills` | `SKILL.md`, a cache-stable index, `skills_list` / `skill_view` |
| 13 | Writing Skills | `steps/ch13-skill-writing` | create, patch, delete, a scanner, approval, read-before-write |
| 14 | The Reflection Loop | `steps/ch14-reflection` | nudge counters and a forked reviewer with only skill and memory tools |
| 15 | Curation | `steps/ch15-curation` | stale to archived, pinning, backups, optional consolidation |

**Part III: The team patterns**

| Ch | Title | Step | You add |
|---|---|---|---|
| 16 | The Sequential Pipeline | `steps/ch16-pipeline` | outline, draft, critique, revise; an agent as a reusable value |
| 17 | The Supervisor | `steps/ch17-supervisor` | the default team: manager, agents-as-tools, fan-out under a bound, `Arc` context |
| 18 | Handoffs | `steps/ch18-handoffs` | ownership transfer, handoff-as-tool, max-handoff limit, context filtering |
| 19 | Fan-out and Gather | `steps/ch19-fan-out` | parallel specialists, a worker limit, an aggregator |
| 20 | The Critic | `steps/ch20-critic` | an evaluator as a tool and as a final gate |
| 21 | Composing Patterns | `steps/ch21-composition` | a supervisor that fans out and hands off only when ownership moves |

**Part IV: Making it robust**

| Ch | Title | Step | You add |
|---|---|---|---|
| 22 | The Turn State Machine | `steps/ch22-turn-state` | an explicit turn |
| 23 | Managing Context | `steps/ch23-context` | compression, summaries, `[SKILL_PRUNED]`, a memory flush |
| 24 | Agents as Actors | `steps/ch24-actors` | one task per agent, a bounded inbox, backpressure |
| 25 | Cancellation and Supervision | `steps/ch25-supervision` | cancel trees and restarts |

**Part V: Loosely coupled teams**

| Ch | Title | Step | You add |
|---|---|---|---|
| 26 | The Blackboard | `steps/ch26-blackboard` | shared state, condition-triggered agents, a thin scheduler |
| 27 | The Message Bus | `steps/ch27-message-bus` | peer-to-peer messaging |
| 28 | Shared Team Skills | `steps/ch28-team-skills` | a layered store, `skill_propose`, a librarian as the single writer |
| 29 | A2A for External Agents | `steps/ch29-a2a` | A2A only for agents outside this process |
| 30 | Seeing Inside | `steps/ch30-tracing` | tracing, benchmarks, a capstone team that learns across runs |

Chapters 1 and 2 are the only step crates in the tree. The rest are added when those chapters are written. The same list drives CI (`.github/chapters.json`).

The book builds with [mdBook](https://rust-lang.github.io/mdBook/): `mdbook build` in `book/` (output in `book/_build/`, not committed).

## Continuous integration

[CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml) runs on every push to `main` and on pull requests:

- `cargo fmt --check` and `cargo clippy --workspace --all-targets` (warnings do not fail the build)
- `cargo test -p …` for every step, each step as its own job, on Ubuntu
- the same tests on Ubuntu, macOS, and Windows for steps whose files changed ([paths-filter](https://github.com/dorny/paths-filter))
- an mdBook build of `book/`

[Benchmarks](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml) run on pull requests and on pushes to `main`. They execute the three chapter 1 criterion benches on Ubuntu and write the means to the job summary. A slow result does not fail the run.

`my-harness` milestone tests are not part of that required build. They run in [My harness](https://github.com/agnathan/rust-agent-harness/actions/workflows/my-harness.yml).

### Milestone tests across operating systems

The workflow installs `cargo-nextest` and runs `cargo nextest run -p my-harness --test chNN` (and falls back to `cargo test` if nextest is unavailable). Unimplemented chapters show as **fail**. JUnit XML is uploaded.

**Push and pull request.** A change under `my-harness/` tests the affected chapters on Ubuntu, macOS, and Windows. This mode is a report: failing stubs do not fail the workflow.

**Manual run.** Actions → My harness → Run workflow.

| Input | Values |
|---|---|
| `chapters` | `all` (default), `1`, `1-30`, or `9,12,14`. Chapters 1–30 are the plan; a chapter runs only after its `tests/chNN.rs` exists (today: 1 and 2) |
| `os` | `all` (default), `ubuntu`, `macos`, or `windows` |

A manual run fails if any selected chapter fails.

## License

[MIT](LICENSE) © 2026 Daniel Holmlund
