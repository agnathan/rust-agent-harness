use std::hint::black_box;

use bus::{publish, Bus, Envelope};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn publish_to_agents(c: &mut Criterion) {
    c.bench_function("publish_to_agents", |b| {
        b.iter_batched(
            || {
                let mut bus = Bus::new();
                let mut receivers = Vec::with_capacity(100);
                for n in 0..100 {
                    receivers.push(bus.subscribe(format!("a{n}"), "work"));
                }
                (bus, receivers)
            },
            |(mut bus, receivers)| {
                let result = publish(
                    black_box(&mut bus),
                    Envelope {
                        topic: "work".into(),
                        body: "ping".into(),
                    },
                );
                black_box((result, receivers));
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, publish_to_agents);
criterion_main!(benches);
