# Chapter 35: Agents as Actors

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 34 can scan a context file, cut it to a head and a tail, wrap an untrusted page, and clone the snapshot `Arc`. The work is a function call on the caller's thread. Nothing in `preflight` or `share` owns a mailbox, accepts a steer while a turn is queued, or refuses the 65th message. A hundred agents would be a hundred calls the caller starts and joins by hand.

This chapter gives each agent a mailbox. `Inbox` holds `Mail` in a `VecDeque` and stops at `INBOX_CAP`, which is 64. The next `try_submit` returns `InboxError::Full` and the length stays 64. `submit` does the same check on a bounded `mpsc` channel of that capacity: `TrySendError::Full` becomes `InboxError::Full`, and `TrySendError::Closed` becomes `InboxError::Closed`. `step` applies one queued value. A steer is stored until a turn. An interrupt marks the next turn, and that turn returns `interrupted` without calling the model. `fan` spawns one tokio task per agent. `serve` reads the channel until the sender is dropped. `demo` returns `agent: ok`.

Restart is still ahead. A task that panics fails `join_next`. This chapter does not record a boot, does not trip a breaker, and does not start the child again. Chapter 36 is supervision.

The step crate is `steps/ch35-actors`. The package is `ch35-actors`. The library name is `harness_core`. This crate is chapter 34's harness copied forward, plus `actors`. `lib.rs` adds `pub mod actors` and still has `pub mod context` and `pub mod composition`. The top comment in `lib.rs` still says chapter 10. `Cargo.toml` describes the package as the previous harness plus this chapter. The `[dependencies]` table does not list `ch34-context`. There is no path dependency on the previous crate. `context` and `composition` in this tree are the copies.

The copied `inbox` module is still here. `SessionInbox` and `INBOX_BOUND` live in `inbox.rs`. `lib.rs` re-exports `inbox::InboxError` and `SessionInbox`. The new mailbox is `actors::Inbox`, `actors::Mail`, and `actors::InboxError`. The tests import those names from `harness_core::actors`. The two inboxes are different types.

| Section | Step | What it adds |
|---|---|---|
| 35.1 | `steps/ch35-actors` | `INBOX_CAP`, `Mail`, `InboxError`, `FakeModel` |
| 35.2 | `steps/ch35-actors` | `Inbox::try_submit` stops at 64 |
| 35.3 | `steps/ch35-actors` | `step` stores a steer and skips an interrupted turn |
| 35.4 | `steps/ch35-actors` | `submit` maps `try_send` |
| 35.5 | `steps/ch35-actors` | `serve` and `fan` |
| 35.6 | `steps/ch35-actors` | the demo and `inbox_submit` |

> **How Hermes does it**
>
> I read `gateway/turn_executor.py` (`_UnboundedThreadExecutor`) and `gateway/session_bot.py` (the module docstring) on Hermes `main` at `9e01b44`. I also read `agent/prompt_builder.py` (`STEER_MARKER_OPEN`, `STEER_MARKER_CLOSE`, `format_steer_marker`) while checking how a steer is delivered.
>
> Hermes has no actor inbox. `_UnboundedThreadExecutor` says one thread per submitted work item, no queue, no cap. The class docstring says `ThreadPoolExecutor(max_workers=None)` is not unbounded, and this executor exists so a turn does not sit in that silent queue. `session_bot.py` says the mailbox is a delivery receipt, not an execution queue. A committed canonical admission is the only consumer. I did not find a capacity of 64 on an agent mailbox in either file.
>
> `format_steer_marker` wraps steer text in `STEER_MARKER_OPEN` and `STEER_MARKER_CLOSE`. The opener says the block is an out-of-band user message, delivered once at that position. `CONTROL_FRAME_OPENERS` lists the frames Hermes treats as its own. That list is F7 in [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).
>
> **Adopted.** One worker per agent. Backpressure when the mailbox is full: the next send fails and the queue does not grow. A steer is kept until the turn that should see it.
>
> **Adapted.** The mailbox is `actors::Inbox` plus `tokio::sync::mpsc::channel(INBOX_CAP)`. Capacity is 64, the const `INBOX_CAP`. `submit` uses `try_send` and returns `InboxError::Full` or `InboxError::Closed`. `step` trims steer text, joins later steers with `\n`, and appends them to the model reply. `Mail::Interrupt` sets a flag. The next `Mail::Turn` returns `interrupted` and does not call `FakeModel::complete`. `fan` uses a `JoinSet`. The model reply is the scripted string. No socket is opened.
>
> **Deferred.** `STEER_MARKER_OPEN` is not wrapped around the steer. F7's control-frame list is not applied. The delivery-receipt mailbox in `session_bot.py` is not this queue. Chapter 36 restarts a failed child. This file does not.

