# Chapter 47: The Message Bus

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 46 keeps a card in a JSON file. A second process can claim it, and the loser does not become the owner. Peers that are alive at the same time need a different shape. One of them publishes. The others receive. A slow receiver must not grow a queue without a limit. This chapter is that bus.

The step crate is `steps/ch47-message-bus`. The book number is 47. The package is `ch47-message-bus`. The library name is `harness_core`. This is the chapter 46 harness copied forward, including `kanban`, plus `bus`. `Cargo.toml` says "Chapter 47: the previous harness plus this chapter." Edition is 2021. `rust-version` is 1.88. There is no path dependency on `ch46-kanban`. The sources are copied. `lib.rs` adds `pub mod bus`. The file comment at the top of `lib.rs` still describes chapter 10's loop. `bus.rs` itself says book chapter 47. `kanban.rs` still says book chapter 46. The binary that prints this chapter is `src/bin/chapter_demo.rs`, named `ch47-message-bus`. `examples/demo.rs` is the inherited eval runner. It does not print the bus line.

| Section | What it walks |
|---|---|
| 47.1 | `BUS_CAP`, `MAX_BOT_EVENTS`, `Envelope`, `Sub` |
| 47.2 | `Bus`, `subscribe`, `bot_events`, `Default` |
| 47.3 | `Publish`, `publish`, `publish_bot` |
| 47.4 | `deliver` and `crate::redact::redact_for_egress` |
| 47.5 | `send_to` and `submit` |
| 47.6 | `FakeModel` and `demo` |
| 47.7 | the notify cursor, the bot-loop guard, and the one egress scrub |

> **How Hermes does it**
>
> I read `hermes_cli/kanban_db_notify.py` at `claim_unseen_events_for_sub` (line 352), `gateway/bot_loop_guard.py`, and `agent/redact.py` at `redact_for_egress` (line 1164) on Hermes `main` at `9e01b44`. I checked those lines.
>
> `claim_unseen_events_for_sub` claims inside `with _kb.write_txn(conn)`. The docstring says the row's `last_event_id` has already been advanced inside `BEGIN IMMEDIATE` when events are returned. `write_txn` in `hermes_cli/kanban_db_connect.py` is the transaction that runs that statement. At line 1273 it executes `BEGIN IMMEDIATE`. Inside the claim, `_notify_cursor` reads the row, `unseen_events_for_sub` lists events with `id > last_event_id` and does not advance the cursor itself, and `_cas_cursor` updates `last_event_id` only when the event list is non-empty. Two gateway watchers on the same board database serialize on SQLite's writer lock. The first claims a given range.
>
> `BotLoopGuardSettings` defaults `max_events` to 20, `window_seconds` to 300.0, and `cooldown_seconds` to 600.0. The module docstring says `{PLATFORM}_ALLOW_BOTS` only decides admission, so two Hermes profiles replying to each other never stop unless this guard counts them. `admit` returns `(allowed, state)`. The state strings in the docstring are `disabled`, `ok`, `tripped`, and `cooldown`. When the conversation is already cooling down, `admit` returns `False, "cooldown"`. When the deque for that conversation already holds `max_events` inside the window, `admit` sets `cooldown_until` to now plus `cooldown_seconds`, clears the deque, and returns `False, "tripped"`. The next call, while that deadline is still ahead, is the `"cooldown"` state. I am not collapsing those two strings into one. Both refuse the message. The window that fills them is a clock.
>
> `redact_for_egress` at line 1164 is the one scrub for text leaving the process. The docstring names chat platforms, A2A peers, and telemetry. The body calls `redact_sensitive_text(text, force=True)`, then, if `"earer"` remains in the text, replaces `_BEARER_RESIDUE_RE` with `Bearer [redacted]`. The regex wants a bracket token or 20 or more opaque characters. The comment above it says the 20-character floor is there so the English word "bearer" does not turn "the bearer of bad news" into `Bearer [redacted]`. The function fails closed. An exception returns `"[redaction-unavailable]"` and never the raw text.
>
> `bus.rs` does not define a function named `redact_for_egress`. I looked. `deliver` calls `crate::redact::redact_for_egress` before `try_send`. That is the chapter 20 function in `redact.rs`, re-exported from `lib.rs`. An earlier draft of this chapter described a second, narrower scrubber that only knew a bearer residue and an `sk-` token. That scrubber is gone. This file does not keep one.
>
> **Adopted.** A bounded `sync_channel` of 64. The 21st bot publish is `Cooldown` and is not sent. Every body that `publish` and `publish_bot` hand to `deliver` goes through the real egress function before `try_send`.
>
> **Adapted.** There is no wall-clock window and no SQLite cursor. Cooldown is a counter in this process, not 600 seconds. `publish` does not increment the counter. `publish_bot` increments it only after `Delivered`. The Rust `redact_for_egress` always runs the residue replace. It does not return `"[redaction-unavailable]"`, because the chapter 20 function has no catch. Hermes's Python function does.
>
> **Deferred.** The 300 second window and the notify cursor. A real socket is not this file. Chapter 49 is the call that leaves the process.

> **Guardrails.** E2 is called here. The name in the catalog is egress scrub. The narrow scrubber is gone. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

[personal note: a full channel that had to say slow]

## What this chapter decides

`subscribe` opens a `sync_channel` of `BUS_CAP` and keeps the name and the topic. `BUS_CAP` is 64.

`publish` calls `deliver`. `deliver` builds a new envelope whose body is `crate::redact::redact_for_egress` of the caller's body, then `try_send`s that envelope to every subscriber on the same topic. `Full` returns `Slow` with that subscriber's name and stops the loop. `Disconnected` names that subscriber too. A topic with no match returns `Delivered(0)`.

`send_to` is one named peer. A missing name is `Disconnected`. A full channel is `Slow`. `send_to` does not call the egress function. The body it queues is the body the caller passed.

