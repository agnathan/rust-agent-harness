# Chapter 30: Handoffs

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 29 can run specialists and take their replies back. It does not give the conversation to a different owner. This chapter records a handoff. The next owner sees the transcript with role `scratch` removed. A board may do this four times. The fifth call is `HandoffError::Limit`, and the owner string stays. The count is an `AtomicU32`. The filter runs before the count moves. Nothing in this module calls a model.

The step crate is `steps/ch30-handoffs`. The package is `ch30-handoffs`. The library name is `harness_core`. It is chapter 29's harness copied forward, plus `handoffs`. `lib.rs` adds `pub mod handoffs`. `pipeline` and `supervisor` are still in the crate, copied forward. There is no path dependency on `ch29-supervisor`.

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

[personal note: a handoff that passed along a scratch note you did not mean the next agent to see]

## What this chapter decides

`Board::new` stores an owner and an `AtomicU32` at 0. `handoff` filters first. An empty filtered vec is `HandoffError::Empty`. The count stays. The owner stays. Otherwise `fetch_add` increments. If the previous value is already `MAX_HANDOFFS`, `fetch_sub` undoes it and the error is `Limit`. The owner is replaced only after that check passes. The returned vec is the filtered one.

`filter_transcript` keeps order. The role it drops is `scratch`.

`demo` starts at `outline`, hands three messages to `draft`, and returns `draft sees 2`. No key.

`filter_100` times the filter on 100 messages. It does not call `handoff`. The mean is `MEAN_30_filter_100`.

## 30.1 The board is a count and an owner

The atomic import is the first line that is new relative to the supervisor.

```rust
use std::sync::atomic::{AtomicU32, Ordering};
```

`AtomicU32` is an unsigned 32-bit integer that `fetch_add` and `load` can update without a `Mutex` around the number. `Ordering` is the memory-ordering argument those methods require. This file uses `Ordering::Relaxed` on every atomic call.

The cap is a public const.

```rust
/// Harness cap. Not a Hermes constant.
pub const MAX_HANDOFFS: u32 = 4;
```

The doc comment is the warning. Hermes `handoff.request` does not count handoffs. Four is the number this harness refuses on. The type is `u32` because the atomic is an `AtomicU32`.

`Message` is the row the filter clones.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub text: String,
}
```

`Clone` is what `.cloned()` calls in the filter. `PartialEq` is what the tests use when they compare text. This `Message` is `handoffs::Message`. Chapter 28's `pipeline::Message` is a different struct with the same two fields. The filter does not call the pipeline.

The errors are an enum.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffError {
    Limit,
    Empty,
}
```

`Limit` means the cap. `Empty` means the filter removed every message. Neither variant carries a string. The caller matches the variant.

`Board` holds the two pieces of ownership.

```rust
pub struct Board {
    count: AtomicU32,
    owner: std::sync::Mutex<String>,
}
```

The count is atomic. The owner is a `Mutex<String>`. They are not one lock. `new` sets both.

```rust
impl Board {
    pub fn new(owner: impl Into<String>) -> Self {
        Self {
            count: AtomicU32::new(0),
            owner: std::sync::Mutex::new(owner.into()),
        }
    }
```

`impl Into<String>` means the caller can pass `String` or `&str`. The demo passes `"outline"`. `Into::into` builds the `String` the mutex stores. `AtomicU32::new(0)` is a count of zero. No handoff has been accepted.

`owner` and `count` are the readers.

```rust
    pub fn owner(&self) -> String {
        self.owner.lock().expect("owner").clone()
    }

    pub fn count(&self) -> u32 {
        self.count.load(Ordering::Relaxed)
    }
```

`owner()` locks, clones the `String`, and drops the guard at the end of the statement. The clone is why the returned name does not borrow the mutex. `expect("owner")` panics if the mutex is poisoned. `count()` loads the atomic. `Ordering::Relaxed` is the ordering: this load is atomic, so you do not observe a torn `u32`, and it does not synchronize other memory by itself. The tests call `count()` on one thread after `handoff` has returned, so the value they read is the value the same thread stored.

