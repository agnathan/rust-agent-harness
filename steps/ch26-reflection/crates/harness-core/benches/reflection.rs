use std::hint::black_box;
use std::time::Instant;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use harness_core::reflection::{spawn_reviewer, ReviewRun};

fn spawn_reviewer_bench(c: &mut Criterion) {
    c.bench_function("spawn_reviewer", |b| {
        b.iter_custom(|iters| {
            let mut forks = Vec::with_capacity(iters as usize);
            let started = Instant::now();
            for _ in 0..iters {
                forks.push(spawn_reviewer(black_box(true)));
            }
            let elapsed = started.elapsed();
            drop(forks);
            elapsed
        });
    });
}

fn cancel_review(c: &mut Criterion) {
    c.bench_function("cancel_review", |b| {
        b.iter_batched(
            ReviewRun::spawn_spinner,
            |run| run.cancel_and_join(),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, spawn_reviewer_bench, cancel_review);
criterion_main!(benches);
