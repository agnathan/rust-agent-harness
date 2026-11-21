# Chapter 31: Fan-out and Gather

Sixteen replies in the bound test, a hundred slots in the queue test, and a peak of 8 in `max_seen`. Those are the three numbers, and the awkward one sits between them: `gather` starts one scoped thread per queued task, and each thread blocks in `pool.acquire` until a permit is free. A full queue can leave up to 100 threads waiting on a pool of 8. A worker pool that pulled from the queue would start a thread only when a permit was free. This `gather` starts them anyway, because it reuses chapter 29's `Pool` instead of growing a second scheduler. That is wasteful, and it's the design the tests lock.

The demo schedules `c`, then `a`, then `b`, and prints them in index order:

```text
a
b
c
```

Finish order is allowed to differ. The parent, and the critic after it, read `task_index` order, so a continuously improving harness doesn't learn a lesson from whichever reply happened to join first. The trace `secret-tool` stays on the task and stays out of that string.

`schedule` accepts until the deque's length is 100, then the next call is `FanError::Full`, and the constant is 100. The file records no measurement that picked 100 rather than another length. Hermes rejects a new async dispatch at the concurrency cap and doesn't queue it. This board queues, and the rejection is the 101st `schedule`, while the pool of 8 is still how many actually run. Heartbeats stay deferred until chapter 36.

Chapter 32's critic reads the summary and doesn't do the hundred tasks. Chapter 33 refuses a nested fan-out whose width would pass the pool, before any task starts. Chapter 35's mailbox is a different bound, 64, on turns, and a hundred task results are not a hundred skills. The parent, or chapter 26's reviewer after the parent finishes, decides what was a procedure and what was a one-off. Writing a skill per task index is the pile chapter 27 exists to clean up.

The step is chapter 30's harness plus `fanout`, at `steps/ch31-fan-out`, package `ch31-fan-out`, and the cap is a length check before `push`. The index is the sort key, so completion order and result order are different vecs. The permit is chapter 29's guard, taken when the work runs rather than when it's queued. Section 31.6 is only the cross-reference back to that guard.

`cargo run --locked -p ch31-fan-out --bin ch31-fan-out` prints the three sorted lines. `my-harness/tests/ch31.rs` checks that the 101st task is refused.

**In this chapter**

- 31.1 The task is an index, a reply, and a trace
- 31.2 `schedule` rejects the 101st
- 31.3 `gather` sorts replies and leaves the trace behind
- 31.4 The demo schedules out of order and prints sorted lines
- 31.5 `schedule_100` fills the queue and doesn't gather
- 31.6 The permit `gather` takes is still chapter 29's

| Section | Step | What it adds |
|---|---|---|
| 31.1 | `steps/ch31-fan-out` | `Task`, `Board`, `QUEUE_CAP` of 100 |
| 31.2 | `steps/ch31-fan-out` | `schedule` and `FanError::Full` |
| 31.3 | `steps/ch31-fan-out` | `gather`, `thread::scope`, sort by `task_index` |
| 31.4 | `steps/ch31-fan-out` | the demo lines `a`, `b`, `c` |
| 31.5 | `steps/ch31-fan-out` | `schedule_100` |

> **How Hermes does it**
>
> I read `tools/delegate_tool_dispatch.py` (`_run_children_parallel`) and `tools/delegate_tool_config.py` (`_get_max_async_children`) on Hermes `main` at `ab3fe9dffb0a`.
>
> `_run_children_parallel` is the parallel child run. `_get_max_async_children` says the concurrency cap for background delegations equals `delegation.max_concurrent_children`. A leftover `delegation.max_async_children` key is ignored with a one-time warning. At capacity a new async dispatch is rejected, not queued.
>
> **Adopted.** Parallel children. Rejection at capacity. The parent summary is the replies.
>
> **Adapted.** `QUEUE_CAP` is 100. The cap is a queue of 100, not the concurrent-children number. The 101st `schedule` returns `FanError::Full`. `gather` sorts by `task_index` and returns replies only, not `tool_trace`. The pool that runs the tasks is still `WORKER_BOUND` of 8. The queue of 100 and the `task_index` order were deferred from chapter 29 to here.
>
> **Deferred.** Heartbeats, to chapter 36. This chapter does not beat. It schedules, refuses at 100, and gathers.

