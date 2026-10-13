# Step 05.2: Tool index

BM25 over tool names and descriptions, built once. Embeddings sit behind a trait and are not used for the ranking bench.

The book chapter is [Chapter 5: Thousands of Tools](../../book/chapter-05-thousands-of-tools.md).

From the repository root:

```bash
cargo test -p ch05-2-harness-core
cargo run -p ch05-2-harness-core --example demo
cargo bench -p ch05-2-harness-core --bench tool_index
```
