use harness_core::{ChatRequest, Message};

fn main() {
    let messages = vec![
        Message::system("You are a helpful assistant."),
        Message::user("Say hello"),
    ];
    let request = ChatRequest {
        model: "openai/gpt-4o-mini",
        messages: &messages,
    };
    println!("{}", serde_json::to_string(&request).expect("json"));
}
