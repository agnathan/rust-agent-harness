use std::hint::black_box;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{Message, MockProvider, Provider};

fn turn_overhead(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("tokio runtime");
    let provider = MockProvider::with_latency("ok", Duration::ZERO);
    let messages = vec![Message::user("hello")];

    c.bench_function("mock_turn_overhead", |b| {
        b.to_async(&runtime).iter(|| async {
            let reply = provider.complete(black_box(&messages)).await.expect("mock");
            black_box(reply);
        });
    });
}

criterion_group!(benches, turn_overhead);
criterion_main!(benches);
