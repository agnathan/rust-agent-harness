# Chapter 35

```bash
cargo nextest run --locked -p ch35-actors
cargo run -p ch35-actors --bin ch35-actors
cargo bench -p ch35-actors --bench actors --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
