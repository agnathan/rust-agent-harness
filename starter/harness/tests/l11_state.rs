use harness::l11_state::*;
fn m() -> Machine {
    Machine::new(Limits {
        max_iterations: 2,
        max_retries: 1,
    })
}
#[test]
fn happy_path() {
    let mut s = m();
    assert_eq!(
        s.step(Event::Start).unwrap(),
        &TurnState::Requesting { attempt: 0 }
    );
    assert_eq!(
        s.step(Event::ModelToolCalls(2)).unwrap(),
        &TurnState::ToolRound { pending: 2 }
    );
    assert_eq!(
        s.step(Event::ToolsDone).unwrap(),
        &TurnState::Requesting { attempt: 0 }
    );
    assert_eq!(
        s.step(Event::ModelText("ok".into())).unwrap(),
        &TurnState::Final("ok".into())
    );
    assert!(s.is_terminal());
    assert_eq!(s.iterations, 2);
    assert!(s.step(Event::Start).is_err());
}
#[test]
fn budget() {
    let mut s = m();
    s.step(Event::Start).unwrap();
    s.step(Event::ModelToolCalls(1)).unwrap();
    s.step(Event::ToolsDone).unwrap();
    s.step(Event::ModelToolCalls(1)).unwrap();
    assert_eq!(
        s.step(Event::ToolsDone).unwrap(),
        &TurnState::BudgetExhausted
    );
}
#[test]
fn retries_and_compaction() {
    let mut s = m();
    s.step(Event::Start).unwrap();
    assert_eq!(
        s.step(Event::ApiError { retryable: true }).unwrap(),
        &TurnState::Recovering { attempt: 1 }
    );
    assert_eq!(
        s.step(Event::RetryReady).unwrap(),
        &TurnState::Requesting { attempt: 1 }
    );
    assert_eq!(
        s.step(Event::ContextTooLong).unwrap(),
        &TurnState::Compacting
    );
    assert_eq!(
        s.step(Event::Compacted).unwrap(),
        &TurnState::Requesting { attempt: 0 }
    );
    s.step(Event::ApiError { retryable: true }).unwrap();
    s.step(Event::RetryReady).unwrap();
    assert_eq!(
        s.step(Event::ApiError { retryable: true }).unwrap(),
        &TurnState::Failed("retries exhausted".into())
    );
    let mut s2 = m();
    s2.step(Event::Start).unwrap();
    assert_eq!(
        s2.step(Event::ApiError { retryable: false }).unwrap(),
        &TurnState::Failed("non-retryable error".into())
    );
}
#[test]
fn cancel_and_invalid() {
    let mut s = m();
    assert!(s.step(Event::ToolsDone).is_err());
    assert_eq!(s.state, TurnState::Preflight);
    s.step(Event::Start).unwrap();
    assert_eq!(s.step(Event::Cancel).unwrap(), &TurnState::Cancelled);
}
