# Chapter 39: The Gateway

A message shows up at a front door the terminal never had to open. A person on a platform has typed, a second bot has answered, or someone has sent a correction while a turn is still running, and that text has to reach the session inbox from chapter 2. Chapter 35 made the inbox the agent's mailbox. This chapter is that door.

Three things decide whether the message gets in. Who may talk is an allowlist and a pairing check. The default after those two is deny, with five bad approvals locking the caller out for an hour. How often is two clocks. Ten minutes have to pass between pairing requests, and the bot guard trips after twenty messages inside five minutes, so two bots can't keep answering each other forever. Whether the door can be stopped is the pause sentinel. A stat error fails safe, so the gateway treats the emergency stop as engaged.

Enqueue, then let the agent task run, is the whole integration. A gateway that called the model itself would bypass the inbox, the approval floor, and the turn table. Corrections arrive as steers and land in the mailbox reflection already knows how to read, which is why this door matters to a continuously improving harness. An interrupt skips the turn and lands in that same mailbox.

This is the chat gateway, not the MCP server. The MCP server is chapters 5, 7, 8, 15, 23, 37, 42, and 51, and it stays a separate program. The crate is `steps/ch39-gateway`, a copy of chapter 38 plus `gateway`, and the package is `ch39-gateway`.

The inbox is a `VecDeque` of 64, the same cap chapter 35 uses, and a platform enqueue is a push onto that queue. Folding keeps text and skips tool chunks, using chapter 19's events so a chat user sees one message. A running session refuses a re-bind, the check that chapter 30 deferred to here. Outbound text calls chapter 20's `redact_for_egress` on the way out.

B4 is the shell guard. A supervised command that would restart, stop, or uninstall this gateway is denied. An unsupervised command returns none, because a foreground run is still allowed to manage the service. Chapter 43's lifecycle check is the same idea applied to a cron prompt, and it stays in chapter 43. Moving it here would make a scheduled job's prompt and an interactive shell the same function even though they see different text.

The tests run this chat front in-process, and nothing binds the MCP port.

`cargo run --locked -p ch39-gateway --bin ch39-gateway` queues, folds, and denies, and the printed line is `gateway: queued`. The exercise is `my-harness/tests/ch39.rs`: a gateway restart command is blocked, and a platform enqueue isn't a model call.

| Section | Step | What it adds |
|---|---|---|
| 39.1 | `steps/ch39-gateway` | the pairing, bot, inbox, and error constants |
| 39.2 | `steps/ch39-gateway` | `Pairing::request` |
| 39.3 | `steps/ch39-gateway` | `Pairing::approve` |
| 39.4 | `steps/ch39-gateway` | `authorize` |
| 39.5 | `steps/ch39-gateway` | `BotGuard::admit` |
| 39.6 | `steps/ch39-gateway` | `is_engaged` |
| 39.7 | `steps/ch39-gateway` | `Inbox::submit` |
| 39.8 | `steps/ch39-gateway` | `fold` |
| 39.9 | `steps/ch39-gateway` | `handoff_request` |
| 39.10 | `steps/ch39-gateway` | `demo` |
| 39.11 | `steps/ch39-gateway` | `gateway_lifecycle_block` |
| 39.12 | `steps/ch39-gateway` | `admit_shell` and `send_out` |

