# Step 04.3: MCP tools in the registry

Remote tools implement the chapter 3 `Tool` trait and share that dispatcher. A missing `readOnlyHint` is a write.

The book chapter is [Chapter 4: MCP Client](../../book/chapter-04-mcp-client.md).

From the repository root:

```bash
cargo test -p ch04-3-harness-core
cargo run -p ch04-3-harness-core --example demo
cargo bench -p ch04-3-harness-core --bench stdio_call
```