[personal note: a queue that kept accepting work until the agent could not see the end of it]

## What this chapter decides

`INBOX_CAP` is 64. `try_submit` returns `Err(InboxError::Full)` when `queue.len()` is already 64, and it does not `push_back`. `submit` returns that same `Full` when `try_send` returns `TrySendError::Full`, and `Closed` when the receiver is gone.

`step` pops one `Mail`. `Steer` stores trimmed text, appending with a newline when a steer is already stored, and returns `None`. Blank text after `trim` is ignored. `Interrupt` sets `interrupted` and returns `None`. `Turn` while the flag is set clears the flag and the steer and returns `interrupted` without `complete`. A turn while the flag is clear calls `complete` and, when a steer is stored, appends it.

`serve` receives until the channel closes, `try_submit`s each mail, and `step`s immediately. `fan` starts `n` tasks. Each task submits one turn, drops the sender, and returns that task's reply. The vec is sorted. `demo` returns `agent: ok`.

## 35.1 The cap, the three mails, and a model that ignores the prompt

```rust
pub const INBOX_CAP: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mail {
    Turn(String),
    Steer(String),
    Interrupt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboxError {
    Full,
    Closed,
}
```

> **Rust sidebar: `enum`**
>
> - `Mail` is one message. `Turn` and `Steer` each hold a `String`. `Interrupt` holds nothing.
> - `InboxError` is `Full` or `Closed`. It derives `Copy`, so the error is copied into `assert_eq!` without a clone.
> - `PartialEq` is what `assert_eq!(..., Err(InboxError::Full))` compares. A `Full` does not equal a `Closed`.

`INBOX_CAP` is 64. The copied `inbox::INBOX_BOUND` is also 64. They are two consts. `SessionInbox::submit` stores `String` prompts and returns `inbox::InboxError::Full { bound }` when `queue.len() >= bound`. `actors::try_submit` stores `Mail` and returns `actors::InboxError::Full` when `queue.len() == INBOX_CAP`. The actors error has no `bound` field. `actors::InboxError` also has `Closed`, which `SessionInbox` does not, because `SessionInbox` is a mutex and a `VecDeque`, not a channel.

```rust
#[derive(Debug, Clone)]
pub struct FakeModel {
    reply: String,
    pub calls: u32,
}

impl FakeModel {
    pub fn new(reply: impl Into<String>) -> Self {
        Self {
            reply: reply.into(),
            calls: 0,
        }
    }

    pub fn complete(&mut self, _prompt: &str) -> String {
        self.calls += 1;
        self.reply.clone()
    }
}
```

`new` takes anything that converts into a `String`. The demo passes `"ok"`. `calls` starts at 0. `complete` increments `calls` and clones `reply`. The parameter is `_prompt`. The underscore means the prompt is unused. A turn's text is the argument `step` passes, and this model does not read it. Every successful `complete` returns the same scripted reply. `context::FakeModel` pops a queue of lines and can return `summary`. `composition::FakeModel` pops a queue and can return `""`. This one holds a single reply. `Interrupt`'s path never calls `complete`, so `calls` stays 0 on that test.

## 35.2 `try_submit` refuses the 65th message

```rust
#[derive(Debug)]
pub struct Inbox {
    queue: VecDeque<Mail>,
    steer: String,
    interrupted: bool,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            steer: String::new(),
            interrupted: false,
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn try_submit(&mut self, mail: Mail) -> Result<(), InboxError> {
        if self.queue.len() == INBOX_CAP {
            return Err(InboxError::Full);
        }
        self.queue.push_back(mail);
        Ok(())
    }
}

impl Default for Inbox {
    fn default() -> Self {
        Self::new()
    }
}
```

> **Rust sidebar: `Result`**
>
> - `Result<(), InboxError>` uses `()` as the success value. Success means the mail is now at the back of the queue. There is no ticket returned.
> - `Err(InboxError::Full)` returns the mail's ownership to the caller only in the sense that `mail` is dropped at the `return`. It was moved into `try_submit`. The queue does not contain it.
> - `Ok(())` runs only after `push_back`. The length check and the push are on `&mut self`, one caller at a time. `Inbox` is not wrapped in a `Mutex` in this module. Sharing it across tasks is the channel in `submit`, not a shared `Inbox`.

