# Chapter 21: Learned Tools

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 8 can reveal a tool after `search_tools` or `set_task`. It does not remember which tools a task used last time. This chapter installs that memory as the offered set before the request. `session_tools` reads the set. It does not score anything. A composite or a macro is a name that expands to steps. Code mode runs a short script of tool calls, and only against tools the session was offered.

The step crate is `steps/gw-learned-tools`. The book number is 21. The package is `gw-learned-tools`. `mcp-gateway` re-exports `LearnedGateway` as `Gateway`. The wrapper still opens an empty catalog. It still binds no socket.

| Section | Step | What it adds |
|---|---|---|
| 21 | `steps/gw-learned-tools` | a predicted offer, composite and macro names, a code-mode script |

> **How Hermes does it**
>
> I read `toolsets.py` (`code_execution`, `create_custom_toolset`, `get_toolset_info`) and `tools/code_execution_tool.py` (`SANDBOX_ALLOWED_TOOLS`, `DEFAULT_MAX_TOOL_CALLS`, `generate_hermes_tools_module`) on Hermes `main` at `8dcf6d5b`.
>
> `code_execution` is the toolset whose description is "Run Python scripts that call tools programmatically." `SANDBOX_ALLOWED_TOOLS` is `web_search`, `web_extract`, `read_file`, `write_file`, `search_files`, `patch`, and `terminal`. `generate_hermes_tools_module` emits a stub only for the intersection of that set and the tools the session has enabled. `DEFAULT_MAX_TOOL_CALLS` is 50. `get_toolset_info` sets `is_composite` when the toolset has `includes`. `debugging` includes `web` and `file`. `create_custom_toolset` registers a runtime toolset from a name, a description, and lists of tools and includes.
>
> I looked for a scorer that ranks tools from memory or past sessions. Hermes does not have one. The offered set changes through tool search, which chapter 8 already adapted.
>
> **Adopted.** The seven allowlist names. The cap of 50. A composite is a name whose steps are other tools. A macro is a named list saved ahead of the request, the same idea as `create_custom_toolset`.
>
> **Adapted.** `refresh_prediction` scores catalog names from a usage log, the memory text, and a session note, then stores an `Arc`. `session_tools` copies that list. It does not call `predict`. The script is lines of `call tool json`, not Python, and there is no `hermes_tools` socket.
>
> **Deferred.** The real interpreter, the UDS and file RPC transports, and stdout caps (`MAX_STDOUT_BYTES` is 50_000) stay out. A live embedder is still later.

[personal note: a tool the score kept offering after you had stopped using it]

## What this chapter decides

`refresh_prediction` is the offline step. Each usage row whose tool is in the catalog and whose task shares a token of three or more characters with the current task adds 1. A mention of the tool name in memory or in the session note adds 3. A name that scores 0 is left out. The rest sort by score, then by name.

`session_tools` clones that `Arc` and copies the names into the view. The first read after a refresh sets `list_changed`. The next read does not. `predict_calls` does not move.

`save_composite` and `save_macro` store a name and a list of steps. `expand` returns that list. Neither one runs the steps.

`run_code_mode` reads `call tool json` lines. A tool outside `SANDBOX_ALLOWED` is `NotAllowed`. A tool that is allowed but not in the offer is `NotOffered`. The 51st call is `TooManyCalls`, and the backend is not called. A line the parser cannot read is `BadLine`.

`ScriptedModel` counts `complete` calls. The demo does not construct one. There is no API key.

The demo's memory text contains `read_file`, and one usage row is that tool on the task "read the note". The offer is `read_file`. The script `call read_file {"path":"note"}` prints `ok`.

## Rust: the offer is an Arc

`offered` is a `Mutex<Arc<Vec<String>>>`. `refresh_prediction` builds a new `Vec`, wraps it, and stores the pointer. `offered_arc` clones the `Arc`, which does not clone the names. `session_tools` then copies the names into the `SessionView`, because the view is a plain struct the caller can keep.

The score stays in `predict`. Nothing in `session_tools` calls it. A test reads `predict_calls` before and after `session_tools` and the number is the same.

