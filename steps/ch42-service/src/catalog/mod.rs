//! Book chapter 7. The server holds the catalog: native tools, plugin tools,
//! and tools copied from an in-memory upstream list. Search is BM25. A reload
//! swaps the `Arc` readers already hold.

mod index;

use std::sync::{Arc, RwLock};

use serde_json::Value;
use thiserror::Error;

pub use index::{Bm25Index, Hit, ToolCard};

pub const DEFAULT_SEARCH_LIMIT: usize = 5;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CatalogError {
    #[error("duplicate tool {name}")]
    Duplicate { name: String },
    #[error("unknown tool {name}")]
    Unknown { name: String },
    #[error("{0}")]
    BadArgs(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Native,
    Plugin,
    Upstream,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginSet {
    pub plugin: String,
    pub tools: Vec<ToolCard>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upstream {
    pub name: String,
    pub tools: Vec<ToolCard>,
}

pub struct Load {
    pub native: Vec<ToolCard>,
    pub plugins: Vec<PluginSet>,
    pub upstreams: Vec<Upstream>,
}

pub struct Snapshot {
    index: Bm25Index,
    origins: Vec<Origin>,
}

impl Snapshot {
    fn build(load: Load) -> Result<Self, CatalogError> {
        let mut cards = Vec::new();
        let mut origins = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        let mut push = |card: ToolCard, origin: Origin| -> Result<(), CatalogError> {
            if !seen.insert(card.name.clone()) {
                return Err(CatalogError::Duplicate { name: card.name });
            }
            cards.push(card);
            origins.push(origin);
            Ok(())
        };
        for card in load.native {
            push(card, Origin::Native)?;
        }
        for plugin in load.plugins {
            for card in plugin.tools {
                push(card, Origin::Plugin)?;
            }
        }
        for upstream in load.upstreams {
            for card in upstream.tools {
                push(card, Origin::Upstream)?;
            }
        }
        Ok(Self {
            origins,
            index: Bm25Index::build(cards),
        })
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    pub fn cards(&self) -> &[ToolCard] {
        self.index.cards()
    }

    pub fn origin(&self, name: &str) -> Option<Origin> {
        self.index
            .cards()
            .iter()
            .position(|card| card.name == name)
            .map(|index| self.origins[index])
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<Hit> {
        self.index.search(query, k)
    }

    pub fn card(&self, name: &str) -> Option<&ToolCard> {
        self.index.card(name)
    }
}

pub trait Embedder: Send + Sync {
    fn embed(&self, text: &str) -> Vec<f32>;
}

/// Eight buckets of byte counts. Not a model. Search does not rank with it.
pub struct HashEmbedder;

impl Embedder for HashEmbedder {
    fn embed(&self, text: &str) -> Vec<f32> {
        let mut buckets = [0f32; 8];
        for byte in text.bytes() {
            buckets[(byte % 8) as usize] += 1.0;
        }
        buckets.to_vec()
    }
}

pub struct Gateway {
    current: RwLock<Arc<Snapshot>>,
    embedder: RwLock<Option<Arc<dyn Embedder>>>,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            current: RwLock::new(Arc::new(Snapshot {
                index: Bm25Index::build(Vec::new()),
                origins: Vec::new(),
            })),
            embedder: RwLock::new(None),
        }
    }

    pub fn set_embedder(&self, embedder: Arc<dyn Embedder>) {
        *self.embedder.write().expect("embedder") = Some(embedder);
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        Arc::clone(&self.current.read().expect("catalog"))
    }

    pub fn len(&self) -> usize {
        self.snapshot().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Install a new catalog. Readers that already cloned the previous `Arc`
    /// keep that snapshot. A duplicate name leaves the previous snapshot in place.
    pub fn reload(&self, load: Load) -> Result<(), CatalogError> {
        let snapshot = Snapshot::build(load)?;
        *self.current.write().expect("catalog") = Arc::new(snapshot);
        Ok(())
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<Hit> {
        if let Some(embedder) = self.embedder.read().expect("embedder").as_ref() {
            std::hint::black_box(embedder.embed(query));
        }
        self.snapshot().search(query, k)
    }

    /// The tool body runs against the snapshot captured at the start of the call.
    pub fn call(&self, name: &str, arguments: &Value) -> Result<String, CatalogError> {
        let snapshot = self.snapshot();
        let card = snapshot.card(name).ok_or_else(|| CatalogError::Unknown {
            name: name.to_string(),
        })?;
        if name == "echo" {
            let text = arguments
                .get("text")
                .and_then(Value::as_str)
                .ok_or_else(|| CatalogError::BadArgs("echo requires text".into()))?;
            return Ok(text.to_string());
        }
        Ok(card.description.clone())
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::new()
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| token.len() >= 2)
        .map(|token| token.to_ascii_lowercase())
        .collect()
}

/// Ten thousand synthetic tools, eight servers. Tool names are stable.
pub fn synthetic(count: usize) -> Vec<ToolCard> {
    const SERVERS: [&str; 8] = [
        "files", "git", "calendar", "mail", "db", "notes", "chat", "ops",
    ];
    const VERBS: [&str; 8] = [
        "list", "read", "write", "rename", "move", "search", "delete", "share",
    ];
    const NOUNS: [&str; 8] = [
        "file", "branch", "event", "message", "row", "note", "thread", "job",
    ];
    (0..count)
        .map(|index| {
            let server = SERVERS[index % SERVERS.len()];
            let verb = VERBS[(index / SERVERS.len()) % VERBS.len()];
            let noun = NOUNS[(index / (SERVERS.len() * VERBS.len())) % NOUNS.len()];
            ToolCard {
                name: format!("{server}_{verb}_{noun}_{index}"),
                description: format!(
                    "Use the {server} server to {verb} a {noun}. Tool number {index}."
                ),
                server: server.to_string(),
            }
        })
        .collect()
}
