use std::hint::black_box;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use kanban::{transact, write_board, Board, Op, Task};

fn claim_transaction(c: &mut Criterion) {
    let mut n = 0u64;
    c.bench_function("claim_transaction", |b| {
        b.iter_batched(
            || {
                n += 1;
                let dir =
                    std::env::temp_dir().join(format!("ch36-bench-{}-{n}", std::process::id()));
                let _ = std::fs::remove_dir_all(&dir);
                write_board(&dir, &Board::new(vec![Task::ready("t1")])).expect("board");
                dir
            },
            |dir: PathBuf| {
                let status = transact(
                    black_box(&dir),
                    "t1",
                    Op::Claim {
                        owner: "worker".into(),
                    },
                )
                .expect("claim");
                let _ = std::fs::remove_dir_all(&dir);
                black_box(status);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, claim_transaction);
criterion_main!(benches);
