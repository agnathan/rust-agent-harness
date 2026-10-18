//! A cached `tools/list`, plus the tools one agent is allowed to see.
//!
//! [`SyntheticSource::list`] stands in for MCP `tools/list`.
//! [`ListCache::notify_list_changed`] stands in for
//! `notifications/tools/list_changed`. When chapter 4's `rmcp` client exists,
//! those two calls are replaced. The cache, the per-agent filter, the index,
//! and the dispatcher stay.
//!
//! A tool group is this same filter. Code mode is one code tool plus the
//! sandbox (chapter 11). A specialist is a sub-agent with its own servers
//! (chapter 25, the supervisor). This chapter only filters.

use crate::catalog::{generate, ToolCard};
use std::collections::BTreeSet;
use std::sync::Arc;

pub struct SyntheticSource {
    cards: Vec<ToolCard>,
    pub list_calls: u64,
}

impl SyntheticSource {
    pub fn new(count: usize) -> Self {
        Self {
            cards: generate(count),
            list_calls: 0,
        }
    }

    /// Stand-in for MCP `tools/list`. [`Self::from_cards`] is the same
    /// method fed by chapter 4's client instead of [`generate`].
    pub fn list(&mut self) -> Vec<ToolCard> {
        self.list_calls += 1;
        self.cards.clone()
    }

    /// Cards already returned by a real `tools/list`.
    pub fn from_cards(cards: Vec<ToolCard>) -> Self {
        Self {
            cards,
            list_calls: 0,
        }
    }

    pub fn replace(&mut self, cards: Vec<ToolCard>) {
        self.cards = cards;
    }
}

/// One listed MCP tool, before it is stored on a [`ToolCard`].
pub struct ListedTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// The shape chapter 4's client hands this cache. `server` is the MCP server
/// name. It stays off the schema the model sees.
pub fn cards_from_listed(server: &str, tools: &[ListedTool]) -> Vec<ToolCard> {
    tools
        .iter()
        .map(|tool| ToolCard {
            name: tool.name.clone(),
            description: tool.description.clone(),
            input_schema: tool.input_schema.clone(),
            server: server.to_string(),
        })
        .collect()
}

pub struct ListCache {
    source: SyntheticSource,
    cached: Option<Arc<[ToolCard]>>,
    dirty: bool,
}

impl ListCache {
    pub fn new(count: usize) -> Self {
        Self {
            source: SyntheticSource::new(count),
            cached: None,
            dirty: true,
        }
    }

    /// Cache a `tools/list` the chapter 4 client already performed.
    pub fn from_cards(cards: Vec<ToolCard>) -> Self {
        Self {
            source: SyntheticSource::from_cards(cards),
            cached: None,
            dirty: true,
        }
    }

    /// A `tools/list_changed` refresh: store the new list and drop the hit.
    pub fn replace_cards(&mut self, cards: Vec<ToolCard>) {
        self.source.replace(cards);
        self.dirty = true;
    }

    pub fn list_calls(&self) -> u64 {
        self.source.list_calls
    }

    pub fn list(&mut self) -> Arc<[ToolCard]> {
        if self.dirty || self.cached.is_none() {
            let cards = self.source.list();
            self.cached = Some(Arc::<[ToolCard]>::from(cards));
            self.dirty = false;
        }
        Arc::clone(self.cached.as_ref().expect("filled above"))
    }

    /// Stand-in for `notifications/tools/list_changed`.
    /// Chapter 4 delivers the real notification and calls this.
    pub fn notify_list_changed(&mut self) {
        self.dirty = true;
    }
}

/// The tools one agent may use. `servers` is the toolset.
pub struct AgentView {
    pub agent: String,
    pub servers: BTreeSet<String>,
}

impl AgentView {
    pub fn new(
        agent: impl Into<String>,
        servers: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            agent: agent.into(),
            servers: servers.into_iter().map(Into::into).collect(),
        }
    }

    pub fn filter<'a>(&self, cards: &'a [ToolCard]) -> Vec<&'a ToolCard> {
        cards
            .iter()
            .filter(|card| self.servers.contains(&card.server))
            .collect()
    }
}
