# Step 01: First Contact

The harness as of Chapter 1. This is a finished snapshot: `Message` and `Role`, a `Provider` trait, `OpenAiCompatible`, `MockProvider`, and the `ask` command.

The book chapter is [Chapter 1: First Contact](../../book/chapter-01-first-contact.md). Build on this in `my-harness/` rather than editing this step. Later chapters copy the idea forward; they do not modify this directory.

From the repository root:

```bash
cargo test -p ch01-harness-core
cargo run -p ch01-harness-cli -- --mock ask "Say hello"
```

`OPENAI_API_KEY`, `OPENAI_BASE_URL`, and `HARNESS_MODEL` configure a real OpenAI-compatible server. `--mock` stays on the offline provider. The executable is named `ch01-harness` so it does not collide with later steps in the shared workspace. The clap command name is still `harness`.
