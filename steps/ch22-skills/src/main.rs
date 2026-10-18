use std::env;
use std::fs;

use skills::demo;

fn main() {
    let root = env::temp_dir().join("ch22-skills-demo");
    let _ = fs::remove_dir_all(&root);
    let (name, line) = demo(&root).expect("skills");
    println!("skill: {name}");
    println!("body: {line}");
    let _ = fs::remove_dir_all(&root);
}
