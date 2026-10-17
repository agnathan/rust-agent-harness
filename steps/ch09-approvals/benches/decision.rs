use std::hint::black_box;

use approvals::skip_decision;
use criterion::{criterion_group, criterion_main, Criterion};

fn skip(c: &mut Criterion) {
    c.bench_function("skip_decision", |b| {
        b.iter(|| {
            let verdict = skip_decision(black_box("echo hello"));
            black_box(verdict);
        });
    });
}

criterion_group!(benches, skip);
criterion_main!(benches);
