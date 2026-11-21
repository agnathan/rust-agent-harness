# rust-agent-harness

[![CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml/badge.svg)](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml)

An AI agent harness in Rust, with a book on how to build it step by step.

This is a hands-on course for people **new to Rust**. You learn the language by growing one harness, not by a tour of the language. The design is informed by Nous Research's [Hermes Agent](https://github.com/NousResearch/hermes-agent) (Python). Each chapter points at the Hermes files that solve the same problem.

The chapter list, with links to the pages and step crates that exist, is [TABLE_OF_CONTENTS.md](TABLE_OF_CONTENTS.md). It follows [book/SUMMARY.md](book/SUMMARY.md).

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
cargo nextest run -p ch01-4-harness-core -p ch02-harness-core -p ch04-4-harness-core -p ch06-5-harness-core -p ch09-10-harness-core --locked
cargo run -p ch04-4-harness-cli -- --mock ask "hello"
cargo run -p ch09-1-harness-cli -- ask "say hello"
mdbook build book
```

The chapter 1 and chapter 2 clients default to OpenRouter (`https://openrouter.ai/api/v1`), `OPENROUTER_API_KEY`, and `HARNESS_MODEL` (default `openai/gpt-4o-mini`). `--base-url` points the same client at another OpenAI-compatible server and then reads `OPENAI_API_KEY` (empty is allowed for Ollama). `--mock` uses the offline provider. Chapter 2 submits each turn to a session inbox before the provider runs.

Chapters 0 through 53, in six parts. Eight chapters are a standalone MCP gateway, marked *(server)* and interleaved with the harness: chapters 5, 7, 8, 15, 23, 37, 42, and 51. The split, and the notes from Hermes and from existing gateways, are [docs/mcp-gateway.md](docs/mcp-gateway.md). Performance is a budget on every chapter, starting with tokio in chapter 1. The targets are [docs/performance.md](docs/performance.md). MCP is first-class: tools are MCP-shaped (chapter 3), the harness is an MCP client (chapter 4, via `rmcp`), and the gateway is the MCP server (chapter 5, then chapters 7, 8, 15, 23, 37, 42, and 51). Part II is learning, and that work stays off the hot path: sessions, memory, recall, skills, reflection, and curation. The design is [docs/self-improvement.md](docs/self-improvement.md). Which team pattern to use is [docs/team-patterns.md](docs/team-patterns.md). The default is a supervisor that calls specialists as tools and fans them out under a bound. Part V runs the harness as a service, and chapter 42 runs the gateway as one. Chapter 39 is the chat gateway into the session inbox. It is a different process from the MCP server.

**Part I: One agent**

| Ch | Title | Step | You add |
|---|---|---|---|
| 0 | Introduction | — | why a harness, and what Hermes is |
| 1 | First Contact | `steps/ch01-1-messages`, `ch01-2-provider`, `ch01-3-cli`, `ch01-4-errors` | messages, one `Event` enum, OpenRouter as the default client, per-agent cost, `ask`, a request timeout |
| 2 | Conversations | `steps/ch02-conversations` | a real conversation, a cache-stable system prompt, a `chat` REPL, and the session inbox |
| 3 | Tools | `steps/ch03-1-registry`, `ch03-2-toolsets`, `ch03-3-dispatch` | an MCP-shaped tool trait, named toolsets gated on env and checks, concurrent reads and exclusive writes |
| 4 | MCP Client | `steps/ch04-1-stdio`, `ch04-2-http`, `ch04-3-registry`, `ch04-4-cache` | `rmcp` over stdio and streamable HTTP; MCP and native tools share one scheduler; a cached `tools/list` |
| 5 | A Minimal MCP Server *(server)* | `steps/ch05-minimal-server` | a fixed tool list, `echo`, and a 32/8 gate; no socket |
| 6 | Thousands of Tools | `steps/ch06-1-schema-cost`, `ch06-2-tool-index`, `ch06-3-search-tool`, `ch06-4-routing`, `ch06-5-lazy-list` | the client half: schema cost, a search index, `search_tools`, routing, a cached list. Crates stay `ch06-*` |
| 7 | Dynamic Catalog *(server)* | `steps/ch07-catalog` | server-side BM25 plus embeddings, and a catalog reload |
| 8 | Session Toolsets *(server)* | `steps/ch08-session-toolsets` | per-session toolsets, `set_task`, compact schemas, token telemetry |
| 9 | The Agent Loop | `steps/ch09-10-evals` | model, tools, results, repeat; cancellation; validation; stall; budget; truncation; steering; canonical args; a typed outcome; stress evals. Sections stay 9.1–9.10 |
| 10 | The Turn State Machine | `steps/ch10-turn-state` | one explicit turn, right after the loop |
| 11 | Approvals and Guardrails | `steps/ch11-approvals` | risk levels, human or second-model review or skip, one approval prompt at a time |
| 12 | Hooks and Plugins | `steps/ch12-hooks` | before/after tool hooks, API-request hooks, rewritten tool results, a swappable memory backend |
| 13 | Sandboxed Execution | `steps/ch13-sandbox` | a code tool outside the process, plus file checkpoint and rewind |
| 14 | When Things Fail | `steps/ch14-when-things-fail` | classify failures, retry, fall back, including MCP crashes and reconnects |
| 15 | Limits and Breakers *(server)* | `steps/ch15-limits` | timeouts, idempotent-only retries, breakers, queues |
| 16 | Built-in Tools I: Web Search and Extract | `steps/ch16-web-tools` | `web_search` and `web_extract`, a fake backend, SSRF, truncation |
| 17 | Built-in Tools II: Files, Terminal, Todo, and Clarify | `steps/ch17-local-tools` | a workspace root, stale writes, one-match patches, terminal gates, a todo list, a clarify pause |
| 18 | Providers | `steps/ch18-providers` | native Anthropic and Bedrock adapters, OpenRouter fallback and routing, key rotation, a cheaper side model |
| 19 | Streaming | `steps/ch19-streaming` | the same events, as tokens arrive |

