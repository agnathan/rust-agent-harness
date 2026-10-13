use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{fixture_binary, McpSession};

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("tokio")
}

fn stdio_echo_round_trip(c: &mut Criterion) {
    let runtime = runtime();
    let session = runtime
        .block_on(McpSession::stdio(&fixture_binary()))
        .expect("fixture");
    c.bench_function("stdio_echo_round_trip", |b| {
        b.to_async(&runtime).iter(|| async {
            let text = session
                .call_text("echo", serde_json::json!({"text": "ping"}))
                .await
                .expect("echo");
            black_box(text);
        });
    });
    runtime.block_on(session.shutdown()).expect("shutdown");
}

criterion_group!(benches, stdio_echo_round_trip);
criterion_main!(benches);
