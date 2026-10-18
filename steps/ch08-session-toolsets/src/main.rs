//! Keyless demo of the three catalog arms. No API key and no socket.

use gw_session::{run_rename_task, Arm};

fn main() {
    for (label, arm) in [
        ("every", Arm::Every),
        ("static", Arm::Static),
        ("dynamic", Arm::Dynamic),
    ] {
        let report = run_rename_task(arm, 2_000);
        println!(
            "{label}: success={} schema_tokens={} call_tokens={} result_tokens={}",
            report.success, report.schema_tokens, report.call_tokens, report.result_tokens
        );
    }
}
