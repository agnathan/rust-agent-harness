# Step 02: Conversations

The harness as of Chapter 2. Everything from Chapter 1, plus `Conversation` (alternation, a stable system prompt, trimming, JSON save/load), a scripted mock, and `harness chat`.

The book chapter is [Chapter 2: Conversations](../../book/chapter-02-conversations.md).

From the repository root:

```bash
cargo test -p ch02-harness-core
cargo run -p ch02-harness-cli -- --mock ask "Say hello"
cargo run -p ch02-harness-cli -- --mock chat
```

Inside `chat`, `/help`, `/reset`, `/history`, `/save`, `/load`, and `/quit` are the commands. The executable is named `ch02-harness` so it can live next to `ch01-harness` in one workspace.
