# Step 04.2: Streamable HTTP

The same session over streamable HTTP on `127.0.0.1`. The fixture picks the port and prints it on stderr.

The book chapter is [Chapter 4: MCP Client](../../book/chapter-04-mcp-client.md).

From the repository root:

```bash
cargo test -p ch04-2-harness-core
cargo run -p ch04-2-harness-core --example demo
cargo bench -p ch04-2-harness-core --bench stdio_call
```
