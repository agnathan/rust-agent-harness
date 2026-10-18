# Chapter 49: Load and Token Capstone

In-memory load. Tokens for every tool, a static set, and a dynamic set. No socket.

```bash
cargo nextest run --locked -p ch49-capstone
cargo run -p ch49-capstone
cargo bench -p ch49-capstone --bench capstone --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