**Part II: Learning**

| Ch | Title | Step | You add |
|---|---|---|---|
| 20 | Sessions on Disk | `steps/ch20-sessions` | SQLite sessions (append-only, WAL) and resume |
| 21 | Memory Files | `steps/ch21-memory` | `MEMORY.md` / `USER.md`, the `memory` tool, a frozen system-prompt snapshot |
| 22 | Recall | `steps/ch22-recall` | FTS5 `session_search`: discover, scroll, read, browse |
| 23 | Learned Tools *(server)* | `steps/ch23-learned-tools` | a predicted offer, composite and macro tools, a code-mode script |
| 24 | Using Skills | `steps/ch24-skills` | `SKILL.md`, a cache-stable `<available_skills>` index, `skills_list` / `skill_view` |
| 25 | Writing Skills | `steps/ch25-skill-writing` | create, patch, delete, a scanner, the approval gate, read-before-write |
| 26 | The Reflection Loop | `steps/ch26-reflection` | nudge counters and a forked reviewer with only skill and memory tools |
| 27 | Curation | `steps/ch27-curation` | stale to archived, pinning, backups, optional consolidation |

**Part III: The team patterns**

| Ch | Title | Step | You add |
|---|---|---|---|
| 28 | The Sequential Pipeline | `steps/ch28-pipeline` | outline, draft, critique, revise; the next stage receives the same `Arc` |
| 29 | The Supervisor | `steps/ch29-supervisor` | the default team: manager, agents-as-tools, eight specialists, one pool |
| 30 | Handoffs | `steps/ch30-handoffs` | ownership transfer, handoff-as-tool, max-handoff limit, context filtering |
| 31 | Fan-out and Gather | `steps/ch31-fan-out` | a queue of 100, gather in `task_index` order |
| 32 | The Critic | `steps/ch32-critic` | an evaluator that takes one permit and does not do the work |
| 33 | Composing Patterns | `steps/ch33-composition` | a supervisor that fans out and hands off only when ownership moves |

**Part IV: Making it robust**

| Ch | Title | Step | You add |
|---|---|---|---|
| 34 | Managing Context | `steps/ch34-context` | compression, summaries, `[SKILL_PRUNED]`, a memory flush |
| 35 | Agents as Actors | `steps/ch35-actors` | one task per agent, the session inbox, backpressure |
| 36 | Supervision and Restart | `steps/ch36-supervision` | restart a failed child, including heartbeats |
| 37 | Balancing and Federation *(server)* | `steps/ch37-balancing` | per-tool pools and upstream MCP federation |

**Part V: Running as a service**