> **How Hermes does it**
>
> I read `gateway/pairing.py` (`ALPHABET`, `CODE_LENGTH`, `CODE_TTL_SECONDS`, `RATE_LIMIT_SECONDS`, `LOCKOUT_SECONDS`, `MAX_PENDING_PER_PLATFORM`, `MAX_FAILED_ATTEMPTS`), `gateway/bot_loop_guard.py` (`BotLoopGuardSettings`, `BotLoopGuard.admit`), `agent/estop.py` (`is_engaged`), `tools/terminal_tool_guards.py` (`gateway_lifecycle_block`, line 255), and `cron/lifecycle_guard.py` (`_GATEWAY_LIFECYCLE_PATTERN`) on Hermes `main` at `9e01b44`. Outbound scrub is chapter 20's `redact_for_egress`, which reads `agent/redact.py`.
>
> `ALPHABET` is `ABCDEFGHJKLMNPQRSTUVWXYZ23456789`. `CODE_LENGTH` is 8. `CODE_TTL_SECONDS` is 3600. `RATE_LIMIT_SECONDS` is 600. `LOCKOUT_SECONDS` is 3600. `MAX_PENDING_PER_PLATFORM` is 3. `MAX_FAILED_ATTEMPTS` is 5. The file comment says the code is chosen with `secrets`, stored as a salted hash, and never logged. `BotLoopGuardSettings` defaults to 20 events, a window of 300.0 seconds, and a cooldown of 600.0 seconds. `admit` returns `(allowed, state)` with state `ok`, `tripped`, or `cooldown`. `is_engaged` returns true if any sentinel path exists, and true if a stat raised `OSError`. `gateway_lifecycle_block` returns immediately when `_is_supervised_gateway_process()` is false. A supervised process then calls `scan_gateway_lifecycle` from the cron module. `force=True` is not a parameter and cannot skip the block. The docstring names `systemctl`, `launchctl`, and `hermes gateway restart|stop|uninstall`.
>
> **Adopted.** Default deny after the allowlist and an already-paired caller. The pairing alphabet, the length 8, the hour TTL, the 600 second request gap, the pending cap of 3, and the lockout at 5 failures for 3600 seconds. The bot-loop numbers 20, 300, and 600. Fail-safe emergency stop. A supervised shell that matches the lifecycle patterns is blocked. An unsupervised shell returns no block. The inbox cap is 64, the same bound as chapter 2 and chapter 35.
>
> **Adapted.** The code is stored in a `Vec<(String, u64)>` so a test can approve it. Hermes stores a salted hash under `~/.hermes/pairing/`. `now` is an argument. There is one pairing record, not a file per platform. `BotGuard` is one conversation. Hermes keys a dict by conversation and takes a lock. `is_engaged` takes a slice of `Stat` values. It does not call `path.exists()`. `gateway_lifecycle_block` takes `(command, supervised)` and returns `Option<&'static str>`. It compiles four `Regex` values in `OnceLock`s. It does not read a referenced script, resolve a cwd, or return JSON. `admit_shell` turns that `Option` into a `Result`. `send_out` calls `crate::redact::redact_for_egress`. It does not keep a second redactor. `FakeModel` is used by `demo` after the push. `submit` does not call it. The bench pushes one turn.
>
> **Deferred.** The rest of `_is_user_authorized` (trusted upstream, group lists, adapter roles, `GATEWAY_ALLOW_ALL_USERS`). Home-channel checks. A real sentinel file. Live platform sockets. Salted hashes and `chmod 0600`. The cron prompt scanner. Chapter 43 keeps `check_gateway_lifecycle` for job text. This chapter does not move that check. Interpreter-image kills (`pkill python`) and the launchctl submit pre-scan stay in Hermes. The MCP server is chapter 42.
>
> I looked for `tui_gateway/methods_session.py` in this checkout. That directory is not on `9e01b44`. Chapter 30 read `handoff.request` at `ab3fe9dffb0a` and left the platform re-bind for here. `handoff_request` is that deferred check. Busy is code 4009, the code chapter 30 recorded. Empty platform is 4023 and an unknown platform is 4024 in this crate.

## What this chapter decides

`authorize` returns true when the user is on the allowlist or the caller passes `paired: true`. It is false otherwise, because the function takes that flag from the caller and leaves the pairing store closed. `Pairing::request` refuses a locked platform, a request inside 600 seconds of the last one, a fourth pending code, and a code that isn't eight characters from the alphabet. `approve` strips whitespace, uppercases, and pairs on a match still inside 3600 seconds, counting each miss and setting the lockout to `now + 3600` on the fifth before it clears the counter. `BotGuard::admit` drops events at or before `now - 300` when that subtraction fits in a `u64`. The 21st event inside the window returns `Tripped` and clears the window. It then sets a cooldown of 600 seconds, and a later `admit` during that cooldown returns `Cooldown`. `is_engaged` returns true on `Stat::Present`, and also when any path is `Stat::Error` and none was present, while a slice of only `Absent` is false. `Inbox::submit` accepts a `Message::Turn` or a `Message::Steer` until 64 and returns `Submit::Full` on the 65th, and `fold` concatenates `Chunk::Text` while it skips `Chunk::Tool`. `handoff_request` returns `Handoff::Busy` with code 4009 when the session is running, and code 4023 for an empty platform. An absent name is code 4024, and any other name comes back `Pending`, trimmed and lowercased. `gateway_lifecycle_block` returns `None` when `supervised` is false, and when `supervised` is true a match on any of the four patterns returns the static blocked sentence, which `admit_shell` maps to `Err`, while `send_out` returns `redact_for_egress(text)`. `demo` queues one turn, folds `hello` and ` there`, calls `FakeModel`, and denies `stranger`. It returns `gateway: queued`, with the key left out of that string. Calling `submit` on a platform turn leaves `FakeModel` unconstructed. The bench `enqueue_platform` is that push, with id `39/enqueue_platform` and the older alias `37/enqueue_platform` still listed in `benchmarks/descriptions.json`.

## 39.1 The constants are the Hermes numbers

The numbers at the top of the module are the ones Hermes already picked, written down so the rest of the file can name them.

```rust
pub const ALPHABET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
pub const CODE_LENGTH: usize = 8;
pub const CODE_TTL_SECONDS: u64 = 3600;
pub const RATE_LIMIT_SECONDS: u64 = 600;
pub const LOCKOUT_SECONDS: u64 = 3600;
pub const MAX_PENDING_PER_PLATFORM: usize = 3;
pub const MAX_FAILED_ATTEMPTS: u32 = 5;
pub const BOT_MAX_EVENTS: usize = 20;
pub const BOT_WINDOW_SECONDS: u64 = 300;
pub const BOT_COOLDOWN_SECONDS: u64 = 600;
pub const INBOX_CAP: usize = 64;
pub const SESSION_BUSY: i32 = 4009;
pub const PLATFORM_REQUIRED: i32 = 4023;
pub const UNKNOWN_PLATFORM: i32 = 4024;
```

