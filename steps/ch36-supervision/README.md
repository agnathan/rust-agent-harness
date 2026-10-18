# Chapter 36

```bash
cargo nextest run --locked -p ch36-supervision
cargo run -p ch36-supervision --bin ch36-supervision
cargo bench -p ch36-supervision --bench supervision --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
