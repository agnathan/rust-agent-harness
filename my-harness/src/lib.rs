//! Learner crate. Each chapter's behavior is `todo!()` until you fill it in.
//! The milestone specs are `tests/chNN.rs`.
#![allow(unused, dead_code)]

pub mod message;
pub mod provider;

pub use message::{Message, Role};
pub use provider::{MockProvider, Provider, ProviderError};
