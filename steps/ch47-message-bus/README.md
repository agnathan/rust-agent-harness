# ch47-message-bus

```bash
cargo nextest run --locked -p ch47-message-bus
cargo run -p ch47-message-bus --bin ch47-message-bus
cargo bench -p ch47-message-bus --bench bus --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
