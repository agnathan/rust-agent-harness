# Section 1.3: CLI

Sections 1.1 and 1.2, plus `OpenAiCompatible` (one shared `reqwest` client, HTTP/2 enabled, connection pooling) and `ch01-harness ask`.

The executable is named `ch01-harness` so it does not collide with later steps. The clap command name is still `harness`.

```bash
cargo test -p ch01-3-harness-core -p ch01-3-harness-cli
cargo run -p ch01-3-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-3-harness-core --bench request_parse
```

No API key is required. The integration test starts a local wiremock server.
