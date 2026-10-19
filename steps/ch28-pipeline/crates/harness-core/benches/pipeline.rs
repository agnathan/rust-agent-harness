use std::hint::black_box;
use std::sync::{Arc, Mutex};

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::pipeline::{hand_off, Context, Message};

fn sample() -> Arc<Context> {
    Arc::new(Context {
        messages: (0..100)
            .map(|i| Message {
                role: "user".into(),
                text: format!("message {i} about the rename procedure"),
            })
            .collect(),
        notes: Mutex::new(Vec::new()),
    })
}

fn arc_pass(c: &mut Criterion) {
    let context = sample();
    c.bench_function("arc_pass", |b| {
        b.iter(|| {
            let next = hand_off(black_box(Arc::clone(&context)));
            black_box(Arc::ptr_eq(&context, &next));
        });
    });
}

fn copy_transcript(c: &mut Criterion) {
    let context = sample();
    c.bench_function("copy_transcript", |b| {
        b.iter(|| {
            let copied = black_box(&context.messages).clone();
            black_box(copied);
        });
    });
}

criterion_group!(benches, arc_pass, copy_transcript);
criterion_main!(benches);
