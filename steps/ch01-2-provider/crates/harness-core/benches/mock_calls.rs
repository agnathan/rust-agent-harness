use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{Message, MockProvider, Provider};
use tokio::task::JoinSet;

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_time()
        .build()
        .expect("tokio runtime")
}

fn mock_turn_overhead(c: &mut Criterion) {
    let rt = runtime();
    let provider = MockProvider::with_latency("ok", Duration::ZERO);
    let messages = vec![Message::user("hello")];
    c.bench_function("mock_turn_overhead", |b| {
        b.to_async(&rt).iter(|| async {
            let reply = provider.complete(black_box(&messages)).await.expect("mock");
            black_box(reply);
        });
    });
}

fn mock_100_concurrent(c: &mut Criterion) {
    let rt = runtime();
    let provider = Arc::new(MockProvider::with_latency("ok", Duration::ZERO));
    let messages = vec![Message::user("hello")];
    c.bench_function("mock_100_concurrent", |b| {
        b.to_async(&rt).iter(|| {
            let provider = Arc::clone(&provider);
            let messages = messages.clone();
            async move {
                let mut set = JoinSet::new();
                for _ in 0..100 {
                    let provider = Arc::clone(&provider);
                    let messages = messages.clone();
                    set.spawn(async move {
                        let reply = provider.complete(&messages).await.expect("mock");
                        black_box(reply);
                    });
                }
                while set.join_next().await.is_some() {}
            }
        });
    });
}

criterion_group!(benches, mock_turn_overhead, mock_100_concurrent);
criterion_main!(benches);
