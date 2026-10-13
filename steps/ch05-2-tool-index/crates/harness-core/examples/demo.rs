use harness_core::{generate, Bm25Index};

fn main() {
    let cards = generate(10_000);
    let index = Bm25Index::build(cards);
    println!("indexed={}", index.len());
    for hit in index.search("rename a file", 5) {
        println!("{:.3} {} ({})", hit.score, hit.name, hit.server);
    }
}
