# rust-agent-harness

[![CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml/badge.svg)](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml)

An AI agent harness in Rust, with a book on how to build it step by step.

This is a hands-on course for people **new to Rust**. You learn the language by growing one harness, not by a tour of the language. The design is informed by Nous Research's [Hermes Agent](https://github.com/NousResearch/hermes-agent) (Python). Each chapter points at the Hermes files that solve the same problem.

## Layout

```
steps/chNN-<slug>/   finished harness at the end of that chapter (cumulative)
my-harness/          your crate; milestone tests start out failing
book/                mdBook source and social drafts
```

A step is the code as of that chapter, library plus CLI, with its tests. Later chapters are new copies under `steps/`, so chapter 1 still builds after chapter 2 exists. They share one cargo workspace at the repo root. Package names carry the chapter (`ch01-harness-core`, `ch02-harness-cli`) and the binaries are `ch01-harness` and `ch02-harness`, so two steps can be built together. The library each test imports is still `harness_core`.

`my-harness/` is where the implementation happens. `tests/ch01.rs` and `tests/ch02.rs` are the milestones. They compile, then panic with `not yet implemented: chapter NN` until you replace the `todo!()` stubs. Finished snapshots stay in `steps/`. There is no solutions directory.

## How to start

1. Install Rust with [rustup](https://rustup.rs). You need 1.85 or newer (`rustc --version`).
2. Add the usual components: `rustup component add rustfmt clippy`.
3. Read [Chapter 1](book/chapter-01-first-contact.md), then make the milestone fail for the right reason and replace the stubs:

   ```bash
   cargo test -p my-harness --test ch01
   ```

4. Run the finished chapter if you want to see the behavior:

   ```bash
   cargo test -p ch01-harness-core
   cargo run -p ch01-harness-cli -- --mock ask "Say hello"
   ```

5. Chapter 2 is the same shape: `cargo test -p my-harness --test ch02`, and `cargo run -p ch02-harness-cli -- --mock chat`.

`OPENAI_API_KEY`, `OPENAI_BASE_URL` (default `https://api.openai.com/v1`), and `HARNESS_MODEL` (default `gpt-4o-mini`) configure a real server. `--mock` uses the offline provider.

| Ch | Title | Step | You add |
|---|---|---|---|
| 0 | Introduction | — | why a harness, and what Hermes is |
| 1 | First Contact | `steps/ch01-first-contact` | one prompt, a provider trait, a mock, `ask` |
| 2 | Conversations | `steps/ch02-conversations` | a real conversation and a `chat` REPL |
| 3 | Tools | `steps/ch03-tools` | a tool trait and a registry |
| 4 | The Agent Loop | `steps/ch04-agent-loop` | model, tools, results, repeat, with a budget |
| 5 | Handoffs | `steps/ch05-handoffs` | agent = instructions + tools + handoff targets; handoff-as-tool; current-agent pointer; max-handoff limit; context filtering |
| 6 | Agents as Tools | `steps/ch06-agents-as-tools` | a manager calls specialists and synthesizes |
| 7 | When Things Fail | `steps/ch07-when-things-fail` | classify failures, retry, fall back |
| 8 | Streaming | `steps/ch08-streaming` | tokens as they arrive |
| 9 | Turn State Machine | `steps/ch09-turn-state` | an explicit turn |
| 10 | Parallel Tools and Fan-out/Gather | `steps/ch10-parallel-tools` | safe batches, fan-out and gather |
| 11 | Managing Context | `steps/ch11-context` | keep the prefix, compress the middle |
| 12 | Remembering | `steps/ch12-remembering` | SQLite sessions and memory files |
| 13 | Agents as Actors | `steps/ch13-actors` | one task per agent, an inbox |
| 14 | Cancellation and Supervision | `steps/ch14-supervision` | cancel trees and restarts |
| 15 | The Blackboard | `steps/ch15-blackboard` | a shared task board |
| 16 | The Message Bus | `steps/ch16-message-bus` | topics, peers, team topologies |
| 17 | MCP and A2A | `steps/ch17-mcp-a2a` | MCP for tools, A2A for remote agents |
| 18 | Tracing, Benchmarks, Capstone | `steps/ch18-tracing` | spans, a benchmark, a team demo |

Chapters 1 and 2 are the only step crates in the tree. The rest are added when those chapters are written. The same list drives CI (`.github/chapters.json`).

The book builds with [mdBook](https://rust-lang.github.io/mdBook/): `mdbook build` in `book/` (output in `book/_build/`, not committed).

## Continuous integration

[CI](https://github.com/agnathan/rust-agent-harness/actions/workflows/ci.yml) runs on every push to `main` and on pull requests:

- `cargo fmt --check` and `cargo clippy --workspace --all-targets` (warnings do not fail the build)
- `cargo test -p …` for every step, each step as its own job, on Ubuntu
- the same tests on Ubuntu, macOS, and Windows for steps whose files changed ([paths-filter](https://github.com/dorny/paths-filter))
- an mdBook build of `book/`

`my-harness` milestone tests are not part of that required build. They run in [My harness](https://github.com/agnathan/rust-agent-harness/actions/workflows/my-harness.yml).

### Milestone tests across operating systems

The workflow installs `cargo-nextest` and runs `cargo nextest run -p my-harness --test chNN` (and falls back to `cargo test` if nextest is unavailable). Unimplemented chapters show as **fail**. JUnit XML is uploaded.

**Push and pull request.** A change under `my-harness/` tests the affected chapters on Ubuntu, macOS, and Windows. This mode is a report: failing stubs do not fail the workflow.

**Manual run.** Actions → My harness → Run workflow.

| Input | Values |
|---|---|
| `chapters` | `all` (default), `1`, `1-18`, or `3,6,12`. Chapters 1–18 are the plan; a chapter runs only after its `tests/chNN.rs` exists (today: 1 and 2) |
| `os` | `all` (default), `ubuntu`, `macos`, or `windows` |

A manual run fails if any selected chapter fails.

## License

[MIT](LICENSE) © 2026 Daniel Holmlund
