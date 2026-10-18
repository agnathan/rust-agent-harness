# Chapter 30: The Critic

Judge a finished reply. `verdict: done` accepts. A missing or unknown verdict rejects. The critic takes one permit from the supervisor pool and does not write the answer.

```bash
cargo nextest run --locked -p ch30-critic
cargo run -p ch30-critic
cargo bench -p ch30-critic --bench critic --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `accepted: done`. No API key.
