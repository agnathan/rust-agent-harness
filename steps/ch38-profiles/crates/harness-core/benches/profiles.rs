use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::profiles::{Profile, Profiles};

fn resolve_profile(c: &mut Criterion) {
    let mut store = Profiles::new(true);
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.set_scope(Some("writer"));
    c.bench_function("resolve_profile", |b| {
        b.iter(|| {
            let resolved = store
                .resolve(black_box("writer"), black_box("API_KEY"))
                .expect("scoped");
            black_box(resolved);
        });
    });
}

criterion_group!(benches, resolve_profile);
criterion_main!(benches);
