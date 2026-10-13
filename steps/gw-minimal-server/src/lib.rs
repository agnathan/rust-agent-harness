//! Book chapter 5, the minimal MCP server. The crate under `mcp-gateway/` is
//! the process. This step re-exports it so CI tests the skeleton with the
//! other steps. It still binds no socket.
pub use mcp_gateway::Gateway;
