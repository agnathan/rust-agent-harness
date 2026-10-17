# Chapter 25: Curation

Mark an unused skill stale at 14 days and archive it at 30 days. Pinned, cron-referenced, bundled, and hub skills stay. The request path only enqueues the pass. A snapshot is a `tar.gz` kept two deep.

```bash
cargo nextest run --locked -p ch21-curation
cargo run -p ch21-curation
cargo bench -p ch21-curation --bench curation --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `stale: 1`. No API key. The request path is `enqueue`, which the bench times and which does not run the pass.
