//! Chapter 5, section 3: `search_tools` and a lazy, append-only tool prompt.
//!
//! Core schemas stay a byte-stable prefix. Matched schemas are appended on
//! the next turn. Eviction drops the oldest revealed names and leaves the
//! prefix alone.

mod catalog;
mod index;
mod session;

pub use catalog::{
    approx_tokens, generate, mention_count, schema_cost, schema_json, tokenize, trim_words,
    SchemaCost, ToolCard, NOUNS, SERVERS, VERBS,
};
pub use index::{Bm25Index, Embedder, HashedBag, Hit, ToolIndex};
pub use session::{milestone, search_tool_card, MilestoneReport, ToolSession, SEARCH_TOOL};
