use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{generate, schema_cost, ToolSession};

fn search_tool(c: &mut Criterion) {
    let cards = generate(10_000);
    c.bench_function("full_schema_tokens_10000", |b| {
        b.iter(|| {
            let cost = schema_cost(black_box(&cards));
            black_box(cost.tokens);
        });
    });
    let mut session = ToolSession::new(generate(10_000), 8);
    let _ = session.search_tools("rename a file", 8);
    session.begin_turn();
    c.bench_function("lazy_visible_prompt_10000", |b| {
        b.iter(|| black_box(session.visible_prompt()));
    });
}

criterion_group!(benches, search_tool);
criterion_main!(benches);
