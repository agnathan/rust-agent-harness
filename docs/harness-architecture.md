# Growing harness, chapters 10–53

Chapters 1–10 grow one harness by copying the previous step and adding the new behavior. From chapter 11 onward the book had drifted into standalone demo crates. This document is the plan that puts it back. Batch 1 (chapters 11–22, plus the chapter 10 driver) is done. Batch 2 (chapters 23–32) is done. Batch 3 (chapters 33–44, including the Browser Tools and Vision insert) is done. Batch 4 (chapters 45–52) is done. Chapter 53 copies the chapter 52 harness and closes the learning loop on one task. The harness line ends at chapter 53. The server line ends at chapter 51.

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
  → ch16-web-tools          (chapter 15 is the server; the harness does not copy it)
  → ch17-local-tools
  → ch18-providers
  → ch19-streaming
  → ch20-sessions
  → ch21-memory
  → ch22-recall
  → ch24-skills
  → … ch32-critic
  → ch33-composition
  → ch34-context
  → ch35-actors
  → ch36-supervision
  → ch38-profiles        (chapter 37 is the server; the harness does not copy it)
  → ch39-gateway
  → ch40-browser
  → ch41-vision
  → ch43-cron            (chapter 42 is the server)
  → ch44-acp
  → ch45-blackboard … ch50-tracing, ch52-evals
  → ch53-the-loop-closes
```

Chapter 33 copies chapter 32. Chapter 38 copies chapter 36. Chapter 40 copies chapter 39. Chapter 43 copies chapter 41. Chapter 45 copies chapter 44. Chapter 46 copies 45, and so on through chapter 50. Chapter 52 copies chapter 50. Chapter 51 is the server copy of chapter 42.

Chapter 16 copies chapter 14, not chapter 15. Chapter 17 copies chapter 16. Chapter 18 copies chapter 17. Team chapters copy the previous harness chapter. Chapter 24 copies chapter 22. Chapter 28 copies chapter 27, and so on. Do not restart from a standalone demo.

**Server lineage.** The gateway is its own crate. It also grows by copy:

```text
ch05-minimal-server
  → ch07-catalog
  → ch08-session-toolsets
  → ch15-limits             (chapter 7 and chapter 8 sources are vendored inside this crate)
  → ch23-learned-tools
  → ch37-balancing
  → ch42-service
  → ch51-capstone
```

Chapter 15 no longer path-depends on `ch07-catalog` or `ch08-session-toolsets`. Their sources live in `steps/ch15-limits/src/catalog/` and `src/session.rs`. The public names chapter 23 already calls are unchanged: `Catalog` (the chapter 7 `Gateway`), `LimitedGateway` (`new`, `catalog`, `open_session`, `session_tools`), `SessionView`. The lib name stays `gw_limits`. The package stays `ch15-limits`. The bench stays `limits`.

Chapter 23 vendors chapter 15's sources into `ch23-learned-tools` the same way chapter 15 vendored chapters 7 and 8. The lib name stays `gw_learned`. There is no path dependency on `ch15-limits`. Chapter 37 vendors that learned gateway and adds `federation`. Its lib name is `balancing`. Chapter 42 vendors chapter 37 and adds `service`. Its lib name is `gwservice`. Chapter 51 copies chapter 42 and adds `capstone`. The lib name stays `gwservice`. The package stays `ch51-capstone`. The bench stays `capstone`. Keep each chapter's package name, lib name, and `[[bench]]` name. Those are what `.github/workflows/benchmarks.yml` passes to `cargo bench -p`.

`mcp-gateway/` stays the process crate. It re-exports chapter 51: `gwservice::LearnedGateway` as `Gateway`, plus `gw_session` from chapter 8. It does not path-depend on `ch23-learned-tools`. Do not bind a socket in any of these steps.

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
- Do not glob-reexport feature modules. `approval`, `hooks`, `sandbox`, and the others export names that collide (`run_script`, `Message`). Tests call `harness_core::<module>::`. `lib.rs` re-exports only the loop, the turn table, and (from chapter 20) the redaction functions.
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
| 16 | `web` | new in this batch. Not a renamed standalone crate |
| 17 | `local_tools` | new in this batch. Not a renamed standalone crate |
| 18 | `adapters` | `providers` (`provider.rs` is already the chapter 9 client) |
| 19 | `stream` | `streaming` |
| 20 | `wal` and `redact` | `sessions` (`session.rs` is the MCP session) |
| 21 | `memory_files` | the memory crate |
| 22 | `recall` | the recall crate, and it calls `crate::wal`, not a `sessions` crate |

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

Chapter 20: `wal::Store::append_turn` runs both strings through `redact::redact_sensitive_text` before the insert. `redact_for_egress` is the same scrub plus the Bearer residue sweep. Later chapters that log, store, or send text call these functions. They do not grow a second, narrower scrub.

## Copy recipe for a later harness chapter

1. Copy `steps/ch(N-1)-<slug>/crates/` to `steps/chN-<slug>/crates/`. Skip chapter 15 when the previous harness chapter is 14: chapter 16 copies chapter 14. Chapter 17 copies chapter 16. Chapter 18 copies chapter 17.
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
12. Write the chapter to the length and shape in the next section. A short tour of the new module is not a chapter.

Inherited chapter 9 tests must still pass. If a new guard would deny `echo`, the guard is wrong.

## Chapter length and shape

Chapters 1–4 run about 3,500–7,000 words. Every chapter from 10 through 53 matches that size and this shape. Do not ship a chapter that only names the new functions.

- Numbered sections (`## N.1`, `## N.2`, …). Each section walks one type or one function.
- Full listings of that code, copied from the step crate. A reader who has not opened the file can still read the function.
- The reason for each design choice, next to the code that embodies it.
- Rust explained for a reader who is new to the language, in a sidebar at the moment the feature appears (`enum`, `match`, `Result`, `Option`, ownership, `&` and `&mut`, `async`, traits, `Arc`, and whatever else that section uses).
- Real run output in **Build, run, and benchmark**: the nextest summary from this tree, the milestone demo, and benchmark means from that pull request's Benchmarks workflow (`mean.point_estimate`). The printed Criterion `time:` line is the slope, not the mean.
- A **Pitfalls** section.
- A **What you implement** exercise that names `my-harness/tests/chNN.rs` and the assertion the reader has to make pass. The stub stays `todo!`. Do not paste the solution into `my-harness/`. Chapter 10 has no learner stub; its exercise is the phase predictions in that chapter.

