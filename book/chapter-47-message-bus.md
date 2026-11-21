# Chapter 47: The Message Bus

Backpressure is what keeps a bus from turning into a log you can append to without limit. A librarian, the role the next chapter will name, can be slow, and each subscriber still has a channel of `BUS_CAP` 64, so a full channel returns `Slow` instead of a 65th slot and the publisher doesn't wait forever. That bound is the same number chapter 44 calls `INBOX_CAP` and the session inbox calls `INBOX_BOUND`, and this channel isn't an alias of either.

The bus is how peers talk without sharing an inbox. A skill change in the next chapter is a separate queue and a single writer, because `skill_propose` pushes a `Proposal` onto a `VecDeque` and `team_skills.rs` doesn't call `publish`. Chapter 49's external call is a call to an agent outside the process, and it uses the same egress scrub on the way out, which is why keeping the scrub in one function matters: a new channel can't forget it, and forgetting it is how a key in a tool result becomes a key on the wire.

That is what a continuously improving harness needs from this bus. An agent can publish an outcome without waiting on the librarian, and a key in a published body is scrubbed before `try_send`, so a learning team doesn't leak the secret that chapter 20 already knows how to hide.

The crate is `steps/ch47-message-bus`, a copy of chapter 46 plus `bus`, and the package is `ch47-message-bus`. An `Envelope` carries the topic and the body, `subscribe` registers a name on a topic, and `publish` fans that envelope out through `deliver`, which calls chapter 20's `redact_for_egress` before `try_send`. `send_to` is the direct path and doesn't scrub, so publishers are supposed to use `publish`. `publish_bot` counts toward the bot-loop guard, twenty delivered events and then `Cooldown`, and ordinary `publish` leaves the counter alone because this file has no clock. A different topic returns `Delivered(0)`.

Each subscriber holds an `mpsc` sender. `try_send` is the publish that returns instead of waiting, and `Full` stops the loop, while the envelope is cloned per subscriber so one slow receiver keeps its own buffer. The scrub returns a new string, and the cooldown is an enum the publisher matches on.

Running `cargo run -p ch47-message-bus --bin ch47-message-bus` prints the demo line `bus: ping ok`. The exercise is `my-harness/tests/ch47.rs`: a published envelope that contains a key is scrubbed, and a slow subscriber hits the bound.

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
> `bus.rs` does not define a function named `redact_for_egress`. `deliver` calls `crate::redact::redact_for_egress` before `try_send`. That is the chapter 20 function in `redact.rs`, re-exported from `lib.rs`.
>
> **Adopted.** A bounded `sync_channel` of 64. The 21st bot publish is `Cooldown` and is not sent. Every body that `publish` and `publish_bot` hand to `deliver` goes through the real egress function before `try_send`.
>
> **Adapted.** There is no wall-clock window and no SQLite cursor. Cooldown is a counter in this process, not 600 seconds. `publish` does not increment the counter. `publish_bot` increments it only after `Delivered`. The Rust `redact_for_egress` always runs the residue replace. It does not return `"[redaction-unavailable]"`, because the chapter 20 function has no catch. Hermes's Python function does.
>
> **Deferred.** The 300 second window and the notify cursor. A real socket is not this file. Chapter 49 is the call that leaves the process.

> **Guardrails.** E2 is called here. The name in the catalog is egress scrub. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

## What this chapter decides

Calling `subscribe` opens a `sync_channel` of `BUS_CAP` and keeps the name and the topic, and `BUS_CAP` is 64. `publish` calls `deliver`, which builds a new envelope whose body is `crate::redact::redact_for_egress` of the caller's body, then `try_send`s that envelope to every subscriber on the same topic. `Full` returns `Slow` with that subscriber's name and stops the loop, `Disconnected` names that subscriber too, and a topic with no match returns `Delivered(0)`.

A direct send names one peer. A missing name is `Disconnected`, a full channel is `Slow`, and the body it queues is the body the caller passed, because `send_to` skips the egress function.

A bot publish returns `Cooldown` when `bot_events` is already `MAX_BOT_EVENTS`, and that call isn't sent. `MAX_BOT_EVENTS` is 20, so the 21st bot publish is the one that cools down, after 20 delivered bot publishes, while a bot publish that returns `Slow` or `Disconnected` leaves the counter where it was.

