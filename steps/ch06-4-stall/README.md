# ch06-4-stall

Stall detection for identical calls and short cycles.

The book chapter is [Chapter 6: The Agent Loop](../../book/chapter-06-agent-loop.md).

From the repository root:

```bash
cargo test -p ch06-4-harness-core
cargo run -p ch06-4-harness-core --example demo
cargo run -p ch06-4-harness-cli -- ask "say hello"
cargo bench -p ch06-4-harness-core --bench loop_iter
```
