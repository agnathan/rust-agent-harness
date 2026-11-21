# Chapter 8: Session Toolsets

On 2,000 tools, a fake "rename a file" succeeds three ways. Showing every schema costs 45,104 estimated tokens, a static handful costs 153, but search plus one describe costs 135. The dynamic arm is the one I would ship, because 135 tokens still name the tool the task needs and 45,104 name the whole catalog. This chapter is the session view that produces that third number.

Chapter 7's gateway has one catalog, so every caller who can search it can see every name. A session should see a smaller set: the tools a task starter picked, plus the names a search revealed, with the full schema staying off that list until something asks for it. The step crate is `steps/ch08-session-toolsets`. It depends on `ch07-catalog`. It binds no socket.

`SessionGateway` is the type a client holds. `set_task` installs a starter set for a task description. `search_tools` returns a short line per hit, `name — blurb`, and refuses more than 7 queries. `describe` returns the server and the full description, and refuses more than 10 names. Those two caps are the ones Hermes puts on its bridge tools. Revealed names accumulate on the session. `SessionView` is what `session_tools` returns: the names this session may see, not the whole catalog. `take_list_changed` flips a flag back to false, which is the bit a later socket would send as `notifications/tools/list_changed`. Results page at 2,000 characters. `Telemetry` records schema tokens, call tokens, and result tokens. `Arm` and `TaskReport` are the three-way comparison: every tool, a static set, and a dynamic set. `run_rename_task` is the fake task those arms share. Token estimates divide bytes by 4.

The server stays a separate program because the session filter is a catalog decision. The harness already knows how to keep a prompt prefix stable. It should receive a small list, not discover the small list by downloading every schema and throwing most of them away. Putting `set_task` and the revealed-name filter in the gateway means two harness processes can share one catalog and still see different sessions. Hermes refuses a session-keyed catalog on the client, because that copy would drift from the live definitions. This server is the source of the list, so the session stores names and the catalog stores cards. `mcp-gateway`, the process crate, re-exports `gw_session` from this chapter — `SessionView`, `Arm`, `Telemetry`, `run_rename_task` — beside the chapter 51 gateway. The re-export is how the process crate stays thin. It still binds no socket.

The three arms are the measurement chapter 51 will report again, on a copied catalog, with task success beside the token count. Collecting the numbers here means the later capstone has a definition of "dynamic" that is this session view. A learned offer, chapter 23, changes which names `session_tools` returns, and it uses this same session type.

A session is a struct keyed by an id. A flag you take and clear is the notification. An enum (`Arm`) is the three experiments, so a fourth way of offering tools has to be added on purpose. The token estimate is integer division, the same four-characters-per-token rule chapter 6 used.

`cargo run -p ch08-session-toolsets` prints the fake task. No port is opened.

**In this chapter**

- What this chapter decides
- The fake task
- Tests
- Build, run, and benchmark
- Pitfalls
- Guardrails in this chapter
- Chapter summary
- What's next

| Section | Step | What it adds |
|---|---|---|
| 8 | `steps/ch08-session-toolsets` | `set_task`, `search_tools`, `describe`, paging, telemetry |

> **How Hermes does it**
>
> `tools/tool_search.py` caps one bridge call at `_MAX_QUERIES_PER_CALL` (7) and `_MAX_DESCRIBE_NAMES_PER_CALL` (10). The module doc says a session-keyed catalog drifts from the live definitions, so Hermes rebuilds on every assembly. `tools/mcp_tool_health.py` treats `tools/list_changed` as the signal to refresh, and it ignores `prompts/list_changed` and `resources/list_changed`. `CHARS_PER_TOKEN` in `tools/tool_search_catalog.py` is 4.0. I searched for a `set_task` tool. Hermes does not have one. `tools.include` and `tools.exclude` are install-time filters, described in the gateway notes.
>
> **Adopted.** More than 7 queries is `TooManyQueries`. More than 10 describe names is `TooManyNames`. A search returns a short line per hit, `name — blurb`, and `describe` returns the server and the full description. Token estimates divide bytes by 4. `take_list_changed` flips back to false, which is the flag a later socket will send as `notifications/tools/list_changed`.
>
> **Adapted.** The session is keyed. Hermes refuses that for the client's catalog because it would drift. This server is the source of the list, so the session stores names and the catalog stores cards. When the revealed names change, `list_changed` becomes true. The client in chapter 4 already drops its cache on that notification. This chapter only sets the flag. `set_task` scores servers by token overlap and reveals up to 8 names from the winning server. That starter set is the runtime version of an include list. I did not find it in Hermes, so the box does not cite a function for it.
>
> **Deferred.** Putting `list_changed` on the wire waits for chapter 42. Untrusted-output wrapping (F4, `agent/tool_dispatch_helpers.py`) waits for chapter 34. The load-test capstone that reports these three arms under real traffic is chapter 51.

## What this chapter decides

A new session's revealed list is empty, and `call` returns `NotOffered` until `search_tools` or `set_task` has revealed that name. The catalog can hold the tool while the session still refuses the call.

`search_tools` runs each query through chapter 7's search with a limit of 5, keeps the first hit for each name, reveals those names in order, and adds the listing's token estimate to `schema_tokens`. It does not add the full schema.

`describe` adds the full text's token estimate, also to `schema_tokens`.

`set_task` picks the server with the highest overlap. A token that is a prefix of the server name, or the other way around, scores an extra 2, which is why `rename a file` prefers `files` over `calendar` on the synthetic catalog. Up to `STARTER_LIMIT` (8) names are revealed, and names that contain a query token sort first.

