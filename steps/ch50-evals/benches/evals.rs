use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use evals::{append_event, run_saved};

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
    c.bench_function("saved_set", |b| {
        b.iter(|| {
            let scores = run_saved();
            black_box(scores);
        });
    });
}

criterion_group!(benches, append_one, saved_set);
criterion_main!(benches);