| Ch | Title | Step | You add |
|---|---|---|---|
| 38 | Profiles | `steps/ch38-profiles` | several personas, each with its own secrets, kept out of the prompt |
| 39 | The Gateway | `steps/ch39-gateway` | allowlist, pairing, and platform messages into the session inbox. Not the MCP server |
| 40 | Browser Tools | `steps/ch40-browser` | navigate and snapshot behind a fake page, secret URL and SSRF, untrusted page text |
| 41 | Vision | `steps/ch41-vision` | sniff, size and type caps, an untrusted caption. No vision API |
| 42 | MCP Gateway Service *(server)* | `steps/ch42-service` | session routing, quotas, a drain, sampling and an OSV check. No socket |
| 43 | Cron | `steps/ch43-cron` | one claim, one crash retry, a prompt scan, the lifecycle guard |
| 44 | Editor Integration | `steps/ch44-acp` | ACP as another client of the session inbox, `end_turn` or `cancelled` |

**Part VI: Loosely coupled teams**

| Ch | Title | Step | You add |
|---|---|---|---|
| 45 | The Blackboard | `steps/ch45-blackboard` | shared state, condition-triggered agents, a thin scheduler |
| 46 | The Kanban Board | `steps/ch46-kanban` | a durable board; processes claim, complete, review, or block |
| 47 | The Message Bus | `steps/ch47-message-bus` | peer-to-peer messaging |
| 48 | Shared Team Skills | `steps/ch48-team-skills` | a layered store, `skill_propose`, a librarian as the single writer |
| 49 | A2A for External Agents | `steps/ch49-a2a` | A2A only for agents outside this process |
| 50 | Seeing Inside | `steps/ch50-tracing` | spans on the chapter 1 events and across the gateway |
| 51 | Load and Token Capstone *(server)* | `steps/ch51-capstone` | load test plus tokens per completed task. No socket |
| 52 | Evals | `steps/ch52-evals` | re-run the chapter 9 task names, plus trajectories, still with no fail threshold |
| 53 | The Loop Closes | `steps/ch53-the-loop-closes` | one task, scored twice: the first attempt fails, a skill is kept, the second attempt passes |

Chapters 1 through 53 have step crates. Chapter 5 has the `mcp-gateway` skeleton. The server steps, including `steps/ch51-capstone`, do not bind a socket. The same list drives CI (`.github/chapters.json`).

The book builds with [mdBook](https://rust-lang.github.io/mdBook/): `mdbook build` in `book/` (output in `book/_build/`, not committed).

## Continuous integration

[CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml) runs on every push to `main` and on pull requests:

- `cargo fmt --check` and `cargo clippy --workspace --all-targets` (warnings do not fail the build)
- `cargo test -p …` for every step, each step as its own job, on Ubuntu
- the same tests on Ubuntu and Windows for steps whose files changed ([paths-filter](https://github.com/dorny/paths-filter))
- macOS for those steps, at most four jobs at a time, and one automatic retry when the first wave does not succeed. A gate fails the workflow if both waves miss
- an mdBook build of `book/`
- `python3 .github/scripts/check_toc_links.py`, which fails if a link in `TABLE_OF_CONTENTS.md` points at a missing file, or if a written chapter does not use the file named in `book/SUMMARY.md`

[Benchmarks](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml) run on pull requests and on pushes to `main`. They execute the criterion benches for the written chapters on Ubuntu. Each section is recorded before the next one starts, and the job summary prints the Criterion median with its low and high. A push to `main` appends that run to [`data.json`](benchmarks/data.json) on the `gh-pages` branch. The [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/) charts it. A slow result does not fail the run.

`my-harness` milestone tests are not part of that required build. They run in [My harness](https://github.com/agnathan/rust-agent-harness/actions/workflows/my-harness.yml).

### Milestone tests across operating systems

The workflow installs `cargo-nextest` and runs `cargo nextest run -p my-harness --test chNN` (and falls back to `cargo test` if nextest is unavailable). Unimplemented chapters show as **fail**. JUnit XML is uploaded.

**Push and pull request.** A change under `my-harness/` tests the affected chapters on Ubuntu, macOS, and Windows. This mode is a report: failing stubs do not fail the workflow.

**Manual run.** Actions → My harness → Run workflow.

| Input | Values |
|---|---|
| `chapters` | `all` (default), `1`, `1-52`, or `9,12,14`. The book is an introduction plus chapters 1–52; a chapter runs only after its `tests/chNN.rs` exists (today: the chapter 1 sections, chapter 2, and chapters 11–32). Those tests panic on `todo!` until the stubs are filled |
| `os` | `all` (default), `ubuntu`, `macos`, or `windows` |

A manual run fails if any selected chapter fails.

## License

[MIT](LICENSE) © 2026 Daniel Holmlund
