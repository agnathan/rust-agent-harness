# Chapter 35: Balancing and Federation

Pick a backend by least outstanding requests, or by the quieter of two indexes. Upstream servers that offer the same tool share a pool. Discovery connects four at a time. No socket.

```bash
cargo nextest run --locked -p gw-balancing
cargo run -p gw-balancing
cargo bench -p gw-balancing --bench balancing --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `pick: echo via alpha`. No API key.
