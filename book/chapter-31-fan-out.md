# Chapter 31: Fan-out and Gather

Chapter 29 can run eight specialists at once. Chapter 30 can hand a task to one next owner. Neither of them is the question "what if there are a hundred independent tasks, and I need their replies in order?" A fan-out that allocates without a cap will start a hundred tasks and then a hundred more. This chapter adds the queue, the refusal, and the gather. The crate is `steps/ch31-fan-out`, a copy of chapter 30 plus `fanout`. The package is `ch31-fan-out`. Heartbeats are not this chapter. They were deferred by chapter 29 and they stay deferred until chapter 36.

A task is an index, a reply, and a trace. `schedule` accepts up to 100 and rejects the 101st. The cap is this book's, not a Hermes config key. Hermes's concurrent-children cap is a different number, and the harness queue of 100 is the one the tests lock. `gather` sorts replies and leaves the trace behind. The sort is by `task_index`, so a reply that finishes first is not the reply that prints first. The demo schedules out of order and prints sorted lines. The bench fills the queue and does not gather, which is how you measure scheduling without measuring the joins. The permit `gather` takes is still chapter 29's pool. Fan-out does not invent a second bound for the work it actually runs. The queue of 100 is how many tasks may be waiting. The pool of 8 is how many run. `gather` starts one scoped thread per queued task, and each thread blocks in `pool.acquire` until a permit is free. A full queue can therefore leave up to 100 threads waiting on a pool of 8. A worker pool that pulled from the queue would not park a thread per task. This chapter keeps the thread per task so the gather can reuse chapter 29's pool.

Chapter 32's critic reads the summary this gather produces. It does not do the hundred tasks. Chapter 33 refuses a nested fan-out whose width would exceed the pool, before any task starts. Chapter 35 will put each agent on its own mailbox, which is a different bound, 64, on turns rather than on tasks. Keeping those numbers apart is the design. One global "max concurrency" would make a full inbox and a full fan-out the same failure.

Learning stays off the gather. A hundred task results are not a hundred skills. The parent, or chapter 26's reviewer after the parent finishes, decides what was a repeated procedure and what was a one-off. Writing a skill per task index is how the index chapter 27 exists to clean up. This chapter returns ordered replies and stops.

Rust, at the queue: the cap is a length check before `push`. The index is an integer the sort key uses, so completion order and result order are different `Vec`s. `Result` is the refusal of the 101st. The pool permit is the guard from chapter 29, taken when the work runs rather than when it is queued.

`cargo run --locked -p ch31-fan-out --bin ch31-fan-out` prints the sorted lines. The exercise is `my-harness/tests/ch31.rs`: the 101st task is refused.

**In this chapter**

- 31.1 The task is an index, a reply, and a trace
- 31.2 `schedule` rejects the 101st
- 31.3 `gather` sorts replies and leaves the trace behind
- 31.4 The demo schedules out of order and prints sorted lines
- 31.5 `schedule_100` fills the queue and does not gather
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

`Board::schedule` pushes a `Task` while the queue length is below 100. At length 100 it returns `Err(FanError::Full)` and does not push. A task carries `task_index`, `reply`, and `tool_trace`.

`gather` drains every task under the queue mutex, drops that lock, and starts one scoped thread per task. Each thread calls `pool.acquire()`. The row it returns is `(task_index, reply.clone())`. `tool_trace` is not copied into the row. After the joins, `sort_by_key` orders the rows by index. The summary is the replies joined with `\n`.

`gather_holding` is the same path with a sleep inside the permit. `gather` passes `Duration::ZERO`, and the sleep is skipped when the duration is zero. Sixteen tasks held for 20 ms peak at `max_seen` of 8.

`demo` schedules indexes 2, 0, and 1, and returns `a`, `b`, and `c` on three lines. No key.

`schedule_100` fills a fresh board inside the timed closure. It does not call `gather`. The mean is `4.12 µs`.

## 31.1 The task is an index, a reply, and a trace

