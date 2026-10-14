//! A native tool with the same shape as an MCP tool: a name, a description,
//! a JSON Schema for the arguments, and a content result.
//!
//! This is not an MCP client. Chapter 4 is the client. The agent loop does
//! not need to know which side a tool came from, so the shape matches now.

use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    /// Safe to run beside other reads.
    Read,
    /// Runs alone. A write never overlaps another call.
    Write,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    /// A JSON Schema object. Chapter 3 checks `required` keys and nothing else.
    pub input_schema: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolContent {
    pub text: String,
}

impl ToolContent {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// What a tool hands back. MCP calls this a content result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub content: Vec<ToolContent>,
    pub is_error: bool,
}

impl ToolResult {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent::text(text)],
            is_error: false,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ToolError {
    #[error("unknown tool {name}")]
    Unknown { name: String },
    #[error("duplicate tool {name}")]
    Duplicate { name: String },
    #[error("invalid arguments for {name}: {reason}")]
    InvalidArgs { name: String, reason: String },
    #[error("{message}")]
    Failed { message: String },
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn schema(&self) -> ToolSchema;
    fn kind(&self) -> ToolKind;
    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError>;
}

#[async_trait]
impl Tool for Arc<dyn Tool> {
    fn schema(&self) -> ToolSchema {
        (**self).schema()
    }

    fn kind(&self) -> ToolKind {
        (**self).kind()
    }

    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
        (**self).call(arguments).await
    }
}

/// Require every name listed in the schema's `required` array.
/// Other schema keywords are stored and not enforced here.
pub fn check_arguments(schema: &ToolSchema, arguments: &Value) -> Result<(), ToolError> {
    let Some(required) = schema
        .input_schema
        .get("required")
        .and_then(Value::as_array)
    else {
        return Ok(());
    };
    let Some(object) = arguments.as_object() else {
        return Err(ToolError::InvalidArgs {
            name: schema.name.clone(),
            reason: "arguments must be a JSON object".into(),
        });
    };
    for key in required {
        let Some(key) = key.as_str() else {
            continue;
        };
        if !object.contains_key(key) {
            return Err(ToolError::InvalidArgs {
                name: schema.name.clone(),
                reason: format!("missing {key}"),
            });
        }
    }
    Ok(())
}

/// Returns its `text` argument. Read-only.
pub struct EchoTool;

impl EchoTool {
    pub fn schema_value() -> ToolSchema {
        ToolSchema {
            name: "echo".into(),
            description: "Return the text argument unchanged.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "text": { "type": "string" } },
                "required": ["text"],
                "additionalProperties": false
            }),
        }
    }
}

#[async_trait]
impl Tool for EchoTool {
    fn schema(&self) -> ToolSchema {
        Self::schema_value()
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
        let schema = self.schema();
        check_arguments(&schema, &arguments)?;
        let text = arguments
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        Ok(ToolResult::text(text))
    }
}

/// A named tool with a fixed reply. Used to fill a registry in tests and benches.
pub struct StaticTool {
    schema: ToolSchema,
    kind: ToolKind,
    reply: String,
}

impl StaticTool {
    pub fn new(name: impl Into<String>, kind: ToolKind) -> Self {
        let name = name.into();
        Self {
            schema: ToolSchema {
                name: name.clone(),
                description: format!("{name} tool"),
                input_schema: serde_json::json!({ "type": "object", "properties": {} }),
            },
            kind,
            reply: name,
        }
    }

    pub fn arc(name: impl Into<String>, kind: ToolKind) -> Arc<dyn Tool> {
        Arc::new(Self::new(name, kind))
    }
}

#[async_trait]
impl Tool for StaticTool {
    fn schema(&self) -> ToolSchema {
        self.schema.clone()
    }

    fn kind(&self) -> ToolKind {
        self.kind
    }

    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
        check_arguments(&self.schema, &arguments)?;
        Ok(ToolResult::text(self.reply.clone()))
    }
}

/// Sleeps for `delay`, then returns its name. `delay` of zero does not sleep.
/// Spans record the start and end of each call so a test can see overlap.
pub struct LatencyTool {
    schema: ToolSchema,
    kind: ToolKind,
    delay: Duration,
    spans: std::sync::Mutex<Vec<(Instant, Instant)>>,
}

impl LatencyTool {
    pub fn new(name: impl Into<String>, kind: ToolKind, delay: Duration) -> Self {
        let name = name.into();
        Self {
            schema: ToolSchema {
                name: name.clone(),
                description: format!("{name} latency tool"),
                input_schema: serde_json::json!({ "type": "object", "properties": {} }),
            },
            kind,
            delay,
            spans: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn spans(&self) -> Vec<(Instant, Instant)> {
        self.spans.lock().expect("spans").clone()
    }
}

#[async_trait]
impl Tool for LatencyTool {
    fn schema(&self) -> ToolSchema {
        self.schema.clone()
    }

    fn kind(&self) -> ToolKind {
        self.kind
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        let started = Instant::now();
        if !self.delay.is_zero() {
            tokio::time::sleep(self.delay).await;
        }
        let ended = Instant::now();
        self.spans.lock().expect("spans").push((started, ended));
        Ok(ToolResult::text(self.schema.name.clone()))
    }
}
