use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use supervision::Supervisor;

fn restart_bookkeeping(c: &mut Criterion) {
    c.bench_function("restart_bookkeeping", |b| {
        b.iter_batched(
            Supervisor::new,
            |supervisor| {
                let tripped = supervisor.record(black_box(0));
                black_box(tripped);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, restart_bookkeeping);
criterion_main!(benches);
