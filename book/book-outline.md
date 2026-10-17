# Book outline

**Working title:** *Learning Rust by Building a Multi-Agent Harness*

**Alternative titles:**
- *Agents in Rust: From First Function to Agent Teams*
- *Fearless Agents*

**Audience:** Developers who know at least one other language (Python, JavaScript, Go) and are curious about AI agents, but are new to Rust. No systems-programming background is assumed.

**Promise:** By the end, the reader has built a working, tested, concurrent harness for teams of LLM agents, and a standalone MCP gateway server beside it. Rust shows up in short sidebars at the moment a harness problem needs it.

**Approach:** Every chapter builds one more piece of the same harness, or of the gateway. The finished code for a harness chapter is a step crate under `steps/chNN-<slug>/`. Server-track chapters use `steps/gw-<slug>/` and the workspace crate `mcp-gateway/`. The reader's own copy grows in `my-harness/`. Each chapter follows the same shape:

1. a problem from agent engineering
2. a **How Hermes does it** box: the Python design, then the Rust one
3. the Rust needed for this piece, in a short sidebar
4. building it, with the milestone tests in `my-harness/tests/chNN.rs`
5. a milestone demo: the mock by default, and an optional real OpenRouter call
6. what was learned, including `[personal note]` placeholders for the author's own experience

The book is six parts and chapters 0 through 48. Eight of those chapters are the MCP gateway, marked *(server)* below, and they sit immediately after the harness chapter they build on. The architecture and the notes from Hermes and from ContextForge, ToolHive, and the Docker MCP gateway are in [`docs/mcp-gateway.md`](../docs/mcp-gateway.md). Performance is a budget on every chapter, not a late part: async tokio from chapter 1, concurrent tools from chapter 3, and learning work off the hot path. The numbers are in [`docs/performance.md`](../docs/performance.md). MCP is first-class from Part I: tools are shaped like MCP tools (chapter 3), the harness is an MCP client (chapter 4, via `rmcp`), and a separate gateway process is the MCP server (chapter 5, then the rest of the server track). Later chapters call those servers and can publish skills through them. Part II is learning: one agent gets better with use before the book builds a team. The design for skills, memory, recall, reflection, curation, and the team librarian is [`docs/self-improvement.md`](../docs/self-improvement.md). Part III is the team patterns. The default team is a supervisor that calls other agents as tools, fans them out under a bound, and shares one context object by `Arc`. The rules for choosing a pattern are in [`docs/team-patterns.md`](../docs/team-patterns.md). Part IV is making that system robust. Part V runs it as a service: profiles, a chat gateway, the MCP gateway as a highly available service, cron, and an editor. Part VI is loosely coupled teams, including a durable kanban board next to the blackboard. Speed is already in force by then.

The verified Hermes catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md): 88 guardrails, layers L1–L9 plus a cross-cutting layer. Written chapters name the layer their code already covers. Chapters that are not written yet carry the Guardrail notes below.

**Cross-cutting.** Secret redaction is not in any written chapter. E1 is `agent/redact.py` (`redact_sensitive_text`, `RedactingFormatter`; `security.redact_secrets: true`). E2 is `redact_for_egress` in the same file, the stricter scrub for text that leaves the process. A later chapter that logs, stores, or sends text says whether it adopted those two.

The harness decides what enters the prompt. The server holds the catalog, searches it, and filters the list a session may see. Chapter 6 is the client half of that split. Chapters 5, 7, and 8 are the server half that exists now: a gate, a catalog, and a session view. They still bind no socket.

