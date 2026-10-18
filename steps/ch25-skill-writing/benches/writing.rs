use std::fs;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use skill_write::{scan_text, Writer};

fn scan_one_skill(c: &mut Criterion) {
    let mut body = String::from(
        "---\nname: rename\ndescription: Rename a file by its path.\norigin: agent\n---\n",
    );
    body.push_str("Rename a file by its path. ");
    while body.len() < 1024 {
        body.push_str("Keep the old name in the note. ");
    }
    let _ = scan_text(&body);
    c.bench_function("scan_one_skill", |b| {
        b.iter(|| {
            let result = scan_text(black_box(&body));
            black_box(result);
        });
    });
}

fn enqueue_unsolicited(c: &mut Criterion) {
    let root = std::env::temp_dir().join(format!("ch21-bench-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let writer = Writer::open(&root).expect("open");
    c.bench_function("enqueue_unsolicited", |b| {
        b.iter(|| {
            writer.enqueue_background(black_box("create notes"));
        });
    });
    let _ = fs::remove_dir_all(&root);
}

criterion_group!(benches, scan_one_skill, enqueue_unsolicited);
criterion_main!(benches);
