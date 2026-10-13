# Contributing

Chapters are written by reading Hermes Agent first. The standing rule, the chapter template, and where a deferred idea gets recorded are in [docs/writing-a-chapter.md](docs/writing-a-chapter.md).

Before writing or revising a chapter:

1. Read the current Hermes files for that area on [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent).
2. Implement the worthwhile ideas in the chapter's step crates.
3. In the "How Hermes does it" box, cite the files and say what was adopted, adapted, or deferred, and why.

`cargo fmt --all`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `mdbook build book` should pass. CI stays keyless: the mock is the default, and `--live` is opt-in.
