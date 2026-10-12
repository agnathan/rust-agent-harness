# Lesson 03: Ownership, borrowing, slices and lifetimes

**Goal:** Learn Rust's core rule through history trimming and transcripts.

## Concepts
- **Ownership.**
  - Every value has exactly one owner, and the value is dropped when the owner goes out of scope.
  - Passing a `Vec` by value *moves* it, so the caller can't use it afterwards.
  - Moving costs nothing at runtime; nothing is copied.
- **Borrowing.**
  - `&T` is a shared, read-only borrow. You can have many at once.
  - `&mut T` is an exclusive, writable borrow. Only one can exist at a time.
  - This rule is what makes data races impossible, and it pays off in later lessons when we have many agents.
- **Slices.** `&[Message]` is a borrowed view of any contiguous run of messages. Prefer `&[T]` over `&Vec<T>` as a parameter type.
- **Lifetimes.** `'a` in `fn longest<'a>(m: &'a [Message]) -> Option<&'a Message>` tells the compiler that the returned reference lives no longer than the input slice.
- **`into_iter()`** takes each item out by value. **`iter()`** borrows each item.

## How Hermes does it
- Context compression (`agent/context_compressor.py`) first prunes old tool results and keeps the system prompt byte-for-byte stable.
- Our `trim_history` is the simplest possible version of that: keep the system message, drop old messages.
- In Python, any part of the code can mutate the list. In Rust, `&mut Vec` makes it explicit who is allowed to change it.

## Tasks
1. `total_chars(&[Message])`.
   - Hint: `.iter().map(..).sum()`.
2. `trim_history(&mut Vec<Message>, keep_last)`.
   - Hint: work out `start` (1 if there's a system message, else 0), then call `msgs.drain(start..start + n_to_drop)`.
3. `into_transcript(conv: Conversation)`, which consumes the conversation.
   - Try using `conv` after calling it and read the compiler error. That error is the lesson.
4. `longest_message` with an explicit lifetime. On ties the *first* match wins.
   - Watch out: `max_by_key` returns the *last* maximum.

## Milestone
`cargo test --test l03_ownership` is green.

## Exercises
- Write `fn first_user<'a>(c: &'a Conversation) -> Option<&'a str>`.
- Try to keep a `&Message` from a `Vec` and then `push` to that same `Vec`. Explain the borrow checker error in one sentence.
