# Chapter 35: Agents as Actors

The 65th message is the one that has to fail. An inbox can hold sixty-four turns, steers, and interrupts. The next submit has to come back full, because a mailbox that keeps accepting mail fills memory with work nobody is running. Backpressure at 64 is the payoff.

The demo is narrower than that cap. `serve` receives one mail, submits it, and calls `step` before the next `recv`, so the in-memory inbox holds at most one message while the demo runs. Nothing here runs while a turn is in flight, so a steer can't actually arrive mid-turn. The 64 cap is the channel bound plus the unit test that fills the queue by hand. A slow model that could pause inside `complete` is deferred. `FakeModel` returns one scripted line and ignores the prompt. This one holds a single reply, unlike chapter 33's and chapter 34's, on purpose.

Three mails share the mailbox. A turn is the job that calls the model. A steer is text held for the next turn that actually calls it. An interrupt sets a flag that skips that next turn and clears the steer. An interrupt that arrives before any turn is queued arms the next turn. A stale interrupt can then skip a later job that had nothing to do with it. Chapter 2 already used a capacity of 64 for one session. This chapter makes that bound the mailbox of an agent task. Chapter 39's chat gateway is the front door that submits here.

A steer is a correction the mailbox can hold until the next real turn. That is the shape a continuously improving harness needs while reflection already runs as a child task. The mailbox stores the steer for that next turn, and the skill write stays with reflection.

The step is a copy of chapter 34 plus `actors`, at `steps/ch35-actors`, package `ch35-actors`. `cargo run -p ch35-actors --bin ch35-actors --offline` prints `agent: ok`. The exercise is `my-harness/tests/ch35.rs`, and a full inbox there keeps the same length.

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

## What this chapter decides

`INBOX_CAP` is 64. `try_submit` returns `Err(InboxError::Full)` when `queue.len()` is already 64, and the mail never reaches `push_back`. `submit` returns that same `Full` when `try_send` returns `TrySendError::Full`, and it returns `Closed` when the receiver is gone.

`step` pops one `Mail`. A `Steer` stores trimmed text, appending with a newline when a steer is already stored, and returns `None`. Blank text after `trim` is ignored. An `Interrupt` sets `interrupted` and returns `None`. A `Turn` while the flag is set clears the flag and the steer and returns `interrupted` without calling `complete`. A turn while the flag is clear calls `complete` and, when a steer is stored, appends it.

`serve` receives until the channel closes, submits each mail, and steps immediately. `fan` starts `n` tasks. Each task submits one turn, drops the sender, and returns that task's reply, and the vec is sorted. `demo` returns `agent: ok`.

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

The cap and the two enums are the public shape of a mail. Enums, `Result`, and `Option` are in [chapter 1](chapter-01-first-contact.md), and the distinction that matters here is which inbox you are holding. `INBOX_CAP` is 64, and the copied `inbox::INBOX_BOUND` is also 64. The session mailbox and the actor mailbox share a number and remain two consts. `SessionInbox::submit` stores `String` prompts and returns `inbox::InboxError::Full { bound }` when `queue.len() >= bound`. `actors::try_submit` stores `Mail` and returns `actors::InboxError::Full` when `queue.len() == INBOX_CAP`. The actors error has no `bound` field. `actors::InboxError` also has `Closed`, which `SessionInbox` doesn't, because `SessionInbox` is a mutex and a `VecDeque`. A turn and an interrupt can't be confused, because they are different variants of the same enum.

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

`new` takes anything that converts into a `String`, the demo passes `"ok"`, and `calls` starts at 0. `complete` increments `calls` and clones `reply`. The parameter is `_prompt`, so the underscore records that a turn's text arrives here and this model ignores it. Every successful `complete` returns the same scripted reply. `context::FakeModel` pops a queue of lines and can return `summary`, and `composition::FakeModel` pops a queue and can return `""`. The interrupt path never calls `complete`, so `calls` stays 0 on that test.

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

`new` starts with an empty queue, an empty steer, and `interrupted` false. `len` and `is_empty` read the queue and leave `steer` and `interrupted` unread. After `step` consumes a `Steer`, the queue can be empty while `steer` still holds text. `is_empty` is true in that state. The cap check uses `len`, which means stored steer text doesn't consume a slot by itself. The slot was taken when the `Steer` mail was pushed, and it was released when `step` popped that mail.

