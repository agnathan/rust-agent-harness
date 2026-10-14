# Step 03.2: Toolsets

Named toolsets on top of the registry. A set is visible only when its env var is set and its check accepts the value.

The book chapter is [Chapter 3: Tools](../../book/chapter-03-tools.md).

From the repository root:

```bash
cargo test -p ch03-2-harness-core
cargo run -p ch03-2-harness-core --example demo
cargo bench -p ch03-2-harness-core --bench toolset
```
