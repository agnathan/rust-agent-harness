use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use gwservice::capstone::{compare, load, sample_catalog};

fn dynamic_tokens(c: &mut Criterion) {
    let catalog = sample_catalog();
    c.bench_function("dynamic_tokens", |b| {
        b.iter(|| {
            let arms = compare(black_box(&catalog), black_box("tool_07"), None);
            black_box(arms[2].tokens);
        });
    });
}

fn load_100(c: &mut Criterion) {
    c.bench_function("load_100", |b| {
        b.iter(|| {
            let report = load(black_box(100));
            black_box(report);
        });
    });
}

criterion_group!(benches, dynamic_tokens, load_100);
criterion_main!(benches);