The helper `submit` builds an `Envelope` and calls `publish`, so the scrub happens inside `deliver` rather than in a local function on the bus. `demo` subscribes `researcher` on `work`, submits `ping`, receives it, asks `FakeModel` for `ok`, and returns `bus: ping ok`.

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

The channel bound passed to `sync_channel` is `BUS_CAP`. The tests and the bench both reach that channel through `subscribe`, which passes `BUS_CAP`, so no caller hands in a different capacity, and a test that wants a smaller channel has to edit `subscribe` first.

The bot cap `MAX_BOT_EVENTS` is a `u32` rather than a `usize`, and the doc comment names the Hermes file. The constant is 20, which is `BotLoopGuardSettings.max_events`, while `window_seconds` and `cooldown_seconds` stay in Hermes, because this process counts deliveries.

An `Envelope` is the value on the channel, and both fields are public owned strings. `Clone` is what `deliver` uses when more than one subscriber matches, and `PartialEq` is why a test can compare a `Publish` value and why a received envelope could be compared field by field. The bus tests compare `Publish` and then read `.body`.

A `Sub` is private, so a caller can't build one, and the only constructor is `subscribe`. The name is the string `Slow` and `Disconnected` return, the topic is the filter, and the `SyncSender` is the writing half. The `Receiver` is what `subscribe` returns to the caller, so the bus keeps no way to read the queue, and a subscriber that drops its receiver makes the next `try_send` on that sender return `Disconnected`.

> **Rust sidebar: a private subscriber and a public envelope**
>
> `Sub` has no `pub`, while `Envelope` does, because the channel's type is `Envelope` and the receiver the caller holds can read `topic` and `body`. The caller can't reach into `Bus.subs` and rename a peer. `SyncSender<Envelope>` is `Clone` as well, but this file never clones the sender and clones the envelope instead, so one sender lives inside one `Sub` for the life of the bus.

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

A new bus starts with no subscribers and a bot counter of 0, and `Vec::new` allocates no channel until the first `subscribe`.

Registration takes `&mut self` because it pushes. The name and the topic are `impl Into<String>`, so the tests pass `&str` and the bench passes `format!("a{n}")`. `sync_channel(BUS_CAP)` returns a sender and a receiver with room for 64 undrained envelopes, which means the 65th `try_send` on a receiver that hasn't called `recv` is `Full`. The sender is stored, the receiver is returned, and the order of `subs` is subscription order, because `deliver` walks that order.

Callers read the counter through `bot_events`, and `publish_bot` is the only writer, so a test can assert the counter without taking a channel lock, since the counter is a field rather than a Hermes clock.

The `Default` impl calls `new`, and nothing in `bus.rs` calls `Bus::default`. The tests call `Bus::new`, and the impl is there so a `Bus` can be built the same way as other zero-state types in the crate.

> **Rust sidebar: `sync_channel` and `try_send`**
>
> `std::sync::mpsc::sync_channel` is a bounded queue. `send` waits when the queue is full, and `try_send` returns instead, with `Ok(())`, `Err(TrySendError::Full(value))`, or `Err(TrySendError::Disconnected(value))`. This file uses `try_send` on every path that writes, so a slow subscriber is a `Publish` value rather than a parked publisher. The `Full` and `Disconnected` arms drop the envelope that the error gave back, and the `_` is that value, which means it sits in no other queue.
>
> The receiver has to stay alive for `Ok`. The bench keeps the 100 receivers in the batched value until after `publish` returns, because dropping them first would be `Disconnected` for each matching name, and `deliver` would return on the first of those.

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

Every send in this file returns a `Publish`. `Delivered` carries a count of how many matching subscribers accepted the envelope on this call, which is a count for this call rather than a running total, while `Slow` and `Disconnected` carry the subscriber name and `Cooldown` carries nothing. There is one counter on the bus, rather than one counter per name.

An ordinary publish takes `&mut Bus` and calls `deliver`, which only needs `&Bus`. The mutable borrow isn't used to change `bot_events`, because ordinary publish leaves the cooldown counter alone, and the signature still asks for `&mut` so a caller can't hold a shared reference and publish while `publish_bot` wants to increment. The bench passes `&mut bus`.

