# Step 04.4: Cached tools/list

`ToolListCache` keeps the last `tools/list`. `notifications/tools/list_changed` drops it. `ask --mock` runs the fixture. Without `--mock` the CLI does not call OpenRouter.

The book chapter is [Chapter 4: MCP Client](../../book/chapter-04-mcp-client.md).

From the repository root:

```bash
cargo test -p ch04-4-harness-core
cargo run -p ch04-4-harness-core --example demo
cargo run -p ch04-4-harness-cli -- --mock ask "hello"
cargo bench -p ch04-4-harness-core --bench stdio_call
```
