# Chapter 28: Handoffs

Filter `scratch` out of the transcript, then move ownership. A turn may hand off four times. That cap is the harness, not a Hermes constant.

```bash
cargo nextest run --locked -p ch24-handoffs
cargo run -p ch24-handoffs
cargo bench -p ch24-handoffs --bench handoffs --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The binary prints `draft sees 2`. No API key.
