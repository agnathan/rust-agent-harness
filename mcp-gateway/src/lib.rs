//! The MCP gateway process.
//!
//! Chapter 5's step crate is the fixed list and the 32/8 gate. Chapter 7's
//! step crate is the catalog and the `Arc` reload. This crate is chapter 8:
//! a session sees only the tools `search_tools` or `set_task` revealed.
//! It still binds no socket.

pub use gw_session::SessionGateway as Gateway;
pub use gw_session::{
    run_rename_task, Arm, Load, SessionError, SessionView, TaskReport, Telemetry,
};
