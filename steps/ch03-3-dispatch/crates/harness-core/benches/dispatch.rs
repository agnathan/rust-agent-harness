use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use harness_core::{Dispatcher, LatencyTool, Tool, ToolCall, ToolKind, ToolRegistry};
use serde_json::json;

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_time()
        .build()
        .expect("tokio runtime")
}

fn batch(delay: Duration) -> (ToolRegistry, Vec<ToolCall>) {
    let mut registry = ToolRegistry::new();
    let calls = (0..8)
        .map(|index| {
            let name = format!("read-{index}");
            registry
                .register(
                    Arc::new(LatencyTool::new(name.clone(), ToolKind::Read, delay))
                        as Arc<dyn Tool>,
                )
                .expect("register");
            ToolCall {
                name,
                arguments: json!({}),
            }
        })
        .collect();
    (registry, calls)
}

fn dispatch_8_reads_concurrent(c: &mut Criterion) {
    let runtime = runtime();
    let delay = Duration::from_millis(1);
    c.bench_function("dispatch_8_reads_concurrent", |b| {
        b.to_async(&runtime).iter_batched(
            || batch(delay),
            |(registry, calls)| async move {
                let outputs = Dispatcher::new(8).run(&registry, &calls).await;
                black_box(outputs);
            },
            BatchSize::SmallInput,
        );
    });
}

fn dispatch_8_reads_sequential(c: &mut Criterion) {
    let runtime = runtime();
    let delay = Duration::from_millis(1);
    c.bench_function("dispatch_8_reads_sequential", |b| {
        b.to_async(&runtime).iter_batched(
            || batch(delay),
            |(registry, calls)| async move {
                let outputs = Dispatcher::new(8).run_sequential(&registry, &calls).await;
                black_box(outputs);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    dispatch_8_reads_concurrent,
    dispatch_8_reads_sequential
);
criterion_main!(benches);
