# Chapter 39

```bash
cargo nextest run --locked -p ch39-gateway
cargo run -p ch39-gateway --bin ch39-gateway
cargo bench -p ch39-gateway --bench gateway --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
