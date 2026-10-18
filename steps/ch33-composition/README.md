# Chapter 33: Composing Patterns

A supervisor fans out on one pool. A nested fan-out that would pass the worker limit is refused. A handoff runs only when ownership should move. The critic loops, a wait names a target, and a quality gate can pause the loop.

```bash
cargo nextest run --locked -p ch33-composition
cargo run -p ch33-composition
cargo bench -p ch33-composition --bench composition --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `fan n0 | n1; owner supervisor; done`. No API key.
