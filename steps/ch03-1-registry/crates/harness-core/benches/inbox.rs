use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use harness_core::{ChatSession, MockProvider, SessionInbox};

fn inbox_submit(c: &mut Criterion) {
    c.bench_function("inbox_submit", |b| {
        b.iter_batched(
            || SessionInbox::new(1),
            |inbox| {
                inbox.submit(black_box("hello")).expect("submit");
            },
            BatchSize::SmallInput,
        );
    });
}

fn chat_turn_overhead(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_time()
        .build()
        .expect("tokio runtime");
    c.bench_function("chat_turn_overhead", |b| {
        b.to_async(&runtime).iter_batched(
            || ChatSession::new(MockProvider::new("ok"), "Be brief.", 8_000),
            |mut session| async move {
                let outcome = session.handle_line(black_box("hello")).await;
                black_box(outcome);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, inbox_submit, chat_turn_overhead);
criterion_main!(benches);