`publish_bot` returns `Cooldown` when `bot_events` is already `MAX_BOT_EVENTS`. That call does not send. `MAX_BOT_EVENTS` is 20. The 21st bot publish is the one that cools down, after 20 delivered bot publishes. A bot publish that returns `Slow` or `Disconnected` does not increment the counter.

`submit` builds an `Envelope` and calls `publish`. The scrub happens inside `deliver`, not in a local function on the bus.

`demo` subscribes `researcher` on `work`, submits `ping`, receives it, asks `FakeModel` for `ok`, and returns `bus: ping ok`.

## The module

`steps/ch47-message-bus/crates/harness-core/src/bus.rs` is the whole new file. This is that file. Nothing in it is a redactor.

```rust
//! Book chapter 47. Peers publish on a bounded channel.
//! A full subscriber is backpressure. Twenty bot messages cool down.
//! Every body goes through `crate::redact::redact_for_egress` before `try_send`.

use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

pub const BUS_CAP: usize = 64;
/// Hermes `gateway/bot_loop_guard.py` admits 20 bot messages, then cools down.
pub const MAX_BOT_EVENTS: u32 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub topic: String,
    pub body: String,
}

struct Sub {
    name: String,
    topic: String,
    tx: SyncSender<Envelope>,
}

pub struct Bus {
    subs: Vec<Sub>,
    bot_events: u32,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            subs: Vec::new(),
            bot_events: 0,
        }
    }

    pub fn subscribe(
        &mut self,
        name: impl Into<String>,
        topic: impl Into<String>,
    ) -> Receiver<Envelope> {
        let (tx, rx) = sync_channel(BUS_CAP);
        self.subs.push(Sub {
            name: name.into(),
            topic: topic.into(),
            tx,
        });
        rx
    }

    pub fn bot_events(&self) -> u32 {
        self.bot_events
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Publish {
    Delivered(usize),
    Slow(String),
    Cooldown,
    Disconnected(String),
}

pub fn publish(bus: &mut Bus, envelope: Envelope) -> Publish {
    deliver(bus, envelope)
}

/// A bot-authored publish. The 21st in this process is `Cooldown` and is not sent.
pub fn publish_bot(bus: &mut Bus, envelope: Envelope) -> Publish {
    if bus.bot_events >= MAX_BOT_EVENTS {
        return Publish::Cooldown;
    }
    let result = deliver(bus, envelope);
    if matches!(result, Publish::Delivered(_)) {
        bus.bot_events += 1;
    }
    result
}

fn deliver(bus: &Bus, envelope: Envelope) -> Publish {
    let envelope = Envelope {
        topic: envelope.topic,
        body: crate::redact::redact_for_egress(&envelope.body),
    };
    let mut delivered = 0usize;
    for sub in &bus.subs {
        if sub.topic != envelope.topic {
            continue;
        }
        match sub.tx.try_send(envelope.clone()) {
            Ok(()) => delivered += 1,
            Err(TrySendError::Full(_)) => return Publish::Slow(sub.name.clone()),
            Err(TrySendError::Disconnected(_)) => {
                return Publish::Disconnected(sub.name.clone());
            }
        }
    }
    Publish::Delivered(delivered)
}

pub fn send_to(bus: &Bus, name: &str, envelope: Envelope) -> Publish {
    let Some(sub) = bus.subs.iter().find(|sub| sub.name == name) else {
        return Publish::Disconnected(name.to_string());
    };
    match sub.tx.try_send(envelope) {
        Ok(()) => Publish::Delivered(1),
        Err(TrySendError::Full(_)) => Publish::Slow(sub.name.clone()),
        Err(TrySendError::Disconnected(_)) => Publish::Disconnected(sub.name.clone()),
    }
}

pub fn submit(bus: &mut Bus, topic: &str, body: &str) -> Publish {
    publish(
        bus,
        Envelope {
            topic: topic.to_string(),
            body: body.to_string(),
        },
    )
}

pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

pub fn demo() -> String {
    let mut bus = Bus::new();
    let rx = bus.subscribe("researcher", "work");
    let published = submit(&mut bus, "work", "ping");
    assert_eq!(published, Publish::Delivered(1));
    let mail = rx.recv().expect("mail");
    let reply = FakeModel { reply: "ok" }.complete(&mail.body);
    format!("bus: {} {reply}", mail.body)
}
```

The module comment is the contract. A full subscriber is backpressure. Twenty bot messages cool down. Every body that reaches `try_send` through `deliver` has already been through `crate::redact::redact_for_egress`. The comment says `try_send`. `send_to` also calls `try_send`, and it does so on the caller's envelope. The comment describes the `deliver` path. The next sections walk each item in the order the file defines it.

## 47.1 `BUS_CAP`, `MAX_BOT_EVENTS`, `Envelope`, and `Sub`

```rust
pub const BUS_CAP: usize = 64;
/// Hermes `gateway/bot_loop_guard.py` admits 20 bot messages, then cools down.
pub const MAX_BOT_EVENTS: u32 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub topic: String,
    pub body: String,
}

struct Sub {
    name: String,
    topic: String,
    tx: SyncSender<Envelope>,
}
```

`BUS_CAP` is the channel bound passed to `sync_channel`. It is the same number as `INBOX_CAP` in chapter 44 and `INBOX_BOUND` in `inbox.rs`. It is not an alias of either constant. Changing the inbox bound does not change this channel. The tests and the bench both reach the channel through `subscribe`, which passes `BUS_CAP`. No caller passes a different capacity. A test that wants a smaller channel cannot, without editing `subscribe`.

`MAX_BOT_EVENTS` is a `u32`, not a `usize`. The doc comment names the Hermes file. The constant is 20, which is `BotLoopGuardSettings.max_events`. It is not `window_seconds` and it is not `cooldown_seconds`. Those two stay in Hermes. This process counts deliveries.

