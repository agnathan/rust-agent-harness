# Chapter 52: Evals

The chapter 9 task names, one red-team case per layer, and a trajectory line. No fail threshold.

```bash
cargo nextest run --locked -p ch52-evals
cargo run -p ch52-evals
cargo bench -p ch52-evals --bench evals --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
