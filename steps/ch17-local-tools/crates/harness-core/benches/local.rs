use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::local_tools::{patch_file, FileState, TodoItem, TodoStore, Workspace};

fn patch_apply(c: &mut Criterion) {
    let root = std::env::temp_dir().join("harness-patch-bench");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join("note.txt");
    std::fs::write(&path, "alpha beta").unwrap();
    let workspace = Workspace::new(&root);
    let mut state = FileState::new();
    harness_core::local_tools::read_file(&workspace, &mut state, "note.txt").unwrap();
    c.bench_function("patch_apply", |b| {
        b.iter(|| {
            std::fs::write(&path, "alpha beta").unwrap();
            state = FileState::new();
            harness_core::local_tools::read_file(&workspace, &mut state, "note.txt").unwrap();
            let result = patch_file(
                black_box(&workspace),
                black_box(&mut state),
                black_box("note.txt"),
                black_box("alpha"),
                black_box("gamma"),
            );
            black_box(result)
        });
    });
    let _ = std::fs::remove_dir_all(&root);
}

fn todo_merge(c: &mut Criterion) {
    c.bench_function("todo_merge", |b| {
        b.iter(|| {
            let mut store = TodoStore::new();
            store
                .apply(
                    vec![TodoItem {
                        id: "a".into(),
                        content: "first".into(),
                        status: "pending".into(),
                    }],
                    false,
                    Some(0),
                )
                .unwrap();
            let revision = store.apply(
                vec![TodoItem {
                    id: "a".into(),
                    content: black_box("second".into()),
                    status: "done".into(),
                }],
                true,
                Some(1),
            );
            black_box(revision)
        });
    });
}

criterion_group!(benches, patch_apply, todo_merge);
criterion_main!(benches);