The bot path checks the counter first. When `bot_events` is already 20, the function returns `Cooldown` and the envelope drops with the stack frame, so `deliver` isn't called and no channel sees that body. The doc comment says the 21st in this process is `Cooldown` and isn't sent, and "in this process" is the adaptation, because Hermes's cooldown is 600 seconds on a conversation key while this counter is one `u32` on one `Bus`. A second `Bus` has its own zero.

When the counter is still under 20, `publish_bot` calls `deliver` and then looks at the result. `matches!(result, Publish::Delivered(_))` is true for `Delivered(0)` as well as `Delivered(1)` or `Delivered(100)`, so a bot publish to a topic nobody subscribed still increments, because the result is `Delivered(0)`. A `Slow` leaves the counter alone, and a `Disconnected` leaves it alone too. The test locks the successful shape: twenty `Delivered(1)` calls, then `Cooldown`, and `bot_events()` equals `MAX_BOT_EVENTS`. A 22nd call is `Cooldown` again, and the counter stays at 20 on the refusal.

> **Rust sidebar: `matches!` and a counter that skips failures**
>
> `matches!` borrows `result` and leaves it in place, so the function can still `return result` afterward. An `if let Publish::Delivered(_) = result` would also work, and `matches!` is the form in the file. The `_` ignores the count, so `Delivered(0)` and `Delivered(100)` both count as one bot event. The guard is a delivery result, rather than a requirement that at least one peer accepted the envelope.
>
> The increment is `+= 1` on a `u32`, and it happens after `deliver` returns, so a panic inside `try_send` leaves the counter unmoved for that call, because the increment happens only after `deliver` returns. `try_send` returns the error this match already handles when the channel is `Full`, and that path panics nowhere.

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

Fan-out stays private in `deliver`, and `publish` together with `publish_bot` are the public doors. The first thing the function does is replace the caller's body and keep the topic, and the new body is `crate::redact::redact_for_egress(&envelope.body)`. The path is `crate::redact`, the module from chapter 20, so the function lives in `redact.rs` rather than in this file.

The scrub runs once, before the loop, so one hundred subscribers mean one scrub and one hundred clones of the scrubbed envelope. That is what the bench times when the body is `ping`. `ping` has nothing to mask, and the function still runs.

The loop skips a subscriber whose topic differs from the envelope topic. The comparison is `!=` on `String`, so `work` misses `other` and also misses `Work`, and a skipped subscriber is a continue rather than a failure. If every subscriber is skipped, `delivered` stays 0 and the function returns `Delivered(0)`, which is a successful publish to nobody. The test `a_different_topic_is_not_delivered` locks it: subscribe `peer` on `other`, publish topic `work` with body `ping`, expect `Delivered(0)`, and `try_recv` is an error. The body was still scrubbed, nobody received it, and `ping` scrubbed is still `ping`.

Each send clones before `try_send`, because `SyncSender::try_send` takes the envelope by value, so the clone is the one that moves and the original stays for the next matching subscriber. On `Ok`, the count goes up by one. On `Full`, the function returns `Slow` with `sub.name.clone()` immediately, so subscribers later in the vec are offered nothing while subscribers earlier in the vec already hold their clone. The return value is a `Slow` rather than a `Delivered` of a partial count, and the earlier sends stay where they landed, which means one full peer stops the fan-out and names itself. The test uses one subscriber, so the partial case stays out of that test, and the loop is what makes the early stop true.

On `Disconnected`, the same early return names that subscriber, and the envelope inside the error is dropped. A dropped receiver is how this arm happens. A name that was never subscribed reaches `send_to`, below, rather than this arm.

The fallthrough is `Publish::Delivered(delivered)`, and the count is the number of `Ok` sends on this call.

### The chapter 20 function

The redact module documents E1 and E2. `redact_sensitive_text` is E1, and `redact_for_egress` is E2, which is the function `deliver` calls.

```rust
/// `redact_for_egress`. The sensitive-text scrub, then the Bearer residue sweep.
pub fn redact_for_egress(text: &str) -> String {
    let scrubbed = redact_sensitive_text(text);
    bearer_residue()
        .replace_all(&scrubbed, "Bearer [redacted]")
        .into_owned()
}
```

