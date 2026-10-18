use crate::tool::{Tool, ToolError, ToolSchema};
use std::collections::HashMap;
use std::sync::Arc;

/// Tools the process can call, keyed by name.
#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) -> Result<(), ToolError> {
        let name = tool.schema().name;
        if self.tools.contains_key(&name) {
            return Err(ToolError::Duplicate { name });
        }
        self.tools.insert(name, tool);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    /// Schemas in name order. This is the list a later chapter shows the model.
    pub fn schemas(&self) -> Vec<ToolSchema> {
        let mut schemas: Vec<ToolSchema> = self.tools.values().map(|tool| tool.schema()).collect();
        schemas.sort_by(|left, right| left.name.cmp(&right.name));
        schemas
    }
}
