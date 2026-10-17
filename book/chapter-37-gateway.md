# Chapter 37: The Gateway

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 36 knows which persona is speaking. It does not know which person is allowed to speak. This chapter is the chat gateway. A user is on the allowlist, already paired, or denied. A pairing code is eight characters and lasts an hour. Twenty bot messages in five minutes start a ten-minute cooldown. A sentinel pauses the process, and a stat error pauses it too. A platform message is a push onto an inbox of 64. Streaming text folds into one string. This is not the MCP server.

The step crate is `steps/ch32-gateway`. The book number is 37. The package is `ch32-gateway`.

| Section | Step | What it adds |
|---|---|---|
| 37 | `steps/ch32-gateway` | allowlist, pairing, bot-loop guard, emergency stop, the session inbox |

> **How Hermes does it**
>
> I read `gateway/authz_mixin.py` (`_is_user_authorized`), `gateway/pairing.py` (`ALPHABET`, `CODE_LENGTH`, `CODE_TTL_SECONDS`, `RATE_LIMIT_SECONDS`, `LOCKOUT_SECONDS`, `MAX_PENDING_PER_PLATFORM`, `MAX_FAILED_ATTEMPTS`, `generate_code`, `_record_failed_attempt`), `gateway/bot_loop_guard.py` (`BotLoopGuardSettings`, `admit`), `agent/estop.py` (`is_engaged`), and `tui_gateway/methods_session.py` (`handoff.request`) on Hermes `main` at `e31c674f`.
>
> Authorization ends in a default deny after the allowlist and pairing checks. The pairing alphabet is `ABCDEFGHJKLMNPQRSTUVWXYZ23456789`. A code is 8 characters, lives 3600 seconds, and a user may request one every 600 seconds. Five failed approvals lock the platform for 3600 seconds. A platform keeps at most 3 pending codes. Only a salted hash of the code is stored. The bot-loop guard allows 20 bot messages in 300 seconds, then a 600 second cooldown. `is_engaged` is true if any sentinel exists, and true if a stat raised. `handoff.request` returns error 4009 while the session is running. An empty platform is 4023. The method writes a pending row for the gateway watcher. Chapter 28 left that re-bind for here.
>
> **Adopted.** Default deny after allowlist and pairing. The pairing constants, the alphabet, the lockout at 5, and the pending cap of 3. The bot-loop numbers. Fail-safe emergency stop. Busy handoff is 4009. A steer is a message on the same inbox. Text chunks fold into one string. The inbox cap is 64, from chapter 2.
>
> **Adapted.** The code is stored in a `Vec`, not a salted hash, so a test can approve it. There is no JSON file. `now` is an argument. Enabled platforms are a slice. `FakeModel` sees the folded text. The bench pushes one turn and does not run pairing.
>
> **Deferred.** The rest of the Hermes authorization order (chat-scoped lists, adapter policy, `GATEWAY_ALLOW_ALL_USERS`). Home-channel checks 4025 and 4026. A real sentinel file. Live platform sockets. The MCP server is chapter 38.

[personal note: a message that was a push, and a stranger the allowlist did not name]

## What this chapter decides

`authorize` is true when the user is in the allowlist, or when `paired` is true. Otherwise it is false.

`request` refuses a locked platform, a request inside 600 seconds of the last one, a fourth pending code, and a code that is not 8 characters from the alphabet. `approve` strips whitespace, uppercases, and pairs on a match that is still inside 3600 seconds. A miss counts. The fifth miss sets the lockout to `now + 3600` and clears the counter. A success clears the counter too.

`BotGuard::admit` drops events older than 300 seconds. The 21st event inside the window returns `Tripped`, clears the window, and sets a cooldown of 600 seconds. A later `admit` during that cooldown returns `Cooldown`.

`is_engaged` returns true on `Present`. It returns true if any path is `Error` and none was present. All `Absent` is false.

`Inbox::submit` accepts a `Turn` or a `Steer` until 64. The 65th is `Full`. `fold` concatenates `Chunk::Text` and skips `Chunk::Tool`.

`handoff_request` returns `Busy` with code 4009 when the session is running. An empty platform is 4023. A platform not in the enabled slice is 4024. Otherwise the result is `Pending` with the lowercased name.

