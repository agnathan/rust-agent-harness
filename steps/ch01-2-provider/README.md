# Section 1.2: Provider

Section 1.1, plus a `Provider` trait and a `MockProvider` with configurable latency. Calls are async on tokio. The benchmark measures one mock call and 100 concurrent calls via `JoinSet`.

```bash
cargo test -p ch01-2-harness-core
cargo bench -p ch01-2-harness-core --bench mock_calls
```
