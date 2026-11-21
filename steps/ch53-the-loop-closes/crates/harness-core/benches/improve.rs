use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};

use criterion::{criterion_group, criterion_main, Criterion};

fn close_once(c: &mut Criterion) {
    let n = AtomicU64::new(0);
    c.bench_function("close_once", |b| {
        b.iter(|| {
            let id = n.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!("ch53-bench-{}-{id}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let report = harness_core::improve::run(&root).expect("loop");
            let passed = report.after.score.passed;
            let _ = std::fs::remove_dir_all(&root);
            black_box(passed);
        });
    });
}

criterion_group!(benches, close_once);
criterion_main!(benches);
