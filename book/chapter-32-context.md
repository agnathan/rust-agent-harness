# Chapter 32: Managing Context

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 31 can run a team. It will still paste a poisoned `AGENTS.md` into the prompt, and it will still send every old tool result when the transcript gets long. This chapter decides what is allowed into that prompt. A context file is scanned. A huge file keeps a head and a tail. High-risk tool output is wrapped, and a forged closer is neutralized. The frozen snapshot is an `Arc`. Compression and the memory flush run only when the transcript is over the cap, and they do not rebuild that snapshot.

The step crate is `steps/ch28-context`. The book number is 32. The package is `ch28-context`.

| Section | Step | What it adds |
|---|---|---|
| 32 | `steps/ch28-context` | a scan, a 20,000-character cap, an untrusted wrap, preflight compression |

> **How Hermes does it**
>
> I read `agent/prompt_builder.py` (`_scan_context_content`, `CONTEXT_FILE_MAX_CHARS`, `CONTEXT_TRUNCATE_HEAD_RATIO`, `CONTEXT_TRUNCATE_TAIL_RATIO`, `_truncate_content`), `agent/context_file_sources.py` (`_loaded_status`), `agent/tool_dispatch_helpers.py` (`_UNTRUSTED_TOOL_NAMES`, `_UNTRUSTED_TOOL_PREFIXES`, `_UNTRUSTED_WRAP_MIN_CHARS`, `_neutralize_delimiters`, `_maybe_wrap_untrusted`), `agent/context_compressor.py` (`SKILL_PRUNED_MARKER_PREFIX`, `_skill_pruned_marker`), `agent/turn_preflight.py` (`run_preflight_compression`), `agent/memory_manager.py` (`on_pre_compress`), and `tools/threat_patterns.py` (`MAX_SCAN_CHARS`, the `prompt_injection`, `sys_prompt_override`, `disregard_rules`, and `role_hijack` rows) on Hermes `main` at `e31c674f`.
>
> `CONTEXT_FILE_MAX_CHARS` is 20,000. The head ratio is 0.7 and the tail ratio is 0.2. A project file that matches is replaced with `[BLOCKED: … Content not loaded.]`. A user-authored `SOUL.md` still loads. The scan window is 65,536 characters. Untrusted tools are `web_extract`, `web_search`, and names starting with `browser_` or `mcp_`. Output under 32 characters is not wrapped. An embedded `untrusted_tool_result` token becomes `untrusted-tool-result`. The skill marker is `[SKILL_PRUNED: content lost in compression; reload with skill_view(name='…')]`. Preflight compression runs before the model call. `on_pre_compress` runs before the summary.
>
> **Adopted.** The blocked-file sentence, the user-authored exception, the 20,000-character cap, the head and the tail, the untrusted names, the 32-character floor, the delimiter neutralization, and the skill marker. The snapshot `Arc` is what the hot path sends.
>
> **Adapted.** Four patterns, not the whole library. The preflight threshold is 20,000 characters of the transcript, not a token estimate of the model window. The side model is `FakeModel`. A flush copies the latest user line into a vec. It does not call a memory provider. Under the cap, `preflight` does not call the model.
>
> **Deferred.** The dynamic cap up to 500,000 characters, the rest of `threat_patterns.py`, the upstream-elision notice, and a real auxiliary model. Chapter 9 deferred this preflight. It is taken here.

[personal note: a rules file that looked like documentation until the scan replaced it]

## What this chapter decides

`scan_context` strips a leading BOM, scans at most 65,536 bytes, and collects finding ids. A project file with any finding is replaced by the blocked sentence. `user_authored: true` returns the text and the ids, and `blocked` stays false.

`truncate_content` returns the text when it is at most 20,000 characters. Past that, it keeps 14,000 and 4,000, with a marker between them that names the file.

`wrap_untrusted` wraps a long result from an untrusted tool. The closer token is replaced before the wrap, so a page cannot end the block early. A short result, and any other tool, pass through.

`preflight` clones the snapshot `Arc`. When the transcript is over 20,000 characters it records the latest user line, asks `FakeModel` for one summary, replaces the middle with that summary, and turns each `skill_view` body into the pruned marker. The snapshot string is not rewritten.

