use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{Message, OwnedChatRequest};

fn fifty_messages() -> OwnedChatRequest {
    let messages = (0..50)
        .map(|index| {
            if index % 2 == 0 {
                Message::user(format!("question {index}"))
            } else {
                Message::assistant(format!("answer {index}"))
            }
        })
        .collect();
    OwnedChatRequest {
        model: "gpt-4o-mini".to_string(),
        messages,
    }
}

fn serde_50_message_request(c: &mut Criterion) {
    let request = fifty_messages();
    c.bench_function("serde_50_message_request", |b| {
        b.iter(|| {
            let bytes = serde_json::to_vec(black_box(&request)).expect("serialize");
            let parsed: OwnedChatRequest =
                serde_json::from_slice(black_box(&bytes)).expect("deserialize");
            black_box(parsed);
        });
    });
}

criterion_group!(benches, serde_50_message_request);
criterion_main!(benches);
