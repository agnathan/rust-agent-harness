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
    chapter_17_stale_write_refused, chapter_18_fallback_model, chapter_19_fold_matches_buffer,
    chapter_20_stored_line, chapter_21_snapshot_changes_on_write,
    chapter_22_discover_hides_subagent, chapter_23_read_invokes_scorer,
    chapter_24_index_changes_when_viewed, chapter_25_force_installs_dangerous,
    chapter_26_review_runs_on_user_turn, chapter_27_enqueue_writes_skill, chapter_28_same_context,
    chapter_29_pool_limit, chapter_29_reflection_starts_reviewer, chapter_30_keeps_scratch,
    chapter_31_refuses_overflow, chapter_32_critic_does_the_work, chapter_33_nested_width_fits,
    chapter_34_shares_snapshot, chapter_35_full_inbox_grows, chapter_36_idle_is_stale,
    chapter_37_pick_opens_connection, chapter_38_unscoped_secret_in_prompt,
    chapter_39_enqueue_calls_model, chapter_39_gateway_restart_allowed,
    chapter_40_navigates_metadata, chapter_41_accepts_image, chapter_42_route_after_kill,
    chapter_43_lifecycle_allowed, chapter_43_retries_claimed, chapter_44_update_submits_once,
};

pub use chat::{ChatCommand, ChatSession, Outcome};
pub use conversation::{Conversation, ConversationError};
pub use event::{buffered_events, Event};
pub use message::{Message, Role};
pub use provider::{timeout_after, MockProvider, OpenAiCompatible, Provider, ProviderError};
