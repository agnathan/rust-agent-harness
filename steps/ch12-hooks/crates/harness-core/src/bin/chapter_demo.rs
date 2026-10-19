//! Keyless demo. A hook blocks `rm`. The fake model does not run the next call.
//! No API key.

use std::sync::Arc;

use harness_core::hooks::{run_script, BlockName, ScriptedCall, ToolHook};
use serde_json::json;

fn main() {
    let hook: Arc<dyn ToolHook> = Arc::new(BlockName {
        name: "rm".into(),
        message: "BLOCKED: hook refused rm".into(),
    });
    let calls = [
        ScriptedCall {
            name: "rm".into(),
            args: json!({"path": "/"}),
        },
        ScriptedCall {
            name: "echo".into(),
            args: json!({"text": "hello"}),
        },
    ];
    let outcomes = run_script(std::slice::from_ref(&hook), &calls);
    println!("result: {}", outcomes[0].result);
    println!("stopped: {}", outcomes.len());
    assert_eq!(outcomes.len(), 1);
}
