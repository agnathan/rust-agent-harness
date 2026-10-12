//! Lesson 04: Result, custom error enums, Display, and the ? operator.
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Command {
    Stop,
    New,
    Help,
    Steer(String),
    Model(String),
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    NotACommand,
    Unknown(String),
    MissingArg(&'static str),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hint: write!(f, ...) per variant: "not a command", "unknown command: /x", "missing argument: text"
        todo!("lesson 04")
    }
}

impl std::error::Error for CommandError {}

/// Parse "/stop", "/new", "/help", "/steer <text>", "/model <name>".
/// Leading/trailing whitespace is ignored. The argument is everything after the first space, trimmed.
pub fn parse_command(input: &str) -> Result<Command, CommandError> {
    // Hint: strip_prefix('/') -> split_once(' ') -> match on the name
    todo!("lesson 04")
}

#[derive(Debug, PartialEq)]
pub enum BudgetError {
    NotANumber(ParseIntError),
    Zero,
}

impl From<ParseIntError> for BudgetError {
    fn from(e: ParseIntError) -> Self {
        // Hint: wrap it
        todo!("lesson 04")
    }
}

/// Parse an iteration budget like "500". Use `?` on str::parse (that's why From exists). 0 is an error.
pub fn parse_budget(s: &str) -> Result<u32, BudgetError> {
    // Hint: let n: u32 = s.trim().parse()?;
    todo!("lesson 04")
}
