# Chapter 8: Session Toolsets

On 2,000 tools, a fake "rename a file" succeeds three ways. Showing every schema costs 45,104 estimated tokens, a static handful costs 153, but search plus one describe costs 135. The dynamic arm is the one I would ship, because 135 tokens still name the tool the task needs and 45,104 name the whole catalog. This chapter is the session view that produces that third number.

Chapter 7's gateway has one catalog, so every caller who can search it can see every name. A session should see a smaller set: the tools a task starter picked, plus the names a search revealed, with the full schema staying off that list until something asks for it. The step crate is `steps/ch08-session-toolsets`, and it depends on `ch07-catalog`. It binds no socket.

`SessionGateway` is the type a client holds; `set_task` installs a starter set for a task description. `search_tools` returns a short line per hit, `name — blurb`, and refuses more than 7 queries. `describe` returns the server and the full description, and refuses more than 10 names. Those two caps are the ones Hermes puts on its bridge tools. Revealed names accumulate on the session. `SessionView` is what `session_tools` returns: the names this session may see, not the whole catalog. `take_list_changed` flips a flag back to false, which is the bit a later socket would send as `notifications/tools/list_changed`. Results page at 2,000 characters. `Telemetry` records schema tokens, call tokens, and result tokens. `Arm` and `TaskReport` are the three-way comparison: every tool, a static set, and a dynamic set. `run_rename_task` is the fake task those arms share. Token estimates divide bytes by 4.

The server stays a separate program because the session filter is a catalog decision. The harness already knows how to keep a prompt prefix stable. It should receive a small list, not discover the small list by downloading every schema and throwing most of them away. Putting `set_task` and the revealed-name filter in the gateway means two harness processes can share one catalog and still see different sessions. Hermes refuses a session-keyed catalog on the client, because that copy would drift from the live definitions. This server is the source of the list, so the session stores names and the catalog stores cards. `mcp-gateway`, the process crate, re-exports `gw_session` from this chapter — `SessionView`, `Arm`, `Telemetry`, `run_rename_task` — beside the chapter 51 gateway. The re-export is how the process crate stays thin, and it still binds no socket.

The three arms are the measurement chapter 51 will report again, on a copied catalog, with task success beside the token count. Collecting the numbers here means the later capstone has a definition of "dynamic" that is this session view. A learned offer, chapter 23, changes which names `session_tools` returns, and it uses this same session type. A continuously improving harness that adds tools without an offer policy eventually shows the model everything again. The dynamic arm is the policy this chapter can already measure: search, one describe, one call, and a token count that stays near the static handful instead of the whole catalog.

A session is a struct keyed by an id. A flag you take and clear is the notification. An enum (`Arm`) is the three experiments, so a fourth way of offering tools has to be added on purpose. The token estimate is integer division, the same four-characters-per-token rule chapter 6 used.

I expected `describe` to refuse a name the session had not revealed. The function checks the cap of 10, then looks the name up on the catalog snapshot. A missing card becomes the line `{name}: unknown`. A present card prints the name, the server, and the description. Either way it adds the text's token estimate to `schema_tokens`. The revealed set plays no part in that lookup. The refusal waits for `call`, which returns `NotOffered` when the name is absent from that set. Search and `set_task` are what put a name there. Describe is a way to spend schema tokens on a card the catalog already holds. That split surprised me, and it is the one to remember when chapter 9's loop searches from the client instead of from this session. The two programs are still two programs. This view waits. The loop calls the client.

`set_task` is the other way onto the revealed set, and the scoring is small enough to do by hand. Tokens come from the task string. For each card, a token scores 1 when the card's name or description contains it. It scores an extra 2 when that card's server starts with the token, or the token starts with the server name. Those points add up per server, and the winning server is the max. Up to 8 cards from that server are revealed, names that contain a query token first, then name order. `rename a file` prefers `files` over `calendar` on the synthetic catalog because `file` is a prefix of `files`, which is the extra 2. The chapter already says that. What I want the scoring to make obvious is that the starter set can include tools the user never searched for. The pitfalls say the same thing in one line. A session that wanted one tool should call `search_tools`, which uses chapter 7's BM25 order and adds no server-name bonus.

