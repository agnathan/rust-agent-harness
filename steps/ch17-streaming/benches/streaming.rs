use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use streaming::{buffer_join, fold_text};

fn chunks() -> Vec<String> {
    (0..100).map(|_| "hello ".to_string()).collect()
}

fn fold_deltas(c: &mut Criterion) {
    let chunks = chunks();
    c.bench_function("fold_deltas", |b| {
        b.iter(|| {
            let text = fold_text(black_box(&chunks));
            black_box(text);
        });
    });
}

fn buffer_join_bench(c: &mut Criterion) {
    let chunks = chunks();
    c.bench_function("buffer_join", |b| {
        b.iter(|| {
            let text = buffer_join(black_box(&chunks));
            black_box(text);
        });
    });
}

criterion_group!(benches, fold_deltas, buffer_join_bench);
criterion_main!(benches);
