//! A keyword index over tool names and descriptions.
//!
//! Built once. Search returns the best names. An [`Embedder`] is optional:
//! the bench and the default search rank with BM25 only. When an embedder is
//! installed, search calls it so a later chapter can swap in a real model
//! without changing the index.

use crate::catalog::{tokenize, ToolCard};
use std::collections::HashMap;

const K1: f64 = 1.2;
const B: f64 = 0.75;

#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub name: String,
    pub server: String,
    pub score: f64,
}

pub struct Bm25Index {
    cards: Vec<ToolCard>,
    by_name: HashMap<String, usize>,
    postings: HashMap<String, Vec<(u32, u32)>>,
    doc_len: Vec<u32>,
    avgdl: f64,
}

impl Bm25Index {
    pub fn build(cards: Vec<ToolCard>) -> Self {
        Self::build_available(cards, |_| true)
    }

    /// `check` is the availability gate. A tool that fails it is not indexed,
    /// so the model cannot be sent its schema later. Hermes does this with a
    /// toolset and a `check_fn`.
    pub fn build_available(cards: Vec<ToolCard>, check: impl Fn(&ToolCard) -> bool) -> Self {
        let cards: Vec<ToolCard> = cards.into_iter().filter(check).collect();
        let mut by_name = HashMap::with_capacity(cards.len());
        let mut postings: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
        let mut doc_len = Vec::with_capacity(cards.len());
        let mut length_sum = 0u64;
        for (index, card) in cards.iter().enumerate() {
            by_name.insert(card.name.clone(), index);
            let counts = term_counts(card);
            let mut len = 0u32;
            for (term, tf) in counts {
                len += tf;
                postings.entry(term).or_default().push((index as u32, tf));
            }
            length_sum += u64::from(len);
            doc_len.push(len);
        }
        let avgdl = if cards.is_empty() {
            1.0
        } else {
            length_sum as f64 / cards.len() as f64
        };
        Self {
            cards,
            by_name,
            postings,
            doc_len,
            avgdl,
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn card(&self, name: &str) -> Option<&ToolCard> {
        self.by_name.get(name).map(|index| &self.cards[*index])
    }

    pub fn cards(&self) -> &[ToolCard] {
        &self.cards
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<Hit> {
        if k == 0 || self.cards.is_empty() {
            return Vec::new();
        }
        let terms = tokenize(query);
        if terms.is_empty() {
            return Vec::new();
        }
        let mut scores = vec![0.0; self.cards.len()];
        let n = self.cards.len() as f64;
        for term in terms {
            let Some(posting) = self.postings.get(&term) else {
                continue;
            };
            let df = posting.len() as f64;
            let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();
            for &(doc, tf) in posting {
                let tf = f64::from(tf);
                let dl = f64::from(self.doc_len[doc as usize]);
                let denom = tf + K1 * (1.0 - B + B * dl / self.avgdl);
                scores[doc as usize] += idf * (tf * (K1 + 1.0)) / denom;
            }
        }
        let mut order: Vec<usize> = (0..self.cards.len()).filter(|&i| scores[i] > 0.0).collect();
        order.sort_by(|left, right| {
            scores[*right]
                .partial_cmp(&scores[*left])
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| self.cards[*left].name.cmp(&self.cards[*right].name))
        });
        order.truncate(k);
        order
            .into_iter()
            .map(|index| Hit {
                name: self.cards[index].name.clone(),
                server: self.cards[index].server.clone(),
                score: scores[index],
            })
            .collect()
    }
}

fn term_counts(card: &ToolCard) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for token in tokenize(&card.description) {
        *counts.entry(token).or_insert(0) += 1;
    }
    // The name is the thing a person types. Count it twice.
    for token in tokenize(&card.name) {
        *counts.entry(token).or_insert(0) += 2;
    }
    counts
}

/// A seam for a vector model. [`HashedBag`] is not one. It only proves the
/// trait is callable. Search ranking stays BM25.
pub trait Embedder: Send + Sync {
    fn embed(&self, text: &str) -> Vec<f32>;
}

pub struct HashedBag {
    pub dims: usize,
}

impl Embedder for HashedBag {
    fn embed(&self, text: &str) -> Vec<f32> {
        let dims = self.dims.max(1);
        let mut bag = vec![0.0f32; dims];
        for token in tokenize(text) {
            let mut hash: u32 = 2_166_136_261;
            for byte in token.bytes() {
                hash ^= u32::from(byte);
                hash = hash.wrapping_mul(16_777_619);
            }
            let slot = (hash as usize) % dims;
            bag[slot] += 1.0;
        }
        bag
    }
}

pub struct ToolIndex {
    bm25: Bm25Index,
    embedder: Option<Box<dyn Embedder>>,
}

impl ToolIndex {
    pub fn new(bm25: Bm25Index) -> Self {
        Self {
            bm25,
            embedder: None,
        }
    }

    pub fn with_embedder(bm25: Bm25Index, embedder: Box<dyn Embedder>) -> Self {
        Self {
            bm25,
            embedder: Some(embedder),
        }
    }

    pub fn bm25(&self) -> &Bm25Index {
        &self.bm25
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<Hit> {
        if let Some(embedder) = &self.embedder {
            let _embedded = embedder.embed(query);
        }
        self.bm25.search(query, k)
    }
}
