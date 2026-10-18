# Chapter 40: Editor Integration

ACP as another client of the session inbox. Protocol bytes on stdout, logs on stderr.

```bash
cargo nextest run --locked -p ch40-acp
cargo run -p ch40-acp
cargo bench -p ch40-acp --bench acp --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
