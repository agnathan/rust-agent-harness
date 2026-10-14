use harness_core::{Message, MockProvider, Provider};

#[tokio::main]
async fn main() {
    let mock = MockProvider::new("(mock) I heard you.");
    let events = mock
        .complete_events(&[Message::user("Say hello")])
        .await
        .expect("mock");
    for event in &events {
        println!("{}", event.report_line());
    }
}
