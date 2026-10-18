//! Chapter 6: the agent loop. Model, native and MCP tools, results, repeat.

mod agent;
mod budget;
mod dispatch;
mod evals;
mod event;
mod inbox;
mod live;
mod merge;
mod message;
mod provider;
mod registry;
mod session;
mod stall;
mod tool;
mod validate;

pub use agent::{run_loop, AgentSession, TurnOutcome};
pub use budget::{check_gate, drop_pending_steer, Budget, CancelFlag, StopReason};
pub use dispatch::{CallOutput, Dispatcher, ToolCall, DISPATCH_BOUND};
pub use evals::{
    eight_delay_calls, run_saved_eval, run_saved_evals, saved_evals, DelayRead, EvalScore,
    SavedEval,
};
pub use event::{format_cost, AgentUsage, Event, Usage};
pub use inbox::{InboxError, SessionInbox, INBOX_BOUND};
pub use live::{
    missing_live_key, tool_definitions, LiveProvider, OpenAiCompatible, DEFAULT_MODEL,
    OPENROUTER_BASE_URL,
};
pub use merge::{register_remote, McpTool};
pub use message::{Message, RequestedTool, Role};
pub use provider::{Provider, ScriptedProvider, ScriptedReply};
pub use registry::ToolRegistry;
pub use session::{fixture_binary, HttpFixture, McpSession, SessionError};
pub use stall::{
    stub_text, StallGuard, CYCLE_HALT_LAPS, STALL_HALT_AFTER, STALL_NOTICE_AT, STUB_MIN_CHARS,
};
pub use tool::{EchoTool, NoteTool, StaticTool, Tool, ToolError, ToolKind, ToolResult, ToolSchema};
pub use validate::{repair_name, MAX_INVALID_STRIKES, MAX_JSON_RETRIES};
