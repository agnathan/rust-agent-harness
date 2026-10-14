//! Chapter 5, section 2: search tool names and descriptions.
//!
//! The index is built once. BM25 ranks the query. [`Embedder`] is the seam
//! for a vector model; the included [`HashedBag`] is not one.

mod catalog;
mod index;

pub use catalog::{
    approx_tokens, generate, mention_count, schema_cost, schema_json, tokenize, trim_words,
    SchemaCost, ToolCard, NOUNS, SERVERS, VERBS,
};
pub use index::{Bm25Index, Embedder, HashedBag, Hit, ToolIndex};