The word-count band is the result of walking the code. A paragraph that does not teach a type, a branch, or a design reason does not belong. Do not repeat the summary to inflate the count.

## Per-chapter plan, 11–52

"Milestone" is the file `my-harness/tests/chNN.rs`. The stub panics with `todo!("chapter NN")` until the reader fills it in. The my-harness workflow records the failure and exits 0 unless someone dispatches it with "fail the workflow" set. The tests must compile: `cargo clippy --workspace --all-targets` builds them.

Bench IDs are `section/function` from `benchmarks/descriptions.json`. Keep both the `id` and the `function` field. The dashboard keys history by that id.

| Ch | Adds | Deferrals this chapter closes | Milestone | Bench IDs to keep |
|---|---|---|---|---|
| 11 | `approval`: the pipeline, `deny_tool` in front of dispatch | A1, A4, A5, A6, B1, B2, as the chapter already claims | `ch11.rs`: a hardline `terminal` command is denied and the tool body does not run | `11/skip_decision` |
| 12 | `hooks`, plus `scan_plugin_shape` / `should_allow_plugin_install` | A8 (a hook can escalate). **H3 structural half**: 400 files, 10 MB total, 1 MB per file, oversized is caution, `--force` cannot pass dangerous. The pattern engine stays chapter 25 (H1) | `ch12.rs`: an oversized plugin is caution; force does not pass dangerous | `12/empty_pre_post`, `12/api_request_hook`, `12/memory_enqueue` |
| 13 | `sandbox`, and `deny_tool` calls `check_execute_code` | **A2** `execute_code` script gate. D1–D4 and I6 as the chapter already claims | `ch13.rs`: cron cannot run a local script; a container env approves; whole-script ask is one prompt | `13/warm_accept`, `13/checkpoint_1000`, `13/rewind_1000` |
| 14 | `failure`, wired into the approval floor before YOLO | J3, J4, and the chapter 9 deferrals already named. **B5** self-repo git mutation. **B6** runtime self-delete. Alias lookup, heredocs, and `shutil.rmtree` inside code stay out, matching Hermes's own limits | `ch14.rs`: YOLO still denies deleting the running binary and `git reset --hard` of this crate's manifest dir | `14/empty_budget`, `14/repetition_guard` |
| 15 | server: vendored catalog + session, then permits, idempotent retry, breaker, caps of 50 | I3 web-search cap of 50 (`web_search`) and subagent cap (`delegate_task`). The outline used to say the web-search cap was not implemented. It is. | `ch15.rs`: the 51st `web_search` is the cap error; an open breaker does not call the backend | `15/open_breaker`, `15/permit_release` |
| 16 | `web`: `web_search` and `web_extract` behind `SearchBackend` | I3's tool name stays `web_search` so chapter 15's cap of 50 applies. No second counter. SSRF floor and secret-URL refusal are this chapter | `ch16.rs`: a metadata address is not a safe extract URL | `16/fake_search`, `16/ssrf_block` |
| 17 | `local_tools`: files, terminal, `todo`, `clarify` | path root, stale write, terminal through chapter 11 and chapter 13. A child may not call `clarify` | `ch17.rs`: a write without a prior read is refused | `17/patch_apply`, `17/todo_merge` |
| 18 | `adapters` beside the chapter 9 provider | K4's remaining body repair, as far as this chapter's tests already go. TLS stays on the OS store | `ch18.rs`: an invalid route falls through to the fallback model; rotation keeps the same client | `18/choose_route`, `18/rotate_key` |
| 19 | `stream` | K5 think-block scrub on the way through the folder | `ch19.rs`: a split think tag does not leak; folding plain chunks matches a buffer | `19/fold_deltas`, `19/buffer_join` |
| 20 | `wal` plus `redact` | **E1** `redact_sensitive_text` / `redacting_formatter`. **E2** `redact_for_egress` (Bearer residue → `Bearer [redacted]`). Prefix families actually read from `_PREFIX_PATTERNS`, env assignment, `Authorization`, PEM. Plugin patterns and the vault are not copied. `append_turn` stores the scrubbed text | `ch20.rs`: a turn that contains `sk-…` and `OPENAI_API_KEY=` is stored without the secret; egress strips a bare Bearer | `20/wal_append` |
| 21 | `memory_files` behind the chapter 12 memory trait | F5, F6 as the chapter already claims. Writes go through the same redaction if they echo a secret into the file | `ch21.rs`: a poisoned entry is blocked before it reaches the file; the snapshot is not rewritten | `21/memory_enqueue` |
| 22 | `recall` over the chapter 20 store | none new. FTS ids were queued in 20; this chapter builds the search | `ch22.rs`: discover hides a subagent and demotes cron | `22/discover_10k` |
| 23 | server copy of chapter 15, plus learned prediction, composite/macro tools, code mode | the offered set changes before the request. Prediction does not run inside the model call | `ch23.rs`: `session_tools` reads the predicted set and does not call the scorer | `23/apply_offered`, `23/predict_offline` |
| 24 | `skills` module on the chapter 22 harness | cache-stable `<available_skills>` | `ch24.rs`: loading 500 skills does not change the index bytes when one is viewed | `24/load_render_500` |
| 25 | skill create/patch/delete on that module | **H1** full skills-guard scan (the pattern half H3 deferred). A9 write gate, off unless configured. Force cannot override dangerous | `ch25.rs`: a dangerous community skill is blocked even with force | `25/scan_one_skill`, `25/enqueue_unsolicited` |
| 26 | forked reviewer, post-turn hook | the reviewer only has skill and memory tools | `ch26.rs`: spawning the reviewer does not run it on the user turn | `26/spawn_reviewer`, `26/cancel_review` |
| 27 | curation off the request path | reuses A9 and H1. The scan stays off the request path | `ch27.rs`: enqueue does not write | `27/enqueue_curation` |
| 28 | pipeline of agent runs | none new | `ch28.rs`: the next stage receives `Arc` context | `28/arc_pass`, `28/copy_transcript` |
| 29 | supervisor, specialists as tools | M5 blocked child tools, M6 depth 1 | `ch29.rs`: eight specialists share one pool; a reflection note does not run the reviewer | `29/schedule_eight`, `29/enqueue_reflection` |
| 30 | handoff | context filtered before the next agent. Platform re-bind stays chapter 39 | `ch30.rs`: a 100-message filter drops what the next agent must not see | `30/filter_100` |
| 31 | fan-out gather | the queue of 100 and `task_index` order deferred from 29. Heartbeats are **not** this chapter | `ch31.rs`: the 101st task is refused | `31/schedule_100` |
| 32 | critic | one permit from the chapter 29 pool | `ch32.rs`: the critic reads the summary and does not do the work | `32/judge_one_permit` |
| 33 | composition | judge continuation, `wait` target, quality-gate shell commands deferred from 30 | `ch33.rs`: nested width 9 is refused before any task starts | `33/nested_eight` |
| 34 | context compression | F2, F3, F4. Preflight compression deferred from chapter 9 | `ch34.rs`: the hot path sends the `Arc` snapshot | `34/share_snapshot` |
| 35 | one task per agent, chapter 2 inbox as the mailbox | backpressure at capacity 64 | `ch35.rs`: a full inbox does not grow without bound | `35/inbox_submit` |
| 36 | restart a failed child | **Heartbeats**, deferred by both 29 and 31. Hermes `tools/delegate_tool.py`: `_start_heartbeat`, `_HEARTBEAT_INTERVAL = 30` seconds, `_HEARTBEAT_STALE_CYCLES_IDLE = 15` (450s), `_HEARTBEAT_STALE_CYCLES_IN_TOOL = 40` (1200s). Re-read those constants on current `main` before quoting them; they have moved before. Progress is an iteration, a tool change, or `last_activity_ts`. An in-flight model wait refreshes that timestamp, so a slow model is not idle | `ch36.rs`: a child with no progress for the idle threshold is stale; one that is inside a tool uses the longer threshold | `36/restart_bookkeeping` |
| 37 | server copy of chapter 23, plus a per-tool backend pool and federation | discovery connects at most 4. No socket. `0` as unlimited stays unimplemented | `ch37.rs`: least-outstanding pick does not open a connection | `37/pick_least` |
| 38 | profiles and a secret map | E4 `get_secret` fails closed when no scope is installed | `ch38.rs`: an unscoped secret read is the error, and the secret is not in the prompt | `38/resolve_profile` |
| 39 | chat gateway into the chapter 2 inbox | L1a, L2a, J5, N2 as the outline already says. **B4** `tools/terminal_tool_guards.py` `gateway_lifecycle_block`: a shell command that would restart or stop this gateway is denied. This is the chat gateway, not the MCP server. Chapter 43's M2 is the cron-prompt form of the same idea and stays there. Outbound text calls `redact_for_egress` | `ch39.rs`: a gateway restart command is blocked; a platform enqueue is not a model call | `39/enqueue_platform` |
| 40 | browser tools on the chapter 39 harness, behind `PageBackend` | secret URL, then SSRF (`allow_private` false), then open, then a recheck of `final_url`. Snapshot caps 15,000 and 2,000,000. Page text is `wrap_untrusted`. Vendors in `plugins/browser/` stay deferred | `ch40.rs`: a metadata URL is not navigated | `40/fake_navigate` |
| 41 | vision on the chapter 40 harness, behind `ImageSource` | download cap 50 MB, inline cap 20 MB, mime jpeg/png/gif/webp, PNG pixel cap 100,000,000. Caption is untrusted. Remote URLs use the chapter 16 SSRF floor | `ch41.rs`: an oversized image or a bad type is rejected | `41/sniff_png` |
| 42 | server copy of chapter 37, plus routes, drain, quotas, bearer check | M7 sampling limit, H4 OSV check before npx/uvx. No socket. OAuth issuance stays deferred | `ch42.rs`: killing the route record fails the next lookup over to the other replica id | `42/route_session` |
| 43 | durable cron claim on the chapter 41 harness | M1 prompt scan, M2 `check_lifecycle` (stays here; do not move it to 39). Unattended dangerous-command denial stays chapter 11's A4 | `ch43.rs`: a claimed job retries at most once; a lifecycle command in the job is blocked | `43/scan_thousand` |
| 44 | ACP adapter on the session inbox | none new | `ch44.rs`: one ACP update becomes one inbox submit | `44/translate_update` |
| 45 | blackboard on the chapter 44 harness | the lock is not held across a model call | `ch45.rs`: 100 waiters, one claim | `45/claim_with_waiters` |
| 46 | durable kanban on the chapter 45 harness | compare-and-swap claim | `ch46.rs`: two claims of one task, one wins | `46/claim_transaction` |
| 47 | message bus on the chapter 46 harness | `deliver` calls chapter 20 `redact_for_egress` | `ch47.rs`: a published envelope that contains a key is scrubbed; a slow subscriber hits the bound | `47/publish_to_agents` |
| 48 | team skill library and librarian on the chapter 47 harness | the index swap is an `Arc` swap. `allow_install` calls chapter 25 `scan_text` and `should_allow_install`. Community dangerous stays refused when `force` is set | `ch48.rs`: `skill_propose` enqueues and workers do not wait | `48/skill_propose`, `48/index_swap` |
| 49 | A2A on the chapter 48 harness | `call` passes `redact_for_egress` into `send`. MCP stays the path for tools we own | `ch49.rs`: cancel reaches an in-flight remote call | `49/cancel_inflight` |
| 50 | tracing on the chapter 49 harness, and the research team | the capstone research team lives here. Chapter 51 is the load and token comparison, not this team | `ch50.rs`: a span per event still finishes the mock turn | `50/event_without_span`, `50/span_per_event` |
| 51 | server copy of chapter 42, plus the three token arms and an in-memory load | no socket. oha is not measured. Token counts use `approx_tokens` (`div_ceil`) | `ch51.rs`: dynamic, static, and every-tool arms each report tokens and task success | `51/dynamic_tokens`, `51/load_100` |
| 52 | chapter 50 harness plus `suite`. Re-runs chapter 9's saved evals on `run_loop`, plus one red-team case per layer L1–L9 and X | each case calls the real module. No `repair_tool` stub. No fail threshold. `FAIL_THRESHOLD` is `None` | `ch52.rs`: the saved set still passes on the mock; one appended trajectory event is ordered | `52/append_event`, `52/saved_set` |
| 53 | chapter 52 harness plus `improve`. One task scored twice on `run_loop` | reflection notes the miss, `Writer::create` scans the skill, the curator archives a stale neighbor, `librarian_accept` shares the name, the second attempt calls `skill_view` | `ch53.rs`: the second attempt passes | `53/close_once` |

