# ch53-the-loop-closes

```bash
cargo nextest run --locked -p ch53-the-loop-closes
cargo run -p ch53-the-loop-closes --bin ch53-the-loop-closes
cargo bench -p ch53-the-loop-closes --bench improve --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The chapter binary is the fake-model before/after. It does not read an API key.

The copied CLI can still call OpenRouter for the older ask path. That path does not run this chapter's loop:

```bash
cargo run -p ch53-harness-cli --bin ch53-harness -- --live --api-key "$OPENROUTER_API_KEY" ask "say hello using the echo tool"
```