`new` starts with an empty queue, an empty steer, and `interrupted` false. `Default` calls `new`. `len` and `is_empty` read the queue. They do not look at `steer` or `interrupted`. After `step` consumes a `Steer`, the queue can be empty while `steer` holds text. `is_empty` is true in that state. The cap check uses `len`, so stored steer text does not consume a slot by itself. The slot was consumed when the `Steer` mail was pushed, and it was released when `step` popped it.

`try_submit` compares `len` with 64 using `==`. The queue grows by one `push_back`, so the length reaches 64 and does not jump over it. At 64 the function returns `Full` before `push_back`. The mail is dropped. The next read of `len` is still 64.

The test pushes `Mail::Turn(format!("job {index}"))` for `index` in `0..INBOX_CAP`, which is 64 turns. `len` is 64. The next call, `Mail::Turn("one more")`, equals `Err(InboxError::Full)`. `len` is still 64. The same check applies to `Steer` and `Interrupt`. A full queue refuses every variant. The test fills the queue with turns. The branch it locks is the length check, and the assertion that the length does not change is the backpressure.

`VecDeque::new` does not reserve 64. The cap is the `if`, not the allocation. `SessionInbox::new` uses `VecDeque::with_capacity(bound.min(64))`. `actors::Inbox::new` does not. Capacity of the allocation and `INBOX_CAP` are different. The refuse happens when `len` is 64, whatever the allocation's spare room.

## 35.3 `step` applies one mail

```rust
pub fn step(&mut self, model: &mut FakeModel) -> Option<String> {
    let mail = self.queue.pop_front()?;
    match mail {
        Mail::Steer(text) => {
            let cleaned = text.trim();
            if !cleaned.is_empty() {
                if self.steer.is_empty() {
                    self.steer = cleaned.to_string();
                } else {
                    self.steer.push('\n');
                    self.steer.push_str(cleaned);
                }
            }
            None
        }
        Mail::Interrupt => {
            self.interrupted = true;
            None
        }
        Mail::Turn(text) => {
            if self.interrupted {
                self.interrupted = false;
                self.steer.clear();
                return Some("interrupted".to_string());
            }
            let mut reply = model.complete(&text);
            if !self.steer.is_empty() {
                reply.push('\n');
                reply.push_str(&self.steer);
                self.steer.clear();
            }
            Some(reply)
        }
    }
}
```

> **Rust sidebar: `Option` from `?`**
>
> - `pop_front` returns `Option<Mail>`. On an empty queue it is `None`.
> - `?` on that `None` returns `None` from `step`. The function's return type is `Option<String>`, so an empty inbox and a steer both surface as `None`. The caller tells them apart by what it queued.
> - `Some(reply)` is a finished turn. `Some("interrupted")` is the skipped turn. Both are `Some`.

`Steer` trims leading and trailing whitespace. `cleaned` is a borrow of `text`. When `trim` yields an empty string, the inner `if` does not run. `steer` is unchanged, and the function returns `None`. The test's first mail is `"  "`. The first `step` returns `None` and does not store spaces.

When `cleaned` is non-empty and `steer` is empty, `steer` becomes that cleaned string. The second mail in the test is `"also this"`. `steer` is then `also this`. A later non-empty steer pushes a newline and then the new text. The third mail is `"and that"`. `steer` becomes `also this\nand that`. Each of those three `step` calls returns `None`. No model call has happened. `calls` is still 0 until the turn.

`Interrupt` sets `interrupted` to true and returns `None`. It does not clear `steer`. It does not increment a counter. A second `Interrupt` sitting behind the first sets the same `bool` to true again. One flag records that the next turn should be dropped. It does not record two drops.

`Turn` checks the flag first. When it is true, the flag becomes false, `steer` is cleared, and the function returns `Some("interrupted")`. `model.complete` is not called. The turn's `text` is dropped. A steer that was stored before the interrupt is discarded here, because `clear` runs before the return. The interrupt test queues `Interrupt` then `Turn("job")` on a fresh inbox, so `steer` was already empty. The first `step` returns `None`. The second returns `Some("interrupted")`. `model.calls` is 0. The scripted reply `"ok"` is never cloned.

