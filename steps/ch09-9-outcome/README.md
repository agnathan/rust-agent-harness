# ch09-9-outcome

Typed TurnOutcome and a budget wrap-up call.

The book chapter is [Chapter 9: The Agent Loop](../../book/chapter-09-agent-loop.md).

From the repository root:

```bash
cargo test -p ch09-9-harness-core
cargo run -p ch09-9-harness-core --example demo
cargo run -p ch09-9-harness-cli -- ask "say hello"
cargo bench -p ch09-9-harness-core --bench loop_iter
```
