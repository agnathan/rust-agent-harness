# Lesson 14: Cancellation trees, shared budgets and a delegating supervisor

**Goal:** Spawn child agents that you can cancel instantly, that share one budget, and that are limited in depth and concurrency.

## Concepts
- **`CancellationToken`** (from tokio-util). `child_token()` builds a tree: cancelling a parent cancels every descendant.
- **`tokio::select!`** races futures; the first one to finish wins and the others are *dropped*. Dropping a future cancels it, which is why Rust cancellation is immediate and complete.
- **Atomics.** `AtomicU32::fetch_update` gives a lock-free shared counter that can never go below zero.
- **Supervisor.** A `JoinSet` of children, a concurrency `Semaphore`, depth limits, and panic isolation: a child that panics gives a `JoinError` instead of crashing the parent.

## How Hermes does it
`delegate_task` in `tools/delegate_tool.py`:
- **Setup.** `_build_child_agent` gives the child a fresh context, a restricted toolset (the "leaf" role has no `delegate_task`, `memory`, `clarify` or `send_message`), and a credential leased from the pool.
- **Running.** Children run in threads, capped by `max_concurrent_children`. `max_spawn_depth` (default 2) limits nesting.
- **Budget.** Children share `IterationBudget` (`agent/iteration_budget.py`).
- **Results.** Only a summary comes back (`_build_result_entry`).
- **Cancellation.** It is cooperative (`_signal_child_stop`) plus timeouts and heartbeats, because Python can't kill a thread. Your `select!` can actually cancel a child.

## Tasks
1. `cancellable_work` with `select!`.
2. `Budget::try_consume`.
   - Hint: `fetch_update(SeqCst, SeqCst, |r| r.checked_sub(1)).is_ok()`.
3. `Supervisor::child`: depth check, `self.token.child_token()`, and the same `Arc<Budget>`.
4. `delegate`. Build the outcomes step by step:
   - Wrap `work` in an `Arc`. For each task, `set.spawn(async move {...})`.
   - Inside each task: acquire a permit (`sem.acquire_owned()`), then consume budget.
   - Run the work in an inner `tokio::spawn` so a panic is caught as a `JoinError`.
   - `select!` the inner task against `token.cancelled()`.
   - Return `(index, outcome)` from each task, then sort by index at the end.

## Milestone
`cargo test --test l14_supervisor` is green. `delegate_cancel` finishes in well under a second, even though the children would sleep 10 s.

## Exercises
- Add per-child timeouts (Hermes's `child_timeout_seconds`).
- Make a child's work an actual agent loop with its own `ScriptedModel`, and return the last assistant text as its summary.
- Implement roles: `Leaf` vs `Orchestrator` toolsets.
