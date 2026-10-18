//! Keyless demo. A path outside the workspace is refused, an unread write is
//! stale, and a child may not call clarify.

use std::fs;

use harness_core::local_tools::{child_may_call, read_file, write_file, FileState, Workspace};

fn main() {
    let root = std::env::temp_dir().join(format!("ch17-demo-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("root");
    fs::write(root.join("note.txt"), "one").expect("seed");
    let workspace = Workspace::new(&root);
    let escaped = workspace.resolve("../outside.txt");
    println!(
        "escape: {}",
        escaped.expect_err("escape").contains("escapes")
    );
    let mut state = FileState::new();
    let stale = write_file(&workspace, &mut state, "note.txt", "two");
    println!(
        "stale: {}",
        stale.expect_err("stale").contains("stale_write_blocked")
    );
    let text = read_file(&workspace, &mut state, "note.txt").expect("read");
    println!("read: {text}");
    println!(
        "child: {}",
        child_may_call("clarify", 1).expect_err("child")
    );
    let _ = fs::remove_dir_all(&root);
}
