# Chapter 40: MCP Gateway Service

Session routing, an `Arc` reload, quotas, a drain, and the MCP client checks. No socket.

```bash
cargo nextest run --locked -p ch40-service
cargo run -p ch40-service
cargo bench -p ch40-service --bench service --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
