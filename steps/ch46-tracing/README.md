# Chapter 46: Seeing Inside

A counting subscriber, one span per event, and 100 mock researchers that keep a note.

```bash
cargo nextest run --locked -p ch46-tracing
cargo run -p ch46-tracing
cargo bench -p ch46-tracing --bench spans --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
