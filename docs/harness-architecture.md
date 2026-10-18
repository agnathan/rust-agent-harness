# Growing harness, chapters 10–48

Chapters 1–10 grow one harness by copying the previous step and adding the new behavior. From chapter 11 onward the book had drifted into standalone demo crates. This document is the plan that puts it back. Batch 1 (chapters 11–20, plus the chapter 10 driver) is done in the same change as this file. Later batches do 21–30, 31–40, and 41–48 from the rules below.

A later step is a new copy, not a path dependency and not a patch of the previous directory. `steps/README.md` already says that. Unique Cargo package names stay. The library name inside a harness step stays `harness_core`.

## Two lineages

The book has two processes. They do not merge into one crate.

**Harness lineage.** Chapter 9's finished loop (`steps/ch09-10-evals`) is copied into chapter 10. Each later harness chapter copies the previous harness chapter and adds modules:

```text
ch09-10-evals
  → ch10-turn-state
  → ch11-approvals
  → ch12-hooks
  → ch13-sandbox
  → ch14-when-things-fail
  → ch16-providers          (chapter 15 is the server; the harness does not copy it)
  → ch17-streaming
  → ch18-sessions
  → ch19-memory
  → ch20-recall
  → ch22-skills … ch34, ch36, ch37, ch39–ch46, ch48
```

Chapter 16 copies chapter 14, not chapter 15. Team chapters copy the previous harness chapter. Chapter 22 copies chapter 20. Chapter 26 copies chapter 25, and so on. Do not restart from a standalone demo.

**Server lineage.** The gateway is its own crate. It also grows by copy:

```text
ch05-minimal-server
  → ch07-catalog
  → ch08-session-toolsets
  → ch15-limits             (chapter 7 and chapter 8 sources are vendored inside this crate)
  → ch21-learned-tools
  → ch35-balancing
  → ch38-service
  → ch47-capstone
```

Chapter 15 no longer path-depends on `ch07-catalog` or `ch08-session-toolsets`. Their sources live in `steps/ch15-limits/src/catalog/` and `src/session.rs`. The public names chapter 21 already calls are unchanged: `Catalog` (the chapter 7 `Gateway`), `LimitedGateway` (`new`, `catalog`, `open_session`, `session_tools`), `SessionView`. The lib name stays `gw_limits`. The package stays `ch15-limits`. The bench stays `limits`.

Chapter 21 still path-depends on chapter 15. The batch that reworks chapter 21 vendors chapter 15's sources into `ch21-learned-tools` the same way, and keeps lib name `gw_learned` plus every type `mcp-gateway` imports (`LearnedGateway`). Chapters 35, 38, and 47 are standalone today. Their batches copy the previous server crate in as modules and add the new behavior on top, so a reader who opens only that step still has the catalog, the session view, the limits, and the learned offer. Keep each chapter's existing package name, lib name, and `[[bench]]` name. Those are what `.github/workflows/benchmarks.yml` passes to `cargo bench -p`.

`mcp-gateway/` stays the process crate. It re-exports the latest server step. After chapter 21 it re-exports `LearnedGateway`. After chapter 47 it re-exports the capstone. Do not bind a socket in any of these steps.

## Harness crate layout

From chapter 10 on, a harness step looks like this:

```text
steps/chNN-<slug>/
  crates/harness-core/     package name = the old bench package (ch11-approvals, …)
    src/lib.rs             lib name harness_core
    src/bin/chapter_demo.rs    bin name chNN-<slug>, except chapter 10's bin is ch10-turn-state
    src/bin/mcp_fixture.rs     bin name chNN_mcp_fixture
    src/bin/worker.rs          from chapter 13 on, bin name sandbox-worker
    benches/<one bench>        the bench name already in benchmarks.yml
    examples/demo.rs
  crates/harness-cli/      package chNN-harness-cli, bin chNN-harness
```

Rules that are easy to get wrong:

