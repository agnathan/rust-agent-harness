# ch50-tracing

```bash
cargo nextest run --locked -p ch50-tracing
cargo run -p ch50-tracing --bin ch50-tracing
cargo bench -p ch50-tracing --bench spans --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
