# Step 04.1: MCP over stdio

An `rmcp` session that lists and calls tools on the chapter 4 fixture. This copy does not speak HTTP yet.

The book chapter is [Chapter 4: MCP Client](../../book/chapter-04-mcp-client.md).

From the repository root:

```bash
cargo test -p ch04-1-harness-core
cargo run -p ch04-1-harness-core --example demo
cargo bench -p ch04-1-harness-core --bench stdio_call
```
