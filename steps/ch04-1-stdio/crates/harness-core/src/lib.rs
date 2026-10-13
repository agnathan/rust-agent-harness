//! Chapter 4, section 1: an MCP client over stdio.
//!
//! The server is `mcp_fixture`, a child process. This crate does not speak HTTP.

mod session;

pub use session::{fixture_binary, McpSession, RemoteTool, SessionError};
