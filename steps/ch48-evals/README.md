# Chapter 48: Evals

The chapter 9 task names, one red-team case per layer, and a trajectory line. No fail threshold.

```bash
cargo nextest run --locked -p ch48-evals
cargo run -p ch48-evals
cargo bench -p ch48-evals --bench evals --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
