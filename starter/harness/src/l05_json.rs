//! Lesson 05: serde + serde_json. Wire format = OpenAI chat messages and tool schemas.
use crate::l02_message::{Message, Role, ToolCall};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WireFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WireToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: WireFunction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WireMessage {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<WireToolCall>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl From<&Message> for WireMessage {
    /// content: None when the message has tool calls and empty content, else Some(content).
    /// tool_calls: None when empty; each with kind "function".
    fn from(m: &Message) -> Self {
        // Hint: build WireMessage field by field; map ToolCall -> WireToolCall
        todo!("lesson 05")
    }
}

/// Parse a role string; None for anything unknown.
pub fn role_from_str(s: &str) -> Option<Role> {
    // Hint: match
    todo!("lesson 05")
}

impl WireMessage {
    /// Back to our internal Message. Unknown role -> Err("unknown role: x").
    pub fn to_message(&self) -> Result<Message, String> {
        // Hint: role_from_str(..).ok_or_else(..)?, then fill fields (content.unwrap_or_default())
        todo!("lesson 05")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    /// A JSON Schema object.
    pub parameters: Value,
}

impl ToolSpec {
    /// {"type":"function","function":{"name":..,"description":..,"parameters":..}}
    pub fn to_openai(&self) -> Value {
        // Hint: json!({ ... })
        todo!("lesson 05")
    }
}

/// Build {"type":"object","properties":{name:{"type":ty,"description":desc},..},"required":[..]}
pub fn object_schema(props: &[(&str, &str, &str)], required: &[&str]) -> Value {
    // Hint: start with serde_json::Map, insert each property
    todo!("lesson 05")
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AddArgs {
    pub a: f64,
    pub b: f64,
}

/// Generic: parse a JSON argument string into any Deserialize type.
pub fn parse_args<T: DeserializeOwned>(raw: &str) -> Result<T, serde_json::Error> {
    // Hint: serde_json::from_str
    todo!("lesson 05")
}
