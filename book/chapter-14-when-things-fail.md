# Chapter 14: When Things Fail

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 13 can kill a child that does not finish. It does not decide what the turn should do next. An empty reply, a reply cut off for length, a reply that repeats one span, a timeout, and a crashed session are different failures. This chapter names them and picks retry, fallback, or stop. The empty-reply retries and the repetition guard were deferred from chapter 9. They land here.

The step crate is `steps/ch14-when-things-fail`. The book number is 14. The package is `ch14-when-things-fail`.

| Section | Step | What it adds |
|---|---|---|
| 14 | `steps/ch14-when-things-fail` | `Kind`, empty retries, repetition, a timeout that drops its permit |

> **How Hermes does it**
>
> I read `agent/empty_response_guard.py`, `agent/repetition_guard.py`, `agent/turn_truncation.py`, and `agent/deadline.py` (`kill_process_tree` is chapter 13; the timeout class is this chapter) on Hermes `main` at `cf71b1aa`.
>
> An empty completion retries up to 3 times. If one attempt costs more than $0.25, the budget drops to 1. Two consecutive empties with usage present and zero output tokens, from the same model, provider, and finish reason, skip the rest and go to the fallback. `is_repetition_dominated` is true when the text is at least 400 characters and a 60-character window repeats enough to cover at least half of it, with a floor of 5 repeats. `REPETITION_LOOP_INTERRUPTED` is the text stored instead of the loop. `turn_truncation.py` continues a `finish_reason == "length"` reply, and the repetition guard refuses to continue a loop.
>
> **Adopted.** The budgets 3 and 1, the $0.25 threshold (strictly greater), the two-empty deterministic rule, the 400 / 60 / 5 / half constants, and the interrupted-loop sentence. A length reply that is not a loop is kept and marked truncated. A crashed fake session reconnects and the next call returns `pong`.
>
> **Adapted.** The repetition check is the window count. I did not port the line fast path or the anchor scan. Fallback is the next `Model` in a slice, not Hermes's provider chain. The timeout uses `tokio::time::timeout` and drops an owned semaphore permit. Retries take that permit one at a time.
>
> **Deferred.** Preflight compression before the model call stays chapter 32. Think-block stripping is chapter 17. Rate-limit backoff on the server is chapter 15.

[personal note: an empty reply you would have retried, and what the cost did to the budget]

## What this chapter decides

`Kind` is `Empty`, `Truncated`, `Repeated`, `Timeout`, `RateLimit`, `Transport`, or `Crash`.

`retry_budget(None)` is 3. `retry_budget(Some(0.25))` is still 3. `retry_budget(Some(0.26))` is 1. The threshold is exceeded, not met.

`deterministic_empty` needs two attempts. Both must have usage, both must have zero output, and the model, provider, and finish reason must match. Missing usage does not count. The fake turn then keeps going until the budget is spent. Three unclear empties and then the text `ok` stay on the first model.

Two zero-output empties skip the rest of that model, including a later text reply, and the next model answers. The demo is that case with a cost of $0.30, so the reduced budget and the deterministic rule agree. The second model says `hello`. `fallbacks` is 1.

`is_repetition_dominated` returns false under 400 characters. `"abcdefghij"` repeated to 800 characters returns true. `length_verdict("length", that)` is `StopRepeated`, and `run_models` stores the interrupted-loop sentence. A short length reply is `Continue`. A `stop` finish is `Finished`.

`call_with_timeout` acquires one permit from a semaphore of 8, races the work against the timeout, and drops the permit on both arms. The test sleeps five seconds inside the work and times out at 30 milliseconds. The permit count is 8 again, and the test finishes in under two seconds.

`retry_in_bound` acquires, records how many permits are out, and drops, three times. The peak is 1. A retry does not hold a second permit.

`FakeMcp::new(1)` fails once with `Crash`, counts a reconnect, and then returns `pong`.

## Rust: `timeout` returns which arm finished

`tokio::time::timeout(duration, future)` waits for the future or the duration. `Ok` is the future's value. `Err` is the elapsed timeout. The permit is an `OwnedSemaphorePermit`. It is not tied to the function scope by a reference. `drop(permit)` before the `match` puts the slot back even though both arms return. Leaving the permit inside the `Ok` arm until the caller finished parsing would hold the bound across work that is not the tool.