`try_submit` compares `len` with 64 using `==`. The queue grows by one `push_back`, so the length reaches 64 and can't jump over it. At 64 the function returns `Full` before `push_back`, the mail is dropped, and the next read of `len` is still 64. `Ok(())` is the success value, and it runs only after that push. The length check and the push sit on `&mut self`, one caller at a time. `Inbox` isn't wrapped in a `Mutex` in this module. Tasks share mail through the channel in `submit`.

The test pushes `Mail::Turn(format!("job {index}"))` for `index` in `0..INBOX_CAP`, which is 64 turns, and `len` is then 64. The next call, `Mail::Turn("one more")`, equals `Err(InboxError::Full)`, and `len` is still 64. The same check applies to `Steer` and `Interrupt`, so a full queue refuses every variant. The branch it locks is the length check, and the assertion that the length stays 64 is the backpressure.

`VecDeque::new` reserves nothing for those 64 slots, so the cap is the `if`. `SessionInbox::new` uses `VecDeque::with_capacity(bound.min(64))`, and `actors::Inbox::new` leaves the deque to grow as mail arrives. The allocation's spare room and `INBOX_CAP` are different numbers. The refuse happens when `len` is 64, whatever spare room the allocation still has. The unit test is what fills this `VecDeque` by hand.

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

`pop_front` returns `Option<Mail>`, and on an empty queue the `?` returns `None` from `step`. The function's return type is `Option<String>`, so an empty inbox and a stored steer both surface as `None`. The caller tells them apart by what it queued. `Some(reply)` is a finished turn, `Some("interrupted")` is the skipped turn, and both are `Some`.

A `Steer` trims leading and trailing whitespace, and `cleaned` is a borrow of `text`. When `trim` yields an empty string, the inner assignment is skipped, `steer` stays as it was, and the function returns `None`. The test's first mail is `"  "`, so that first `step` returns `None` and stores no spaces. When `cleaned` is non-empty and `steer` is empty, `steer` becomes that cleaned string. The second mail, `"also this"`, leaves the field as `also this`. A later non-empty steer pushes a newline and then the new text. The third mail, `"and that"`, makes `steer` into `also this\nand that`. Each of those three `step` calls returns `None`. No model call has happened yet, and `calls` is still 0 until the turn.

`Interrupt` sets `interrupted` to true and returns `None`. It leaves `steer` where it was, and it doesn't increment a counter. A second `Interrupt` behind the first sets the same `bool` to true again. One flag records that the next turn should be dropped, and a second interrupt doesn't add a second drop.

`Turn` checks the flag first. When the flag is true it becomes false, `steer` is cleared, and the function returns `Some("interrupted")` without calling `model.complete`. The turn's text is dropped, and a steer stored before the interrupt is discarded by that `clear`. The interrupt test queues `Interrupt` and then `Turn("job")` on a fresh inbox, where `steer` was already empty. The first `step` returns `None`, the second returns `Some("interrupted")`, `model.calls` is 0, and the scripted reply `"ok"` is never cloned. A later turn would see the flag clear and would call `complete`, and the test stops at the skipped turn.

When the flag is false, `complete(&text)` runs. `calls` becomes 1 on the first such turn, and `reply` starts as the model's string. If `steer` is non-empty, a newline and the steer are pushed onto `reply` and `steer` is cleared. The test's fourth `step` returns `Some("ok\nalso this\nand that")`, with `calls` at 1. A turn with an empty steer returns the model string alone. That is the demo's path, and the reply is `ok`, with no newline and no steer suffix. Another `step` on that empty queue returns `None` through `?` and leaves `calls` at 1. An empty queue doesn't call the model.

Queue order is the order `step` sees. A `Turn` queued before an `Interrupt` is applied first, while `interrupted` is still false, so the model runs. The `Interrupt` applies on the following `step` and arms the flag for a later turn. The skip belongs to the next turn, not to a turn that already left the queue. The same flag is why an interrupt queued before any turn can skip a later job. The flag waits, and the next `Turn` pays it.

