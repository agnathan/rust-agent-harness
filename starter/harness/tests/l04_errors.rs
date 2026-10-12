use harness::l04_errors::*;
#[test]
fn commands() {
    assert_eq!(parse_command(" /stop "), Ok(Command::Stop));
    assert_eq!(parse_command("/new"), Ok(Command::New));
    assert_eq!(
        parse_command("/steer  focus on tests "),
        Ok(Command::Steer("focus on tests".into()))
    );
    assert_eq!(
        parse_command("/model gpt-x"),
        Ok(Command::Model("gpt-x".into()))
    );
}
#[test]
fn command_errors() {
    assert_eq!(parse_command("hello"), Err(CommandError::NotACommand));
    assert_eq!(
        parse_command("/dance"),
        Err(CommandError::Unknown("dance".into()))
    );
    assert_eq!(
        parse_command("/steer"),
        Err(CommandError::MissingArg("text"))
    );
    assert_eq!(
        parse_command("/dance").unwrap_err().to_string(),
        "unknown command: /dance"
    );
    assert_eq!(
        CommandError::MissingArg("name").to_string(),
        "missing argument: name"
    );
}
#[test]
fn budget() {
    assert_eq!(parse_budget(" 500 "), Ok(500));
    assert_eq!(parse_budget("0"), Err(BudgetError::Zero));
    assert!(matches!(
        parse_budget("lots"),
        Err(BudgetError::NotANumber(_))
    ));
}
