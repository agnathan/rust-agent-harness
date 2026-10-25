# Chapter 30: Handoffs

> Confusing delegation with handoff is how a team loses the parent.

A specialist that returns a summary leaves the parent in charge. A handoff is the other move: the next agent owns the task, and the first agent is done. The demo starts at `outline`, drops a scratch row, and prints `draft sees 2`, the new owner and the two rows that survived. A continuously improving harness learns from what that next agent can see, so a debugging aside that rides along becomes part of the lesson.

The code filters the transcript and then writes the owner. It never reads or writes a system prompt, so chapter 2's cache rule stays a different mechanism and this chapter doesn't claim to protect it. What the filter does protect is the scratch. A handoff that forwarded every tool result would forward every secret and every poisoned page the first agent saw. Chapter 20's scrub already ran on the way into the session. This cut is structural: role `scratch` is dropped, the other rows stay in order, and then the owner changes.

`Board` holds a count and an owner. `handoff` filters, then counts, then writes the owner, and `MAX_HANDOFFS` is 4 so a task can't bounce forever. That cap belongs to this harness, and the bench times the filter on 100 messages, the copy the next owner receives. Platform re-bind, where a chat session starts talking to the new owner, stays in chapter 39. This chapter changes the owner field and leaves the chat platform alone.

Nothing here writes a skill. The lesson, if there is one, is still chapter 26's job, and it reviews the parent's view. A handoff that let the next agent inherit the reviewer's tool slice would spread writes across owners. The next agent gets the filtered transcript and the tools its own role already had. Chapter 33 calls this board only when the route says ownership should move.

Chapter 28's `hand_off` is a pointer, and this `handoff` is a new vec. The names sit close together on purpose, and the types don't. The step is chapter 29's harness plus `handoffs`, at `steps/ch30-handoffs`, package `ch30-handoffs`. The filter allocates a new `Vec` so the next agent can't push scratch back into the caller's buffer, and one iteration keeps the order. Under contention the add-then-undo on the count can briefly show a number past 4. Section 30.6 says that plainly, because the tests call `handoff` from one thread.

`cargo run --locked -p ch30-handoffs --bin ch30-handoffs` prints `draft sees 2`. `my-harness/tests/ch30.rs` checks that scratch is dropped.

**In this chapter**

- 30.1 The board is a count and an owner
- 30.2 `filter_transcript` drops scratch and keeps order
- 30.3 `handoff` filters, then counts, then writes the owner
- 30.4 The demo shows the next owner and the filtered length
- 30.5 `filter_100` times the copy the next owner receives
- 30.6 The count moves one accepted call at a time

| Section | Step | What it adds |
|---|---|---|
| 30.1 | `steps/ch30-handoffs` | `Message`, `Board`, `MAX_HANDOFFS` |
| 30.2 | `steps/ch30-handoffs` | `filter_transcript` drops role `scratch` |
| 30.3 | `steps/ch30-handoffs` | `Board::handoff`, the atomic count, `Empty` and `Limit` |
| 30.4 | `steps/ch30-handoffs` | the demo `draft sees 2` |
| 30.5 | `steps/ch30-handoffs` | `filter_100` |

> **How Hermes does it**
>
> I read `tui_gateway/methods_session.py` (`handoff.request`) on Hermes `main` at `ab3fe9dffb0a`.
>
> `handoff.request` only writes pending. The gateway watcher claims and re-binds. If the session is running, the error is 4009. `platform` is required.
>
> **Adopted.** A handoff is a recorded transfer, not a model call. A refused transfer leaves the current owner in place.
>
> **Adapted.** The context is filtered before the next agent. `filter_transcript` drops role `scratch`. `MAX_HANDOFFS` is 4. That cap is this harness. It is not a Hermes constant.
>
> **Deferred.** Platform re-bind, to chapter 39. The watcher that claims the pending row. Error 4009. The required platform field. This `Board` does not take a platform.

## What this chapter decides

`Board::new` stores an owner and an `AtomicU32` at 0. `handoff` filters first. An empty filtered vec is `HandoffError::Empty`, so the count and the owner both stay. Otherwise `fetch_add` increments. If the previous value is already `MAX_HANDOFFS`, `fetch_sub` undoes it and the error is `Limit`. The owner is replaced only after that check passes, and the returned vec is the filtered one.

`filter_transcript` keeps order, and the role it drops is `scratch`.

`demo` starts at `outline`, hands three messages to `draft`, and returns `draft sees 2`, and no key.

