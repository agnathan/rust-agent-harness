# Chapter 29: The Supervisor

Eight specialists at once, one shared context. A child cannot call `delegate_task`, `clarify`, `memory`, `send_message`, or `cronjob_manage`. Depth is 1. The worktree marker is off unless opted in, and no git worktree is created.

```bash
cargo nextest run --locked -p ch29-supervisor
cargo run -p ch29-supervisor
cargo bench -p ch29-supervisor --bench supervisor --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `three steps | the draft`. No API key.
