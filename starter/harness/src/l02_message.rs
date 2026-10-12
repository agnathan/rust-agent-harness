//! Lesson 02: structs and enums as the agent's core data types.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl Role {
    /// "system" | "user" | "assistant" | "tool"
    pub fn as_str(&self) -> &'static str {
        // Hint: match on self
        todo!("lesson 02")
    }
}

/// A model's request to run a tool. `arguments` is a JSON string (as OpenAI sends it).
#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_call_id: Option<String>,
}

impl Message {
    fn new(role: Role, content: impl Into<String>) -> Self {
        Message {
            role,
            content: content.into(),
            tool_calls: Vec::new(),
            tool_call_id: None,
        }
    }
    pub fn system(content: impl Into<String>) -> Self {
        // Hint: call Self::new with the right Role
        todo!("lesson 02")
    }
    pub fn user(content: impl Into<String>) -> Self {
        // Hint: same pattern as system()
        todo!("lesson 02")
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        // Hint: same pattern as system()
        todo!("lesson 02")
    }
    /// Assistant message with empty content that asks for tool calls.
    pub fn assistant_tool_calls(calls: Vec<ToolCall>) -> Self {
        // Hint: build with Self::new then set tool_calls (struct update or `let mut m`)
        todo!("lesson 02")
    }
    /// A tool result answering the call with id `call_id`.
    pub fn tool_result(call_id: impl Into<String>, content: impl Into<String>) -> Self {
        // Hint: Role::Tool + tool_call_id = Some(..)
        todo!("lesson 02")
    }
    pub fn has_tool_calls(&self) -> bool {
        // Hint: Vec::is_empty
        todo!("lesson 02")
    }
}

#[derive(Debug, Clone, Default)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Conversation {
    /// Start a conversation with a system prompt as message 0.
    pub fn new(system_prompt: &str) -> Self {
        // Hint: vec![Message::system(..)]
        todo!("lesson 02")
    }
    pub fn push(&mut self, m: Message) {
        // Hint: Vec::push
        todo!("lesson 02")
    }
    /// The most recent assistant message, if any.
    pub fn last_assistant(&self) -> Option<&Message> {
        // Hint: iter().rev().find(..)
        todo!("lesson 02")
    }
    pub fn count_role(&self, role: &Role) -> usize {
        // Hint: iter().filter(..).count()
        todo!("lesson 02")
    }
    /// Hermes invariant: never two same-role messages in a row,
    /// EXCEPT consecutive Tool messages (one per tool call) are fine.
    pub fn roles_alternate(&self) -> bool {
        // Hint: look at slice::windows(2)
        todo!("lesson 02")
    }
}