`fanout` imports the pool from the same crate.

```rust
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::supervisor::{Pool, WORKER_BOUND};
```

`VecDeque` is the queue. `Mutex` guards it. `thread` is `thread::scope` and `thread::sleep`. `Duration` is the hold. `Pool` and `WORKER_BOUND` are chapter 29's types. `WORKER_BOUND` is 8. `acquire` still waits on that pool's condvar. This file does not declare a second limit.

```rust
pub const QUEUE_CAP: usize = 100;
```

100 is the queue. It is not 8, and it is not Hermes's concurrent-children default of 10. Chapter 29's reflection deque is a private `QUEUE_CAP` of 64 inside `supervisor.rs`. This const is public and lives in `fanout`. `schedule` reads this one.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub task_index: usize,
    pub reply: String,
    pub tool_trace: String,
}
```

`task_index` is the sort key. It is not the position in the deque. The demo pushes index 2 first. `reply` is the line the parent will see. `tool_trace` is the string the tests set to `secret-tool`. `Clone` is derived. `gather` clones `reply` and does not clone the whole `Task`, so the trace is not carried by that clone.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanError {
    Full,
}
```

`Copy` means `Err(FanError::Full)` can be compared without moving the only copy. The test uses `assert_eq!` against `Err(FanError::Full)`. There is one variant. The refusal is "the queue is at `QUEUE_CAP`". It is not a tool error and it is not `TeamError`.

```rust
pub struct Board {
    queue: Mutex<VecDeque<Task>>,
}
```

One mutex. The deque is empty in `new`.

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

The tests call `Board::new()`. The default is the same empty deque.

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
> - `Result<(), FanError>` uses `()` as the success payload. The task has already been moved into the deque on the `Ok` path. The caller does not get the task back. On `Err`, the `task` binding is dropped at the end of the function because `push_back` never took it. The queue does not contain it.
> - The length check and the `push_back` share one guard. Another `schedule` cannot pass the length check until this guard drops. The cap holds for concurrent `schedule` calls because they serialize on this mutex.
> - `return Err` happens while the guard is still alive. The guard drops during unwind of that return, which unlocks. The length is unchanged.

`QUEUE_CAP` is 100. Lengths 0 through 99 push. The call that observes length 100 returns `Full`. Hermes rejects a new async dispatch when the concurrency cap is hit, and it does not queue that dispatch. This board queues, and the rejection is the 101st `schedule`. The adaptation is the number and the place: a queue of 100, not the concurrent-children count. The concurrent limit is still the pool of 8, applied later in `gather`, not here.

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

`0..QUEUE_CAP` is `0..100`. One hundred `unwrap`s. The length is 100. The next task uses index 100 and reply `no`. The `Result` equals `Err(FanError::Full)`. The length is still 100, so the refused task was not stored. The helper sets `tool_trace` to `secret-tool` on every task, including the one that is dropped.

```rust
fn task(index: usize, reply: &str) -> Task {
    Task {
        task_index: index,
        reply: reply.into(),
        tool_trace: "secret-tool".into(),
    }
}
```

The milestone asks `chapter_31_refuses_overflow(99)` to be false and `chapter_31_refuses_overflow(100)` to be true. Ninety-nine tasks already queued leave room for one more. One hundred already queued is the `Full` case this test hits. The stub panics until that function exists. The reference behavior is this `schedule`.

A leftover Hermes key named `delegation.max_async_children` is ignored with a one-time warning. `_get_max_async_children` says the background cap equals `delegation.max_concurrent_children`. This file does not read config. `QUEUE_CAP` is the const 100. I am not treating 100 as that concurrent-children number. Eight permits remain the run cap. One hundred slots remain the queue cap.

## 31.3 `gather` sorts replies and leaves the trace behind

`gather` is a thin wrapper. The work is `gather_holding`.

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

