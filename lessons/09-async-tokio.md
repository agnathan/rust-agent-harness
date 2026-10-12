# Lesson 09: async/await and tokio

**Goal:** Learn why async exists (waiting on lots of network I/O at once) and how to use tokio.

## Concepts
- **Futures are lazy.** An `async fn` returns a *future* that does nothing until something `.await`s it.
- **tokio** is the runtime that drives futures. `#[tokio::main]` starts it in a program, and `#[tokio::test]` starts it in a test.
- **Concurrency without threads.** `futures::future::join_all` runs many futures concurrently on one task. They overlap while each one is waiting.
- **`tokio::spawn`** creates an independent *task*, a lightweight thread that costs kilobytes, not megabytes. **`JoinSet`** manages a group of tasks and collects their results.
- **Timeouts.** `tokio::time::timeout(dur, fut)` gives up on a future that takes too long.
- **Never block inside async.** Don't call `std::thread::sleep` or do heavy CPU work directly; use `tokio::time::sleep` or `spawn_blocking`.

## How Hermes does it
- Hermes's turn loop is *synchronous*. Each running agent occupies an OS thread:
  - The gateway hands turns to threads with `run_in_executor` (`gateway/run_turn.py`).
  - Child agents run in `DaemonThreadPoolExecutor` (`tools/delegate_tool_child_run.py`).
  - Tool batches use up to 8 threads (`agent/tool_executor.py::_MAX_TOOL_WORKERS`).
- Python's GIL serializes CPU work across those threads.
- In Rust, thousands of agents can be tokio tasks, which is the main performance win of a rewrite.

## Tasks
1. `slow_tool`, using `sleep(Duration::from_millis(ms)).await`.
2. `run_sequential` (a loop with `.await`) and `run_concurrent` (`join_all`). Compare how long the tests take.
3. `with_timeout`.
   - Hint: `.map_err(|_| "timeout")`.
4. `spawn_and_sum` with a `JoinSet`.
   - Hint: `while let Some(r) = set.join_next().await`.

## Milestone
`cargo test --test l09_async` is green. The concurrent version takes about 100 ms; the sequential one about 300 ms.

## Exercises
- Make an async `Model` trait (`#[async_trait]`) and port lesson 07's loop to it. Switch reqwest to its async client.
- Spawn 10,000 tasks that each sleep 1 s. How long does it take, and how much memory does it use?
