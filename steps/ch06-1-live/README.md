# ch06-1-live

Optional --live OpenRouter call. The mock is the default.

The book chapter is [Chapter 6: The Agent Loop](../../book/chapter-06-agent-loop.md).

From the repository root:

```bash
cargo test -p ch06-1-harness-core
cargo run -p ch06-1-harness-core --example demo
cargo run -p ch06-1-harness-cli -- ask "say hello"
cargo bench -p ch06-1-harness-core --bench loop_iter
```