`filter_100` times the filter on 100 messages, and it doesn't call `handoff`, and the mean is `2.13 µs`.

## 30.1 The board is a count and an owner

The atomic import is the first line that is new relative to the supervisor.

```rust
use std::sync::atomic::{AtomicU32, Ordering};
```

`AtomicU32` is an unsigned 32-bit integer that `fetch_add` and `load` can update without a `Mutex` around the number. `Ordering` is the memory-ordering argument those methods require, and this file uses `Ordering::Relaxed` on every atomic call.

The cap is a public const.

```rust
/// Harness cap. Not a Hermes constant.
pub const MAX_HANDOFFS: u32 = 4;
```

The doc comment is the warning. Hermes `handoff.request` doesn't count handoffs, and four is the number this harness refuses on. The type is `u32` because the atomic is an `AtomicU32`.

`Message` is the row the filter clones.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub text: String,
}
```

`Clone` is what `.cloned()` calls in the filter. `PartialEq` is what the tests use when they compare text, and this `Message` is `handoffs::Message`. Chapter 28's `pipeline::Message` is a different struct with the same two fields, and the filter doesn't call the pipeline.

The errors are an enum.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffError {
    Limit,
    Empty,
}
```

`Limit` means the cap. `Empty` means the filter removed every message. Neither variant carries a string, and the caller matches the variant.

`Board` holds the two pieces of ownership.

```rust
pub struct Board {
    count: AtomicU32,
    owner: std::sync::Mutex<String>,
}
```

The count is atomic, and the owner is a `Mutex<String>`, and they are not one lock. `new` sets both.

```rust
impl Board {
    pub fn new(owner: impl Into<String>) -> Self {
        Self {
            count: AtomicU32::new(0),
            owner: std::sync::Mutex::new(owner.into()),
        }
    }
```

`impl Into<String>` means the caller can pass `String` or `&str`, and the demo passes `"outline"`. `Into::into` builds the `String` the mutex stores. `AtomicU32::new(0)` is a count of zero, and no handoff has been accepted.

`owner` and `count` are the readers.

```rust
    pub fn owner(&self) -> String {
        self.owner.lock().expect("owner").clone()
    }

    pub fn count(&self) -> u32 {
        self.count.load(Ordering::Relaxed)
    }
```

`owner()` locks, clones the `String`, and drops the guard at the end of the statement. The clone is why the returned name doesn't borrow the mutex. `expect("owner")` panics if the mutex is poisoned. `count()` loads the atomic. `Ordering::Relaxed` is the ordering: this load is atomic, so you don't observe a torn `u32`, and it doesn't synchronize other memory by itself. The tests call `count()` on one thread after `handoff` has returned, so the value they read is the value the same thread stored.

> **Rust sidebar: `AtomicU32`**
>
> - `fetch_add(1, Ordering::Relaxed)` adds one and returns the previous value. Two calls cannot receive the same previous value. The hardware update is the atomic part.
> - `fetch_sub` subtracts one and also returns the previous value. `handoff` ignores that return. It only needs the count to go back down.
> - `Relaxed` does not order this atomic with the owner mutex. The mutex has its own lock. The function's order is source order on one thread: filter, then `fetch_add`, then maybe `fetch_sub`, then the owner lock. The tests call `handoff` from that one thread.
> - `load` is the read `count()` uses. It takes the same `Ordering` argument. There is no `Mutex` around `count`.

## 30.2 `filter_transcript` drops scratch and keeps order

A handoff that forwards every tool result forwards every secret and every poisoned page the first agent saw. The filter is the cut that stops that, and it's a free function: `handoff` calls it, the bench calls it, and `filter_keeps_order` calls it without a board.

```rust
pub fn filter_transcript(messages: &[Message]) -> Vec<Message> {
    messages
        .iter()
        .filter(|message| message.role != "scratch")
        .cloned()
        .collect()
}
```

`messages` is a shared slice, and the function can't remove a row from the caller's vec. `iter` yields `&Message`. `filter` keeps the borrow when the closure returns true, and the closure reads `role` and compares it to the literal `"scratch"`. A role of `user` stays, and a role of `assistant` stays, and a role of `scratch` goes. A user message whose text contains the word scratch stays, because the comparison is the role field, not a search of `text`.

`.cloned()` turns `&Message` into `Message` by `Clone`, and both strings are copied. `.collect()` builds a new `Vec<Message>`. Iterator order is slice order, and `filter` doesn't reorder, so the kept rows stay in the order they had. The new vec is shorter by the number of scratch rows, and the caller's slice is unchanged.