When the flag is false, `complete(&text)` runs. `calls` becomes 1 on the first such turn, and `reply` starts as the model's string. If `steer` is non-empty, a newline and the steer are pushed onto `reply`, then `steer` is cleared. The test's fourth `step` returns `Some("ok\nalso this\nand that")`. `calls` is 1. A turn with an empty steer returns the model string alone. The demo is that path: the reply is `ok`, with no newline and no steer suffix.

Queue order is the order `step` sees. A `Turn` queued before an `Interrupt` is applied first, while `interrupted` is still false, so the model runs. The `Interrupt` applies on the following `step` and arms the flag for a later turn. The skip belongs to the next turn, not to a turn that already left the queue.

The steer test is four mails and four `step` calls, and each call takes one branch.

The inbox starts empty. `steer` is `""`. `interrupted` is false. `calls` is 0.

The first mail is `Steer("  ")`. `pop_front` yields it. `trim` on two spaces is `""`. `is_empty` on that cleaned string is true, so the assignment and the newline append are both skipped. `steer` is still `""`. The function returns `None`. The queue length is now 3.

The second mail is `Steer("also this")`. `trim` leaves `also this`. `steer` is empty, so the field becomes that string. No newline is pushed on this arm. The return is `None`. The queue length is 2.

The third mail is `Steer("and that")`. `trim` leaves `and that`. `steer` is not empty, so the `else` arm runs: `push('\n')` then `push_str("and that")`. The stored steer is the two lines `also this` and `and that`. The return is `None`. The queue length is 1. `complete` has not run. `calls` is still 0.

The fourth mail is `Turn("job")`. The flag is false, so the interrupt return does not run. `complete(&"job")` clones `ok` and sets `calls` to 1. `steer` is not empty, so `reply` becomes `ok`, then a newline, then `also this\nand that`. `clear` sets `steer` back to `""`. The return is `Some("ok\nalso this\nand that")`. The queue length is 0. A fifth `step` on this inbox would hit `pop_front` of `None` and return `None` through `?`, with `calls` still 1, because an empty queue does not call the model.

The interrupt inbox is a second `Inbox::new`. Its first mail is `Interrupt`. `step` sets `interrupted` to true and returns `None`. `calls` on that inbox's model is still 0. The second mail is `Turn("job")`. The flag is true, so `complete` is not called, the flag is set back to false, `steer.clear()` runs on the empty string, and the return is `Some("interrupted")`. The literal is not the model's `ok`. The turn text `job` is dropped with the `Mail::Turn` after the flag check. A following turn, if one were queued, would see `interrupted` false and would call `complete`. The test stops at the skipped turn.

`trim` only drops ends. A steer of `also this` with spaces in the middle keeps those spaces, because they are not at the ends. The test's second and third strings have no leading or trailing whitespace, so `cleaned` equals the queued text. The first string is only whitespace, which is the empty-after-trim branch.

`format_steer_marker` in `prompt_builder.py` would wrap the steer in `STEER_MARKER_OPEN` and `STEER_MARKER_CLOSE` and deliver it as a user row. `step` appends the trimmed text to the model reply. The marker strings are not in `actors.rs`. F7 is the catalog row for those frames. This function does not recognize `CONTROL_FRAME_OPENERS`.

## 35.4 `submit` is `try_send` with two error arms

```rust
pub fn submit(tx: &mpsc::Sender<Mail>, mail: Mail) -> Result<(), InboxError> {
    match tx.try_send(mail) {
        Ok(()) => Ok(()),
        Err(TrySendError::Full(_)) => Err(InboxError::Full),
        Err(TrySendError::Closed(_)) => Err(InboxError::Closed),
    }
}
```

> **Rust sidebar: `mpsc`**
>
> - `mpsc` means many producers, one consumer. `tokio::sync::mpsc::channel(INBOX_CAP)` returns a `Sender<Mail>` and a `Receiver<Mail>`. The `usize` argument is the buffer capacity, 64.
> - `try_send` does not wait. `Ok(())` means the mail is in the buffer or was handed to a waiting `recv`. `Err(TrySendError::Full(mail))` means the buffer is at capacity and no `recv` is waiting. The mail is inside the error. This match uses `_`, so that mail is dropped. It is not put back.
> - `Err(TrySendError::Closed(mail))` means the receiver has been dropped. Further sends fail the same way. The test that fills the channel keeps `_rx` bound so the receiver is alive and the error is `Full`, not `Closed`.
> - `Sender` is cheap to clone. Each clone is another producer. `fan` does not clone. Each task builds its own channel.

