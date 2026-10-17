use std::collections::VecDeque;
use std::hint::black_box;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use team_skills::{skill_propose, swap_index, Index, Library, Proposal, Skill, Trust};

fn skill_propose_enqueue(c: &mut Criterion) {
    let proposal = Proposal {
        name: "cite".into(),
        body: "Name the file.".into(),
        force: false,
        trust: Trust::Community,
    };
    c.bench_function("skill_propose", |b| {
        b.iter_batched(
            VecDeque::new,
            |mut queue| {
                let queued = skill_propose(black_box(&mut queue), proposal.clone());
                black_box((queued, queue));
            },
            BatchSize::SmallInput,
        );
    });
}

fn index_swap(c: &mut Criterion) {
    c.bench_function("index_swap", |b| {
        b.iter_batched(
            || {
                let library = Library::new(Index::bundled(vec![Skill {
                    name: "echo".into(),
                    body: "Say it back.".into(),
                    layer: team_skills::Layer::Bundled,
                }]));
                let next = Arc::new(Index {
                    skills: vec![Skill {
                        name: "cite".into(),
                        body: "Name the file.".into(),
                        layer: team_skills::Layer::Team,
                    }],
                    version: 2,
                    team_md: "# TEAM\n- cite (team)\n".into(),
                });
                (library, next)
            },
            |(library, next)| {
                let previous = swap_index(black_box(&library), next);
                black_box(previous);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, skill_propose_enqueue, index_swap);
criterion_main!(benches);
