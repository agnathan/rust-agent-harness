use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use cron::{Job, Store};

fn scan_thousand(c: &mut Criterion) {
    let mut store = Store::new();
    for n in 0..1000 {
        store.add(Job::new(format!("j{n}"), 0, "summarize"));
    }
    c.bench_function("scan_thousand", |b| {
        b.iter(|| {
            let due = store.scan_due(black_box(0));
            black_box(due);
        });
    });
}

criterion_group!(benches, scan_thousand);
criterion_main!(benches);
