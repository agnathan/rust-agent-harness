use std::collections::HashSet;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{generate, prepare_card, schema_json, Router};

fn routing(c: &mut Criterion) {
    let cards = generate(10_000);
    let router = Router::build(cards.clone());
    c.bench_function("routed_search_10000", |b| {
        b.iter(|| {
            let hits = router.search(black_box("rename a file"), 8);
            black_box(hits);
        });
    });
    let names: HashSet<String> = cards.iter().map(|card| card.name.clone()).collect();
    c.bench_function("shrink_schemas_10000", |b| {
        b.iter(|| {
            let prepared: Vec<_> = cards
                .iter()
                .map(|card| prepare_card(card, black_box(&names)))
                .collect();
            black_box(schema_json(&prepared));
        });
    });
}

criterion_group!(benches, routing);
criterion_main!(benches);
