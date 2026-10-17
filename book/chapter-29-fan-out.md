# Chapter 29: Fan-out and Gather

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 27 runs a handful of specialists. It does not show what happens when a hundred of them are waiting, and it does not sort their replies. This chapter's board accepts 100 tasks and refuses the 101st. Gather runs them on the chapter 27 pool, so eight permits still hold. The summary is the replies in `task_index` order. The tool trace on each task is not in that string.

The step crate is `steps/ch25-fan-out`. The book number is 29. The package is `ch25-fan-out`. It depends on `ch23-supervisor`.

| Section | Step | What it adds |
|---|---|---|
| 29 | `steps/ch25-fan-out` | a queue of 100, the same bound of eight, an aggregator |

> **How Hermes does it**
>
> I read `tools/delegate_tool_dispatch.py` (`_run_children_parallel`) and `tools/delegate_tool.py` (the module doc) on Hermes `main` at `8dcf6d5b`. I had already read `_get_max_async_children` in `tools/delegate_tool_config.py`.
>
> `_run_children_parallel` runs the batch's children together and sorts `results` by `task_index` before returning. The module doc says the parent only ever sees the delegation call and the summary result, never the child's intermediate tool calls or reasoning. `_get_max_async_children` says a new async dispatch at capacity is rejected, not queued, so a runaway model cannot pile up unbounded background work.
>
> **Adopted.** Parallel children, sort by `task_index`, and a summary that does not contain the tool trace. A full board refuses the next task.
>
> **Adapted.** `QUEUE_CAP` is 100. Hermes rejects when the concurrency cap is hit. This board queues up to 100, then returns `Full`, and gather still takes permits from the pool of 8. The replies are scripted strings on the task.
>
> **Deferred.** Heartbeats, credential leases, and the child's real tool log. Chapter 30's critic reads a summary like this one and does not do the work.

[personal note: a fan-out that came back in a different order than you launched it]

## What this chapter decides

`Board::schedule` pushes a `Task` until the queue length is 100. The next call is `FanError::Full` and the length stays 100. A task carries `task_index`, `reply`, and `tool_trace`.

`gather` drains the queue and starts one thread per task. Each thread acquires a permit from the chapter 27 pool. The result row is `(task_index, reply)`. `tool_trace` is not copied. The rows are sorted by index. The summary is the replies joined with newlines.

`gather_holding` sleeps inside the permit so a test can see `max_seen` reach 8. `gather` passes a zero duration. The bench does not call gather.

`demo` schedules indexes 2, 0, and 1 with replies `c`, `a`, and `b`, and prints them sorted. No key.

## Rust: drain, then sort

`drain(..)` takes every task out of the `VecDeque` under the mutex, then the lock is dropped before any thread starts. The workers do not share that mutex. They share the pool's permit mutex, which is the bound.

`sort_by_key` on the index is the aggregator. Spawn order is not the summary order.

## Tests

Four tests. 100 schedules succeed. The 101st is `Full`. Gather of indexes 2, 0, 1 is `a`, `b`, `c` on three lines, and the string does not contain `secret-tool`. The board is empty afterward. Sixteen tasks that each hold a permit for 20 ms peak at `max_seen` 8, and the summary has 16 lines. The demo is `a`, `b`, `c`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch25-fan-out
```

```text
Starting 4 tests across 3 binaries
     Summary [   0.045s] 4 tests run: 4 passed, 0 skipped
```

```bash
cargo run -p ch25-fan-out
```

```text
a
b
c
```

The binary schedules three tasks out of order and prints the gathered summaries. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `schedule_100` | 4.635 µs | 100 tasks, no unbounded queue | schedule 100 tasks onto a fresh board |

`schedule_100` uses `iter_batched`. The setup closure calls `Board::new` and is not timed. The timed closure schedules 100 tasks, indexes 0 through 99, reply `reply N`, tool trace `secret-tool`, then passes `len` through `black_box`. No pool is built. No thread is started. No model is called.

The table is `mean.point_estimate` from Criterion on this machine, with the Benchmarks flags `--warm-up-time 1 --measurement-time 2 --sample-size 10`. The workflow mean on ubuntu-latest replaces this number when that run is green.

### Milestone demo

The `cargo run` above. The lines are `a`, `b`, and `c`.

## Pitfalls

The queue cap and the permit cap are different. A board can hold 100 tasks. Gather still runs eight at a time. Filling the board does not mean 100 threads.

`tool_trace` is on the struct so a test can prove the summary omitted it. A later change that formats the whole `Task` into the summary would put `secret-tool` in the parent string.

Sort is by `task_index`, not by finish time. A fast task with a high index still prints last.

Hermes refuses an async dispatch at the concurrency cap instead of queueing it. This chapter queues, then refuses at 100. Both are bounded. They are not the same policy.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Bound.** Gather takes permits from `Pool` with `WORKER_BOUND` of 8. Sixteen holders peaked at 8. That is the same pool as chapter 27, including M5 and M6 on `child_call`. This chapter does not add a new permit count.

**Queue.** `QUEUE_CAP` is 100. The 101st schedule is `Full`. The queue does not grow past that.

**Summary.** The parent string is replies in index order. `tool_trace` stays on the task.

**Left for later.** Chapter 30 judges a finished summary and takes one permit from this same pool. E1 and E2 are still absent.

## Chapter summary

- **Queue.** 100 tasks. The next schedule is `Full`.
- **Bound.** Gather uses the supervisor pool of 8.
- **Aggregator.** Sort by `task_index`. The summary is the replies.
- **Trace.** Child tool text is not in the summary.
- **Time.** Scheduling 100 tasks measured **4.635 µs**.

[personal note: a batch that came back sorted, and the tool log you were glad the parent never saw]

## What's next

Chapter 30 is the critic. It reads the result, it does not do the work, and it takes one permit from the same pool. A missing verdict rejects.