`Envelope` is the value on the channel. Both fields are public owned strings. `Clone` is what `deliver` uses when more than one subscriber matches. `PartialEq` is why a test can compare a `Publish` value, and why a received envelope could be compared field by field. The bus tests compare `Publish` and then read `.body`.

`Sub` is private. A caller cannot build one. The only constructor is `subscribe`. The name is the string `Slow` and `Disconnected` return. The topic is the filter. The `SyncSender` is the writing half. The `Receiver` is what `subscribe` returns to the caller, so the bus does not keep a way to read the queue. A subscriber that drops its receiver makes the next `try_send` on that sender return `Disconnected`.

### Rust: a private subscriber and a public envelope

`Sub` has no `pub`. `Envelope` does. The channel's type is `Envelope`, so the receiver the caller holds can read `topic` and `body`. The caller cannot reach into `Bus.subs` and rename a peer. `SyncSender<Envelope>` is `Clone` as well, but this file never clones the sender. It clones the envelope. One sender lives inside one `Sub` for the life of the bus.

## 47.2 `Bus`, `subscribe`, `bot_events`, and `Default`

```rust
pub struct Bus {
    subs: Vec<Sub>,
    bot_events: u32,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            subs: Vec::new(),
            bot_events: 0,
        }
    }

    pub fn subscribe(
        &mut self,
        name: impl Into<String>,
        topic: impl Into<String>,
    ) -> Receiver<Envelope> {
        let (tx, rx) = sync_channel(BUS_CAP);
        self.subs.push(Sub {
            name: name.into(),
            topic: topic.into(),
            tx,
        });
        rx
    }

    pub fn bot_events(&self) -> u32 {
        self.bot_events
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
```

`new` starts with no subscribers and a bot counter of 0. `Vec::new` does not allocate a channel. The first `subscribe` does.

`subscribe` takes `&mut self` because it pushes. The name and the topic are `impl Into<String>`, so the tests pass `&str` and the bench passes `format!("a{n}")`. `sync_channel(BUS_CAP)` returns a sender and a receiver with room for 64 undrained envelopes. With a bound of 64, the 65th `try_send` on a receiver that has not called `recv` is `Full`. The sender is stored. The receiver is returned. Order of `subs` is subscription order. `deliver` walks that order.

`bot_events` is a read. `publish_bot` is the only writer. A test can assert the counter without taking a channel lock, because the counter is a field, not a Hermes clock.

`Default` calls `new`. Nothing in `bus.rs` calls `Bus::default`. The tests call `Bus::new`. The impl is there so a `Bus` can be built the same way as other zero-state types in the crate.

### Rust: `sync_channel` and `try_send`

`std::sync::mpsc::sync_channel` is a bounded queue. `send` waits when the queue is full. `try_send` does not wait. It returns `Ok(())`, `Err(TrySendError::Full(value))`, or `Err(TrySendError::Disconnected(value))`. This file uses `try_send` on every path that writes. A slow subscriber is a `Publish` value, not a parked publisher. The `Full` and `Disconnected` arms drop the envelope that the error gave back. The `_` is that value. It is not queued somewhere else.

The receiver has to stay alive for `Ok`. The bench keeps the 100 receivers in the batched value until after `publish` returns. Dropping them first would be `Disconnected` for each matching name, and `deliver` would return on the first of those.

## 47.3 `Publish`, `publish`, and `publish_bot`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Publish {
    Delivered(usize),
    Slow(String),
    Cooldown,
    Disconnected(String),
}

pub fn publish(bus: &mut Bus, envelope: Envelope) -> Publish {
    deliver(bus, envelope)
}

