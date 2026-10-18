use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::adapters::{choose_next, Pool};

fn choose_route(c: &mut Criterion) {
    let models = vec![
        "primary".to_string(),
        "fallback".to_string(),
        "last".to_string(),
    ];
    c.bench_function("choose_route", |b| {
        b.iter(|| {
            let next = choose_next(black_box(&models), black_box(Some("primary")));
            black_box(next);
        });
    });
}

fn rotate_key(c: &mut Criterion) {
    let mut pool = Pool::new(vec!["sk-one".into(), "sk-two".into(), "sk-three".into()]);
    c.bench_function("rotate_key", |b| {
        b.iter(|| {
            let key = pool.mark_exhausted_and_rotate();
            black_box(key);
        });
    });
}

criterion_group!(benches, choose_route, rotate_key);
criterion_main!(benches);
