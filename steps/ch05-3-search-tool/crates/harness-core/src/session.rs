//! `search_tools` plus a lazy view of the catalog.
//!
//! The core schemas are a stable prefix. A search stages names and does not
//! change that prefix. [`ToolSession::begin_turn`] appends the staged names.
//! If the revealed list is over the cap, the oldest revealed names are
//! dropped. The core prefix is not rewritten.

use crate::catalog::{approx_tokens, schema_cost, schema_json, trim_words, ToolCard};
use crate::index::Bm25Index;
use std::collections::VecDeque;

pub const SEARCH_TOOL: &str = "search_tools";

pub struct ToolSession {
    index: Bm25Index,
    core: Vec<String>,
    revealed: VecDeque<String>,
    staged: Vec<String>,
    cap: usize,
    search_card: ToolCard,
}

impl ToolSession {
    pub fn new(cards: Vec<ToolCard>, cap: usize) -> Self {
        let search_card = search_tool_card();
        let index = Bm25Index::build(cards);
        Self {
            index,
            core: vec![SEARCH_TOOL.to_string()],
            revealed: VecDeque::new(),
            staged: Vec::new(),
            cap,
            search_card,
        }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn staged_len(&self) -> usize {
        self.staged.len()
    }

    pub fn revealed_len(&self) -> usize {
        self.revealed.len()
    }

    /// Stages matches that are not already visible. Returns a short listing,
    /// not the full schemas. Those schemas arrive on the next turn.
    pub fn search_tools(&mut self, query: &str, k: usize) -> String {
        let hits = self.index.search(query, k);
        if hits.is_empty() {
            return "no tools".to_string();
        }
        let mut lines = Vec::with_capacity(hits.len());
        for hit in hits {
            if self.is_visible(&hit.name) || self.staged.iter().any(|name| name == &hit.name) {
                lines.push(format!("{} (already visible)", hit.name));
                continue;
            }
            self.staged.push(hit.name.clone());
            let blurb = self
                .index
                .card(&hit.name)
                .map(|card| trim_words(&card.description, 80))
                .unwrap_or_default();
            lines.push(format!("{} — {blurb}", hit.name));
        }
        lines.join("\n")
    }

    pub fn begin_turn(&mut self) {
        let staged = std::mem::take(&mut self.staged);
        for name in staged {
            if !self.is_visible(&name) {
                self.revealed.push_back(name);
            }
        }
        while self.revealed.len() > self.cap {
            self.revealed.pop_front();
        }
    }

    pub fn visible_names(&self) -> Vec<String> {
        self.core
            .iter()
            .cloned()
            .chain(self.revealed.iter().cloned())
            .collect()
    }

    /// Byte-stable prefix. Reveal and eviction do not change it.
    pub fn core_prompt(&self) -> String {
        format!("{}\n", schema_json(std::slice::from_ref(&self.search_card)))
    }

    /// Core prefix, then the revealed schemas. Empty revealed means the
    /// visible prompt is exactly the core prompt.
    pub fn visible_prompt(&self) -> String {
        let mut prompt = self.core_prompt();
        if !self.revealed.is_empty() {
            let cards = self.revealed_cards();
            prompt.push_str(&schema_json(&cards));
            prompt.push('\n');
        }
        prompt
    }

    fn is_visible(&self, name: &str) -> bool {
        self.core.iter().any(|core| core == name) || self.revealed.iter().any(|got| got == name)
    }

    fn revealed_cards(&self) -> Vec<ToolCard> {
        self.revealed
            .iter()
            .filter_map(|name| self.index.card(name).cloned())
            .collect()
    }
}

pub fn search_tool_card() -> ToolCard {
    ToolCard {
        name: SEARCH_TOOL.to_string(),
        description:
            "Search the tool catalog by keywords. Matching schemas are added on the next turn."
                .to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "query": { "type": "string" }
            },
            "required": ["query"]
        }),
        server: "core".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneReport {
    pub tools: usize,
    pub all_tokens: usize,
    pub visible_before: usize,
    pub listing: String,
    pub this_turn: Vec<String>,
    pub next_turn: Vec<String>,
    pub visible_after: usize,
    pub core_same: bool,
}

/// The chapter demo: a few thousand synthetic tools, a search, and the next turn.
pub fn milestone(count: usize, query: &str) -> MilestoneReport {
    let cards = crate::catalog::generate(count);
    let all_tokens = schema_cost(&cards).tokens;
    let mut session = ToolSession::new(cards, 8);
    let core = session.core_prompt();
    let visible_before = approx_tokens(&session.visible_prompt());
    let listing = session.search_tools(query, 5);
    let this_turn = session.visible_names();
    session.begin_turn();
    let next_turn = session.visible_names();
    let visible_after = approx_tokens(&session.visible_prompt());
    let core_same = session.core_prompt() == core && session.visible_prompt().starts_with(&core);
    MilestoneReport {
        tools: count,
        all_tokens,
        visible_before,
        listing,
        this_turn,
        next_turn,
        visible_after,
        core_same,
    }
}
