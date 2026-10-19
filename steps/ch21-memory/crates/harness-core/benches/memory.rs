use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::hooks::{MemoryBackend, TurnRecord};
use harness_core::memory_files::MemoryFiles;

fn memory_enqueue(c: &mut Criterion) {
    let root = std::env::temp_dir().join(format!("ch18-bench-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let files = MemoryFiles::open(&root).expect("open");
    let record = TurnRecord {
        user: "hi",
        assistant: "ok",
    };
    c.bench_function("memory_enqueue", |b| {
        b.iter(|| {
            files.enqueue_turn(black_box(&record)).expect("queue");
        });
    });
    drop(files);
    let _ = std::fs::remove_dir_all(&root);
}

criterion_group!(benches, memory_enqueue);
criterion_main!(benches);
