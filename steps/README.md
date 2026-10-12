# Steps

Each `chNN-<slug>/` is the harness at the end of that book chapter: the library and the CLI, with that chapter's tests. Later chapters are new copies under `steps/`, so this directory keeps building after the book moves on.

Steps are members of the root cargo workspace. Package names are prefixed (`ch01-harness-core`, `ch01-harness-cli`) because cargo needs unique names across chapters, and the binary is `ch01-harness` so it will not collide with a later step. The library crate name stays `harness_core`. Inside the CLI the clap app is still called `harness`.

```bash
cargo test -p ch01-harness-core
cargo run -p ch01-harness-cli -- --mock ask "Say hello"
```

| Step | Chapter | Adds |
|---|---|---|
| ch01-first-contact | 1 | Message/Role, Provider, OpenAI-compatible and mock providers, `ask` |

Your own copy is `my-harness/` at the repo root. Milestone tests there start failing until you implement the chapter.