> **Rust sidebar: `AtomicU32`**
>
> - `fetch_add(1, Ordering::Relaxed)` adds one and returns the previous value. Two calls cannot receive the same previous value. The hardware update is the atomic part.
> - `fetch_sub` subtracts one and also returns the previous value. `handoff` ignores that return. It only needs the count to go back down.
> - `Relaxed` does not order this atomic with the owner mutex. The mutex has its own lock. The function's order is source order on one thread: filter, then `fetch_add`, then maybe `fetch_sub`, then the owner lock. The tests call `handoff` from that one thread.
> - `load` is the read `count()` uses. It takes the same `Ordering` argument. There is no `Mutex` around `count`.

## 30.2 `filter_transcript` drops scratch and keeps order

The filter is a free function. `handoff` calls it. The bench calls it. The test `filter_keeps_order` calls it without a board.

```rust
pub fn filter_transcript(messages: &[Message]) -> Vec<Message> {
    messages
        .iter()
        .filter(|message| message.role != "scratch")
        .cloned()
        .collect()
}
```

`messages` is a shared slice. The function cannot remove a row from the caller's vec. `iter` yields `&Message`. `filter` keeps the borrow when the closure returns true. The closure reads `role` and compares it to the literal `"scratch"`. A role of `user` stays. A role of `assistant` stays. A role of `scratch` goes. A user message whose text contains the word scratch stays, because the comparison is the role field, not a search of `text`.

`.cloned()` turns `&Message` into `Message` by `Clone`. Both strings are copied. `.collect()` builds a new `Vec<Message>`. Iterator order is slice order, and `filter` does not reorder, so the kept rows stay in the order they had. The new vec is shorter by the number of scratch rows. The caller's slice is unchanged.

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

That is the whole policy. One role. A new vec. The next owner is handed that vec, not the slice the caller still holds.

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
> - The return type is `Result<Vec<Message>, HandoffError>`. `Ok` carries the filtered transcript. `Err` carries `Empty` or `Limit`. The `Vec` is moved into `Ok` at the bottom. On the error paths the local `filtered` is dropped with the stack frame, and the caller does not receive it.
> - `return Err(...)` skips the owner write. That is why a refusal leaves the owner alone. The write is a later statement, and these returns never reach it.
> - `?` is not used here. The function matches its own conditions and returns the enum. Callers in the tests use `unwrap` on the success path and `matches!` on the error path.

The first statement filters. The empty check is next, before any atomic update. A transcript of only scratch produces an empty vec. The function returns `Empty`. `fetch_add` does not run. `count` stays 0. The owner stays the name from `new`. The test `an_empty_filtered_transcript_does_not_move_ownership` builds that case: one scratch message, `handoff` to `draft`, expects `Empty`, expects owner `outline`, expects count 0.

When the filtered vec has at least one message, `fetch_add(1, Ordering::Relaxed)` increments and returns the old count into `current`. The first successful call sees `current` of 0 and leaves the atomic at 1. The fourth successful call sees `current` of 3 and leaves the atomic at 4. `3 >= 4` is false, so the fourth call is allowed. `MAX_HANDOFFS` is 4, and the check is on the previous value, so four accepted handoffs fit. The call that sees `current` of 4 has already moved the atomic to 5. `4 >= 4` is true. `fetch_sub(1)` puts the atomic back to 4. The function returns `Limit`. The owner line does not run.

`*self.owner.lock().expect("owner") = next.to_string()` locks, writes a new `String` through the guard, and drops the guard at the end of the statement. `next` is `&str`. `to_string` owns it. The dereference `*` assigns through `MutexGuard<String>`, which implements `DerefMut`.

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

The first call's filtered length is 2. The scratch text `draft junk` is gone. `all` checks every remaining role. The owner is `draft`. The loop `1..MAX_HANDOFFS` is `1..4`, so the indexes are 1, 2, and 3. Those three calls succeed. The owners in order after the first are `agent-1`, `agent-2`, and `agent-3`. The count is 4. The next call passes `one-more`. It gets `Limit`. The owner is still `agent-3`, because the `Limit` path returns before the assignment. Four accepted transfers, then a refusal. The fifth attempt is the one that sees `current >= 4`.

`fetch_add` returns a unique previous value to each caller. On one thread the story is the loop above. The tests do not spawn a second thread against `Board`. The owner mutex and the atomic are still two steps. The source order on this thread is the order the test observes.

