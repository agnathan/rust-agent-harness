# Lesson 02: Structs and enums as Message and Role

**Goal:** Model an agent conversation with Rust types.

## Concepts
- **`struct`** groups named fields. **`enum`** is one-of-several variants, and variants can carry data.
- **`impl`** blocks add methods.
  - `&self` borrows the value, `&mut self` mutably borrows it, and `self` consumes it.
  - `Self::new(..)` is a convention for constructors, not a keyword.
- **`match`** must handle every variant. That exhaustiveness check is the main reason enums are so useful for modelling states and messages.
- **`#[derive(Debug, Clone, PartialEq)]`** auto-generates printing, copying and `==`.
- **`Option<T>`** is `Some(v)` or `None`; Rust has no null. **`Vec<T>`** is a growable array.
- **`impl Into<String>`** lets a function accept both `"text"` and `String`.

## How Hermes does it
- Messages are plain Python dicts in OpenAI format: `{"role": "user", "content": ...}`.
- Assistant messages may contain `tool_calls`. Tool results carry a `tool_call_id`.
- Rule from `agent/AGENTS.md`: roles must strictly alternate. The only exception is a run of consecutive tool results, one per call.
- Our `Role` enum makes an invalid role impossible to construct, which is something a dict can't guarantee.

## Tasks
1. Implement `Role::as_str` with a `match`.
2. Implement the `Message` constructors.
   - Hint: the private `Self::new` does most of the work.
   - For `assistant_tool_calls`, use `let mut m = ...; m.tool_calls = calls; m`.
3. Implement `Conversation::new`, `push`, `last_assistant` and `count_role`.
   - Hint: `.iter().rev().find(|m| ...)`.
4. Implement `roles_alternate`.
   - Hint: `self.messages.windows(2).all(|w| ...)`.

## Milestone
`cargo test --test l02_message` is green.

## Exercises
- Add `Role::Developer` (newer OpenAI APIs have it). Watch the compiler point at every `match` you now need to update.
- Make `ToolCall.arguments` a `serde_json::Value` instead of a `String`. What gets easier, and what gets harder?
