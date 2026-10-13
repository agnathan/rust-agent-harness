//! A cached `tools/list`. `notifications/tools/list_changed` drops it.

use std::sync::Arc;

use crate::session::{McpSession, RemoteTool, SessionError};

pub struct ToolListCache {
    session: Arc<McpSession>,
    cached: Option<Arc<[RemoteTool]>>,
}

impl ToolListCache {
    pub fn new(session: Arc<McpSession>) -> Self {
        Self {
            session,
            cached: None,
        }
    }

    pub fn session(&self) -> &McpSession {
        &self.session
    }

    pub fn list_calls(&self) -> u64 {
        self.session.list_calls()
    }

    /// Returns the cached list. A `list_changed` notification, or a cold cache,
    /// calls `tools/list` again.
    pub async fn list(&mut self) -> Result<Arc<[RemoteTool]>, SessionError> {
        if self.session.take_list_changed() {
            self.cached = None;
        }
        if let Some(cached) = &self.cached {
            return Ok(Arc::clone(cached));
        }
        let tools = self.session.list_tools().await?;
        let cached = Arc::<[RemoteTool]>::from(tools);
        self.cached = Some(Arc::clone(&cached));
        Ok(cached)
    }
}
