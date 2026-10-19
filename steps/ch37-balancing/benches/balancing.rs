use std::hint::black_box;

use balancing::federation::{Backend, Policy, Pool};
use criterion::{criterion_group, criterion_main, Criterion};

fn pick_least(c: &mut Criterion) {
    let pool = Pool::new(vec![
        Backend::new("a", "echo", 3),
        Backend::new("b", "echo", 1),
        Backend::new("c", "echo", 4),
        Backend::new("d", "echo", 2),
    ]);
    c.bench_function("pick_least", |b| {
        b.iter(|| {
            let index = pool.pick(black_box(Policy::LeastOutstanding));
            black_box(index);
        });
    });
}

criterion_group!(benches, pick_least);
criterion_main!(benches);
