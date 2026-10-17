# Chapter 38: MCP Gateway Service

Session routing, an `Arc` reload, quotas, a drain, and the MCP client checks. No socket.

```bash
cargo nextest run --locked -p gw-service
cargo run -p gw-service
cargo bench -p gw-service --bench service --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
