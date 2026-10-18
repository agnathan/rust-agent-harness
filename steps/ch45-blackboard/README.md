# Chapter 45: The Blackboard

Shared facts, a condition that wakes one waiter, and a scheduler that drops the lock before the model.

```bash
cargo nextest run --locked -p ch45-blackboard
cargo run -p ch45-blackboard
cargo bench -p ch45-blackboard --bench blackboard --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