`ALPHABET` drops `I`, `O`, `0`, and `1`, which is the unambiguous alphabet `pairing.py` describes. `CODE_LENGTH` is 8, so a code is eight of those characters. `CODE_TTL_SECONDS` is 3600, one hour, and `RATE_LIMIT_SECONDS` is 600, ten minutes between requests from the same record. `LOCKOUT_SECONDS` is 3600, the lock after too many bad approvals, with `MAX_PENDING_PER_PLATFORM` at 3 and `MAX_FAILED_ATTEMPTS` at 5.

`BOT_MAX_EVENTS` is 20, `BOT_WINDOW_SECONDS` is 300, and `BOT_COOLDOWN_SECONDS` is 600, the defaults on `BotLoopGuardSettings` in `gateway/bot_loop_guard.py`. Hermes stores the window and the cooldown as `float`, and this file stores whole seconds in `u64`.

`INBOX_CAP` is 64, the same bound chapter 2's session inbox and chapter 35's actor mailbox use as `INBOX_BOUND` in `inbox.rs`. This module keeps its own queue, so `SessionInbox::submit` stays on the session type. `SESSION_BUSY`, `PLATFORM_REQUIRED`, and `UNKNOWN_PLATFORM` are the integers `handoff_request` puts on its variants.

> **Rust sidebar.** A `const` is a value the compiler inlines, and the type is part of the declaration. `usize` is a length, `u64` is a timestamp in seconds, `u32` is a small counter, and `i32` is an error code that can be compared with `==`. `ALPHABET` is `&str`, a borrowed string that lives for the whole program. The underscore in `3_600` is a digit separator whose value is still 3600.

## 39.2 A request stores the code in a Vec

Issuing a code is the first gate on who may talk. The errors are the five ways that request or a later approval can fail.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairError {
    RateLimited,
    Locked,
    TooManyPending,
    BadCode,
    Unknown,
}
```

`Pairing` holds the pending codes, the last request time, the failure count, the lockout deadline, and whether this record is paired.

```rust
pub struct Pairing {
    pending: Vec<(String, u64)>,
    last_request: Option<u64>,
    fails: u32,
    lockout_until: u64,
    paired: bool,
}
```

The tuple is `(code, created_at)`, and both fields are private, with `paired()` and `pending_len()` as the reads the tests use. `Default` calls `new()`, which starts unpaired, unlocked, and with an empty vec.

`request` is the whole policy for issuing a code.

```rust
pub fn request(&mut self, now: u64, code: &str) -> Result<(), PairError> {
    if now < self.lockout_until {
        return Err(PairError::Locked);
    }
    if let Some(last) = self.last_request {
        if now.saturating_sub(last) < RATE_LIMIT_SECONDS {
            return Err(PairError::RateLimited);
        }
    }
    self.pending
        .retain(|(_, created)| now.saturating_sub(*created) <= CODE_TTL_SECONDS);
    if self.pending.len() >= MAX_PENDING_PER_PLATFORM {
        return Err(PairError::TooManyPending);
    }
    if !valid_code(code) {
        return Err(PairError::BadCode);
    }
    self.pending.push((code.to_string(), now));
    self.last_request = Some(now);
    Ok(())
}
```

The lockout is checked first, so a caller inside the lock gets no new code and the rate-limit clock stays where it was. The rate limit uses `saturating_sub`, which means a `now` smaller than `last` is a gap of zero and the request is refused. Expired pending rows are dropped before the cap is applied, so a code older than 3600 seconds leaves one of the three slots free. The fourth live code is `TooManyPending`, refused before `valid_code` runs. A bad code is refused before `push`. The caller supplies the code, where Hermes would generate it with `secrets.choice`. This crate takes the string so a test can request `ABCDEFGH` and approve it later.

`valid_code` is private.

```rust
fn valid_code(code: &str) -> bool {
    code.len() == CODE_LENGTH && code.chars().all(|c| ALPHABET.contains(c))
}
```

`code.len()` is bytes, and every character in the alphabet is one ASCII byte, so a length of 8 is eight characters. `IIIIIIII` has length 8 and fails `ALPHABET.contains`, because `I` is missing from the string. `abcd2345` fails too, because `request` leaves the case alone and `approve` is the call that uppercases.

> **Rust sidebar.** `Result<(), PairError>` is either `Ok(())` or `Err` of one of those five. The unit value `()` means success carries no payload. `if let Some(last)` reads the `Option` only when a previous request exists. So the first request has `last_request == None` and skips the gap check. `&mut self` means the method changes the struct, and the caller can use the `Pairing` again only after the call returns.

## 39.3 Approve pairs, or counts a miss

A code that was stored still has to be accepted. A wrong guess has to count, because five misses are what lock the platform.

```rust
pub fn approve(&mut self, now: u64, code: &str) -> Result<(), PairError> {
    if now < self.lockout_until {
        return Err(PairError::Locked);
    }
    self.pending
        .retain(|(_, created)| now.saturating_sub(*created) <= CODE_TTL_SECONDS);
    let code = code
        .split_whitespace()
        .collect::<String>()
        .to_ascii_uppercase();
    if let Some(index) = self
        .pending
        .iter()
        .position(|(pending, _)| pending == &code)
    {
        self.pending.remove(index);
        self.fails = 0;
        self.paired = true;
        return Ok(());
    }
    self.fails += 1;
    if self.fails >= MAX_FAILED_ATTEMPTS {
        self.lockout_until = now.saturating_add(LOCKOUT_SECONDS);
        self.fails = 0;
    }
    Err(PairError::Unknown)
}
```

Whitespace is removed and the code is uppercased before the search, so `abcd efgh` becomes `ABCDEFGH` and matches the row `request` stored. A hit removes that one row, clears `fails`, and sets `paired`, while the other pending rows stay.

A miss increments `fails`, and on the fifth miss `lockout_until` becomes `now + 3600` and the counter returns to zero. The next `request` or `approve` before that deadline is `Locked`, rather than another `Unknown`. The test approves `NOPE2345` at timestamps 0, 1, 2, 3, and 4. The fifth call sets the deadline to 3604, and `request` at timestamp 10 is `Locked`. `approve` at `LOCKOUT_SECONDS + 10` (3610) is past the deadline, the code was never requested, and the result is `Unknown`.

A code requested at 0 and approved at `CODE_TTL_SECONDS + 1` (3601) is dropped by `retain` first, so the approval is `Unknown` and `paired` stays false. The comparison is `<= CODE_TTL_SECONDS`, which means an approval at exactly 3600 still sees the row.

The code sits in the `Vec` as text. Hermes stores a salted hash and says codes are never logged. This crate keeps the plaintext so a test can approve the same string it requested. Printing `pending` would print the code, and `demo` leaves that vec alone.

## 39.4 Authorize is allowlist, then paired, then deny

The same message is authorized here, before it is queued, and the decision is one boolean.

```rust
pub fn authorize(allowlist: &[&str], paired: bool, user: &str) -> bool {
    allowlist.contains(&user) || paired
}
```

`authorize(allowlist, paired, user)` takes `paired` from the caller and doesn't open the pairing store. The function returns the allowlist match or the flag, so an allowlisted user passes with `paired` false, and a paired user passes with an empty allowlist. A stranger with an empty allowlist and `paired` false is denied. The security-relevant part is whatever the caller passed. Hermes `_is_user_authorized` has more steps before this default deny. Those steps stay deferred, which leaves this bool as the decision the chapter tests.

`contains` on `&[&str]` compares the user string with each entry. The caller built the slice, and the comparison is the whole read.

## 39.5 Twenty bot messages, then ten minutes of quiet

Once a caller is allowed to talk, the next limit is how often a bot may speak, and the answer is a window of timestamps.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BotAdmit {
    Ok,
    Tripped,
    Cooldown,
}

pub struct BotGuard {
    events: VecDeque<u64>,
    cooldown_until: u64,
}
```

