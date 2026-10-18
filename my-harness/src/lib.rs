//! Learner crate. Each chapter's behavior is `todo!()` until you fill it in.
//! The milestone specs are `tests/chNN.rs`. Step snapshots under `steps/` are
//! the finished reference for that chapter; this crate is the one you grow.
#![allow(unused, dead_code)]

pub mod chat;
pub mod conversation;
pub mod event;
pub mod message;
pub mod milestones;
pub mod provider;

pub use milestones::{
    chapter_11_terminal_verdict, chapter_12_plugin_decision, chapter_13_execute_code,
    chapter_14_yolo_allows, chapter_15_web_search_allowed, chapter_16_blocks_metadata,
    chapter_17_stale_write_refused, chapter_18_fallback_model,
    chapter_19_fold_matches_buffer, chapter_20_stored_line, chapter_21_snapshot_changes_on_write,
    chapter_22_discover_hides_subagent,
};

pub use chat::{ChatCommand, ChatSession, Outcome};
pub use conversation::{Conversation, ConversationError};
pub use event::{buffered_events, Event};
pub use message::{Message, Role};
pub use provider::{timeout_after, MockProvider, OpenAiCompatible, Provider, ProviderError};