The order test builds three messages and checks the two that remain.

```rust
#[test]
fn filter_keeps_order() {
    let messages = vec![
        message("user", "a"),
        message("scratch", "b"),
        message("assistant", "c"),
    ];
    let filtered = filter_transcript(&messages);
    assert_eq!(filtered[0].text, "a");
    assert_eq!(filtered[1].text, "c");
}
```

`message` is the test helper that sets `role` and `text` with `.into()`. Index 0 of the result is `a`. Index 1 is `c`. `b` was the scratch row and is absent. The length is 2 because the asserts index `[1]` and the vec was built from two survivors. The helper is:

```rust
fn message(role: &str, text: &str) -> Message {
    Message {
        role: role.into(),
        text: text.into(),
    }
}
```

That is the whole policy, and one role. A new vec, and the next owner is handed that vec, not the slice the caller still holds.

## 30.3 `handoff` filters, then counts, then writes the owner

`Board::handoff` is the transfer.

```rust
    pub fn handoff(&self, next: &str, messages: &[Message]) -> Result<Vec<Message>, HandoffError> {
        let filtered = filter_transcript(messages);
        if filtered.is_empty() {
            return Err(HandoffError::Empty);
        }
        let current = self.count.fetch_add(1, Ordering::Relaxed);
        if current >= MAX_HANDOFFS {
            self.count.fetch_sub(1, Ordering::Relaxed);
            return Err(HandoffError::Limit);
        }
        *self.owner.lock().expect("owner") = next.to_string();
        Ok(filtered)
    }
```

> **Rust sidebar: `Result`**
>
> - The return type is `Result<Vec<Message>, HandoffError>`. `Err` carries `Empty` or `Limit`. On the error paths the local `filtered` is dropped with the stack frame, and the caller does not receive it.
> - `return Err(...)` skips the owner write.
> - Callers in the tests use `unwrap` on the success path and `matches!` on the error path.
> - `Option`, `Result`, enums, and `Arc` are taught in [chapter 1](chapter-01-first-contact.md).

The first statement filters, and the empty check is next, before any atomic update, and a transcript of only scratch produces an empty vec, and the function returns `Empty`. `fetch_add` doesn't run. `count` stays 0, and the owner stays the name from `new`. The test `an_empty_filtered_transcript_does_not_move_ownership` builds that case: one scratch message, `handoff` to `draft`, expects `Empty`, expects owner `outline`, expects count 0.

When the filtered vec has at least one message, `fetch_add(1, Ordering::Relaxed)` increments and returns the old count into `current`. The first successful call sees `current` of 0 and leaves the atomic at 1, and the fourth successful call sees `current` of 3 and leaves the atomic at 4. `3 >= 4` is false, so the fourth call is allowed. `MAX_HANDOFFS` is 4, and the check is on the previous value, so four accepted handoffs fit. The call that sees `current` of 4 has already moved the atomic to 5. `4 >= 4` is true. `fetch_sub(1)` puts the atomic back to 4, and the function returns `Limit`, and the owner line doesn't run.

`*self.owner.lock().expect("owner") = next.to_string()` locks, writes a new `String` through the guard, and drops the guard at the end of the statement. `next` is `&str`. `to_string` owns it, and the dereference `*` assigns through `MutexGuard<String>`, which implements `DerefMut`.

The success test walks the cap.

```rust
#[test]
fn scratch_is_dropped_and_the_fifth_handoff_is_refused() {
    let board = Board::new("outline");
    let messages = vec![
        message("user", "Write a note."),
        message("scratch", "draft junk"),
        message("assistant", "three steps"),
    ];
    let filtered = board.handoff("draft", &messages).unwrap();
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|message| message.role != "scratch"));
    assert_eq!(board.owner(), "draft");
    for index in 1..MAX_HANDOFFS {
        board.handoff(&format!("agent-{index}"), &messages).unwrap();
    }
    assert_eq!(board.count(), MAX_HANDOFFS);
    assert!(matches!(
        board.handoff("one-more", &messages),
        Err(HandoffError::Limit)
    ));
    assert_eq!(board.owner(), "agent-3");
}
```

