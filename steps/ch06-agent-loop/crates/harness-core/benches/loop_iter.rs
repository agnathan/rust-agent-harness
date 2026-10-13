use std::hint::black_box;
use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{
    check_gate, eight_delay_calls, run_loop, AgentUsage, Budget, CancelFlag, DelayRead, Dispatcher,
    ScriptedProvider, ScriptedReply, Tool, ToolRegistry, DISPATCH_BOUND,
};

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("tokio")
}

fn gate_check(c: &mut Criterion) {
    let cancel = CancelFlag::new();
    let budget = Budget::default();
    let usage = AgentUsage::default();
    let started = Instant::now();
    c.bench_function("gate_check", |b| {
        b.iter(|| black_box(check_gate(&cancel, &budget, &usage, started)));
    });
}

fn loop_iteration(c: &mut Criterion) {
    let runtime = runtime();
    let registry = ToolRegistry::new();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    c.bench_function("loop_iteration", |b| {
        b.to_async(&runtime).iter(|| async {
            let provider = ScriptedProvider::new(vec![ScriptedReply::text("ok")]);
            let mut messages = vec![harness_core::Message::user("status")];
            let outcome = run_loop(
                &mut messages,
                &provider,
                &registry,
                &dispatcher,
                &Budget::default(),
                &CancelFlag::new(),
                Instant::now(),
            )
            .await;
            black_box(outcome.stop);
        });
    });
}

fn loop_8_reads(c: &mut Criterion) {
    let runtime = runtime();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(DelayRead::new(Duration::from_millis(1))) as Arc<dyn Tool>)
        .unwrap();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    c.bench_function("loop_8_reads", |b| {
        b.to_async(&runtime).iter(|| async {
            let provider = ScriptedProvider::new(vec![
                ScriptedReply::tools(eight_delay_calls()),
                ScriptedReply::text("done"),
            ]);
            let mut messages = vec![harness_core::Message::user("fan out")];
            let outcome = run_loop(
                &mut messages,
                &provider,
                &registry,
                &dispatcher,
                &Budget::default(),
                &CancelFlag::new(),
                Instant::now(),
            )
            .await;
            black_box(outcome.stop);
        });
    });
}

criterion_group!(benches, gate_check, loop_iteration, loop_8_reads);
criterion_main!(benches);