`Reply::Empty` stores the cost as `OrderedF64`, an integer number of micros. `f64` is not `Eq`, and the tests match on replies.

## Tests

Eight tests. The budgets are 3 and 1. Two matching zero-output empties are deterministic, and a pair that disagrees on zero output is not. Those two empties skip a later `late` and the fallback says `hello`. Three empties without usage still reach `ok`. A short repeat is kept, a long one stops continuation, and the turn text is the interrupted-loop sentence. The timeout drops the permit. Three retries peak at one permit. A crashed session reconnects once.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch14-when-things-fail
```

```text
Starting 8 tests across 3 binaries
     Summary [   0.035s] 8 tests run: 8 passed, 0 skipped
```

```bash
cargo run -p ch14-when-things-fail
```

```text
result: hello
fallbacks: 1
```

The binary runs two fake models. The first emits two empty replies at $0.30. The second says `hello`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `empty_budget` | 0.9040 ns | under 1 µs | `retry_budget(Some(0.30))` |
| `repetition_guard` | 2.551 µs | under 1 ms | the window scan on an 800-character loop |

`empty_budget` calls `retry_budget` with `Some(0.30)` passed through `black_box`. The timed closure is `b.iter`. There is no model and no session. The result is the reduced budget, 1.

`repetition_guard` builds `"abcdefghij"` repeated 80 times before the bench (800 characters). Each sample calls `is_repetition_dominated` on that string. Building the string is excluded. The scan is a `HashMap` of 60-character windows. It does not call a model.

The timeout is not a bench. The test above is the measurement of the cancel: a 30 millisecond timeout against a 5 second sleep, permits back to 8, elapsed under 2 seconds. A bench that slept would measure the timer, not the permit.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38993207081](https://github.com/agnathan/rust-agent-harness/actions/runs/38993207081), section `14`.

### Milestone demo

The `cargo run` above. The result is `hello`. One fallback.

## Pitfalls

Two zero-output empties never reach a third reply on that model. If you wanted the model to "try once more" after a proven empty pair, this rule will not do it. The fallback is the next try.

`retry_budget` only looks at the first empty's cost. A later cheap empty does not raise the budget again.

A length reply under 400 characters is continued even if it is `"ok ok ok"`. The guard fails open on short text, which is what Hermes does. Do not point this function at a one-line stutter and expect a stop.

`call_with_timeout` drops the permit when the timeout wins. The work future is dropped too, which is what cancels the sleep. A tool that has already started a side effect does not get undone. The permit is the only thing this function returns.

`FakeMcp` stops reconnecting after three crashes. The test uses one.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layers L3 and L4.

**J4.** Empty replies retry 3 times, or 1 time when one attempt costs more than $0.25. Two consecutive zero-output empties with usage skip the rest and fall back. Missing usage keeps the full budget.

**J3.** At least 400 characters, a 60-character window, at least 5 repeats, covering at least half the text. The stored text is the interrupted-loop sentence, not the loop itself.

**Length continuation.** `finish_reason == "length"` keeps the fragment unless the repetition guard fired. That is the deferral from chapter 9.

**Timeout.** `Kind::Timeout` drops the tool permit. Retries in `retry_in_bound` never hold two permits. The bound stays 8.

**Left for later.** Server-side 429 and 503 behavior is chapter 15. Think-block stripping is chapter 17 (K5). Outbound sanitization beyond this chapter's fake replies is chapter 16 (K4). Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Empty.** Three retries, or one if the attempt costs more than $0.25. Two proven empties fall back.
- **Repeat.** An 800-character loop is not continued. The turn stores the interrupted sentence.
- **Timeout.** The permit is back. The 5 second sleep does not finish.
- **Session.** One crash, one reconnect, then `pong`.
- **Time.** The budget check measured **0.9040 ns**. The repetition scan measured **2.551 µs**.

[personal note: a fallback you did not expect, and which empty rule sent the turn there]

## What's next

Chapter 15 is the server. A tool gets its own concurrency limit, a retry only when it is marked idempotent, and a breaker that stops calling a backend that is already failing. The web-search cap of 50, still missing after chapter 9, is there too.