### Part I: One agent

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 0 | Introduction | — | why Rust | what a harness is, and what Hermes is |
| 1 | First Contact | `steps/ch01-1-messages`, `ch01-2-provider`, `ch01-3-cli`, `ch01-4-errors` | tokio, structs, enums, traits, serde | 1.1 messages and serde; 1.2 `Provider`, one `Event` enum, and a latency mock that emits those events; 1.3 OpenRouter as the default client, the same events from the buffered call, usage plus `cost` accumulated per agent, and `ask`; 1.4 one request timeout and the edges of a buffered reply |
| 2 | Conversations | `steps/ch02-conversations` | ownership, private fields, `Result` | alternating history, a cache-stable system prompt, trim, JSON save/load, a `chat` REPL, and one per-session inbox the CLI submits to |
| 3 | Tools | `steps/ch03-1-registry`, `ch03-2-toolsets`, `ch03-3-dispatch` | traits, JSON Schema, a semaphore | 3.1 an MCP-shaped tool trait and registry; 3.2 named toolsets gated on env vars and checks, and the model is told only about tools that passed; 3.3 read-only calls run concurrently, writes run alone, both under a bound |
| 4 | MCP Client | `steps/ch04-1-stdio`, `ch04-2-http`, `ch04-3-registry`, `ch04-4-cache` | `rmcp`, stdio, HTTP | 4.1 an `rmcp` session over stdio; 4.2 the same session over streamable HTTP; 4.3 MCP tools in the chapter 3 registry and dispatcher; 4.4 a `tools/list` cache dropped by `list_changed` |
| 5 | A Minimal MCP Server *(server)* | `steps/gw-minimal-server` | a second crate | a fixed tool list, `echo`, and a gate that accepts 32 calls and runs 8. No socket. The process crate grows in chapters 7 and 8 |
| 6 | Thousands of Tools | `steps/ch05-1-schema-cost`, `ch05-2-tool-index`, `ch05-3-search-tool`, `ch05-4-routing`, `ch05-5-lazy-list` | an inverted index, append-only prompts | the client half of the catalog. Sections stay 5.1–5.5 to match `steps/ch05-*`: measure schema tokens for 1,000–10,000 tools; a BM25 index, with embeddings behind a trait; `search_tools` reveals schemas on the next turn, append-only, with a cap; route by server, then shrink names and descriptions; a cached `tools/list`, invalidated by `tools/list_changed`, a per-agent toolset, and `cards_from_listed` so chapter 4's real list uses the same cache. The 10,000-tool benches still use the synthetic catalog |
| 7 | Dynamic Catalog *(server)* | `steps/gw-catalog` | BM25 on the server | native tools, an in-memory upstream list, and plugin tools; server-side BM25 with exact-name first; a reload that stores a new `Arc`. Live upstream connections stay chapter 35 |
| 8 | Session Toolsets *(server)* | `steps/gw-session-toolsets` | a per-session filter | `search_tools` (7 queries), `set_task`, `describe` (10 names), a revealed-name filter, a `list_changed` flag, 2,000-character pages, and schema/call/result tokens for every-tool, static, and dynamic arms. The flag is not on the wire yet |
| 9 | The Agent Loop | `steps/ch06-10-evals` | enums for replies | ten sections, still numbered 6.1–6.10 to match `steps/ch06-*`: optional live OpenRouter, cancellation token with soft and hard stop, tool-call validation, stall detection, budget warning and wrap-up, output truncation, steering, canonical arguments, a typed turn outcome, and a stress eval set |
| 10 | The Turn State Machine | `steps/ch07-turn-state` | enum state machines | one `Phase` and a tested `step` table. `run_script` is a fake model. It does not call chapter 9's `run_loop` |
| 11 | Approvals and Guardrails | `steps/ch09-approvals` | a mutex, risk as an enum | risk levels on a tool call; modes for a human, a second model, or skipping the check; concurrent calls wait on one approval prompt, and the wait does not hold a tool-concurrency permit |
| 12 | Hooks and Plugins | `steps/ch10-hooks` | traits as extension points | before/after tool hooks, an API-request hook, rewriting a tool result, and a swappable memory backend; a hook may enable a toolset, and the availability check stays in the registry from chapter 3 |
| 13 | Sandboxed Execution | `steps/ch11-sandbox` | a child process, a snapshot | a code tool that runs outside the harness process, plus a file checkpoint and rewind |
| 14 | When Things Fail | `steps/ch12-when-things-fail` | matching on errors | classify failures, retry, fall back; MCP crashes, timeouts, reconnects; a timeout releases its concurrency permit. Deferred from chapter 9: empty-response retries after tool calls (`agent/conversation_loop.py`), length continuation when a reply is cut off (`agent/turn_truncation.py`), and the repetition guard (`agent/repetition_guard.py`) |
| 15 | Limits and Breakers *(server)* | `steps/gw-limits` | a permit and a breaker | per-tool concurrency limits and queues, timeouts, retries only when the tool is marked idempotent, circuit breakers, hedging, and 429/503 backpressure |
| 16 | Providers | `steps/ch13-providers` | another adapter behind the same trait | native Anthropic and Bedrock adapters beside the chapter 1 OpenRouter client; OpenRouter fallback and routing (a `models` array and provider preferences); key rotation on the pooled client; a cheaper side model for summaries |
| 17 | Streaming | `steps/ch14-streaming` | folding a stream | server-sent deltas as the chapter 1 `Event` values, overlapped with safe tool execution, for each adapter |

