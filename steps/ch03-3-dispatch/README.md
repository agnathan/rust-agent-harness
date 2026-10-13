# Step 03.3: Dispatch

Read-only tool calls run together, up to a bound of 8. A write runs alone. The CLI can list the gated tools and call one.

The book chapter is [Chapter 3: Tools](../../book/chapter-03-tools.md).

From the repository root:

```bash
cargo test -p ch03-3-harness-core
cargo run -p ch03-3-harness-core --example demo
HARNESS_ECHO=1 cargo run -p ch03-3-harness-cli -- tools
cargo bench -p ch03-3-harness-core --bench dispatch
```

The executable is named `ch03-3-harness`.
