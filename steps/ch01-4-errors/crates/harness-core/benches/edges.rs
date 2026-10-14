use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{events_for_reply, ChatResponse};

fn parse_response_edges(c: &mut Criterion) {
    let with_usage = br#"{"model":"openai/gpt-4o-mini","choices":[{"message":{"role":"assistant","content":"ok"}}],"usage":{"prompt_tokens":50,"completion_tokens":1,"total_tokens":51,"cost":0.25}}"#;
    let answered_elsewhere = br#"{"model":"anthropic/claude-3.5-sonnet","choices":[{"message":{"role":"assistant","content":"ok"}}]}"#;
    c.bench_function("parse_response_edges", |b| {
        b.iter(|| {
            let full: ChatResponse = serde_json::from_slice(black_box(with_usage)).expect("usage");
            let bare: ChatResponse =
                serde_json::from_slice(black_box(answered_elsewhere)).expect("model");
            let full_message = &full.choices[0].message;
            let bare_message = &bare.choices[0].message;
            black_box((
                events_for_reply(full_message, full.usage.as_ref(), full.model.as_deref()),
                events_for_reply(bare_message, bare.usage.as_ref(), bare.model.as_deref()),
            ));
        });
    });
}

criterion_group!(benches, parse_response_edges);
criterion_main!(benches);
