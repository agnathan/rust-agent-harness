use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use harness_core::suite::append_event;

fn append_one(c: &mut Criterion) {
    c.bench_function("append_event", |b| {
        b.iter_batched(
            Vec::new,
            |mut log| {
                append_event(black_box(&mut log), black_box("echo-hello"), true);
                black_box(log);
            },
            BatchSize::SmallInput,
        );
    });
}

fn saved_set(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    c.bench_function("saved_set", |b| {
        b.iter(|| {
            let scores = runtime.block_on(harness_core::run_saved_evals());
            black_box(scores);
        });
    });
}

criterion_group!(benches, append_one, saved_set);
criterion_main!(benches);
