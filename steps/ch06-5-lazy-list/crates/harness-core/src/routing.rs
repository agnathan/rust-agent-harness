//! Two-level routing, then a smaller schema.
//!
//! The first level picks a server from a one-line summary. The second level
//! searches only that server's index. [`prepare_card`] trims the description,
//! normalizes the name, and drops a sentence that mentions a tool the agent
//! cannot call.

use crate::catalog::{tokenize, ToolCard, SERVERS};
use crate::index::{Bm25Index, Hit};
use std::collections::{HashMap, HashSet};

pub struct Router {
    servers: Vec<ServerRoute>,
}

struct ServerRoute {
    server: String,
    summary: String,
    index: Bm25Index,
}

impl Router {
    pub fn build(cards: Vec<ToolCard>) -> Self {
        let mut groups: HashMap<String, Vec<ToolCard>> = HashMap::new();
        for card in cards {
            groups.entry(card.server.clone()).or_default().push(card);
        }
        let servers = SERVERS
            .iter()
            .map(|server| {
                let group = groups.remove(*server).unwrap_or_default();
                ServerRoute {
                    server: (*server).to_string(),
                    summary: server_summary(server),
                    index: Bm25Index::build(group),
                }
            })
            .collect();
        Self { servers }
    }

    pub fn route(&self, query: &str) -> &str {
        let tokens = tokenize(query);
        let mut best = self
            .servers
            .first()
            .map(|route| route.server.as_str())
            .unwrap_or("");
        let mut best_score = -1i32;
        for route in &self.servers {
            let mut score = 0;
            let summary_tokens = tokenize(&route.summary);
            for token in &tokens {
                if summary_tokens.iter().any(|got| got == token) || route.server == *token {
                    score += 1;
                }
            }
            if score > best_score {
                best_score = score;
                best = route.server.as_str();
            }
        }
        best
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<Hit> {
        let server = self.route(query);
        self.servers
            .iter()
            .find(|route| route.server == server)
            .map(|route| route.index.search(query, k))
            .unwrap_or_default()
    }
}

pub fn server_summary(server: &str) -> String {
    let noun = match server {
        "files" => "file",
        "git" => "branch",
        "calendar" => "event",
        "mail" => "message",
        "notes" => "note",
        "web" => "page",
        "shell" => "command",
        "db" => "row",
        _ => "item",
    };
    format!("{server}: list, read, write, rename, and move a {noun}")
}

pub fn shrink_card(card: &ToolCard) -> ToolCard {
    let mut name = normalize_name(&card.name);
    let prefix = format!("{}_", card.server);
    if !card.server.is_empty() && !name.starts_with(&prefix) {
        name = format!("{prefix}{name}");
    }
    ToolCard {
        name,
        description: crate::catalog::trim_words(&card.description, 80),
        input_schema: card.input_schema.clone(),
        server: card.server.clone(),
    }
}

pub fn rewrite_description(description: &str, available: &HashSet<String>) -> String {
    let mut kept = Vec::new();
    for sentence in description.split_inclusive(". ") {
        if mentions_unavailable(sentence, available) {
            continue;
        }
        let trimmed = sentence.trim();
        if !trimmed.is_empty() {
            kept.push(trimmed);
        }
    }
    let mut text = kept.join(" ");
    while text.contains("  ") {
        text = text.replace("  ", " ");
    }
    text.trim().to_string()
}

pub fn prepare_card(card: &ToolCard, available: &HashSet<String>) -> ToolCard {
    let mut shrunk = shrink_card(card);
    shrunk.description = rewrite_description(&shrunk.description, available);
    shrunk
}

fn mentions_unavailable(sentence: &str, available: &HashSet<String>) -> bool {
    let Some(rest) = sentence.split("tool:").nth(1) else {
        return false;
    };
    let name: String = rest
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect();
    !name.is_empty() && !available.contains(&name)
}

fn normalize_name(name: &str) -> String {
    let mut out = String::new();
    let mut prev_us = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_us = false;
        } else if !prev_us && !out.is_empty() {
            out.push('_');
            prev_us = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}
