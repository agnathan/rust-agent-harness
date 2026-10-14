//! Chapter 5, section 1: what it costs to put every tool schema in the prompt.
//!
//! The cards are MCP-shaped. A real `tools/list` from chapter 4 replaces
//! [`generate`]. The token count is the chapter 2 estimate (characters / 4).

mod catalog;

pub use catalog::{
    approx_tokens, generate, mention_count, schema_cost, schema_json, tokenize, trim_words,
    SchemaCost, ToolCard, NOUNS, SERVERS, VERBS,
};
