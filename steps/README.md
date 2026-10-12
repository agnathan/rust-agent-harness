# Steps

Each `chNN-<slug>/` is the harness at the end of that book chapter: the library and the CLI, with that chapter's tests. Chapters are cumulative. A later step is a new copy, not a patch on the previous directory, so every chapter still builds on its own.

They are members of the root cargo workspace. Package names are prefixed (`ch01-harness-core`, `ch02-harness-cli`, …) because cargo needs unique names, and the binaries are `ch01-harness` and `ch02-harness` so they do not overwrite one file in `target/`. The library crate name stays `harness_core`, which is what the tests import. Inside the CLI the clap app is still called `harness`.

```bash
cargo test -p ch01-harness-core
cargo test -p ch02-harness-core
cargo run -p ch02-harness-cli -- --mock chat
```

| Step | Chapter | Adds |
|---|---|---|
| ch01-first-contact | 1 | Message/Role, Provider, OpenAI-compatible and mock providers, `ask` |
| ch02-conversations | 2 | Conversation, scripted mock, JSON save/load, `chat` REPL |

Your own copy is `my-harness/` at the repo root. Milestone tests there start failing until you implement each chapter. The course plan is 30 chapters in five parts, listed in `book/book-outline.md` and `.github/chapters.json`. MCP is a first-class tool boundary from chapter 3. Part II is how one agent learns; the design is `docs/self-improvement.md`. How the team patterns fit together is `docs/team-patterns.md`. Only the steps in the table above exist so far.
