//! Keyless demo. A warm worker accepts one job. No API key and no Docker.

use std::path::PathBuf;

use sandbox::{run_script, ScriptJob, Worker};

fn worker_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("exe");
    path.pop();
    let name = if cfg!(windows) {
        "sandbox-worker.exe"
    } else {
        "sandbox-worker"
    };
    path.push(name);
    path
}

fn main() {
    let mut worker = Worker::spawn(&worker_path()).expect("worker");
    let replies = run_script(
        &mut worker,
        &[ScriptJob {
            line: "ping".into(),
        }],
    )
    .expect("accept");
    println!("reply: {}", replies[0]);
    assert_eq!(replies[0], "ok ping");
    worker.quit().expect("quit");
}
