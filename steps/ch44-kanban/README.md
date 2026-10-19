# Chapter 44: The Kanban Board

A JSON board. Claim, complete, review, and block are one compare-and-swap under a lock file.

```bash
cargo nextest run --locked -p ch44-kanban
cargo run -p ch44-kanban
cargo bench -p ch44-kanban --bench kanban --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
