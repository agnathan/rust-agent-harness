//! Lesson 01: functions, strings, and your first tests.

/// Return "<name> v<version> ready", e.g. "hermes v3 ready".
pub fn agent_banner(name: &str, version: u32) -> String {
    // Hint: use the format! macro
    todo!("lesson 01")
}

/// Count whitespace-separated words (a crude "token" estimate).
pub fn count_words(text: &str) -> usize {
    // Hint: look at str::split_whitespace and Iterator::count
    todo!("lesson 01")
}

/// True if the input (ignoring leading spaces) starts with '/', like "/stop".
pub fn is_slash_command(input: &str) -> bool {
    // Hint: trim_start + starts_with
    todo!("lesson 01")
}
