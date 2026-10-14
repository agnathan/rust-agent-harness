//! Chapter 5, section 5: cache `tools/list`, and give each agent a toolset.
//!
//! [`SyntheticSource::list`] stands in for MCP `tools/list`.
//! [`ListCache::notify_list_changed`] stands in for
//! `notifications/tools/list_changed`. Chapter 4 replaces those two calls.

mod cache;
mod catalog;
mod config;
mod event;
mod index;
mod message;
mod provider;
mod routing;
mod session;

pub use cache::{cards_from_listed, AgentView, ListCache, ListedTool, SyntheticSource};
pub use catalog::{
    approx_tokens, generate, mention_count, schema_cost, schema_json, tokenize, trim_words,
    SchemaCost, ToolCard, NOUNS, SERVERS, VERBS,
};
pub use config::{resolve_endpoint, Endpoint, EndpointInput, DEFAULT_MODEL, OPENROUTER_BASE_URL};
pub use event::{events_for_reply, format_cost, AgentUsage, Event, Usage};
pub use index::{Bm25Index, Embedder, HashedBag, Hit, ToolIndex};
pub use message::{Message, Role};
pub use provider::{openai_tool, OpenAiCompatible, Provider, ProviderError};
pub use routing::{prepare_card, rewrite_description, server_summary, shrink_card, Router};
pub use session::{milestone, search_tool_card, MilestoneReport, ToolSession, SEARCH_TOOL};