The channel test builds `mpsc::channel(INBOX_CAP)`, sends 64 `Mail::Turn("hello")` values, and expects the 65th to be `Err(InboxError::Full)`. Nothing calls `recv` during that test. The buffer fills to 64. `try_send` then fails. `Inbox::try_submit` is not used. The channel cap and the inbox cap are the same number, checked in two places. `serve` documents the pairing with `expect("the channel cap is the inbox cap")`.

`submit` borrows `tx` with `&Sender`. It does not need `&mut`. `try_send` takes `&self` on the sender. The mail is moved into the call. On `Full` and on `Closed` the value inside the `TrySendError` is discarded by `_`.

## 35.5 `serve` drains one channel, and `fan` starts `n` tasks

```rust
pub async fn serve(mut rx: mpsc::Receiver<Mail>, model: &mut FakeModel) -> Vec<String> {
    let mut inbox = Inbox::new();
    let mut replies = Vec::new();
    while let Some(mail) = rx.recv().await {
        inbox
            .try_submit(mail)
            .expect("the channel cap is the inbox cap");
        if let Some(reply) = inbox.step(model) {
            replies.push(reply);
        }
    }
    replies
}

pub async fn fan(n: usize) -> Vec<String> {
    let mut set = JoinSet::new();
    for index in 0..n {
        set.spawn(async move {
            let (tx, rx) = mpsc::channel(INBOX_CAP);
            submit(&tx, Mail::Turn(format!("job {index}"))).expect("empty inbox");
            drop(tx);
            let mut model = FakeModel::new(format!("ok {index}"));
            let replies = serve(rx, &mut model).await;
            replies.into_iter().next().unwrap_or_default()
        });
    }
    let mut replies = Vec::with_capacity(n);
    while let Some(joined) = set.join_next().await {
        replies.push(joined.expect("agent"));
    }
    replies.sort();
    replies
}
```

> **Rust sidebar: `async`**
>
> - `async fn` returns a future. The body runs when something `.await`s it. `serve` and `fan` are `async`. `step`, `try_submit`, `submit`, and `demo` are not. They run to completion on the caller.
> - `.await` on `rx.recv()` waits until a mail arrives or the channel closes. A closed empty channel yields `None`, and the `while let` ends. `serve` then returns the `replies` vec.
> - `tokio::test` on the two async tests starts a runtime for that test. `demo` does not need one.
> - `await` points are the places this future can pause: `recv`, `serve(...).await` inside `fan`, and `join_next`. `try_submit` and `step` between those points run without pausing.

> **Rust sidebar: `JoinSet`**
>
> - `JoinSet::new()` is a set of spawned tasks. `spawn` takes a future and starts it on the runtime.
> - `async move` moves `index` into the task. Each task has its own `index`, its own channel, and its own `FakeModel`.
> - `join_next().await` finishes whichever task completes next. `joined` is a `Result`. `expect("agent")` panics if the task panicked. The `Ok` value is the reply string.
> - The loop ends when the set is empty. `sort` orders the strings. Completion order is not index order. The test looks up `ok 0` and `ok 99` with `contains`, after the sort.

`serve` builds a fresh `Inbox` and an empty `replies` vec. Each `recv` that yields `Some(mail)` submits that mail and immediately `step`s. The inbox queue goes from 0 to 1 and, because `step` pops, back to 0 before the next `recv`. A `Steer` or an `Interrupt` makes `step` return `None`, and nothing is pushed to `replies`. A `Turn` pushes the `Some` string. The `expect` on `try_submit` panics if the inbox ever returns `Full`. With this pairing the local inbox holds at most one mail, and one is under 64. The cap the sender feels is the channel buffer in `submit`. The `expect` string is the reason those two numbers are the same const: a receiver that stepped more slowly than it submitted, or a channel created with a larger capacity, would be a different program. `fan` creates the channel with `INBOX_CAP` and submits one mail.

`drop(tx)` drops the only sender. After `serve` has received that one turn, the next `recv` is `None`. `serve` returns a vec of one reply. `into_iter().next()` takes it. `unwrap_or_default` would be `""` if `serve` returned an empty vec. A single `Turn` produces one `Some`, so `next` is that reply. The model was built as `ok {index}`. Task 0's reply is `ok 0`. Task 99's reply is `ok 99`.

