# Chapter 38: Profiles

Several personas in one config. Each profile keeps its own secrets. The prompt is the persona.

```bash
cargo nextest run --locked -p ch38-profiles
cargo run -p ch38-profiles
cargo bench -p ch38-profiles --bench profiles --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```
