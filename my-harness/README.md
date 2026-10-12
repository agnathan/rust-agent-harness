# my-harness

This is the crate to implement, one chapter at a time. `steps/` holds a finished snapshot of each chapter. The tests here are the milestones, and they panic with `not yet implemented: chapter NN` until the matching functions are filled in.

```bash
cargo test -p my-harness --test ch01
cargo test -p my-harness --test ch02
```

| Test file | Chapter | Replace |
|---|---|---|
| `tests/ch01_1.rs` | 1.1 Messages | `Message` and `ChatRequest` serialize to OpenAI JSON |
| `tests/ch01_2.rs` | 1.2 Provider | `MockProvider::complete`, including configurable latency |
| `tests/ch01_3.rs` | 1.3 CLI | `ChatResponse` parsing and `OpenAiCompatible::new` |
| `tests/ch02.rs` | Conversations | `Conversation`, `ChatCommand::parse`, `ChatSession::handle_line` |

A push runs these tests and records the table. Unimplemented chapters fail the tests and do not fail that workflow. A manual run of the My harness workflow fails if a chapter you selected fails.