`trim` only drops the ends. A steer of `also this` with spaces in the middle keeps those spaces, because they aren't at the ends. The test's second and third strings have no leading or trailing whitespace, so `cleaned` equals the queued text. The first string is only whitespace, which is the empty-after-trim branch.

`format_steer_marker` in `prompt_builder.py` would wrap the steer in `STEER_MARKER_OPEN` and `STEER_MARKER_CLOSE` and deliver it as a user row. `step` appends the trimmed text to the model reply. The marker strings aren't in `actors.rs`, and F7 is the catalog row for those frames. This function leaves `CONTROL_FRAME_OPENERS` unrecognized.

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
> `mpsc` means many producers and one consumer, and `tokio::sync::mpsc::channel(INBOX_CAP)` returns a `Sender<Mail>` and a `Receiver<Mail>` whose buffer holds 64. `try_send` returns at once. `Ok(())` means the mail is in the buffer or was handed to a waiting `recv`. `Err(TrySendError::Full(mail))` means the buffer is at capacity and no `recv` is waiting. This match uses `_`, so that mail is dropped. `Err(TrySendError::Closed(mail))` means the receiver has been dropped, and further sends fail the same way. The test that fills the channel keeps `_rx` bound, which is why the error is `Full`. `Sender` is cheap to clone, each clone is another producer, and `fan` doesn't clone one, because each task builds its own channel.

The channel test builds `mpsc::channel(INBOX_CAP)`, sends 64 `Mail::Turn("hello")` values, and expects the 65th to be `Err(InboxError::Full)`. Nothing calls `recv` during that test, so the buffer fills to 64 and the next `try_send` fails. `Inbox::try_submit` isn't on this path. The channel cap and the inbox cap are the same number, checked in two places. `serve` documents the pairing with `expect("the channel cap is the inbox cap")`. If the test dropped `_rx` before the first `submit`, every `try_send` would match `TrySendError::Closed` and return `InboxError::Closed`, including the first. A channel with no receiver is closed even when the buffer is empty. The test holds the receiver, so the error it asserts is `Full`.

`submit` borrows `tx` as `&Sender` and doesn't need `&mut`, because `try_send` takes `&self` on the sender. The mail moves into the call. On `Full` and on `Closed`, the value inside the `TrySendError` is discarded by `_`.

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
> An `async fn` returns a future, and the body runs when something `.await`s it. `serve` and `fan` can pause, while `step`, `try_submit`, `submit`, and `demo` run straight through on the caller. `.await` on `rx.recv()` waits until a mail arrives or the channel closes. A closed empty channel yields `None`, which ends the `while let` and lets `serve` return the `replies` vec. `tokio::test` starts a runtime for the two async tests, and `demo` doesn't need one. The future can pause at `recv`, at `serve(...).await` inside `fan`, and at `join_next`, while `try_submit` and `step` between those points run without pausing.

> **Rust sidebar: `JoinSet`**
>
> `JoinSet::new()` is a set of spawned tasks, and `spawn` takes a future and starts it on the runtime. `async move` moves `index` into the task, so each task has its own index, its own channel, and its own `FakeModel`. `join_next().await` finishes whichever task completes next, and `joined` is a `Result`. `expect("agent")` panics if that task panicked, and the `Ok` value is the reply string. The loop ends when the set is empty, and `sort` then orders the strings, because completion order isn't index order. The test looks up `ok 0` and `ok 99` with `contains` after the sort.

`serve` builds a fresh `Inbox` and an empty `replies` vec. Each `recv` that yields `Some(mail)` submits that mail and immediately steps. The inbox queue goes from 0 to 1 and, because `step` pops, back to 0 before the next `recv`. The local inbox holds at most one mail while this loop runs, and one is under 64. A `Steer` or an `Interrupt` makes `step` return `None`, and nothing is pushed onto `replies`. A `Turn` pushes the `Some` string. The `expect` on `try_submit` panics if the inbox ever returns `Full`. The cap the sender feels is the channel buffer in `submit`. That `expect` string is why the two caps are the same const. A receiver that stepped more slowly than it submitted, or a channel created with a larger capacity, would be a different program. `fan` creates the channel with `INBOX_CAP` and submits one mail.

