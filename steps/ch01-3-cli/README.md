# Section 1.3: CLI

Sections 1.1 and 1.2, plus `OpenAiCompatible`: one shared `reqwest` client (HTTP/2, connection pooling) whose default server is OpenRouter. `ch01-harness ask` takes an OpenRouter model id. `--base-url` still points the same client at OpenAI, Ollama, or a local fake.

The executable is named `ch01-harness` so it does not collide with later steps. The clap command name is still `harness`.

```bash
cargo test -p ch01-3-harness-core -p ch01-3-harness-cli
cargo run -p ch01-3-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-3-harness-core --bench request_parse
```

`--mock` needs no key. A live OpenRouter call reads `OPENROUTER_API_KEY`. The integration test starts a local wiremock server.
