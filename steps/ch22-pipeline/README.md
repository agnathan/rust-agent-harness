# Chapter 26: The Sequential Pipeline

Outline, draft, critique, revise. Each stage is a scripted reply. The next stage receives the same `Arc`.

```bash
cargo nextest run --locked -p ch22-pipeline
cargo run -p ch22-pipeline
cargo bench -p ch22-pipeline --bench pipeline --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `revise: the revision`. No API key.
