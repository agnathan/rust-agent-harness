# Lesson 16: SQLite persistence, tracing and benchmarks

**Goal:** Make sessions durable, make agent behaviour observable, and measure performance.

## Concepts
- **rusqlite** with the `bundled` feature compiles SQLite into your binary.
  - `execute` / `execute_batch` run statements.
  - `prepare` + `query_map` read rows.
  - Always use `params![..]`, never string formatting, to avoid SQL injection.
- **WAL mode** (`journal_mode=WAL`) lets readers and a writer work at the same time.
- **Append-only messages** give a reliable log that you can replay.
- **tracing** records structured logs in *spans*. `#[tracing::instrument]` wraps a function in a span; `tracing_subscriber::fmt().init()` prints them.
- **Benchmarks.** Use `criterion` for micro-benchmarks and a plain `Instant` for end-to-end throughput.

## How Hermes does it
- `hermes_state.py` `SessionDB` uses SQLite in WAL mode, an FTS5 full-text index (trigram tokenizer plus `native/fts5_cjk`), append-only messages and a read pool (`hermes_state_*.py`).
- `session_search` (`tools/session_search_tool.py`) gives cross-session recall.
- Observability is logging plus plugins (`plugins/observability`).

## Tasks
1. `migrate` (`execute_batch` with `CREATE TABLE IF NOT EXISTS`), `create_session`, and `append` (return `last_insert_rowid()`).
2. `load` with `query_map`, converting roles with `role_from_str`.
3. `search` with `LIKE '%' || ?1 || '%'`.
4. Tracing: in an example binary, call `tracing_subscriber::fmt().with_env_filter("info").init()`. Add `#[tracing::instrument]` to your loop and tool calls, and run with `RUST_LOG=debug`.
5. Benchmarks:
   - `cargo add --dev criterion`.
   - Create `harness/benches/core.rs` with `[[bench]] name="core" harness=false` in `Cargo.toml`.
   - Benchmark `plan_batches` on 100 calls, and `run_agent_loop` with a `ScriptedModel` doing 50 tool rounds.
   - Then write an example that spawns 1,000 lesson-13 actors and measures asks per second.

## Milestone
`cargo test --test l16_persistence` is green; you have one trace printout and one benchmark number.

## Exercises
- Replace `LIKE` with an FTS5 virtual table (`CREATE VIRTUAL TABLE messages_fts USING fts5(content)`) plus triggers.
- Store `tool_calls` as a JSON column, and make save → load → save round-trip exactly.
- Export traces to Jaeger with `tracing-opentelemetry` and `opentelemetry-otlp`.
- **Capstone:** wire lessons 10–16 into one binary. A team of three agents (an orchestrator plus two workers) on a real model, with streaming, parallel tools, cancellation on Ctrl-C (`tokio::signal::ctrl_c` → `cancel_all`), persistence and tracing.
