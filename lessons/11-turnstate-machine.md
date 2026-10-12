# Lesson 11: The turn as an explicit state machine

**Goal:** Replace "the state is whatever the code happens to be doing" with an enum and a transition function.

## Concepts
- **Enums with data** (`Requesting { attempt }`) make invalid states impossible to represent.
- **Matching on a tuple.** `match (&self.state, ev)` lets you write the whole transition table in one place.
- **`matches!`** is a quick boolean pattern test.
- **Pure transitions.** The machine only decides the *next state*; something else performs the I/O. That makes every edge case testable without a network, and states can be logged or saved to resume later.

## How Hermes does it
- Hermes splits the turn into roughly 35 phase modules: `agent/turn_preflight.py`, `turn_api_call.py`, `turn_api_error.py`, `turn_tool_round.py`, `turn_context_compaction.py`, `turn_recovery*.py`, `turn_stop_gates.py`, `turn_final_response.py`.
- The *states* are implicit: attributes on the agent plus whichever phase is currently running.
- Errors go through `FailoverReason` to a recovery choice (retry, rotate, compress, abort).
- Our `TurnState` makes all of that explicit (see the deep dive, §A1 and §B2).

## Tasks
1. `is_terminal` using `matches!`.
2. `step`. Copy the table from the doc comment into a `match`, checking `is_terminal` first.
   - Hint: compute `next`, then `self.state = next; Ok(&self.state)`.
   - Invalid transitions: `(s, e) => return Err(format!("invalid: {s:?} + {e:?}"))`.

## Milestone
`cargo test --test l11_state` is green.

## Exercises
- Write `drive(machine, model, tools)`, which loops: inspect the state → do the I/O → feed the resulting event back. This is lesson 07's loop rebuilt on the state machine.
- Add `Event::Steer(String)`. Which states should accept it?
- Derive `serde::Serialize` on `TurnState` and log every transition as JSON.
