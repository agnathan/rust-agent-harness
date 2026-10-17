# Chapter 47: Load and Token Capstone

In-memory load. Tokens for every tool, a static set, and a dynamic set. No socket.

```bash
cargo nextest run --locked -p gw-capstone
cargo run -p gw-capstone
cargo bench -p gw-capstone --bench capstone --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
