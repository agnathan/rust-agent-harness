//! Chapter 4, section 4: cache `tools/list`, and drop it on `tools/list_changed`.

mod cache;
mod dispatch;
mod merge;
mod registry;
mod session;
mod tool;

pub use cache::ToolListCache;
pub use dispatch::{CallOutput, Dispatcher, ToolCall, DISPATCH_BOUND};
pub use merge::{register_remote, McpTool};
pub use registry::ToolRegistry;
pub use session::{fixture_binary, HttpFixture, McpSession, RemoteTool, SessionError};
pub use tool::{EchoTool, StaticTool, Tool, ToolError, ToolKind, ToolResult, ToolSchema};
