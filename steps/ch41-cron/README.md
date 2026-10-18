# Chapter 41: Cron

Claim a job once, scan the prompt, and keep the watchdog off the due scan.

```bash
cargo nextest run --locked -p ch41-cron
cargo run -p ch41-cron
cargo bench -p ch41-cron --bench cron --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
