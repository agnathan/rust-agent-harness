use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::recall::{install_fts, session_search, SessionQuery};
use harness_core::wal::Store;
use rusqlite::params;

fn discover_10k(c: &mut Criterion) {
    let path = std::env::temp_dir().join(format!("ch17-bench-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let mut store = Store::open(&path).expect("open");
    store.migrate().expect("migrate");
    {
        let conn = store.connection_mut();
        let tx = conn.transaction().expect("tx");
        tx.execute(
            "INSERT INTO sessions (id, source, parent_id, started_at, title)
             VALUES ('desk', 'user', NULL, 1, NULL)",
            [],
        )
        .expect("session");
        {
            let mut stmt = tx
                .prepare(
                    "INSERT INTO messages (session_id, role, content, ts) VALUES ('desk', 'user', ?1, 1)",
                )
                .expect("stmt");
            for n in 0..10_000 {
                let content = if n == 5_000 {
                    "zephyrtoken in the middle".to_string()
                } else {
                    format!("note {n}")
                };
                stmt.execute(params![content]).expect("row");
            }
        }
        tx.commit().expect("commit");
    }
    install_fts(&mut store).expect("fts");
    let query = SessionQuery {
        query: Some("zephyrtoken".into()),
        session_id: None,
        around_message_id: None,
    };
    c.bench_function("discover_10k", |b| {
        b.iter(|| {
            let hits = session_search(black_box(&store), black_box(&query)).expect("search");
            black_box(hits);
        });
    });
    drop(store);
    let _ = std::fs::remove_file(&path);
}

criterion_group!(benches, discover_10k);
criterion_main!(benches);
