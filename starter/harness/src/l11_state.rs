//! Lesson 11: the turn as an explicit state machine (Hermes's turn_*.py phases, made explicit).

#[derive(Debug, Clone, PartialEq)]
pub enum TurnState {
    Preflight,
    Requesting { attempt: u32 },
    ToolRound { pending: usize },
    Compacting,
    Recovering { attempt: u32 },
    Final(String),
    Failed(String),
    Cancelled,
    BudgetExhausted,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Start,
    ModelText(String),
    ModelToolCalls(usize),
    ToolsDone,
    ContextTooLong,
    Compacted,
    ApiError { retryable: bool },
    RetryReady,
    Cancel,
}

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_iterations: u32,
    pub max_retries: u32,
}

#[derive(Debug)]
pub struct Machine {
    pub state: TurnState,
    pub iterations: u32,
    pub limits: Limits,
}

impl Machine {
    pub fn new(limits: Limits) -> Self {
        Machine {
            state: TurnState::Preflight,
            iterations: 0,
            limits,
        }
    }

    /// Final, Failed, Cancelled, BudgetExhausted are terminal.
    pub fn is_terminal(&self) -> bool {
        // Hint: matches!(..)
        todo!("lesson 11")
    }

    /// Transition table:
    /// - terminal state + any event          -> Err("terminal"), unchanged
    /// - any + Cancel                        -> Cancelled
    /// - Preflight + Start, ToolRound + ToolsDone
    ///       -> if iterations >= max_iterations: BudgetExhausted
    ///          else iterations += 1; Requesting{attempt:0}
    /// - Requesting + ModelText(t)           -> Final(t)
    /// - Requesting + ModelToolCalls(n)      -> ToolRound{pending:n}
    /// - Requesting + ContextTooLong         -> Compacting
    /// - Requesting{a} + ApiError{retryable:true}  -> a < max_retries ? Recovering{attempt:a+1} : Failed("retries exhausted")
    /// - Requesting + ApiError{retryable:false}    -> Failed("non-retryable error")
    /// - Recovering{a} + RetryReady          -> Requesting{attempt:a}
    /// - Compacting + Compacted              -> Requesting{attempt:0}
    /// - anything else                       -> Err(format!("invalid: {state:?} + {event:?}")), unchanged
    pub fn step(&mut self, ev: Event) -> Result<&TurnState, String> {
        // Hint: match (&self.state, ev) { ... } computing the next state, then assign
        todo!("lesson 11")
    }
}
