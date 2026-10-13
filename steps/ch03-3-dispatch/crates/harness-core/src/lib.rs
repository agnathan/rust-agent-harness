pub mod chat;
pub mod config;
pub mod conversation;
pub mod dispatch;
pub mod event;
pub mod message;
pub mod provider;
pub mod registry;
pub mod session;
pub mod tool;
pub mod toolset;

pub use chat::{ChatCommand, ChatSession, Outcome};
pub use config::{resolve_endpoint, Endpoint, EndpointInput, DEFAULT_MODEL, OPENROUTER_BASE_URL};
pub use conversation::{Conversation, ConversationError};
pub use dispatch::{CallOutput, Dispatcher, ToolCall, DISPATCH_BOUND};
pub use event::{events_for_reply, format_cost, AgentUsage, Event, Usage};
pub use message::{Message, Role};
pub use provider::{
    ChatRequest, ChatResponse, Choice, DetailedCompletion, MockProvider, OpenAiCompatible,
    Provider, ProviderError,
};
pub use registry::ToolRegistry;
pub use session::{InboxError, SessionInbox, INBOX_BOUND};
pub use tool::{
    check_arguments, EchoTool, LatencyTool, StaticTool, Tool, ToolContent, ToolError, ToolKind,
    ToolResult, ToolSchema,
};
pub use toolset::{Catalog, EnvMap, Toolset};