`drop(tx)` drops the only sender. After `serve` has received that one turn, the next `recv` is `None`. `serve` returns a vec of one reply, and `into_iter().next()` takes it. `unwrap_or_default` would be `""` if `serve` returned an empty vec. A single `Turn` produces one `Some`, so `next` is that reply. The model was built as `ok {index}`, so task 0's reply is `ok 0` and task 99's reply is `ok 99`. Nothing in the task body reads another task's inbox. A `Full` on one task's channel would take 64 sends before a `recv`, and `fan` sends once.

`fan(100)` spawns 100 of those tasks, `replies.len()` is 100, and the vec contains `ok 0` and `ok 99`. `sort` is lexicographic on the strings, so `ok 10` sorts before `ok 2`. The character `1` comes before `2` after the common prefix `ok `. The test doesn't assert the full sorted order. It asserts the length and those two members.

`Vec::with_capacity(n)` reserves room for `n` replies, and the length stays 0 until `push`, so the reservation isn't the inbox cap. `n` in the test is 100, which is larger than 64, and each task has its own channel of 64. The 100 tasks are 100 inboxes, not one inbox of 100. `_UnboundedThreadExecutor` starts a thread per item and keeps no queue. `fan` starts a task per index and gives each task a bounded channel, and the bound is per mailbox.

The `expect` holds for the loop as written, because `step` runs inside the loop after each successful push. The next mail finds a `len` of 0 whether the previous mail was a turn, a steer, or an interrupt. A `step` that left the mail in the queue would let that queue grow to 64. The next `try_submit` would then panic. The channel-cap test is `tokio::test` and `async` because it constructs `mpsc::channel`, a tokio type, even though that test never `.await`s a `recv`. The full-inbox test, the steer test, and the demo test are ordinary `#[test]` functions, and they call `try_submit` and `step` without a runtime.

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

A new inbox is empty, so `try_submit` of `Turn("hello")` is `Ok(())`. `expect("empty")` is the panic if that send had been `Full` or `Closed`. `FakeModel::new("ok")` keeps `calls` at 0 until `step`. The queue holds one turn, `interrupted` is false, and `steer` is empty. `step` calls `complete`, gets `ok`, skips the steer append, and returns `Some("ok")`. `expect("a turn")` panics on `None`, a `Turn` returns `Some`, and `format!` builds `agent: ok`.

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

I ran `cargo run -p ch35-actors --bin ch35-actors --offline`, and the process printed `agent: ok`. The package also builds `ch35_mcp_fixture` and `sandbox-worker`, and a `cargo run -p ch35-actors --offline` without `--bin` asks which binary. The example `demo` is the copied eval runner. It calls `run_saved_evals` and `run_stress_evals`, and it leaves `actors::demo` uncalled.

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

`iter_batched` runs the setup closure outside the timed sample, and each setup builds a new channel of capacity 64. The timed closure sends one `Mail::Turn("hello")` through `submit` and keeps `rx` alive with `black_box`. One send on an empty channel is `Ok`. The sample doesn't loop to 64, doesn't call `step`, and doesn't spawn `fan`. `BatchSize::SmallInput` is Criterion's batch size for a setup that allocates a channel per sample. The refusal at 64 is `a_full_inbox_refuses_the_next_message` and `the_channel_matches_the_inbox_cap`, and the bench times a successful submit. `unwrap` on `submit` panics if the send is `Full` or `Closed`. The fresh channel is empty and `rx` is still held, so the sample takes the `Ok` arm.

## Tests

Five tests live in `tests/actors.rs`, and the package also runs the suites copied forward from chapter 34, including `context` and `composition`. Nextest runs the package.

`a_full_inbox_refuses_the_next_message` pushes 64 turns, sees `len` of `INBOX_CAP`, refuses `one more` with `InboxError::Full`, and sees `len` still at `INBOX_CAP`.

`a_steer_is_appended_and_an_interrupt_skips_the_model` steps a blank steer, `also this`, `and that`, and a turn. The first three results are `None`, the turn is `ok\nalso this\nand that`, and `calls` is 1. A second inbox steps `Interrupt` and then a turn, the results are `None` and `interrupted`, and `calls` is 0.

