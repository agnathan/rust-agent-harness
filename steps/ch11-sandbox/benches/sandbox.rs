use std::hint::black_box;
use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use sandbox::{thousand_files, Manifest, Worker};

fn warm_accept(c: &mut Criterion) {
    let bin = Path::new(env!("CARGO_BIN_EXE_sandbox-worker"));
    let mut worker = Worker::spawn(bin).expect("worker");
    c.bench_function("warm_accept", |b| {
        b.iter(|| {
            let reply = worker.accept(black_box("ping")).expect("accept");
            black_box(reply);
        });
    });
    worker.quit().expect("quit");
}

fn checkpoint_1000(c: &mut Criterion) {
    let files = thousand_files();
    c.bench_function("checkpoint_1000", |b| {
        b.iter(|| {
            let snap = Manifest::checkpoint(black_box(&files));
            black_box(snap);
        });
    });
}

fn rewind_1000(c: &mut Criterion) {
    let snap = Manifest::checkpoint(&thousand_files());
    c.bench_function("rewind_1000", |b| {
        b.iter(|| {
            let mut live = Vec::new();
            snap.rewind_into(black_box(&mut live));
            black_box(live);
        });
    });
}

criterion_group!(benches, warm_accept, checkpoint_1000, rewind_1000);
criterion_main!(benches);
