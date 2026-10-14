//! The MCP gateway is a separate process from the harness.
//!
//! The harness decides which tools enter a prompt. It knows the task, the
//! context budget, the prompt cache, and the other servers. This process holds
//! the catalog a client may search and list. The chapters that fill the catalog,
//! filter it per session, and serve it over streamable HTTP are not written yet.
//!
//! This skeleton binds no socket and loads no tools.

/// One tool the catalog can offer. The skeleton has no fields beyond the name
/// because no chapter has defined a schema yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCard {
    pub name: String,
}

/// The tools this process knows about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catalog {
    tools: Vec<ToolCard>,
}

impl Catalog {
    pub fn empty() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

/// The tools one session is allowed to list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionView {
    pub session_id: String,
    pub tools: Vec<ToolCard>,
}

/// A gateway handle. Later chapters will swap the catalog behind an `Arc` and
/// serve sessions over HTTP. This type does neither.
#[derive(Debug, Clone)]
pub struct Gateway {
    catalog: Catalog,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            catalog: Catalog::empty(),
        }
    }

    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    /// Tools offered to `session_id`. The skeleton offers none, for every session.
    pub fn session_tools(&self, session_id: impl Into<String>) -> SessionView {
        SessionView {
            session_id: session_id.into(),
            tools: Vec::new(),
        }
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::new()
    }
}
