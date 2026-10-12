//! Lesson 03: ownership, borrowing, slices, lifetimes.
use crate::l02_message::{Conversation, Message, Role};

/// Total characters across all message contents. Borrow, don't take ownership.
pub fn total_chars(msgs: &[Message]) -> usize {
    // Hint: map + sum (use .chars().count() or .len()? the tests use ASCII, either works)
    todo!("lesson 03")
}

/// Crude context compression: keep the system message (if messages[0] is System)
/// plus the last `keep_last` other messages. Mutates in place.
pub fn trim_history(msgs: &mut Vec<Message>, keep_last: usize) {
    // Hint: split off the system message, then drain the front of the rest
    todo!("lesson 03")
}

/// Consume the conversation and render "role: content" lines joined by '\n'.
pub fn into_transcript(conv: Conversation) -> String {
    // Hint: into_iter() moves each Message out; collect::<Vec<_>>().join("\n")
    todo!("lesson 03")
}

/// Longest message by content length; first one wins ties. Note the lifetime 'a:
/// the returned reference lives as long as the slice it points into.
pub fn longest_message<'a>(msgs: &'a [Message]) -> Option<&'a Message> {
    // Hint: a plain loop is fine; max_by_key returns the LAST max, so be careful
    todo!("lesson 03")
}
