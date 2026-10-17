# Chapter 27: The Supervisor

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 26 runs stages one after another. It does not run them at the same time, and it does not stop a child from calling the tools the parent uses. This chapter is the supervisor. Specialists are jobs. At most eight hold a permit at once. They share one `Arc` context. A child at depth 1 cannot call `delegate_task`, `clarify`, `memory`, `send_message`, or `cronjob_manage`. A worktree is a marker you opt into. This crate does not create one. A reflection note from the team run is a queue entry.

The step crate is `steps/ch23-supervisor`. The book number is 27. The package is `ch23-supervisor`.

| Section | Step | What it adds |
|---|---|---|
| 27 | `steps/ch23-supervisor` | eight specialists, one `Arc`, blocked child tools, depth 1 |

> **How Hermes does it**
>
> I read `tools/delegate_tool.py` (the module doc), `tools/delegate_tool_toolsets.py` (`DELEGATE_BLOCKED_TOOLS`), `tools/delegate_tool_config.py` (`MAX_DEPTH`, `_DEFAULT_MAX_CONCURRENT_CHILDREN`, `_get_max_concurrent_children`, `_get_worktree_isolation`), `tools/delegate_tool_dispatch.py` (`_run_children_parallel`), `tools/subagent_worktree.py` (the module doc), and `hermes_cli/config_defaults.py` (`delegation.max_concurrent_children`) on Hermes `main` at `8dcf6d5b`.
>
> The module doc says the parent only sees the delegation call and the summary, never the child's intermediate tool calls. `DELEGATE_BLOCKED_TOOLS` is `delegate_task`, `clarify`, `memory`, `send_message`, and `cronjob_manage`. `MAX_DEPTH` is 1: parent at 0, child at 1. `_DEFAULT_MAX_CONCURRENT_CHILDREN` is 10, and the config default is the same number. `_run_children_parallel` runs the batch together and sorts results by `task_index`. `subagent_worktree.py` is opt-in git worktree isolation. `_get_worktree_isolation` reads `delegation.worktree_isolation` and defaults to false. The key is not in the defaults file I opened; the `.get(..., False)` is the default.
>
> **Adopted.** The five blocked names, depth 1, a parallel batch, a shared context, and a worktree that stays off until asked.
>
> **Adapted.** The pool limit is 8. That is the harness measurement. `HERMES_MAX_CONCURRENT_CHILDREN` is 10 so the Hermes default stays visible, and the pool does not use it. `worktree(true)` returns the string `opt-in`. It does not call git. Specialists are scripted replies, which is the live-model deferral from chapter 26. The parent sees the reply strings, not a tool trace, because the jobs have no tool trace.
>
> **Deferred.** Credential leasing, the heartbeat, and a real git worktree. Chapter 29 schedules a hundred jobs onto a queue of 100 and gathers them in `task_index` order. Chapter 30 takes one permit from this pool.

[personal note: a team of specialists you ran together, and the tool you did not want a child to have]

## What this chapter decides

`WORKER_BOUND` is 8. `Pool::new` stores that limit, a condvar, and one `Arc<Context>`. `acquire` waits while `inflight` is at the limit, then increments it and updates `max_seen`. Dropping the permit decrements and notifies one waiter.

`run_specialists` uses `thread::scope`. Each job acquires a permit, optionally sleeps, clones the context `Arc`, and returns its reply. Join order is spawn order.

`child_call` at depth 1 refuses every name in `BLOCKED_TOOLS`. `read_file` at depth 1 is allowed. `delegate_task` at depth 0 is the parent's call and is allowed. Depth 2 refuses every tool.

`worktree(false)` is `None`. `worktree(true)` is `Some("opt-in")`.

`enqueue_reflection` pushes a string onto a queue of 64. It does not acquire a permit and it does not start a reviewer.

`demo` runs two specialists and returns `three steps | the draft`. No key.

## Rust: a permit is a guard

`Permit` holds a reference to the pool. `Drop` releases the slot even if the specialist panics. `thread::scope` joins every thread before `run_specialists` returns, so a permit cannot outlive the pool borrow.