`admit` is the window.

```rust
pub fn admit(&mut self, now: u64) -> BotAdmit {
    if self.cooldown_until > now {
        return BotAdmit::Cooldown;
    }
    if let Some(cutoff) = now.checked_sub(BOT_WINDOW_SECONDS) {
        while self.events.front().copied().is_some_and(|t| t <= cutoff) {
            self.events.pop_front();
        }
    }
    if self.events.len() >= BOT_MAX_EVENTS {
        self.cooldown_until = now.saturating_add(BOT_COOLDOWN_SECONDS);
        self.events.clear();
        return BotAdmit::Tripped;
    }
    self.events.push_back(now);
    BotAdmit::Ok
}
```

Cooldown is checked before the window is pruned, so a timestamp inside the cooldown returns `Cooldown` and leaves the deque alone. The cutoff uses `checked_sub`, and when `now` is smaller than 300 the subtraction doesn't fit in a `u64`, the `if let` is skipped, and the events stay. Twenty calls at timestamp 0 stay `Ok` because nothing is pruned. The 21st sees `len() >= 20`, sets `cooldown_until` to 600, clears the deque, and returns `Tripped`. A call at 599 is `Cooldown` because `600 > 599`, and a call at 600 sits outside the cooldown, so the empty deque makes that call `Ok` and starts a new window.

Hermes computes `cutoff = now - window_seconds` with floats and drops events `<= cutoff`, so a clock at 0 would subtract 300 and drop those events. `saturating_sub` would turn `now - 300` at timestamp 0 into 0 and drop every event, so the 21st call would never trip. `checked_sub` skips the prune when the subtraction doesn't fit, which is why twenty calls at 0 stay `Ok` and the 21st returns `Tripped`. The test `twenty_bot_messages_then_a_cooldown` is that sequence.

> **Rust sidebar.** `VecDeque` pushes and pops at both ends in constant time, `front()` returns `Option<&u64>`, `copied()` turns that into `Option<u64>`. `is_some_and` runs the predicate only when there is a front element. `Copy` on `BotAdmit` means the enum is returned by copy, and the three variants are the states Hermes names `ok`, `tripped`, and `cooldown`. Hermes also returns `disabled` when the guard is turned off in config. This struct has no enabled flag, so the guard is always on.

## 39.6 A stat error pauses

The third question, whether the door can be stopped, is answered by a list of stats the caller already took.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    Present,
    Absent,
    Error,
}

