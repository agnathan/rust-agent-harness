# Step 03.1: Tool registry

An MCP-shaped `Tool` trait (name, description, JSON Schema, content result) and a registry. This copy still includes the chapter 2 conversation code.

The book chapter is [Chapter 3: Tools](../../book/chapter-03-tools.md).

From the repository root:

```bash
cargo test -p ch03-1-harness-core
cargo run -p ch03-1-harness-core --example demo
cargo bench -p ch03-1-harness-core --bench registry
```
