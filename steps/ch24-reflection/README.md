# Chapter 24: The Reflection Loop

Nudge counters fire every 10 user turns and every 10 tool iterations. Turn finalize builds a reviewer fork. The fork's nudge intervals are 0. Memory is on the tool list only when the memory nudge fired. Cancel sets a flag and joins the worker.

```bash
cargo nextest run --locked -p ch24-reflection
cargo run -p ch24-reflection
cargo bench -p ch24-reflection --bench reflection --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `note: prefer short notes`. No API key.
