use std::hint::black_box;
use std::sync::Mutex;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use harness_core::blackboard::{tick, Board, Waiter};

fn claim_with_waiters(c: &mut Criterion) {
    c.bench_function("claim_with_waiters", |b| {
        b.iter_batched(
            || {
                let mut waiters = Vec::with_capacity(100);
                for n in 0..99 {
                    waiters.push(Waiter::new(format!("w{n}"), format!("k{n}")));
                }
                waiters.push(Waiter::new("chosen", "ready"));
                let mut board = Board::with_waiters(waiters);
                board.post("ready", "notes").expect("clean fact");
                Mutex::new(board)
            },
            |board| {
                let claim = tick(black_box(&board));
                black_box(claim);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, claim_with_waiters);
criterion_main!(benches);
