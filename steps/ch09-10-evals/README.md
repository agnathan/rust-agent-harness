# ch09-10-evals

Stress evals, optionally live.

The book chapter is [Chapter 9: The Agent Loop](../../book/chapter-09-agent-loop.md).

From the repository root:

```bash
cargo test -p ch09-10-harness-core
cargo run -p ch09-10-harness-core --example demo
cargo run -p ch09-10-harness-cli -- ask "say hello"
cargo bench -p ch09-10-harness-core --bench loop_iter
```
