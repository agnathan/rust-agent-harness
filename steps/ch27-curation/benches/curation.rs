use std::fs;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use curation::Curator;

fn enqueue_curation(c: &mut Criterion) {
    let root = std::env::temp_dir().join(format!("ch23-bench-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let curator = Curator::open(&root).expect("open");
    c.bench_function("enqueue_curation", |b| {
        b.iter(|| {
            curator.enqueue();
            black_box(curator.queue_len());
        });
    });
    let _ = fs::remove_dir_all(&root);
}

criterion_group!(benches, enqueue_curation);
criterion_main!(benches);
