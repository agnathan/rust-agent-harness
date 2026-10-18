//! Book chapter 10. One phase enum and the only transitions a turn is allowed
//! to make. Chapter 9's `run_loop` calls `Driver::advance` at each observation.
//! `run_script` is still the fake model: a list of `ScriptStep` values, no HTTP.

/// Where a turn is. Terminal phases have no exit except staying there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Idle,
    CallingModel,
    Dispatching,
    WrappingUp,
    Finished,
    Cancelled,
    SoftStopped,
    Budget,
    Partial,
    InvalidToolCalls,
    Stalled,
}

impl Phase {
    pub fn is_terminal(self) -> bool {
        !matches!(
            self,
            Phase::Idle | Phase::CallingModel | Phase::Dispatching | Phase::WrappingUp
        )
    }

    pub fn all() -> &'static [Phase] {
        &[
            Phase::Idle,
            Phase::CallingModel,
            Phase::Dispatching,
            Phase::WrappingUp,
            Phase::Finished,
            Phase::Cancelled,
            Phase::SoftStopped,
            Phase::Budget,
            Phase::Partial,
            Phase::InvalidToolCalls,
            Phase::Stalled,
        ]
    }
}

/// What the loop observed. The chapter 9 loop is still the thing that detects
/// these. This chapter only accepts or refuses the move.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnEvent {
    Start,
    ModelText,
    ModelToolCalls,
    ToolsDone,
    HardCancel,
    SoftStop,
    BudgetHit,
    Truncated,
    InvalidStrikeout,
    Stall,
    WrapUpText,
    WrapUpToolCalls,
    /// The provider batch had no `Done` event. The turn ends `Budget` and does
    /// not start a wrap-up.
    NoDone,
    /// The budget is already spent and the iteration counter is zero, but the
    /// turn has already entered `CallingModel` (an `execute_code` refund can
    /// do that). This is not `BudgetHit`, which would enter `WrappingUp`.
    BudgetAbort,
    /// A soft-stop wrap-up asked for tools, or returned no text. Chapter 9
    /// does not run those tools and the stop reason stays soft stop, so this
    /// is not `WrapUpToolCalls` (that row is the budget partial).
    SoftWrapIgnoredTools,
}