**Guardrail notes**

- **Chapter 11.** The nine-layer map (L1–L9 plus X) from the catalog. The approval pipeline order is hardline, then deny, then allowlist, then smart, then ask (`tools/approval.py` `check_all_command_guards`, catalog A1; the layer diagram states that order). Hardline patterns are never bypassable, including in YOLO (`tools/approval_detection.py` `HARDLINE_PATTERNS`, B1). Dangerous-command normalization and detection are B2 (`detect_dangerous_command`, `DANGEROUS_PATTERNS`). The wait times out at 300 seconds and denies (`approvals.timeout: 300`, A5). Cron, `-q`, and unattended mode deny by default (`approvals.cron_mode` / `single_query_mode` / `unattended_mode: "deny"`, A4). A denial tells the model the user has not consented and must not retry or rephrase (`_STOP_COMMAND`, `_STOP_ACTION`, A6; breaker threshold 3).
- **Chapter 12.** A plugin `pre_tool_call` hook can veto or rewrite a call. The hook returns `(block_message, final_args)` (`agent/tool_executor.py`, about line 672). The same hook can escalate to the human gate (A8, `tools/approval.py` `request_tool_approval`).
- **Chapter 13.** The sandbox is the real boundary, and Hermes leaves it off (`terminal.backend: "local"`, D1). When the backend is Docker: `--cap-drop ALL`, `--security-opt no-new-privileges`, and a pids limit (`tools/environments/docker.py`, D2). Child env is an allowlist plus declared passthrough (`tools/code_execution_env.py`, `tools/mcp_tool_config.py` `_build_safe_env`, D3; `tools/env_passthrough.py`, D4). Process-tree timeouts and `kill_process_tree` are I6 (`agent/deadline.py`). File-path checks stay defense-in-depth, as chapter 3 already quotes from `agent/file_safety.py`.
- **Chapter 14.** Empty-response retries (J4, `agent/empty_response_guard.py`: 3 retries, or 1 if one attempt costs more than $0.25) and the repetition guard (J3, `agent/repetition_guard.py`: at least 5 contiguous repeats covering at least half the output). Both were deferred from chapter 9.
- **Chapter 15.** Server-side rate limits and breakers are this chapter's own topic. Also the per-turn caps in I3 (`agent/tool_guardrails.py` `LoopCapConfig`): 50 web searches and 50 subagent spawns. Chapter 9 already counts a tool named `delegate` against `max_subagent_iterations` (default 50) and returns `error: subagent iteration cap`. The web-search cap of 50 is not implemented yet.
- **Chapters 16 and 17.** Outbound message sanitization (K4, `agent/message_sanitization.py`), think-block stripping (K5, `agent/think_scrubber.py` `StreamingThinkScrubber`), and TLS via the OS trust store (`agent/ssl_verify.py`; `ssl_verify: false` is for development only). Chapter 9 uniquifies duplicate tool-call ids inside one batch; the rest of K4 is still open.

**Performance budget**

