use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use critic::as_tool;
use supervisor::{Pool, WORKER_BOUND};

fn judge_one_permit(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    let raw = "verdict: done\nThe note is short enough.";
    c.bench_function("judge_one_permit", |b| {
        b.iter(|| {
            let outcome = as_tool(black_box(&pool), black_box(raw));
            black_box(outcome);
        });
    });
}

criterion_group!(benches, judge_one_permit);
criterion_main!(benches);
