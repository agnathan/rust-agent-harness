# Chapter 33: Agents as Actors

One task per agent. The mailbox holds 64 messages. The 65th is refused. A steer is appended to the next turn. An interrupt skips the model.

```bash
cargo nextest run --locked -p ch29-actors
cargo run -p ch29-actors
cargo bench -p ch29-actors --bench actors --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `agent: ok`. No API key.
