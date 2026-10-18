# Step 05.5: Lazy tool list

A cached stand-in for MCP `tools/list`, invalidated by a stand-in for `notifications/tools/list_changed`, and a per-agent view of that cache. The CLI demo is `--mock` by default. A live OpenRouter call is optional and uses the same client rules as chapter 1.

The book chapter is [Chapter 6: Thousands of Tools](../../book/chapter-06-thousands-of-tools.md).

From the repository root:

```bash
cargo test -p ch06-5-harness-core -p ch06-5-harness-cli
cargo run -p ch06-5-harness-core --example demo
cargo run -p ch06-5-harness-cli -- --mock ask "rename a file"
cargo bench -p ch06-5-harness-core --bench lazy_list
```
