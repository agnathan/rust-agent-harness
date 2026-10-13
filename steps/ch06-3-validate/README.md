# ch06-3-validate

Tool-call repair, retries, and timeouts.

The book chapter is [Chapter 6: The Agent Loop](../../book/chapter-06-agent-loop.md).

From the repository root:

```bash
cargo test -p ch06-3-harness-core
cargo run -p ch06-3-harness-core --example demo
cargo run -p ch06-3-harness-cli -- ask "say hello"
cargo bench -p ch06-3-harness-core --bench loop_iter
```
