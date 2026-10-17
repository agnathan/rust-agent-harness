use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use gw_catalog::{synthetic, Gateway, Load};

fn gateway_of(count: usize) -> Gateway {
    let gateway = Gateway::new();
    gateway
        .reload(Load {
            native: synthetic(count),
            plugins: vec![],
            upstreams: vec![],
        })
        .expect("reload");
    gateway
}

fn catalog_search(c: &mut Criterion) {
    let gateway = gateway_of(10_000);
    c.bench_function("catalog_search_10k", |b| {
        b.iter(|| {
            let hits = gateway.search(black_box("rename a file"), 8);
            black_box(hits);
        });
    });
}

fn catalog_reload(c: &mut Criterion) {
    let gateway = gateway_of(10_000);
    let cards = synthetic(10_000);
    c.bench_function("catalog_reload_10k", |b| {
        b.iter(|| {
            gateway
                .reload(Load {
                    native: cards.clone(),
                    plugins: vec![],
                    upstreams: vec![],
                })
                .expect("reload");
            black_box(gateway.len());
        });
    });
}

fn catalog_swap(c: &mut Criterion) {
    let gateway = gateway_of(1);
    let cards = synthetic(1);
    c.bench_function("catalog_arc_swap", |b| {
        b.iter(|| {
            let held = gateway.snapshot();
            gateway
                .reload(Load {
                    native: cards.clone(),
                    plugins: vec![],
                    upstreams: vec![],
                })
                .expect("reload");
            black_box(held.len());
        });
    });
}

criterion_group!(benches, catalog_search, catalog_reload, catalog_swap);
criterion_main!(benches);
