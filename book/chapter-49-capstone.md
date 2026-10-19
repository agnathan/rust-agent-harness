# Chapter 49: Load and Token Capstone

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 48 can see a turn. It does not say whether offering every tool, a fixed set, or a searched set spent fewer schema tokens on a task that still succeeded. Chapter 8 collects that telemetry. This chapter is the comparison, plus a load of 100 in-memory dispatches. There is still no socket.

The step crate is `steps/ch49-capstone`. The book number is 47. The package is `ch49-capstone`. It does not bind a port. `mcp-gateway/` is unchanged.

| Section | Step | What it adds |
|---|---|---|
| 49 | `steps/ch49-capstone` | three arms, a deadline, a load of 100, a nearest-rank p99 |

> **How Hermes does it**
>
> I read `tools/tool_search_catalog.py` (`CHARS_PER_TOKEN`) on Hermes `main` at `e31c674f`. The constant is `4.0`. A length check uses `ceil(len(text) / CHARS_PER_TOKEN)`.
>
> Hermes does not publish this three-arm comparison. The gateway docs in this repo already said chapter 49 would measure tokens per completed task instead of adopting another project's percentage. The dispatcher bound of 8 is the harness bound from chapter 3, applied here as waves of at most 8.
>
> **Adopted.** Schema tokens are `len / 4`, integer division. A task counts as success only when the needed tool is in the offered set.
>
> **Adapted.** The catalog is 32 synthetic cards, `tool_00` through `tool_31`, each schema `{"name":"tool_NN"}` (18 bytes, 4 tokens). Every arm offers 32. The static arm offers the first 8, which includes `tool_07`. The dynamic arm offers the one card whose name is the needed tool. The load is a loop, not oha, because this step does not listen.
>
> **Deferred.** A real embedder, a live upstream, and a socket. oha throughput stays unmeasured. Chapter 40 already said that.

[personal note: three token counts and one task that still finished]

## What this chapter decides

`sample_catalog` builds 32 cards. `schema_tokens` is `schema.len() / 4`.

`compare` returns three `Arm` values: `every`, `static`, and `dynamic`. Success requires the needed name to be offered. `deadline` fails an arm whose token count is above the limit. `None` means no limit.

`tool_07` succeeds on all three arms. Tokens are 128, 32, and 4. `tool_31` succeeds on every and dynamic, and fails on static, because the static set is `tool_00` through `tool_07`. `missing` fails all three.

`load` runs 100 requests. Each request calls `compare` for `tool_07` and `FakeModel`. A wave holds at most 8, then the in-flight count resets. 100 requests are 13 waves. `p99` sorts the per-request samples and takes index `ceil(0.99 * n) - 1`. For 1..=100 that index is 98 and the value is 99. The load report uses the same rank on the measured samples.

`admit` is false at 8.

`demo` prints the three token counts, three successes, `load=100/100`, and the p99 of that process. No key. No socket.

## Rust: integer tokens and a rank

`CHARS_PER_TOKEN` is 4, matching the Hermes constant, with integer division instead of `ceil`. A schema of 18 bytes is 4 tokens, not 5. The p99 rank uses `div_ceil` so a short sample still picks a real index. Empty input returns 0.

## Tests

Four tests. The three arms are 128, 32, and 4 tokens, all successful for `tool_07`. `tool_31` fails only the static arm. `missing` fails all three. A deadline equal to the dynamic token count fails every and static and keeps dynamic. `admit(7)` is true and `admit(8)` is false. `p99` of 1..=100 is 99. A load of 100 has 100 successes and 13 waves. The demo line contains `success=1/1/1` and `load=100/100`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch49-capstone
```

```text
     Summary [   0.004s] 4 tests run: 4 passed, 0 skipped
```

```bash
cargo run -p ch49-capstone
```

```text
capstone: every=128 static=32 dynamic=4 success=1/1/1 load=100/100 p99_ns=1160
```

That `p99_ns` is the debug binary's nearest-rank p99 of 100 in-memory dispatches on this run. It is not the Criterion mean. No key. No socket.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `dynamic_tokens` | 245.5 ns | the dynamic arm's token count | `compare` and the dynamic token field |
| `load_100` | 30.05 µs | the load of 100 dispatches | 100 compares, 13 waves, p99 computed inside |

`dynamic_tokens` builds the catalog before the bench. The timed closure is `b.iter`. Inside: `compare` of `tool_07` with no deadline, then `black_box` of `arms[2].tokens`. Catalog construction is excluded. `load` is not called.

`load_100` also builds the catalog before the bench. The timed closure calls `load` with 100 and `black_box` of the report. The 100 compares, the `FakeModel` calls, the wave counter, the sample vec, the sort, and the p99 index are inside the sample. No socket, no sleep, no oha. The deadline path is not taken. All 100 succeed.

The table is `mean.point_estimate` from Benchmarks run [38991844877](https://github.com/agnathan/rust-agent-harness/actions/runs/38991844877) on ubuntu-latest (`bench-run.json`, 245.5200445938794 ns and 30047.278824845125 ns). One hundred requests in 30.05 µs is about 3.33 million a second in that run, which is an in-memory loop, not a server. The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `capstone: every=128 static=32 dynamic=4 success=1/1/1 load=100/100 p99_ns=1160`.

## Pitfalls

The Criterion mean of `load_100` is the whole batch. The `p99_ns` in the demo is one debug process's per-request rank. They are not the same number.

Integer division makes 18 bytes 4 tokens. A ceil would make the arm totals different. The test pins 128, 32, and 4.

The static set is the first 8 names. `tool_07` is in it. `tool_31` is not. A static set that was "any 8" would not pin that failure.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Deadline (I6).** An arm over the token deadline is not a success, even when the tool is offered. The wide arms fail a deadline set to the dynamic count. The dynamic arm still succeeds.

**Bound.** `admit` refuses an in-flight count of 8. `load` resets the wave at 8. 100 requests are 13 waves, not one wave of 100.

**Success.** A completed task is a success only when the needed tool was offered and the model reply is `tool_07`. `missing` does not count.

**Left for later.** A listening socket, oha, and a live catalog. E1 and E2 are not this server's response path. The process still does not bind.

## Chapter summary

- **Arms.** Every tool 128 tokens, static 32, dynamic 4. All three succeed for `tool_07`.
- **Load.** 100 in-memory dispatches, 13 waves of at most 8.
- **p99.** Nearest rank. The debug demo on this run printed **1160 ns**.
- **Model.** `FakeModel` answers `tool_07`. The demo needs no key and no socket.
- **Time.** The dynamic arm measured **245.5 ns**. The load of 100 measured **30.05 µs**.

[personal note: the same task, three token counts]

## What's next

Chapter 50 re-runs the saved tasks from chapter 9, adds one red-team case per layer, and appends a trajectory line. There is still no fail threshold.