The first call's filtered length is 2, and the scratch text `draft junk` is gone. `all` checks every remaining role, and the owner is `draft`. The loop `1..MAX_HANDOFFS` is `1..4`, so the indexes are 1, 2, and 3, and those three calls succeed. The owners in order after the first are `agent-1`, `agent-2`, and `agent-3`, and the count is 4. The next call passes `one-more`, and it gets `Limit`. The owner is still `agent-3`, because the `Limit` path returns before the assignment, and four accepted transfers, then a refusal. The fifth attempt is the one that sees `current >= 4`.

`fetch_add` returns a unique previous value to each caller, and on one thread the story is the loop above. The tests don't spawn a second thread against `Board`, and the owner mutex and the atomic are still two steps. The source order on this thread is the order the test observes.

Hermes writes a pending row and lets a watcher claim it, and this function writes the owner string in the same call that accepts the count. The recorded transfer is the `Ok` and the new owner, and there is no model call between them. Error 4009 is the Hermes busy-session error, and it isn't a variant of `HandoffError`. This board doesn't know whether a session is running. Platform is required on the Hermes method. `handoff` takes `next` and `messages`. Chapter 39 is where the platform re-bind goes.

## 30.4 The demo shows the next owner and the filtered length

```rust
pub fn demo() -> String {
    let board = Board::new("outline");
    let messages = vec![
        Message {
            role: "user".into(),
            text: "Write a note.".into(),
        },
        Message {
            role: "scratch".into(),
            text: "draft junk".into(),
        },
        Message {
            role: "assistant".into(),
            text: "three steps".into(),
        },
    ];
    let filtered = board.handoff("draft", &messages).expect("handoff");
    format!("{} sees {}", board.owner(), filtered.len())
}
```

`Board::new("outline")` is the starting owner, and three messages: user, scratch, assistant. `handoff("draft", &messages)` filters to the user row and the assistant row, increments the count from 0 to 1, and sets the owner to `draft`. `expect("handoff")` panics if the `Result` is `Err`. This input isn't empty and the count was 0, so the `Result` is `Ok`. `format!` prints the owner and the filtered length, and the string is `draft sees 2`. The scratch text isn't in that string, and the length is the evidence the scratch row was dropped.

The test is the equality.

```rust
#[test]
fn the_demo_shows_the_next_owner() {
    assert_eq!(harness_core::handoffs::demo(), "draft sees 2");
}
```

The binary is `src/bin/chapter_demo.rs`.

```rust
use harness_core::handoffs::demo;

fn main() {
    println!("{}", demo());
}
```

The package binary is `ch30-handoffs`. `Cargo.toml` sets `name = "ch30-handoffs"` and `path = "src/bin/chapter_demo.rs"`, and the other binaries in the package are `ch30_mcp_fixture` and `sandbox-worker`. The run line passes `--bin ch30-handoffs`.

No key, and the three texts are literals. `expect` is the only panic path, and the demo input doesn't hit it.

## 30.5 `filter_100` times the copy the next owner receives

The bench file is `benches/handoffs.rs`, and the `[[bench]]` name is `handoffs` with `harness = false`. The workflow section is `30`, label `chapter 30 handoffs`, package `ch30-handoffs`.

```rust
fn filter_100(c: &mut Criterion) {
    let messages: Vec<Message> = (0..100)
        .map(|i| Message {
            role: if i % 5 == 0 {
                "scratch".into()
            } else {
                "user".into()
            },
            text: format!("message {i} about the rename procedure"),
        })
        .collect();
    c.bench_function("filter_100", |b| {
        b.iter(|| {
            let filtered = filter_transcript(black_box(&messages));
            black_box(filtered);
        });
    });
}

criterion_group!(benches, filter_100);
criterion_main!(benches);
```

The vec is built before `b.iter`. Indexes `0, 5, 10, ...` are role `scratch`, because `i % 5 == 0`, and the other indexes are role `user`. Every text is `message {i} about the rename procedure`, and one hundred messages. The timed closure calls `filter_transcript` and passes the slice through `black_box`, and the new vec goes through `black_box` so the clone stays in the sample. `Board` isn't constructed. `fetch_add` doesn't run, and the mean is the filter, which is the allocation the next owner receives.

The mean is `2.13 µs`, and it's `mean.point_estimate`.

`black_box` is `std::hint::black_box`, imported at the top of the bench file. It stops the compiler from deleting a result that the function doesn't otherwise use.

A handoff of this vec would also pay for the atomic and the owner lock. Those are not in the sample. The chapter's question for the bench is the cost of dropping scratch out of a hundred-row transcript. The cap is what the unit test walks, four successes and then `Limit`.

