# Chapter 17: Streaming

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 16 builds a request and waits for a whole reply. A stream arrives as deltas. This chapter folds those deltas into the same reply string, strips think blocks as the chunks arrive, and lets a read-only tool start before the stream finishes. A write waits until `finish`. Nothing here opens a socket.

The step crate is `steps/ch14-streaming`. The book number is 17. The package is `ch14-streaming`.

| Section | Step | What it adds |
|---|---|---|
| 17 | `steps/ch14-streaming` | think-block scrubbing, folded deltas, read-only overlap |

> **How Hermes does it**
>
> I read `agent/think_scrubber.py` (`StreamingThinkScrubber`, `THINK_TAGS`, `feed`, `flush`, `reset`) on Hermes `main` at `cf71b1aa`.
>
> The tags are `think`, `thinking`, `reasoning`, `thought`, `REASONING_SCRATCHPAD`, and the CJK tags `思考`, `反思`, `推理`, `推敲`. `feed` returns the visible part of one delta. A tag split across deltas is held so the opening `<think>` is not emitted on its own. A closed pair is removed wherever it sits. An opening tag that never closes is entered only at the start of the buffer or after a newline, so the prose `see <think> later` is not stripped. `flush` drops an unterminated block. Leaking a partial thought is worse than a short reply. `reset` clears the state for the next turn.
>
> **Adopted.** Those nine tags. `feed`, `flush`, and `reset`. Holding a partial tag. Stripping a closed pair. Leaving an inline `<think>` that never closes. Dropping an unterminated block on flush. A chunk with no `<` and an empty hold buffer is returned as-is.
>
> **Adapted.** The newline-boundary flag and the orphan-close regex are not ported. Case folding is ASCII, which covers the English tags. The CJK tags do not change under that fold. Overlap is a rule on the delta: a read-only tool is recorded while text is still arriving, and a write is stored until `finish`. Hermes starts safe tools from the stream in the tool executor. This chapter does not call that executor.
>
> **Deferred.** The streaming repetition watch from chapter 14's guard stays a full-string check. Provider SSE parsers stay out, with the socket.

[personal note: a think tag that arrived in two chunks, and what leaked before the hold]

## What this chapter decides

`ThinkScrubber::feed("<think>secret</think>hello")` plus `flush` is `hello`.

`feed("<thi")` returns nothing. `feed("nk>secret</think>ok")` returns `ok`. The secret is not in the output.

`<思考>秘密</思考>ok` is `ok`. `see <think> later` is kept, because the tag is not at the start or after a newline and it never closes. A newline, then `<think>secret` with no close, flushes to an empty string.

`Assembler` pushes `Delta::Text` through the scrubber. `Delta::Tool { read_only: true }` is appended to `started` immediately. A write goes to a deferred list. `finish` flushes the scrubber and then appends the deferred names. The test sees `echo` before `finish`, and `echo` then `write` after.

The demo chunks are `hel`, `<thi`, `nk>no</think>`, `lo`, and a read-only `echo`. The reply is `hello`. Started tools are `echo`.

`fold_text` and `buffer_join` on 100 copies of `hello ` are the same string. The fold is the streamed assembly. The join is the buffered reply. Neither calls a model.

## Rust: the hold is the tail of the buffer

`feed` appends the chunk to a `String`. A complete pair is drained, and the text before it is kept. A partial suffix that is a prefix of a tag stays in the buffer. The next chunk is appended to that suffix, so `<thi` and `nk>` become `<think>`.

The fast path is the common delta. If the scrubber is not inside a block, the hold buffer is empty, and the chunk has no `<`, `feed` returns the chunk. It does not scan the tag list. A chunk that might be a tag pays the scan.

`flush` takes the hold buffer only when the scrubber is not inside a block. Inside a block, the buffer is cleared and the function returns an empty string.

## Tests

Five tests. A closed block is removed, and a tag split across two feeds does not leak. CJK tags are stripped, an inline mention is kept, and an unterminated block at a newline flushes empty. A read starts during the stream and a write waits for `finish`. The demo reply is `hello`. Folding the 100 plain chunks matches the buffer.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch14-streaming
```

```text
Starting 5 tests across 3 binaries
     Summary [   0.007s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch14-streaming
```

```text
reply: hello
started: echo
```

The binary is the split think block plus one read-only tool. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `fold_deltas` | CI_MEAN_17_FOLD | within 1 ms of `buffer_join` | scrub and join 100 plain chunks |
| `buffer_join` | CI_MEAN_17_BUF | the buffered baseline | join the same 100 chunks, no scrubber |

`fold_deltas` builds the 100 strings before the bench. Each is `hello `, six characters, no `<`. The timed closure is `b.iter`. Inside: `fold_text` on that slice passed through `black_box`, which constructs a scrubber, calls `feed` 100 times, and `flush`. The fast path returns each chunk without scanning tags. Building the vec is excluded.

`buffer_join` uses the same vec, built before the bench. Each sample concatenates the 100 strings and does not construct a scrubber.

The budget is the difference. Assembling the streamed reply should add less than 1 ms over the buffered join. Both numbers exclude model time. There is no SSE parser in the sample.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, CI_RUN_17, section `17`.

### Milestone demo

The `cargo run` above. The reply is `hello`. `echo` started.

## Pitfalls

The fast path is wrong for a chunk that hides a tag without a `<`. These tags all use `<`. A model that emits a fullwidth bracket is not caught. Hermes folds some of that with NFKC in the threat scanner, not in this scrubber.

An inline `<think>` that never closes is kept. That is the boundary rule. Do not point a one-line mention of the tag at this function and expect it to disappear.

`flush` drops an open block. If you needed the thought for a log, it is already gone. `last_hidden` from Hermes is not stored here.

A write recorded during the stream does not run. `finish` only moves the name onto `started`. This chapter does not execute tools.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L3.

**K5.** `ThinkScrubber` strips the nine tags in `agent/think_scrubber.py`, including a tag split across deltas. An unterminated block is dropped on flush.

**Overlap.** A read-only tool is recorded while the stream is open. A write waits until `finish`. The tool body is not this chapter.

**Left for later.** The repetition guard on a finished string stays chapter 14. A streaming runaway watch is not ported. Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Fold.** Deltas become one string. A split `<think>` does not leak.
- **Tags.** English and the four CJK tags. An inline mention without a close is kept.
- **Overlap.** `echo` starts. `write` waits for `finish`.
- **Time.** Folding 100 plain chunks measured **CI_MEAN_17_FOLD**. Joining them measured **CI_MEAN_17_BUF**.

[personal note: a stream that showed the thought before the close tag arrived]

## What's next

Chapter 18 puts the session on disk. One turn is a WAL append. Search stays a background job until chapter 20. The chapter 9 eval set runs again on the mock.
