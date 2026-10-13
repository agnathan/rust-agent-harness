# Step 06: The agent loop

The model returns text or tool calls. The chapter 3 dispatcher runs native tools and MCP tools. Results are appended, and the model is asked again. `check_gate` runs before every model call. `run_saved_evals` is the small mock set later chapters re-run.

The book chapter is [Chapter 6: The Agent Loop](../../book/chapter-06-agent-loop.md).

From the repository root:

```bash
cargo test -p ch06-harness-core
cargo run -p ch06-harness-core --example demo
cargo run -p ch06-harness-cli -- --mock ask "say hello"
cargo bench -p ch06-harness-core --bench loop_iter
```
