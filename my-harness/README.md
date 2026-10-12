# my-harness

This is the crate to implement, one chapter at a time. `steps/` holds a finished snapshot of each chapter. The tests here are the milestones, and they panic with `not yet implemented: chapter NN` until the matching functions are filled in.

```bash
cargo test -p my-harness --test ch01
```

| Test file | Chapter | Replace |
|---|---|---|
| `tests/ch01.rs` | First Contact | `Message` serialization, `ChatResponse` parsing, `MockProvider::complete` |

A push runs these tests and records the table. Unimplemented chapters fail the tests and do not fail that workflow. A manual run of the My harness workflow fails if a chapter you selected fails.
