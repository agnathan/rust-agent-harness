# Chapter 42: The Kanban Board

A JSON board. Claim, complete, review, and block are one compare-and-swap under a lock file.

```bash
cargo nextest run --locked -p ch36-kanban
cargo run -p ch36-kanban
cargo bench -p ch36-kanban --bench kanban --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
