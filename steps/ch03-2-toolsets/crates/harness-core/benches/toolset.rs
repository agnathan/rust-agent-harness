use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{Catalog, EnvMap, StaticTool, ToolKind, Toolset};

fn catalog_of(count: usize) -> (Catalog, EnvMap) {
    let tools = (0..count)
        .map(|index| StaticTool::arc(format!("tool-{index:02}"), ToolKind::Read))
        .collect();
    let mut catalog = Catalog::new();
    catalog.add(Toolset::gated("bench", "HARNESS_BENCH", tools));
    let mut env = EnvMap::new();
    env.insert("HARNESS_BENCH", "1");
    (catalog, env)
}

fn available_tools_50(c: &mut Criterion) {
    let (catalog, env) = catalog_of(50);
    c.bench_function("available_tools_50", |b| {
        b.iter(|| {
            let schemas = catalog.available_schemas(black_box(&env)).expect("schemas");
            black_box(schemas.len());
        });
    });
}

criterion_group!(benches, available_tools_50);
criterion_main!(benches);
