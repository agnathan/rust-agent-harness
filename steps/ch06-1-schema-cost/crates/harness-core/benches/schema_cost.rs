use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{generate, schema_cost};

fn schema_tokens(c: &mut Criterion) {
    let thousand = generate(1_000);
    let ten_thousand = generate(10_000);
    c.bench_function("schema_tokens_1000", |b| {
        b.iter(|| {
            let cost = schema_cost(black_box(&thousand));
            black_box(cost.tokens);
        });
    });
    c.bench_function("schema_tokens_10000", |b| {
        b.iter(|| {
            let cost = schema_cost(black_box(&ten_thousand));
            black_box(cost.tokens);
        });
    });
}

criterion_group!(benches, schema_tokens);
criterion_main!(benches);