## Where the orphaned deferrals live

| Id | Hermes | Home | State after batch 1 |
|---|---|---|---|
| A2 | `tools/approval.py` `check_execute_code_guard` | **13** | Implemented. Cron/unattended deny a local script. Isolated env approves. Whole-script ask is one prompt. Interactive CLI approves the script and still gates `terminal`. |
| B4 | `tools/terminal_tool_guards.py` `gateway_lifecycle_block` | **39** | Implemented. A supervised shell that would restart, stop, or uninstall the gateway is denied. Unsupervised returns none. Chapter 43 keeps M2 for cron prompts. |
| B5 | `tools/self_repo_guard.py` `detect_self_repo_git_mutation` | **14** | Implemented, adapted. Global `git -C`, wrappers `sudo` / `command` / `time` / `nohup` / `env`, the worktree-mutation subcommands, and `reset --hard/--merge/--keep`. Alias lookup and heredocs are not parsed. Hermes only activates the block on Windows (`guard_active`); this harness runs the detector on every platform. |
| B6 | `agent/runtime_self_protection.py` `command_deletes_runtime`, `is_protected_path` | **14** | Implemented, adapted. The caller names the runtime path (`current_exe` in the floor) because there is no `sys.executable`. Covers `rm` / `rmdir` / `rd` / `del` / `erase` / `remove-item` / `ri`, `find … -delete` / `-exec rm`, and `uv python uninstall` when the spec is `--all` or appears as `cpython-<spec>` on the path. Overlap is both directions, so deleting a parent of the binary is a hit. `xargs` and code-level `rmtree` stay out. |
| E1 | `agent/redact.py` `redact_sensitive_text`, `RedactingFormatter` | **20** | Implemented as `redact_sensitive_text` and `redacting_formatter`. Mask is the prefix through the first `-` or `_`, then `***`. |
| E2 | `redact_for_egress` | **20** | Implemented beside E1. Chapters 39, 47, and 49 call it. Those chapters do not define a second scrubber. |
| heartbeats | `tools/delegate_tool.py` `_start_heartbeat` | **36** | Implemented as `is_stale`. Interval 30s. Idle stale at 15 cycles (450s). In a tool, 40 cycles (1200s). Progress is a new iteration, a different tool, or a newer `last_activity_ts`. The function does not sleep and does not start a thread. |
| H3 | `tools/plugin_guard.py` `scan_plugin`, `should_allow_plugin_install` | **12** for the structural caps and the force rule; **25** for the skills-guard pattern engine (H1) | Structural half implemented. Caps: `MAX_PLUGIN_FILE_COUNT = 400`, total 10 MB, one file 1 MB. Verdicts safe/caution/dangerous map to Allow / Confirm (force overrides caution) / Block (force does not override dangerous). Oversized is caution, not a hard block. `scan_plugin_shape` never returns Dangerous by itself. |

