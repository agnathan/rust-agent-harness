use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use gw_learned::{predict, LearnedGateway, PredictInput, Use};

fn apply_offered(c: &mut Criterion) {
    let gateway = LearnedGateway::new();
    gateway.open_session("desk");
    let catalog: Vec<String> = (0..32).map(|n| format!("tool_{n}")).collect();
    let usage = vec![Use {
        task: "tool work".into(),
        tool: "tool_0".into(),
    }];
    gateway.refresh_prediction(&PredictInput {
        task: "tool work",
        usage: &usage,
        memory: "tool_0",
        session_note: "",
        catalog: &catalog,
    });
    let _ = gateway.session_tools("desk");
    c.bench_function("apply_offered", |b| {
        b.iter(|| {
            let view = gateway.session_tools(black_box("desk")).expect("session");
            black_box(view);
        });
    });
}

fn predict_offline(c: &mut Criterion) {
    let catalog: Vec<String> = (0..64).map(|n| format!("tool_{n}")).collect();
    let usage: Vec<Use> = (0..1_000)
        .map(|n| Use {
            task: format!("task {}", n % 17),
            tool: format!("tool_{}", n % 64),
        })
        .collect();
    let input = PredictInput {
        task: "task 3",
        usage: &usage,
        memory: "tool_3 is the one for this",
        session_note: "tool_9",
        catalog: &catalog,
    };
    c.bench_function("predict_offline", |b| {
        b.iter(|| {
            let names = predict(black_box(&input));
            black_box(names);
        });
    });
}

criterion_group!(benches, apply_offered, predict_offline);
criterion_main!(benches);
