use std::hint::black_box;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{
    fixture_binary, register_remote, Dispatcher, McpSession, ToolCall, ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .expect("tokio")
}

fn batch(count: usize) -> Vec<ToolCall> {
    (0..count)
        .map(|_| ToolCall {
            name: "echo".into(),
            arguments: json!({"text": "ping"}),
        })
        .collect()
}

fn mcp_50_concurrent(c: &mut Criterion) {
    let runtime = runtime();
    let session = Arc::new(
        runtime
            .block_on(McpSession::stdio(fixture_binary()))
            .expect("fixture"),
    );
    let mut registry = ToolRegistry::new();
    runtime
        .block_on(register_remote(&mut registry, Arc::clone(&session)))
        .expect("register");
    let calls = batch(50);
    c.bench_function("mcp_50_concurrent", |b| {
        b.to_async(&runtime).iter(|| async {
            let outputs = Dispatcher::new(DISPATCH_BOUND).run(&registry, &calls).await;
            black_box(outputs.len());
        });
    });
}

fn mcp_50_sequential(c: &mut Criterion) {
    let runtime = runtime();
    let session = Arc::new(
        runtime
            .block_on(McpSession::stdio(fixture_binary()))
            .expect("fixture"),
    );
    let mut registry = ToolRegistry::new();
    runtime
        .block_on(register_remote(&mut registry, session))
        .expect("register");
    let calls = batch(50);
    c.bench_function("mcp_50_sequential", |b| {
        b.to_async(&runtime).iter(|| async {
            let outputs = Dispatcher::new(DISPATCH_BOUND)
                .run_sequential(&registry, &calls)
                .await;
            black_box(outputs.len());
        });
    });
}

criterion_group!(benches, mcp_50_concurrent, mcp_50_sequential);
criterion_main!(benches);