- The package name is the bench package (`ch11-approvals`), not `harness-core`. `cargo bench -p ch11-approvals --bench decision` has to keep working.
- Do not glob-reexport feature modules. `approval`, `hooks`, `sandbox`, and the others export names that collide (`run_script`, `Message`). Tests call `harness_core::<module>::`. `lib.rs` re-exports only the loop, the turn table, and (from chapter 18) the redaction functions.
- The MCP fixture include is `include!("../../../../../ch04-fixture/src/main.rs")`. That path is correct only at `steps/chNN/crates/harness-core/src/bin/`. Do not flatten the package.
- Rename the fixture binary every chapter (`ch11_mcp_fixture`, …). Many packages otherwise all install `mcp_fixture` and overwrite `target/debug/mcp_fixture`. `session.rs` reads `option_env!("CARGO_BIN_EXE_chNN_mcp_fixture")` and falls back to that filename.
- Keep `sandbox-worker` from chapter 13 onward. `tests/sandbox.rs` and `benches/sandbox.rs` compile `env!("CARGO_BIN_EXE_sandbox-worker")`.
- Drop inherited `[[bench]]` entries. The workflow runs one bench per chapter. An extra bench still gets `clippy --all-targets`. Chapter 9's `loop_iter` is not copied forward. The `demo` example is copied.
- `Cargo.lock` is committed. CI uses `--locked`.

Feature modules, in the chapter that adds them. Later copies keep the module.

| Chapter | Module | Was the standalone lib |
|---|---|---|
| 10 | `turn` | the old `turn_state` crate, now driven by the loop |
| 11 | `approval` | `approvals` |
| 12 | `hooks` | `hooks` |
| 13 | `sandbox` | `sandbox` |
| 14 | `failure` | `failures` |
| 16 | `adapters` | `providers` (`provider.rs` is already the chapter 9 client) |
| 17 | `stream` | `streaming` |
| 18 | `wal` and `redact` | `sessions` (`session.rs` is the MCP session) |
| 19 | `memory_files` | the memory crate |
| 20 | `recall` | the recall crate, and it calls `crate::wal`, not a `sessions` crate |

## How the loop picks up a feature

Chapter 10: `run_loop` and `run_loop_with_inbox` call `drive_loop`, which holds a `turn::Driver` and calls `advance` at every observation. `run_loop_driven` returns `(TurnOutcome, Phase)`. `note` panics if the move is illegal, so a chapter 9 outcome that the table cannot name fails the test instead of drifting.

Three events exist because the live loop has moves the original 15-row table did not name:

- `NoDone` — the provider batch had no `Done`. From `CallingModel` or `WrappingUp` the phase is `Budget`.
- `BudgetAbort` — `CallingModel` to `Budget` when the counter is already 0 after a refund. This is not `BudgetHit`, which would enter `WrappingUp`.
- `SoftWrapIgnoredTools` — a soft-stop wrap-up asked for tools or returned no text. The phase is `SoftStopped`. Those tools are not run. A budget wrap-up that asks for tools stays `Partial` via `WrapUpToolCalls`.

A budget hit while the phase is still `Idle` and no model call has started is `BudgetHit` straight to `Budget`. It does not enter `CallingModel`. Cancel before start is `Start` then `HardCancel`, because `Idle` + `HardCancel` is illegal. `Driver::advance` records `WrapKind` on `SoftStop`, and on `BudgetHit` only when the phase is `CallingModel`. When the next phase is `Finished`, it applies `finish_wrap` so soft stop and budget do not both land on `Finished`.

Chapter 11: `approval::deny_tool` runs before `dispatcher.run_raced`. A denial is pushed as the tool result and the tool body is not called. The loop never takes a permit for that call, so a human wait cannot hold one. `Ask` with no prompt installed becomes a denial. `run_call` is still the path that waits, and it drops the permit first. Only `terminal` and `execute_code` are gated. `echo` and `delegate` are unchanged.

Chapter 12: hooks and the plugin shape check are on the `hooks` module. The loop does not call the hook runner yet; the chapter's own tests do. Wiring every hook into `drive_loop` without changing chapter 9's eval results is the follow-on inside chapter 12's crate if a later edit needs it. Do not let a hook hold the dispatch permit across the human wait.

Chapter 13: `deny_tool` calls `sandbox::check_execute_code` for `execute_code`. The default loop path passes interactive presence, mode off false, yolo false, and `whole_script` false, so a local interactive script is approved here and its `terminal` calls still hit chapter 11. Cron, single-query, and unattended deny. An isolated `env_type` (anything other than `local`) approves. `whole_script` asks once.

