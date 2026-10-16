use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use gw_session::{run_rename_task, Arm};

fn dynamic_task(c: &mut Criterion) {
    c.bench_function("dynamic_rename_task", |b| {
        b.iter(|| {
            let report = run_rename_task(Arm::Dynamic, black_box(2_000));
            black_box(report);
        });
    });
}

fn every_task(c: &mut Criterion) {
    c.bench_function("every_tool_rename_task", |b| {
        b.iter(|| {
            let report = run_rename_task(Arm::Every, black_box(2_000));
            black_box(report);
        });
    });
}

criterion_group!(benches, dynamic_task, every_task);
criterion_main!(benches);
