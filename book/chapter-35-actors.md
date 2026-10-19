# Chapter 35: Agents as Actors

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 34 can share a snapshot. It still runs the agent on the caller's thread, and a burst of messages has nowhere to wait. This chapter gives each agent a task and a mailbox. The mailbox is the session inbox, with room for 64 messages. The 65th send is refused. A steer is stored and appended to the next turn. An interrupt drops that turn before the model runs.

The step crate is `steps/ch35-actors`. The book number is 33. The package is `ch35-actors`.

| Section | Step | What it adds |
|---|---|---|
| 35 | `steps/ch35-actors` | one task per agent, an inbox of 64, steer and interrupt |

> **How Hermes does it**
>
> I read `agent/interrupt_control.py` (`steer`) on Hermes `main` at `e31c674f`.
>
> `steer` returns false for empty text. Otherwise it strips the text and appends it to `_pending_steer`, joining with a newline when a steer is already waiting. It does not cancel the turn. `redirect` is the path that cancels a model request. During tool execution, `redirect` degrades to `steer`.
>
> **Adopted.** Empty steer text is ignored. Two steers join with a newline. A steer does not cancel the turn. An interrupt is a separate message, and the turn it marks does not call the model.
>
> **Adapted.** The mailbox is a `tokio` channel of 64. That capacity is the harness budget. It is not a constant in `steer`. `FakeModel` answers the turn. One hundred tasks is the test, not a Hermes pool size. Chapter 2's inbox is this mailbox. The bench there submitted to a capacity of 1. This chapter is the 64.
>
> **Deferred.** A live model request that `redirect` cancels mid-stream, and the gateway's busy-session policy. Chapter 39 is where a platform message becomes a steer or an interrupt.

[personal note: a second message that arrived while the first turn was still running]

## What this chapter decides

`Inbox` holds a `VecDeque` and refuses `try_submit` when the length is 64. `submit` is the same rule on a `tokio` channel: `try_send`, and `TrySendError::Full` is `InboxError::Full`. The sender does not wait.

`step` pops one message. `Steer` of blank text is dropped. Other steers accumulate. `Interrupt` sets a flag. `Turn` after that flag returns `interrupted` and does not call the model. A turn with a stored steer appends it after the reply.

`serve` reads the channel until the sender is dropped, and runs `step` for each message. `fan` starts `n` tasks. Each one submits one job and returns the scripted reply.

`demo` submits `hello` and returns `agent: ok`. No key.

## Rust: `try_send` is the backpressure

`mpsc::channel(64)` allocates the bound up front. `try_send` either takes a slot or returns `Full`. It does not grow the queue. The bench builds that channel in the setup closure, so the sample is the send, not the allocation of the 64 slots.

`JoinSet` is how `fan` waits. One hundred tasks is one hundred inboxes. They do not share a queue.

## Tests

Five tests. Sixty-four turns fill the inbox. The 65th is `Full`, and the length stays 64. A blank steer is ignored. `also this` and `and that` ride along on the next turn, and the model is called once. An interrupt then a turn returns `interrupted` and the model is not called. One hundred tasks return `ok 0` through `ok 99`. The channel of 64 matches: the 65th `submit` is `Full`. The demo is `agent: ok`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch35-actors
```

```text
Starting 5 tests across 3 binaries
     Summary [   0.007s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch35-actors
```

```text
agent: ok
```

The binary steps one scripted turn. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `inbox_submit` | 289.2 ns | enqueue under 1 ms | one `try_send` into an empty inbox of 64 |

`inbox_submit` uses `iter_batched`. The setup closure builds `mpsc::channel(64)` and is not timed. The timed routine calls `submit` with `Mail::Turn("hello")` passed through `black_box`, then `black_box` of the receiver so the channel stays open. The string `hello` is allocated inside the sample. No task is spawned. `FakeModel` is not constructed. The inbox is empty, so this sample does not take the `Full` path. Filling 64 and refusing the 65th is the test.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `33`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `agent: ok`.

## Pitfalls

`try_send` does not wait. A caller that needed the message to land has to notice `Full`. Blocking on `send` would hide the backpressure this chapter is for.

`serve` pulls one message at a time into a fresh `Inbox`. The bound the sender feels is the channel. The `VecDeque` cap is the same number, for the path that does not spawn a task.

A steer that arrives after `step` has already answered the turn waits for the next one. It does not rewrite the reply that already left.

`fan` sorts the replies as strings. `ok 10` sorts before `ok 2`. The test checks membership, not numeric order.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Mailbox.** Capacity 64. The 65th message is `Full`. The queue does not grow.

**Steer.** Blank text is ignored. Two steers join with a newline. A steer does not cancel the model call. That matches `interrupt_control.steer`.

**Interrupt.** The marked turn returns `interrupted` and does not call `FakeModel`. The model call is the thing an interrupt is for.

**Tasks.** One hundred agents means one hundred inboxes. They do not share a queue, and they do not share a model.

**Left for later.** The platform policy that chooses steer or interrupt is chapter 39. E1 and E2 are still absent. The mailbox text is not redacted.

## Chapter summary

- **Inbox.** 64 messages. `try_send` refuses the next one.
- **Steer.** Stored, joined with newlines, appended to the next turn.
- **Interrupt.** The next turn skips the model.
- **Tasks.** `fan(100)` is one hundred scripted agents.
- **Time.** One submit measured **289.2 ns**.

[personal note: a queue you wanted to see fill, on purpose]

## What's next

Chapter 36 restarts a failed child. The restart is bookkeeping. The child's work stays outside the measurement.