`fan(100)` spawns 100 of those tasks. `replies.len()` is 100. The vec contains `ok 0` and `ok 99`. `sort` is lexicographic on the strings. `ok 10` sorts before `ok 2`, because the character `1` is before `2` after the common prefix `ok `. The test does not assert the full sorted order. It asserts the length and the two members.

One task, the one with `index` 0, runs this sequence. `mpsc::channel(64)` builds an empty buffer. `submit` of `Turn("job 0")` hits `try_send`, the buffer has room, and the match arm is `Ok(())`. `expect("empty inbox")` is the panic if that first send had been `Full` or `Closed`. The sender is then dropped. `FakeModel::new("ok 0")` sets `calls` to 0 and `reply` to `ok 0`. `serve` builds its own `Inbox` with an empty queue. `recv` returns the one turn. `try_submit` sees length 0, pushes the turn, and returns `Ok`. `step` pops it, the flag is false, `steer` is empty, `complete` returns `ok 0`, and `replies` inside `serve` becomes a vec of that one string. The next `recv` is `None` because the sender is gone and the buffer is empty. `serve` returns that vec. `next` takes `ok 0`. `join_next` later pushes that string into `fan`'s vec. The same sequence with `index` 99 submits `job 99` and returns `ok 99`. Nothing in the task body reads another task's inbox. A `Full` on task 0's channel would require 64 sends before a `recv`. `fan` sends once.

The channel test never calls `serve`. It keeps `_rx` so the receiver stays alive, then `submit`s `Turn("hello")` sixty-four times. Each `try_send` finds room. The sixty-fifth `submit` matches `TrySendError::Full` and returns `InboxError::Full`. The mail inside that error is the overflow turn, and the `_` pattern drops it. `Inbox::len` is not involved. If the test dropped `_rx` before the first `submit`, every `try_send` would match `TrySendError::Closed` and return `InboxError::Closed`, including the first, because a channel with no receiver is closed even when the buffer is empty. The test holds the receiver, so the error it asserts is `Full`.

`serve`'s `expect` is the other side of that number. Suppose a caller built `mpsc::channel(INBOX_CAP)` and filled it, and `serve` then received all 64 before stepping. The code does not do that. It steps inside the loop, after each `try_submit`. After a `Turn`, the queue is empty again. After a `Steer`, the queue is empty and `steer` holds the text. After an `Interrupt`, the queue is empty and the flag is true. The next mail, whatever it is, finds `len` of 0. The `expect` holds for the loop as written. A `step` that did not pop would let the queue grow until 64 and then panic on the next `try_submit`. `step` pops, including the empty-queue `?` only when `serve` calls it after a successful push, so the pop sees the mail just submitted.

`Vec::with_capacity(n)` reserves room for `n` replies. The length is still 0 until `push`. The reservation is not the inbox cap. `n` in the test is 100, which is larger than 64. Each task has its own channel of 64. The 100 tasks are 100 inboxes, not one inbox of 100. `_UnboundedThreadExecutor` starts a thread per item and does not keep a queue. `fan` starts a task per index and gives each task a bounded channel. The bound is per mailbox.

The channel-cap test is `tokio::test` and `async` because it constructs `mpsc::channel`, which is a tokio type, even though it never `.await`s a `recv`. The full-inbox test, the steer test, and the demo test are ordinary `#[test]` functions. They call `try_submit` and `step` and do not need a runtime.

## 35.6 The demo answers, and the bench submits one turn

```rust
pub fn demo() -> String {
    let mut inbox = Inbox::new();
    inbox.try_submit(Mail::Turn("hello".into())).expect("empty");
    let mut model = FakeModel::new("ok");
    let reply = inbox.step(&mut model).expect("a turn");
    format!("agent: {reply}")
}
```

`Inbox::new` is empty, so `try_submit` of `Turn("hello")` is `Ok(())`. `expect("empty")` panics on `Full` or `Closed`. A new inbox does not return those. `FakeModel::new("ok")` has `calls` of 0 until `step`. The queue holds one turn, `interrupted` is false, and `steer` is empty. `step` calls `complete`, gets `ok`, skips the steer append, and returns `Some("ok")`. `expect("a turn")` panics on `None`. A `Turn` returns `Some`. `format!` builds `agent: ok`.

```rust
#[test]
fn the_demo_answers() {
    assert_eq!(harness_core::actors::demo(), "agent: ok");
}
```

