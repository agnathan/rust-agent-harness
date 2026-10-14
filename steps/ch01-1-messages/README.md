# Section 1.1: Messages

`Message` and `Role`, serialized to the OpenAI chat JSON shape, plus a 50-message serde benchmark.

```bash
cargo test -p ch01-1-harness-core
cargo bench -p ch01-1-harness-core --bench serde_messages
```

The book section is [Chapter 1: First Contact](../../book/chapter-01-first-contact.md).
