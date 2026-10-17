//! BM25 over tool names and descriptions. An exact name match ranks first.
//! Ranking does not use the embedder.

use std::collections::HashMap;

use crate::tokenize;

const K1: f64 = 1.2;
const B: f64 = 0.75;

#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub name: String,
    pub server: String,
    pub score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCard {
    pub name: String,
    pub description: String,
    pub server: String,
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
        let mut by_name = HashMap::with_capacity(cards.len());
        let mut postings: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
        let mut doc_len = Vec::with_capacity(cards.len());
        let mut length_sum = 0u64;
        for (index, card) in cards.iter().enumerate() {
            by_name.insert(card.name.clone(), index);
            let mut counts: HashMap<String, u32> = HashMap::new();
            for term in tokenize(&format!("{} {}", card.name, card.description)) {
                *counts.entry(term).or_insert(0) += 1;
            }
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

    pub fn cards(&self) -> &[ToolCard] {
        &self.cards
    }

    pub fn card(&self, name: &str) -> Option<&ToolCard> {
        self.by_name.get(name).map(|index| &self.cards[*index])
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<Hit> {
        if k == 0 || self.cards.is_empty() {
            return Vec::new();
        }
        let exact = query.trim().to_ascii_lowercase();
        let mut exact_hit = None;
        if let Some(index) = self
            .cards
            .iter()
            .position(|card| card.name.eq_ignore_ascii_case(&exact))
        {
            exact_hit = Some(Hit {
                name: self.cards[index].name.clone(),
                server: self.cards[index].server.clone(),
                score: f64::INFINITY,
            });
        }
        let terms = tokenize(query);
        if terms.is_empty() && exact_hit.is_none() {
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
        let mut order: Vec<usize> = (0..self.cards.len())
            .filter(|&index| scores[index] > 0.0)
            .filter(|&index| {
                exact_hit
                    .as_ref()
                    .is_none_or(|hit| self.cards[index].name != hit.name)
            })
            .collect();
        order.sort_by(|left, right| {
            scores[*right]
                .partial_cmp(&scores[*left])
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| self.cards[*left].name.cmp(&self.cards[*right].name))
        });
        let room = k.saturating_sub(usize::from(exact_hit.is_some()));
        order.truncate(room);
        let mut hits = Vec::with_capacity(order.len() + usize::from(exact_hit.is_some()));
        if let Some(hit) = exact_hit {
            hits.push(hit);
        }
        hits.extend(order.into_iter().map(|index| Hit {
            name: self.cards[index].name.clone(),
            server: self.cards[index].server.clone(),
            score: scores[index],
        }));
        hits
    }
}
