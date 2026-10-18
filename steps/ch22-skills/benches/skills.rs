use std::fs;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use skills::{load_tree, render_index};

fn load_render_500(c: &mut Criterion) {
    let root = std::env::temp_dir().join(format!("ch18-bench-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    for n in 0..500 {
        let category = format!("cat{}", n % 10);
        let name = format!("skill{n:03}");
        let dir = root.join(&category).join(&name);
        fs::create_dir_all(&dir).expect("dir");
        let text = format!("---\nname: {name}\ndescription: Skill {n}\n---\nBody {n}.\n");
        fs::write(dir.join("SKILL.md"), text).expect("write");
    }
    c.bench_function("load_render_500", |b| {
        b.iter(|| {
            let skills = load_tree(black_box(&root)).expect("load");
            let index = render_index(black_box(&skills), black_box(&[]));
            black_box(index);
        });
    });
    let _ = fs::remove_dir_all(&root);
}

criterion_group!(benches, load_render_500);
criterion_main!(benches);
