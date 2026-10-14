use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{StaticTool, Tool, ToolKind, ToolRegistry};

fn registry_of(count: usize) -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    for index in 0..count {
        registry
            .register(StaticTool::arc(format!("tool-{index:02}"), ToolKind::Read))
            .expect("register");
    }
    registry
}

fn registry_lookup(c: &mut Criterion) {
    let registry = registry_of(50);
    c.bench_function("registry_lookup", |b| {
        b.iter(|| {
            let tool = registry.get(black_box("tool-25")).expect("tool");
            black_box(tool.kind());
        });
    });
}

criterion_group!(benches, registry_lookup);
criterion_main!(benches);
