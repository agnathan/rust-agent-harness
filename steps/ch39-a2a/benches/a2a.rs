use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use a2a::{cancel, wait_cancel};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn cancel_inflight(c: &mut Criterion) {
    c.bench_function("cancel_inflight", |b| {
        b.iter_batched(
            || {
                let flag = Arc::new(AtomicBool::new(false));
                let started = Arc::new(AtomicBool::new(false));
                let flag_worker = Arc::clone(&flag);
                let started_worker = Arc::clone(&started);
                let handle = thread::spawn(move || wait_cancel(&flag_worker, &started_worker));
                while !started.load(Ordering::SeqCst) {
                    thread::yield_now();
                }
                (flag, handle)
            },
            |(flag, handle)| {
                cancel(black_box(&flag));
                handle.join().expect("worker");
                black_box(());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, cancel_inflight);
criterion_main!(benches);
