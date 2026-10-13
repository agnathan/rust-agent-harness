//! Chapter 5, section 4: pick a server, then shrink the schema.
//!
//! Routing searches one server's index. Shrinking trims descriptions and
//! normalizes names. A sentence that mentions `tool:` plus a name the agent
//! cannot call is dropped, which is what Hermes does when it rewrites a
//! description down to the tools that are actually available.

mod catalog;
mod index;
mod routing;
mod session;

pub use catalog::{
    approx_tokens, generate, mention_count, schema_cost, schema_json, tokenize, trim_words,
    SchemaCost, ToolCard, NOUNS, SERVERS, VERBS,
};
pub use index::{Bm25Index, Embedder, HashedBag, Hit, ToolIndex};
pub use routing::{prepare_card, rewrite_description, server_summary, shrink_card, Router};
pub use session::{milestone, MilestoneReport, ToolSession, SEARCH_TOOL};