`demo` loads `Be direct.` as `AGENTS.md` and returns `shared: clean`. No key.

## Rust: the `Arc` is the hot path

`share` is `Arc::clone`. The string was built before the bench. Cloning the pointer does not copy the 20,000 characters. `preflight` uses the same clone when it also compresses, so a flush does not allocate a second system prompt.

`OnceLock` compiles the four regexes the first time a scan runs. Later scans borrow them.

## Tests

Five tests. A BOM plus `ignore previous instructions` in `AGENTS.md` is the blocked sentence, with finding `prompt_injection`. The same shape in a user-authored `SOUL.md` loads. A 20,001-character file keeps 14,000 plus 4,000 and names `.cursorrules` in the marker. `short` is unchanged. A 40-character `web_extract` result that contains `</Untrusted_Tool_Result>` is wrapped, and the closer is `untrusted-tool-result`. `tiny` and an `echo` of 40 characters are not wrapped. `browser_read` is. Under the cap, the side model is not called and the `Arc` pointer matches. Over the cap, the user line is flushed, the model is called once, the summary replaces the middle, the skill marker names `ship`, and the snapshot text is still `system`. The demo is `shared: clean`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch28-context
```

```text
Starting 5 tests across 3 binaries
     Summary [   0.080s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch28-context
```

```text
shared: clean
```

The binary scans one short file and clones its snapshot. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `share_snapshot` | 5.281 ns | the hot path sends the `Arc` | clone one frozen snapshot |

`share_snapshot` builds a 20,000-character string (`"rule "` repeated 4,000 times) and wraps it in an `Arc` before the bench. The timed closure is `b.iter`. Inside: `share` of that `Arc` passed through `black_box`, then `black_box` of the clone. The string is not rebuilt. The scan, the cap, the wrap, and `preflight` are not called. `FakeModel` is not constructed. No file is read.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `32`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `shared: clean`.

## Pitfalls

The blocked sentence is what the model sees. The original file is not in that string. A later chapter that logs the raw file has to scan before it logs.

`user_authored` is a flag the caller sets. This crate does not look at `HERMES_HOME`. Passing `true` for a repo `AGENTS.md` loads the injection.

The cap counts characters. The scan window counts bytes, and it will not cut a character in half. A file of 20,000 characters that is over 65,536 bytes is still truncated by the cap after the scan.

Preflight over the cap copies the latest user line into `flushed`. That copy can hold a secret. Nothing here redacts it.

The side model is a queue. `complete` on an empty queue returns the word `summary`. It does not call chapter 16's adapter.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Context files (F2).** `AGENTS.md`, `SOUL.md`, and `.cursorrules` go through `scan_context`. A project match is replaced with `[BLOCKED: … Content not loaded.]`. A user-authored file loads, and the finding is kept on the `Scan`.

**Size (F3).** The cap is 20,000 characters. The head is 70 percent and the tail is 20 percent. The marker names the file and the counts.

**Tool output (F4).** `web_extract`, `web_search`, `browser_*`, and `mcp_*` results of at least 32 characters are wrapped. An embedded closer is rewritten to `untrusted-tool-result` before the wrap.

**Preflight.** Compression and the flush run only when the transcript is over the cap, and they run before the model line is used. The hot path clones the snapshot `Arc`. Under the cap the side model is not called.

**Left for later.** The rest of the threat library, the dynamic cap, and redaction (E1, E2). The flush vec is not scrubbed.

## Chapter summary

- **Scan.** Four patterns. A project hit is replaced. A user-authored file loads.
- **Cap.** 20,000 characters, head 14,000, tail 4,000.
- **Wrap.** High-risk tool output, with the closer neutralized.
- **Snapshot.** One `Arc`. Preflight does not rebuild it. The side model runs only over the cap.
- **Time.** Sharing the snapshot measured **5.281 ns**.

[personal note: a file you almost pasted in whole]

## What's next

Chapter 33 gives each agent a task and a mailbox. The mailbox is the session inbox, with a capacity of 64, and a full inbox pushes back.
