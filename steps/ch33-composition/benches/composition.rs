use std::hint::black_box;

use composition::nested_fanout;
use criterion::{criterion_group, criterion_main, Criterion};
use supervisor::{Pool, WORKER_BOUND};

fn nested_eight(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    c.bench_function("nested_eight", |b| {
        b.iter(|| {
            let summary = nested_fanout(black_box(&pool), black_box(0), black_box(WORKER_BOUND))
                .expect("eight fit");
            black_box(summary);
        });
    });
}

criterion_group!(benches, nested_eight);
criterion_main!(benches);
