//! Chapter 10: chapter 9's agent loop, plus the turn table that `run_loop` drives.

pub mod adapters;
mod agent;
pub mod approval;
mod budget;
mod canonical;
mod dispatch;
mod evals;
mod event;
pub mod failure;
pub mod hooks;
mod inbox;
mod live;
pub mod local_tools;
mod merge;
mod message;
mod provider;
mod registry;
pub mod sandbox;
mod session;
mod stall;
mod tool;
mod truncate;
mod turn;
mod validate;
pub mod web;

pub use agent::{
    append_steer_text, run_loop, run_loop_driven, run_loop_with_inbox, AgentSession, TurnOutcome,
};
pub use budget::{
    check_gate, drop_pending_steer, inject_budget_notices, iteration_warning,
    normalize_warning_ratio, Budget, CancelFlag, NoticeLatch, StopReason,
    DEFAULT_SUBAGENT_ITERATIONS, DEFAULT_WARNING_RATIO, ITERATION_WARNING_PREFIX,
    RUN_WRAPUP_NOTICE,
};
pub use canonical::{canonical_json, canonical_messages, prompt_prefix};
pub use dispatch::{CallOutput, Dispatcher, ToolCall, DISPATCH_BOUND};
pub use evals::{
    eight_delay_calls, prepare_live_evals, run_saved_eval, run_saved_evals, run_stress_eval,
    run_stress_evals, saved_evals, stress_eval_names, DelayRead, EvalScore, SavedEval,
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
pub use truncate::{truncate_head_tail, HEAD_RATIO, TOOL_OUTPUT_MAX_CHARS};
pub use turn::{
    finish_wrap, run_script, step, Driver, Illegal, Phase, ScriptStep, TurnEvent, WrapKind,
};
pub use validate::{repair_name, MAX_INVALID_STRIKES, MAX_JSON_RETRIES};