## Tests

Six tests. A refresh puts `web_search` ahead of `read_file` when the session note names `web_search` and the usage log has two `read_file` hits, and a second `session_tools` leaves `predict_calls` alone. An empty log and empty memory offer nothing. A composite and a macro expand, and a missing name does not. Code mode rejects `memory`, rejects `web_search` when it was not offered, and runs one `read_file` line. Fifty-one `read_file` lines return `TooManyCalls`. The scripted model starts at zero calls, and the demo offer is `read_file`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p gw-learned-tools
```

```text
Starting 6 tests across 3 binaries
     Summary [   0.007s] 6 tests run: 6 passed, 0 skipped
```

```bash
cargo run -p gw-learned-tools
```

```text
offered: read_file
code: ok
```

The binary opens one session, refreshes from one usage row and a memory sentence, and runs one code-mode line against a fake backend. No key. No socket.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `apply_offered` | 94.69 ns | the offered set only | copy 32 names into a session view |
| `predict_offline` | 174.4 µs | off the request path | score 1,000 rows against 64 names |

`apply_offered` builds the gateway, opens `desk`, and calls `refresh_prediction` before the bench. One `session_tools` also runs first, so `list_changed` is already clear. The timed closure is `b.iter`. Inside: `session_tools` with the session id passed through `black_box`. That clones the `Arc` and copies 32 names. `predict` is not called.

`predict_offline` builds 64 names and 1,000 usage rows before the bench. The timed closure is `b.iter`. Inside: `predict` with the input passed through `black_box`. No gateway, no session, no model.

The table is `mean.point_estimate` from Criterion on this machine, with the Benchmarks flags `--warm-up-time 1 --measurement-time 2 --sample-size 10`. The workflow mean on ubuntu-latest replaces these two numbers when that run is green.

### Milestone demo

The `cargo run` above. The offer is `read_file`. The code-mode body is `ok`.

## Pitfalls

A mention in memory is worth more than two past uses. The session note in the test names `web_search`, so that tool sorts first even though the log only contains `read_file`. If the note is stale, the offer is stale until something calls `refresh_prediction` again.

`session_tools` copies the names. Holding the `Arc` from `offered_arc` is the cheaper read. The view is a snapshot: a later refresh does not change a view you already took.

Code mode is not a sandbox. The allowlist is a filter on names. The backend in the demo is a closure in this process. Chapter 13 is the boundary that runs code outside the harness.

The underscore is part of the tool name. `read_file` matches as one name, not as the two words `read` and `file`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The new limit sits on the tool the script is allowed to call.

**Allowlist.** `SANDBOX_ALLOWED` is the seven names from `SANDBOX_ALLOWED_TOOLS`. `memory` is `NotAllowed` before the backend runs.

**Offer.** A name on the allowlist still has to be in the installed offer. `web_search` is `NotOffered` when the view does not list it.

**Cap.** `CODE_MODE_MAX_CALLS` is 50, from `DEFAULT_MAX_TOOL_CALLS`. The 51st line returns before any call. This is not chapter 15's cap of 50 web searches.

**Prediction.** `session_tools` does not score, and it does not run a tool. The score is a background install.

**Left for later.** This interpreter is not the Docker sandbox (D1, D2). Secret redaction (E1, E2) is still absent. The Python `hermes_tools` RPC is not ported.

## Chapter summary

- **Offer.** Installed by `refresh_prediction`. Read by `session_tools`. The score does not run on that read.
- **Rank.** A memory or session mention is 3. A matching use is 1. Zero stays out.
- **Names.** A composite and a macro expand to steps. They do not run them.
- **Code mode.** `call tool json`, allowlist, offer, and 50 calls.
- **Time.** Applying 32 names measured **94.69 ns**. Scoring 1,000 rows measured **174.4 µs**.

[personal note: the first time the offer was right without a search]

## What's next

Chapter 22 is skills. A directory of `SKILL.md` files becomes a cache-stable index. The full text loads when the model asks for it.