`lock` yields the guard. `drain(..)` removes every element and yields them. `collect` builds a `Vec<Task>`. The guard is a temporary that lives until the statement ends, so the drain finishes before the lock is released. After the semicolon the queue is empty and the mutex is free. The workers do not lock `Board`. They lock the pool inside `acquire`.

`thread::scope` is the same scoped spawn as `run_specialists`. The threads borrow `task` and `pool`. They finish before `thread::scope` returns. `join().expect("worker")` panics if a worker panics. The rows vec is spawn order until `sort_by_key`.

`_permit` holds one slot from the chapter 29 pool. `WORKER_BOUND` is 8, so a ninth task waits on the condvar. `hold.is_zero()` is true for `Duration::ZERO`, and `gather` passes that zero. The sleep runs only for a non-zero hold. `gather_holding` is how the bound test makes the overlap visible.

The tuple is `(task.task_index, task.reply.clone())`. `task_index` is `usize`, which is `Copy`. `reply.clone()` is a new `String`. `tool_trace` is not read. The parent string cannot contain `secret-tool` unless that text was also the reply. The tests use reply `ok`, `a`, `b`, or `c`, and the trace `secret-tool`.

`sort_by_key(|(index, _)| *index)` orders by the index. Spawn order is discarded as the sort key. A task scheduled first with index 2 prints after a task scheduled later with index 0. The pattern `|(index, _)|` borrows the tuple. `*index` copies the `usize` out as the key.

The map drops the index and keeps the reply. `collect::<Vec<_>>()` builds the vec `join` needs. `join("\n")` puts a newline between replies and does not add one after the last reply. Three replies become `a\nb\nc`.

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

Index 2 is queued first. The summary is `a`, then `b`, then `c`. `contains` is false for the trace. `len` is 0 because `drain` emptied the board. A second `gather` on that board would join an empty list into an empty string. The test stops at the first gather.

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

Sixteen lines. `max_seen` equals `WORKER_BOUND`, which is 8. The queue accepted 16 because 16 is below 100. The pool never ran more than 8 at once. `lines()` splits the summary on newlines. Sixteen replies of `ok` are sixteen lines. The indexes were `0..16`, so the sort does not change the text. The peak is the evidence of the bound.

`Pool::new(WORKER_BOUND)` is the chapter 29 constructor. `max_seen` starts at 0 and reaches 8 inside `acquire`. This chapter does not change `acquire`. It calls it.

Heartbeats are not in `gather` or `gather_holding`. There is no pulse, no deadline, and no restart. Those stay in chapter 36. A task that is waiting on the condvar waits until a permit drops. It does not send a heartbeat while it waits.

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

The array is three pairs. The loop schedules index 2, then 0, then 1. Each `expect("schedule")` panics on `Full`. Three is below 100, so each call is `Ok(())`. `gather` sorts and joins. The string is `a\nb\nc`. The trace is on every task and is absent from the string, for the same reason as the unit test: the worker tuple clones `reply` only.

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

The package binary is `ch31-fan-out`. No key. The replies are the literals `c`, `a`, and `b`.

## 31.5 `schedule_100` fills the queue and does not gather

The bench is `benches/fan_out.rs`. `Cargo.toml` names it `fan_out` with `harness = false`. The workflow section is `31`, label `chapter 31 fan-out`, package `ch31-fan-out`.

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

`iter_batched` takes a setup closure, a timed closure, and a batch size. `Board::new` is the setup. It is not the timed work. The timed closure schedules indexes `0..100`, which is 0 through 99, one hundred tasks. Reply text is `reply {index}`. The trace is `secret-tool` again. `expect("schedule")` is safe inside this loop because the board starts empty and the loop stops at 100. `black_box(board.len())` keeps the pushes in the sample. `len` locks and returns 100.

No `Pool` is built. No `thread::scope` runs. No `gather`. The mean is the cost of filling the queue, which is the rejection boundary the next `schedule` would hit. The mean is `4.12 µs`. It is `mean.point_estimate`. The printed Criterion `time:` line is the slope, not the mean.