Hermes writes a pending row and lets a watcher claim it. This function writes the owner string in the same call that accepts the count. The recorded transfer is the `Ok` and the new owner. There is no model call between them. Error 4009 is the Hermes busy-session error. It is not a variant of `HandoffError`. This board does not know whether a session is running. Platform is required on the Hermes method. `handoff` takes `next` and `messages`. Chapter 39 is where the platform re-bind goes.

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

`Board::new("outline")` is the starting owner. Three messages: user, scratch, assistant. `handoff("draft", &messages)` filters to the user row and the assistant row, increments the count from 0 to 1, and sets the owner to `draft`. `expect("handoff")` panics if the `Result` is `Err`. This input is not empty and the count was 0, so the `Result` is `Ok`. `format!` prints the owner and the filtered length. The string is `draft sees 2`. The scratch text is not in that string. The length is the evidence the scratch row was dropped.

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

The package binary is `ch30-handoffs`. `Cargo.toml` sets `name = "ch30-handoffs"` and `path = "src/bin/chapter_demo.rs"`. The other binaries in the package are `ch30_mcp_fixture` and `sandbox-worker`. The run line passes `--bin ch30-handoffs`.

No key. The three texts are literals. `expect` is the only panic path, and the demo input does not hit it.

## 30.5 `filter_100` times the copy the next owner receives

The bench file is `benches/handoffs.rs`. The `[[bench]]` name is `handoffs` with `harness = false`. The workflow section is `30`, label `chapter 30 handoffs`, package `ch30-handoffs`.

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

The vec is built before `b.iter`. Indexes `0, 5, 10, ...` are role `scratch`, because `i % 5 == 0`. The other indexes are role `user`. Every text is `message {i} about the rename procedure`. One hundred messages. The timed closure calls `filter_transcript` and passes the slice through `black_box`. The new vec goes through `black_box` so the clone stays in the sample. `Board` is not constructed. `fetch_add` does not run. The mean is the filter, which is the allocation the next owner receives.

The mean token is `MEAN_30_filter_100`. It is `mean.point_estimate`. The Criterion `time:` line is the slope, not this mean.

`black_box` is `std::hint::black_box`, imported at the top of the bench file. It stops the compiler from deleting a result that the function does not otherwise use.

A handoff of this vec would also pay for the atomic and the owner lock. Those are not in the sample. The chapter's question for the bench is the cost of dropping scratch out of a hundred-row transcript. The cap is what the unit test walks, four successes and then `Limit`.

The iterator chain is lazy until `collect`. `filter` decides. `.cloned()` runs only for a row the predicate kept. A scratch row is borrowed long enough to read `role`, then dropped from the pipeline of the iterator. It is not cloned. In `filter_100`, `i % 5 == 0` is true for 20 indexes in `0..100`: 0, 5, 10, and so on through 95. The other 80 rows are role `user` and are cloned. Each clone copies two `String`s, the role and the text. The mean `MEAN_30_filter_100` is that work, repeated by `b.iter`, on the same input vec. The input vec is not cleared. The next iteration filters it again.

## 30.6 The count moves one accepted call at a time

The success test's five calls are a table of the atomic. `fetch_add` returns the previous value. The owner write runs only when that previous value is below 4.

| Call | `next` | Previous count | Atomic after the call | Owner after the call |
|---|---|---|---|---|
| 1 | `draft` | 0 | 1 | `draft` |
| 2 | `agent-1` | 1 | 2 | `agent-1` |
| 3 | `agent-2` | 2 | 3 | `agent-2` |
| 4 | `agent-3` | 3 | 4 | `agent-3` |
| 5 | `one-more` | 4 | 4, after `fetch_sub` | `agent-3` |

Call 4 is the last `Ok`. Previous count 3 fails the test `current >= MAX_HANDOFFS`. The atomic sits at 4. Call 5 reads previous count 4, which passes that test, subtracts one, and returns `Limit`. The owner cell does not change. `board.count()` after call 4 is 4, which is the assert in the test, placed before the refusing call. After call 5 the source has executed `fetch_sub`, so a later `count()` on that same board is 4 again. The test does not assert `count()` after the error. It asserts the owner.

The messages on every one of those calls are the same three rows: `Write a note.`, `draft junk`, and `three steps`. The filter result is the same two-row vec each time. Call 5 builds that vec and then drops it, because the `Err` path does not return it. The scratch row is removed even on the call that does not move the owner. The filter does not know whether the cap will refuse.