impl TurnEvent {
    pub fn all() -> &'static [TurnEvent] {
        &[
            TurnEvent::Start,
            TurnEvent::ModelText,
            TurnEvent::ModelToolCalls,
            TurnEvent::ToolsDone,
            TurnEvent::HardCancel,
            TurnEvent::SoftStop,
            TurnEvent::BudgetHit,
            TurnEvent::Truncated,
            TurnEvent::InvalidStrikeout,
            TurnEvent::Stall,
            TurnEvent::WrapUpText,
            TurnEvent::WrapUpToolCalls,
            TurnEvent::NoDone,
            TurnEvent::BudgetAbort,
            TurnEvent::SoftWrapIgnoredTools,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Illegal {
    pub from: Phase,
    pub event: TurnEvent,
}

/// The transition table. `Ok` is a legal next phase. `Err` is a move the loop
/// must not make.
pub fn step(from: Phase, event: TurnEvent) -> Result<Phase, Illegal> {
    let next = match (from, event) {
        (Phase::Idle, TurnEvent::Start) => Phase::CallingModel,
        (Phase::Idle, TurnEvent::BudgetHit) => Phase::Budget,
        (Phase::CallingModel, TurnEvent::ModelText) => Phase::Finished,
        (Phase::CallingModel, TurnEvent::ModelToolCalls) => Phase::Dispatching,
        (Phase::CallingModel, TurnEvent::HardCancel) => Phase::Cancelled,
        (Phase::CallingModel, TurnEvent::SoftStop) => Phase::WrappingUp,
        (Phase::CallingModel, TurnEvent::BudgetHit) => Phase::WrappingUp,
        (Phase::CallingModel, TurnEvent::Truncated) => Phase::Partial,
        (Phase::CallingModel, TurnEvent::InvalidStrikeout) => Phase::InvalidToolCalls,
        (Phase::Dispatching, TurnEvent::ToolsDone) => Phase::CallingModel,
        (Phase::Dispatching, TurnEvent::HardCancel) => Phase::Cancelled,
        (Phase::Dispatching, TurnEvent::Stall) => Phase::Stalled,
        (Phase::WrappingUp, TurnEvent::WrapUpText) => match_wrap_text(from),
        (Phase::WrappingUp, TurnEvent::WrapUpToolCalls) => Phase::Partial,
        (Phase::WrappingUp, TurnEvent::HardCancel) => Phase::Cancelled,
        (Phase::CallingModel, TurnEvent::NoDone) => Phase::Budget,
        (Phase::WrappingUp, TurnEvent::NoDone) => Phase::Budget,
        (Phase::CallingModel, TurnEvent::BudgetAbort) => Phase::Budget,
        (Phase::WrappingUp, TurnEvent::SoftWrapIgnoredTools) => Phase::SoftStopped,
        _ => {
            return Err(Illegal { from, event });
        }
    };
    Ok(next)
}

fn match_wrap_text(from: Phase) -> Phase {
    // A wrap-up that answers in text ends the turn. Soft stop and budget both
    // enter WrappingUp; the caller records which one it was. The table's exit
    // for a text wrap-up is Finished when the caller uses `finish_wrap`.
    let _ = from;
    Phase::Finished
}

/// Soft stop's wrap-up text is `SoftStopped`. A budget wrap-up's text is `Budget`.
pub fn finish_wrap(kind: WrapKind) -> Phase {
    match kind {
        WrapKind::Soft => Phase::SoftStopped,
        WrapKind::Budget => Phase::Budget,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapKind {
    Soft,
    Budget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptStep {
    Text,
    Tools,
    HardCancel,
    SoftStop,
    Budget,
    Stall,
    Truncated,
    Invalid,
}

/// A fake model. Each script step is one observation. No network and no key.
pub fn run_script(script: &[ScriptStep]) -> Result<Phase, Illegal> {
    let mut phase = step(Phase::Idle, TurnEvent::Start)?;
    let mut wrap = None;
    for item in script {
        let event = match (phase, item) {
            (Phase::CallingModel, ScriptStep::Text) => TurnEvent::ModelText,
            (Phase::CallingModel, ScriptStep::Tools) => TurnEvent::ModelToolCalls,
            (Phase::CallingModel, ScriptStep::HardCancel) => TurnEvent::HardCancel,
            (Phase::CallingModel, ScriptStep::SoftStop) => {
                wrap = Some(WrapKind::Soft);
                TurnEvent::SoftStop
            }
            (Phase::CallingModel, ScriptStep::Budget) => {
                wrap = Some(WrapKind::Budget);
                TurnEvent::BudgetHit
            }
            (Phase::CallingModel, ScriptStep::Truncated) => TurnEvent::Truncated,
            (Phase::CallingModel, ScriptStep::Invalid) => TurnEvent::InvalidStrikeout,
            (Phase::Dispatching, ScriptStep::Tools) => TurnEvent::ToolsDone,
            (Phase::Dispatching, ScriptStep::HardCancel) => TurnEvent::HardCancel,
            (Phase::Dispatching, ScriptStep::Stall) => TurnEvent::Stall,
            (Phase::WrappingUp, ScriptStep::Text) => TurnEvent::WrapUpText,
            (Phase::WrappingUp, ScriptStep::Tools) => TurnEvent::WrapUpToolCalls,
            (Phase::WrappingUp, ScriptStep::HardCancel) => TurnEvent::HardCancel,
            _ => {
                return Err(Illegal {
                    from: phase,
                    event: TurnEvent::Start,
                });
            }
        };
        phase = step(phase, event)?;
        if phase == Phase::Finished {
            if let Some(kind) = wrap {
                phase = finish_wrap(kind);
            }
        }
        if phase.is_terminal() {
            return Ok(phase);
        }
    }
    Ok(phase)
}

/// Walks `step` for one real turn. `run_loop` holds one of these.
#[derive(Debug)]
pub struct Driver {
    phase: Phase,
    wrap: Option<WrapKind>,
}

impl Driver {
    pub fn new() -> Self {
        Self {
            phase: Phase::Idle,
            wrap: None,
        }
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Record one observation. A text wrap-up then applies `finish_wrap` so a
    /// soft stop and a budget wrap-up do not both land on `Finished`.
    pub fn advance(&mut self, event: TurnEvent) -> Result<Phase, Illegal> {
        if event == TurnEvent::SoftStop {
            self.wrap = Some(WrapKind::Soft);
        } else if event == TurnEvent::BudgetHit && self.phase == Phase::CallingModel {
            self.wrap = Some(WrapKind::Budget);
        }
        let next = step(self.phase, event)?;
        self.phase = next;
        if self.phase == Phase::Finished {
            if let Some(kind) = self.wrap.take() {
                self.phase = finish_wrap(kind);
            }
        }
        Ok(self.phase)
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
