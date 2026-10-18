use std::env;
use std::fs;

use harness_core::recall::demo_db;

fn main() {
    let path = env::temp_dir().join("ch20-recall-demo.db");
    let _ = fs::remove_file(&path);
    let hit = demo_db(&path).expect("search");
    println!("mode: discover");
    println!("hit: {hit}");
    let _ = fs::remove_file(&path);
}