/// A bot-authored publish. The 21st in this process is `Cooldown` and is not sent.
pub fn publish_bot(bus: &mut Bus, envelope: Envelope) -> Publish {
    if bus.bot_events >= MAX_BOT_EVENTS {
        return Publish::Cooldown;
    }
    let result = deliver(bus, envelope);
    if matches!(result, Publish::Delivered(_)) {
        bus.bot_events += 1;
    }
    result
}
```

`Publish` is the result of every send in this file. `Delivered` carries a count. That count is how many matching subscribers accepted the envelope on this call. It is not a running total. `Slow` and `Disconnected` carry the subscriber name. `Cooldown` carries nothing. There is one counter on the bus, not one counter per name.

`publish` takes `&mut Bus` and calls `deliver`, which only needs `&Bus`. The mutable borrow is not used to change `bot_events`. Ordinary publish does not count toward the cooldown. The signature still asks for `&mut` so a caller cannot hold a shared reference and publish while `publish_bot` wants to increment. The bench passes `&mut bus`.

`publish_bot` checks the counter first. If `bot_events` is already 20, the function returns `Cooldown` and the envelope is dropped with the stack frame. `deliver` is not called. No channel sees that body. The doc comment on the function says the 21st in this process is `Cooldown` and is not sent. "In this process" is the adaptation. Hermes's cooldown is 600 seconds on a conversation key. This counter is one `u32` on one `Bus`. A second `Bus` has its own zero.

When the counter is still under 20, `publish_bot` calls `deliver` and then looks at the result. `matches!(result, Publish::Delivered(_))` is true for `Delivered(0)` as well as `Delivered(1)` or `Delivered(100)`. A bot publish to a topic nobody subscribed still increments, because the result is `Delivered(0)`. A `Slow` does not increment. A `Disconnected` does not increment. The 21st successful shape is what the test locks: twenty `Delivered(1)` calls, then `Cooldown`, and `bot_events()` equals `MAX_BOT_EVENTS`. A 22nd call is `Cooldown` again. The counter does not move past 20 on the refusal.

Hermes `admit` would have returned `"tripped"` on the call that filled the window, and `"cooldown"` on a later call. This enum has one variant for the refusal. The call that finds the counter already at 20 is `Cooldown`. There is no separate `Tripped`. The message that would have been the 21st delivery is the one that is not sent. The previous 20 were sent.

### Rust: `matches!` and a counter that skips failures

`matches!` borrows `result` and does not move it. The function can still `return result` afterward. An `if let Publish::Delivered(_) = result` would also work. `matches!` is the form in the file. The `_` ignores the count. `Delivered(0)` and `Delivered(100)` both count as one bot event. The guard is "a delivery result," not "at least one peer."

The increment is `+= 1` on a `u32`. It happens after `deliver` returns, so a panic inside `try_send` would not leave the counter ahead of the channels. `try_send` does not panic on `Full`. It returns the error this match already handles.

## 47.4 `deliver` and the chapter 20 scrub

```rust
fn deliver(bus: &Bus, envelope: Envelope) -> Publish {
    let envelope = Envelope {
        topic: envelope.topic,
        body: crate::redact::redact_for_egress(&envelope.body),
    };
    let mut delivered = 0usize;
    for sub in &bus.subs {
        if sub.topic != envelope.topic {
            continue;
        }
        match sub.tx.try_send(envelope.clone()) {
            Ok(()) => delivered += 1,
            Err(TrySendError::Full(_)) => return Publish::Slow(sub.name.clone()),
            Err(TrySendError::Disconnected(_)) => {
                return Publish::Disconnected(sub.name.clone());
            }
        }
    }
    Publish::Delivered(delivered)
}
```

`deliver` is private. `publish` and `publish_bot` are the public doors. The first thing the function does is throw away the caller's body and keep the topic. The new body is `crate::redact::redact_for_egress(&envelope.body)`. The path is `crate::redact`, the module from chapter 20. It is not a function in this file. I searched `bus.rs` for `fn redact`. There is no `fn redact_for_egress` here.

The scrub runs once, before the loop. One hundred subscribers do not mean one hundred scrubs. They mean one scrub and one hundred clones of the scrubbed envelope. That is what the bench times when the body is `ping`. `ping` has nothing to mask, and the function still runs.

The loop skips a subscriber whose topic is not equal to the envelope topic. The comparison is `!=` on `String`. `work` does not match `other`. It also does not match `Work`. A skipped subscriber is not a failure. The loop continues. If every subscriber is skipped, `delivered` stays 0 and the function returns `Delivered(0)`. That is a successful publish to nobody. The test `a_different_topic_is_not_delivered` locks it: subscribe `peer` on `other`, publish topic `work` with body `ping`, expect `Delivered(0)`, and `try_recv` is an error. The body was still scrubbed. Nobody received it. `ping` scrubbed is `ping`.

`try_send` clones first. `SyncSender::try_send` takes the envelope by value. The clone is the one that moves. The original stays so the next matching subscriber can be cloned again. On `Ok`, the count goes up by one. On `Full`, the function returns `Slow` with `sub.name.clone()` immediately. Subscribers later in the vec are not offered this envelope. Subscribers earlier in the vec already have their clone. The return value is not `Delivered` of a partial count. The earlier sends are not rolled back. One full peer stops the fan-out and names itself. The test uses one subscriber, so the partial case does not show up there. The loop is what makes it true.

On `Disconnected`, the same early return names that subscriber. The envelope inside the error is dropped. A dropped receiver is how this arm happens. A name that was never subscribed does not reach `deliver`'s `Disconnected` arm. That case is `send_to`, below.

`Publish::Delivered(delivered)` is the fallthrough. The count is the number of `Ok` sends on this call.

### The chapter 20 function

`redact.rs` documents E1 and E2. `redact_sensitive_text` is E1. `redact_for_egress` is E2. This is the function `deliver` calls:

```rust
/// `redact_for_egress`. The sensitive-text scrub, then the Bearer residue sweep.
pub fn redact_for_egress(text: &str) -> String {
    let scrubbed = redact_sensitive_text(text);
    bearer_residue()
        .replace_all(&scrubbed, "Bearer [redacted]")
        .into_owned()
}
```

`redact_sensitive_text` runs three replaces, in this order. Prefix patterns first. Then env assignments. Then the `Authorization` header. The prefix replace keeps the characters through the first `-` or `_`, then writes `***`. The comment in `redact.rs` says short matches are fully masked and the prefix stays so a second pass does not see a new key. The env replace keeps the variable name and writes `=***`. The header replace keeps `Authorization:` or `Proxy-Authorization:` and the following spaces, and writes `***` in place of the scheme and the token.

The residue regex is the same shape as Hermes `_BEARER_RESIDUE_RE`:

```rust
Regex::new(r"(?i)\bBearer\s+(?:\[[^\]]+\]|[A-Za-z0-9._~+/-]{20,}=*)")
```

It runs on the text the three replaces already produced. A `Bearer` that the header replace already consumed is not there to match. The opaque alternative needs 20 or more characters from that class. A shorter token does not become `Bearer [redacted]`. The bracket alternative matches an already-masked residue such as `Bearer [redacted-jwt]` and folds it to the same marker.

The test `egress_scrubs_a_bearer_and_an_sk_token` calls `harness_core::redact_for_egress` for three strings, then publishes a fourth. These are the four outcomes.

The header string is `Authorization: Bearer sk-live-0123456789abcdef now`. The prefix pattern `sk-[A-Za-z0-9_-](?:\.?[A-Za-z0-9_-]){9,}` matches `sk-live-0123456789abcdef`. The first `-` is the prefix cut, so the match becomes `sk-***`. The text at that moment still has `Authorization: Bearer `. The header regex then matches `Authorization:` plus the scheme plus the token, and replaces that whole header value with `Authorization: ***`. The word `Bearer` is inside the span the header replace consumes. The residue sweep does not see it. The result contains `Authorization: ***`. It does not contain `sk-live`. It does not become `Bearer [redacted]`. The test asserts the two facts it cares about: no `sk-live`, and the string `Authorization: ***` is present.

The bare bearer string is `see Bearer abcdefghijklmnopqrstuvwxyz012345 now`. No prefix matches. No env assignment matches. No `Authorization:` header matches. The residue regex matches `Bearer` plus the 32-character token. Thirty-two is more than twenty. The replace writes `Bearer [redacted]`. The result contains `Bearer [redacted]`. It does not contain `abcdefghijklmnopqrstuvwxyz012345`.

The assigned string is `ghp_abcdefghijklmnopqrst OPENAI_API_KEY=supersecret`. The prefix `ghp_[A-Za-z0-9]{10,}` matches `ghp_abcdefghijklmnopqrst`. The first `_` is the prefix cut, so that match becomes `ghp_***`. The env regex then matches `OPENAI_API_KEY=supersecret` and writes `OPENAI_API_KEY=***`. The result contains `ghp_***` and `OPENAI_API_KEY=***`. It does not contain `ghp_abcdefghijklmnopqrst`. It does not contain `supersecret`. There is no `Bearer` in the string, so the residue sweep does not change it.

The published body is `see sk-live-0123456789abcdef and OPENAI_API_KEY=supersecret`. `publish` calls `deliver`, which calls the same function. The `sk-` pattern rewrites the key to `sk-***`. The env pattern rewrites the assignment to `OPENAI_API_KEY=***`. There is no `Authorization` header, so the header pass does not eat `sk-***`. The mail that `recv` returns contains `sk-***` and `OPENAI_API_KEY=***`. It does not contain `sk-live`. It does not contain `supersecret`. `Publish` is `Delivered(1)`. The scrubbed body is what sat on the channel.

`redacting_formatter` is not this path. It calls `redact_sensitive_text` only. A log line does not get the residue sweep. The bus does not call `redacting_formatter`. E2 is the call.

Hermes's function at line 1164 is the one I read, not a second copy I am inventing beside it:

```python
def redact_for_egress(text: str) -> str:
    """The one scrub for text leaving the process for a remote reader (chat platforms, A2A peers,
    telemetry). ``redact_sensitive_text(force=True)`` — the only secret-pattern list — plus a bearer
    sweep, because a ``Bearer <opaque>`` value with no vendor prefix carries no shape the prefix
    matcher can key on. Fails CLOSED: if the redactor raises, the raw text is never returned."""
    text = str(text or "")
    try:
        text = redact_sensitive_text(text, force=True)
    except Exception:
        return REDACTION_UNAVAILABLE
    if "earer" in text:
        text = _BEARER_RESIDUE_RE.sub("Bearer [redacted]", text)
    return text
