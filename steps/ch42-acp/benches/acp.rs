use std::hint::black_box;

use acp::{translate, Inbox, Update, INBOX_CAP};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn translate_update(c: &mut Criterion) {
    let update = Update {
        text: "hello".to_string(),
        cancelled: false,
    };
    c.bench_function("translate_update", |b| {
        b.iter_batched(
            || Inbox::new(INBOX_CAP),
            |mut inbox| {
                let mail = translate(black_box(&update));
                let submitted = inbox.submit(mail);
                black_box(submitted);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, translate_update);
criterion_main!(benches);
