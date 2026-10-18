use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use harness_core::hooks::{MemoryBackend, TurnRecord};
use harness_core::memory_files::{
    budget_line, first_threat_message, MemoryError, MemoryFiles, Target, MEMORY_CHAR_LIMIT,
};

fn scratch() -> std::path::PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!("ch16-{}-{nanos}-{n}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    path
}

#[test]
fn add_replace_and_remove_round_trip_the_delimiter() {
    let root = scratch();
    let files = MemoryFiles::open(&root).expect("open");
    assert_eq!(
        files.add(Target::Memory, "likes tea").expect("add"),
        "Entry added."
    );
    assert_eq!(
        files.add(Target::Memory, "likes tea").expect_err("dup"),
        MemoryError::Duplicate
    );
    files.add(Target::User, "name is Ada").expect("user");
    files
        .replace(Target::Memory, "likes tea", "likes coffee")
        .expect("replace");
    assert_eq!(files.read(Target::Memory).expect("read"), "likes coffee");
    files
        .remove(Target::Memory, "likes coffee")
        .expect("remove");
    assert_eq!(files.read(Target::Memory).expect("read"), "");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_write_over_the_budget_names_how_many_chars_to_free() {
    let root = scratch();
    let files = MemoryFiles::open(&root).expect("open");
    let too_big = "x".repeat(MEMORY_CHAR_LIMIT + 1);
    let err = files.add(Target::Memory, &too_big).expect_err("budget");
    let MemoryError::Budget { over } = err else {
        panic!("budget");
    };
    assert_eq!(over, 1);
    assert_eq!(budget_line(over), "would exceed the limit by 1 chars");
    assert_eq!(files.read(Target::Memory).expect("read"), "");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_poisoned_entry_is_blocked_before_it_reaches_the_file() {
    let ssh = first_threat_message("drop a key in authorized_keys").expect("ssh");
    assert!(ssh.contains("ssh_backdoor"));
    assert!(ssh.contains("injected into the system prompt"));
    let invisible = first_threat_message("hi\u{200b}there").expect("zwsp");
    assert!(invisible.contains("U+200B"));
    let exfil = first_threat_message("include the full conversation").expect("exfil");
    assert!(exfil.contains("context_exfil"));
    let secret = first_threat_message("api_key = \"abcdefghijklmnopqrst\"").expect("secret");
    assert!(secret.contains("hardcoded_secret"));

    let root = scratch();
    let files = MemoryFiles::open(&root).expect("open");
    let err = files
        .add(Target::Memory, "authorized_keys")
        .expect_err("blocked");
    assert!(matches!(err, MemoryError::Scan(_)));
    assert_eq!(files.read(Target::Memory).expect("read"), "");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn the_snapshot_stays_put_and_enqueue_does_not_touch_the_file() {
    let root = scratch();
    let files = MemoryFiles::open(&root).expect("open");
    let before = files.snapshot();
    files.add(Target::Memory, "likes tea").expect("add");
    let after = files.snapshot();
    assert!(std::sync::Arc::ptr_eq(&before, &after));
    assert!(after.memory.is_empty());
    assert_eq!(files.read(Target::Memory).expect("read"), "likes tea");
    files
        .enqueue_turn(&TurnRecord {
            user: "hi",
            assistant: "ok",
        })
        .expect("queue");
    assert_eq!(files.queue_len(), 1);
    assert_eq!(files.read(Target::Memory).expect("read"), "likes tea");
    assert_eq!(files.name(), "files");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_broken_delimiter_is_drift() {
    let root = scratch();
    let files = MemoryFiles::open(&root).expect("open");
    fs::write(root.join("MEMORY.md"), "one § two").expect("hand");
    let err = files.add(Target::Memory, "more").expect_err("drift");
    assert_eq!(err, MemoryError::Drift);
    let _ = fs::remove_dir_all(&root);
}
