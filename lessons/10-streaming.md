# Lesson 10: Streaming tokens (SSE)

**Goal:** Parse a model's streamed output and render it live while building up the final reply.

## Concepts
- **Server-Sent Events (SSE).** With `"stream": true`, an OpenAI-compatible API sends `data: {json}` lines and ends with `data: [DONE]`.
- **Tool calls arrive in fragments.** The first chunk has the `id` and `name`; later chunks append pieces of the `arguments` string, keyed by `index`.
- **An accumulator** is a struct that folds deltas into the final message.
- **Channels.** `tokio::sync::mpsc` connects a producer (the HTTP stream) to a consumer (the UI).
- **`let ... else`** is a clean way to return early when a pattern doesn't match.

## How Hermes does it
- `agent/chat_completion_helpers.py::interruptible_streaming_api_call` streams the response while watching for interrupts.
- `agent/chat_completion_stream_monitor.py` watches for stalled streams.
- The gateway's "stream is the message" contract (`gateway/AGENTS.md`) requires that each draft frame is a *prefix* of the next one, so chat platforms never show duplicated text.

## Tasks
1. `parse_sse_line`.
   - Hint: `let Some(data) = line.trim().strip_prefix("data:") else { return Ok(None) };`
2. `Accumulator::apply` and `finish`.
   - Hint: `self.calls.get_mut(index)`.
3. `consume`: `while let Some(d) = rx.recv().await`. Stop at `Done`, and call `on_text` for each text delta.

## Milestone
`cargo test --test l10_streaming` is green.

## Exercises
- Wire it up for real: reqwest (async, `stream` feature) → `bytes_stream()` → split into lines → `parse_sse_line` → `mpsc` → print tokens as they arrive.
- Add a stall watchdog: if no delta arrives for 30 s, abort with an error.