Sensitive text goes through three replaces in `redact_sensitive_text`, in this order: prefix patterns first, then env assignments, then the `Authorization` header. The prefix replace keeps the characters through the first `-` or `_`, then writes `***`. The comment in `redact.rs` says short matches are fully masked and the prefix stays so a second pass sees no new key. The env replace keeps the variable name and writes `=***`. The header replace keeps `Authorization:` or `Proxy-Authorization:` and the following spaces, and writes `***` in place of the scheme and the token.

The residue regex is the same shape as Hermes `_BEARER_RESIDUE_RE`:

```rust
Regex::new(r"(?i)\bBearer\s+(?:\[[^\]]+\]|[A-Za-z0-9._~+/-]{20,}=*)")
```

It runs on the text the three replaces already produced. A `Bearer` that the header replace already consumed is gone before this sweep can match it. The opaque alternative needs 20 or more characters from that class, so a shorter token stays as written instead of becoming `Bearer [redacted]`. The bracket alternative matches an already-masked residue such as `Bearer [redacted-jwt]` and folds it to the same marker.

The test `egress_scrubs_a_bearer_and_an_sk_token` calls `harness_core::redact_for_egress` for three strings, then publishes a fourth. These are the four outcomes.

The header string is `Authorization: Bearer sk-live-0123456789abcdef now`. The prefix pattern `sk-[A-Za-z0-9_-](?:\.?[A-Za-z0-9_-]){9,}` matches `sk-live-0123456789abcdef`, and the first `-` is the prefix cut, so the match becomes `sk-***` while the text still has `Authorization: Bearer `. The header regex then matches `Authorization:` plus the scheme plus the token and replaces that whole header value with `Authorization: ***`, and the word `Bearer` sits inside the span that replace consumes, so the residue sweep misses it. The result contains `Authorization: ***`, the `sk-live` token is gone, and the line doesn't become `Bearer [redacted]`. The test asserts the two facts it cares about: no `sk-live`, and the string `Authorization: ***` is present.

The bare bearer string is `see Bearer abcdefghijklmnopqrstuvwxyz012345 now`. No prefix matches, no env assignment matches, and no `Authorization:` header matches, so the residue regex matches `Bearer` plus the 32-character token. Thirty-two is more than twenty, the replace writes `Bearer [redacted]`, and the result contains that marker while `abcdefghijklmnopqrstuvwxyz012345` is gone.

The assigned string is `ghp_abcdefghijklmnopqrst OPENAI_API_KEY=supersecret`. The prefix `ghp_[A-Za-z0-9]{10,}` matches `ghp_abcdefghijklmnopqrst`, the first `_` is the prefix cut, and that match becomes `ghp_***`. The env regex then matches `OPENAI_API_KEY=supersecret` and writes `OPENAI_API_KEY=***`. The result contains `ghp_***` and `OPENAI_API_KEY=***`, both `ghp_abcdefghijklmnopqrst` and `supersecret` are gone, and with no `Bearer` in the string the residue sweep leaves the text alone.

The published body is `see sk-live-0123456789abcdef and OPENAI_API_KEY=supersecret`. `publish` calls `deliver`, which calls the same function, so the `sk-` pattern rewrites the key to `sk-***` and the env pattern rewrites the assignment to `OPENAI_API_KEY=***`. With no `Authorization` header, the header pass leaves `sk-***` in place. The mail that `recv` returns contains `sk-***` and `OPENAI_API_KEY=***`, both `sk-live` and `supersecret` are gone, and `Publish` is `Delivered(1)`, because the scrubbed body is what sat on the channel.

Log lines take a different path, because `redacting_formatter` calls `redact_sensitive_text` only, so a log line skips the residue sweep. The bus calls `redact_for_egress` through `deliver`, and that call is E2.

Hermes's function at line 1164 is the scrub the box above describes, and the fence below is that function:

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

The Python marker `REDACTION_UNAVAILABLE` is `"[redaction-unavailable]"`. The `except` returns that marker, and the chapter 20 Rust function has no `except`, so a regex replace that finishes returns the scrubbed string. `bus.rs` calls that existing function, and the fail-closed branch stays in the Python function.

> **Rust sidebar: one replace, then the clones**
>
> The shadowing `let envelope = Envelope { ... }` drops the unscrubbed body when the new value is bound, and the topic `String` moves into the new struct. After that line, the only body in scope is the scrubbed one. `clone` on `Envelope` clones both strings, so topic `work` is four characters, cloned once per matching subscriber, and body `ping` is four characters, already scrubbed, cloned the same way. A secret would have been replaced before the first clone, and later subscribers see only the scrubbed body, because the raw body is gone before the loop.

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

