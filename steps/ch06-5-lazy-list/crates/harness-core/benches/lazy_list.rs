use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use harness_core::{AgentView, ListCache};

fn lazy_list(c: &mut Criterion) {
    c.bench_function("cache_cold_10000", |b| {
        b.iter_batched(
            || ListCache::new(10_000),
            |mut cache| black_box(cache.list()),
            BatchSize::LargeInput,
        );
    });
    let mut cache = ListCache::new(10_000);
    let _ = cache.list();
    c.bench_function("cache_hit_10000", |b| {
        b.iter(|| black_box(cache.list()));
    });
    let cards = cache.list();
    let view = AgentView::new("files-agent", ["files"]);
    c.bench_function("agent_filter_10000", |b| {
        b.iter(|| black_box(view.filter(black_box(&cards))));
    });
}

criterion_group!(benches, lazy_list);
criterion_main!(benches);
