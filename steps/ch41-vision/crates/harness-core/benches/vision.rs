use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::vision::{prepare, sniff, tiny_png};

fn sniff_png(c: &mut Criterion) {
    let bytes = tiny_png();
    c.bench_function("sniff_png", |b| {
        b.iter(|| {
            let mime = sniff(black_box(&bytes));
            let prepared = prepare(black_box(&bytes));
            black_box((mime, prepared));
        });
    });
}

criterion_group!(benches, sniff_png);
criterion_main!(benches);
