//! The MCP gateway process.
//!
//! Chapter 5's step crate is the fixed list and the 34/8 gate. Chapter 7's
//! step crate is the catalog and the `Arc` reload. Chapter 8's step crate
//! is the session toolset. Chapter 15 wraps that gateway with per-tool
//! permits, idempotent retries, and a breaker. Chapter 23 installs a
//! predicted offer on top of that wrapper. Chapter 51 copies that
//! service and adds the token arms. This process re-exports the capstone.
//! It still binds no socket.

pub use gw_session::{
    run_rename_task, Arm, Load, SessionError, SessionView, TaskReport, Telemetry,
};
pub use gwservice::LearnedGateway as Gateway;