Chapter 14: `approval::check` calls `failure::command_deletes_runtime` and `failure::self_repo_git_mutation` before the fast path and before YOLO. The runtime path is `std::env::current_exe()`. The source root is `CARGO_MANIFEST_DIR` of that step. Both denials are hardline.

Chapter 18: `wal::Store::append_turn` runs both strings through `redact::redact_sensitive_text` before the insert. `redact_for_egress` is the same scrub plus the Bearer residue sweep. Later chapters that log, store, or send text call these functions. They do not grow a second, narrower scrub.

## Copy recipe for a later harness chapter

1. Copy `steps/ch(N-1)-<slug>/crates/` to `steps/chN-<slug>/crates/`. Skip chapter 15 when the previous harness chapter is 14: chapter 16 copies chapter 14.
2. In `harness-core/Cargo.toml`, set `package.name` to the name `.github/workflows/benchmarks.yml` already passes (see the bench table below). Set the demo bin name to `chN-<slug>`. Set the fixture bin name to `chN_mcp_fixture`. Keep `sandbox-worker` if `tests/sandbox.rs` is present. Replace the `[[bench]]` with the one bench that chapter already had.
3. Point `session.rs` at `CARGO_BIN_EXE_chN_mcp_fixture`.
4. Move the old standalone `src/lib.rs` in as `pub mod <module>;`. Rewrite its `use old_lib::` to `use crate::<module>::` or `crate::`. Do not `pub use` the whole module from the crate root.
5. Copy the old tests and benches. Rewrite imports the same way. Add one integration test that the new function is actually reachable from the behavior the chapter claims (the loop, the store, the gateway), not only as a free function.
6. Rewrite `harness-cli` to package `chN-harness-cli`, bin `chN-harness`, depending on the new package.
7. Replace the workspace member `"steps/chN-<slug>"` with the two crate paths.
8. Update `benchmarks/descriptions.json` `source` to the new bench path. Do not change `id` or `function`.
9. Add `my-harness/tests/chNN.rs` that calls a `todo!("chapter NN")` stub. Do not implement the learner's solution in `my-harness/`. The step crate is the reference.
10. Fix the chapter's pasted nextest summary from a real `cargo nextest run -p <package>`. Count binaries honestly. Many old chapters say "across 3 binaries" for a crate that had one test binary; that sentence is a lie and has to go.
11. Quote benchmark means from that PR's Benchmarks workflow (`mean.point_estimate` in the uploaded estimates), not from a laptop. Do not rewrite `gh-pages` history. Do not move git tags. List the retag in the PR body.

Inherited chapter 9 tests must still pass. If a new guard would deny `echo`, the guard is wrong.

## Per-chapter plan, 11–48

"Milestone" is the file `my-harness/tests/chNN.rs`. The stub panics with `todo!("chapter NN")` until the reader fills it in. The my-harness workflow records the failure and exits 0 unless someone dispatches it with "fail the workflow" set. The tests must compile: `cargo clippy --workspace --all-targets` builds them.

Bench IDs are `section/function` from `benchmarks/descriptions.json`. Keep both the `id` and the `function` field. The dashboard keys history by that id.