The condvar wait is inside the mutex. A ninth specialist blocks in `acquire` until one of the eight drops its permit. `max_seen` is updated under that same mutex, so the test can read it after the join.

## Tests

Five tests. Eight jobs that each sleep 40 ms share one context pointer. `max_seen` is 8. The elapsed time stays under four sleeps, so they overlapped. The Hermes constant is 10 and the pool limit is not 10. Nine jobs still peak at 8. Each blocked name at depth 1 is `Blocked`. `read_file` at depth 1 is allowed. `delegate_task` at depth 0 is allowed. Depth 2 is refused. The worktree is `None` until opted in. Enqueueing a reflection leaves `max_seen` at 0. The demo string is `three steps | the draft`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch23-supervisor
```

```text
Starting 5 tests across 3 binaries
     Summary [   0.045s] 5 tests run: 5 passed, 0 skipped
```

The eight-specialist test is the line that slept. Nextest recorded it at 0.044s. Each specialist slept 40 ms. Eight of those in series would be 320 ms.

```bash
cargo run -p ch23-supervisor
```

```text
three steps | the draft
```

The binary runs two scripted specialists and enqueues a reflection note. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `schedule_eight` | 148.1 µs | eight specialists, one context | run eight jobs that do not sleep |
| `enqueue_reflection` | 20.81 ns | off the hot path | record a reflection note |

`schedule_eight` builds the pool and eight jobs before the bench. The jobs' `hold` is `None`. The timed closure is `b.iter`. Inside: `run_specialists` with the pool and the jobs passed through `black_box`. That starts eight threads, takes eight permits, clones the context `Arc`, and joins. No model is called. The 40 ms sleep from the test is not in this sample.

`enqueue_reflection` times `enqueue_reflection` of the string `team run finished`. No specialist runs. The queue is capped at 64.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38986915984](https://github.com/agnathan/rust-agent-harness/actions/runs/38986915984), section `27`.

### Milestone demo

The `cargo run` above. The two replies are `three steps` and `the draft`.

## Pitfalls

The pool limit is 8. Hermes will run 10. Copying the Hermes default into `Pool::new` would miss the budget this chapter is here to hold.

`delegate_task` is legal at depth 0 because that is the parent. The same name at depth 1 is `Blocked`. A helper that forgets to pass the depth will allow a child to delegate.

`worktree(true)` does not isolate files. Two opted-in specialists still share the process and the context `Arc`. The marker is the decision. The git worktree is not built.

`schedule_eight` includes thread start. It is not the overlap test. The overlap test is the 40 ms sleep, and nextest finished that test in 0.044s.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**M5.** `BLOCKED_TOOLS` is `DELEGATE_BLOCKED_TOOLS`: `delegate_task`, `clarify`, `memory`, `send_message`, `cronjob_manage`. A child at depth 1 cannot call them.

**M6.** `MAX_DEPTH` is 1. Depth 2 is refused even for `read_file`.

**Bound.** Hermes caps concurrent children at 10 (`delegation.max_concurrent_children`, `_DEFAULT_MAX_CONCURRENT_CHILDREN`). This pool caps at 8. The constant `HERMES_MAX_CONCURRENT_CHILDREN` documents the 10. It is not the permit count.

**D6.** `worktree` follows `_get_worktree_isolation`: off unless opted in. Opt-in does not create a worktree.

**Reflection.** `enqueue_reflection` does not run chapter 24's reviewer. The team run records the note. Measured **20.81 ns**.

**Left for later.** Chapter 29 is the queue of 100 and the aggregator. Chapter 30's critic takes one permit from this pool. E1 and E2 are still absent.

## Chapter summary

- **Bound.** Eight permits. A ninth waits. Hermes documents 10.
- **Context.** One `Arc` per pool. Specialists clone the pointer.
- **Child.** The five blocked tools at depth 1. Depth stops at 1.
- **Worktree.** Off by default. Opt-in is a marker, not a git command.
- **Time.** Scheduling eight empty specialists measured **148.1 µs**. The 40 ms overlap test finished in **0.044 s**.

[personal note: the specialist that only worked because it could not call the parent's tools]

## What's next

Chapter 28 hands ownership to another agent. The transcript is filtered first. There is a limit on how many handoffs a turn may make.
