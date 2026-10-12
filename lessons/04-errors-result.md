# Lesson 04: Errors with Result, custom error types and `?`

**Goal:** Parse slash commands and budgets without panicking.

## Concepts
- **`Result<T, E>`** is either `Ok(T)` or `Err(E)`. Rust has no exceptions, so errors are values you have to handle.
- **`?`** returns early with the error, converting it via `From` if needed. It's the main tool for clean error code.
- **Custom error enums.** Implement `std::fmt::Display` (the human-readable message) and `std::error::Error` (the standard error trait).
- **`impl From<ParseIntError> for BudgetError`** is what lets `s.parse()?` turn a parse error into your own error type automatically.
- Use **`unwrap()`/`expect()`** only in tests, or where a failure really is a bug.
- Later you may use the `thiserror` crate, which writes `Display` and `From` for you. Do it by hand once first.

## How Hermes does it
- The gateway intercepts `/stop`, `/new`, `/queue` and similar commands in `gateway/run_busy.py`, using a handler table (`_command_handler_table`) rather than if-chains.
- `agent/error_classifier.py::FailoverReason` is an error enum that decides how to recover: rotate key, back off, compress context, and so on. Our enums are a small-scale version of that idea.

## Tasks
1. Implement `Display for CommandError`. The exact strings are in the comment.
2. `parse_command`:
   - Hint: `let body = s.strip_prefix('/').ok_or(CommandError::NotACommand)?;`
   - Then `body.split_once(' ')`.
   - Match guards help: `"steer" if arg.is_empty() => Err(..)`.
3. Implement `From<ParseIntError>`, then `parse_budget` using `?`.

## Milestone
`cargo test --test l04_errors` is green.

## Exercises
- Add `/queue <text>`.
- Rewrite the errors with `thiserror` (`cargo add thiserror`) and compare how much code it saves.
