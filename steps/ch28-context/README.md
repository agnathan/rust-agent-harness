# Chapter 32: Managing Context

Scan a context file, cap it, wrap untrusted tool output, and share the frozen snapshot by `Arc`. Preflight compression runs only when the transcript is over the cap.

```bash
cargo nextest run --locked -p ch28-context
cargo run -p ch28-context
cargo bench -p ch28-context --bench context --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `shared: clean`. No API key.
