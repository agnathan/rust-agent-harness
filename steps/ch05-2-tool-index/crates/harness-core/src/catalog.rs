//! A synthetic catalog of MCP-shaped tools.
//!
//! Chapter 4's client will fill this from `tools/list`. Until that chapter
//! exists, [`generate`] builds the same shape: a name, a description, and a
//! JSON Schema. The server name stays on the card so a later index can route
//! by server. It is not part of the schema the model sees.

use serde_json::{json, Value};

pub const SERVERS: &[&str] = &[
    "files", "git", "calendar", "mail", "notes", "web", "shell", "db",
];
pub const VERBS: &[&str] = &[
    "list", "read", "write", "rename", "delete", "search", "create", "move",
];
pub const NOUNS: &[&str] = &[
    "file", "branch", "event", "message", "note", "page", "command", "row",
];

const STOPWORDS: &[&str] = &[
    "a", "an", "the", "of", "to", "and", "or", "for", "in", "on", "with", "from",
];

#[derive(Debug, Clone, PartialEq)]
pub struct ToolCard {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub server: String,
}

impl ToolCard {
    /// The object a model is shown. `server` stays off this object.
    pub fn schema_value(&self) -> Value {
        json!({
            "name": self.name,
            "description": self.description,
            "input_schema": self.input_schema,
        })
    }
}

/// Deterministic catalog. Tool `i` always has the same name.
pub fn generate(count: usize) -> Vec<ToolCard> {
    (0..count).map(|index| card_at(index, count)).collect()
}

fn card_at(index: usize, count: usize) -> ToolCard {
    let server = SERVERS[index % SERVERS.len()];
    let verb = VERBS[(index / SERVERS.len()) % VERBS.len()];
    let noun = NOUNS[(index / (SERVERS.len() * VERBS.len())) % NOUNS.len()];
    let name = format!("{server}_{verb}_{noun}_{index}");
    let other_index = (index + 17) % count.max(1);
    let other_server = SERVERS[other_index % SERVERS.len()];
    let other_verb = VERBS[(other_index / SERVERS.len()) % VERBS.len()];
    let other_noun = NOUNS[(other_index / (SERVERS.len() * VERBS.len())) % NOUNS.len()];
    let other = format!("{other_server}_{other_verb}_{other_noun}_{other_index}");
    let extra = if index % 5 == 0 {
        format!(" See also tool:{other}.")
    } else {
        String::new()
    };
    let description =
        format!("Use the {server} server to {verb} a {noun}. Tool number {index}.{extra}");
    let field = if matches!(verb, "read" | "list" | "search") {
        "query"
    } else {
        "path"
    };
    ToolCard {
        name,
        description,
        input_schema: json!({
            "type": "object",
            "properties": {
                field: { "type": "string" }
            },
            "required": [field],
        }),
        server: server.to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaCost {
    pub tools: usize,
    pub bytes: usize,
    pub tokens: usize,
}

/// Same estimate as chapter 2: characters divided by 4, rounded up.
/// This is not a tokenizer.
pub fn approx_tokens(text: &str) -> usize {
    text.chars().count().div_ceil(4)
}

pub fn schema_json(cards: &[ToolCard]) -> String {
    let values: Vec<Value> = cards.iter().map(ToolCard::schema_value).collect();
    serde_json::to_string(&values).expect("tool schemas are JSON")
}

pub fn schema_cost(cards: &[ToolCard]) -> SchemaCost {
    let body = schema_json(cards);
    SchemaCost {
        tools: cards.len(),
        bytes: body.len(),
        tokens: approx_tokens(&body),
    }
}

/// How many cards contain every query token in the name or the description.
/// A large count is the selection problem: the words match too many tools.
pub fn mention_count(cards: &[ToolCard], query: &str) -> usize {
    let tokens = tokenize(query);
    if tokens.is_empty() {
        return 0;
    }
    cards
        .iter()
        .filter(|card| {
            let hay = format!("{} {}", card.name, card.description);
            let hay_tokens = tokenize(&hay);
            tokens
                .iter()
                .all(|token| hay_tokens.iter().any(|got| got == token))
        })
        .count()
}

pub fn trim_words(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut last_space = None;
    let mut byte_end = 0;
    for (count, (idx, ch)) in text.char_indices().enumerate() {
        if count == max_chars {
            break;
        }
        if ch.is_whitespace() {
            last_space = Some(idx);
        }
        byte_end = idx + ch.len_utf8();
    }
    let cut = last_space.unwrap_or(byte_end);
    text[..cut].trim_end().to_string()
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| token.len() >= 2)
        .map(|token| token.to_ascii_lowercase())
        .filter(|token| !STOPWORDS.contains(&token.as_str()))
        .collect()
}
