# Chapter 28: Handoffs

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 27 can run specialists and take their replies back. It does not give the conversation to someone else. This chapter moves ownership. The next agent sees the transcript with `scratch` removed. A turn may do this four times. The fifth call is refused, and the owner does not change.

The step crate is `steps/ch24-handoffs`. The book number is 28. The package is `ch24-handoffs`.

| Section | Step | What it adds |
|---|---|---|
| 28 | `steps/ch24-handoffs` | a filtered transcript and a cap of four handoffs |

> **How Hermes does it**
>
> I read `tui_gateway/methods_session.py` (`handoff.request`) on Hermes `main` at `8dcf6d5b`.
>
> `handoff.request` queues a desktop handoff. The handler writes `pending`. A gateway watcher claims it and re-binds the session to another platform. A busy session is rejected. The platform name is required. There is no loop over agents, and there is no counter of how many handoffs a turn has made. The handler does not filter a transcript.
>
> **Adopted.** Ownership moves only when the request is accepted. A refused call leaves the current owner in place. The Hermes path is a platform transfer, and this chapter says so.
>
> **Adapted.** `MAX_HANDOFFS` is 4. That number is the harness cap. It is not in `handoff.request`. Before the owner changes, `filter_transcript` drops every message whose role is `scratch`. The next agent receives that vec, not the raw one.
>
> **Deferred.** The gateway watcher, the platform name check, and the busy-session rejection. Chapter 37 is the chat gateway that submits onto the session inbox. That is where a platform re-bind belongs.

[personal note: a handoff that passed along a scratch note you did not mean the next agent to see]

## What this chapter decides

`Board` starts with an owner and a count of 0. `handoff` filters first. An empty result is `HandoffError::Empty`, the count stays, and the owner stays. Otherwise the count increments. If it was already 4, the increment is undone and the error is `Limit`. The owner string is replaced only after the count is accepted.

`filter_transcript` keeps order. Role `scratch` is the only role it drops.

`demo` hands `outline` to `draft` with three messages, one of them scratch, and returns `draft sees 2`. No key.

## Rust: the count and the undo

`fetch_add` returns the old value. If that old value is already `MAX_HANDOFFS`, the call `fetch_sub`s and returns `Limit`. Two threads could both pass. This board is used from one turn. The test is single-threaded.

The filter allocates a new vec. The bench times that allocation for 100 messages. The original slice is not modified.

## Tests

Four tests. Scratch is dropped, two messages remain, and the owner is `draft`. Three more handoffs reach the cap. The next one is `Limit` and the owner stays `agent-3`. A transcript of only scratch is `Empty`, the owner stays `outline`, and the count stays 0. Filter keeps `a` then `c`. The demo string is `draft sees 2`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch24-handoffs
```

```text
Starting 4 tests across 3 binaries
     Summary [   0.003s] 4 tests run: 4 passed, 0 skipped
```

```bash
cargo run -p ch24-handoffs
```

```text
draft sees 2
```

The binary filters one scratch line and moves the owner. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `filter_100` | 2.664 µs | under 1 ms | drop scratch from 100 messages |

`filter_100` builds 100 messages before the bench. Every fifth role is `scratch`. The rest are `user`. Each text is `message N about the rename procedure`. The timed closure is `b.iter`. Inside: `filter_transcript` with the slice passed through `black_box`, then `black_box` of the new vec. No board is updated. No model is called.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38986915984](https://github.com/agnathan/rust-agent-harness/actions/runs/38986915984), section `28`.

### Milestone demo

The `cargo run` above. The next owner is `draft`. It sees 2 messages.

## Pitfalls

Four is not a Hermes number. Raising it to match a config key that does not exist would be inventing a source.

The filter drops a role, not a pattern inside the text. A user message that says "scratch" is kept. A scratch message that contains the real answer is dropped. Put the answer in a role the next agent is allowed to see.

`Empty` is checked before the count moves. A handoff of nothing must not burn one of the four.

The fifth refusal restores the count. Forgetting the `fetch_sub` would leave the count at 5 and make a later successful handoff impossible even after you raised the cap in a new board. This board does not reset.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Filter.** The next owner receives the filtered vec. Scratch does not cross. That is the context filter on this row. Hermes `handoff.request` does not do it. The messages are still in the caller's slice.

**Cap.** `MAX_HANDOFFS` is 4. The fifth call is `Limit`. The owner string is unchanged.

**Left for later.** Platform re-bind and the busy-session check are chapter 37. E1 and E2 are still absent. A scratch line is not scanned for secrets here.

## Chapter summary

- **Filter.** Role `scratch` is dropped. Order of the rest is kept.
- **Cap.** Four handoffs. The fifth is refused. The number is the harness cap.
- **Owner.** Changes only after a non-empty filtered transcript is accepted.
- **Time.** Filtering 100 messages measured **2.664 µs**.

[personal note: a transfer that was really a platform change, and the filter you still wanted]

## What's next

Chapter 29 schedules a hundred mock subagents onto a queue that stops at 100, runs them under the same bound of eight, and gathers the replies in order.
