# Chapter 48: Shared Team Skills

Workers enqueue a proposal. The librarian is the only writer. Readers keep an `Arc`.

```bash
cargo nextest run --locked -p ch48-team-skills
cargo run -p ch48-team-skills
cargo bench -p ch48-team-skills --bench skills --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
