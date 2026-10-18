use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use harness_core::wal::Store;

struct TempDb {
    path: PathBuf,
    store: Store,
}

impl TempDb {
    fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let path = std::env::temp_dir().join(format!("ch15-{}-{nanos}-{n}.db", std::process::id()));
        let store = Store::open(&path).expect("open");
        store.migrate().expect("migrate");
        Self { path, store }
    }
}

fn cleanup(path: &std::path::Path) {
    let _ = fs::remove_file(path);
    let mut wal = path.as_os_str().to_os_string();
    wal.push("-wal");
    let _ = fs::remove_file(wal);
    let mut shm = path.as_os_str().to_os_string();
    shm.push("-shm");
    let _ = fs::remove_file(shm);
}

#[test]
fn a_turn_appends_two_rows_and_queues_fts_without_a_virtual_table() {
    let mut db = TempDb::new();
    assert_eq!(db.store.journal_mode().expect("mode"), "wal");
    let rows = db.store.append_turn("s1", "hi", "pong").expect("append");
    assert_eq!(rows, 2);
    assert_eq!(
        db.store.messages("s1").expect("read"),
        vec![
            ("user".into(), "hi".into()),
            ("assistant".into(), "pong".into())
        ]
    );
    assert_eq!(db.store.pending_fts(), 2);
    let tables: i64 = db
        .store
        .connection()
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name = 'messages_fts_trigram'",
            [],
            |row| row.get(0),
        )
        .expect("master");
    assert_eq!(tables, 0);
    let path = db.path.clone();
    drop(db);
    cleanup(&path);
}

#[test]
fn a_second_turn_resumes_the_same_session() {
    let mut db = TempDb::new();
    db.store.append_turn("s1", "one", "a").expect("first");
    db.store.append_turn("s1", "two", "b").expect("second");
    let messages = db.store.messages("s1").expect("read");
    assert_eq!(messages.len(), 4);
    assert_eq!(messages[2].1, "two");
    let sessions: i64 = db
        .store
        .connection()
        .query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))
        .expect("sessions");
    assert_eq!(sessions, 1);
    let path = db.path.clone();
    drop(db);
    cleanup(&path);
}

#[tokio::test(flavor = "multi_thread")]
async fn chapter_9_saved_and_stress_evals_still_pass() {
    let saved = harness_core::run_saved_evals().await;
    assert!(
        saved.iter().all(|score| score.passed),
        "saved evals: {saved:?}"
    );
    let stress = harness_core::run_stress_evals().await;
    assert!(
        stress.iter().all(|score| score.passed),
        "stress evals: {stress:?}"
    );
}
