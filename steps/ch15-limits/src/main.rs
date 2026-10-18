use gw_limits::{CallError, CallOpts, FailureKind, LimitedGateway, Outcome, Script};

fn main() {
    let gateway = LimitedGateway::new();
    gateway.set_idempotent("echo", true);
    let script = Script::new(vec![
        Outcome::Fail(FailureKind::Transport),
        Outcome::Ok("ok".into()),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
        Outcome::Fail(FailureKind::Status(503)),
    ]);
    let result = gateway
        .dispatch("echo", "", CallOpts::default(), &script)
        .expect("echo");
    let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    let _ = gateway.dispatch("search", "", CallOpts::default(), &script);
    let stopped = gateway.dispatch("search", "", CallOpts::default(), &script);
    println!("result: {result}");
    let word = match stopped {
        Err(CallError::OpenBreaker) => "open",
        other => panic!("expected an open breaker, got {other:?}"),
    };
    println!("breaker: {word}");
}