| Ch | Adds | Deferrals this chapter closes | Milestone | Bench IDs to keep |
|---|---|---|---|---|
| 11 | `approval`: the pipeline, `deny_tool` in front of dispatch | A1, A4, A5, A6, B1, B2, as the chapter already claims | `ch11.rs`: a hardline `terminal` command is denied and the tool body does not run | `11/skip_decision` |
| 12 | `hooks`, plus `scan_plugin_shape` / `should_allow_plugin_install` | A8 (a hook can escalate). **H3 structural half**: 400 files, 10 MB total, 1 MB per file, oversized is caution, `--force` cannot pass dangerous. The pattern engine stays chapter 23 (H1) | `ch12.rs`: an oversized plugin is caution; force does not pass dangerous | `12/empty_pre_post`, `12/api_request_hook`, `12/memory_enqueue` |
| 13 | `sandbox`, and `deny_tool` calls `check_execute_code` | **A2** `execute_code` script gate. D1–D4 and I6 as the chapter already claims | `ch13.rs`: cron cannot run a local script; a container env approves; whole-script ask is one prompt | `13/warm_accept`, `13/checkpoint_1000`, `13/rewind_1000` |
| 14 | `failure`, wired into the approval floor before YOLO | J3, J4, and the chapter 9 deferrals already named. **B5** self-repo git mutation. **B6** runtime self-delete. Alias lookup, heredocs, and `shutil.rmtree` inside code stay out, matching Hermes's own limits | `ch14.rs`: YOLO still denies deleting the running binary and `git reset --hard` of this crate's manifest dir | `14/empty_budget`, `14/repetition_guard` |
| 15 | server: vendored catalog + session, then permits, idempotent retry, breaker, caps of 50 | I3 web-search cap of 50 (`web_search`) and subagent cap (`delegate_task`). The outline used to say the web-search cap was not implemented. It is. | `ch15.rs`: the 51st `web_search` is the cap error; an open breaker does not call the backend | `15/open_breaker`, `15/permit_release` |
| 16 | `adapters` beside the chapter 9 provider | K4's remaining body repair, as far as this chapter's tests already go. TLS stays on the OS store | `ch16.rs`: an invalid route falls through to the fallback model; rotation keeps the same client | `16/choose_route`, `16/rotate_key` |
| 17 | `stream` | K5 think-block scrub on the way through the folder | `ch17.rs`: a split think tag does not leak; folding plain chunks matches a buffer | `17/fold_deltas`, `17/buffer_join` |
| 18 | `wal` plus `redact` | **E1** `redact_sensitive_text` / `redacting_formatter`. **E2** `redact_for_egress` (Bearer residue → `Bearer [redacted]`). Prefix families actually read from `_PREFIX_PATTERNS`, env assignment, `Authorization`, PEM. Plugin patterns and the vault are not copied. `append_turn` stores the scrubbed text | `ch18.rs`: a turn that contains `sk-…` and `OPENAI_API_KEY=` is stored without the secret; egress strips a bare Bearer | `18/wal_append` |
| 19 | `memory_files` behind the chapter 12 memory trait | F5, F6 as the chapter already claims. Writes go through the same redaction if they echo a secret into the file | `ch19.rs`: a poisoned entry is blocked before it reaches the file; the snapshot is not rewritten | `19/memory_enqueue` |
| 20 | `recall` over the chapter 18 store | none new. FTS ids were queued in 18; this chapter builds the search | `ch20.rs`: discover hides a subagent and demotes cron | `20/discover_10k` |
| 21 | server copy of chapter 15, plus learned prediction, composite/macro tools, code mode | the offered set changes before the request. Prediction does not run inside the model call | `ch21.rs`: `session_tools` reads the predicted set and does not call the scorer | `21/apply_offered`, `21/predict_offline` |
| 22 | `skills` module on the chapter 20 harness | cache-stable `<available_skills>` | `ch22.rs`: loading 500 skills does not change the index bytes when one is viewed | `22/load_render_500` |
| 23 | skill create/patch/delete on that module | **H1** full skills-guard scan (the pattern half H3 deferred). A9 write gate, off unless configured. Force cannot override dangerous | `ch23.rs`: a dangerous community skill is blocked even with force | `23/scan_one_skill`, `23/enqueue_unsolicited` |
| 24 | forked reviewer, post-turn hook | the reviewer only has skill and memory tools | `ch24.rs`: spawning the reviewer does not run it on the user turn | `24/spawn_reviewer`, `24/cancel_review` |
| 25 | curation off the request path | reuses A9 and H1. The scan stays off the request path | `ch25.rs`: enqueue does not write | `25/enqueue_curation` |
| 26 | pipeline of agent runs | none new | `ch26.rs`: the next stage receives `Arc` context | `26/arc_pass`, `26/copy_transcript` |
| 27 | supervisor, specialists as tools | M5 blocked child tools, M6 depth 1 | `ch27.rs`: eight specialists share one pool; a reflection note does not run the reviewer | `27/schedule_eight`, `27/enqueue_reflection` |
| 28 | handoff | context filtered before the next agent. Platform re-bind stays chapter 37 | `ch28.rs`: a 100-message filter drops what the next agent must not see | `28/filter_100` |
| 29 | fan-out gather | the queue of 100 and `task_index` order deferred from 27. Heartbeats are **not** this chapter | `ch29.rs`: the 101st task is refused | `29/schedule_100` |
| 30 | critic | one permit from the chapter 27 pool | `ch30.rs`: the critic reads the summary and does not do the work | `30/judge_one_permit` |
| 31 | composition | judge continuation, `wait` target, quality-gate shell commands deferred from 30 | `ch31.rs`: nested width 9 is refused before any task starts | `31/nested_eight` |
| 32 | context compression | F2, F3, F4. Preflight compression deferred from chapter 9 | `ch32.rs`: the hot path sends the `Arc` snapshot | `32/share_snapshot` |
| 33 | one task per agent, chapter 2 inbox as the mailbox | backpressure at capacity 64 | `ch33.rs`: a full inbox does not grow without bound | `33/inbox_submit` |
| 34 | restart a failed child | **Heartbeats**, deferred by both 27 and 29. Hermes `tools/delegate_tool.py`: `_start_heartbeat`, `_HEARTBEAT_INTERVAL = 30` seconds, `_HEARTBEAT_STALE_CYCLES_IDLE = 15` (450s), `_HEARTBEAT_STALE_CYCLES_IN_TOOL = 40` (1200s). Re-read those constants on current `main` before quoting them; they have moved before. Progress is an iteration, a tool change, or `last_activity_ts`. An in-flight model wait refreshes that timestamp, so a slow model is not idle | `ch34.rs`: a child with no progress for the idle threshold is stale; one that is inside a tool uses the longer threshold | `34/restart_bookkeeping` |
| 35 | server copy of chapter 21, plus a per-tool backend pool and federation | discovery connects at most 4. No socket. `0` as unlimited stays unimplemented | `ch35.rs`: least-outstanding pick does not open a connection | `35/pick_least` |
| 36 | profiles and a secret map | E4 `get_secret` fails closed when no scope is installed | `ch36.rs`: an unscoped secret read is the error, and the secret is not in the prompt | `36/resolve_profile` |
| 37 | chat gateway into the chapter 2 inbox | L1a, L2a, J5, N2 as the outline already says. **B4** `tools/terminal_tool_guards.py` `gateway_lifecycle_block`: a shell command that would restart or stop this gateway is denied. This is the chat gateway, not the MCP server. Chapter 39's M2 is the cron-prompt form of the same idea and stays there. Outbound text calls `redact_for_egress` | `ch37.rs`: a gateway restart command is blocked; a platform enqueue is not a model call | `37/enqueue_platform` |
| 38 | server copy of chapter 35, plus routes, drain, quotas, bearer check | M7 sampling limit, H4 OSV check before npx/uvx. No socket. OAuth issuance stays deferred | `ch38.rs`: killing the route record fails the next lookup over to the other replica id | `38/route_session` |
| 39 | durable cron claim | M1 prompt scan, M2 `check_gateway_lifecycle` (already the chapter's job; do not move it to 37). Unattended dangerous-command denial stays chapter 11's A4 | `ch39.rs`: a claimed job retries at most once; a lifecycle command in the job is blocked | `39/scan_thousand` |
| 40 | ACP adapter on the session inbox | none new | `ch40.rs`: one ACP update becomes one inbox submit | `40/translate_update` |
| 41 | blackboard | the lock is not held across a model call | `ch41.rs`: 100 waiters, one claim | `41/claim_with_waiters` |
| 42 | durable kanban | compare-and-swap claim | `ch42.rs`: two claims of one task, one wins | `42/claim_transaction` |
| 43 | message bus | replace the narrow scrub this chapter has today with `redact_for_egress` from chapter 18. Do not keep a second redactor | `ch43.rs`: a published envelope that contains a key is scrubbed; a slow subscriber hits the bound | `43/publish_to_agents` |
| 44 | team skill library and librarian | the index swap is an `Arc` swap | `ch44.rs`: `skill_propose` enqueues and workers do not wait | `44/skill_propose`, `44/index_swap` |
| 45 | A2A for agents outside this process | outbound text calls `redact_for_egress`. MCP stays the path for tools we own | `ch45.rs`: cancel reaches an in-flight remote call | `45/cancel_inflight` |
| 46 | tracing on chapter 1 events, and the research team | the capstone research team lives here. Chapter 47 is the load and token comparison, not this team | `ch46.rs`: a span per event still finishes the mock turn | `46/event_without_span`, `46/span_per_event` |
| 47 | server copy of chapter 38, plus the three token arms and an in-memory load | no socket. oha is not measured | `ch47.rs`: dynamic, static, and every-tool arms each report tokens and task success | `47/dynamic_tokens`, `47/load_100` |
| 48 | re-run chapter 9's saved evals, plus one red-team case per layer L1–L9 and the cross-cutting layer | the checklist is section 4 of `docs/hermes-guardrails.md`. No fail threshold | `ch48.rs`: the saved set still passes on the mock; one appended trajectory event is ordered | `48/append_event`, `48/saved_set` |

## Where the orphaned deferrals live

| Id | Hermes | Home | State after batch 1 |
|---|---|---|---|
| A2 | `tools/approval.py` `check_execute_code_guard` | **13** | Implemented. Cron/unattended deny a local script. Isolated env approves. Whole-script ask is one prompt. Interactive CLI approves the script and still gates `terminal`. |
| B4 | `tools/terminal_tool_guards.py` `gateway_lifecycle_block` | **37** | Not this batch. Chapter 39 keeps M2 for cron prompts. |
| B5 | `tools/self_repo_guard.py` `detect_self_repo_git_mutation` | **14** | Implemented, adapted. Global `git -C`, wrappers `sudo` / `command` / `time` / `nohup` / `env`, the worktree-mutation subcommands, and `reset --hard/--merge/--keep`. Alias lookup and heredocs are not parsed. Hermes only activates the block on Windows (`guard_active`); this harness runs the detector on every platform. |
| B6 | `agent/runtime_self_protection.py` `command_deletes_runtime`, `is_protected_path` | **14** | Implemented, adapted. The caller names the runtime path (`current_exe` in the floor) because there is no `sys.executable`. Covers `rm` / `rmdir` / `rd` / `del` / `erase` / `remove-item` / `ri`, `find … -delete` / `-exec rm`, and `uv python uninstall` when the spec is `--all` or appears as `cpython-<spec>` on the path. Overlap is both directions, so deleting a parent of the binary is a hit. `xargs` and code-level `rmtree` stay out. |
| E1 | `agent/redact.py` `redact_sensitive_text`, `RedactingFormatter` | **18** | Implemented as `redact_sensitive_text` and `redacting_formatter`. Mask is the prefix through the first `-` or `_`, then `***`. |
| E2 | `redact_for_egress` | **18** | Implemented beside E1. Chapters 37, 43, and 45 call it. Chapter 43's narrower scrub is replaced, not kept. |
| heartbeats | `tools/delegate_tool.py` `_start_heartbeat` | **34** | Plan only. Chapters 27 and 29 both deferred it; it does not belong in either. |
| H3 | `tools/plugin_guard.py` `scan_plugin`, `should_allow_plugin_install` | **12** for the structural caps and the force rule; **23** for the skills-guard pattern engine (H1) | Structural half implemented. Caps: `MAX_PLUGIN_FILE_COUNT = 400`, total 10 MB, one file 1 MB. Verdicts safe/caution/dangerous map to Allow / Confirm (force overrides caution) / Block (force does not override dangerous). Oversized is caution, not a hard block. `scan_plugin_shape` never returns Dangerous by itself. |

## my-harness

`my-harness/` is the reader's crate. `steps/` is the finished reference. Milestone tests compile and panic with `not yet implemented` until the reader fills the stub in. Changing `my-harness/src` selects every chapter that has a `tests/chNN.rs` in the my-harness workflow. Do not break compilation of `ch01.rs` and `ch02.rs`.

## Benchmarks and tags

Do not change a bench `id` or `function` to "improve" the name. The gh-pages dashboard appends points for that id. New means belong in the chapter after the PR's Benchmarks workflow, as `mean.point_estimate`. The printed Criterion `time:` line is the slope, not the mean.

Do not move existing `step-NN` tags. After this batch merges, tags `step-10` through `step-20` point at commits whose trees are no longer the chapters. The PR body lists the retag. Nobody force-moves them in the batch itself.

## What batch 1 changed that later batches must not undo

- Chapter 10's `drive_loop` is the thing that walks the table. Do not split the table back out into a crate that never calls `run_loop`.
- Chapters 11–14 and 16–20 are copies. Do not replace them with the old standalone crates.
- Chapter 15 vendors chapter 7 and chapter 8 and still builds `ch21-learned-tools`.
- Redaction, the plugin shape check, the execute_code gate, and the two command floors are in the modules named above. Later chapters call them. They do not reimplement a smaller version.
