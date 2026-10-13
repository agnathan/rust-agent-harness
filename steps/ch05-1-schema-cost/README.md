# Step 05.1: Schema cost

A synthetic catalog of MCP-shaped tools, and the token estimate for putting every schema in the prompt.

The book chapter is [Chapter 5: Thousands of Tools](../../book/chapter-05-thousands-of-tools.md).

From the repository root:

```bash
cargo test -p ch05-1-harness-core
cargo run -p ch05-1-harness-core --example demo
cargo bench -p ch05-1-harness-core --bench schema_cost
```
