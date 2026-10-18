use std::hint::black_box;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use sessions::Store;

fn wal_append(c: &mut Criterion) {
    let path: PathBuf = std::env::temp_dir().join(format!("ch15-bench-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let mut store = Store::open(&path).expect("open");
    store.migrate().expect("migrate");
    c.bench_function("wal_append", |b| {
        b.iter(|| {
            let rows = store
                .append_turn(black_box("bench"), black_box("hi"), black_box("pong"))
                .expect("append");
            black_box(rows);
        });
    });
    drop(store);
    let _ = std::fs::remove_file(&path);
}

criterion_group!(benches, wal_append);
criterion_main!(benches);
