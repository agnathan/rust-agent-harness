use std::hint::black_box;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use gw_minimal::{CallGate, ACCEPT_BOUND, EXECUTE_BOUND};
use tokio::runtime::Builder;

fn echo_through_gate(c: &mut Criterion) {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_time()
        .build()
        .expect("runtime");
    let gate = CallGate::new(ACCEPT_BOUND, EXECUTE_BOUND);
    c.bench_function("echo_through_gate", |b| {
        b.to_async(&runtime).iter(|| async {
            let text = gate
                .run(|| async { black_box("hello") })
                .await
                .expect("permit");
            black_box(text);
        });
    });
}

fn eight_sleeps(c: &mut Criterion) {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_time()
        .build()
        .expect("runtime");
    let gate = CallGate::new(ACCEPT_BOUND, EXECUTE_BOUND);
    c.bench_function("eight_concurrent_sleeps", |b| {
        b.to_async(&runtime).iter(|| async {
            let mut tasks = Vec::with_capacity(8);
            for _ in 0..8 {
                let gate = gate.clone();
                tasks.push(tokio::spawn(async move {
                    gate.run(|| async {
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    })
                    .await
                    .expect("permit");
                }));
            }
            for task in tasks {
                task.await.expect("join");
            }
        });
    });
}

criterion_group!(benches, echo_through_gate, eight_sleeps);
criterion_main!(benches);