## What this chapter decides

`Board::schedule` pushes a `Task` while the queue length is below 100, and at length 100 it returns `Err(FanError::Full)` and doesn't push. A task carries `task_index`, `reply`, and `tool_trace`.

`gather` drains every task under the queue mutex, drops that lock, and starts one scoped thread per task. Each thread calls `pool.acquire()`, and the row it returns is `(task_index, reply.clone())`. `tool_trace` isn't copied into the row, and after the joins, `sort_by_key` orders the rows by index. The summary is the replies joined with `\n`.

`gather_holding` is the same path with a sleep inside the permit. `gather` passes `Duration::ZERO`, and the sleep is skipped when the duration is zero. Sixteen tasks held for 20 ms peak at `max_seen` of 8.

`demo` schedules indexes 2, 0, and 1, and returns `a`, `b`, and `c` on three lines. No key.

`schedule_100` fills a fresh board inside the timed closure, and it doesn't call `gather`. The mean is `4.12 µs`.

## 31.1 The task is an index, a reply, and a trace

`fanout` imports the pool from the same crate.

```rust
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::supervisor::{Pool, WORKER_BOUND};
```

`VecDeque` is the queue. `Mutex` guards it. `thread` is `thread::scope` and `thread::sleep`. `Duration` is the hold. `Pool` and `WORKER_BOUND` are chapter 29's types. `WORKER_BOUND` is 8. `acquire` still waits on that pool's condvar, and this file doesn't declare a second limit.

```rust
pub const QUEUE_CAP: usize = 100;
```

100 is the queue, and it isn't 8, and it isn't Hermes's concurrent-children default of 10. Chapter 29's reflection deque is a private `QUEUE_CAP` of 64 inside `supervisor.rs`, and this const is public and lives in `fanout`. `schedule` reads this one.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub task_index: usize,
    pub reply: String,
    pub tool_trace: String,
}
```

`task_index` is the sort key, and it isn't the position in the deque, and the demo pushes index 2 first. `reply` is the line the parent will see. `tool_trace` is the string the tests set to `secret-tool`. `Clone` is derived. `gather` clones `reply` and doesn't clone the whole `Task`, so the trace isn't carried by that clone.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanError {
    Full,
}
```

`Copy` means `Err(FanError::Full)` can be compared without moving the only copy, and the test uses `assert_eq!` against `Err(FanError::Full)`. There is one variant, and the refusal is "the queue is at `QUEUE_CAP`", and it isn't a tool error and it isn't `TeamError`.

```rust
pub struct Board {
    queue: Mutex<VecDeque<Task>>,
}
```

One mutex, and the deque is empty in `new`.

```rust
impl Board {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.lock().expect("queue").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
```

`len` locks, reads the length, and drops the guard. `is_empty` calls `len`, so it locks again. `expect("queue")` panics if the mutex is poisoned. `Default` forwards to `new`.

```rust
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
```

The tests call `Board::new()`, and the default is the same empty deque.

## 31.2 `schedule` rejects the 101st

```rust
    pub fn schedule(&self, task: Task) -> Result<(), FanError> {
        let mut queue = self.queue.lock().expect("queue");
        if queue.len() == QUEUE_CAP {
            return Err(FanError::Full);
        }
        queue.push_back(task);
        Ok(())
    }
```

> **Rust sidebar: `Result`**
>
> - `Result<(), FanError>` uses `()` as the success payload. On `Err`, the `task` binding is dropped at the end of the function because `push_back` never took it.
> - The length check and the `push_back` share one guard. Another `schedule` cannot pass the length check until this guard drops. The cap holds for concurrent `schedule` calls because they serialize on this mutex.
> - `return Err` happens while the guard is still alive.
> - `Option`, `Result`, enums, and `Arc` are taught in [chapter 1](chapter-01-first-contact.md).

