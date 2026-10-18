# Chapter 37: The Gateway

The chat gateway. Allowlist, pairing, bot-loop guard, emergency stop, and the session inbox.

```bash
cargo nextest run --locked -p ch37-gateway
cargo run -p ch37-gateway
cargo bench -p ch37-gateway --bench gateway --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