The empty path never enters the table.

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

One row, role `scratch`, text `nope`. `filter_transcript` collects an empty vec. `is_empty` is true. `HandoffError::Empty` comes back. `fetch_add` is not in the executed path. The owner lock is not taken. `outline` and `0` are the values `new` stored.

`HandoffError` is a plain enum. The file does not derive `thiserror` and does not implement `std::error::Error` for it. `expect("handoff")` in `demo` still works. `expect` is a method on `Result`. It panics with the provided message when the value is `Err`, and it does not need the error type to be an `Error`. The tests use `matches!` so they can name `Empty` or `Limit` without panicking.

This crate still contains chapter 28's `hand_off`. It is a different function, on a different type, in `pipeline.rs`.

```rust
/// What the next stage is given. The pointer is the same allocation.
pub fn hand_off(context: Arc<Context>) -> Arc<Context> {
    context
}
```

`pipeline::hand_off` moves an `Arc<Context>` and returns it. The message vec inside that context is the same allocation. It does not look at roles. A `scratch` message in that vec is still there after the call. `Board::handoff` takes a slice, allocates a filtered vec, and returns that vec inside a `Result`. The name in the demo string is the board's owner, not an `Arc`. Chapter 28 answers "is it the same pointer?" Chapter 30 answers "which rows does the next owner receive, and how many times may the owner change?"

The pipeline context and the handoff board are not wired together in this crate. `demo` in `handoffs` builds its own `Vec<Message>`. It does not call `pipeline::run`. A later chapter can put a filtered vec into a context. This chapter stops at the `Result`.

`Ordering::Relaxed` on the count means the atomic operations do not, by themselves, publish the owner string. The owner string is published by the mutex. On the test thread the statements run in source order, so the `assert_eq!` on `owner()` after `handoff` returns sees the assignment that happened before `Ok`. A second thread is not part of `tests/handoffs.rs`. I am not adding one. The implementation's atomic is there so the increment is a single update, and the tests exercise it from one caller.

The Hermes method this replaces as the idea, not as a port, writes pending and returns. The watcher does the re-bind. Error 4009 is that method's response when the session is running. Nothing in `handoffs.rs` returns the integer 4009. `platform` is required there. `Board::handoff`'s parameters are `next` and `messages`. Chapter 39 is the chapter that takes the platform back up. Until then, the recorded transfer is the filtered vec and the owner `String`.

The `Ok` value from the demo is worth writing out as two structs, because the length `2` is the only number the binary prints. The first kept `Message` has role `user` and text `Write a note.` The second has role `assistant` and text `three steps`. The dropped `Message` has role `scratch` and text `draft junk`. `filtered.len()` is 2 because those are the two clones `collect` produced. `board.owner()` is the `String` `draft` stored by the assignment. The format string `"{} sees {}"` puts a space, the word `sees`, and a space between them. No other punctuation. The scratch text is not interpolated. A reader of the terminal line knows the owner and the count of rows, and does not know the row text. The test `the_demo_shows_the_next_owner` compares that line and does not look at the notes a pipeline would have pushed. `handoffs::demo` does not call `pipeline::demo`.

`Clone` on `Message` is derived, so the clone is field-wise. `role.clone()` and `text.clone()` each allocate. A handoff that keeps two short rows allocates two small strings for roles and two for texts, plus the vec buffer. The bench scales that to 80 kept rows of a longer sentence, `message {i} about the rename procedure`. The chapter's mean for that scale is `MEAN_30_filter_100`, and it is only the filter. I keep the atomic out of the sample so a change to `fetch_add` does not move the filter's mean, and a change to `filter` does not hide inside the cap test. The cap test is the correctness check. It runs once. The bench runs the filter in `b.iter`.

`MAX_HANDOFFS` as a `u32` matches `AtomicU32`. Comparing `current >= MAX_HANDOFFS` does not need a cast. `count()` returns `u32` from `load`. The test's `assert_eq!(board.count(), MAX_HANDOFFS)` compares those two `u32` values. Four fits in a `u32` with room to spare. The undo uses `fetch_sub(1)`, not a store of a constant, so the atomic goes back by one from whatever the `fetch_add` left, which on this path is 5 going back to 4. I am describing that path for a count that was already 4. The test reaches that state through the four successes and then one refusal.

