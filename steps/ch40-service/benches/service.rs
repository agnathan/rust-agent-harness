use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use gwservice::Service;

fn route_session(c: &mut Criterion) {
    let mut service = Service::new(&["a", "b", "c", "d"], 8);
    for n in 0..64 {
        service.place(&format!("s{n}"));
    }
    c.bench_function("route_session", |b| {
        b.iter(|| {
            let route = service.route(black_box("s0"));
            black_box(route);
        });
    });
}

criterion_group!(benches, route_session);
criterion_main!(benches);
