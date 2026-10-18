//! Keyless demo. A scripted model asks for a tool, the tool finishes, and the
//! model answers. No API key.

use turn_state::{run_script, Phase, ScriptStep};

fn main() {
    let phase =
        run_script(&[ScriptStep::Tools, ScriptStep::Tools, ScriptStep::Text]).expect("script");
    println!("phase: {phase:?}");
    assert_eq!(phase, Phase::Finished);
}