pub fn is_engaged(paths: &[Stat]) -> bool {
    let mut saw_error = false;
    for path in paths {
        match path {
            Stat::Present => return true,
            Stat::Absent => {}
            Stat::Error => saw_error = true,
        }
    }
    saw_error
}
```

`Present` returns immediately, so a later `Error` is unnecessary once a sentinel has been seen, and `Error` then `Present` is true because of `Present`. `Absent` then `Error` finishes the loop and returns the flag, while two `Absent` values return false. Hermes `is_engaged` loops candidate paths, returns true on `path.exists()`, and sets the same flag on `OSError`. An empty slice here is false, because there was no error and no sentinel, and Hermes always has at least the process-home path. The caller passes the stats it already took, which keeps this function from inventing a path of its own.

> **Rust sidebar.** `match` names every variant, and the empty arm `Stat::Absent => {}` is the branch that changes nothing. So forgetting that arm is a compile error because the match has to be complete. `saw_error` is a `bool` on the stack, and it lives only for the call rather than on a sentinel file.

## 39.7 The inbox is a VecDeque of 64

The same message, once it is authorized, is queued here, and the queue is the integration with the agent task.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Turn(String),
    Steer(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Submit {
    Accepted,
    Full,
}

pub struct Inbox {
    q: VecDeque<Message>,
    cap: usize,
}
```

`new` allocates the capacity up front and stores the cap separately. `with_capacity` is a hint, and the refusal is the `len` check rather than the allocation.

```rust
pub fn submit(&mut self, message: Message) -> Submit {
    if self.q.len() >= self.cap {
        return Submit::Full;
    }
    self.q.push_back(message);
    Submit::Accepted
}
```

A `Turn` and a `Steer` take the same slot. Chapter 35's mailbox distinguishes them when the turn runs, and this queue only admits them, so `front` borrows the oldest message while `len` and `is_empty` read the deque. Nothing in `submit` constructs `FakeModel`. The test `a_supervised_restart_is_blocked_and_enqueue_is_not_a_model_call` submits `Message::Turn("hi")` and asserts `Accepted`, and the model type stays out of that test.

The capacity test pushes one steer and then `INBOX_CAP - 1` turns, the length is 64, and the next turn is `Full`. The steer is still `front`, because `push_back` adds at the other end.

> **Rust sidebar.** `Message` owns its `String`, and `submit` takes the message by value, so the queue keeps it after the caller returns. `Submit` is `Copy` because it carries no string, and `Full` names no dropped message, because the message was moved into the function and dropped on the `Full` path. Build the string again if you need to retry.

## 39.8 Fold keeps text and skips tool chunks

The same message is folded here, on the way from stream chunks to the one line a chat user should see.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chunk {
    Text(String),
    Tool(String),
}

pub fn fold(chunks: &[Chunk]) -> String {
    let mut out = String::new();
    for chunk in chunks {
        if let Chunk::Text(text) = chunk {
            out.push_str(text);
        }
    }
    out
}
```

`Text("hello")`, `Tool("nope")`, and `Text("!")` fold to `hello!`, and the tool name stays out of the string. `demo` folds `hello`, a tool chunk `skip`, and ` there` into `hello there`, and that string is the prompt `FakeModel::complete` receives. `complete` discards the prompt and returns the canned reply, so the fold itself stops at the string.

Chapter 19 folds stream deltas into one assistant message. This function is the gateway's smaller version, a slice of chunks already in memory, text only.

## 39.9 A running session refuses the re-bind

A platform that wants to take over a session has to ask while the session is idle, because a running turn already owns the mailbox.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Handoff {
    Busy { code: i32 },
    PlatformRequired { code: i32 },
    UnknownPlatform { code: i32, platform: String },
    Pending { platform: String },
}

pub fn handoff_request(running: bool, platform: &str, enabled: &[&str]) -> Handoff {
    if running {
        return Handoff::Busy { code: SESSION_BUSY };
    }
    let platform = platform.trim().to_ascii_lowercase();
    if platform.is_empty() {
        return Handoff::PlatformRequired {
            code: PLATFORM_REQUIRED,
        };
    }
    if !enabled.contains(&platform.as_str()) {
        return Handoff::UnknownPlatform {
            code: UNKNOWN_PLATFORM,
            platform,
        };
    }
    Handoff::Pending { platform }
}
```

`running` is checked before the name is cleaned, so a busy session with a good platform is still `Busy { code: 4009 }`. The test passes `true`, `"telegram"`, and `&["telegram"]`.

An idle session trims and lowercases. `"  "` becomes empty and returns `PlatformRequired { code: 4023 }`, `"slack"` with only `telegram` enabled returns `UnknownPlatform` carrying the lowercased name and code 4024. `" Telegram "` with `telegram` enabled returns `Pending { platform: "telegram" }`. There is no watcher and no pending row on disk, so the variant is the decision. Chapter 30's `Board::handoff` filters a transcript and has no view of whether a session is running, and this function is the check that board deferred.

> **Rust sidebar.** A struct-like enum variant holds fields, and `Busy { code }` and `Pending { platform }` are different shapes. So a caller matches them instead of reading a nullable code. `platform.as_str()` borrows the local `String` long enough for `contains`. On the `UnknownPlatform` path the same `String` moves into the variant, so it is built once.

## 39.10 The demo queues, folds, and denies

