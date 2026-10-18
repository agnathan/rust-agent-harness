use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::{run_script, step, Phase, ScriptStep, TurnEvent};

fn one_transition(c: &mut Criterion) {
    c.bench_function("turn_transition", |b| {
        b.iter(|| {
            let next = step(black_box(Phase::Idle), black_box(TurnEvent::Start)).expect("step");
            black_box(next);
        });
    });
}

fn scripted_turn(c: &mut Criterion) {
    c.bench_function("scripted_turn", |b| {
        b.iter(|| {
            let phase = run_script(black_box(&[
                ScriptStep::Tools,
                ScriptStep::Tools,
                ScriptStep::Text,
            ]))
            .expect("script");
            black_box(phase);
        });
    });
}

criterion_group!(benches, one_transition, scripted_turn);
criterion_main!(benches);