- **Chapter 0.** State the rule: overhead is measured without model latency.
- **Chapter 1.** Four benches, each under 1 ms. 1.1: serde of 50 messages (measured 6.96 µs). 1.2: one mock call that also builds events (measured 235.7 ns) and 100 concurrent calls (measured 108.9 µs, target < 50 ms). 1.3: build an OpenRouter request and parse a response that carries `model`, `usage`, and `cost` (measured 2.133 µs). 1.4: parse a response with usage and one without (measured 792.7 ns). One HTTP client, pooled, HTTP/2 enabled. The default endpoint is OpenRouter.
- **Chapter 2.** The system prompt is byte-identical across the turns of a session. Per-turn overhead stays < 1 ms (measured 628.2 ns). Submitting one turn to the session inbox is < 1 µs (measured 32.78 ns). The provider borrows the history.
- **Chapter 3.** A lookup in a 50-tool registry is < 1 µs (measured 13.96 ns). The available-tool list for 50 tools is < 1 ms (measured 21.89 µs). Eight read-only tools that each sleep 1 ms finish together in 2.362 ms, against 17.55 ms one at a time. A write runs alone.
- **Chapter 4.** Eight remote reads that each sleep 40 ms overlap on the chapter 3 bound. A stdio echo round trip measured 71.29 µs. Streamable HTTP on loopback measured 215.7 µs. Fifty zero-delay calls measured 1.908 ms concurrent and 3.603 ms sequential. A cold tools/list measured 82.87 µs, and the cache hit measured 168.0 ns.
- **Chapter 5.** *(server)* Thirty-two in-flight calls accepted, eight executed at once. A permit round trip measured 45.53 ns. Eight 1 ms sleeps measured 2.129 ms.
- **Chapter 6.** Schema-token estimate for 10,000 tools < 50 ms (measured 21.06 ms). Search over that index < 1 ms (measured 146.8 µs). The lazy prompt renders in 12.26 µs, and its core bytes stay identical when a result is revealed or evicted. A cache hit on `tools/list` is 17.56 ns. Filtering one agent's servers out of 10,000 tools is 77.00 µs. These are the client-half numbers. The step crates are `ch05-*`.
- **Chapter 7.** *(server)* Search over 10,000 tools measured 299.0 µs. Rebuilding that index measured 27.42 ms. Swapping a one-tool `Arc` while holding the old one measured 2.467 µs.
- **Chapter 8.** *(server)* On 2,000 synthetic tools, schema-token estimates were 45,104 (every tool), 153 (static), and 135 (dynamic), all three successful. The dynamic arm measured 5.687 ms. The every-tool arm measured 8.355 ms.
- **Chapter 9.** Ten sections, crates `ch06-*`, section labels 6.1–6.10. Loop-iteration overhead < 1 ms excluding the model (measured 608.0 ns on the finished loop; 248.5 ns on the section 6.1 loop). Checking the cancellation token or the budget is < 1 µs (measured 11.92 ns). A cancelled call does not start the next model request. Eight 1 ms reads in one turn measured 2.421 ms. Name repair 188.8 ns, stall observe 100.0 ns, a budget notice 58.20 ns, truncation 3.174 µs, a steer append 47.12 ns, canonical arguments 771.0 ns. Re-running the saved eval set and the four stress evals on the mock stays inside the worker bound.
- **Chapter 10.** A state transition measured 1.103 ns. A three-step scripted turn measured 7.803 ns.
- **Chapter 11.** A skip decision is < 1 µs (measured 252.3 ns). Concurrent approvals serialize onto one prompt. Waiting on a human releases the tool permit.
- **Chapter 12.** An empty before/after hook adds < 0.1 ms (measured 10.91 ns). An API-request hook does not copy the conversation (measured 2.202 ns on 1,000 messages). A memory-backend call that only records the turn enqueues in < 1 ms (measured 54.88 ns).
- **Chapter 13.** A warm sandbox accepts a job in < 50 ms (measured 29.74 µs). Checkpoint and rewind of a 1,000-path manifest are each < 10 ms (measured 50.91 µs and 81.06 µs).
- **Chapter 14.** A timeout cancels the in-flight call and releases its permit. Retries stay inside the bound. The empty-response budget check measured 0.9046 ns. The repetition guard on an 800-character loop measured 2.580 µs.
- **Chapter 15.** *(server)* A timeout releases that tool's permit. A retry runs only when the tool is marked idempotent. An open breaker stops new calls to that backend (measured 30.01 ns). One execute-permit round trip measured 60.17 ns.
- **Chapter 16.** Choosing the next native adapter, or the next OpenRouter route, adds < 1 ms, excluding the failed call (measured 6.229 ns). Rotation reuses the pooled client from chapter 1 (measured 3.471 ns). A side-model summary does not run on the user turn unless that turn asked for it.
- **Chapter 17.** Assembling a streamed reply from chapter 1 events adds < 1 ms versus a buffered reply, excluding model time. Folding 100 plain chunks measured 2.806 µs. Joining the same chunks measured 659.7 ns.

