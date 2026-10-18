use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use gw_limits::{CallOpts, FailureKind, LimitedGateway, Outcome, Script};

fn open_breaker(c: &mut Criterion) {
    let gateway = LimitedGateway::new();
    let script = Script::new(vec![
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
    ]);
    for _ in 0..3 {
        let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    }
    let idle = Script::new(vec![]);
    c.bench_function("open_breaker", |b| {
        b.iter(|| {
            let err = gateway.dispatch(black_box("search"), "", CallOpts::default(), &idle);
            let _ = black_box(err);
        });
    });
}

fn permit_release(c: &mut Criterion) {
    let gateway = LimitedGateway::new();
    c.bench_function("permit_release", |b| {
        b.iter(|| {
            gateway
                .roundtrip_execute(black_box("echo"))
                .expect("permit");
        });
    });
}

criterion_group!(benches, open_breaker, permit_release);
criterion_main!(benches);
