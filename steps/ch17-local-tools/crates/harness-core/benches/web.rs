use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::web::{is_safe_url, search_result, FakeSearch, StdResolver};

fn fake_search(c: &mut Criterion) {
    let backend = FakeSearch::example();
    c.bench_function("fake_search", |b| {
        b.iter(|| {
            let raw = search_result(black_box(&backend), black_box("rust"), black_box(Some(5)));
            black_box(raw)
        });
    });
}

fn ssrf_block(c: &mut Criterion) {
    let resolver = StdResolver;
    c.bench_function("ssrf_block", |b| {
        b.iter(|| {
            let blocked = is_safe_url(
                black_box("http://169.254.169.254/latest"),
                black_box(true),
                black_box(&resolver),
            );
            black_box(blocked)
        });
    });
}

criterion_group!(benches, fake_search, ssrf_block);
criterion_main!(benches);