### Part II: Learning

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 18 | Sessions on Disk | `steps/ch15-sessions` | SQLite, WAL | an append-only session store and a way to resume |
| 19 | Memory Files | `steps/ch16-memory` | bounded text, atomic writes | `MEMORY.md` and `USER.md`, a `memory` tool, a frozen snapshot in the system prompt, injection scanning; the file backend behind the trait from chapter 12 |
| 20 | Recall | `steps/ch17-recall` | FTS5 | a `session_search` tool: discover, scroll, read, browse |
| 21 | Learned Tools *(server)* | `steps/gw-learned-tools` | a prediction off the request path | learned tool prediction from memory and sessions, plus composite and macro tools and a code-mode tool. The prediction changes the offered set. It does not run inside the model call |
| 22 | Using Skills | `steps/ch18-skills` | a directory of markdown | `SKILL.md` (agentskills.io-compatible), a cache-stable `<available_skills>` index, `skills_list` / `skill_view` |
| 23 | Writing Skills | `steps/ch19-skill-writing` | patch and validate | create, patch, delete, and supporting files, with a scanner, the approval gate from chapter 11, and read-before-write |
| 24 | The Reflection Loop | `steps/ch20-reflection` | a supervised child task | nudge counters, the post-turn hook from chapter 12, a forked reviewer with only skill and memory tools |
| 25 | Curation | `steps/ch21-curation` | a scheduled job | usage telemetry, stale then archived, pinning, backups, optional consolidation. A write uses chapter 23's scan and the A9 gate, off the request path |

**Guardrail notes**

- **Chapter 19.** A memory threat scan on writes and on loaded entries (F5, `tools/memory_tool_store.py`). `MEMORY.md` and `USER.md` enter the prompt once per session and stay frozen (F6, `tools/memory_tool.py`). A mid-session write does not rewrite that snapshot.
- **Chapter 23.** Skill and memory writes go through the approval gate (A9, `tools/write_approval.py` `evaluate_gate`, `stage_write`). The gate is off unless `memory.write_approval` or `skills.write_approval` is set. Skills Guard scans a community skill before install (H1, `tools/skills_guard.py` `scan_skill`, `should_allow_install`). `force` cannot override a dangerous verdict on a community skill.
- **Chapter 25.** Curation that writes a skill or a memory file uses the same A9 gate, and an install uses H1. The scan stays off the request path, with the rest of this chapter's work.

**Performance budget**

- **Chapter 18.** One local WAL append of a turn < 1 ms (measured 143.9 µs). FTS updates are a background task. The chapter re-runs the chapter 9 eval set.
- **Chapter 19.** The frozen snapshot is built once and shared by `Arc`. A memory write does not rewrite the system prompt. Enqueue of a background write < 1 ms (measured 53.23 ns).
- **Chapter 20.** Discover over 10,000 messages < 20 ms (measured 60.27 µs), and only when the model calls the tool.
- **Chapter 21.** *(server)* Prediction stays off the request path except for the offered set. Applying that set measured 94.69 ns. Scoring 1,000 usage rows against 64 names measured 174.4 µs, and `session_tools` does not call it.
- **Chapter 22.** Load and render 500 skills < 10 ms, once per session (measured 1.946 ms). The index is `Arc` and byte-stable.
- **Chapter 23.** Security scan of one skill < 5 ms (measured 255.1 ns). A write the model did not ask for is a background task (enqueue measured 21.15 ns).
- **Chapter 24.** Spawning the reviewer adds < 1 ms to turn finalize (measured 60.01 ns). Cancelling it when the user returns takes < 50 ms (measured 758.7 µs).
- **Chapter 25.** Curation never runs on the request path. Enqueue measured 29.05 ns.

