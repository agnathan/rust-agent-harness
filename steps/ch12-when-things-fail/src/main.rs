//! Keyless demo. Two expensive empty replies fall back to a second fake model.
//! No API key.

use failures::demo_turn;

fn main() {
    let turn = demo_turn();
    println!("result: {}", turn.text);
    println!("fallbacks: {}", turn.fallbacks);
    assert_eq!(turn.text, "hello");
    assert_eq!(turn.fallbacks, 1);
}