## my-harness

`my-harness/` is the reader's crate. `steps/` is the finished reference. Milestone tests compile and panic with `not yet implemented` until the reader fills the stub in. Changing `my-harness/src` selects every chapter that has a `tests/chNN.rs` in the my-harness workflow. Do not break compilation of `ch01.rs` and `ch02.rs`.

## Benchmarks and tags

Do not change a bench `id` or `function` to "improve" the name. The gh-pages dashboard appends points for that id. New means belong in the chapter after the PR's Benchmarks workflow, as `mean.point_estimate`. The printed Criterion `time:` line is the slope, not the mean.

Do not move existing `step-NN` tags. After this batch merges, tags `step-10` through `step-15` point at commits whose trees changed, and tags `step-16` through `step-48` point at the chapters that are now numbered two higher. New `step-16` and `step-17` tags do not exist yet. The PR body lists the retag. Nobody force-moves them in the batch itself.

## Planned chapters: built-in tools, browser, and vision

Chapters 16 and 17 are built. Browser Tools is chapter 40 and Vision is chapter 41. Both are written. The book is chapters 0–53. Means for the shifted chapters 10–15 and 18–22 stay the `mean.point_estimate` values from [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918). Means for `16/fake_search`, `16/ssrf_block`, `17/patch_apply`, and `17/todo_merge` are `mean.point_estimate` from [run 39114427940](https://github.com/agnathan/rust-agent-harness/actions/runs/39114427940). Means for chapters 23–32 stay the values from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589). Means for chapters 33–44 are the `mean.point_estimate` values from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). Means for chapters 45–52 are the `mean.point_estimate` values from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). The mean for `53/close_once` is 701.61 µs, `mean.point_estimate` from [run 40163536226](https://github.com/agnathan/rust-agent-harness/actions/runs/40163536226).

### Where the two tool chapters go

They are adjacent, in Part I, after chapter 15 (Limits and Breakers) and before Providers. That is the first place the loop, the approval floor, the sandbox, and the cap of 50 all exist. Putting them immediately after chapter 9 would teach the tools before the guardrails they call. Part I currently ends at Streaming, so both chapters stay in Part I.

The insert is applied. The book is chapters 0–53.

| After the insert | Was | Title | This batch |
|---|---|---|---|
| 16 | new | Built-in Tools I: Web Search and Extract | built |
| 17 | new | Built-in Tools II: Files, Terminal, Todo, and Clarify | built |
| 18 | 16 | Providers | renamed |
| 19 | 17 | Streaming | renamed |
| 20 | 18 | Sessions on Disk | renamed |
| 21 | 19 | Memory Files | renamed |
| 22 | 20 | Recall | renamed |
| 23–50 | 21–48 | the rest, same order | renamed |

Server chapters move with the same shift: Learned Tools becomes 23, Balancing 37, MCP Gateway Service 40, Load and Token Capstone 49. Harness copies skip the server directories. Chapter 16 copies chapter 14. Chapter 17 copies chapter 16. Chapter 18 copies chapter 17.

### What the rename must touch

Do not move git tags in the pull request. After it merges, retag so the content that was `step-16` through `step-48` is `step-18` through `step-50`, and add `step-16` and `step-17` for the new tool chapters. `step-01` through `step-15` stay on their numbers; `step-10` through `step-15` still need the retag onto the merge commit because those trees changed. List that in the PR body.

For each current chapter N from 16 to 48, the new number is N+2:

- `steps/chNN-<slug>` directory, and every `chNN` package name, bin name, and fixture bin inside it.
- Workspace members, `.github/chapters.json`, `.github/path-filters.yml`, `.github/workflows/benchmarks.yml` (the section id becomes N+2; the package argument follows the directory).
- `benchmarks/descriptions.json`: the `id` becomes `"<N+2>/<function>"`. Keep the old id in `aliases`, the same way `6.1/schema_tokens_1000` still aliases `5.1/schema_tokens_1000`. Do not change `function`. That is what keeps the gh-pages history on one line.
- Book filename `book/chapter-NN-….md`, `book/SUMMARY.md`, `TABLE_OF_CONTENTS.md`, `book/book-outline.md`, `docs/performance.md`, and prose that names the chapter by number. A path like `steps/ch18-providers` changes too.
- `my-harness/tests/chNN.rs` and the stub in `milestones.rs`.
- Chapter 15's public API stays so the learned-tools crate still builds. Its path dependency is updated to the new directory name when Learned Tools moves.

Bench ids for the two new chapters are new, not aliases of an older row: `16/fake_search`, `16/ssrf_block`, `17/patch_apply`, `17/todo_merge`. Quote `mean.point_estimate` from the Benchmarks run that builds them.

### Built-in Tools I: Web Search and Extract

Copy the harness at chapter 14 (the previous harness; chapter 15 is the server). Add `web` as `pub mod web`, not a glob re-export. The module is behind a `SearchBackend` trait so tests never need a key.

Hermes, read on current `main` (do not invent a path that is not in this list):

- `tools/web_tools.py` `web_search_tool` returns JSON `{"success", "data": {"web": [{title, url, description, position}]}}`. `limit` is clamped to 1…100, default 5. Metadata only. `_get_search_backend` prefers `web.search_backend`, then `web.backend`, then autodetect. Built-in names in `_BUILTIN_AVAILABILITY`: exa, parallel, keenable, firecrawl, tavily, perplexity, searxng, brave-free, ddgs, xai. Keys are env vars (`TAVILY_API_KEY`, `EXA_API_KEY`, and the rest of that map).
- `tools/web_tools.py` `web_extract_tool` is async. Order of gates, from `tools/web_tools_extract.py`: secret-URL refusal, then SSRF, then provider resolution, then per-URL website policy, then the disk cache, then the vendor. A cache hit must not skip a gate.
- `tools/web_tools_extract.py` `_validate_extract_urls` refuses the whole call when the URL or its unquoted form matches the redact prefix regex. The error starts `Blocked: URL contains what appears to be an API key or token.`
- `tools/url_safety.py` blocks private, loopback, link-local, reserved, and CGNAT (`100.64.0.0/10`). Cloud metadata hostnames `metadata.google.internal` and `metadata.goog`, and the metadata IPs (`169.254.169.254`, `169.254.170.2`, `169.254.169.253`, `100.100.100.200`, `fd00:ec2::254`, and the IPv4-mapped forms) stay blocked even when `security.allow_private_urls` is on. `async_is_safe_url` fails closed on DNS errors. `_MAX_SSRF_CONNECT_IPS` is 8.
- `tools/web_tools_truncate.py`: default extract budget `DEFAULT_EXTRACT_CHAR_LIMIT = 15000`, clamped to 2_000…500_000. Over budget is about 75% head and 25% tail, cut on line boundaries, plus a `[TRUNCATED]` footer. Stored full text is capped at `MAX_STORED_TEXT_CHARS = 2_000_000`. Inline `data:image/...;base64` becomes `[IMAGE]`. A binary magic-byte body is an error, not page text.
- `agent/tool_dispatch_helpers.py` `_maybe_wrap_untrusted`: `web_search` and `web_extract` are in `_UNTRUSTED_TOOL_NAMES`. Strings of at least 32 characters are wrapped in `<untrusted_tool_result source="…">` after `_neutralize_delimiters` rewrites an embedded `untrusted_tool_result` token to `untrusted-tool-result`. The wrapper says the block is data, not instructions.
- `agent/tool_guardrails.py` `_DEFAULT_MAX_WEB_SEARCHES_PER_TURN = 50`. The block text is `loop_web_search_cap`. Chapter 15 already returns `CallError::Cap` for a tool named `web_search` at `MAX_WEB_SEARCHES`. The new tool uses that name so the existing cap applies. Do not add a second counter.

Adopt the result shape, the clamp, the secret-URL refusal, the SSRF floor (metadata always blocked), the head/tail truncation, the base64 rewrite, and the untrusted wrapper. Adapt the backends to one trait: `FakeSearch` for tests and benches, and one optional real backend that reads a single env var (name it in the chapter; do not call the network in CI). Defer the vendor ladder (Firecrawl, Tavily, Exa, Perplexity, ddgs, keyless rescue, TTL memo) and `tools/website_policy.py`. Say so in the Hermes box.

The chapter is the chapter 1–4 shape: numbered sections, full listings, a Rust sidebar when the trait and `async` show up, real nextest output, `mean.point_estimate` from that PR's Benchmarks run, pitfalls, guardrails (I3, the SSRF floor, F4 wrapping), a What you implement exercise, and a social draft.

### Built-in Tools II: Files, Terminal, Todo, and Clarify

Same copy chain, one chapter later. The module sits on top of chapter 11's approval floor and chapter 13's sandbox. Hermes, read on current `main`:

- `tools/file_tools.py` `read_file_tool` / `write_file_tool` / `patch_tool`. `_DEFAULT_MAX_READ_CHARS = 100_000`. `patch_tool` takes `mode`, `path`, and `old_string`.
- `tools/file_tools_paths.py` `_authoritative_workspace_root` and `_path_resolution_warning`. A relative path that resolves outside the workspace root warns. Absolute paths are the caller's way to name a file under the root on purpose.
- `tools/file_state.py` `FileStateRegistry::check_stale` and `tools/file_tools_write_guards.py` `_stale_overwrite_blocker`. A `write_file` of a file the task has not read, or whose mtime moved, is refused (`stale_write_blocked`). `patch` is the edit that should land instead of clobbering.
- `tools/terminal_tool.py` routes the command through the environment backend. `FOREGROUND_MAX_TIMEOUT` defaults to 600 seconds (`TERMINAL_MAX_FOREGROUND_TIMEOUT`). `tools/terminal_tool_result.py` `_truncate_head_tail` uses `tools/tool_output_limits.py` `DEFAULT_MAX_BYTES = 50_000`. The approval callback is the same gate as chapter 11. This harness already denies through `deny_tool` before `run_raced`. The terminal tool must call that path, and `execute_code` still calls `check_execute_code`. Do not grow a second approval pipeline.
- `tools/todo_tool.py` `TodoStore` / `todo_tool`. One list, `merge` to update by id, `MAX_TODO_ITEMS = 256`, `MAX_TODO_CONTENT_CHARS = 4000`. A monotonic revision so a stale write can be rejected. The list is agent-owned state, not a file in the user's repo.
- `tools/clarify_tool.py` `clarify_tool`. At most `MAX_QUESTIONS = 5` questions and `MAX_CHOICES = 4` choices. It pauses for a user. Tests use the fake model and a fake user reply. `tools/delegate_tool_toolsets.py` `DELEGATE_BLOCKED_TOOLS` includes `clarify` (also `delegate_task`, `memory`, `send_message`, `cronjob_manage`). A child agent must not call `clarify`. Chapter 29 already states that block. This chapter implements the tool and keeps the block. Do not let a child pass it.

Adopt path-root checks, stale-read refusal, the read cap, head/tail terminal truncation, the timeout, todo merge, and the clarify pause. Adapt terminal execution to chapter 13's worker and chapter 11's `deny_tool` rather than porting Docker, sudo, and the gateway lifecycle block (B4 stays chapter 39). Defer SSH path resolution, container mirrors, and protected-instruction approval prompts; name them in the Hermes box.

Same chapter shape as Tools I. Milestone ideas: a path outside the workspace is refused; a stale write is refused; a hardline terminal command is denied and the body does not run; a child call to `clarify` is denied; `todo` merge updates one item. Benches stay keyless.

### Browser Tools and Vision

Written. They sit in Part V, after the gateway (chapter 39) and before the MCP gateway service (chapter 42). Editor Integration is chapter 44. The insert shifted every former chapter 40 and later up by two. Bench ids `40/fake_navigate` and `41/sniff_png` are new. Former ids from 40 upward are the new id plus two, with the old id kept as an alias.

Hermes, read on current `main` when the chapters were written:

- `tools/browser_tool.py` `evaluate_url_safety`, `_secret_url_error`, `_url_policy_error`, `browser_navigate`. `DEFAULT_SNAPSHOT_THRESHOLD` is 15000. `MAX_STORED_SNAPSHOT_CHARS` is 2_000_000.
- `agent/tool_dispatch_helpers.py` `_UNTRUSTED_TOOL_PREFIXES` includes `browser_`.
- `plugins/browser/` (browserbase, browser_use, firecrawl) stays deferred.
- `tools/vision_tools.py` `_VISION_MAX_DOWNLOAD_BYTES` is 50 MiB. `_MAX_BASE64_BYTES` is 20 MiB.
- `tools/vision_tools_image_prep.py` `_ANTHROPIC_SUPPORTED_MEDIA_TYPES` is jpeg, png, gif, and webp. `_VISION_MAX_VALIDATED_AGGREGATE_PIXELS` is 100_000_000.

The harness modules are `browser` (`PageBackend`, `FakeBrowser`) and `vision` (`ImageSource`, `FakeImages`). Tests and benches do not launch a browser and do not call a vision API.

## What batch 1 changed that later batches must not undo

- Chapter 10's `drive_loop` is the thing that walks the table. Do not split the table back out into a crate that never calls `run_loop`.
- Chapters 11–14 and 16–22 are copies. Do not replace them with the old standalone crates.
- Chapter 15 vendors chapter 7 and chapter 8 and still builds `ch23-learned-tools`.
- Redaction, the plugin shape check, the execute_code gate, and the two command floors are in the modules named above. Later chapters call them. They do not reimplement a smaller version.

## What batch 2 changed that later batches must not undo

- Chapter 23 vendors chapter 15. It does not path-depend on `ch15-limits`. `catalog`, `session`, and `limits` are modules in that crate. `LearnedGateway` is still the type `mcp-gateway` re-exports.
- Chapters 24–32 are copies of the previous harness. Chapter 24 copies chapter 22. Do not replace them with the old standalone crates. The new modules are `skills`, `skill_write`, `reflection`, `curation`, `pipeline`, `supervisor`, `handoffs`, `fanout`, and `critic`. `lib.rs` does not glob-reexport them.
- Chapter 33 still compiles by depending on chapter 32's harness and calling `harness_core::supervisor`, `handoffs`, `fanout`, and `critic`. Its batch copies chapter 32 instead of path-depending on four crates.
- H1, the skills-guard install policy and the pattern scan, and A9, the write-approval gate that stays off unless set, live in chapter 25. Chapter 27 calls those functions. It does not grow a second scanner.
- Heartbeats stay chapter 36. Chapter 31 does not start them. Hermes `_get_max_async_children` now ignores `delegation.max_async_children` and returns the concurrent-children cap. The harness queue of 100 is this book's cap, not that key.

## What batch 3 changed that later batches must not undo

- Chapters 33–36, 38–41, and 43–44 are copies of the previous harness. Chapter 33 copies chapter 32. Chapter 38 copies chapter 36. Chapter 40 copies chapter 39. Chapter 43 copies chapter 41. Do not replace them with standalone crates. The new modules are `composition`, `context`, `actors`, `supervision` (heartbeats live in that module), `profiles`, `gateway` (B4 and `send_out` live there), `browser`, `vision`, `cron`, and `acp`. `lib.rs` does not glob-reexport them.
- Chapter 37 vendors chapter 23. Lib name `balancing`, module `federation`. Chapter 42 vendors chapter 37. Lib name `gwservice`, module `service`. At the end of batch 3, `mcp-gateway` re-exported chapter 23's `LearnedGateway`. Batch 4 moves that re-export to chapter 51.
- Browser Tools is chapter 40. Vision is chapter 41. Former chapters 40–50 are 42–52. Bench ids moved with them and kept the old id as an alias. `40/fake_navigate` and `41/sniff_png` are new ids.
- Heartbeats are `is_stale` in chapter 36. B4 is `gateway_lifecycle_block` in chapter 39. Cron M2 stays in chapter 43.

## What batch 4 changed that later batches must not undo

- Chapters 45–50 and 52 are copies of the previous harness. Chapter 45 copies chapter 44. Chapter 52 copies chapter 50. Do not replace them with standalone crates. The new modules are `blackboard`, `kanban`, `bus`, `team_skills`, `a2a`, `spans`, and `suite`. `lib.rs` does not glob-reexport them.
- Chapter 51 copies chapter 42. Lib name `gwservice`, module `capstone`. It is not in the harness copy chain. `mcp-gateway` re-exports `gwservice::LearnedGateway` as `Gateway` and `gw_session` from chapter 8. No socket.
- `bus::deliver` and `a2a::call` call `crate::redact::redact_for_egress`. There is no second scrubber in those files.
- `team_skills::allow_install` calls chapter 25 `scan_text` and `should_allow_install`. The local `rm -rf /` check stays, because that scanner does not name the command. Community plus `force` does not override a dangerous body.
- Chapter 52 `suite` re-runs `evals::run_saved_evals` and `run_stress_evals` on this copy of `run_loop`. `unknown-tool-repair` is `repair_name` inside the loop (`eco` → `echo`). The red-team cases call the real modules. `FAIL_THRESHOLD` is `None`.
- The kanban two-process test uses the bin `chNN_kanban_claim` and `env!("CARGO_BIN_EXE_chNN_kanban_claim")`. Do not point that test at the chapter binary name. Cargo turns hyphens into underscores, and a shared bin name overwrites `target/debug`.
