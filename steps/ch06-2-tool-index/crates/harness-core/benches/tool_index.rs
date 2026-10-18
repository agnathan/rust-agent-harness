use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use harness_core::{generate, Bm25Index};

fn tool_index(c: &mut Criterion) {
    let cards = generate(10_000);
    c.bench_function("index_build_10000", |b| {
        b.iter_batched(
            || cards.clone(),
            |owned| black_box(Bm25Index::build(owned)),
            BatchSize::LargeInput,
        );
    });
    let index = Bm25Index::build(cards);
    c.bench_function("index_search_10000", |b| {
        b.iter(|| {
            let hits = index.search(black_box("rename a file"), 8);
            black_box(hits);
        });
    });
}

criterion_group!(benches, tool_index);
criterion_main!(benches);