`QUEUE_CAP` is 100. Lengths 0 through 99 push, and the call that observes length 100 returns `Full`. Hermes rejects a new async dispatch when the concurrency cap is hit, and it doesn't queue that dispatch. This board queues, and the rejection is the 101st `schedule`. The adaptation is the number and the place: a queue of 100, not the concurrent-children count. The concurrent limit is still the pool of 8, applied later in `gather`, not here.

The test fills the board and then refuses.

```rust
#[test]
fn the_queue_holds_one_hundred_and_refuses_the_next() {
    let board = Board::new();
    for index in 0..QUEUE_CAP {
        board.schedule(task(index, "ok")).unwrap();
    }
    assert_eq!(board.len(), QUEUE_CAP);
    assert_eq!(board.schedule(task(QUEUE_CAP, "no")), Err(FanError::Full));
    assert_eq!(board.len(), QUEUE_CAP);
}
```

`0..QUEUE_CAP` is `0..100`. One hundred `unwrap`s leave the length at 100, and the next task uses index 100 and reply `no`. The `Result` equals `Err(FanError::Full)`, and the length is still 100, so the refused task was not stored. The helper sets `tool_trace` to `secret-tool` on every task, including the one that is dropped.

```rust
fn task(index: usize, reply: &str) -> Task {
    Task {
        task_index: index,
        reply: reply.into(),
        tool_trace: "secret-tool".into(),
    }
}
```

The milestone asks `chapter_31_refuses_overflow(99)` to be false and `chapter_31_refuses_overflow(100)` to be true. Ninety-nine tasks already queued leave room for one more, and one hundred already queued is the `Full` case this test hits. The stub panics until that function exists, and the reference behavior is this `schedule`.

A leftover Hermes key named `delegation.max_async_children` is ignored with a one-time warning. `_get_max_async_children` says the background cap equals `delegation.max_concurrent_children`, and this file doesn't read config. `QUEUE_CAP` is the const 100. I am not treating 100 as that concurrent-children number, and eight permits remain the run cap. One hundred slots remain the queue cap.

## 31.3 `gather` sorts replies and leaves the trace behind

`gather` is a thin wrapper, and the work is `gather_holding`.

```rust
    pub fn gather(&self, pool: &Pool) -> String {
        self.gather_holding(pool, Duration::ZERO)
    }

    pub fn gather_holding(&self, pool: &Pool, hold: Duration) -> String {
        let tasks: Vec<Task> = self.queue.lock().expect("queue").drain(..).collect();
        let mut rows: Vec<(usize, String)> = thread::scope(|scope| {
            let handles: Vec<_> = tasks
                .iter()
                .map(|task| {
                    scope.spawn(|| {
                        let _permit = pool.acquire();
                        if !hold.is_zero() {
                            thread::sleep(hold);
                        }
                        (task.task_index, task.reply.clone())
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|handle| handle.join().expect("worker"))
                .collect()
        });
        rows.sort_by_key(|(index, _)| *index);
        rows.into_iter()
            .map(|(_, reply)| reply)
            .collect::<Vec<_>>()
            .join("\n")
    }
```

The drain takes the whole deque.

`lock` yields the guard. `drain(..)` removes every element and yields them. `collect` builds a `Vec<Task>`. The guard is a temporary that lives until the statement ends, so the drain finishes before the lock is released. After the semicolon the queue is empty and the mutex is free, and the workers don't lock `Board`. They lock the pool inside `acquire`.

`thread::scope` is the same scoped spawn as `run_specialists`, and the threads borrow `task` and `pool`. They finish before `thread::scope` returns. `join().expect("worker")` panics if a worker panics, and the rows vec is spawn order until `sort_by_key`.

`_permit` holds one slot from the chapter 29 pool. `WORKER_BOUND` is 8, so a ninth task waits on the condvar. `hold.is_zero()` is true for `Duration::ZERO`, and `gather` passes that zero, and the sleep runs only for a non-zero hold. `gather_holding` is how the bound test makes the overlap visible.

