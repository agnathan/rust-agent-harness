use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use fanout::{Board, Task, QUEUE_CAP};

fn schedule_100(c: &mut Criterion) {
    c.bench_function("schedule_100", |b| {
        b.iter_batched(
            Board::new,
            |board| {
                for index in 0..QUEUE_CAP {
                    board
                        .schedule(Task {
                            task_index: index,
                            reply: format!("reply {index}"),
                            tool_trace: "secret-tool".into(),
                        })
                        .expect("schedule");
                }
                black_box(board.len());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, schedule_100);
criterion_main!(benches);