### Part III: The team patterns

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 26 | The Sequential Pipeline | `steps/ch22-pipeline` | an agent as a value you can pass around | outline, draft, critique, revise; each stage is an agent run in order |
| 27 | The Supervisor | `steps/ch23-supervisor` | the tool trait, used for agents | the default team: a manager, specialists called as tools and fanned out under a bound, one `Arc`-shared context object; the reflection loop learns from team runs off the hot path. Chapter 24's forked agent and prompt-cache parity stay out of that note: the team run only enqueues it. Chapter 26's live model per stage is not required here; specialists stay scripted |
| 28 | Handoffs | `steps/ch24-handoffs` | a pointer to the current agent | routing and ownership transfer; handoff-as-tool; a max-handoff limit; context filtered before the next agent sees it |
| 29 | Fan-out and Gather | `steps/ch25-fan-out` | a worker limit, an aggregator | the dedicated gather: the supervisor already runs specialists in parallel; this chapter adds the worker limit's tests and the aggregator. Chapter 27 defers the queue of 100 and the `task_index` order to here |
| 30 | The Critic | `steps/ch26-critic` | the same tool trait | an evaluator agent, both as a tool and as a final gate. It takes one permit from the chapter 27 pool. Chapter 29's summary is what it reads; it does not do the work |
| 31 | Composing Patterns | `steps/ch27-composition` | nesting the patterns above | a supervisor that fans out and uses a handoff only when ownership should move. Chapter 30 defers the judge continuation loop, the `wait` target, and the quality-gate shell commands to here |

**Guardrail notes**

- **Chapter 27.** A child cannot call `delegate_task`, `clarify`, `memory`, `send_message`, or `cronjob_manage` (M5, `tools/delegate_tool_toolsets.py` `DELEGATE_BLOCKED_TOOLS`). Depth is 1 (M6, `tools/delegate_tool_config.py` `MAX_DEPTH`). Hermes caps concurrent children at 10 (`delegation.max_concurrent_children`). The performance budget on this row stays eight specialists; that number is the harness measurement, not the Hermes default. Each child can get its own git worktree (D6, `tools/subagent_worktree.py`), and Hermes leaves `delegation.worktree_isolation` opt-in.

**Performance budget**

- **Chapter 26.** The next stage receives `Arc` context (measured 10.78 ns). Copying 100 messages measured 7.089 µs and is not the hand-off.
- **Chapter 27.** Up to eight specialists at once, each with `Arc` context. Eight specialists that each slept 40 ms finished in 0.044 s in nextest. Scheduling eight with no sleep measured 98.54 µs. A reflection note measured 21.23 ns and does not run the reviewer.
- **Chapter 28.** Filtering a 100-message transcript for a handoff < 1 ms (measured 2.740 µs).
- **Chapter 29.** The worker limit holds. One hundred mock subagents can be scheduled without an unbounded queue (scheduling 100 measured 4.635 µs). The 101st is refused.
- **Chapter 30.** The critic shares `Arc` context and takes one permit from the same bound (one judgment measured 175.3 ns).
- **Chapter 31.** Nested fan-out cannot exceed the worker limit. Eight children on the shared pool measured 103.0 µs. Width 9 is refused before any task is scheduled.

### Part IV: Making it robust

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 32 | Managing Context | `steps/ch28-context` | slicing a `Vec` | compression, structured summaries from the side model in chapter 16, `[SKILL_PRUNED]` markers, a memory flush before compression. Deferred from chapter 9: preflight compression before the model call (`agent/turn_context.py`, `agent/context_compressor.py`) |
| 33 | Agents as Actors | `steps/ch29-actors` | bounded channels | one tokio task per agent; the mailbox is the session inbox from chapter 2; steering and backpressure |
| 34 | Supervision and Restart | `steps/ch30-supervision` | atomics | restart a failed child. The cancellation token and the budget are already on the call from chapter 9 |
| 35 | Balancing and Federation *(server)* | `steps/gw-balancing` | a pool per tool | least-outstanding or power-of-two-choices across a per-tool backend pool, and federation of upstream MCP servers through that pool. Placed after cancellation (chapter 9) and supervision (chapter 34) |

**Guardrail notes**

- **Chapter 32.** Context files such as `AGENTS.md`, `SOUL.md`, and `.cursorrules` are scanned, and a match is replaced with `[BLOCKED: … Content not loaded.]` (F2, `agent/prompt_builder.py` `_scan_context_content`, `agent/context_file_sources.py`). The size cap is 20,000 characters, keeping the head and a tail (F3, `CONTEXT_FILE_MAX_CHARS`). High-risk tool output is wrapped as an untrusted result, and an embedded closer is neutralized (F4, `agent/tool_dispatch_helpers.py` `_maybe_wrap_untrusted`, `_neutralize_delimiters`).

**Performance budget**

