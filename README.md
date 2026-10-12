# rust-agent-harness

[![CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml/badge.svg)](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml)

An AI agent harness in Rust, with a book on how to build it step by step.

This is a hands-on course for people **new to Rust**. You learn the language by building, one piece at a time, a high-performance harness for teams of AI agents. The design is informed by Nous Research's [Hermes Agent](https://github.com/NousResearch/hermes-agent) (Python). Each lesson points at the Hermes files that solve the same problem, so you can compare the two approaches.

**You write the code.** Every lesson gives you a skeleton with `todo!()` stubs, a test file that fails until your implementation is correct, and hints, a milestone, and exercises. Reference solutions are not part of this repository. The tests are the specification.

## How to start

1. **Install Rust** with [rustup](https://rustup.rs). You need 1.85 or newer (`rustc --version`).
2. **Add the usual components:** `rustup component add rustfmt clippy`.
3. **Use an editor with rust-analyzer** (VS Code, RustRover, or Zed). Turn on format-on-save.
4. **Read a lesson, then make its tests pass.**
   ```bash
   cd starter
   cargo test --test l01_hello
   ```
   Open `lessons/01-cargo-and-first-functions.md`, implement the `todo!()` stubs in `starter/harness/src/l01_hello.rs`, and re-run that command until it is green.
5. Repeat for the next lesson: `cargo test --test l02_message`, and so on.

The first build downloads tokio, reqwest, rusqlite, and the rest. That takes a few minutes.

Useful commands, from `starter/`:

- `cargo check` type-checks without producing a binary.
- `cargo clippy` prints lints. Warnings are allowed; they do not fail CI.
- `cargo test -- --nocapture` shows `println!` output.
- `cargo doc --open` opens the docs for every crate you depend on.

When a test fails, read the whole compiler error, then `rustc --explain E0502` (use the code from the error), then the lesson hints.

## Lessons

| # | Lesson | You build |
|---|---|---|
| 01 | Cargo, functions, strings and tests | banner, word count, slash-command detection |
| 02 | Structs and enums | `Role`, `Message`, `ToolCall`, `Conversation` |
| 03 | Ownership, borrowing and lifetimes | history trimming, transcripts |
| 04 | Errors and `Result` | slash-command parser, budget parser |
| 05 | serde and the JSON wire format | OpenAI messages and tool schemas |
| 06 | Traits and trait objects | the `Tool` trait and `ToolRegistry` |
| 07 | Synchronous agent loop | `run_agent_loop` against a scripted mock model |
| 08 | A real provider with reqwest | OpenAI-compatible client |
| 09 | async and tokio | sequential vs concurrent work, timeouts, tasks |
| 10 | Streaming | SSE parsing and delta accumulation |
| 11 | State machines | the `TurnState` enum and its transition table |
| 12 | Parallel tools | batch planner, semaphore-limited execution |
| 13 | Actors | one tokio task per agent, mpsc inbox, backpressure |
| 14 | Cancellation and supervision | token trees, shared budget, `delegate()` |
| 15 | Message bus and team topologies | router, topics, blackboard, orchestrator–worker |
| 16 | Persistence, tracing and benchmarks | SQLite store, spans, criterion, capstone |

## Continuous integration

[CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml) runs on every push and pull request:

- `cargo fmt --check`, `cargo build`, and `cargo clippy` for `starter/` (clippy warnings do not fail the build)
- a per-lesson test report, which lists pass or fail for each starter lesson and **does not fail the workflow** when unimplemented stubs panic
- an mdBook build of `book/`

### Testing lessons across operating systems

[Lessons](https://github.com/agnathan/rust-agent-harness/actions/workflows/lessons.yml) builds and tests individual lessons on Ubuntu, macOS, and Windows. It installs `cargo-nextest` and runs `cargo nextest run --test lNN_name` for each selected lesson (and falls back to `cargo test --test lNN_name` if nextest is unavailable). Each cell is pass or fail. Unimplemented lessons show as **fail**. JUnit XML is uploaded as a workflow artifact.

**Push and pull request.** A change under `starter/harness/src/lNN_*.rs` or `starter/harness/tests/lNN_*.rs` tests only those lessons, on all three operating systems. The job summary is a lesson-by-OS table. This mode is a report: failing starter stubs do not fail the workflow, so `main` stays green while the lessons are still `todo!()`.

**Manual run.** Actions → Lessons → Run workflow.

| Input | Values |
|---|---|
| `lessons` | `all` (default), `7`, `1-5`, or `3,8,12` |
| `os` | `all` (default), `ubuntu`, `macos`, or `windows` |

A manual run is an explicit selection. The workflow **fails** if any selected lesson fails, including `all`. Lessons you did not select are not run and cannot fail the workflow.

## Layout

```
starter/     your workspace (stubs and tests)
lessons/     the course text
book/        mdBook source and social drafts
```

The book builds with [mdBook](https://rust-lang.github.io/mdBook/): `mdbook build` in `book/` (output in `book/_build/`, not committed).

Hermes source paths in the lessons refer to commit `71d34bad` of NousResearch/hermes-agent.

## License

[MIT](LICENSE) © 2026 Daniel Holmlund