The binary prints that line.

```rust
use harness_core::actors::demo;

fn main() {
    println!("{}", demo());
}
```

I ran `cargo run -p ch35-actors --bin ch35-actors --offline`. The process printed `agent: ok`. The package also builds `ch35_mcp_fixture` and `sandbox-worker`. A `cargo run -p ch35-actors --offline` without `--bin` asks which binary. The example `demo` is the copied eval runner. It calls `run_saved_evals` and `run_stress_evals`. It does not call `actors::demo`.

The bench is `benches/actors.rs`. `Cargo.toml` names it `actors` with `harness = false`.

```rust
fn inbox_submit(c: &mut Criterion) {
    c.bench_function("inbox_submit", |b| {
        b.iter_batched(
            || mpsc::channel::<Mail>(INBOX_CAP),
            |(tx, rx)| {
                submit(black_box(&tx), black_box(Mail::Turn("hello".into()))).unwrap();
                black_box(rx);
            },
            BatchSize::SmallInput,
        );
    });
}
```

`iter_batched` runs the setup closure outside the timed sample. Each setup builds a new channel of capacity 64. The timed closure `try_send`s one `Mail::Turn("hello")` through `submit` and keeps `rx` alive with `black_box`. One send on an empty channel is `Ok`. The sample does not loop to 64, does not call `step`, and does not spawn `fan`. `BatchSize::SmallInput` is Criterion's batch size for a setup that allocates a channel per sample. The refusal at 64 is `a_full_inbox_refuses_the_next_message` and `the_channel_matches_the_inbox_cap`. The bench times a successful submit.

`unwrap` on `submit` panics on `Full` or `Closed`. The fresh channel is empty and `rx` is still held, so the sample takes the `Ok` arm.

## Tests

Five tests in `tests/actors.rs`.

`a_full_inbox_refuses_the_next_message` pushes 64 turns, sees `len` of `INBOX_CAP`, refuses `one more` with `InboxError::Full`, and sees `len` still at `INBOX_CAP`.

`a_steer_is_appended_and_an_interrupt_skips_the_model` steps a blank steer, `also this`, `and that`, and a turn. The first three results are `None`. The turn is `ok\nalso this\nand that`. `calls` is 1. A second inbox steps `Interrupt` then a turn. The results are `None` and `interrupted`. `calls` is 0.

`one_hundred_agents_each_answer_once` awaits `fan(100)` and expects 100 replies, including `ok 0` and `ok 99`.

`the_channel_matches_the_inbox_cap` fills a channel of `INBOX_CAP` and expects `Full` on the next `submit`.

`the_demo_answers` expects `agent: ok`.

The package also runs the tests copied forward from chapter 34, including `context` and `composition`. Nextest runs the package.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch35-actors
```

```text
    Starting 181 tests across 42 binaries
     Summary [   0.382s] 181 tests run: 181 passed, 0 skipped