`search_tools` errors if the query list is longer than 7. Each query searches with a limit of 5. Hits are deduped, names are revealed, and the blurb is the description trimmed to 80 characters. The listing's token estimate is added to `schema_tokens`. The full schema stays off that counter until `describe`. On the dynamic arm of the fake task, both the short listing and the describe text land in the same counter, which is why there is no separate describe field in the demo output. 135 is the sum, not the search alone.

`call` locks the session map, checks the revealed set, and drops the guard before `catalog.call`. The sidebar says why. After the catalog returns, the session records that the name was called and adds call tokens and result tokens. `offered_unused` is revealed minus called. `take_list_changed` is a `mem::take` of the flag, so the second read is false. `page_text` slices on characters at 2,000. A page with more characters left ends with `[page N continues]`. Paging walks `char`s. `approx_tokens` divides bytes by 4. A multibyte page is a different cut from the token estimate, which the pitfalls already flag. I am repeating it because the demo's token numbers are easy to treat as a bill, and they are character estimates.

`cargo run -p ch08-session-toolsets` prints the fake task; no port is opened.

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

- **Every.** Reveal every tool, and schema tokens are the full `name / server / description` blob of the whole catalog. Then call the `files` `rename_file` tool.
- **Static.** Reveal that rename tool plus up to seven other `files` tools. Schema tokens are the full blob of those cards.
- **Dynamic.** `search_tools` for `rename a file`, `describe` the first hit, call it. Success means the called name contains `rename` and `file`.

On 2,000 tools the keyless binary printed:

```text
every: success=true schema_tokens=45104 call_tokens=4 result_tokens=14
static: success=true schema_tokens=153 call_tokens=4 result_tokens=14
dynamic: success=true schema_tokens=135 call_tokens=4 result_tokens=15
```

All three succeed. Dynamic sends fewer schema tokens than the static set, and the static set sends fewer than every tool, which is why I'd ship the dynamic arm and keep the static set for a task whose tools are known in advance. Call tokens are the JSON arguments; result tokens are the description `call` returns. These are character estimates, not a tokenizer and not a bill. `schema_tokens` is one counter: the dynamic arm adds the short listing and the describe text to it, so there is no separate describe field.

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

Read the demo lines as three experiments on one catalog, not as three products. Every tool reveals the whole set, and `schema_tokens` is the full `name / server / description` blob. On 2,000 tools that blob is 45,104 estimated tokens. Static reveals the rename tool plus up to seven other `files` tools, and the blob of those cards is 153. Dynamic searches `rename a file`, describes the first hit, and calls it. Success for that arm is a called name that contains `rename` and `file`. The schema counter for dynamic is 135, and it is one counter: the short listing plus the describe text. There is no separate describe field, so 135 stays a single sum. Call tokens are 4 on every arm, the JSON arguments. Result tokens are 14, 14, and 15, the description `call` returns. These are character estimates, rather than a tokenizer or a bill.

The benches time `run_rename_task` with the catalog build inside the sample. There is no model and no sleep. Dynamic measured 5.687 ms. Every-tool measured 8.355 ms. The demo, not the bench, is where 135 and 45,104 come from. I expected the every-tool arm to be dramatically slower, because 45,104 tokens sounds like a lot of work. The bench is a few milliseconds either way, on an in-process catalog with no model. The cost the demo is pointing at is the token estimate, the thing a provider would bill if those schemas were in the prompt. The time is real and small. The token gap is the reason to ship the dynamic arm and keep the static set for a task whose tools are known ahead of time.

