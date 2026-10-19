# Chapter 43

```bash
cargo nextest run --locked -p ch43-cron
cargo run -p ch43-cron --bin ch43-cron
cargo bench -p ch43-cron --bench cron --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
