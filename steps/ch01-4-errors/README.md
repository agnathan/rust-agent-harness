# Section 1.4: Timeouts and response edges

Section 1.3, plus a request timeout and the cases a buffered OpenRouter response can take: no `usage` object, a `model` field that is not the id we asked for, and an empty `choices` array.

```bash
cargo test -p ch01-4-harness-core -p ch01-4-harness-cli
cargo run -p ch01-4-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-4-harness-core --bench edges
```

No API key is required. The integration test starts a local wiremock server.
