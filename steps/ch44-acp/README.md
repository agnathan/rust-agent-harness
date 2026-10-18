# Chapter 44

```bash
cargo nextest run --locked -p ch44-acp
cargo run -p ch44-acp --bin ch44-acp
cargo bench -p ch44-acp --bench acp --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
