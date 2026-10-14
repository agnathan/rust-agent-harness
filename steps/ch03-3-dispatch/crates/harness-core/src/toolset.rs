//! Named groups of tools. A group is visible only when its env var is set
//! to a non-empty value and its check accepts that value. The model is told
//! about the tools that passed, and no others.

use crate::registry::ToolRegistry;
use crate::tool::{Tool, ToolError, ToolSchema};
use std::collections::HashMap;
use std::sync::Arc;

/// Environment values the catalog may read. The library does not touch the
/// process environment, so tests pass a map.
#[derive(Debug, Clone, Default)]
pub struct EnvMap {
    values: HashMap<String, String>,
}

impl EnvMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

pub struct Toolset {
    name: String,
    env_var: String,
    check: Box<dyn Fn(&str) -> bool + Send + Sync>,
    tools: Vec<Arc<dyn Tool>>,
}

impl Toolset {
    /// Visible when `env_var` is set and non-empty. The default check accepts any such value.
    pub fn gated(
        name: impl Into<String>,
        env_var: impl Into<String>,
        tools: Vec<Arc<dyn Tool>>,
    ) -> Self {
        Self {
            name: name.into(),
            env_var: env_var.into(),
            check: Box::new(|_| true),
            tools,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn env_var(&self) -> &str {
        &self.env_var
    }

    /// Extra gate. `value` is the env var's text. Return false to hide the set.
    pub fn with_check(mut self, check: impl Fn(&str) -> bool + Send + Sync + 'static) -> Self {
        self.check = Box::new(check);
        self
    }
}

#[derive(Default)]
pub struct Catalog {
    sets: Vec<Toolset>,
}

impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, toolset: Toolset) {
        self.sets.push(toolset);
    }

    /// Tools whose env var is set and whose check returned true.
    pub fn available(&self, env: &EnvMap) -> Result<ToolRegistry, ToolError> {
        let mut registry = ToolRegistry::new();
        for toolset in &self.sets {
            let Some(value) = env.get(&toolset.env_var) else {
                continue;
            };
            if value.is_empty() || !(toolset.check)(value) {
                continue;
            }
            for tool in &toolset.tools {
                registry.register(Arc::clone(tool))?;
            }
        }
        Ok(registry)
    }

    pub fn available_schemas(&self, env: &EnvMap) -> Result<Vec<ToolSchema>, ToolError> {
        Ok(self.available(env)?.schemas())
    }
}
