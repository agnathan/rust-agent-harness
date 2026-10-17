//! The MCP gateway process.
//!
//! Chapter 5's step crate is the fixed list and the 32/8 gate. Chapter 7's
//! step crate is the catalog and the `Arc` reload. Chapter 8's step crate
//! is the session toolset. Chapter 15 wraps that gateway with per-tool
//! permits, idempotent retries, and a breaker. Chapter 21 installs a
//! predicted offer on top of that wrapper. It still binds no socket.

pub use gw_learned::LearnedGateway as Gateway;
pub use gw_session::{
    run_rename_task, Arm, Load, SessionError, SessionView, TaskReport, Telemetry,
};
