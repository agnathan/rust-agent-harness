use std::hint::black_box;
use std::sync::Arc;

use contextprep::{share, CONTEXT_FILE_MAX_CHARS};
use criterion::{criterion_group, criterion_main, Criterion};

fn share_snapshot(c: &mut Criterion) {
    let snapshot = Arc::new("rule ".repeat(CONTEXT_FILE_MAX_CHARS / 5));
    c.bench_function("share_snapshot", |b| {
        b.iter(|| {
            let shared = share(black_box(&snapshot));
            black_box(shared);
        });
    });
}

criterion_group!(benches, share_snapshot);
criterion_main!(benches);
