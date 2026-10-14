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

A step is the code as of that chapter, library plus CLI, with its tests. Later chapters are new copies under `steps/`, so chapter 1 still builds after chapter 2 exists. They share one cargo workspace at the repo root. Package names carry the section (`ch01-1-harness-core`, `ch01-3-harness-cli`, `ch01-4-harness-cli`, `ch02-harness-cli`) and the binaries are `ch01-harness`, `ch01-4-harness`, and `ch02-harness`, so two steps can be built together. The library each test imports is still `harness_core`.

`my-harness/` is where the implementation happens. `tests/ch01_1.rs`, `ch01_2.rs`, `ch01_3.rs`, `ch01_4.rs`, and `tests/ch02.rs` are the milestones. They compile, then panic with `todo!` until you replace the stubs. Finished snapshots stay in `steps/`. There is no solutions directory.

## How to start

1. Install Rust with [rustup](https://rustup.rs). You need 1.85 or newer (`rustc --version`).
2. Add the usual components: `rustup component add rustfmt clippy`.
3. Read [Chapter 1](book/chapter-01-first-contact.md), then make the milestone fail for the right reason and replace the stubs:

   ```bash
   cargo test -p my-harness --test ch01_1 --test ch01_2 --test ch01_3 --test ch01_4
   ```

4. Run the finished chapter if you want to see the behavior:

   ```bash
   cargo test -p ch01-1-harness-core
   cargo test -p ch01-2-harness-core
   cargo test -p ch01-3-harness-core -p ch01-3-harness-cli
   cargo run -p ch01-3-harness-cli -- --mock ask "Say hello"
   cargo test -p ch01-4-harness-core -p ch01-4-harness-cli
   cargo run -p ch01-4-harness-cli -- --mock ask "Say hello"
   ```

5. Chapter 2 is the same shape: `cargo test -p my-harness --test ch02`, and `cargo run -p ch02-harness-cli -- --mock chat`.

## Run it in your browser

Open this repo in GitHub Codespaces. The dev container installs stable Rust (clippy and rustfmt included), `cargo-nextest` 0.9.148, and mdBook 0.4.52, then runs `cargo fetch --locked`. Chapter 4's crates need rustc 1.88, which current stable provides. After the container finishes creating:

```bash
rustc --version
cargo nextest run -p ch01-4-harness-core -p ch02-harness-core -p ch04-4-harness-core -p ch05-5-harness-core -p ch06-10-harness-core --locked
cargo run -p ch04-4-harness-cli -- --mock ask "hello"
cargo run -p ch06-1-harness-cli -- ask "say hello"
mdbook build book
```

The chapter 1 and chapter 2 clients default to OpenRouter (`https://openrouter.ai/api/v1`), `OPENROUTER_API_KEY`, and `HARNESS_MODEL` (default `openai/gpt-4o-mini`). `--base-url` points the same client at another OpenAI-compatible server and then reads `OPENAI_API_KEY` (empty is allowed for Ollama). `--mock` uses the offline provider. Chapter 2 submits each turn to a session inbox before the provider runs.

Chapters 0 through 48, in six parts. Eight chapters are a standalone MCP gateway, marked *(server)* and interleaved with the harness. The split, and the notes from Hermes and from existing gateways, are [docs/mcp-gateway.md](docs/mcp-gateway.md). Performance is a budget on every chapter, starting with tokio in chapter 1. The targets are [docs/performance.md](docs/performance.md). MCP is first-class: tools are MCP-shaped (chapter 3), the harness is an MCP client (chapter 4, via `rmcp`), and the gateway is the MCP server (chapter 5, skeleton only). Part II is learning, and that work stays off the hot path: sessions, memory, recall, skills, reflection, and curation. The design is [docs/self-improvement.md](docs/self-improvement.md). Which team pattern to use is [docs/team-patterns.md](docs/team-patterns.md). The default is a supervisor that calls specialists as tools and fans them out under a bound. Part V runs the harness as a service, and chapter 38 runs the gateway as one. Chapter 37 is the chat gateway into the session inbox. It is a different process from the MCP server.

**Part I: One agent**

| Ch | Title | Step | You add |
|---|---|---|---|
| 0 | Introduction | — | why a harness, and what Hermes is |
| 1 | First Contact | `steps/ch01-1-messages`, `ch01-2-provider`, `ch01-3-cli`, `ch01-4-errors` | messages, one `Event` enum, OpenRouter as the default client, per-agent cost, `ask`, a request timeout |
| 2 | Conversations | `steps/ch02-conversations` | a real conversation, a cache-stable system prompt, a `chat` REPL, and the session inbox |
| 3 | Tools | `steps/ch03-1-registry`, `ch03-2-toolsets`, `ch03-3-dispatch` | an MCP-shaped tool trait, named toolsets gated on env and checks, concurrent reads and exclusive writes |
| 4 | MCP Client | `steps/ch04-1-stdio`, `ch04-2-http`, `ch04-3-registry`, `ch04-4-cache` | `rmcp` over stdio and streamable HTTP; MCP and native tools share one scheduler; a cached `tools/list` |
| 5 | A Minimal MCP Server *(server)* | `steps/gw-minimal-server` | an empty catalog in `mcp-gateway/`; no socket yet |
| 6 | Thousands of Tools | `steps/ch05-1-schema-cost`, `ch05-2-tool-index`, `ch05-3-search-tool`, `ch05-4-routing`, `ch05-5-lazy-list` | the client half: schema cost, a search index, `search_tools`, routing, a cached list. Crates stay `ch05-*` |
| 7 | Dynamic Catalog *(server)* | `steps/gw-catalog` | server-side BM25 plus embeddings, and a catalog reload. Not written |
| 8 | Session Toolsets *(server)* | `steps/gw-session-toolsets` | per-session toolsets, `set_task`, compact schemas, token telemetry. Not written |
| 9 | The Agent Loop | `steps/ch06-10-evals` | model, tools, results, repeat; cancellation; validation; stall; budget; truncation; steering; canonical args; a typed outcome; stress evals. Sections stay 6.1–6.10 |
| 10 | The Turn State Machine | `steps/ch07-turn-state` | one explicit turn, right after the loop |
| 11 | Approvals and Guardrails | `steps/ch09-approvals` | risk levels, human or second-model review or skip, one approval prompt at a time |
| 12 | Hooks and Plugins | `steps/ch10-hooks` | before/after tool hooks, API-request hooks, rewritten tool results, a swappable memory backend |
| 13 | Sandboxed Execution | `steps/ch11-sandbox` | a code tool outside the process, plus file checkpoint and rewind |
| 14 | When Things Fail | `steps/ch12-when-things-fail` | classify failures, retry, fall back, including MCP crashes and reconnects |
| 15 | Limits and Breakers *(server)* | `steps/gw-limits` | timeouts, idempotent-only retries, breakers, queues. Not written |
| 16 | Providers | `steps/ch13-providers` | native Anthropic and Bedrock adapters, OpenRouter fallback and routing, key rotation, a cheaper side model |
| 17 | Streaming | `steps/ch14-streaming` | the same events, as tokens arrive |

**Part II: Learning**

| Ch | Title | Step | You add |
|---|---|---|---|
| 18 | Sessions on Disk | `steps/ch15-sessions` | SQLite sessions (append-only, WAL) and resume |
| 19 | Memory Files | `steps/ch16-memory` | `MEMORY.md` / `USER.md`, the `memory` tool, a frozen system-prompt snapshot |
| 20 | Recall | `steps/ch17-recall` | FTS5 `session_search`: discover, scroll, read, browse |
| 21 | Learned Tools *(server)* | `steps/gw-learned-tools` | learned tool prediction and composite tools. Not written |
| 22 | Using Skills | `steps/ch18-skills` | `SKILL.md`, a cache-stable index, `skills_list` / `skill_view` |
| 23 | Writing Skills | `steps/ch19-skill-writing` | create, patch, delete, a scanner, the approval gate, read-before-write |
| 24 | The Reflection Loop | `steps/ch20-reflection` | nudge counters and a forked reviewer with only skill and memory tools |
| 25 | Curation | `steps/ch21-curation` | stale to archived, pinning, backups, optional consolidation |

**Part III: The team patterns**

| Ch | Title | Step | You add |
|---|---|---|---|
| 26 | The Sequential Pipeline | `steps/ch22-pipeline` | outline, draft, critique, revise; an agent as a reusable value |
| 27 | The Supervisor | `steps/ch23-supervisor` | the default team: manager, agents-as-tools, fan-out under a bound, `Arc` context |
| 28 | Handoffs | `steps/ch24-handoffs` | ownership transfer, handoff-as-tool, max-handoff limit, context filtering |
| 29 | Fan-out and Gather | `steps/ch25-fan-out` | parallel specialists, a worker limit, an aggregator |
| 30 | The Critic | `steps/ch26-critic` | an evaluator as a tool and as a final gate |
| 31 | Composing Patterns | `steps/ch27-composition` | a supervisor that fans out and hands off only when ownership moves |

**Part IV: Making it robust**

| Ch | Title | Step | You add |
|---|---|---|---|
| 32 | Managing Context | `steps/ch28-context` | compression, summaries, `[SKILL_PRUNED]`, a memory flush |
| 33 | Agents as Actors | `steps/ch29-actors` | one task per agent, the session inbox, backpressure |
| 34 | Supervision and Restart | `steps/ch30-supervision` | restart a failed child |
| 35 | Balancing and Federation *(server)* | `steps/gw-balancing` | per-tool pools and upstream MCP federation. Not written |

**Part V: Running as a service**

| Ch | Title | Step | You add |
|---|---|---|---|
| 36 | Profiles | `steps/ch31-profiles` | several personas, each with its own secrets |
| 37 | The Gateway | `steps/ch32-gateway` | platform adapters into the session inbox; interrupts, steers, and chapter 1 events. Not the MCP server |
| 38 | MCP Gateway Service *(server)* | `steps/gw-service` | HA replicas, shared state, OAuth 2.1, drain. Not written |
| 39 | Cron | `steps/ch33-cron` | at-most-once runs across crashes, catch-up, a watchdog, chaining onto the same inbox |
| 40 | Editor Integration | `steps/ch34-acp` | ACP as another client of the session inbox |

**Part VI: Loosely coupled teams**

| Ch | Title | Step | You add |
|---|---|---|---|
| 41 | The Blackboard | `steps/ch35-blackboard` | shared state, condition-triggered agents, a thin scheduler |
| 42 | The Kanban Board | `steps/ch36-kanban` | a durable board; processes claim, complete, review, or block |
| 43 | The Message Bus | `steps/ch37-message-bus` | peer-to-peer messaging |
| 44 | Shared Team Skills | `steps/ch38-team-skills` | a layered store, `skill_propose`, a librarian as the single writer |
| 45 | A2A for External Agents | `steps/ch39-a2a` | A2A only for agents outside this process |
| 46 | Seeing Inside | `steps/ch40-tracing` | spans on the chapter 1 events and across the gateway |
| 47 | Load and Token Capstone *(server)* | `steps/gw-capstone` | load test plus tokens per completed task. Not written |
| 48 | Evals | `steps/ch41-evals` | re-run the chapter 9 tasks, plus trajectories |

Chapters 1 through 4, 6, and 9 have harness step crates. Chapter 5 has the `mcp-gateway` skeleton. Later server rows are the plan, not empty crates. The same list drives CI (`.github/chapters.json`).

The book builds with [mdBook](https://rust-lang.github.io/mdBook/): `mdbook build` in `book/` (output in `book/_build/`, not committed).

## Continuous integration

[CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml) runs on every push to `main` and on pull requests:

- `cargo fmt --check` and `cargo clippy --workspace --all-targets` (warnings do not fail the build)
- `cargo test -p …` for every step, each step as its own job, on Ubuntu
- the same tests on Ubuntu and Windows for steps whose files changed ([paths-filter](https://github.com/dorny/paths-filter))
- macOS for those steps, at most four jobs at a time, and one automatic retry when the first wave does not succeed. A gate fails the workflow if both waves miss
- an mdBook build of `book/`

[Benchmarks](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml) run on pull requests and on pushes to `main`. They execute the criterion benches for the written chapters on Ubuntu. Each section is recorded before the next one starts, and the job summary prints the Criterion median with its low and high. A push to `main` appends that run to [`data.json`](benchmarks/data.json) on the `gh-pages` branch. The [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/) charts it. A slow result does not fail the run.

`my-harness` milestone tests are not part of that required build. They run in [My harness](https://github.com/agnathan/rust-agent-harness/actions/workflows/my-harness.yml).

### Milestone tests across operating systems

The workflow installs `cargo-nextest` and runs `cargo nextest run -p my-harness --test chNN` (and falls back to `cargo test` if nextest is unavailable). Unimplemented chapters show as **fail**. JUnit XML is uploaded.

**Push and pull request.** A change under `my-harness/` tests the affected chapters on Ubuntu, macOS, and Windows. This mode is a report: failing stubs do not fail the workflow.

**Manual run.** Actions → My harness → Run workflow.

| Input | Values |
|---|---|
| `chapters` | `all` (default), `1`, `1-48`, or `9,12,14`. Chapters 1–48 are the plan; a chapter runs only after its `tests/chNN.rs` exists (today: 1 and 2) |
| `os` | `all` (default), `ubuntu`, `macos`, or `windows` |

A manual run fails if any selected chapter fails.

## License

[MIT](LICENSE) © 2026 Daniel Holmlund
