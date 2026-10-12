use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{ChatRequest, ChatResponse, Message};

fn build_request_and_parse_response(c: &mut Criterion) {
    let messages: Vec<Message> = (0..50)
        .map(|index| Message::user(format!("question {index}")))
        .collect();
    let response = br#"{"id":"chatcmpl-bench","choices":[{"index":0,"message":{"role":"assistant","content":"ok"},"finish_reason":"stop"}]}"#;
    c.bench_function("build_request_and_parse_response", |b| {
        b.iter(|| {
            let request = ChatRequest {
                model: "gpt-4o-mini",
                messages: black_box(&messages),
            };
            let bytes = serde_json::to_vec(&request).expect("serialize");
            let parsed: ChatResponse = serde_json::from_slice(black_box(response)).expect("parse");
            black_box((bytes, parsed.choices.len()));
        });
    });
}

criterion_group!(benches, build_request_and_parse_response);
criterion_main!(benches);