## Tests

Four tests in `tests/handoffs.rs`.

Scratch is dropped. Two messages remain. The owner becomes `draft`. Three more handoffs reach count 4. The next call is `Limit`. The owner stays `agent-3`.

A transcript of only scratch is `Empty`. The owner stays `outline`. The count stays 0.

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

The binary filters one scratch line and moves the owner from `outline` to `draft`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `filter_100` | MEAN_30_filter_100 | the transcript the next owner receives | drop role `scratch` from 100 messages |

`filter_100` builds the vec before `b.iter`. The timed closure is `filter_transcript` and `black_box` of the new vec. No board. No model.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, run BENCH_RUN_URL, section `30`. The id is `30/filter_100`. The old id `28/filter_100` is an alias in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. The printed Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The next owner is `draft`. It sees 2 messages.

## Pitfalls

`MAX_HANDOFFS` is 4. The comment on the const says it is not a Hermes constant. `handoff.request` does not define this number. Raising the const to match a Hermes key that is not in that method would invent a source.

The filter compares `role` to `scratch`. It does not search `text`. A user message that says scratch is kept. A scratch message that holds the sentence you meant to pass on is dropped. The sentence the next agent must see needs a role other than `scratch`.

`Empty` returns before `fetch_add`. A handoff of nothing does not burn one of the four. If the empty check moved below the increment, a scratch-only transcript would consume a slot and still return `Empty`. The source checks empty first.

The `Limit` path calls `fetch_sub`. The atomic is back at 4 after the refusal. The owner write is below that return, so `one-more` does not become the owner. Forgetting the `fetch_sub` would leave the count one higher than the cap after a refused call. The source undoes the increment it just did.

`Ordering::Relaxed` is the ordering on every atomic operation in this file. The owner update is a separate mutex. The tests are one thread. `count()` after `handoff` returns on that thread sees the update that thread made.

Error 4009 is the Hermes response when the session is running. `HandoffError` has `Limit` and `Empty`. A running session is not a state this `Board` stores. Platform re-bind is chapter 39.

`filter_transcript` allocates. `handoff` returns that allocation inside `Ok`. The caller's original vec still has the scratch rows. A caller who prints `messages` after a successful handoff still sees scratch. The next owner is supposed to read the `Ok` value.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). This chapter filters context on a transfer.

**Filter.** `filter_transcript` drops role `scratch` before the owner changes. The next agent receives the `Vec` inside `Ok`. The caller's slice still holds the unfiltered rows. Hermes `handoff.request` writes pending and does not do this filter. The filter is the adaptation.

**Cap.** `MAX_HANDOFFS` is 4. The call that observes a previous count greater than or equal to 4 returns `Limit` and restores the count with `fetch_sub`. The owner string changes only on `Ok`.

**Recorded transfer.** `handoff` does not call a model. It filters, updates the count, and stores the next owner. That is the idea taken from a method that writes pending for a watcher.

**Left for later.** The gateway watcher, error 4009, and the required `platform` field are chapter 39. This board does not re-bind a platform.

## Chapter summary

- **Filter.** Role `scratch` is dropped. The other rows stay, in order. The result is a new vec.
- **Cap.** Four accepted handoffs. The next is `Limit`. `MAX_HANDOFFS` is this harness, not a Hermes constant.
- **Owner.** Changes only after a non-empty filtered transcript is accepted. `Empty` and `Limit` leave the owner as it was.
- **Atomic.** `AtomicU32` with `Ordering::Relaxed`. `fetch_add` returns the previous count. The refusal undoes itself.
- **Time.** `filter_100` mean `MEAN_30_filter_100`. That is `mean.point_estimate`. The Criterion `time:` line is the slope.

[personal note: a transfer that was really a platform change, and the filter you still wanted]

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

`todo!("chapter 30")` is the panic. The test wants `chapter_30_keeps_scratch()` to be false. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 31 puts tasks on a queue of 100. The 101st `schedule` returns `FanError::Full`. `gather` runs them on the supervisor pool, sorts by `task_index`, and returns the replies. The tool trace stays off that string. Heartbeats are still chapter 36.
