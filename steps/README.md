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
| ch05-1-schema-cost | 6 (section 5.1) | Synthetic MCP-shaped catalog, schema-token estimate |
| ch05-2-tool-index | 6 (section 5.2) | BM25 index over names and descriptions, optional `Embedder` |
| ch05-3-search-tool | 6 (section 5.3) | `search_tools`, append-only reveal, cap and eviction |
| ch05-4-routing | 6 (section 5.4) | Route by server, trim descriptions, consistent names |
| ch05-5-lazy-list | 6 (section 5.5) | Cached `tools/list`, `tools/list_changed`, per-agent view, `cards_from_listed`, `ask` |
| ch06-1-live | 9 (section 6.1) | Optional `--live` OpenRouter call with echo and note. Mock is the default |
| ch06-2-cancel | 9 (section 6.2) | `CancellationToken`, soft stop and hard stop, raced with `select!` |
| ch06-3-validate | 9 (section 6.3) | Unknown-tool repair, JSON retry, timeouts as tool results |
| ch06-4-stall | 9 (section 6.4) | Identical-call stub and A,B cycle halt |
| ch06-5-budget | 9 (section 6.5) | Iteration warning, 80% time wrap-up, refund, subagent cap |
| ch06-6-truncate | 9 (section 6.6) | Per-tool output cap, head and tail kept |
| ch06-7-steer | 9 (section 6.7) | Session-inbox steer appended to the latest tool result |
| ch06-8-canonical | 9 (section 6.8) | Sorted, compact tool-call arguments on the send path |
| ch06-9-outcome | 9 (section 6.9) | `TurnOutcome` enum and a budget wrap-up call |
| ch06-10-evals | 9 (section 6.10) | Stress evals, optionally live, still keyless in CI |
| ch07-turn-state | 10 | `Phase` and a tested `step` table |
| ch09-approvals | 11 | Approval pipeline, one human prompt, permit dropped before the wait |
| ch10-hooks | 12 | Tool hooks, a borrowing API hook, a memory queue |
| ch11-sandbox | 13 | Child worker, Docker argv, env scrub, checkpoints |
| ch12-when-things-fail | 14 | Empty retries, repetition guard, timeout drops the permit |
| ch13-providers | 16 | Anthropic and Bedrock bodies, a route, key rotation |
| ch14-streaming | 17 | Think-block scrubber, folded deltas, read-only overlap |
| ch15-sessions | 18 | WAL sessions, one append, FTS ids queued |
| ch16-memory | 19 | MEMORY.md and USER.md, frozen snapshot, strict scan |
| ch17-recall | 20 | FTS5 trigram session_search: discover, scroll, read, browse |
| gw-learned-tools | 21 *(server)* | Predicted offer, composite and macro tools, code-mode lines |
| ch18-skills | 22 | `SKILL.md`, a cache-stable index, `skills_list` / `skill_view` |
| ch19-skill-writing | 23 | create, patch, delete, a scanner, the approval gate, read-before-write |
| ch20-reflection | 24 | nudge counters, a forked reviewer, cancel on return |
| ch21-curation | 25 | stale, then archived, a snapshot, off the request path |
| ch22-pipeline | 26 | outline, draft, critique, revise, one shared `Arc` |
| ch23-supervisor | 27 | eight specialists, one `Arc`, blocked child tools, depth 1 |
| ch24-handoffs | 28 | a filtered transcript and a cap of four handoffs |
| ch25-fan-out | 29 | a queue of 100, the supervisor bound, summaries in index order |
| ch26-critic | 30 | a verdict, one permit, accept only on done |
| ch27-composition | 31 | nested fan-out under one pool, a handoff only when ownership moves, the critic loop |
| ch28-context | 32 | a scan, a 20,000-character cap, an untrusted wrap, preflight compression |
| ch29-actors | 33 | one task per agent, an inbox of 64, steer and interrupt |
| ch30-supervision | 34 | one restart, a chain of three, cancel and budget |
| gw-limits | 15 *(server)* | Per-tool permits, idempotent retry, a breaker, caps of 50 |
| gw-minimal-server | 5 *(server)* | Empty `mcp-gateway` catalog. No socket |

Your own copy is `my-harness/` at the repo root. Milestone tests there start failing until you implement each chapter. The course plan is chapters 0–48 in six parts, listed in `book/book-outline.md` and `.github/chapters.json`. Performance budgets for every chapter are `docs/performance.md`. MCP is a first-class tool boundary from chapter 3 (client in chapter 4, gateway server in chapter 5). The gateway design is `docs/mcp-gateway.md`. Part II is how one agent learns; the design is `docs/self-improvement.md`. How the team patterns fit together is `docs/team-patterns.md`. Only the steps in the table above exist so far. Later `gw-*` rows in the outline are the plan, not empty crates. Section numbers inside chapters 6 and 9 stay aligned with the `ch05-*` and `ch06-*` crate names.