A named send takes `&Bus`, leaves the bot counter alone, and skips the scrub. `find` walks `subs` for an equal name, the first match wins, and two subscribers with the same name would hide the second from this function, because `subscribe` accepts a duplicate name. The tests use unique names.

A missing name hits the `else` and returns `Disconnected(name.to_string())`, which is the same variant as a dropped receiver, with a different cause. No `try_send` runs, and the envelope the caller moved in is dropped. The test file skips this arm, while the function still has it, so a name that was never passed to `subscribe` is `Disconnected`.

When the name is present, `try_send` moves the caller's envelope. There is one sender, so there is no clone. `Ok` is `Delivered(1)` even when the envelope's topic differs from the subscriber's topic, because `send_to` compares names and `publish` compares topics. A direct send is a name, rather than a topic filter.

A full channel is `Slow` with that name. The test `a_full_subscriber_is_slow_and_a_direct_send_names_it` fills the channel with `BUS_CAP` publishes of `"0"` through `"63"`, then one more `publish` of `"more"` expects `Slow("slow")`, and then `send_to` of `"direct"` to `"slow"` expects `Slow("slow")` again. `try_recv` then yields body `"0"`, the first accepted body, while `"more"` and `"direct"` stay off the queue. The channel still held 64 when `try_recv` ran, so the head was `"0"`.

The small wrapper `demo` uses is `submit`. It copies `topic` and `body` into an `Envelope` and calls `publish`, `publish` calls `deliver`, and `deliver` scrubs, while `submit` itself never mentions `redact`, because the one call sits inside `deliver`.

> **Rust sidebar: `let` `else` on the find**
>
> `let Some(sub) = ... else { return ... }` is a refutable binding, so the `None` path leaves the function and the `Some` path binds `sub` for the `try_send`. `find` returns `Option<&Sub>`, `sub` is a shared reference, and `try_send` only needs `&self` on the sender. `name.to_string()` on the missing path copies the `&str` the caller still owns, and the caller's `Envelope` stays out of that string.

### A hole left open

A published body is scrubbed in `deliver` via `crate::redact::redact_for_egress`, before the envelope is offered to anyone. `send_to` is the direct path and doesn't scrub, because it `try_send`s the caller's envelope, so a key on that path leaves as given. The chapter keeps the path, and publishers are supposed to use `publish`.

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

The stand-in model ignores the prompt, and `let _ = prompt` keeps the argument from looking unused. The reply is `'static`, and `demo` uses `ok`. The kanban module has its own `FakeModel`, while this one is `harness_core::bus::FakeModel`, so they are different types, and the bench leaves this one unbuilt.

The demo builds an empty bus, subscribes `researcher` on `work`, and submits topic `work` with body `ping`. `submit` publishes, `deliver` scrubs `ping`, which stays `ping`, and `try_send` runs once. The `assert_eq!` wants `Delivered(1)`, so a topic typo panics inside `demo` instead of returning a different line. `recv` blocks until that one envelope is available, and it is available. `expect("mail")` is the panic text if the sender were dropped first, and the sender is inside the bus, which is still in scope.

The demo calls `complete` with `&mail.body`. The body is `ping`, the reply is `ok`, and the format string is `bus: {} {reply}`, where the first placeholder is the body and `{reply}` is the named argument. The returned string is `bus: ping ok`, and the test `demo_delivers_ping` expects that exact string. The colon and the spaces are literal, `mail.body` is the scrubbed body, and for `ping` that is the same four characters, with `{reply}` inserting `ok` and no extra punctuation. A change to the `Publish` count leaves the line alone if the assert still passes, a change to the reply changes the line, and the bus drops at the end of `demo` along with the receiver, so the printed line is the only thing `main` keeps.

The binary source in `chapter_demo.rs` prints that line:

```rust
use harness_core::bus::demo;

fn main() {
    println!("{}", demo());
}
```

Running the crate needs `cargo run` with `--bin ch47-message-bus`. The package also builds `ch47_mcp_fixture`, `sandbox-worker`, and `ch47_kanban_claim`, and the example binary prints eval lines, with no key read. `recv` in `demo` is the blocking receive, while the publish path the bench measures is `try_send`, which returns without waiting. `demo` can wait, because it is the only receiver and the send already happened.

