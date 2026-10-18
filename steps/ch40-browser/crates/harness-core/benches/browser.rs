use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::browser::{navigate, public_resolver, FakeBrowser, Session};

fn fake_navigate(c: &mut Criterion) {
    let backend = FakeBrowser::example();
    let resolver = public_resolver();
    c.bench_function("fake_navigate", |b| {
        b.iter(|| {
            let mut session = Session::new();
            let raw = navigate(
                black_box(&mut session),
                black_box(&backend),
                black_box("https://example.com/notes"),
                black_box(&resolver),
            )
            .expect("canned page");
            black_box(raw);
        });
    });
}

criterion_group!(benches, fake_navigate);
criterion_main!(benches);
