use std::env;
use std::fs;

use skill_write::demo;

fn main() {
    let root = env::temp_dir().join("ch23-skill-writing-demo");
    let _ = fs::remove_dir_all(&root);
    let verdict = demo(&root).expect("write");
    println!("scan: {verdict}");
    println!("created: rename");
    let _ = fs::remove_dir_all(&root);
}
