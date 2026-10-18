//! Sort object keys and drop insignificant whitespace before a request is
//! sent. The stored transcript is not rewritten. The copy the provider sees
//! is what has to stay byte-identical across repeats.

use serde_json::{Map, Value};

use crate::message::{Message, Role};

pub fn canonical_json(raw: &str) -> Option<String> {
    let value: Value = serde_json::from_str(raw).ok()?;
    Some(canonical_value(&value))
}

pub fn canonical_value(value: &Value) -> String {
    serde_json::to_string(&sort_keys(value)).unwrap_or_else(|_| "{}".to_string())
}

fn sort_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let mut out = Map::new();
            for key in keys {
                out.insert(key.clone(), sort_keys(&map[&key]));
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(sort_keys).collect()),
        other => other.clone(),
    }
}

pub fn canonical_messages(messages: &[Message]) -> Vec<Message> {
    messages
        .iter()
        .map(|message| {
            let mut copy = message.clone();
            for call in &mut copy.tool_calls {
                let raw = call
                    .raw_arguments
                    .clone()
                    .unwrap_or_else(|| call.arguments.to_string());
                if let Some(text) = canonical_json(&raw) {
                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                        call.arguments = value;
                        call.raw_arguments = Some(text);
                    }
                }
            }
            copy
        })
        .collect()
}

pub fn prompt_prefix(messages: &[Message]) -> String {
    let mut out = String::new();
    for message in canonical_messages(messages) {
        out.push_str(match message.role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::Tool => "tool",
        });
        out.push('|');
        out.push_str(&message.content);
        out.push('|');
        if let Some(id) = &message.tool_call_id {
            out.push_str(id);
        }
        for call in &message.tool_calls {
            out.push('|');
            out.push_str(&call.id);
            out.push(':');
            out.push_str(&call.name);
            out.push(':');
            let args = call
                .raw_arguments
                .clone()
                .unwrap_or_else(|| canonical_value(&call.arguments));
            out.push_str(&args);
        }
        out.push('\n');
    }
    out
}