`page_text` slices on characters, `PAGE_CHARS` 2,000. A page that is not the last one ends with `[page N continues]`.

`offered_unused` is the revealed names that have not been `call`ed.

## The fake task

The headline is the demo, not the bench: 135 schema tokens against 45,104 for every tool, on 2,000 tools, with no model in the process. `run_rename_task` loads a synthetic catalog. Three arms:

- **Every.** Reveal every tool. Schema tokens are the full `name / server / description` blob of the whole catalog. Then call the `files` `rename_file` tool.
- **Static.** Reveal that rename tool plus up to seven other `files` tools. Schema tokens are the full blob of those cards.
- **Dynamic.** `search_tools` for `rename a file`, `describe` the first hit, call it. Success means the called name contains `rename` and `file`.

On 2,000 tools the keyless binary printed:

```text
every: success=true schema_tokens=45104 call_tokens=4 result_tokens=14
static: success=true schema_tokens=153 call_tokens=4 result_tokens=14
dynamic: success=true schema_tokens=135 call_tokens=4 result_tokens=15
```

All three succeed. Dynamic sends fewer schema tokens than the static set, and the static set sends fewer than every tool, which is why I'd ship the dynamic arm and keep the static set for a task whose tools are known in advance. Call tokens are the JSON arguments. Result tokens are the description `call` returns. These are character estimates, not a tokenizer and not a bill. `schema_tokens` is one counter: the dynamic arm adds the short listing and the describe text to it, so there is no separate describe field.

> **Rust sidebar: drop the mutex guard before the tool runs**
>
> `call` locks the session map, checks that the name is revealed, and drops the guard before `catalog.call`. The tool body must not run while the map is locked. A body that tried to `search_tools` on the same gateway would deadlock if the guard were still held. The lock is for the session record. It is not for the tool.

## Tests

`search_reveals_names_and_describe_is_capped` checks the short listing, the `list_changed` flag, the full describe text, and both caps. `a_call_must_be_offered_and_paging_splits_long_text` refuses `echo` until search reveals it, then checks a 2,010-character page. `set_task_prefers_the_files_server` uses 64 synthetic tools and expects every picked name to start with `files_`. `three_arms_succeed_and_dynamic_sends_fewer_schema_tokens` uses 256 tools and checks the token order.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch08-session-toolsets
```

```text
Starting 4 tests across 3 binaries
     Summary [   0.015s] 4 tests run: 4 passed, 0 skipped
```

```bash
cargo run -p ch08-session-toolsets
```

The output is the three lines in the fake-task section. No API key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `dynamic_rename_task` | 5.687 ms | the dynamic arm | search, describe, and one call on 2,000 tools |
| `every_tool_rename_task` | 8.355 ms | the every-tool arm | reveal 2,000 schemas, then one call |

Both benches call `run_rename_task` inside the sample, including catalog build. The 2,000-tool `synthetic` vec is built inside the timed closure. There is no model and no sleep. The dynamic arm measured 5.687 ms and the every-tool arm measured 8.355 ms. The demo, not the bench, records 135 schema tokens against 45,104 for every tool, on the same 2,000-tool catalog. The bench is the time. The demo is the token count.

The means are `mean.point_estimate` from [run 38863938100](https://github.com/agnathan/rust-agent-harness/actions/runs/38863938100), section `8`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`set_task` can reveal tools the user did not search for. That is the starter set. A session that only wanted one tool should search, not set a broad task.

Paging is on characters. A 2,000-character page is a different cut from 500 tokens when the text is multibyte, because `approx_tokens` divides bytes by 4 while the page function walks `char`s.

The first search hit for a tied query can be a `calendar_` tool in chapter 6's tie-break. `set_task` adds the server-name bonus so the starter set stays on `files`. `search_tools` does not add that bonus. It uses chapter 7's BM25 order.

## Guardrails in this chapter

Layer L2's "do not show everything," applied to the tool list. The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Caps on one request.** 7 queries, 10 describe names, 5 hits per query, 8 starter tools. The 7 and the 10 are Hermes. The 5 is `search_default_limit` in `ToolSearchConfig` (the default in `from_raw` is 5). The 8 is this chapter's starter cap, the same bound the rest of the book uses for in-flight work.

**A session cannot call a tool it was not shown.** `NotOffered` is the filter. It is not an approval. Approvals are chapter 11.

**Paging.** A long result is split at 2,000 characters. Wrapping untrusted tool output (F4) is chapter 34. This page marker is not that wrapper.

Secret redaction (E1, E2) is still not in this process. Rate limits and the per-turn web-search cap of 50 are chapter 15. This chapter caps one search call's query list. It does not count web searches across a turn.

## Chapter summary

- **Session.** Empty until `search_tools` or `set_task`. `call` checks the revealed list.
- **Listing versus schema.** Search returns a short line. `describe` returns the full text. Both add to `schema_tokens`.
- **Three arms, 2,000 tools.** Every tool 45,104 schema tokens, static 153, dynamic 135. All three succeed. The dynamic arm measured **5.687 ms**. The every-tool arm measured **8.355 ms**.
- **Flag.** `take_list_changed` is true once after the revealed set grows.

## What's next

Chapter 9's agent loop calls the client-side `search_tools` from chapter 6. Wiring that loop to this session view waits until both exist. Chapter 10 is the transition table around the loop's exits.