The iterator chain is lazy until `collect`. `filter` decides. `.cloned()` runs only for a row the predicate kept. A scratch row is borrowed long enough to read `role`, then dropped from the pipeline of the iterator. It isn't cloned. In `filter_100`, `i % 5 == 0` is true for 20 indexes in `0..100`: 0, 5, 10, and so on through 95. The other 80 rows are role `user` and are cloned, and each clone copies two `String`s, the role and the text. The mean `2.13 µs` is that work, repeated by `b.iter`, on the same input vec. The input vec isn't cleared, and the next iteration filters it again.

## 30.6 The count can briefly lie under contention

On one thread the five calls in `scratch_is_dropped_and_the_fifth_handoff_is_refused` are this table. `fetch_add` returns the previous value, and the owner write runs only when that previous value is below 4. Section 30.3 already walks the same function, so this section is the weak point, not a second tour.

| Call | `next` | Previous count | Atomic after the call | Owner after the call |
|---|---|---|---|---|
| 1 | `draft` | 0 | 1 | `draft` |
| 2 | `agent-1` | 1 | 2 | `agent-1` |
| 3 | `agent-2` | 2 | 3 | `agent-2` |
| 4 | `agent-3` | 3 | 4 | `agent-3` |
| 5 | `one-more` | 4 | 4, after `fetch_sub` | `agent-3` |

Call 4 is the last `Ok`. Call 5 has already moved the atomic to 5, then `fetch_sub` puts it back to 4 and returns `Limit` before the owner changes. The test asserts the owner, and it asserts `count()` before that refusal. The same three rows go in every time, scratch included, and the filter still drops `draft junk` on the call that refuses.

That undo is safe only because the tests call `handoff` from one thread. Two threads can both `fetch_add` before either decides to undo, so `count()` can briefly read 5, or higher, while the cap is 4. The final count comes back once each failed attempt subtracts, and a successful handoff isn't granted an extra slot by that window, but a reader of the atomic in the middle of the window sees a number the cap was supposed to forbid. I would not ship the add-then-undo under load. A `compare_exchange` loop reserves the slot only when the previous value is still below 4:

```rust
loop {
    let current = self.count.load(Ordering::Relaxed);
    if current >= MAX_HANDOFFS {
        return Err(HandoffError::Limit);
    }
    if self
        .count
        .compare_exchange(current, current + 1, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        break;
    }
}
```

That loop isn't the function in `handoffs.rs`. The crate keeps `fetch_add` and `fetch_sub`, and the tests never start the second thread that would show the window.

The empty path never enters the table. A transcript of only scratch returns `Empty` with the owner and the count still at the values from `new`.

```rust
#[test]
fn an_empty_filtered_transcript_does_not_move_ownership() {
    let board = Board::new("outline");
    let only_scratch = vec![message("scratch", "nope")];
    assert!(matches!(
        board.handoff("draft", &only_scratch),
        Err(HandoffError::Empty)
    ));
    assert_eq!(board.owner(), "outline");
    assert_eq!(board.count(), 0);
}
```

`HandoffError` is a plain enum. `expect("handoff")` still works, because `expect` is a method on `Result` and the error type doesn't have to implement `std::error::Error`. The tests use `matches!` so they can name `Empty` or `Limit` without panicking.

This crate still contains chapter 28's `hand_off`, a different function on a different type.

```rust
/// What the next stage is given. The pointer is the same allocation.
pub fn hand_off(context: Arc<Context>) -> Arc<Context> {
    context
}
```

`pipeline::hand_off` moves an `Arc<Context>` and leaves every role in that vec, scratch included. `Board::handoff` allocates a filtered vec and returns it inside a `Result`. The two functions answer different questions, and `handoffs::demo` builds its own `Vec<Message>` rather than calling `pipeline::run`.

`Ordering::Relaxed` doesn't publish the owner string, and the mutex does. On the test thread the statements run in source order, so `owner()` after a successful `handoff` sees the assignment. Hermes writes a pending row and lets a watcher re-bind. Error 4009 and the required `platform` field live there. This `Board` takes `next` and `messages`, and chapter 39 is where the platform comes back.

The demo's `Ok` value is two structs: role `user` with `Write a note.`, and role `assistant` with `three steps`. The dropped row is `scratch` / `draft junk`. `format!("{} sees {}")` is why the binary prints `draft sees 2` and nothing of the scratch text. The bench's `2.13 µs` is only the filter, on 80 kept rows, so a change to `fetch_add` doesn't move that mean.