`demo` queues one turn, folds `hello` and ` there`, calls `FakeModel`, denies `stranger`, and returns `gateway: queued`. No key.

## Rust: the inbox is a VecDeque

`submit` checks `len` against the cap, then `push_back`. A steer and a turn are the same queue. The bench builds an empty inbox in the setup closure and times one push of `Turn("hello")`.

## Tests

Eight tests. A stranger is denied. An allowlisted user and a paired user are not. A code with `I` is rejected. A second request inside 600 seconds is rate limited. The fourth pending code is refused. `abcd efgh` approves `ABCDEFGH`. A code older than 3600 seconds does not pair. Five bad approvals lock the next request. Twenty bot messages at timestamp 0 are allowed and the next one trips; 599 seconds later is still cooling down; 600 is allowed. A present sentinel and a stat error both engage. An inbox takes one steer and 63 turns, then refuses. Folded text is `hello!`. A running session returns 4009. The demo is `gateway: queued`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch32-gateway
```

```text
     Summary [   0.008s] 8 tests run: 8 passed, 0 skipped
```

```bash
cargo run -p ch32-gateway
```

```text
gateway: queued
```

The binary queues one turn and denies a stranger. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `enqueue_platform` | 77.90 ns | enqueue a platform message under 1 ms | one turn pushed onto an empty inbox of 64 |

`enqueue_platform` uses `iter_batched`. The setup closure calls `Inbox::new(64)` and is not timed. The timed routine calls `submit` with `Message::Turn` of `hello`, the string built inside the sample and passed through `black_box`, then `black_box` of `Submit`. The queue is empty, so the result is `Accepted`. Pairing, the bot guard, `fold`, and `FakeModel` are not called. The `Full` path is not taken.

The printed slope on this machine was `time: [73.434 ns 86.186 ns 94.376 ns]`. The table is `mean.point_estimate` from `target/criterion/enqueue_platform/new/estimates.json` (77.8967 ns), with the Benchmarks flags `--warm-up-time 1 --measurement-time 2 --sample-size 10`. The workflow mean on ubuntu-latest replaces this number when that run is green.

### Milestone demo

The `cargo run` above. The line is `gateway: queued`.

## Pitfalls

The bench is one push. A number that includes pairing, a lockout, or 64 fills is the capacity test.

The code sits in the `Vec` as text. Hermes stores a salted hash. This crate is easier to test and worse to leak. Do not print `pending`.

Timestamp 0 is inside the bot window. Pruning with a saturating subtract treated every event at 0 as already expired. The cutoff is `checked_sub`, so a `now` smaller than 300 seconds does not drop a fresh event.

`authorize` does not read the pairing store itself. The caller passes `paired`. A test that forgets that flag will deny a user the approve step already accepted.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Authorization (L1a).** Allowlist, then pairing, then deny. An empty allowlist and an unpaired caller do not get in.

**Pairing (L2a).** Eight characters from the alphabet with no I, O, 0, or 1. One hour to live. One request per user per 10 minutes. Three pending codes. Five failures lock the platform for one hour.

**Bot loop (J5).** Twenty bot messages in 300 seconds, then 600 seconds of cooldown. The message that trips is not appended.

**Emergency stop (N2).** Any present sentinel engages. A stat error engages. No sentinel and a clean stat stays open.

**Left for later.** The salted hash, the sentinel file, and the longer allowlist order. E1 and E2 are still absent. The MCP service is the next chapter.

## Chapter summary

- **Deny.** Allowlist or paired. Everyone else stops.
- **Pair.** Eight characters, one hour, five strikes, three pending.
- **Bots.** Twenty in five minutes, then ten minutes of quiet.
- **Stop.** A sentinel, or a stat error, engages.
- **Inbox.** A turn or a steer, cap 64. Text folds into one string.
- **Model.** `FakeModel` answers `ok`. The demo needs no key.
- **Time.** One enqueue measured **77.90 ns**.

[personal note: a gateway that refused first and queued second]

## What's next

Chapter 38 is the MCP gateway service, on the server track. Replicas, a session route, a drain, and the checks Hermes runs as an MCP client.
