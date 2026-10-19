# Chapter 39: The Gateway

The chat gateway. Allowlist, pairing, bot-loop guard, emergency stop, and the session inbox.

```bash
cargo nextest run --locked -p ch39-gateway
cargo run -p ch39-gateway
cargo bench -p ch39-gateway --bench gateway --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
