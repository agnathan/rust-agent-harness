use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::approval::skip_decision;

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