The binary has to show the door working, which means a queued turn, a folded reply, and a stranger turned away.

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
    let mut inbox = Inbox::new(INBOX_CAP);
    let accepted = inbox.submit(Message::Turn("hi".to_string()));
    let text = fold(&[
        Chunk::Text("hello".to_string()),
        Chunk::Tool("skip".to_string()),
        Chunk::Text(" there".to_string()),
    ]);
    let _ = FakeModel { reply: "ok" }.complete(&text);
    if accepted == Submit::Accepted && !authorize(&["owner"], false, "stranger") {
        return "gateway: queued".to_string();
    }
    "gateway: open".to_string()
}
```

The binary `ch39-gateway` is `src/bin/chapter_demo.rs`, and it prints `demo()`, so the line is `gateway: queued`. The stranger is denied because the allowlist is `owner` and `paired` is false, the turn was accepted. The model reply is dropped with `let _ =`, so no key is formatted into the return value.

`examples/demo.rs` is the chapter 9 eval printer, and the milestone line is the other binary. `cargo run --locked -p ch39-gateway --bin ch39-gateway` is that line.

`submit` finishes before `FakeModel` is constructed, so a reader who only calls `submit` has made a queue push, and the bench is that reader.

## 39.11 B4 is four patterns, and only when supervised

The shell guard sits on the same path, and it only matters once the process is the supervised gateway rather than a foreground shell.

The `use` lines for `OnceLock` and `Regex` sit under `demo`. A `use` in Rust is allowed anywhere at module scope, and these two are next to the functions that need them.

```rust
pub fn gateway_lifecycle_block(command: &str, supervised: bool) -> Option<&'static str> {
    if !supervised {
        return None;
    }
    if hermes_gateway().is_match(command)
        || systemctl_gateway().is_match(command)
        || launchctl_gateway().is_match(command)
        || kill_gateway().is_match(command)
    {
        return Some(
            "Blocked: command cannot restart, stop, or uninstall the gateway from inside the gateway process.",
        );
    }
    None
}
```

`supervised` false returns `None` before any regex runs. Hermes does the same with `_is_supervised_gateway_process()`. A foreground `hermes gateway run`, and any process that merely imported the gateway, may still restart the service. The marker that means this code was loaded is a different probe from supervision. This bool is the supervision probe, passed in by the caller rather than read from an environment variable.

The four patterns are the branches of `_GATEWAY_LIFECYCLE_PATTERN` in `cron/lifecycle_guard.py`, copied into this file so a shell check can stand without the cron scanner. Chapter 43 still has its own lifecycle check for job prompts. The two copies are the point of the split. A cron job is a prompt and this function is a terminal command, so they stay separate functions.

```rust
fn hermes_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)(?:^|[^/\w.\-])hermes(?:\.(?:exe|cmd|bat|com|ps1))?\s+gateway\s+(?:restart|stop|uninstall)\b",
        )
        .expect("pattern")
    })
}
```

`(?i)` is case insensitive. The prefix `(?:^|[^/\w.\-])` keeps `/docs/hermes gateway restart-notes.md` from matching, because the character before `hermes` is `/`. Hermes writes that idea as a lookbehind, `(?<![/\w.\-])`. A command at the start of the string matches `^`. `hermes.exe`, `hermes.cmd`, `hermes.bat`, `hermes.com`, and `hermes.ps1` are the Windows launcher suffixes, and the verb is `restart`, `stop`, or `uninstall`. `hermes gateway start` stays unmatched, because Hermes leaves `start` out of this branch on purpose so that starting a sibling profile is allowed, while `hermes gateway restart` matches.

```rust
fn systemctl_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)systemctl\s+(?:-\S+\s+)*(?:restart|stop|start)\b[^\n]*\bhermes[.\-]?gateway",
        )
        .expect("pattern")
    })
}

fn launchctl_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(r"(?i)launchctl\s+(?:bootout|kickstart|unload)\b[^\n]*\bhermes").expect("pattern")
    })
}

fn kill_gateway() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)\b(?:p?kill|taskkill|stop-process)\b[^\n]*\bhermes\b[^\n]*\bgateway|\b(?:p?kill|taskkill|stop-process)\b[^\n]*\bgateway\b[^\n]*\bhermes",
        )
        .expect("pattern")
    })
}
```

`systemctl` allows flags (`-S+`) before `restart`, `stop`, or `start`, then `hermes-gateway` or `hermes.gateway` on the same line. `start` is included here, so `systemctl start hermes-gateway` is a block. `launchctl` in this crate is `bootout`, `kickstart`, or `unload`, and the rest of the line only has to contain `hermes`. Hermes's launchctl branch also names `load`, `stop`, `restart`, `submit`, `bootstrap`, `remove`, and `disable`. It requires a gateway label, so those extra verbs, and `contains_launchctl_submit_command`, stay in Hermes. `kill` matches both token orders, `pkill` or `kill` or `taskkill` or `stop-process`, with `hermes` and `gateway` on the same line. `skill` stays clear of `kill` because of the leading `\b`. A kill of the Python interpreter image, with no `hermes` token, stays unmatched, and that is Hermes branch E, which stays deferred.

The patterns hard-code `hermes gateway restart`, and the same branches cover `stop` and `uninstall`, because that is what Hermes blocks, including the Windows suffixes on the binary. A reader whose program isn't named `hermes` has to swap that name for their own binary before these checks describe their process. The verbs and the word boundaries are the part worth keeping once the name matches.

`Regex::new(...).expect("pattern")` panics if the pattern fails to compile. The patterns are fixed strings in the source, so a panic there is a bug in the pattern, and it happens once, inside `get_or_init`.

> **Rust sidebar.** `OnceLock<Regex>` builds the regex on the first call and returns the same `&Regex` after that. `get_or_init` runs the closure only when the cell is empty. `'static` means the reference lives for the rest of the process. The bench skips this function, so the sample pays nothing for the compile. `Option<&'static str>` is `None` when the command may run, and `Some` of one sentence when it may not. The sentence is a string literal, so it has no owner to free.

## 39.12 admit_shell is a Result, and send_out is chapter 20

The same message is scrubbed on the way out, after the shell guard has turned the lifecycle block into a `Result` the caller can match.

```rust
pub fn admit_shell(command: &str, supervised: bool) -> Result<(), &'static str> {
    match gateway_lifecycle_block(command, supervised) {
        Some(reason) => Err(reason),
        None => Ok(()),
    }
}

