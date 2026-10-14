use std::hint::black_box;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{fixture_binary, McpSession, ToolListCache};

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("tokio")
}

fn cache_cold(c: &mut Criterion) {
    let runtime = runtime();
    let session = Arc::new(
        runtime
            .block_on(McpSession::stdio(fixture_binary()))
            .expect("fixture"),
    );
    c.bench_function("mcp_list_cold", |b| {
        b.to_async(&runtime).iter(|| async {
            let mut cache = ToolListCache::new(Arc::clone(&session));
            let listed = cache.list().await.expect("list");
            black_box(listed.len());
        });
    });
}

fn cache_hit(c: &mut Criterion) {
    let runtime = runtime();
    let session = Arc::new(
        runtime
            .block_on(McpSession::stdio(fixture_binary()))
            .expect("fixture"),
    );
    let mut cache = ToolListCache::new(session);
    runtime.block_on(cache.list()).expect("warm");
    c.bench_function("mcp_list_hit", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let listed = cache.list().await.expect("hit");
                black_box(listed.len());
            });
        });
    });
}

criterion_group!(benches, cache_cold, cache_hit);
criterion_main!(benches);