The tuple is `(task.task_index, task.reply.clone())`. `task_index` is `usize`, which is `Copy`. `reply.clone()` is a new `String`. `tool_trace` isn't read, and the parent string can't contain `secret-tool` unless that text was also the reply. The tests use reply `ok`, `a`, `b`, or `c`, and the trace `secret-tool`.

`sort_by_key(|(index, _)| *index)` orders by the index. Spawn order is discarded as the sort key. A task scheduled first with index 2 prints after a task scheduled later with index 0. The pattern `|(index, _)|` borrows the tuple. `*index` copies the `usize` out as the key.

The map drops the index and keeps the reply. `collect::<Vec<_>>()` builds the vec `join` needs. `join("\n")` puts a newline between replies and doesn't add one after the last reply. Three replies become `a\nb\nc`.

The sort test schedules out of order and checks the string.

```rust
#[test]
fn gather_sorts_summaries_and_hides_tool_traces() {
    let board = Board::new();
    board.schedule(task(2, "c")).unwrap();
    board.schedule(task(0, "a")).unwrap();
    board.schedule(task(1, "b")).unwrap();
    let pool = Pool::new(WORKER_BOUND);
    let summary = board.gather(&pool);
    assert_eq!(summary, "a\nb\nc");
    assert!(!summary.contains("secret-tool"));
    assert_eq!(board.len(), 0);
}
```

Index 2 is queued first, and the summary is `a`, then `b`, then `c`. `contains` is false for the trace. `len` is 0 because `drain` emptied the board. A second `gather` on that board would join an empty list into an empty string. The test stops at the first gather.

The bound test queues 16 tasks and holds each permit for 20 ms.

```rust
#[test]
fn gather_respects_the_worker_bound() {
    let board = Board::new();
    for index in 0..16 {
        board.schedule(task(index, "ok")).unwrap();
    }
    let pool = Pool::new(WORKER_BOUND);
    let summary = board.gather_holding(&pool, Duration::from_millis(20));
    assert_eq!(summary.lines().count(), 16);
    assert_eq!(pool.max_seen(), WORKER_BOUND);
}
```

Sixteen lines, and `max_seen` equals `WORKER_BOUND`. The queue accepted 16 because 16 is under 100, and the pool's peak is the evidence of the bound. `summary.lines().count()` is 16 because `join("\n")` inserts a newline between replies and not after the last one, so sixteen `ok` lines are sixteen pieces. A trailing newline would add an empty extra line, and the indexes were `0..16`, so the sort doesn't change the text. `join` itself is the one call at the end of `gather`, and `join().expect("worker")` earlier in the same function is the thread join: it panics if a worker panics, and the rows stay in spawn order until `sort_by_key`.

`Pool::new(WORKER_BOUND)` is the chapter 29 constructor, and this chapter calls `acquire` and leaves the function where chapter 29 put it. A task waiting on the condvar waits until a permit drops. Heartbeats, a pulse, a deadline, a restart, stay in chapter 36.

## 31.4 The demo schedules out of order and prints sorted lines

```rust
pub fn demo() -> String {
    let board = Board::new();
    let pool = Pool::new(WORKER_BOUND);
    for (index, reply) in [(2, "c"), (0, "a"), (1, "b")] {
        board
            .schedule(Task {
                task_index: index,
                reply: reply.into(),
                tool_trace: "secret-tool".into(),
            })
            .expect("schedule");
    }
    board.gather(&pool)
}
```

The array is three pairs, and the loop schedules index 2, then 0, then 1. Each `expect("schedule")` panics on `Full`, and three is below 100, so each call is `Ok(())`. `gather` sorts and joins, and the string is `a\nb\nc`. The trace is on every task and is absent from the string, for the same reason as the unit test: the worker tuple clones `reply` only.

The test compares the whole string.

```rust
#[test]
fn the_demo_is_sorted() {
    assert_eq!(harness_core::fanout::demo(), "a\nb\nc");
}
```

The binary prints that string with `println!`, so the terminal shows three lines.

