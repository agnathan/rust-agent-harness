use std::env;

use harness_core::hooks::{MemoryBackend, TurnRecord};
use harness_core::memory_files::{MemoryFiles, Target};

fn main() {
    let root = env::temp_dir().join("ch21-memory-demo");
    let _ = std::fs::remove_dir_all(&root);
    let files = MemoryFiles::open(&root).expect("open");
    let before = files.snapshot();
    files.add(Target::Memory, "likes tea").expect("add");
    let after = files.snapshot();
    files
        .enqueue_turn(&TurnRecord {
            user: "hi",
            assistant: "ok",
        })
        .expect("queue");
    println!("file: {}", files.read(Target::Memory).expect("read"));
    let word = if before.memory.is_empty() && after.memory.is_empty() {
        "frozen"
    } else {
        "changed"
    };
    println!("snapshot: {word}");
}
