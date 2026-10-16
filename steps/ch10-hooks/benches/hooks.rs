use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use hooks::{
    empty_pre_post, run_api_hook, MemoryBackend, MemoryQueue, Message, NopApi, NopHook, TurnRecord,
};
use serde_json::json;

fn empty_hook(c: &mut Criterion) {
    let hook = NopHook;
    let args = json!({});
    c.bench_function("empty_pre_post", |b| {
        b.iter(|| {
            let n = empty_pre_post(black_box(&hook), black_box(&args));
            black_box(n);
        });
    });
}

fn api_hook(c: &mut Criterion) {
    let messages = vec![
        Message {
            role: "user".into(),
            content: "hello".into(),
        };
        1_000
    ];
    let hook = NopApi;
    c.bench_function("api_request_hook", |b| {
        b.iter(|| {
            run_api_hook(black_box(&hook), black_box(&messages));
        });
    });
}

fn memory_enqueue(c: &mut Criterion) {
    let queue = MemoryQueue::new();
    c.bench_function("memory_enqueue", |b| {
        b.iter(|| {
            queue
                .enqueue_turn(black_box(&TurnRecord {
                    user: "hello",
                    assistant: "ok",
                }))
                .expect("enqueue");
        });
    });
}

criterion_group!(benches, empty_hook, api_hook, memory_enqueue);
criterion_main!(benches);
