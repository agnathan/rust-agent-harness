//! Book chapter 5. A minimal MCP server process: a fixed tool list, a session
//! view of that list, and a gate that accepts 32 calls and runs 8 at once.
//!
//! This step binds no socket. Streamable HTTP is chapter 40. The process crate
//! `mcp-gateway` starts from the same empty catalog; chapters 7 and 8 fill it.

use std::future::Future;
use std::sync::Arc;

use serde_json::Value;
use thiserror::Error;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Calls waiting or running. The 33rd is refused.
pub const ACCEPT_BOUND: usize = 32;
/// Calls inside a tool body at once.
pub const EXECUTE_BOUND: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCard {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionView {
    pub session_id: String,
    pub tools: Vec<ToolCard>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GatewayError {
    #[error("duplicate tool {name}")]
    Duplicate { name: String },
    #[error("unknown tool {name}")]
    Unknown { name: String },
    #[error("gateway saturated ({bound} calls already accepted)")]
    Saturated { bound: usize },
    #[error("{0}")]
    BadArgs(String),
}

/// Two semaphores. The accept permit is held for the whole call, including
/// the wait for an execute permit, so a full gate refuses new work instead of
/// queueing past [`ACCEPT_BOUND`].
#[derive(Clone)]
pub struct CallGate {
    accept: Arc<Semaphore>,
    execute: Arc<Semaphore>,
    accept_bound: usize,
}

impl CallGate {
    pub fn new(accept_bound: usize, execute_bound: usize) -> Self {
        Self {
            accept: Arc::new(Semaphore::new(accept_bound)),
            execute: Arc::new(Semaphore::new(execute_bound)),
            accept_bound,
        }
    }

    pub fn accept_bound(&self) -> usize {
        self.accept_bound
    }

    pub async fn run<T, F, Fut>(&self, body: F) -> Result<T, GatewayError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        let _accept = self.acquire_accept()?;
        let _execute = self
            .execute
            .clone()
            .acquire_owned()
            .await
            .expect("execute semaphore");
        Ok(body().await)
    }

    fn acquire_accept(&self) -> Result<OwnedSemaphorePermit, GatewayError> {
        self.accept
            .clone()
            .try_acquire_owned()
            .map_err(|_| GatewayError::Saturated {
                bound: self.accept_bound,
            })
    }
}

#[derive(Clone)]
pub struct Gateway {
    tools: Vec<ToolCard>,
    gate: CallGate,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
            gate: CallGate::new(ACCEPT_BOUND, EXECUTE_BOUND),
        }
    }

    pub fn register(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<(), GatewayError> {
        let name = name.into();
        if self.tools.iter().any(|tool| tool.name == name) {
            return Err(GatewayError::Duplicate { name });
        }
        self.tools.push(ToolCard {
            name,
            description: description.into(),
        });
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// Registration order is list order. Every session sees the same fixed list.
    pub fn session_tools(&self, session_id: impl Into<String>) -> SessionView {
        SessionView {
            session_id: session_id.into(),
            tools: self.tools.clone(),
        }
    }

    pub fn gate(&self) -> &CallGate {
        &self.gate
    }

    /// `echo` returns the string `text`. Any other registered name returns its description.
    pub async fn call(&self, name: &str, arguments: &Value) -> Result<String, GatewayError> {
        if !self.tools.iter().any(|tool| tool.name == name) {
            return Err(GatewayError::Unknown {
                name: name.to_string(),
            });
        }
        let description = self
            .tools
            .iter()
            .find(|tool| tool.name == name)
            .map(|tool| tool.description.clone())
            .unwrap_or_default();
        let arguments = arguments.clone();
        self.gate
            .run(|| async move {
                if name == "echo" {
                    let text = arguments
                        .get("text")
                        .and_then(Value::as_str)
                        .ok_or_else(|| GatewayError::BadArgs("echo requires text".into()))?;
                    Ok(text.to_string())
                } else {
                    Ok(description)
                }
            })
            .await?
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::new()
    }
}
