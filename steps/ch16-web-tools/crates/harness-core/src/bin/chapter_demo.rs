//! Keyless demo. The inherited fallback turn, then one fake search, then the
//! metadata address refused. No API key and no socket.

use harness_core::failure::demo_turn;
use harness_core::web::{is_safe_url, search_result, FakeSearch, StdResolver};

fn main() {
    let turn = demo_turn();
    println!("result: {}", turn.text);
    println!("fallbacks: {}", turn.fallbacks);
    assert_eq!(turn.text, "hello");
    assert_eq!(turn.fallbacks, 1);
    let raw = search_result(&FakeSearch::example(), "rust", Some(1)).expect("search");
    println!("search: {raw}");
    let blocked = is_safe_url("http://169.254.169.254/latest", true, &StdResolver);
    println!("metadata: {}", blocked.expect_err("metadata stays blocked"));
}