pub fn send_out(text: &str) -> String {
    crate::redact::redact_for_egress(text)
}
```

`admit_shell("hermes gateway restart", true)` is `Err` of the blocked sentence, and the test checks that the sentence contains `Blocked`. `admit_shell("hermes gateway restart", false)` is `Ok(())`, and `admit_shell("echo hello", true)` is `Ok(())`. `echo` is the command the architecture note says a new guard must still allow, and this guard allows it.

`send_out` is one call. `redact_for_egress` in `redact.rs` runs `redact_sensitive_text` and then the Bearer residue sweep. The sensitive-text pass masks prefix families (`sk-`, `ghp_`, Slack, Google, AWS, and the rest of the list in that file), `NAME=value` assignments for names that look like secrets, and `Authorization` headers. The test passes `Authorization: Bearer sk-abcdefghijklmnopqrstuvwxyz`. After the prefix pass the `sk-` body is masked, and the header pass replaces the credential, so the assertion is that the original secret substring is absent. `gateway.rs` keeps a single call, and the pattern table stays in the redactor chapter 20 already wrote.

> **Rust sidebar.** `Result<(), &'static str>` uses the error string as the `Err` payload, and `match` on `Option` has two arms, both written. `crate::redact::redact_for_egress` is a path into another module of this same crate, the function chapter 20 put there. Calling it by that path is the whole of `send_out`.

## Tests

`tests/gateway.rs` walks the decisions above.

`strangers_are_denied_and_allowlisted_or_paired_are_not` denies `stranger` against `["owner"]`, accepts `owner`, accepts a paired stranger with an empty allowlist, and denies that stranger when `paired` is false. `pairing_code_rules` rejects `IIIIIIII`, rate-limits a second request at `RATE_LIMIT_SECONDS - 1`, accepts one at `RATE_LIMIT_SECONDS` and another at twice that, refuses the fourth pending code, and approves `abcd efgh` into the paired state. `expired_code_and_five_failures_lock_the_platform` approves one second after the TTL and stays unpaired, then locks after `MAX_FAILED_ATTEMPTS` misses.

`twenty_bot_messages_then_a_cooldown` allows `BOT_MAX_EVENTS` admits at timestamp 0, trips the next, stays in `Cooldown` at `BOT_COOLDOWN_SECONDS - 1`, and allows the admit at `BOT_COOLDOWN_SECONDS`. `estop_fails_safe` covers `Present`, `Absent` plus `Error`, two `Absent` values, and `Error` followed by `Present`. `inbox_holds_a_steer_and_folds_text` keeps the steer at `front`, fills to `INBOX_CAP`, refuses the next turn, and folds to `hello!`.

`a_busy_session_rejects_the_rebind` expects `Handoff::Busy { code: SESSION_BUSY }` for a running session, `PlatformRequired` for blank platform text, `UnknownPlatform` for `slack`, and `Pending` for ` Telegram `. `demo_queues_and_denies_a_stranger` expects `gateway: queued`. `a_supervised_restart_is_blocked_and_enqueue_is_not_a_model_call` expects `admit_shell("hermes gateway restart", true)` to be an error containing `Blocked`, the same command with `supervised` false to be `Ok`, and `echo hello` under supervision to be `Ok`. It also expects one `submit` of a turn to be `Accepted`, `send_out` of an `Authorization: Bearer` line to drop `sk-abcdefghijklmnopqrstuvwxyz`, and `FakeModel` to stay unnamed.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch39-gateway
```

```text
    Starting 205 tests across 45 binaries
     Summary [   0.560s] 205 tests run: 205 passed, 0 skipped
