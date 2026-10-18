use harness_core::{finish_wrap, run_script, step, Phase, ScriptStep, TurnEvent, WrapKind};

#[test]
fn every_pair_is_in_the_table() {
    let mut legal = 0;
    let mut illegal = 0;
    for from in Phase::all() {
        for event in TurnEvent::all() {
            match step(*from, *event) {
                Ok(next) => {
                    legal += 1;
                    if from.is_terminal() {
                        panic!("terminal {from:?} accepted {event:?} -> {next:?}");
                    }
                }
                Err(err) => {
                    illegal += 1;
                    assert_eq!(err.from, *from);
                    assert_eq!(err.event, *event);
                }
            }
        }
    }
    assert_eq!(legal + illegal, Phase::all().len() * TurnEvent::all().len());
    assert!(legal > 0 && illegal > 0);
}

#[test]
fn a_tool_then_text_finishes() {
    let phase = run_script(&[ScriptStep::Tools, ScriptStep::Tools, ScriptStep::Text]).unwrap();
    assert_eq!(phase, Phase::Finished);
}

#[test]
fn soft_stop_and_budget_leave_wrapping_up_differently() {
    let soft = run_script(&[
        ScriptStep::Tools,
        ScriptStep::Tools,
        ScriptStep::SoftStop,
        ScriptStep::Text,
    ])
    .unwrap();
    assert_eq!(soft, Phase::SoftStopped);
    let budget = run_script(&[
        ScriptStep::Tools,
        ScriptStep::Tools,
        ScriptStep::Budget,
        ScriptStep::Text,
    ])
    .unwrap();
    assert_eq!(budget, Phase::Budget);
    assert_eq!(finish_wrap(WrapKind::Soft), Phase::SoftStopped);
    let partial = run_script(&[
        ScriptStep::Tools,
        ScriptStep::Tools,
        ScriptStep::Budget,
        ScriptStep::Tools,
    ])
    .unwrap();
    assert_eq!(partial, Phase::Partial);
}

#[test]
fn stall_cancel_and_invalid_are_terminal() {
    assert_eq!(
        run_script(&[ScriptStep::Tools, ScriptStep::Stall]).unwrap(),
        Phase::Stalled
    );
    assert_eq!(
        run_script(&[ScriptStep::HardCancel]).unwrap(),
        Phase::Cancelled
    );
    assert_eq!(
        run_script(&[ScriptStep::Invalid]).unwrap(),
        Phase::InvalidToolCalls
    );
    assert_eq!(
        run_script(&[ScriptStep::Truncated]).unwrap(),
        Phase::Partial
    );
}

#[test]
fn dispatch_cannot_start() {
    let err = step(Phase::Dispatching, TurnEvent::Start).unwrap_err();
    assert_eq!(err.from, Phase::Dispatching);
}
