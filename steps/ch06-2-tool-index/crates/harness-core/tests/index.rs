use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use harness_core::{generate, Bm25Index, Embedder, HashedBag, ToolIndex};

#[test]
fn rename_file_hits_the_files_server() {
    let index = Bm25Index::build(generate(512));
    let hits = index.search("rename a file", 5);
    assert!(!hits.is_empty());
    assert!(
        hits.iter()
            .any(|hit| hit.server == "files" && hit.name.contains("rename")),
        "{hits:?}"
    );
    assert!(hits.len() <= 5);
}

#[test]
fn check_fn_drops_a_server() {
    let cards = generate(80);
    let index = Bm25Index::build_available(cards, |card| card.server != "shell");
    assert!(index.len() < 80);
    assert!(index.cards().iter().all(|card| card.server != "shell"));
    let hits = index.search("run a command", 8);
    assert!(hits.iter().all(|hit| hit.server != "shell"));
}

#[test]
fn empty_query_and_zero_k_return_nothing() {
    let index = Bm25Index::build(generate(16));
    assert!(index.search("", 5).is_empty());
    assert!(index.search("rename a file", 0).is_empty());
}

struct Counting {
    hits: Arc<AtomicUsize>,
}

impl Embedder for Counting {
    fn embed(&self, text: &str) -> Vec<f32> {
        self.hits.fetch_add(1, Ordering::SeqCst);
        HashedBag { dims: 32 }.embed(text)
    }
}

#[test]
fn embedder_is_called_and_ranking_stays_bm25() {
    let cards = generate(128);
    let bm25 = Bm25Index::build(cards);
    let expected = bm25.search("rename a file", 3);
    let hits = Arc::new(AtomicUsize::new(0));
    let index = ToolIndex::with_embedder(
        bm25,
        Box::new(Counting {
            hits: Arc::clone(&hits),
        }),
    );
    let got = index.search("rename a file", 3);
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    assert_eq!(
        got.iter().map(|hit| hit.name.clone()).collect::<Vec<_>>(),
        expected
            .iter()
            .map(|hit| hit.name.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn hashed_bag_is_deterministic_and_not_all_zero() {
    let bag = HashedBag { dims: 32 };
    let first = bag.embed("rename a file");
    let second = bag.embed("rename a file");
    assert_eq!(first, second);
    assert!(first.iter().any(|value| *value > 0.0));
    assert_ne!(first, bag.embed("delete a branch"));
}