```rust
use harness_core::fanout::demo;

fn main() {
    println!("{}", demo());
}
```

The package binary is `ch31-fan-out`. It takes no key, and the replies are the literals `c`, `a`, and `b`.

## 31.5 `schedule_100` fills the queue and does not gather

The bench is `benches/fan_out.rs`. `Cargo.toml` names it `fan_out` with `harness = false`, and the workflow section is `31`, label `chapter 31 fan-out`, package `ch31-fan-out`.

```rust
fn schedule_100(c: &mut Criterion) {
    c.bench_function("schedule_100", |b| {
        b.iter_batched(
            Board::new,
            |board| {
                for index in 0..QUEUE_CAP {
                    board
                        .schedule(Task {
                            task_index: index,
                            reply: format!("reply {index}"),
                            tool_trace: "secret-tool".into(),
                        })
                        .expect("schedule");
                }
                black_box(board.len());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, schedule_100);
criterion_main!(benches);
```

`iter_batched` takes a setup closure, a timed closure, and a batch size. `Board::new` is the setup, and it isn't the timed work, and the timed closure schedules indexes `0..100`, which is 0 through 99, one hundred tasks. Reply text is `reply {index}`, and the trace is `secret-tool` again. `expect("schedule")` is safe inside this loop because the board starts empty and the loop stops at 100. `black_box(board.len())` keeps the pushes in the sample. `len` locks and returns 100.

No `Pool` is built, `thread::scope` doesn't run, and `gather` isn't called. The mean is the cost of filling the queue, which is the rejection boundary the next `schedule` would hit. The mean is `4.12 µs`, and it's `mean.point_estimate`.

`BatchSize::SmallInput` is the Criterion batch size argument on this call, and the setup produces a small board, an empty deque, and the routine fills it.

The imports at the top of the bench file are `black_box`, `criterion_group`, `criterion_main`, `BatchSize`, `Criterion`, and `Board`, `Task`, `QUEUE_CAP` from `harness_core::fanout`. `Pool` isn't imported, and the bench can't accidentally gather.

## 31.6 The permit `gather` takes is still chapter 29's

`gather` calls `pool.acquire`, and that function, its `Condvar`, and the `Drop` that wakes one waiter are the pool from chapter 29, copied forward in this crate. Sixteen tasks on a limit of 8 therefore peak at `max_seen` of 8, which the bound test already asserts, and the opening is where the hundred waiting threads are owned as a design.

The two waits use two locks. `schedule` holds `Board.queue` only for the length check and the `push_back`, and `acquire` holds `Pool.state` while it decides whether to increment or sleep. A worker blocked on the condvar has already dropped the queue guard, because `drain` finished before `thread::scope` started, so those hundred threads don't stall a later `schedule` on the queue mutex. They stall on the pool. The unit tests don't call `schedule` during `gather`, and the split is still what the two locks are for.

`tool_trace` stays on the `Task` until the scope returns and `tasks` drops. The closure clones `reply` and never reads the trace, so `secret-tool` can sit in the drained vec and still be absent from the summary the parent, and the next chapter's critic, will read. `assert!(!summary.contains("secret-tool"))` is that split, not a second redactor.

The bench's `4.12 µs` includes the `format!` of `reply {index}` inside the timed closure. `Board::new` is outside `b.iter`. Moving the format into the setup would time a different sample than `schedule_100`. The closure stops at index 99, and the unit test is the one that passes index 100 and expects `FanError::Full`.

## Tests

Four tests in `tests/fan_out.rs`.

One hundred `schedule` calls succeed, and the next is `FanError::Full`, and the length stays 100.

Gather of indexes 2, 0, 1 equals `a\nb\nc`, and the string doesn't contain `secret-tool`. The board length is 0.

Sixteen tasks held for 20 ms produce 16 lines and `max_seen` of 8.

`demo` equals `a\nb\nc`.

The package also runs the tests copied forward from chapter 30, including the supervisor tests and the handoff tests. Nextest runs all of them.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch31-fan-out
```

```text
Starting 159 tests across 38 binaries
     Summary [   0.360s] 159 tests run: 159 passed, 0 skipped
