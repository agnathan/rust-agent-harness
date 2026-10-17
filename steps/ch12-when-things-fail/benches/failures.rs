use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use failures::{repeated_fragment, repetition_probe, retry_budget};

fn empty_budget(c: &mut Criterion) {
    c.bench_function("empty_budget", |b| {
        b.iter(|| {
            let budget = retry_budget(black_box(Some(0.30)));
            black_box(budget);
        });
    });
}

fn repetition_guard(c: &mut Criterion) {
    let text = repeated_fragment();
    c.bench_function("repetition_guard", |b| {
        b.iter(|| {
            let hit = repetition_probe(black_box(&text));
            black_box(hit);
        });
    });
}

criterion_group!(benches, empty_budget, repetition_guard);
criterion_main!(benches);
