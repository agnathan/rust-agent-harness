# ch09-6-truncate

Per-tool output cap with head-and-tail truncation.

The book chapter is [Chapter 9: The Agent Loop](../../book/chapter-09-agent-loop.md).

From the repository root:

```bash
cargo test -p ch09-6-harness-core
cargo run -p ch09-6-harness-core --example demo
cargo run -p ch09-6-harness-cli -- ask "say hello"
cargo bench -p ch09-6-harness-core --bench loop_iter
```
