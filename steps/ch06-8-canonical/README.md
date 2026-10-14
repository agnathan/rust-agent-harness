# ch06-8-canonical

Canonical tool-call arguments on the send path.

The book chapter is [Chapter 6: The Agent Loop](../../book/chapter-06-agent-loop.md).

From the repository root:

```bash
cargo test -p ch06-8-harness-core
cargo run -p ch06-8-harness-core --example demo
cargo run -p ch06-8-harness-cli -- ask "say hello"
cargo bench -p ch06-8-harness-core --bench loop_iter
```