## Tests

Four tests in `tests/handoffs.rs`.

Scratch is dropped, and two messages remain, and the owner becomes `draft`, and three more handoffs reach count 4. The next call is `Limit`, and the owner stays `agent-3`.

A transcript of only scratch is `Empty`, and the owner stays `outline`, and the count stays 0.

`filter_keeps_order` expects text `a` then text `c`.

`demo` equals `draft sees 2`.

The package also runs the tests copied forward from chapter 29. Nextest runs all of them.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch30-handoffs
```

```text
Starting 155 tests across 37 binaries
     Summary [   0.457s] 155 tests run: 155 passed, 0 skipped
```

```bash
cargo run --locked -p ch30-handoffs --bin ch30-handoffs
```

```text
draft sees 2
```

The binary filters one scratch line and moves the owner from `outline` to `draft`. It reads no key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `filter_100` | 2.13 µs | the transcript the next owner receives | drop role `scratch` from 100 messages |

`filter_100` builds the vec before `b.iter`, and the timed closure is `filter_transcript` and `black_box` of the new vec. The bench builds no board and calls no model.

The next owner is `draft`, and it sees 2 messages.

The means are `mean.point_estimate` from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `30`. Bench ids: `30/filter_100`, `28/filter_100`, and how to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`MAX_HANDOFFS` is 4, and the comment on the const says it isn't a Hermes constant. `handoff.request` doesn't define this number. Raising the const to match a Hermes key that isn't in that method would invent a source.

The filter compares `role` to `scratch`, and it doesn't search `text`. A user message that says scratch is kept, and a scratch message that holds the sentence you meant to pass on is dropped. The sentence the next agent must see needs a role other than `scratch`.

`Empty` returns before `fetch_add`, and a handoff of nothing doesn't burn one of the four. If the empty check moved below the increment, a scratch-only transcript would consume a slot and still return `Empty`. The source checks empty first.

The `Limit` path calls `fetch_sub`, and the atomic is back at 4 after the refusal. The owner write is below that return, so `one-more` doesn't become the owner. Forgetting the `fetch_sub` would leave the count one higher than the cap after a refused call. The source undoes the increment it just did.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md), and this chapter filters context on a transfer.

**Filter.** `filter_transcript` drops role `scratch` before the owner changes, and the next agent receives the `Vec` inside `Ok`. The caller's slice still holds the unfiltered rows. Hermes `handoff.request` writes pending and doesn't do this filter, and the filter is the adaptation.

**Cap.** `MAX_HANDOFFS` is 4. The call that observes a previous count greater than or equal to 4 returns `Limit` and restores the count with `fetch_sub`. The owner string changes only on `Ok`.

**Recorded transfer.** `handoff` doesn't call a model, and it filters, updates the count, and stores the next owner. That is the idea taken from a method that writes pending for a watcher.

**Left for later.** The gateway watcher, error 4009, and the required `platform` field are chapter 39. This board doesn't re-bind a platform.

## Chapter summary

- **Filter.** Role `scratch` is dropped, and the other rows stay, in order, and the result is a new vec.
- **Cap.** Four accepted handoffs, and the next is `Limit`. `MAX_HANDOFFS` is this harness, not a Hermes constant.
- **Owner.** Changes only after a non-empty filtered transcript is accepted. `Empty` and `Limit` leave the owner as it was.
- **Atomic.** `AtomicU32` with `Ordering::Relaxed`. `fetch_add` returns the previous count, and the refusal undoes itself.
- **Time.** `filter_100` mean `2.13 µs`, and that is `mean.point_estimate`.

## What you implement

`my-harness/tests/ch30.rs` is the milestone.

```rust
use my_harness::chapter_30_keeps_scratch;

#[test]
fn a_hundred_message_filter_drops_what_the_next_agent_must_not_see() {
    assert!(!chapter_30_keeps_scratch());
}
```

The stub panics.

```rust
pub fn chapter_30_keeps_scratch() -> bool {
    todo!("chapter 30")
}
```

`todo!("chapter 30")` is the panic, and the test wants `chapter_30_keeps_scratch()` to be false.

## What's next

Chapter 31 puts tasks on a queue of 100, and the 101st `schedule` returns `FanError::Full`. `gather` runs them on the supervisor pool, sorts by `task_index`, and returns the replies, and the tool trace stays off that string. Heartbeats are still chapter 36.