`BatchSize::SmallInput` is the Criterion batch size argument on this call. The setup produces a small board, an empty deque, and the routine fills it.

The imports at the top of the bench file are `black_box`, `criterion_group`, `criterion_main`, `BatchSize`, `Criterion`, and `Board`, `Task`, `QUEUE_CAP` from `harness_core::fanout`. `Pool` is not imported. The bench cannot accidentally gather.

## 31.6 The permit `gather` takes is still chapter 29's

`gather_holding` calls `pool.acquire`. The function it calls is the one in this crate's `supervisor.rs`, copied forward from chapter 29. The bound is the same const.

```rust
pub const WORKER_BOUND: usize = 8;
```

```rust
pub fn acquire(&self) -> Permit<'_> {
    let mut state = self.state.lock().expect("state");
    while state.inflight >= self.limit {
        state = self.cv.wait(state).expect("state");
    }
    state.inflight += 1;
    state.max_seen = state.max_seen.max(state.inflight);
    Permit { pool: self }
}
```

```rust
impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let mut state = self.pool.state.lock().expect("state");
        state.inflight = state.inflight.saturating_sub(1);
        self.pool.cv.notify_one();
    }
}
```

`Permit` borrows the pool. The worker closure holds `_permit` until the closure returns, which is after the reply is cloned and after the optional sleep. `Drop` then decrements `inflight` and wakes one waiter. Sixteen tasks and a limit of 8 mean the first eight `acquire` calls increment and return, and `max_seen` becomes 8. The next `acquire` finds `inflight >= limit` and waits. When one of the eight sleeps finishes, that permit drops, `notify_one` runs, and a waiter loops, sees a free slot, and increments. The peak stays 8 because the increment happens only after the `while` sees room. The test's `pool.max_seen()` is read after `gather_holding` returns, which is after every join, which is after every `Drop`. `inflight` is back to 0. `max_seen` is still 8, because nothing in `Drop` clears the peak.

The demo's three tasks never fill those eight slots. They still go through `acquire`. `max_seen` after the demo is whatever peak three overlapping acquires reached, and the demo does not read it. The string it returns does not depend on the peak.

The rows before `sort_by_key` follow join order. The handles were collected from `tasks.iter()`, and `tasks` came from `drain`, which follows `push_back` order. The demo pushes index 2, then 0, then 1. Joins happen in that handle order. The vec before the sort is `(2, "c")`, then `(0, "a")`, then `(1, "b")`. `sort_by_key` reorders by the copied index. The vec after the sort is `(0, "a")`, `(1, "b")`, `(2, "c")`. `join("\n")` builds `a\nb\nc`. Finish order inside the pool can differ from that. The join loop waits on handle 0, then handle 1, then handle 2, so a fast index-1 worker still sits in the vec at the slot its handle occupied until the sort runs. The sort is the aggregator. Spawn order is not the summary.

`tool_trace` on those three tasks is the owned string `secret-tool`. The closure does not mention the field. After `clone` of `reply`, the `Task` is still in the `tasks` vec, trace and all, until `thread::scope` returns and `tasks` drops. The summary `String` has already been built from the replies only. `assert!(!summary.contains("secret-tool"))` is the test that the drop of `tasks` is not what the parent reads.

`schedule` and `gather` use different mutexes. `schedule` locks `Board.queue`. `acquire` locks `Pool.state`. A worker blocked in `acquire` does not hold the queue lock. The drain statement released it. That is why sixteen waits do not block a hypothetical `schedule` on the queue mutex. They block on the pool. The unit tests do not call `schedule` during `gather`. The split is still what the two locks are.

`_run_children_parallel` is the Hermes function I read for the parallel batch. This `thread::scope` is the harness batch. `_get_max_async_children` ties the background cap to `delegation.max_concurrent_children` and ignores a leftover `delegation.max_async_children` key with a one-time warning. I did not add that key to `Board`. The refusal the board implements is `FanError::Full` at 100 queued tasks. The refusal the pool implements is a wait at 8 permits. Heartbeats are not a third mechanism in this file. There is no timer in `schedule`. The only `thread::sleep` is the hold `gather_holding` receives, and `gather` passes `Duration::ZERO` so `is_zero` skips it.

