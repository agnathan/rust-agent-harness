use std::hint::black_box;
use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{
    check_gate, eight_delay_calls, run_loop, tool_definitions, AgentUsage, Budget, CancelFlag,
    DelayRead, Dispatcher, EchoTool, NoteTool, ScriptedProvider, ScriptedReply, Tool, ToolRegistry,
    DISPATCH_BOUND,
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
            black_box(outcome.stop());
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
            black_box(outcome.stop());
        });
    });
}

fn live_tool_body(c: &mut Criterion) {
    let schemas = vec![EchoTool.schema(), NoteTool.schema()];
    c.bench_function("live_tool_body", |b| {
        b.iter(|| black_box(tool_definitions(&schemas)));
    });
}

fn outcome_label(c: &mut Criterion) {
    let outcome = harness_core::TurnOutcome::Finished {
        events: Vec::new(),
        usage: harness_core::AgentUsage::default(),
        text: "ok".into(),
    };
    c.bench_function("outcome_label", |b| {
        b.iter(|| black_box(outcome.label()));
    });
}

fn canonical_args(c: &mut Criterion) {
    let raw = r#"{"b":1,"a":2,"nested":{"z":1,"m":2}}"#;
    c.bench_function("canonical_args", |b| {
        b.iter(|| black_box(harness_core::canonical_json(raw)));
    });
}

fn steer_append(c: &mut Criterion) {
    let mut messages = vec![
        harness_core::Message::user("go"),
        harness_core::Message::tool("call-1", "looked"),
    ];
    c.bench_function("steer_append", |b| {
        b.iter(|| {
            messages[1].content = "looked".to_string();
            black_box(harness_core::append_steer_text(
                &mut messages,
                "use the short form",
            ));
        });
    });
}

fn truncate_output(c: &mut Criterion) {
    let text = "abcdefghij".repeat(400);
    c.bench_function("truncate_output", |b| {
        b.iter(|| black_box(harness_core::truncate_head_tail(&text, 1024)));
    });
}

fn budget_notice(c: &mut Criterion) {
    c.bench_function("budget_notice", |b| {
        b.iter(|| black_box(harness_core::iteration_warning(4, 8)));
    });
}

fn stall_observe(c: &mut Criterion) {
    c.bench_function("stall_observe", |b| {
        b.iter(|| {
            let mut guard = harness_core::StallGuard::new();
            black_box(guard.observe(0, "echo", "{}", "hello", false));
        });
    });
}

fn validate_repair(c: &mut Criterion) {
    let known = vec!["echo".to_string(), "note".to_string()];
    c.bench_function("validate_repair", |b| {
        b.iter(|| black_box(harness_core::repair_name("eco", &known)));
    });
}

fn cancel_select(c: &mut Criterion) {
    let runtime = runtime();
    c.bench_function("cancel_select", |b| {
        b.to_async(&runtime).iter(|| async {
            let flag = CancelFlag::new();
            flag.hard_stop();
            let token = flag.token();
            let hit = tokio::select! {
                biased;
                _ = token.cancelled() => true,
                _ = std::future::ready(()) => false,
            };
            black_box(hit);
        });
    });
}

criterion_group!(
    benches,
    gate_check,
    loop_iteration,
    loop_8_reads,
    live_tool_body,
    cancel_select,
    validate_repair,
    stall_observe,
    budget_notice,
    truncate_output,
    steer_append,
    canonical_args,
    outcome_label
);
criterion_main!(benches);
