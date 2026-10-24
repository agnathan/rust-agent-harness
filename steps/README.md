# Steps

Each `chNN-<slug>/` is the harness at the end of that book chapter: the library and the CLI, with that chapter's tests. Chapters are cumulative. A later step is a new copy, not a patch on the previous directory, so every chapter still builds on its own.

They are members of the root cargo workspace. Package names are prefixed (`ch01-1-harness-core`, `ch01-3-harness-cli`, `ch01-4-harness-cli`, `ch02-harness-cli`, …) because cargo needs unique names, and the binaries are `ch01-harness`, `ch01-4-harness`, and `ch02-harness` so they do not overwrite one file in `target/`. The library crate name stays `harness_core`, which is what the tests import. Inside the CLI the clap app is still called `harness`.

```bash
cargo test -p ch01-1-harness-core
cargo test -p ch01-2-harness-core
cargo test -p ch01-3-harness-cli
cargo test -p ch01-4-harness-cli
cargo test -p ch02-harness-core
cargo run -p ch02-harness-cli -- --mock chat
```

| Step | Chapter | Adds |
|---|---|---|
| ch01-1-messages | 1.1 | `Message` / `Role`, OpenAI JSON, a 50-message serde bench |
| ch01-2-provider | 1.2 | `Provider`, latency-configurable `MockProvider`, tokio, a 100-call `JoinSet` bench |
| ch01-3-cli | 1.3 | OpenRouter by default, shared `reqwest` client (pool, HTTP/2), `ask`, wiremock tests, request-parse bench |
| ch01-4-errors | 1.4 | request timeout, buffered-reply edges (no usage, a different answered model), parse bench |
| ch02-conversations | 2 | Conversation, scripted mock, JSON save/load, `chat` REPL, session inbox, OpenRouter events and cost |
| ch03-1-registry | 3.1 | MCP-shaped `Tool` trait, JSON Schema, registry, lookup bench |
| ch03-2-toolsets | 3.2 | Named toolsets gated on env vars and checks, available-list bench |
| ch03-3-dispatch | 3.3 | Concurrent reads, exclusive writes, bound of 8, dispatch bench |
| ch04-1-stdio | 4.1 | `rmcp` over stdio, list and call, round-trip bench |
| ch04-2-http | 4.2 | Streamable HTTP on `127.0.0.1`, HTTP round-trip bench |
| ch04-3-registry | 4.3 | MCP tools in the chapter 3 registry and dispatcher |
| ch04-4-cache | 4.4 | Cached `tools/list`, `list_changed`, `ask` |
| ch06-1-schema-cost | 6 (section 6.1) | Synthetic MCP-shaped catalog, schema-token estimate |
| ch06-2-tool-index | 6 (section 6.2) | BM25 index over names and descriptions, optional `Embedder` |
| ch06-3-search-tool | 6 (section 6.3) | `search_tools`, append-only reveal, cap and eviction |
| ch06-4-routing | 6 (section 6.4) | Route by server, trim descriptions, consistent names |
| ch06-5-lazy-list | 6 (section 6.5) | Cached `tools/list`, `tools/list_changed`, per-agent view, `cards_from_listed`, `ask` |
| ch09-1-live | 9 (section 9.1) | Optional `--live` OpenRouter call with echo and note. Mock is the default |
| ch09-2-cancel | 9 (section 9.2) | `CancellationToken`, soft stop and hard stop, raced with `select!` |
| ch09-3-validate | 9 (section 9.3) | Unknown-tool repair, JSON retry, timeouts as tool results |
| ch09-4-stall | 9 (section 9.4) | Identical-call stub and A,B cycle halt |
| ch09-5-budget | 9 (section 9.5) | Iteration warning, 80% time wrap-up, refund, subagent cap |
| ch09-6-truncate | 9 (section 9.6) | Per-tool output cap, head and tail kept |
| ch09-7-steer | 9 (section 9.7) | Session-inbox steer appended to the latest tool result |
| ch09-8-canonical | 9 (section 9.8) | Sorted, compact tool-call arguments on the send path |
| ch09-9-outcome | 9 (section 9.9) | `TurnOutcome` enum and a budget wrap-up call |
| ch09-10-evals | 9 (section 9.10) | Stress evals, optionally live, still keyless in CI |
| ch10-turn-state | 10 | Chapter 9's harness, plus `Phase` / `Driver`. `run_loop` drives the table |
| ch11-approvals | 11 | Chapter 10's harness, plus the approval pipeline. `deny_tool` runs before dispatch |
| ch12-hooks | 12 | Tool hooks, a borrowing API hook, a memory queue |
| ch13-sandbox | 13 | Child worker, Docker argv, env scrub, checkpoints |
| ch14-when-things-fail | 14 | Empty retries, repetition guard, timeout drops the permit |
| ch18-providers | 18 | Anthropic and Bedrock bodies, a route, key rotation |
| ch19-streaming | 19 | Think-block scrubber, folded deltas, read-only overlap |
| ch20-sessions | 20 | WAL sessions, one append, FTS ids queued |
| ch21-memory | 21 | MEMORY.md and USER.md, frozen snapshot, strict scan |
| ch22-recall | 22 | FTS5 trigram session_search: discover, scroll, read, browse |
| ch23-learned-tools | 23 *(server)* | Predicted offer, composite and macro tools, code-mode lines |
| ch24-skills | 24 | `SKILL.md`, a cache-stable index, `skills_list` / `skill_view` |
| ch25-skill-writing | 25 | create, patch, delete, a scanner, the approval gate, read-before-write |
| ch26-reflection | 26 | nudge counters, a forked reviewer, cancel on return |
| ch27-curation | 27 | stale, then archived, a snapshot, off the request path |
| ch28-pipeline | 28 | outline, draft, critique, revise, one shared `Arc` |
| ch29-supervisor | 29 | eight specialists, one `Arc`, blocked child tools, depth 1 |
| ch30-handoffs | 30 | a filtered transcript and a cap of four handoffs |
| ch31-fan-out | 31 | a queue of 100, the supervisor bound, summaries in index order |
| ch32-critic | 32 | a verdict, one permit, accept only on done |
| ch33-composition | 33 | nested fan-out under one pool, a handoff only when ownership moves, the critic loop |
| ch34-context | 34 | a scan, a 20,000-character cap, an untrusted wrap, preflight compression |
| ch35-actors | 35 | one task per agent, an inbox of 64, steer and interrupt |
| ch36-supervision | 36 | one restart, a chain of three, heartbeats, cancel and budget |
| ch37-balancing | 37 *(server)* | least outstanding, power of two, federation, discovery cap of 4 |
| ch38-profiles | 38 | personas, a secret map per profile, fail closed with no scope |
| ch39-gateway | 39 | allowlist, pairing, bot-loop guard, emergency stop, the session inbox, gateway lifecycle block |
| ch40-browser | 40 | navigate and snapshot behind `PageBackend`, secret URL and SSRF, untrusted page text |
| ch41-vision | 41 | sniff, size and type caps, untrusted caption, no vision API |
| ch42-service | 42 *(server)* | session routing, an `Arc` reload, quotas, a drain, sampling and OSV |
| ch43-cron | 43 | one claim, one crash retry, a prompt scan, the lifecycle guard |
| ch44-acp | 44 | one inbox message per ACP update, `end_turn` or `cancelled` |
| ch45-blackboard | 45 | facts and waiters, a claim that drops the lock, pause and an injection check |
| ch46-kanban | 46 | one JSON board, a `create_new` lock, claim, complete, review, or block |
| ch47-message-bus | 47 | a bus of 64, `try_send`, a bot cooldown of 20, egress redaction |
| ch48-team-skills | 48 | a proposal queue, a librarian as the only writer, an `Arc` index swap |
| ch49-a2a | 49 | A2A for a URL outside this process, a pool of 8, a cancel flag |
| ch50-tracing | 50 | a counting subscriber, content-free span names, 100 mock researchers |
| ch51-capstone | 51 *(server)* | three token arms and a load of 100. No socket |
| ch52-evals | 52 | the chapter 9 task names, one red-team case per layer, no fail threshold |
| ch15-limits | 15 *(server)* | Chapters 7 and 8 vendored, plus per-tool permits, idempotent retry, a breaker, caps of 50 |
| ch05-minimal-server | 5 *(server)* | Fixed tool list, `echo`, a 32/8 gate. No socket |

From chapter 10 on, each harness directory is a copy of the previous one plus that chapter's modules. The layout and the plan for chapters 23–52 are `docs/harness-architecture.md`. Your own copy is `my-harness/` at the repo root. Milestone tests there compile and panic with `todo!` until you implement each chapter. The course plan is chapters 0–52 in six parts, listed in `book/book-outline.md` and `.github/chapters.json`. Performance budgets for every chapter are `docs/performance.md`. MCP is a first-class tool boundary from chapter 3 (client in chapter 4, gateway server in chapter 5). The gateway design is `docs/mcp-gateway.md`. Part II is how one agent learns; the design is `docs/self-improvement.md`. How the team patterns fit together is `docs/team-patterns.md`. The steps in the table above are the written chapters, including `ch51-capstone`. None of the server steps bind a socket. Section numbers inside chapters 6 and 9 stay aligned with the `ch06-*` and `ch09-*` crate names.