The bench's loop is `0..QUEUE_CAP`. The last index that is stored is 99. The task the unit test refuses uses index `QUEUE_CAP`, which is 100, and reply `no`. That task is a different value from the last successful one. Both can exist as Rust values. Only the successful ones are in the deque when `len` is read.

Index 0 in `schedule_100` is a `Task` with `task_index` 0, reply `reply 0`, and trace `secret-tool`. Index 99 is `task_index` 99, reply `reply 99`, and the same trace. The loop variable is both the index and the source of the reply text. `format!` allocates that reply on every push. Those allocations are inside the timed closure, so they are part of `4.12 µs`. `Board::new` is outside it. A reader who moves the `format!` into the setup closure would be measuring a different sample than this file. The file formats inside the routine.

`expect("schedule")` panics with that message if `schedule` returns `Err`. Inside `0..QUEUE_CAP` on a new board the `Result` is `Ok(())` every time. The panic path is the 101st call, and the bench does not make that call. The unit test does, and it uses `assert_eq!` rather than `expect`, so `Full` is a value the test holds, not a panic. `FanError` is `Copy`, so the `assert_eq!` can compare `Err(FanError::Full)` without moving a unique error out of the call. The right-hand side is a fresh `Err`. The left-hand side is the call. They match on the variant.

`push_back` on a `VecDeque` adds at the tail. `drain(..)` yields from the front, which is the oldest task, through to the tail. Schedule order and drain order match. Sort order matches schedule order only when `task_index` was pushed in sorted order. The fill test pushes `0, 1, 2, ...` so those two orders match. The demo pushes `2, 0, 1` so they do not. `gather` always sorts. It does not have a fast path that skips the sort when the indexes happen to be ordered. The sixteen-line test pushes `0..16` and still calls `sort_by_key`. The lines are all `ok`, so the sort does not change what `lines()` counts. `summary.lines().count()` is 16 because `join("\n")` inserted 15 newlines between 16 replies, and `lines` yields 16 pieces. A trailing newline would add an empty extra line. `join` does not add one. The count equals the number of replies.

The pool those 16 tasks share is `Pool::new(WORKER_BOUND)`. `limit()` on that pool is 8. `HERMES_MAX_CONCURRENT_CHILDREN` in the copied supervisor file is still 10. `gather` does not read the 10. The wait in `acquire` compares `inflight` with `self.limit`. Sixteen `Ok` replies still come back, because waiting is not `FanError::Full`. `Full` is only `schedule`. A board with 16 tasks is not full. A board with 100 tasks is full. A pool with 8 permits taken is busy, and the next worker waits. Those three sentences are the three numbers in this chapter: 16 in the bound test, 100 in the queue test, 8 in `max_seen`.

`is_empty` is not used by `gather`. After `drain`, `len` is 0, so `is_empty` would be true. The sort test asserts `len` directly. `Default::default` builds the same board as `new`. The tests do not go through `Default`. The impl is there so a `Board` can sit in a struct that wants `Default`. The queue inside is still a `Mutex` of an empty `VecDeque`.

The parent string is the contract the critic in the next chapter can be handed. This chapter does not call `parse_verdict`. It returns the replies. A trace that never enters that string cannot be judged, because it is not in the string. That is the point of leaving `tool_trace` on the `Task` only.

`schedule` returns `Result<(), FanError>` and `gather` returns `String`. A caller who wants both the refusal and the summary handles the `Result` first, then calls `gather` on the board that still holds the tasks that were accepted. `gather` does not return `Full`. By the time it runs, the tasks it drains are the ones `schedule` stored. The 101st task never reaches `thread::scope`.

## Tests

Four tests in `tests/fan_out.rs`.

One hundred `schedule` calls succeed. The next is `FanError::Full`. The length stays 100.

