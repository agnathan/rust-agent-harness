# Chapter 4 fixture

An offline MCP server. Stdio is the default. `--http` binds `127.0.0.1:0` and prints `MCP_URL=` on stderr. `echo` is read-only, `note` is a write, and `grow` reveals `extra` and sends `notifications/tools/list_changed`.

The book chapter is [Chapter 4: MCP Client](../../book/chapter-04-mcp-client.md).

From the repository root:

```bash
cargo test -p ch04-mcp-fixture
```