```

```bash
cargo run --locked -p ch31-fan-out --bin ch31-fan-out
```

```text
a
b
c
```

The binary schedules three tasks out of order and prints the gathered replies, and no key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `schedule_100` | 4.12 µs | fill the queue, then the next call is `Full` | `schedule` of indexes 0 through 99 on a fresh board |

`schedule_100` doesn't include `Board::new` in the timed closure, and it doesn't include `gather`. The tool trace is stored on the task and never formatted into the bench's `black_box` value, which is `len`.

The lines are `a`, `b`, and `c`.

The means are `mean.point_estimate` from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `31`. Bench ids: `31/schedule_100`, `29/schedule_100`, and how to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The queue cap and the permit cap are different numbers. `QUEUE_CAP` is 100. `WORKER_BOUND` is 8, and a full board holds 100 tasks. `gather` still runs eight at a time. Filling the board doesn't start 100 threads at once. It starts one scoped thread per drained task, and those threads block inside `acquire` past eight.

`tool_trace` exists so a test can show the summary omitted it. A format of the whole `Task` inside the worker would put `secret-tool` in the parent string. The worker returns the index and the reply clone, and that is the whole row.

Sort uses `task_index`, not finish time, and a fast task with a high index is still last after `sort_by_key`. Join order before the sort is spawn order, and the string the caller sees is the sorted one.

Hermes rejects a new async dispatch at the concurrency cap and doesn't queue it. This `schedule` queues until 100 and then returns `Full`, and both are a refusal at a cap. The cap in this file is the queue length, and the cap in `acquire` is 8, and it waits instead of returning `Full`. A task already on the board isn't rejected when the pool is busy, and it waits for a permit.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Bound.** `gather` takes permits from `Pool` with `WORKER_BOUND` of 8. Sixteen holders peaked at `max_seen` 8. M5 and M6 still live on `child_call` in the copied supervisor, and this chapter doesn't add a second permit count.

**Queue.** `QUEUE_CAP` is 100, and the 101st `schedule` is `FanError::Full`, and the length stays 100, and that refusal is the adaptation of "at capacity, reject". The capacity adapted here is the queue, not `delegation.max_concurrent_children`.

**Summary.** The parent string is replies in `task_index` order. `tool_trace` stays on the `Task` and isn't cloned into the row.

**Left for later.** Heartbeats are chapter 36. Chapter 32's critic reads a finished reply, takes one permit from this same pool, and doesn't write the notes.

## Chapter summary

- **Queue.** 100 tasks, and the next `schedule` is `FanError::Full`, and the refused task isn't stored.
- **Bound.** `gather` uses the supervisor pool. `WORKER_BOUND` is 8. `max_seen` peaks at 8.
- **Aggregator.** `sort_by_key` on `task_index`, and the summary is the replies joined with newlines.
- **Trace.** `tool_trace` isn't in the summary, and the demo's trace is `secret-tool` and the printed lines are `a`, `b`, `c`.
- **Time.** `schedule_100` mean `4.12 µs`, and that is `mean.point_estimate`.

## What you implement

`my-harness/tests/ch31.rs` is the milestone.

```rust
use my_harness::chapter_31_refuses_overflow;

#[test]
fn the_101st_task_is_refused() {
    assert!(!chapter_31_refuses_overflow(99));
    assert!(chapter_31_refuses_overflow(100));
}
```

The stub panics.

```rust
pub fn chapter_31_refuses_overflow(already: usize) -> bool {
    let _ = already;
    todo!("chapter 31")
}
```

`todo!("chapter 31")` is the panic, and the test wants `chapter_31_refuses_overflow(99)` to be false and `chapter_31_refuses_overflow(100)` to be true.

## What's next

Chapter 32 is the critic, and it takes one permit from this pool, locks the notes, and doesn't push. `accepted` is true only for `Verdict::Done`, and a missing verdict word is `Reject`, and the 30 second timeout is a constant, and the function doesn't sleep it.
