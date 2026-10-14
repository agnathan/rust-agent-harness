use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{fixture_binary, HttpFixture, McpSession};

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("tokio")
}

fn http_echo_round_trip(c: &mut Criterion) {
    let runtime = runtime();
    let fixture = runtime
        .block_on(HttpFixture::spawn(fixture_binary()))
        .expect("http");
    let session = runtime
        .block_on(McpSession::streamable_http(&fixture.url))
        .expect("connect");
    c.bench_function("http_echo_round_trip", |b| {
        b.to_async(&runtime).iter(|| async {
            let text = session
                .call_text("echo", serde_json::json!({"text": "ping"}))
                .await
                .expect("echo");
            black_box(text);
        });
    });
    runtime.block_on(session.shutdown()).ok();
}

criterion_group!(benches, http_echo_round_trip);
criterion_main!(benches);