```

`REDACTION_UNAVAILABLE` is `"[redaction-unavailable]"`. The `except` returns that marker. The chapter 20 Rust function has no `except`. A regex replace that finishes returns the scrubbed string. I am not going to pretend `bus.rs` added the fail-closed branch. It calls the Rust function that already exists.

### Rust: one replace, then the clones

The shadowing `let envelope = Envelope { ... }` drops the unscrubbed body when the new value is bound. The topic `String` is moved into the new struct. After that line, the only body in scope is the scrubbed one. `clone` on `Envelope` clones both strings. Topic `work` is four characters, cloned once per matching subscriber. Body `ping` is four characters, already scrubbed, cloned the same way. A secret would have been replaced before the first clone. Later subscribers cannot observe the raw body, because the raw body is gone before the loop.

## 47.5 `send_to` and `submit`

```rust
pub fn send_to(bus: &Bus, name: &str, envelope: Envelope) -> Publish {
    let Some(sub) = bus.subs.iter().find(|sub| sub.name == name) else {
        return Publish::Disconnected(name.to_string());
    };
    match sub.tx.try_send(envelope) {
        Ok(()) => Publish::Delivered(1),
        Err(TrySendError::Full(_)) => Publish::Slow(sub.name.clone()),
        Err(TrySendError::Disconnected(_)) => Publish::Disconnected(sub.name.clone()),
    }
}

pub fn submit(bus: &mut Bus, topic: &str, body: &str) -> Publish {
    publish(
        bus,
        Envelope {
            topic: topic.to_string(),
            body: body.to_string(),
        },
    )
}
```

`send_to` takes `&Bus`. It does not change the bot counter and it does not scrub. `find` walks `subs` for an equal name. The first match wins. Two subscribers with the same name would hide the second from this function. `subscribe` does not refuse a duplicate name. The tests use unique names.

A missing name hits the `else` and returns `Disconnected(name.to_string())`. That is the same variant as a dropped receiver, with a different cause. No `try_send` runs. The envelope the caller moved in is dropped. The test file does not call this arm. The function does. A name that was never passed to `subscribe` is `Disconnected`.

When the name is present, `try_send` moves the caller's envelope. There is no clone, because there is one sender. `Ok` is `Delivered(1)` even if the envelope's topic is not the subscriber's topic. `send_to` does not compare topics. `publish` does. A direct send is a name, not a topic filter.

`Full` is `Slow` with that name. The test `a_full_subscriber_is_slow_and_a_direct_send_names_it` fills the channel with `BUS_CAP` publishes of `"0"` through `"63"`, then one more `publish` of `"more"` expects `Slow("slow")`, then `send_to` of `"direct"` to `"slow"` expects `Slow("slow")` again. `try_recv` then yields body `"0"`, the first accepted body. `"more"` and `"direct"` were not queued. The channel still held 64 when `try_recv` ran, so the head was `"0"`.

`submit` is the small wrapper `demo` uses. It copies `topic` and `body` into an `Envelope` and calls `publish`. `publish` calls `deliver`. `deliver` scrubs. `submit` itself does not mention `redact`. Putting the scrub in `submit` and a different scrub in `publish` was the old shape. This file has one call, inside `deliver`.

### Rust: `let` `else` on the find

`let Some(sub) = ... else { return ... }` is a refutable binding. The `None` path leaves the function. The `Some` path binds `sub` for the `try_send`. `find` returns `Option<&Sub>`. `sub` is a shared reference. `try_send` only needs `&self` on the sender. `name.to_string()` on the missing path copies the `&str` the caller still owns. The caller's `Envelope` is not in that string.

## 47.6 `FakeModel` and `demo`

```rust
pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

