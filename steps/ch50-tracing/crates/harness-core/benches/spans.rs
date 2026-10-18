use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::spans::{mock_turn, CountingSubscriber, FakeModel};

fn both(c: &mut Criterion) {
    let model = FakeModel { reply: "ok" };
    let subscriber = CountingSubscriber::new();
    tracing::subscriber::with_default(subscriber, || {
        c.bench_function("event_without_span", |b| {
            b.iter(|| {
                let reply = model.complete(black_box("hello"));
                black_box(reply);
            });
        });
        c.bench_function("span_per_event", |b| {
            b.iter(|| {
                let mut sink = Vec::with_capacity(3);
                let reply = mock_turn(black_box(&model), black_box("hello"), &mut sink);
                black_box((reply, sink));
            });
        });
    });
}

criterion_group!(benches, both);
criterion_main!(benches);
