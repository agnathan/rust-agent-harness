# Chapter 31: Fan-out and Gather

Schedule up to 100 tasks. The 101st is refused. Gather runs them under the supervisor bound of 8 and joins summaries in `task_index` order. Child tool traces stay off the summary.

```bash
cargo nextest run --locked -p ch31-fan-out
cargo run -p ch31-fan-out
cargo bench -p ch31-fan-out --bench fan_out --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints three lines, `a`, `b`, and `c`. No API key.