- **Chapter 32.** Compression and the memory flush run between turns or in the background. The hot path sends the `Arc` snapshot (measured 15.12 ns). Preflight compression, deferred from chapter 9, runs before the model call.
- **Chapter 33.** The session inbox capacity is 64. One hundred or more mock agents at once. A full inbox applies backpressure. One submit measured 208.2 ns.
- **Chapter 34.** Restarting a failed child adds < 1 ms, excluding the child's work (one record measured 32.47 ns).
- **Chapter 35.** *(server)* Picking a backend is a local choice among a pool (one least-outstanding pick measured 2.502 ns). Discovery connects at most 4 at once.

### Part V: Running as a service

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 36 | Profiles | `steps/ch31-profiles` | config, a secret map | several personas in one config, each with its own secrets, kept out of the system prompt |
| 37 | The Gateway | `steps/ch32-gateway` | the session inbox | platform adapters submit to the chapter 2 inbox; a mid-turn message becomes an interrupt or a steer; streaming folds chapter 1 events into one message. This is the chat gateway, not the MCP server. Chapter 28 defers the platform re-bind from `handoff.request` to here |
| 38 | MCP Gateway Service *(server)* | `steps/gw-service` | `rmcp`, axum, a shared store | mostly stateless replicas routed by session id, hot reload via `arc-swap`, shared sessions and rate limits in Redis or something like it, health and readiness, OAuth 2.1 as the MCP resource server, per-tenant quotas, audit logs, OpenTelemetry, and a drain that finishes in-flight calls. Not written |
| 39 | Cron | `steps/ch33-cron` | a durable claim | at-most-once runs across crashes, catch-up, a watchdog, and chaining the next job by submitting to the same inbox |
| 40 | Editor Integration | `steps/ch34-acp` | a protocol adapter | ACP (Agent Client Protocol) as another client of the chapter 2 session inbox |

**Guardrail notes**

- **Chapter 36.** Secrets are scoped per profile. In multiplex mode, `get_secret` fails closed with `UnscopedSecretError` when no scope is installed (E4, `agent/secret_scope.py`).
- **Chapter 37.** The chat gateway, not the MCP server. Authorization is default-deny after the allowlist and pairing checks (L1a, `gateway/authz_mixin.py` `_is_user_authorized`). A pairing code is 8 characters, lives 1 hour, and five failures lock the caller out for 1 hour (L2a, `gateway/pairing.py` `PairingStore`; max 3 pending). Two bots answering each other hit the bot-loop guard: 20 bot messages in 300 seconds, then a 600 second cooldown (J5, `gateway/bot_loop_guard.py`). Emergency stop is a sentinel from `hermes pause` that fails safe on a stat error (N2, `agent/estop.py`).
- **Chapter 38.** Auth for this process is the OAuth 2.1 resource-server plan already in the row. The catalog has no gateway-OAuth row; Hermes is the MCP client. The related client checks are a per-server sampling limit and form-mode elicitation routed through approval (M7, `tools/mcp_tool_sampling.py`), and an OSV malware check before an npx or uvx launch (H4, `tools/osv_check.py`, 10 second timeout).
- **Chapter 39.** A cron prompt is scanned before it runs (M1, `tools/cronjob_prompt_scan.py`). A job that would restart or stop the gateway, or kill its interpreter, is blocked (M2, `cron/lifecycle_guard.py` `check_gateway_lifecycle`). Unattended cron denial of dangerous commands is A4, written with chapter 11.

**Performance budget**

- **Chapter 36.** Resolving a profile and its secrets is < 1 ms (one resolve measured 59.49 ns). Secrets are not copied into the prompt.
- **Chapter 37.** Enqueue of a platform message is < 1 ms. A steer is a push onto a bounded queue. Streaming updates one message.
- **Chapter 38.** *(server)* Throughput and p99 under load (oha or something like it). Killing one replica fails over by session id. Drain finishes the calls already accepted. No measurement yet.
- **Chapter 39.** A crash retries a claimed job at most once. Scanning 1,000 due jobs is < 10 ms. The watchdog and a chained job stay off the request path.
- **Chapter 40.** Translating an ACP update onto the session queue is < 1 ms.

### Part VI: Loosely coupled teams