```

```bash
cargo run -p ch35-actors --bin ch35-actors --offline
```

```text
agent: ok
```

The binary submits one turn to an empty inbox and prints the model reply. No key. `fan` is not in this binary. The hundred agents run in the tokio test.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `inbox_submit` | @@MEAN:35/inbox_submit@@ | one `try_send` into an empty channel of 64 | `submit` of `Mail::Turn("hello")` |

Each sample gets a new channel from the batch setup. The timed closure sends one turn. It does not fill the inbox. The Mean cell is `mean.point_estimate` for id `35/inbox_submit` from this PR's Benchmarks run. The printed Criterion `time:` line is the slope.

### Milestone demo

The `cargo run` above. The line is `agent: ok`.

## Pitfalls

`harness_core::InboxError` is the re-export from `inbox.rs`. It is `Full { bound }`. `harness_core::actors::InboxError` is `Full` or `Closed`. The tests in `tests/actors.rs` import the actors names. A test that imports `InboxError` from the crate root will not match `actors::try_submit`.

`len` is the queue. A stored `steer` and a raised `interrupted` flag do not add to `len`. After a steer is applied, `is_empty` can be true while the next turn will still append that steer. The cap counts queued `Mail` values waiting for `step`.

`try_submit` drops the refused mail. The caller that still needs the text has to have kept a copy. `TrySendError::Full` also drops the mail, because `submit` matches it with `_`. Neither path pushes the 65th message.

`Interrupt` is one `bool`. Two interrupt mails before one turn still skip that one turn once. The second interrupt does not skip a second turn unless another `Interrupt` is queued after the flag was cleared. A `Turn` already at the front of the queue runs if `step` reaches it before the interrupt mail.

`complete` ignores the prompt. The reply depends on `FakeModel::new`, not on `Mail::Turn`'s string. `fan` puts the index in both the prompt (`job {index}`) and the reply (`ok {index}`). The assertion looks at the reply. A model that echoed the prompt would be a different `complete`.

`serve` calls `step` before the next `recv`, so the `VecDeque` inside `serve` stays at or under one. The `Full` a producer observes on that path comes from `submit` filling the channel. The `Full` the unit test observes comes from calling `try_submit` in a loop on one `Inbox`. Both caps are `INBOX_CAP`. They are two checks.

`fan` sorts strings. The order after `sort` is not numeric order of the index. `join_next` order is not index order either. `expect("agent")` panics the `fan` future if a task panics. This chapter does not catch that panic and start the task again.

`demo` uses `expect` on `try_submit` and on `step`. An inbox that was already full would panic in `demo` instead of returning a string. The demo starts from `Inbox::new`. The test expects `agent: ok`, which is the string built after both `expect`s succeed.

Hermes's turn executor has no queue and no cap. Copying that shape would accept every submit. The harness mailbox stops at 64. The receipt mailbox in `session_bot.py` is not a place to put `Mail::Turn`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Inbox backpressure.** `INBOX_CAP` is 64. `Inbox::try_submit` returns `InboxError::Full` when `queue.len()` is 64 and does not `push_back`. The test's length is 64 before the extra submit and 64 after it. `submit` returns `InboxError::Full` when the `mpsc` buffer of `INBOX_CAP` is full. The 65th `try_send` does not enter the channel. `InboxError::Closed` is the arm for a dropped receiver.

**Steer.** Non-empty trimmed steer text is stored and appended on the next turn that actually calls the model, joined with newlines. A whitespace-only steer is popped and not stored. The Hermes marker `STEER_MARKER_OPEN` is not applied. F7 stays in `agent/prompt_builder.py`.

**Interrupt.** `Mail::Interrupt` sets the flag. The next `Mail::Turn` returns `interrupted`, clears the flag and the steer, and does not call `FakeModel::complete`. The test's `calls` stays 0.

**Left for later.** Chapter 36 is supervision and restart. A panicked task fails `join_next`. This module does not record the failure or start the agent again. The unbounded thread-per-turn executor and the delivery-receipt mailbox stay as they are in the Hermes files cited above.

## What you implement

`my-harness/tests/ch35.rs` is the milestone. A full inbox does not grow.

```rust
use my_harness::chapter_35_full_inbox_does_not_grow;

#[test]
fn a_full_inbox_does_not_grow() {
    assert!(chapter_35_full_inbox_does_not_grow());
}
```

The stub panics.

```rust
pub fn chapter_35_full_inbox_does_not_grow() -> bool {
    todo!("chapter 35")
}
```

`todo!("chapter 35")` is the panic. The test wants the function to return true when an inbox that already holds `INBOX_CAP` messages refuses the next submit and keeps the same length. The step crate is the reference. Do not copy it into `my-harness/`.

## Chapter summary

- **Cap.** `INBOX_CAP` is 64. `try_submit` and `submit` both return `Full` at that capacity. The queue length stays 64.
- **Mail.** `Turn` calls the model. `Steer` waits for a turn. `Interrupt` drops the next turn before `complete`.
- **Tasks.** `fan` starts one tokio task per agent, each with its own channel of 64. One hundred tasks return one hundred replies.
- **Demo.** `agent: ok`.
- **Time.** Bench id `35/inbox_submit`. The Mean column is `mean.point_estimate` from this PR's Benchmarks run. The Criterion `time:` line is the slope.
- **Hermes.** `gateway/turn_executor.py` has no queue and no cap. `gateway/session_bot.py` describes a delivery receipt. This mailbox is the harness inbox.

[personal note: the sixty-fifth message was the one that had to wait outside]

## What's next

Chapter 36 is supervision and restart. A failed child can be started again, and a chain of restarts can trip a breaker. This chapter's `JoinSet` returns the panic from `join_next` and does not arm that breaker. Cancellation and the budget are already on the call from earlier chapters. Chapter 36 is where a failed actor is started again.
