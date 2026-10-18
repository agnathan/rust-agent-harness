# Chapter 43: The Message Bus

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 42 is a card with an owner. Peers that live at the same time need a different shape: one publishes, the others receive, and a slow receiver must not grow a queue without a limit. This chapter is that bus.

The step crate is `steps/ch43-message-bus`. The book number is 43. The package is `ch43-message-bus`.

| Section | Step | What it adds |
|---|---|---|
| 43 | `steps/ch43-message-bus` | a bounded publish, a direct send, a bot cooldown |

> **How Hermes does it**
>
> I read `hermes_cli/kanban_db_notify.py` (`claim_unseen_events_for_sub`), `gateway/bot_loop_guard.py` (`BotLoopGuardSettings`), and `agent/redact.py` (`redact_for_egress`) on Hermes `main` at `e31c674f`.
>
> `claim_unseen_events_for_sub` claims events inside `BEGIN IMMEDIATE` and advances the subscriber cursor so two watchers do not both take the same range. `BotLoopGuardSettings` defaults to 20 events, a 300 second window, and a 600 second cooldown. The module docstring says two bots replying never stop unless this guard counts them. `redact_for_egress` is the scrub for text leaving toward a remote reader, including A2A, and it fails closed.
>
> **Adopted.** A subscriber that cannot take another message is backpressure, not a bigger queue. The 21st bot-authored publish is a cooldown and is not sent. A body is scrubbed before it is queued.
>
> **Adapted.** The bus is `sync_channel` with capacity 64, the inbox cap from chapter 2. There is no 300 second window and no SQLite cursor. `publish` does not count toward the cooldown. `publish_bot` does. The scrub masks a `Bearer` residue and an `sk-` token.
>
> **Deferred.** The notify cursor and the wall-clock cooldown. A2A's remote call is chapter 45. Egress to a real socket is not this chapter.

[personal note: a full channel that had to say slow]

## What this chapter decides

`subscribe` opens a `sync_channel` of 64 and keeps the name and the topic.

`publish` `try_send`s to every subscriber on that topic. `Full` returns `Slow` with the subscriber's name and does not keep going. `Disconnected` names that subscriber too. A topic with no match returns `Delivered(0)`.

`send_to` is one peer. The same `Full` is `Slow`.

`publish_bot` refuses when 20 bot publishes have already been delivered. The refused one is `Cooldown` and is not sent.

`submit` runs `redact_for_egress` and then `publish`. The bench calls `publish` on the body `ping`, which has nothing to scrub.

`demo` delivers `ping` and prints `bus: ping ok`. No key.

## Rust: try_send is the bound

`sync_channel(64)` blocks on `send` when it is full. `try_send` does not block. It returns `Full` or `Disconnected`. The bus uses `try_send` so a slow subscriber is a result, not a parked publisher. The receivers have to stay alive for the send to succeed. The bench holds them until after `publish` returns.

## Tests

Five tests. Filling 64 and publishing once more returns `Slow("slow")`, and `send_to` does too. Twenty `publish_bot` calls deliver, and the next is `Cooldown`. `Bearer sk-live-0123456789abcdef` does not survive `submit`. A different topic delivers zero and the receiver is empty. The demo is `bus: ping ok`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch43-message-bus
```

```text
     Summary [   0.006s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch43-message-bus
```

```text
bus: ping ok
```

The binary publishes one envelope. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `publish_to_agents` | 45.75 µs | one small publish under 1 ms at 100 agents | `try_send` of `ping` to 100 empty channels |

`publish_to_agents` uses `iter_batched`. The setup closure builds a `Bus`, subscribes `a0` through `a99` on topic `work`, and keeps the 100 receivers. That setup is not timed. The timed routine calls `publish` with topic `work` and body `ping`, then `black_box` of the result and the receivers. The 100 clones and the 100 `try_send` calls are inside the sample. The channels are empty, so the result is `Delivered(100)`. `redact_for_egress`, `publish_bot`, and `FakeModel` are not called. The `Full` path is not taken.

The table is `mean.point_estimate` from Benchmarks run [38991844877](https://github.com/agnathan/rust-agent-harness/actions/runs/38991844877) on ubuntu-latest (`bench-run.json`, 45745.398594771235 ns). The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `bus: ping ok`.

## Pitfalls

The bench is 100 successful sends. A number that fills 64 first is the slow-subscriber test.

`publish` does not trip the cooldown. A bench that called `publish_bot` 21 times would return `Cooldown` and would not be this sample.

The receivers are part of the batched value so they are not dropped before `try_send`. Dropping them first would be `Disconnected`, which is a different result.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Backpressure.** Capacity is 64. `try_send` returns `Slow` instead of allocating a 65th slot.

**Bot loop (J5).** `BotLoopGuardSettings` uses 20 events in 300 seconds. This crate counts 20 `publish_bot` deliveries and then returns `Cooldown`. There is no clock. Ordinary `publish` does not count.

**Egress (E2).** `submit` masks `Bearer` and `sk-` before the body is queued. The pattern list in `redact_for_egress` is wider. Two shapes are what this chapter implements.

**Left for later.** The SQLite cursor, the 600 second cooldown, and a real socket. Chapter 45 scrubs again on the way out of the process.

## Chapter summary

- **Bus.** Topic publish and a direct send.
- **Bound.** 64. The next one is `Slow`.
- **Bots.** The 21st `publish_bot` is `Cooldown`.
- **Model.** `FakeModel` answers `ok`. The demo needs no key.
- **Time.** One publish to 100 agents measured **45.75 µs**.

[personal note: a channel that stayed at 64]

## What's next

Chapter 44 puts skills on a queue. Workers propose. A librarian is the only writer, and readers keep an `Arc`.