Gather of indexes 2, 0, 1 equals `a\nb\nc`. The string does not contain `secret-tool`. The board length is 0.

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

The binary schedules three tasks out of order and prints the gathered replies. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `schedule_100` | 4.12 µs | fill the queue, then the next call is `Full` | `schedule` of indexes 0 through 99 on a fresh board |

`schedule_100` does not include `Board::new` in the timed closure. It does not include `gather`. The tool trace is stored on the task and never formatted into the bench's `black_box` value, which is `len`.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `31`. The id is `31/schedule_100`. The old id `29/schedule_100` is an alias in `benchmarks/descriptions.json`. The printed Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The lines are `a`, `b`, and `c`.

## Pitfalls

The queue cap and the permit cap are different numbers. `QUEUE_CAP` is 100. `WORKER_BOUND` is 8. A full board holds 100 tasks. `gather` still runs eight at a time. Filling the board does not start 100 threads at once. It starts one scoped thread per drained task, and those threads block inside `acquire` past eight.

`tool_trace` exists so a test can show the summary omitted it. A format of the whole `Task` inside the worker would put `secret-tool` in the parent string. The worker returns the index and the reply clone. That is the whole row.

Sort uses `task_index`, not finish time. A fast task with a high index is still last after `sort_by_key`. Join order before the sort is spawn order. The string the caller sees is the sorted one.

Hermes rejects a new async dispatch at the concurrency cap and does not queue it. This `schedule` queues until 100 and then returns `Full`. Both are a refusal at a cap. The cap in this file is the queue length. The cap in `acquire` is 8, and it waits instead of returning `Full`. A task already on the board is not rejected when the pool is busy. It waits for a permit.

`drain` empties the board before the threads run. A `schedule` that arrives during `gather` lands on an empty deque and is not part of this gather. The test does not do that. It schedules, then gathers, then checks length 0.

The reflection queue of 64 in `supervisor.rs` is not this queue. `enqueue_reflection` does not call `schedule`. `schedule` does not push a reflection string.

Heartbeats are not hiding in `thread::sleep`. The sleep is the test hold. `gather` passes a zero duration and skips it. Chapter 36 is the heartbeat chapter.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Bound.** `gather` takes permits from `Pool` with `WORKER_BOUND` of 8. Sixteen holders peaked at `max_seen` 8. M5 and M6 still live on `child_call` in the copied supervisor. This chapter does not add a second permit count.

**Queue.** `QUEUE_CAP` is 100. The 101st `schedule` is `FanError::Full`. The length stays 100. That refusal is the adaptation of "at capacity, reject". The capacity adapted here is the queue, not `delegation.max_concurrent_children`.

**Summary.** The parent string is replies in `task_index` order. `tool_trace` stays on the `Task` and is not cloned into the row.

**Left for later.** Heartbeats are chapter 36. Chapter 32's critic reads a finished reply, takes one permit from this same pool, and does not write the notes.

## Chapter summary

- **Queue.** 100 tasks. The next `schedule` is `FanError::Full`. The refused task is not stored.
- **Bound.** `gather` uses the supervisor pool. `WORKER_BOUND` is 8. `max_seen` peaks at 8.
- **Aggregator.** `sort_by_key` on `task_index`. The summary is the replies joined with newlines.
- **Trace.** `tool_trace` is not in the summary. The demo's trace is `secret-tool` and the printed lines are `a`, `b`, `c`.
- **Time.** `schedule_100` mean `4.12 µs`. That is `mean.point_estimate`. The Criterion `time:` line is the slope.

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

`todo!("chapter 31")` is the panic. The test wants `chapter_31_refuses_overflow(99)` to be false and `chapter_31_refuses_overflow(100)` to be true. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 32 is the critic. It takes one permit from this pool, locks the notes, and does not push. `accepted` is true only for `Verdict::Done`. A missing verdict word is `Reject`. The 30 second timeout is a constant. The function does not sleep it.
