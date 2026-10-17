# Chapter 34: Supervision and Restart

A failed child runs once more. Three chained restarts skip auto-resume. The bench is the bookkeeping, not the child's work.

```bash
cargo nextest run --locked -p ch30-supervision
cargo run -p ch30-supervision
cargo bench -p ch30-supervision --bench supervision --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `restarted: ok`. No API key.
