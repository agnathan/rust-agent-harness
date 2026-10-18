# Step 05.1: Schema cost

A synthetic catalog of MCP-shaped tools, and the token estimate for putting every schema in the prompt.

The book chapter is [Chapter 6: Thousands of Tools](../../book/chapter-06-thousands-of-tools.md).

From the repository root:

```bash
cargo test -p ch06-1-harness-core
cargo run -p ch06-1-harness-core --example demo
cargo bench -p ch06-1-harness-core --bench schema_cost
```