```

```bash
cargo run --locked -p ch39-gateway --bin ch39-gateway
```

```text
gateway: queued
```

The binary queues one turn and denies a stranger, and the printed line carries no key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `enqueue_platform` | 152.0 ns | enqueue a platform message under 1 ms | one turn pushed onto an empty inbox of 64 |

`enqueue_platform` is `benches/gateway.rs`. The id is `39/enqueue_platform`, and the alias `37/enqueue_platform` is an older id of this same function.

```rust
fn enqueue_platform(c: &mut Criterion) {
    c.bench_function("enqueue_platform", |b| {
        b.iter_batched(
            || Inbox::new(INBOX_CAP),
            |mut inbox| {
                let submitted = inbox.submit(Message::Turn(black_box("hello".to_string())));
                black_box(submitted);
            },
            BatchSize::SmallInput,
        );
    });
}
```

The setup closure calls `Inbox::new(64)` and stays outside the timer. The timed routine builds `hello`, calls `submit` with `Message::Turn`, and passes both the string and the `Submit` through `black_box`. The queue is empty, so the result is `Accepted`. Pairing, `authorize`, `fold`, `handoff_request`, `gateway_lifecycle_block`, `send_out`, and `FakeModel` stay out of the sample, and the `Full` path is left untaken.

The means are `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `39`. Bench ids: `39/enqueue_platform`, `37/enqueue_platform`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The bench is one push onto an empty inbox, so a number that includes pairing, a lockout, a regex compile, or 64 fills is a different question, and those questions are the tests.

`checked_sub` is the bot window on purpose. Replacing it with `saturating_sub` makes every event recorded at timestamp 0 look expired. The 21st call never trips, which is the contrast section 39.5 walks between the two subtractions.

The pairing code is plaintext in the `Vec`, so a log of `pending` is a log of the code, and Hermes refused that by storing a hash.

`gateway_lifecycle_block` matches the command string and leaves the script body alone. Hermes `scan_gateway_lifecycle` reads that file, and it fails closed when the script can't be read, so a supervised `bash restart.sh` whose file contains `hermes gateway restart` is `Ok` here. Chapter 43's prompt scan is still the place a job's text is checked, and that scan has to stay even though this function exists.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Authorization (L1a).** Allowlist, then an already-paired caller, then deny. An empty allowlist and an unpaired caller do not get in. The longer Hermes chain stays deferred.

**Pairing (L2a).** Eight characters from the alphabet with no `I`, `O`, `0`, or `1`. `CODE_TTL_SECONDS` is 3600. `RATE_LIMIT_SECONDS` is 600. `MAX_PENDING_PER_PLATFORM` is 3. `MAX_FAILED_ATTEMPTS` is 5, and `LOCKOUT_SECONDS` is 3600.

**Bot loop (J5).** `BOT_MAX_EVENTS` is 20. `BOT_WINDOW_SECONDS` is 300. `BOT_COOLDOWN_SECONDS` is 600. The 21st event in the window trips. The next event inside the cooldown stays out.

**Emergency stop (N2).** `Stat::Present` engages. `Stat::Error` engages when nothing was present. `Stat::Absent` does not.

**Gateway lifecycle (B4).** `tools/terminal_tool_guards.py` `gateway_lifecycle_block` refuses restart, stop, and uninstall from inside a supervised gateway. `admit_shell` is that decision as a `Result`. `supervised` false returns `Ok`. `echo hello` stays allowed. Chapter 43's M2 cron check stays in chapter 43.

**Egress (E2).** `send_out` calls `redact_for_egress` from chapter 20. The Bearer line in the test loses the `sk-` secret. Plugin patterns and the vault are still not copied.

**Left for later.** Live sockets, salted pairing files, the rest of the authorization order, and the MCP gateway. E1's formatter stays in chapter 20. This chapter uses the egress function.

## What you implement

`my-harness/tests/ch39.rs` is the milestone. A gateway restart command is blocked, and a platform enqueue isn't a model call.

```rust
use my_harness::{chapter_39_enqueue_is_a_model_call, chapter_39_restart_blocked};

#[test]
fn a_gateway_restart_is_blocked_and_enqueue_is_not_a_model_call() {
    assert!(chapter_39_restart_blocked("hermes gateway restart"));
    assert!(!chapter_39_enqueue_is_a_model_call());
}
```

The stubs panic.

```rust
pub fn chapter_39_restart_blocked(command: &str) -> bool {
    let _ = command;
    todo!("chapter 39")
}

pub fn chapter_39_enqueue_is_a_model_call() -> bool {
    todo!("chapter 39")
}
```

`todo!("chapter 39")` is the panic. The restart assertion wants true for `hermes gateway restart` under supervision, and the enqueue assertion wants false, because `Inbox::submit` of a `Message::Turn` stops at the queue and `FakeModel` stays unbuilt.

## Chapter summary

- **Allow.** The allowlist or an already-paired caller gets in, and a stranger stays out until one of those two is true.
- **Pair.** A code is eight characters from the alphabet, it lives one hour, three may be pending, and five failures lock the platform for an hour.
- **Bots.** Twenty messages in 300 seconds trip the guard, and the cooldown after that trip is 600 seconds.
- **Stop.** A present sentinel engages the pause, and so does a stat error when nothing was present.
- **Inbox.** The queue holds 64 messages, a turn and a steer are the same push, and `submit` stops at the deque.
- **Re-bind.** A running session comes back as code 4009, and an unknown or empty platform has its own code.
- **B4.** A supervised restart, stop, or uninstall is blocked, and an unsupervised command is left alone.
- **Egress.** `send_out` is `redact_for_egress`, the scrub chapter 20 already owns.
- **Model.** `demo` calls `FakeModel` and prints `gateway: queued`, and the bench times the push instead.
- **Time.** `enqueue_platform` mean `152.0 ns`, which is `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).

## What's next

Chapter 40 opens a page. The URL is checked before anything is fetched, and the text that comes back is marked as data.
