# Chapter 44: Shared Team Skills

Workers enqueue a proposal. The librarian is the only writer. Readers keep an `Arc`.

```bash
cargo nextest run --locked -p ch44-team-skills
cargo run -p ch44-team-skills
cargo bench -p ch44-team-skills --bench skills --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
