use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use supervisor::{run_specialists, Job, Pool, WORKER_BOUND};

fn schedule_eight(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    let jobs: Vec<Job> = (0..WORKER_BOUND)
        .map(|i| Job {
            name: "specialist",
            reply: format!("reply {i}"),
            hold: None,
        })
        .collect();
    c.bench_function("schedule_eight", |b| {
        b.iter(|| {
            let replies = run_specialists(black_box(&pool), black_box(&jobs));
            black_box(replies);
        });
    });
}

fn enqueue_reflection(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    c.bench_function("enqueue_reflection", |b| {
        b.iter(|| {
            pool.enqueue_reflection(black_box("team run finished"));
        });
    });
}

criterion_group!(benches, schedule_eight, enqueue_reflection);
criterion_main!(benches);