pub fn demo() -> String {
    let mut bus = Bus::new();
    let rx = bus.subscribe("researcher", "work");
    let published = submit(&mut bus, "work", "ping");
    assert_eq!(published, Publish::Delivered(1));
    let mail = rx.recv().expect("mail");
    let reply = FakeModel { reply: "ok" }.complete(&mail.body);
    format!("bus: {} {reply}", mail.body)
}
```

`FakeModel` ignores the prompt. `let _ = prompt` keeps the argument from looking unused. The reply is `'static`. `demo` uses `ok`. The kanban module has its own `FakeModel`. This one is `harness_core::bus::FakeModel`. They are different types. The bench does not construct this one.

`demo` builds an empty bus, subscribes `researcher` on `work`, and submits topic `work` with body `ping`. `submit` publishes. `deliver` scrubs `ping`, which stays `ping`, and `try_send`s once. The `assert_eq!` wants `Delivered(1)`. A topic typo would panic inside `demo` instead of returning a different line. `recv` blocks until that one envelope is available. It is available. `expect("mail")` is the panic text if the sender were dropped first. The sender is inside the bus, which is still in scope.

`complete` is called with `&mail.body`. The body is `ping`. The reply is `ok`. The format string is `bus: {} {reply}`. The first placeholder is the body. `{reply}` is the named argument. The returned string is `bus: ping ok`. The test `demo_delivers_ping` expects that exact string.

`chapter_demo.rs` prints it:

```rust
use harness_core::bus::demo;

fn main() {
    println!("{}", demo());
}
```

`cargo run` needs `--bin ch47-message-bus`. The package also builds `ch47_mcp_fixture`, `sandbox-worker`, and `ch47_kanban_claim`. The example binary prints eval lines. No key is read. `recv` in `demo` is the blocking receive. The publish path that the bench measures is `try_send`, which does not block. `demo` is allowed to wait because it is the only receiver and the send already happened.

### Rust: the format of the demo line

`format!("bus: {} {reply}", mail.body)` is the whole return value. The colon and the spaces are literal. `mail.body` is the scrubbed body. For `ping` that is the same four characters. `{reply}` inserts `ok` with no extra punctuation. The test compares the full string. A change to the `Publish` count does not change the line if the assert still passes. A change to the reply changes the line. The bus is dropped at the end of `demo`. The receiver is dropped with it. The printed line is the only thing `main` keeps.

## 47.7 The notify cursor, the bot-loop guard, and the one egress scrub

`claim_unseen_events_for_sub` starts at line 352. The signature takes the connection, a task id, a platform, a chat id, an optional thread id, and optional kinds. The return is `(old_cursor, new_cursor, events)`. The docstring says that when events are returned, `last_event_id` has already been advanced inside `BEGIN IMMEDIATE`, so concurrent gateway watchers serialize on the writer lock and only the first claims a given range. Callers send the events, then leave the cursor or call `rewind_notify_cursor` on delivery failure.

The body is `with _kb.write_txn(conn)`. I read `write_txn`. When the connection is not already in a transaction, line 1273 runs `_execute_boundary_with_retry(conn, "BEGIN IMMEDIATE")`. The claim does not type `BEGIN IMMEDIATE` itself. The context manager does. Inside the block, `_notify_cursor` selects `last_event_id`. A missing row returns `0, 0, []`. `unseen_events_for_sub` selects `task_events` with `id > last_event_id`. Its own docstring says the cursor is not advanced there. If that list is empty, the claim returns the old cursor twice and an empty list, and `_cas_cursor` is not called. If the list is non-empty, `_cas_cursor` runs:

```python
"UPDATE kanban_notify_subs SET last_event_id = ? " + _SUB_KEY_WHERE + " AND last_event_id = ?"
```

The update is conditional on the previous id. That is the compare-and-swap. This chapter does not open SQLite for the bus. `kanban.rs` is still the chapter 46 JSON board, copied forward. `bus.rs` does not read `last_event_id`. A subscriber here is a channel, not a row. Two threads calling `publish` on one `Bus` would need a lock this file does not take. `publish` wants `&mut Bus`. The compiler serializes those calls. Hermes serializes them with `BEGIN IMMEDIATE`. The deferred piece is the cursor.

`BotLoopGuardSettings` is a frozen dataclass. The defaults I read are `enabled: bool = True`, `max_events: int = 20`, `window_seconds: float = 300.0`, `cooldown_seconds: float = 600.0`. `settings_from_config` reads `gateway.bot_loop_guard`. Unusable values keep the default. `admit` takes a conversation key. Under the lock it sweeps, then if `cooldown_until` for that key is still in the future it returns `False, "cooldown"`. Otherwise it drops timestamps older than `now - window_seconds`. If the deque length is already `max_events`, it sets the cooldown deadline, clears the deque, and returns `False, "tripped"`. Otherwise it appends `now` and returns `True, "ok"`. Disabled settings return `True, "disabled"` before the lock's counting work.

This crate's `publish_bot` has the 20 and not the clock. There is no `window_seconds`. There is no `cooldown_seconds`. There is no conversation key. Twenty delivered bot publishes, then every later `publish_bot` is `Cooldown` until the process drops the `Bus`. Restarting the process is the only reset, because `new` sets the counter to 0. Hermes would admit again after 600 seconds, and would also forget events that aged out of 300 seconds. Both of those are deferred.