## 47.7 The notify cursor, the bot-loop guard, and the one egress scrub

Hermes claims unseen events in `claim_unseen_events_for_sub`, which starts at line 352. The signature takes the connection, a task id, a platform, a chat id, an optional thread id, and optional kinds, and the return is `(old_cursor, new_cursor, events)`. The docstring says that when events are returned, `last_event_id` has already been advanced inside `BEGIN IMMEDIATE`, so concurrent gateway watchers serialize on the writer lock and only the first claims a given range. Callers send the events, then leave the cursor or call `rewind_notify_cursor` on delivery failure.

The body is `with _kb.write_txn(conn)`. When the connection isn't already in a transaction, line 1273 runs `_execute_boundary_with_retry(conn, "BEGIN IMMEDIATE")`, so the claim itself never types that statement and the context manager does. Inside the block, `_notify_cursor` selects `last_event_id`, and a missing row returns `0, 0, []`. `unseen_events_for_sub` selects `task_events` with `id > last_event_id`, and its own docstring says the cursor stays put there. If that list is empty, the claim returns the old cursor twice and an empty list, and `_cas_cursor` is skipped. If the list has rows, `_cas_cursor` runs:

```python
"UPDATE kanban_notify_subs SET last_event_id = ? " + _SUB_KEY_WHERE + " AND last_event_id = ?"
```

The update is conditional on the previous id, which is the compare-and-swap. This chapter leaves SQLite closed for the bus. `kanban.rs` is still the chapter 46 JSON board, copied forward, and `bus.rs` never reads `last_event_id`, because a subscriber here is a channel rather than a row. Two threads calling `publish` on one `Bus` would need a lock this file never takes, and `publish` wants `&mut Bus`, so the compiler serializes those calls while Hermes serializes them with `BEGIN IMMEDIATE`. The deferred piece is the cursor.

### The twenty-first bot publish

Twenty delivered bot publishes can land, and the next one can't. `MAX_BOT_EVENTS` is 20, so the 21st `publish_bot` is `Cooldown` and isn't sent, while ordinary `publish` leaves the counter alone. Hermes's settings, already in the box above, are a 300 second window and a 600 second cooldown, and this file has no clock, only the count.

Hermes keeps the guard settings in `BotLoopGuardSettings`, a frozen dataclass whose defaults are `enabled: bool = True`, `max_events: int = 20`, `window_seconds: float = 300.0`, and `cooldown_seconds: float = 600.0`. `settings_from_config` reads `gateway.bot_loop_guard`, and unusable values keep the default. `admit` takes a conversation key, and under the lock it sweeps, then returns `False, "cooldown"` when `cooldown_until` for that key is still in the future. Otherwise it drops timestamps older than `now - window_seconds`, and when the deque length is already `max_events` it sets the cooldown deadline, clears the deque, and returns `False, "tripped"`. Otherwise it appends `now` and returns `True, "ok"`. Disabled settings return `True, "disabled"` before the lock's counting work.

Those two return strings are Hermes's, and they depend on that clock. This crate's `publish_bot` has the 20 and skips the clock, so there is no `window_seconds`, no `cooldown_seconds`, and no conversation key on the bus. Twenty delivered bot publishes, then every later `publish_bot` is `Cooldown` until the process drops the `Bus`. Restarting the process is the only reset, because `new` sets the counter to 0. Hermes would admit again after 600 seconds, and would also forget events that aged out of 300 seconds, and both of those stay deferred.

The egress function is the one in section 47.4. Hermes names it the one scrub for text leaving the process, and `deliver` uses that role for text leaving the publisher's stack frame and entering another subscriber's queue, even though the queue is still this process. Prefix families, env assignments, the authorization header, and the bearer residue all run, because they live in `redact.rs` and `deliver` calls that whole function.

The session inbox in `inbox.rs` is still the chapter 2 queue. `SessionInbox::submit` returns `InboxError::Full` when that queue is at its bound, and `bus.rs` never calls `SessionInbox::submit`, so a published envelope stays off the session inbox.

