use std::hint::black_box;

use actors::{submit, Mail, INBOX_CAP};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use tokio::sync::mpsc;

fn inbox_submit(c: &mut Criterion) {
    c.bench_function("inbox_submit", |b| {
        b.iter_batched(
            || mpsc::channel::<Mail>(INBOX_CAP),
            |(tx, rx)| {
                submit(black_box(&tx), black_box(Mail::Turn("hello".into()))).unwrap();
                black_box(rx);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, inbox_submit);
criterion_main!(benches);
