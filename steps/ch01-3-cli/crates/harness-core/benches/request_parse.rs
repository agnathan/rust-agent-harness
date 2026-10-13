use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{events_for_reply, ChatRequest, ChatResponse, Message};

fn build_request_and_parse_response(c: &mut Criterion) {
    let messages: Vec<Message> = (0..50)
        .map(|index| Message::user(format!("question {index}")))
        .collect();
    let response = br#"{"id":"gen-bench","model":"openai/gpt-4o-mini","choices":[{"index":0,"message":{"role":"assistant","content":"ok"},"finish_reason":"stop"}],"usage":{"prompt_tokens":50,"completion_tokens":1,"total_tokens":51,"cost":0.25}}"#;
    c.bench_function("build_request_and_parse_response", |b| {
        b.iter(|| {
            let request = ChatRequest {
                model: "openai/gpt-4o-mini",
                messages: black_box(&messages),
            };
            let bytes = serde_json::to_vec(&request).expect("serialize");
            let parsed: ChatResponse = serde_json::from_slice(black_box(response)).expect("parse");
            let message = &parsed.choices[0].message;
            let events = events_for_reply(message, parsed.usage.as_ref(), parsed.model.as_deref());
            black_box((bytes, events));
        });
    });
}

criterion_group!(benches, build_request_and_parse_response);
criterion_main!(benches);
