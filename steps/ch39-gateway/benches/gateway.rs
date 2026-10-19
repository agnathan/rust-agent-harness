use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use gateway::{Inbox, Message, INBOX_CAP};

fn enqueue_platform(c: &mut Criterion) {
    c.bench_function("enqueue_platform", |b| {
        b.iter_batched(
            || Inbox::new(INBOX_CAP),
            |mut inbox| {
                let submitted = inbox.submit(Message::Turn(black_box("hello".to_string())));
                black_box(submitted);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, enqueue_platform);
criterion_main!(benches);