`one_hundred_agents_each_answer_once` awaits `fan(100)` and expects 100 replies, including `ok 0` and `ok 99`.

`the_channel_matches_the_inbox_cap` fills a channel of `INBOX_CAP` and expects `Full` on the next `submit`.

`the_demo_answers` expects `agent: ok`.

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

The binary submits one turn to an empty inbox and prints the model reply, and no key is read. `fan` isn't in this binary. The hundred agents run in the tokio test.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `inbox_submit` | 250.1 ns | one `try_send` into an empty channel of 64 | `submit` of `Mail::Turn("hello")` |

The means are `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `35`, with bench ids `35/inbox_submit` and `33/inbox_submit`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`complete` ignores the prompt, so the reply depends on `FakeModel::new` and not on the string inside `Mail::Turn`. `fan` puts the index in both the prompt, `job {index}`, and the reply, `ok {index}`. The assertion looks at the reply. A model that echoed the prompt would be a different `complete`.

`fan` sorts strings, so the order after `sort` isn't the numeric order of the index. `join_next` order isn't index order either. `expect("agent")` panics the `fan` future if a task panics. This chapter leaves that panic uncaught, and it doesn't start the task again.

`demo` uses `expect` on `try_submit` and on `step`. An inbox that was already full would panic in `demo`. The demo starts from `Inbox::new`, and the test expects `agent: ok`, which is the string built after both `expect`s succeed.

Hermes's turn executor has no queue and no cap, so copying that shape would accept every submit. The harness mailbox stops at 64. The receipt mailbox in `session_bot.py` is the wrong place to put a `Mail::Turn`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Inbox backpressure.** `INBOX_CAP` is 64. `Inbox::try_submit` returns `InboxError::Full` when `queue.len()` is 64 and does not `push_back`. The test's length is 64 before the extra submit and 64 after it. `submit` returns `InboxError::Full` when the `mpsc` buffer of `INBOX_CAP` is full. The 65th `try_send` does not enter the channel. `InboxError::Closed` is the arm for a dropped receiver.

**Steer.** Non-empty trimmed steer text is stored and appended on the next turn that actually calls the model, joined with newlines. A whitespace-only steer is popped and not stored. The Hermes marker `STEER_MARKER_OPEN` is not applied. F7 stays in `agent/prompt_builder.py`.

**Interrupt.** `Mail::Interrupt` sets the flag. The next `Mail::Turn` returns `interrupted`, clears the flag and the steer, and does not call `FakeModel::complete`. The test's `calls` stays 0.

**Left for later.** Chapter 36 is supervision and restart. A panicked task fails `join_next`. This module does not record the failure or start the agent again. The unbounded thread-per-turn executor and the delivery-receipt mailbox stay as they are in the Hermes files cited above.

## What you implement

`my-harness/tests/ch35.rs` is the milestone, and a full inbox there keeps its length.

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

`todo!("chapter 35")` is the panic. The test wants true when an inbox that already holds `INBOX_CAP` messages refuses the next submit and keeps the same length.

## Chapter summary

- **Cap.** `INBOX_CAP` is 64, `try_submit` and `submit` both return `Full` at that capacity, and the queue length stays 64.
- **Mail.** `Turn` calls the model, `Steer` waits for a turn, and `Interrupt` drops the next turn before `complete`.
- **Tasks.** `fan` starts one tokio task per agent, each with its own channel of 64. One hundred tasks return one hundred replies.
- **Demo.** `cargo run -p ch35-actors --bin ch35-actors --offline` prints `agent: ok`, which is one turn on an empty inbox.
- **Time.** `inbox_submit` mean `250.1 ns` is one `try_send` into an empty channel of 64, and that is `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).
- **Hermes.** `gateway/turn_executor.py` has no queue and no cap, `gateway/session_bot.py` describes a delivery receipt, and this mailbox is the harness inbox.

## What's next

Chapter 36 retries one completion after the child fails, with the bookkeeping around that second call. It doesn't start a new actor task, and the panic from `join_next` doesn't get a fresh mailbox. Chapter 43's cron chains a job by submitting to this inbox, and chapter 44's editor protocol turns one update into one submit. Chapter 47's bus keeps its own bound and delivers into this one, so 64 stays what "the agent is busy" means at those later doors.