| Ch | Title | Step | Rust, just in time | What the harness gains |
|---|---|---|---|---|
| 41 | The Blackboard | `steps/ch35-blackboard` | shared state behind a lock | shared state, agents woken by a condition, a thin scheduler |
| 42 | The Kanban Board | `steps/ch36-kanban` | a compare-and-swap claim | a durable board; separate processes claim, complete, review, or block a task |
| 43 | The Message Bus | `steps/ch37-message-bus` | bounded channels | peer-to-peer messaging with backpressure |
| 44 | Shared Team Skills | `steps/ch38-team-skills` | a single writer | a layered skill store, `skill_propose` on the bus, a librarian agent, `skills.updated`, a versioned library, `TEAM.md` |
| 45 | A2A for External Agents | `steps/ch39-a2a` | traits at the process boundary | A2A only for agents outside this process; MCP already covers tools, agents, and skills we expose |
| 46 | Seeing Inside | `steps/ch40-tracing` | tracing | spans on the chapter 1 events and across the gateway, the same criterion bench with tracing on, a capstone research team that learns across runs |
| 47 | Load and Token Capstone *(server)* | `steps/gw-capstone` | a load generator | the load test (throughput and p99) together with tokens per completed task for every tool, a static set, and a dynamic set, each with task success. Not written |
| 48 | Evals | `steps/ch41-evals` | a batch over the benches | re-run the saved tasks from chapter 9, plus trajectories, scored with the chapter 1 and chapter 9 benches, still with no fail threshold |

**Guardrail notes**

- **Chapter 48.** One red-team eval per layer, L1 through L9 and the cross-cutting layer. The checklist is section 4 of the catalog. The catalog does not name an eval suite or a file to copy. The saved tasks from chapter 9 stay the regression set; these cases are the extra set, still with no fail threshold.

**Performance budget**

- **Chapter 41.** A claim < 1 ms with 100 waiters. The lock is not held across a model call.
- **Chapter 42.** Claim, complete, review, or block is one transaction < 5 ms. Two processes cannot hold the same task.
- **Chapter 43.** Publish of one small envelope < 1 ms at 100 agents. A slow subscriber hits a bounded channel.
- **Chapter 44.** `skill_propose` enqueues in < 1 ms. The index swap is an `Arc` swap. Workers do not wait on the librarian.
- **Chapter 45.** Remote calls use the HTTP pool and the same bound. Cancel reaches an in-flight call in < 50 ms.
- **Chapter 46.** A span per event adds < 0.1 ms. The chapter 1 bench still meets its budget with tracing on. Spans cross the harness and the gateway. The capstone runs 100 or more concurrent mock agents.
- **Chapter 47.** *(server)* The load test and the three-arm token comparison. No measurement yet.
- **Chapter 48.** Re-running the saved eval set, and one hundred mock trajectories, stays inside the worker bound. Appending one trajectory event is < 1 ms.

Chapters 1 through 4, chapter 6, and chapters 9 through 20 have harness step crates. Chapter 6 is five sections under `steps/ch05-*`. Chapter 9 is ten sections under `steps/ch06-*`, and the section labels in that chapter stay 6.1–6.10 so they match the crates. Chapter 10 is `steps/ch07-turn-state`. Chapters 5, 7, 8, 15, and 21 have server step crates. `mcp-gateway` re-exports the chapter 21 learned gateway, which wraps the chapter 15 limits and still opens an empty catalog. Chapter 6's 10,000-tool benches still use a synthetic catalog. A real `tools/list` enters that cache through `cards_from_listed`. Chapter 9 starts a saved eval set that later chapters re-run, and chapter 48 scores that set. Pull requests and pushes to `main` report the Criterion median, low, and high in the Benchmarks workflow and upload one JSON artifact per commit. A push to `main` appends the run to the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/) on `gh-pages`. The same job still updates the `dev/bench` chart when it can publish. There is no fail threshold. Later rows are the plan, not empty crates. The same list is `.github/chapters.json`, which CI uses. Draft rows in `book/SUMMARY.md` are the chapters that are not written yet. Planned harness steps keep the directory names already in the CI filters (`ch07-turn-state` is book chapter 10). Server steps use the `gw-` prefix.

**Recurring sidebars:**

- "How Hermes does it": file paths and design notes from NousResearch/hermes-agent.
- "Guardrails": the layer from [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md), and which catalog ids the chapter adopts or leaves for later. The line is in [`docs/chapter-template.md`](../docs/chapter-template.md).
- Rust, introduced when the harness needs it.
- "[personal note]": the author's own experience while working through the chapter.
