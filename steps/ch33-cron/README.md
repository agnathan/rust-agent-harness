# Chapter 39: Cron

Claim a job once, scan the prompt, and keep the watchdog off the due scan.

```bash
cargo nextest run --locked -p ch33-cron
cargo run -p ch33-cron
cargo bench -p ch33-cron --bench cron --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
