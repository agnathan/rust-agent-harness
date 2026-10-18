//! Keyless demo. Registers `echo` and calls it. No socket and no API key.

use gw_minimal::Gateway;
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut gateway = Gateway::new();
    gateway
        .register("echo", "Return the text argument.")
        .expect("echo");
    let view = gateway.session_tools("demo");
    let names: Vec<&str> = view.tools.iter().map(|tool| tool.name.as_str()).collect();
    let text = gateway
        .call("echo", &json!({"text": "hello"}))
        .await
        .expect("call");
    println!("tools: {}", names.join(", "));
    println!("echo: {text}");
    println!("accept_bound: {}", gateway.gate().accept_bound());
}
