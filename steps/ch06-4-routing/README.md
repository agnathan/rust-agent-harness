# Step 05.4: Routing and smaller schemas

Pick a server from a one-line summary, search only that server's index, then trim descriptions and normalize names. Sentences that mention an unavailable tool are dropped.

The book chapter is [Chapter 6: Thousands of Tools](../../book/chapter-06-thousands-of-tools.md).

From the repository root:

```bash
cargo test -p ch06-4-harness-core
cargo run -p ch06-4-harness-core --example demo
cargo bench -p ch06-4-harness-core --bench routing
```