> **Rust sidebar: `&mut` publish, shared deliver**
>
> `publish` and `publish_bot` take `&mut Bus`, while `deliver` and `send_to` take `&Bus`, and the split matches what they mutate. `deliver` only reads `subs` and writes through `SyncSender`, which is safe from a shared reference, and `publish_bot` writes `bot_events`, so it needs `&mut`. A caller who has published can call `bot_events` only after that mutable borrow ends. The tests do the call after the publish expression has finished, and the bench holds `&mut bus` only inside the timed closure.
>
> `Envelope` isn't `Copy`. Every send moves or clones it, and `publish_bot` on the cooldown path moves the envelope into the function and drops it without cloning, which is the "not sent." The test receives nothing after the cooldown call. The counter staying at 20 is the evidence the 21st left the count alone, and the early return is the evidence it sent nothing.

## Tests

The new tests are `steps/ch47-message-bus/crates/harness-core/tests/bus.rs`. The package also builds the copied harness tests, and the nextest line below is the package run.

The full-channel test `a_full_subscriber_is_slow_and_a_direct_send_names_it` builds a bus, subscribes `"slow"` on `"work"`, and publishes `BUS_CAP` envelopes. The loop variable `n` runs `0..BUS_CAP`, so the bodies are `"0"` through `"63"`, and each result is `Delivered(1)`. The next body is `"more"`, that result is `Slow("slow")`, and `send_to` of an envelope with topic `"work"` and body `"direct"` to the name `"slow"` is `Slow("slow")` again. `rx.try_recv().unwrap().body` is `"0"`, the head of the queue, and the two refused bodies are absent. The receiver was held for the whole test, so the `Slow` is `Full`, rather than `Disconnected`.

The cooldown test `the_twenty_first_bot_message_cools_down` subscribes `"peer"` on `"work"` and calls `publish_bot` with body `"ping"` in a loop of `MAX_BOT_EVENTS`. Each of those 20 matches `Delivered(1)`, the next `publish_bot` of `"ping"` equals `Cooldown`, and `bus.bot_events()` equals `MAX_BOT_EVENTS`. `let _ = rx` drops the receiver after those asserts. During the 20 sends and the cooldown call, the receiver was still alive, and the cooldown return happened before `deliver`, so it depended on neither a full channel nor a disconnected one. The 21st `"ping"` sits behind none of the first 20.

The egress test `egress_scrubs_a_bearer_and_an_sk_token` is the four strings in section 47.4. The first three call `redact_for_egress` directly, and the fourth goes through `publish` and `recv`. The header assertion is `!header.contains("sk-live")` and `header.contains("Authorization: ***")`. The bare bearer assertion is `bearer.contains("Bearer [redacted]")` and `!bearer.contains("abcdefghijklmnopqrstuvwxyz012345")`. The assigned assertion is no `ghp_abcdefghijklmnopqrst`, no `supersecret`, and `OPENAI_API_KEY=***` present. The mail assertion is no `sk-live`, no `supersecret`, `sk-***` present, and `OPENAI_API_KEY=***` present, and `published` equals `Delivered(1)`.

The other-topic test `a_different_topic_is_not_delivered` subscribes `"peer"` on `"other"` and publishes topic `"work"`, body `"ping"`. The result is `Delivered(0)`, and `rx.try_recv().is_err()` is true, so the zero is a count, rather than a `Slow` or a `Disconnected`.

The demo test `demo_delivers_ping` expects `demo()` to equal `bus: ping ok`, which is the line the binary prints.

The test module imports `demo`, `publish`, `publish_bot`, `send_to`, `Bus`, `Envelope`, `Publish`, `BUS_CAP`, and `MAX_BOT_EVENTS` from `harness_core::bus`, and `redact_for_egress` from `harness_core`. That last import is the re-export, and the test imports its scrub from the crate root rather than from `bus`.

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

The binary publishes one envelope and prints the line, and no key is read.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `publish_to_agents` | 47.98 µs | one small publish under 1 ms at 100 agents | `redact_for_egress` of `ping`, then `try_send` to 100 empty channels |

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

Setup stays outside the timer, because `iter_batched` runs the setup closure there. That closure builds a `Bus`, subscribes `a0` through `a99` on topic `work`, and keeps the 100 receivers.

