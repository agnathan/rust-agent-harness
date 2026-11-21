//! Tool-call checks before a batch runs. The shape follows Hermes
//! `validate_tool_calls`: repair a close name, retry malformed JSON without
//! touching the transcript, and close every path with a tool result so the
//! next message is not tool-role followed by user.

use std::collections::HashSet;

use serde_json::Value;

use crate::message::RequestedTool;

pub const MAX_INVALID_STRIKES: u32 = 3;
pub const MAX_JSON_RETRIES: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArgIssue {
    Malformed,
    Truncated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlannedCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    pub error: Option<String>,
    pub malformed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Classified {
    pub calls: Vec<PlannedCall>,
    pub truncated: bool,
    pub malformed: bool,
}

pub fn classify(mut calls: Vec<RequestedTool>, known: &[String]) -> Classified {
    uniquify_ids(&mut calls);
    let mut planned = Vec::with_capacity(calls.len());
    let mut truncated = false;
    let mut malformed = false;
    for call in calls {
        let issue = argument_issue(&call);
        truncated |= issue == Some(ArgIssue::Truncated);
        malformed |= issue == Some(ArgIssue::Malformed);
        let name = repair_name(&call.name, known).unwrap_or(call.name);
        let known_ok = known.iter().any(|item| item == &name);
        let error = if issue == Some(ArgIssue::Truncated) {
            Some(format!("error: truncated JSON arguments for {name}"))
        } else if issue == Some(ArgIssue::Malformed) {
            Some(format!("error: malformed JSON arguments for {name}"))
        } else if !known_ok {
            Some(unknown_message(&name, known))
        } else {
            None
        };
        planned.push(PlannedCall {
            id: call.id,
            name,
            arguments: if call.arguments.is_null() {
                serde_json::json!({})
            } else {
                call.arguments
            },
            error,
            malformed: issue == Some(ArgIssue::Malformed),
        });
    }
    Classified {
        calls: planned,
        truncated,
        malformed,
    }
}

pub fn unknown_message(name: &str, known: &[String]) -> String {
    if name.trim().is_empty() {
        "error: tool name is empty".to_string()
    } else {
        let mut names = known.to_vec();
        names.sort();
        format!(
            "error: Tool '{name}' does not exist. Available tools: {}",
            names.join(", ")
        )
    }
}

/// A unique prefix of length >= 2, or a unique Levenshtein distance of at most 2.
/// Adapted from Hermes `_repair_tool_call`: this chapter uses those two rules
/// and does not pull the rest of the Python repair table.
pub fn repair_name(name: &str, known: &[String]) -> Option<String> {
    if name.is_empty() || known.iter().any(|item| item == name) {
        return None;
    }
    if name.len() >= 2 {
        let prefixes: Vec<&String> = known.iter().filter(|item| item.starts_with(name)).collect();
        if prefixes.len() == 1 {
            return Some(prefixes[0].clone());
        }
    }
    let mut best: Vec<&String> = Vec::new();
    let mut best_distance = 3usize;
    for item in known {
        let distance = levenshtein(name, item);
        if distance <= 2 && distance < best_distance {
            best_distance = distance;
            best.clear();
            best.push(item);
        } else if distance == best_distance && distance <= 2 {
            best.push(item);
        }
    }
    if best.len() == 1 {
        Some(best[0].clone())
    } else {
        None
    }
}

pub fn levenshtein(left: &str, right: &str) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    let mut current = vec![0; right.len() + 1];
    for (row, left_char) in left.iter().enumerate() {
        current[0] = row + 1;
        for (column, right_char) in right.iter().enumerate() {
            let cost = usize::from(left_char != right_char);
            current[column + 1] = (previous[column + 1] + 1)
                .min(current[column] + 1)
                .min(previous[column] + cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[right.len()]
}

fn uniquify_ids(calls: &mut [RequestedTool]) {
    let mut seen = HashSet::new();
    for (index, call) in calls.iter_mut().enumerate() {
        if !seen.insert(call.id.clone()) {
            call.id = format!("{}-{index}", call.id);
        }
    }
}

fn argument_issue(call: &RequestedTool) -> Option<ArgIssue> {
    let raw = call.raw_arguments.as_deref()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if !(trimmed.ends_with('}') || trimmed.ends_with(']')) {
        return Some(ArgIssue::Truncated);
    }
    match serde_json::from_str::<Value>(trimmed) {
        Ok(_) => None,
        Err(_) => Some(ArgIssue::Malformed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_close_name_repairs_and_a_tie_does_not() {
        let known = vec!["echo".into(), "note".into()];
        assert_eq!(repair_name("eco", &known).as_deref(), Some("echo"));
        assert_eq!(repair_name("ech", &known).as_deref(), Some("echo"));
        assert_eq!(repair_name("echo", &known), None);
        let tied = vec!["echo".into(), "ecko".into()];
        assert_eq!(repair_name("eco", &tied), None);
    }

    #[test]
    fn a_blank_name_does_not_list_the_catalog() {
        let message = unknown_message("", &["echo".into()]);
        assert_eq!(message, "error: tool name is empty");
        assert!(!message.contains("Available tools"));
    }
}
