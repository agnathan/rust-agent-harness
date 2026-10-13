use std::collections::HashSet;

use harness_core::{generate, prepare_card, schema_json, Router};

fn main() {
    let cards = generate(2_000);
    let router = Router::build(cards.clone());
    let server = router.route("rename a file");
    println!("route={server}");
    for hit in router.search("rename a file", 5) {
        println!("{} ({})", hit.name, hit.server);
    }
    let names: HashSet<String> = cards.iter().map(|card| card.name.clone()).collect();
    let prepared: Vec<_> = cards
        .iter()
        .map(|card| prepare_card(card, &names))
        .collect();
    println!("full_bytes={}", schema_json(&cards).len());
    println!("shrunk_bytes={}", schema_json(&prepared).len());
}