The means are `mean.point_estimate` from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), section `47`. Bench ids: `47/publish_to_agents`, `45/publish_to_agents`, `43/publish_to_agents`. Command: `cargo bench -p ch47-message-bus --bench bus --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The bench is 100 successful sends of a body that scrubs to itself, and `redact_for_egress` sits inside the sample. A number that fills 64 first is the slow-subscriber test, rather than this one, and a number that called `publish_bot` 21 times would return `Cooldown` and would miss this sample.

The receivers are part of the batched value so they stay alive through `try_send`. Dropping them first would be `Disconnected`, which is a different result. `let _ = rx` in the cooldown test drops the receiver after the asserts, and moving that line above the loop would disconnect the channel, `deliver` would return `Disconnected`, and the counter would stay 0 because only `Delivered` increments. The 21st call would then still try to send.

The demo uses `recv`, which waits, and the bench uses `try_send`, which returns at once. Measuring `demo` would include a receive and a format, and this sample includes neither.

The Rust egress function returns the scrubbed string, and `"[redaction-unavailable]"` stays in Hermes, where an exception takes that branch. A panic in the regex crate would still panic here, because this file has no catch around the replace.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Egress scrub (E2).** The catalog row is E2. The name is egress scrub. One stricter scrub for remote readers, and it also strips a bearer residue. The Hermes function is `agent/redact.py` `redact_for_egress`. This chapter calls the chapter 20 Rust function of that name, from `deliver`, before `try_send`. `bus.rs` does not define its own scrubber. The header `Authorization: Bearer sk-live-0123456789abcdef now` becomes a line that contains `Authorization: ***` and does not contain `sk-live`, and it does not become `Bearer [redacted]`, because the header replace consumes the scheme before the residue sweep. A bare `Bearer` plus 32 opaque characters does become `Bearer [redacted]`.

**Bot loop (J5).** The catalog row is J5, the gateway bot-loop guard. Hermes counts 20 bot messages in 300 seconds and then cools down for 600 seconds. `admit` returns `"tripped"` on the call that fills the window and `"cooldown"` while that deadline still holds. This crate counts 20 delivered `publish_bot` results and then returns `Cooldown`. There is no clock. Ordinary `publish` does not count.

**Backpressure.** `BUS_CAP` is 64. `try_send` returns `Slow(name)` instead of allocating a 65th slot. A missing name on `send_to` is `Disconnected`. A different topic is `Delivered(0)`.

**Left for later.** The 300 second window, the 600 second cooldown, and the notify cursor. Chapter 49 calls this same egress function on the way out of `call`. That chapter's pool records urls and bodies in memory. It does not open a socket, and it does not grow another scrubber.

## What you implement

The milestone is `my-harness/tests/ch47.rs`.

```rust
use my_harness::chapter_47_published;

#[test]
fn a_published_envelope_scrubs_the_key() {
    let body = chapter_47_published("see sk-live-0123456789abcdef");
    assert!(!body.contains("sk-live"), "{body}");
    assert!(body.contains("sk-***"), "{body}");
}
```

The stub panics on `todo!("chapter 47")`.

```rust
pub fn chapter_47_published(body: &str) -> String {
    let _ = body;
    todo!("chapter 47")
}
```

That panic is `todo!("chapter 47")`. The test passes `see sk-live-0123456789abcdef` and wants a string that keeps `sk-***` while the `sk-live` token is gone. In the step crate that is `publish` of an envelope whose body is that text, then the body a subscriber receives, after `deliver` has already called `crate::redact::redact_for_egress`. The `sk-` prefix stays, the rest of the token becomes `***`, and the stub stays `todo!`.

## Chapter summary

- **Bus.** `publish` fans an envelope out on a topic, `send_to` names one peer, and `Delivered(0)` is a topic nobody took.
- **Bound.** `BUS_CAP` is 64, and the next `try_send` on a full channel is `Slow(name)`.
- **Bots.** `MAX_BOT_EVENTS` is 20, so the 21st `publish_bot` is `Cooldown` and isn't sent.
- **Egress.** E2 runs because `deliver` calls `crate::redact::redact_for_egress` before the envelope is queued.
- **Model.** `FakeModel` answers `ok`, and the demo line is `bus: ping ok`.
- **Time.** `publish_to_agents` mean 47.98 µs, which is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

## What's next

Peers can talk, and the next chapter decides who may write a shared skill.
