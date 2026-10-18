use std::env;
use std::fs;

use harness_core::curation::demo;

fn main() {
    let root = env::temp_dir().join("ch27-curation-demo");
    let _ = fs::remove_dir_all(&root);
    let line = demo(&root).expect("curate");
    println!("{line}");
    let _ = fs::remove_dir_all(&root);
}
