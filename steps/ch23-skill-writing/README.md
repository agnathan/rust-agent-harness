# Chapter 23: Writing Skills

Create, patch, and delete `SKILL.md`, plus supporting files. A scan runs before the write. The approval gate is off until `set_write_approval(true)`. A reviewer has to view a file before patching it. A write the model did not ask for is `enqueue_background`, which does not create a file.

```bash
cargo nextest run --locked -p ch23-skill-writing
cargo run -p ch23-skill-writing
cargo bench -p ch23-skill-writing --bench writing --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `scan: safe` and `created: rename`. No API key.
