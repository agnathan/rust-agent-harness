use harness_core::{ChatSession, MockProvider, Outcome};

/// Two mock turns through the session inbox. The mock reports no usage, so the
/// ledger stays empty. This is not a live OpenRouter call.
#[tokio::main]
async fn main() {
    let mock = MockProvider::scripted(["Hi.", "Still here."]);
    let mut session = ChatSession::new(mock, "Be brief.", 4_000);
    for line in ["Hello", "Do you remember me?"] {
        println!("you> {line}");
        match session.handle_line(line).await {
            Outcome::Reply(text) => {
                println!("assistant> {text}");
                for event in &session.last_events {
                    println!("{}", event.report_line());
                }
            }
            other => println!("{other:?}"),
        }
    }
    println!(
        "agent: calls={} cost_known={}",
        session.usage.calls, session.usage.cost_known
    );
}