The egress function is the one in section 47.4. Hermes names it the one scrub for text leaving the process. `deliver` uses that role for text leaving the publisher's stack frame and entering another subscriber's queue. The queue is still this process. The scrub is still E2, not a private list of two patterns. Prefix families, env assignments, the authorization header, and the bearer residue all run, because they live in `redact.rs` and `deliver` does not pick among them.

`inbox.rs` is still the chapter 2 queue. `INBOX_BOUND` is 64. `SessionInbox::submit` returns `InboxError::Full` when that queue is at the bound. `bus.rs` does not call `SessionInbox::submit`. A published envelope is not an inbox mail. The numbers match. The types do not.

### Rust: `&mut` publish, shared deliver

`publish` and `publish_bot` take `&mut Bus`. `deliver` and `send_to` take `&Bus`. The split matches what they mutate. `deliver` only reads `subs` and writes through `SyncSender`, which is safe from a shared reference. `publish_bot` writes `bot_events`, so it needs `&mut`. A caller who has published cannot call `bot_events` until that mutable borrow ends. The tests do the call after the publish expression has finished. The bench holds `&mut bus` only inside the timed closure.

`Envelope` is not `Copy`. Every send moves or clones it. `publish_bot` on the cooldown path moves the envelope into the function and drops it without cloning. That drop is the "not sent." The test does not recv after the cooldown call. The counter staying at 20 is the evidence the 21st did not increment, and the early return is the evidence it did not send.

## Tests

The new tests are `steps/ch47-message-bus/crates/harness-core/tests/bus.rs`. The package also builds the copied harness tests. The nextest line below is the package run.

`a_full_subscriber_is_slow_and_a_direct_send_names_it` builds a bus, subscribes `"slow"` on `"work"`, and publishes `BUS_CAP` envelopes. The loop variable `n` runs `0..BUS_CAP`, so the bodies are `"0"` through `"63"`. Each result is `Delivered(1)`. The next body is `"more"`. That result is `Slow("slow")`. `send_to` of an envelope named topic `"work"` and body `"direct"` to the name `"slow"` is `Slow("slow")` again. `rx.try_recv().unwrap().body` is `"0"`. The head of the queue is the first body. The two refused bodies are absent. The receiver was held for the whole test, so the `Slow` is `Full`, not `Disconnected`.

`the_twenty_first_bot_message_cools_down` subscribes `"peer"` on `"work"` and calls `publish_bot` with body `"ping"` in a loop of `MAX_BOT_EVENTS`. Each of those 20 matches `Delivered(1)`. The next `publish_bot` of `"ping"` equals `Cooldown`. `bus.bot_events()` equals `MAX_BOT_EVENTS`. `let _ = rx` drops the receiver after those asserts. During the 20 sends and the cooldown call, the receiver was still alive. The cooldown return happened before `deliver`, so it did not depend on the channel being full or disconnected. The 21st `"ping"` is not sitting behind the first 20.

`egress_scrubs_a_bearer_and_an_sk_token` is the four strings in section 47.4. The first three call `redact_for_egress` directly. The fourth goes through `publish` and `recv`. The header assertion is `!header.contains("sk-live")` and `header.contains("Authorization: ***")`. The bare bearer assertion is `bearer.contains("Bearer [redacted]")` and `!bearer.contains("abcdefghijklmnopqrstuvwxyz012345")`. The assigned assertion is no `ghp_abcdefghijklmnopqrst`, no `supersecret`, and `OPENAI_API_KEY=***` present. The mail assertion is no `sk-live`, no `supersecret`, `sk-***` present, and `OPENAI_API_KEY=***` present. `published` equals `Delivered(1)`.

`a_different_topic_is_not_delivered` subscribes `"peer"` on `"other"` and publishes topic `"work"`, body `"ping"`. The result is `Delivered(0)`. `rx.try_recv().is_err()` is true. The zero is a count, not a `Slow` and not a `Disconnected`.

`demo_delivers_ping` expects `demo()` to equal `bus: ping ok`.

The test module imports `demo`, `publish`, `publish_bot`, `send_to`, `Bus`, `Envelope`, `Publish`, `BUS_CAP`, and `MAX_BOT_EVENTS` from `harness_core::bus`, and `redact_for_egress` from `harness_core`. That last import is the re-export. The test does not import a scrubber out of `bus`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch47-message-bus
```

```text
    Starting 248 tests across 53 binaries
     Summary [   0.380s] 248 tests run: 248 passed, 0 skipped
