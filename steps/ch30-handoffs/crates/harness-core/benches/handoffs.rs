use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::handoffs::{filter_transcript, Message};

fn filter_100(c: &mut Criterion) {
    let messages: Vec<Message> = (0..100)
        .map(|i| Message {
            role: if i % 5 == 0 {
                "scratch".into()
            } else {
                "user".into()
            },
            text: format!("message {i} about the rename procedure"),
        })
        .collect();
    c.bench_function("filter_100", |b| {
        b.iter(|| {
            let filtered = filter_transcript(black_box(&messages));
            black_box(filtered);
        });
    });
}

criterion_group!(benches, filter_100);
criterion_main!(benches);
