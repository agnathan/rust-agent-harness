# Step 05.3: search_tools

A core tool set plus `search_tools`. Matched schemas are appended on the next turn. The core prefix stays byte-identical, including across eviction.

The book chapter is [Chapter 5: Thousands of Tools](../../book/chapter-05-thousands-of-tools.md).

From the repository root:

```bash
cargo test -p ch05-3-harness-core
cargo run -p ch05-3-harness-core --example demo
cargo bench -p ch05-3-harness-core --bench search_tool
```