```

```bash
cargo run -p ch47-message-bus --bin ch47-message-bus
```

```text
bus: ping ok
```

The binary publishes one envelope and prints the line. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `publish_to_agents` | MEAN_PENDING_47 | one small publish under 1 ms at 100 agents | `redact_for_egress` of `ping`, then `try_send` to 100 empty channels |

The function name is `publish_to_agents`. The bench id is `47/publish_to_agents`. The old ids `45/publish_to_agents` and `43/publish_to_agents` are aliases in `benchmarks/descriptions.json`. The function name did not change when the id moved.

```rust
fn publish_to_agents(c: &mut Criterion) {
    c.bench_function("publish_to_agents", |b| {
        b.iter_batched(
            || {
                let mut bus = Bus::new();
                let mut receivers = Vec::with_capacity(100);
                for n in 0..100 {
                    receivers.push(bus.subscribe(format!("a{n}"), "work"));
                }
                (bus, receivers)
            },
            |(mut bus, receivers)| {
                let result = publish(
                    black_box(&mut bus),
                    Envelope {
                        topic: "work".into(),
                        body: "ping".into(),
                    },
                );
                black_box((result, receivers));
            },
            BatchSize::SmallInput,
        );
    });
}
```

`iter_batched` runs the setup closure outside the timer. That closure builds a `Bus`, subscribes `a0` through `a99` on topic `work`, and keeps the 100 receivers. It is not timed. The timed routine calls `publish` with the envelope passed through `black_box`, then `black_box` of the result and the receivers. `redact_for_egress` of the body `ping` runs inside `deliver` before the 100 clones and the 100 `try_send` calls. `publish_bot` and `FakeModel` are not called. The `Slow` path is not taken. Every channel is empty, so the result is `Delivered(100)`. The bot-event counter stays 0. The descriptions file says the budget is under a millisecond, and that a number that fills 64 first would be the slow-subscriber test.

The mean is MEAN_PENDING_47. It is `mean.point_estimate` from this PR's Benchmarks workflow, section `47`. The id is `47/publish_to_agents`. The old ids `45/publish_to_agents` and `43/publish_to_agents` are aliases in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. `.github/scripts/run_bench_section.sh` runs `cargo bench -p ch47-message-bus --bench bus --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10` for that section. The same flags are in `steps/ch47-message-bus/README.md`. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `bus: ping ok`.

## Pitfalls

The bench is 100 successful sends of a body that scrubs to itself. `redact_for_egress` is inside the sample. A number that fills 64 first is the slow-subscriber test, not this one. A number that called `publish_bot` 21 times would return `Cooldown` and would not be this sample.

`publish` does not trip the cooldown. The counter moves only in `publish_bot`, and only on `Delivered`. A `Slow` bot publish leaves the counter where it was. Filling one subscriber and then calling `publish_bot` does not advance you toward `Cooldown`.

The receivers are part of the batched value so they are not dropped before `try_send`. Dropping them first would be `Disconnected`, which is a different result. `let _ = rx` in the cooldown test drops the receiver after the asserts. Moving that line above the loop would disconnect the channel, `deliver` would return `Disconnected`, and the counter would stay 0 because only `Delivered` increments. The 21st call would then still try to send.

`deliver` returns on the first `Full`. Earlier matching subscribers already received a clone. The return value does not say how many. Callers that need all-or-nothing will not get it from this loop.

`send_to` does not call `crate::redact::redact_for_egress`. A direct send of a secret queues the raw body. `publish` and `publish_bot` do not have that hole. The test's direct body is `"direct"`.

`Delivered(0)` is a different topic, or a bus with no subscribers. It is not backpressure. `Cooldown` is not backpressure either. The channel never saw the 21st bot body.

`demo` uses `recv`, which waits. The bench uses `try_send`, which does not. Measuring `demo` would include a receive and a format. This sample does not.

The Rust egress function does not return `"[redaction-unavailable]"`. Hermes's does, on an exception. A panic in the regex crate would still panic here. I am not describing a catch this file does not have.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Egress scrub (E2).** The catalog row is E2. The name is egress scrub. One stricter scrub for remote readers, and it also strips a bearer residue. The Hermes function is `agent/redact.py` `redact_for_egress`. This chapter calls the chapter 20 Rust function of that name, from `deliver`, before `try_send`. The narrow scrubber that used to live beside it is gone. `bus.rs` does not define one. The header `Authorization: Bearer sk-live-0123456789abcdef now` becomes a line that contains `Authorization: ***` and does not contain `sk-live`, and it does not become `Bearer [redacted]`, because the header replace consumes the scheme before the residue sweep. A bare `Bearer` plus 32 opaque characters does become `Bearer [redacted]`.

**Bot loop (J5).** The catalog row is J5, the gateway bot-loop guard. Hermes counts 20 bot messages in 300 seconds and then cools down for 600 seconds. `admit` returns `"tripped"` on the call that fills the window and `"cooldown"` while that deadline still holds. This crate counts 20 delivered `publish_bot` results and then returns `Cooldown`. There is no clock. Ordinary `publish` does not count.

**Backpressure.** `BUS_CAP` is 64. `try_send` returns `Slow(name)` instead of allocating a 65th slot. A missing name on `send_to` is `Disconnected`. A different topic is `Delivered(0)`.

**Left for later.** The 300 second window, the 600 second cooldown, and the notify cursor. Chapter 49 calls this same egress function on the way out of `call`. That chapter's pool records urls and bodies in memory. It does not open a socket, and it does not grow another scrubber.

## What you implement

`my-harness/tests/ch47.rs` is the milestone.

```rust
use my_harness::chapter_47_published;

#[test]
fn a_published_envelope_scrubs_the_key() {
    let body = chapter_47_published("see sk-live-0123456789abcdef");
    assert!(!body.contains("sk-live"), "{body}");
    assert!(body.contains("sk-***"), "{body}");
}
```

The stub panics.

```rust
pub fn chapter_47_published(body: &str) -> String {
    let _ = body;
    todo!("chapter 47")
}
```

`todo!("chapter 47")` is the panic. The test passes `see sk-live-0123456789abcdef` and wants a string that does not contain `sk-live` and does contain `sk-***`. In the step crate that is `publish` of an envelope whose body is that text, then the body a subscriber receives. `deliver` has already called `crate::redact::redact_for_egress`. The `sk-` prefix stays, the rest of the token becomes `***`. The stub stays `todo!`. Do not paste `bus.rs` into `my-harness/`.

## Chapter summary

- **Bus.** `publish` fans out on a topic. `send_to` names one peer. `Delivered(0)` is a topic nobody took.
- **Bound.** `BUS_CAP` is 64. The next `try_send` is `Slow(name)`.
- **Bots.** `MAX_BOT_EVENTS` is 20. The 21st `publish_bot` is `Cooldown` and is not sent.
- **Egress.** E2. `deliver` calls `crate::redact::redact_for_egress`. There is no second scrubber in this file.
- **Model.** `FakeModel` answers `ok`. The demo line is `bus: ping ok`.
- **Time.** `publish_to_agents` mean MEAN_PENDING_47. That is `mean.point_estimate` from this PR's Benchmarks workflow. The Criterion `time:` line is the slope.

[personal note: a channel that stayed at 64]

## What's next

Chapter 48 is shared team skills. Workers enqueue. The librarian is the only writer.
