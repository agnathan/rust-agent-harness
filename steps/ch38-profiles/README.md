# Chapter 38

```bash
cargo nextest run --locked -p ch38-profiles
cargo run -p ch38-profiles --bin ch38-profiles
cargo bench -p ch38-profiles --bench profiles --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
