use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use recall::{insert_message, install_fts, session_search, SessionQuery};
use sessions::Store;

struct Temp {
    path: PathBuf,
    store: Store,
}

impl Temp {
    fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let path = std::env::temp_dir().join(format!("ch17-{}-{nanos}-{n}.db", std::process::id()));
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
fn discover_hides_a_subagent_and_demotes_cron() {
    let mut db = Temp::new();
    insert_message(&mut db.store, "child", "subagent", "zephyrtoken hidden").expect("child");
    insert_message(&mut db.store, "nightly", "cron", "zephyrtoken cron").expect("cron");
    insert_message(&mut db.store, "desk", "user", "zephyrtoken desk").expect("user");
    install_fts(&mut db.store).expect("fts");
    let hits = session_search(
        &db.store,
        &SessionQuery {
            query: Some("zephyrtoken".into()),
            session_id: None,
            around_message_id: None,
        },
    )
    .expect("discover");
    assert_eq!(hits[0].session_id, "desk");
    assert!(hits.iter().all(|hit| hit.session_id != "child"));
    assert_eq!(hits[1].session_id, "nightly");
    let path = db.path.clone();
    drop(db);
    cleanup(&path);
}

#[test]
fn scroll_read_and_browse_are_the_other_three_shapes() {
    let mut db = Temp::new();
    let anchor = insert_message(&mut db.store, "desk", "user", "alpha").expect("a");
    insert_message(&mut db.store, "desk", "user", "beta").expect("b");
    insert_message(&mut db.store, "nightly", "cron", "gamma").expect("c");
    insert_message(&mut db.store, "child", "subagent", "delta").expect("d");
    install_fts(&mut db.store).expect("fts");

    let scrolled = session_search(
        &db.store,
        &SessionQuery {
            query: None,
            session_id: Some("desk".into()),
            around_message_id: Some(anchor),
        },
    )
    .expect("scroll");
    assert!(scrolled.iter().any(|hit| hit.content == "alpha"));
    assert!(scrolled.iter().any(|hit| hit.content == "beta"));

    let read = session_search(
        &db.store,
        &SessionQuery {
            query: None,
            session_id: Some("desk".into()),
            around_message_id: None,
        },
    )
    .expect("read");
    assert_eq!(read.len(), 2);

    let hidden = session_search(
        &db.store,
        &SessionQuery {
            query: None,
            session_id: Some("child".into()),
            around_message_id: None,
        },
    )
    .expect("hidden");
    assert!(hidden.is_empty());

    let browsed = session_search(
        &db.store,
        &SessionQuery {
            query: None,
            session_id: None,
            around_message_id: None,
        },
    )
    .expect("browse");
    assert_eq!(
        browsed.first().map(|hit| hit.session_id.as_str()),
        Some("desk")
    );
    assert!(browsed.iter().any(|hit| hit.session_id == "nightly"));
    assert!(browsed.iter().all(|hit| hit.session_id != "child"));
    let path = db.path.clone();
    drop(db);
    cleanup(&path);
}

#[test]
fn the_demo_hit_is_the_user_session() {
    let path = std::env::temp_dir().join(format!("ch17-demo-test-{}.db", std::process::id()));
    let _ = fs::remove_file(&path);
    let hit = recall::demo_db(&path).expect("demo");
    assert_eq!(hit, "desk");
    cleanup(&path);
}