Chapter 51 reports these three arms again, on a copied catalog, with task success beside the token count. The definition of dynamic has to be this session view, or the capstone is measuring a different offer. A learned offer in chapter 23 changes which names `session_tools` returns and uses this same session type. The harness loop in chapter 9 calls chapter 6's client search, and leaves this view for a later wiring. Both can be true. The server decides what a session has been shown. The harness decides what enters the prompt. A continuously improving harness that learns a new procedure still has to be offered the tool that procedure calls, and it still has to avoid pasting the other 1,999 schemas into the prefix. That is the job of the revealed set, the 7-query cap, and the 10-name describe cap. The flag `take_list_changed` is how a later socket would tell the client the set moved. This chapter only sets the flag. Chapter 4's client already drops its cache when it hears the notification. The wire is chapter 42.

## Chapter summary

- **Session.** Empty until `search_tools` or `set_task`. `call` checks the revealed list.
- **Listing versus schema.** Search returns a short line. `describe` returns the full text. Both add to `schema_tokens`.
- **Three arms, 2,000 tools.** Every tool 45,104 schema tokens, static 153, dynamic 135. All three succeed. The dynamic arm measured **5.687 ms**. The every-tool arm measured **8.355 ms**.
- **Flag.** `take_list_changed` is true once after the revealed set grows.

The caps are four different numbers, and they get blurred if you only remember "the search is limited." Seven queries in one `search_tools` call, ten names in one `describe`, five hits per query, eight starter tools from `set_task`. The 7 and the 10 are Hermes. The 5 is `search_default_limit` in `ToolSearchConfig`, and the default in `from_raw` is 5. The 8 is this chapter's starter cap, the same bound the rest of the book uses for in-flight work. A call that passes 8 queries gets `TooManyQueries` and reveals nothing from that request. A describe of 11 names gets `TooManyNames`. The session's revealed set is unchanged by the refusal. I expected the extra names to be truncated to the cap. The function returns the error instead, which is the stricter reading, and it matches the Hermes caps the box adopted rather than a silent trim.

Paging is the other cut people confuse with the token estimate. `page_text` walks characters and stops at 2,000. A page that still has text ends with `[page N continues]`. `approx_tokens` divides bytes by 4. On ASCII the two views are close. On multibyte text a 2,000-character page is more than 500 tokens, or fewer, depending on the bytes, and the pitfalls already say the cuts differ. The demo's schema numbers are the token estimate. The page marker is a slice of a tool result. Wrapping that result as untrusted data is chapter 34. This chapter only splits it.

`offered_unused` is the revealed names that have not been called. It is the list you would look at if a task revealed eight tools and used one. The dynamic arm of the fake task calls the tool it described, so the unused set is whatever search revealed besides that name. The every-tool arm reveals 2,000 names and calls one. The unused set is the rest, which is another way to see the 45,104. Most of those schemas were offered and never called. The static arm is the compromise when you already know the server. Dynamic is the compromise when you know the task and not the names.

`list_changed` is a bool on the session, not a message on a wire. Search or `set_task` sets it when the revealed names grow. `take_list_changed` returns the value and clears it, so a second take is false. I expected the flag to stay true until a client acknowledged it. The take is the acknowledgement, and this process has no client connected. Chapter 4's client is the one that drops a cache when it hears the notification. Chapter 42 is where the flag could become a notification on a socket. Until then the bool is the whole protocol, and the tests can set it and take it without binding anything. A second take returning false is the assertion that the flag was consumed, rather than left stuck true. That matters.

## What's next

Chapter 9 returns to the harness. The loop there calls the client-side `search_tools` from chapter 6, the one that keeps a lazy prompt, not the session view this chapter just built. Wiring the two together waits until both exist. The next chapter is the loop: the model asks, the tool answers, and a repeated call can stop the turn. The revealed set stays on this side of the split until a later chapter wires them. Chapter 10 then puts a transition table around that loop's exits.
