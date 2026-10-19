//! Keyless demo. A fake model proposes `rm -rf /`. The pipeline denies it.
//! No API key.

use harness_core::approval::{run_script, Policy, ScriptCall};

fn main() {
    let verdicts = run_script(
        &Policy::manual(),
        &[
            ScriptCall {
                command: "rm -rf /".to_string(),
            },
            ScriptCall {
                command: "echo hello".to_string(),
            },
        ],
    );
    println!("verdict: {}", verdicts[0].label());
    println!("stopped: {}", verdicts.len());
    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].label(), "deny");
}
