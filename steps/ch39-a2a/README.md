# Chapter 45: A2A for External Agents

A trait for an agent in another process. The pool holds 8. Cancel is a flag.

```bash
cargo nextest run --locked -p ch39-a2a
cargo run -p ch39-a2a
cargo bench -p ch39-a2a --bench a2a --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
