# Chapter 48: Seeing Inside

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 47 calls out of the process. Nothing in that call says how long it took, or which event it was, without putting the prompt in the log. This chapter is a span per event, a gateway child span, and a research team that writes a note the next run can see.

The step crate is `steps/ch48-tracing`. The book number is 46. The package is `ch48-tracing`. The library crate is `spans`, so it does not shadow the `tracing` dependency.

| Section | Step | What it adds |
|---|---|---|
| 48 | `steps/ch48-tracing` | a counting subscriber, content-free event names, 100 mock researchers |

> **How Hermes does it**
>
> I read `agent/monitoring/emitter.py` (`MonitoringEmitter.emit`), `agent/monitoring/events.py` (`GatewayHealthEvent`), `agent/monitoring/otlp_exporter.py` (the module docstring), and `agent/monitoring/redaction.py` (the module docstring) on Hermes `main` at `e31c674f`.
>
> `emit` must return in microseconds, must not block on disk or the network, and must not raise into the caller. The queue holds 10,000 events. On `Full` it drops the oldest. `GatewayHealthEvent` is content-free: no prompts, messages, tool args, or session history. The OTLP exporter is an optional extra with no default destination. `redaction.py` scrubs every string before it leaves the process.
>
> **Adopted.** The span field is the event name, not the prompt. A secret in the prompt does not appear in the sink. The subscriber counts spans and does not write a file. The gateway name is its own span under the turn.
>
> **Adapted.** There is no background dispatcher and no OTLP endpoint. `CountingSubscriber` hands out `Id`s and drops the fields. The research team is 100 `FakeModel` calls into one `Memory`, then one more call that sees the notes.
>
> **Deferred.** A collector, the 10,000-event ring, and drop-oldest. Chapter 50 appends a trajectory line. The load numbers are chapter 49.

[personal note: a span that was only a name]

## What this chapter decides

`safe_field` returns `"request"` for any prompt, including one that contains `sk-live-`.

`mock_turn` enters a `turn` span, emits `request`, calls `FakeModel`, emits `response`, enters a `gateway` span, and emits `gateway`. The sink receives those three names.

`CountingSubscriber` implements `tracing::Subscriber`. `new_span` increments an `AtomicU64` and returns that id. The other methods do nothing. Without a subscriber the `tracing` macros do not run. The bench installs this one.

`research_team` spawns `n` threads. Each calls `FakeModel` and pushes `"{index}:{reply}"`. A second call sees `last()`.

`demo` prints `trace: ok events=3 spans=5 team=101`. Five spans are the turn, the three emits, and the gateway span. 101 is 100 notes plus the follow-up. No key.

## Rust: a subscriber is what makes a span real

`tracing::info_span!` looks up the thread-local subscriber. `with_default` installs `CountingSubscriber` for the closure. `Id::from_u64` rejects 0, so the counter starts at 1. The bench's `event_without_span` calls `FakeModel` only. `span_per_event` calls `mock_turn` under the same subscriber. The difference is the spans.

## Tests

Three tests. A secret prompt yields sink `["request", "response", "gateway"]`, at least three spans, and `safe_field` is `"request"`. The sink text does not contain `sk-`. One hundred agents leave 100 notes, and one more agent leaves 101 whose last line ends in `kept`. The demo starts with `trace: ok events=3` and contains `team=101`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch48-tracing
```

```text
     Summary [   0.006s] 3 tests run: 3 passed, 0 skipped
```

```bash
cargo run -p ch48-tracing
```

```text
trace: ok events=3 spans=5 team=101
```

The binary records one turn and 101 notes. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `event_without_span` | 17.29 ns | the chapter 1 mock floor, no span | `FakeModel::complete("hello")` |
| `span_per_event` | 115.7 ns | a span per event under 0.1 ms, and the chapter 1 budget under 1 ms with tracing on | `mock_turn` under `CountingSubscriber` |

Both functions run inside one `with_default`, so the subscriber is installed before either sample. That install is not timed. `event_without_span` is `b.iter` of `complete` on `hello`. `span_per_event` is `b.iter` of `mock_turn`, which builds a three-name sink, enters the turn, request, response, and gateway spans, and calls `complete`. The sink allocation and the span enters are inside that sample. `research_team` is not called. No file is written. The prompt `hello` has no secret. The 100 agents are the test and the demo, not this sample.

The table is `mean.point_estimate` from Benchmarks run [38991844877](https://github.com/agnathan/rust-agent-harness/actions/runs/38991844877) on ubuntu-latest (`bench-run.json`, 17.288160993245015 ns and 115.74375520610401 ns). The span sample is 98.46 ns above the bare call, under 0.1 ms, and under the chapter 1 budget of 1 ms. The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `trace: ok events=3 spans=5 team=101`.

## Pitfalls

The bench is one turn. A number that spawns 100 threads is the research test. That test is allowed to take longer than 0.1 ms.

A subscriber that writes the prompt into a field would fail the secret test. `safe_field` exists so the span cannot grow a body later by accident.

`event_without_span` is the floor. Quoting only `span_per_event` hides the add. Both means are the comparison.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Content-free events (E1).** `GatewayHealthEvent` carries no prompt. The sink stores `request`, `response`, and `gateway`. A prompt of `sk-live-0123456789abcdef` is passed to the model and does not appear in the sink.

**No exporter on the hot path.** `emit` in Hermes must not block on the network. This subscriber increments an atomic and returns. There is no OTLP client.

**Gateway span.** The gateway event is a child of the turn, so one trace crosses the harness call and the gateway name. No socket is opened.

**Left for later.** The 10,000-event queue, drop-oldest, and redaction of e-mail and phone numbers. Chapter 49 measures tokens. Chapter 50 appends the trajectory line.

## Chapter summary

- **Subscriber.** Counts span ids. Does not store the prompt.
- **Turn.** `request`, `response`, `gateway`.
- **Team.** 100 mock agents, then one more that sees the notes.
- **Model.** `FakeModel` answers `ok` or `kept`. The demo needs no key.
- **Time.** One bare call measured **17.29 ns**. The same call with spans measured **115.7 ns**.

[personal note: five spans and none of them held the prompt]

## What's next

Chapter 49 is the server capstone: a load of in-memory dispatches, and tokens per completed task for every tool, a static set, and a dynamic set.
