//! MCP tools land in the same registry as native tools, and the same dispatcher runs them.

use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use crate::registry::ToolRegistry;
use crate::session::{McpSession, RemoteTool, SessionError};
use crate::tool::{Tool, ToolError, ToolKind, ToolResult, ToolSchema};

pub struct McpTool {
    session: Arc<McpSession>,
    info: RemoteTool,
}

impl McpTool {
    pub fn new(session: Arc<McpSession>, info: RemoteTool) -> Self {
        Self { session, info }
    }
}

#[async_trait]
impl Tool for McpTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.info.name.clone(),
            description: self.info.description.clone(),
            input_schema: self.info.input_schema.clone(),
        }
    }

    fn kind(&self) -> ToolKind {
        if self.info.read_only {
            ToolKind::Read
        } else {
            ToolKind::Write
        }
    }

    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
        self.session
            .call_text(&self.info.name, arguments)
            .await
            .map(ToolResult::text)
            .map_err(|error: SessionError| ToolError::Failed {
                message: error.to_string(),
            })
    }
}

/// List the session once and register each tool. A duplicate name is `ToolError::Duplicate`.
pub async fn register_remote(
    registry: &mut ToolRegistry,
    session: Arc<McpSession>,
) -> Result<(), ToolError> {
    let tools = session
        .list_tools()
        .await
        .map_err(|error| ToolError::Failed {
            message: error.to_string(),
        })?;
    for tool in tools {
        registry.register(Arc::new(McpTool::new(Arc::clone(&session), tool)) as Arc<dyn Tool>)?;
    }
    Ok(())
}
