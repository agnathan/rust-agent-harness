//! Lesson 06: traits and trait objects. The Tool trait and a registry.
use crate::l05_json::{object_schema, ToolSpec};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolError {
    BadArgs(String),
    Failed(String),
    NotFound(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::BadArgs(s) => write!(f, "bad arguments: {s}"),
            ToolError::Failed(s) => write!(f, "tool failed: {s}"),
            ToolError::NotFound(s) => write!(f, "unknown tool: {s}"),
        }
    }
}

/// Like Hermes's ToolEntry (tools/registry.py) but as a trait.
pub trait Tool: Send + Sync {
    fn spec(&self) -> ToolSpec;
    fn call(&self, args: &Value) -> Result<String, ToolError>;
    /// Default method: provided for free to every implementor.
    fn name(&self) -> String {
        self.spec().name
    }
}

/// echo: {"text": string} -> returns the text. Missing/non-string text -> BadArgs("text").
pub struct EchoTool;
/// add: {"a": number, "b": number} -> returns the sum formatted with `{}` (2+3 -> "5").
pub struct AddTool;

impl Tool for EchoTool {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            name: "echo".into(),
            description: "Echo text back".into(),
            parameters: object_schema(&[("text", "string", "text to echo")], &["text"]),
        }
    }
    fn call(&self, args: &Value) -> Result<String, ToolError> {
        // Hint: args.get("text").and_then(Value::as_str)
        todo!("lesson 06")
    }
}

impl Tool for AddTool {
    fn spec(&self) -> ToolSpec {
        // Hint: like EchoTool::spec, name "add", two number params a and b
        todo!("lesson 06")
    }
    fn call(&self, args: &Value) -> Result<String, ToolError> {
        // Hint: Value::as_f64; BadArgs("a") / BadArgs("b") when missing
        todo!("lesson 06")
    }
}

#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns false (and does nothing) if a tool with that name exists.
    pub fn register(&mut self, tool: Box<dyn Tool>) -> bool {
        // Hint: HashMap::contains_key / insert
        todo!("lesson 06")
    }
    pub fn len(&self) -> usize {
        self.tools.len()
    }
    /// All specs, sorted by name (HashMap order is random; tests need stability,
    /// and Hermes cares because a stable tool list keeps the prompt cache valid).
    pub fn specs(&self) -> Vec<ToolSpec> {
        // Hint: collect then sort_by
        todo!("lesson 06")
    }
    pub fn dispatch(&self, name: &str, args: &Value) -> Result<String, ToolError> {
        // Hint: get(name).ok_or(NotFound)?.call(args)
        todo!("lesson 06")
    }
}
