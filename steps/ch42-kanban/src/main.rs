use std::env;
use std::path::Path;
use std::process::ExitCode;

use kanban::{demo, transact, Op, TxError};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        println!("{}", demo());
        return ExitCode::SUCCESS;
    };
    if command != "claim" {
        eprintln!("usage: ch42-kanban [claim <dir> <id> <owner>]");
        return ExitCode::from(2);
    }
    let Some(dir) = args.next() else {
        return ExitCode::from(2);
    };
    let Some(id) = args.next() else {
        return ExitCode::from(2);
    };
    let Some(owner) = args.next() else {
        return ExitCode::from(2);
    };
    match transact(
        Path::new(&dir),
        &id,
        Op::Claim {
            owner: owner.clone(),
        },
    ) {
        Ok(_) => {
            println!("claimed");
            ExitCode::SUCCESS
        }
        Err(TxError::Lost | TxError::Parents | TxError::Paused | TxError::Busy) => {
            println!("lost");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::from(1)
        }
    }
}
