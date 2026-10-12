//! Lesson 07: a synchronous agent loop against a mock model.
//! Mirrors agent/conversation_loop.py::run_conversation in miniature.
use crate::l02_message::{Message, ToolCall};
use crate::l05_json::ToolSpec;
use crate::l06_tools::ToolRegistry;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum ModelReply {
    Text(String),
    ToolCalls(Vec<ToolCall>),
}

pub trait Model {
    fn complete(&mut self, messages: &[Message], tools: &[ToolSpec]) -> Result<ModelReply, String>;
}

/// Replays canned replies in order. Errors with "script exhausted" when empty.
pub struct ScriptedModel {
    replies: VecDeque<ModelReply>,
    pub calls: usize,
}

impl ScriptedModel {
    pub fn new(replies: Vec<ModelReply>) -> Self {
        // Hint: VecDeque::from
        todo!("lesson 07")
    }
}

impl Model for ScriptedModel {
    fn complete(
        &mut self,
        _messages: &[Message],
        _tools: &[ToolSpec],
    ) -> Result<ModelReply, String> {
        // Hint: increment calls, pop_front
        todo!("lesson 07")
    }
}

#[derive(Debug, PartialEq)]
pub enum LoopError {
    Model(String),
    BudgetExhausted,
}

#[derive(Debug)]
pub struct LoopOutcome {
    pub final_text: String,
    pub messages: Vec<Message>,
    pub api_calls: usize,
}

/// The loop:
/// messages = [system, user]; while api_calls < max_iterations:
///   reply = model.complete(messages, registry.specs())  (count it)
///   Text(t)       -> push assistant(t), return Ok
///   ToolCalls(cs) -> push assistant_tool_calls(cs); for each call push tool_result(id, output)
///                    where output = dispatch result, or "error: <ToolError Display>" on Err,
///                    or "error: invalid JSON arguments" if arguments don't parse.
/// Out of iterations -> Err(BudgetExhausted). Model error -> Err(Model(e)).
pub fn run_agent_loop(
    model: &mut dyn Model,
    registry: &ToolRegistry,
    system: &str,
    user: &str,
    max_iterations: usize,
) -> Result<LoopOutcome, LoopError> {
    // Hint: follow the doc comment; serde_json::from_str::<serde_json::Value>(&call.arguments)
    todo!("lesson 07")
}
