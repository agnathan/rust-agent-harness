use std::env;
use std::fs;

use sessions::{demo_reply, Store};

fn main() {
    let path = env::temp_dir().join("ch15-sessions-demo.db");
    let _ = fs::remove_file(&path);
    let mut store = Store::open(&path).expect("open");
    store.migrate().expect("migrate");
    let reply = demo_reply();
    let rows = store.append_turn("demo", "hi", reply).expect("append");
    let mode = store.journal_mode().expect("journal");
    println!("reply: {reply}");
    println!("rows: {rows} journal: {mode}");
    drop(store);
    let _ = fs::remove_file(&path);
}
