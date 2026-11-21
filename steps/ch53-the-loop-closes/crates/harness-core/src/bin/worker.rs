//! Long-lived child. One line in, one `ok` line out. `sleep-ms N` waits first.

use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let stdin = io::stdin();
    let mut out = io::stdout();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or_default();
        if line == "quit" {
            break;
        }
        if let Some(raw) = line.strip_prefix("sleep-ms ") {
            let millis = raw.parse::<u64>().unwrap_or(0);
            thread::sleep(Duration::from_millis(millis));
            let _ = writeln!(out, "ok slept");
        } else {
            let _ = writeln!(out, "ok {line}");
        }
        let _ = out.flush();
    }
}
