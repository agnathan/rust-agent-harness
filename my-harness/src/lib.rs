//! Learner crate. Each chapter's behavior is `todo!()` until you fill it in.
//! The milestone specs are `tests/chNN.rs`. Step snapshots under `steps/` are
//! the finished reference for that chapter; this crate is the one you grow.
#![allow(unused, dead_code)]

pub mod chat;
pub mod conversation;
pub mod event;
pub mod message;
pub mod provider;

pub use chat::{ChatCommand, ChatSession, Outcome};
pub use conversation::{Conversation, ConversationError};
pub use event::{buffered_events, Event};
pub use message::{Message, Role};
pub use provider::{timeout_after, MockProvider, OpenAiCompatible, Provider, ProviderError};
