# Chapter 43: The Message Bus

One small envelope, a channel of 64, and a cooldown after 20 bot messages.

```bash
cargo nextest run --locked -p ch43-message-bus
cargo run -p ch43-message-bus
cargo bench -p ch43-message-bus --bench bus --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
