# Chapter 29: The Supervisor

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 28 can run four stages in order on one context. A lot of work is not a sequence. A manager names specialists, they run, and only a summary comes back. That is the default team in this book. The crate is `steps/ch29-supervisor`, a copy of chapter 28 plus `supervisor`. The package is `ch29-supervisor`. Specialists are tools. They use the same `Tool` shape chapter 3 defined, so the loop does not learn a second way to call an agent.

The bound is 8. Hermes caps concurrent children higher, at 10, and that default stays in the comments as Hermes's number. The harness measurement, and the pool, is eight, the same bound the dispatcher has used since chapter 3. `acquire` waits for a permit, and `Drop` wakes one waiter, so a finished specialist releases the slot even when the caller forgets. `thread::scope` runs the jobs and joins them, which keeps the borrows honest: a specialist cannot outlive the scope that lent it the context. `child_call` is a `Result`. The blocked tools are the ones a child must not have: `delegate_task`, `clarify`, `memory`, `send_message`, and cron. Depth is 1. A child does not spawn another supervisor. The worktree is a marker, opt-in, not a git operation this chapter performs. The reflection note is a string you enqueue. It does not run chapter 26's reviewer on the team turn.

Eight specialists share one pool and one `Arc` context. That is the shape chapter 31 will push to a queue of 100, and the shape chapter 32's critic borrows one permit from. Heartbeats are not here. Chapter 31 does not start them either. Chapter 36 implements `is_stale`. Putting the liveness check in the supervisor chapter would hide a clock inside the scheduler. The scheduler's job is the pool, the depth, and the tool slice.

The learning loop stays off this path on purpose. A team run can leave a note that reflection should look at the summaries. The reviewer still has only skill and memory tools, and it still runs after the turn. A specialist that could write memory would race the others to define the lesson. The parent decides what to keep. That is the same rule chapter 48 will use for a team library: workers propose, one writer applies.

Rust, at the pool: a semaphore permit is a guard, and `Drop` is the release. `thread::scope` is the standard-library way to borrow data across threads without `'static`. The blocked-tool check is a match on the name before the child runs. Depth is an integer you refuse to increment past 1.

`cargo run --locked -p ch29-supervisor --bin ch29-supervisor` runs the scripted specialists. The exercise is `my-harness/tests/ch29.rs`: eight specialists share one pool, and a reflection note does not run the reviewer.

**In this chapter**

- 29.1 The bound is 8, and the Hermes default stays 10
- 29.2 `acquire` waits, and `Drop` wakes one thread
- 29.3 `thread::scope` runs the jobs and joins them
- 29.4 `child_call` is a `Result`
- 29.5 The worktree is a marker, and the reflection is a string

| Section | Step | What it adds |
|---|---|---|
| 29.1 | `steps/ch29-supervisor` | the bound, the blocked names, `Pool`, `Job` |
| 29.2 | `steps/ch29-supervisor` | `acquire`, `Permit`, the condvar |
| 29.3 | `steps/ch29-supervisor` | `run_specialists` and `thread::scope` |
| 29.4 | `steps/ch29-supervisor` | `child_call` at depth 0, 1, and 2 |
| 29.5 | `steps/ch29-supervisor` | `worktree`, `enqueue_reflection`, the demo |

> **How Hermes does it**
>
> I read `tools/delegate_tool_toolsets.py` (`DELEGATE_BLOCKED_TOOLS`), `tools/delegate_tool_config.py` (`MAX_DEPTH`, `_DEFAULT_MAX_CONCURRENT_CHILDREN`, `_get_max_concurrent_children`, `_get_worktree_isolation`), `tools/delegate_tool_dispatch.py` (`_run_children_parallel`), `tools/delegate_tool.py` (the module doc for delegation), `tools/subagent_worktree.py`, and `hermes_cli/config_defaults.py` (`delegation.max_concurrent_children`) on Hermes `main` at `ab3fe9dffb0a`.
>
> `DELEGATE_BLOCKED_TOOLS` is `delegate_task`, `clarify`, `memory`, `send_message`, `cronjob_manage`. `MAX_DEPTH` is 1. `_DEFAULT_MAX_CONCURRENT_CHILDREN` is 10. `_get_max_concurrent_children` reads that cap. `_get_worktree_isolation` defaults to false. `_run_children_parallel` is the parallel child run. `tools/subagent_worktree.py` is the worktree module. `delegation.max_concurrent_children` is the config key.
>
> **Adopted.** The blocked list. Depth 1. The opt-in worktree flag. Parallel children on a shared context.
>
> **Adapted.** `WORKER_BOUND` is 8, not 10. `HERMES_MAX_CONCURRENT_CHILDREN` keeps the 10 visible. The pool uses 8. `worktree` returns `None` unless opt-in, and it does not create a git worktree. Specialists return scripted replies.
>
> **Deferred.** Heartbeats, to chapter 36. A real git worktree. The queue of 100 and `task_index` order, to chapter 31. Live models, still deferred from chapter 28.

[personal note: a team of specialists you ran together, and the tool you did not want a child to have]

## What this chapter decides

`Pool::new(WORKER_BOUND)` stores limit 8, a mutex of inflight and `max_seen`, a `Condvar`, and one `Arc<Context>`. `acquire` waits while `inflight` is at the limit, then increments it and updates `max_seen`. Dropping the `Permit` decrements and calls `notify_one`.

`run_specialists` uses `thread::scope`. Each job acquires a permit, optionally sleeps, clones the context `Arc`, and returns its reply. Join order is spawn order.

`child_call` denies a blocked tool at depth greater than or equal to 1. It denies any tool at depth greater than 1. `read_file` at depth 1 is `Ok`. `delegate_task` at depth 0 is `Ok`.

`worktree(false)` is `None`. `worktree(true)` is `Some("opt-in")`.

`enqueue_reflection` pushes a string onto a queue of 64. It does not call `acquire`. It does not call `spawn_reviewer`. After that call alone, `max_seen` stays 0.

`demo` runs two specialists and returns `three steps | the draft`. No key.

## 29.1 The bound is 8, and the Hermes default stays 10

The module imports what the pool needs.

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
```

`VecDeque` is the reflection queue. `Arc` is the shared context, the same pointer kind as chapter 28. `Condvar` is the wait that blocks a ninth job. `Mutex` guards the counters. `thread` is `thread::scope`, `thread::sleep`, and the join. `Duration` is the optional hold a test uses to prove the jobs overlapped.

The constants are the policy.

```rust
pub const WORKER_BOUND: usize = 8;
/// Hermes `delegation.max_concurrent_children`. The pool uses `WORKER_BOUND`.
pub const HERMES_MAX_CONCURRENT_CHILDREN: usize = 10;
pub const MAX_DEPTH: u8 = 1;
pub const BLOCKED_TOOLS: &[&str] = &[
    "delegate_task",
    "clarify",
    "memory",
    "send_message",
    "cronjob_manage",
];
const QUEUE_CAP: usize = 64;
```

`WORKER_BOUND` is 8. That is the permit count. `HERMES_MAX_CONCURRENT_CHILDREN` is 10, which is `_DEFAULT_MAX_CONCURRENT_CHILDREN`. The doc comment on the 10 names the config key `delegation.max_concurrent_children`. The pool does not read the 10. The test asserts the constant equals 10 and asserts the pool's limit is not that constant.

`MAX_DEPTH` is a `u8` of 1. Depth 0 is the parent. Depth 1 is the child. `BLOCKED_TOOLS` is the five names from `DELEGATE_BLOCKED_TOOLS`, in that order. `QUEUE_CAP` is private and is 64. It caps the reflection deque. It is not chapter 31's public `QUEUE_CAP` of 100. Two different queues, two different numbers.

`TeamError` is the `Err` side of `child_call`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamError {
    Blocked(String),
}
```

The string is the tool name that was refused. Tests match `TeamError::Blocked(name)` against the name they passed in.

`Context` in this module is smaller than chapter 28's. The pipeline's `Context` had messages and notes. The supervisor's `Context` is the notes mutex alone. `pipeline.rs` is still in the crate, with its own `Context`. `supervisor::Context` is a different type.

```rust
pub struct Context {
    pub notes: Mutex<Vec<String>>,
}
```

`Job` is a specialist.

```rust
pub struct Job {
    pub name: &'static str,
    pub reply: String,
    pub hold: Option<Duration>,
}
```

`name` is a static string. `run_specialists` does not read `name`. The demo sets it to `outline` and `draft` so the job is labeled. The reply is what the thread returns. `hold` is `Some(duration)` when a test wants the thread to sleep inside the permit, and `None` when the bench wants no sleep.

`State` is private. Callers use methods.

```rust
struct State {
    inflight: usize,
    max_seen: usize,
    reflection: VecDeque<String>,
}
```

`inflight` is how many permits are out. `max_seen` is the highest `inflight` has been. `reflection` is the deque of note strings.

`Pool` owns the limit, the state, the condvar, and the context.

```rust
pub struct Pool {
    limit: usize,
    state: Mutex<State>,
    cv: Condvar,
    context: Arc<Context>,
}
```

`Permit` borrows the pool.

```rust
pub struct Permit<'a> {
    pool: &'a Pool,
}
```

The lifetime `'a` is the borrow of the `Pool`. A permit cannot outlive the pool reference it was acquired from. `acquire` returns `Permit<'_>`, which ties that lifetime to `&self`.

`Pool::new` builds all of that.

```rust
pub fn new(limit: usize) -> Self {
    Self {
        limit: limit.max(1),
        state: Mutex::new(State {
            inflight: 0,
            max_seen: 0,
            reflection: VecDeque::new(),
        }),
        cv: Condvar::new(),
        context: Arc::new(Context {
            notes: Mutex::new(Vec::new()),
        }),
    }
}
```

`limit.max(1)` means a caller who passes 0 still gets a limit of 1. The tests pass `WORKER_BOUND`, so the stored limit is 8. `inflight` and `max_seen` start at 0. The deque is empty. `Condvar::new` is a fresh condvar with no waiters. `Arc::new` is one context for the life of the pool.

The small readers lock or clone and return.

```rust
pub fn limit(&self) -> usize {
    self.limit
}

pub fn context(&self) -> Arc<Context> {
    Arc::clone(&self.context)
}

pub fn max_seen(&self) -> usize {
    self.state.lock().expect("state").max_seen
}

pub fn reflection_len(&self) -> usize {
    self.state.lock().expect("state").reflection.len()
}
```

`limit` reads the field. No lock, because the field is not behind the mutex and nothing writes it after `new`. `context` clones the `Arc`. That increments the strong count and returns another pointer to the same `Context`. `max_seen` and `reflection_len` lock, copy a `usize` out, and drop the guard at the end of the statement. A `usize` is `Copy`, so the caller does not hold the lock. `expect("state")` panics if that mutex is poisoned.

## 29.2 `acquire` waits, and `Drop` wakes one thread

The permit is the bound.

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

`lock` takes the mutex. The `while` is the wait. If `inflight` is already at `limit`, this caller cannot increment yet. `Condvar::wait` takes the guard, unlocks the mutex, and blocks the thread. When it wakes, it locks again and returns the guard. The assignment `state = ...` is that new guard. The `while` checks the count again. A wake can be spurious, and another thread can take the slot between the wake and this check. The loop goes back to `wait` if the count is still at the limit.

Once `inflight` is below the limit, the code adds one and sets `max_seen` to the greater of the old peak and the new `inflight`. Eight concurrent acquires on a limit of 8 leave `max_seen` at 8. A ninth acquire stays inside `wait` until a `Drop` runs. That ninth job does not push `max_seen` to 9.

`Permit { pool: self }` stores the borrow. The guard in `state` drops at the end of `acquire`, which unlocks the mutex before the caller does its work. The permit is the token. The lock is not held during the specialist's reply.

> **Rust sidebar: `Condvar`**
>
> - A condition variable waits on a mutex. `wait` consumes the guard and returns a new one. You pass the guard in because the unlock and the sleep have to be one step. If they were separate, a `notify` could land in the gap and be lost.
> - `notify_one` wakes one waiter. `notify_all` would wake every waiter. This `Drop` uses `notify_one`, because one released permit is one free slot.
> - The waiter rechecks `inflight` in the `while`. That recheck is the correctness of the bound. The notify is the hint.

`Drop` gives the slot back.

```rust
impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let mut state = self.pool.state.lock().expect("state");
        state.inflight = state.inflight.saturating_sub(1);
        self.pool.cv.notify_one();
    }
}
```

`saturating_sub` subtracts one and stops at zero. A permit is only dropped after `acquire` incremented, so the count should be at least one. The saturating form is what the source writes. Then `notify_one` wakes a thread parked in `wait`, if any. The guard drops at the end of `drop`, so the woken thread can lock.

The `'_` in `impl Drop for Permit<'_>` is the anonymous lifetime. `Drop` does not name it because the method only needs `&mut self`.

This is the M6 bound in this crate. Hermes documents 10 concurrent children. This pool hands out 8 permits. The constant 10 remains in the file so the test can show the two numbers side by side.

## 29.3 `thread::scope` runs the jobs and joins them

`run_specialists` is the parallel batch.

```rust
pub fn run_specialists(pool: &Pool, jobs: &[Job]) -> Vec<String> {
    thread::scope(|scope| {
        let handles: Vec<_> = jobs
            .iter()
            .map(|job| {
                scope.spawn(|| {
                    let _permit = pool.acquire();
                    if let Some(hold) = job.hold {
                        thread::sleep(hold);
                    }
                    let _shared = pool.context();
                    job.reply.clone()
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("specialist"))
            .collect()
    })
}
```

> **Rust sidebar: `thread::scope`**
>
> - `thread::scope` takes a closure and passes it a scope. `scope.spawn` starts a thread that may borrow local data. The borrow is safe because every spawned thread is finished before `thread::scope` returns. The handles are joined inside the closure. A thread that is still running at the end of the scope is joined by the scope itself.
> - The closure borrows `pool` and `job`. `pool` is `&Pool`. `job` is `&Job` from the slice. Neither has to be `'static`. An ordinary `thread::spawn` would require the closure to own its data or use an `Arc`, because that thread might outlive the caller. Scoped threads do not outlive `thread::scope`.
> - `join` returns `Result`. `Ok` is the closure's return value, here a `String`. `Err` means the thread panicked. `expect("specialist")` panics the caller with that message. The replies vec is spawn order, because the handles were collected in spawn order and joined in that same order. Finish order can differ. The vec does not.

Inside the thread, `_permit` is the acquire. The underscore name means the binding exists so `Drop` runs at the end of the closure, not so later lines read the permit. `job.hold` is copied out of the `Option` because `Duration` is `Copy`. `thread::sleep` blocks that worker for the hold. The other workers keep their own permits. Eight sleeps of 40 ms overlap. Nine jobs on a limit of 8 cannot all sleep at once. The ninth waits in `acquire` until one `Drop` notifies.

`pool.context()` clones the `Arc` into `_shared`. The clone is the touch of the shared context. The reply does not read the notes. The `Arc` drops at the end of the closure, then the permit drops, in reverse order of the bindings: `_shared` first, then `_permit`. The context's strong count goes down before the slot is released. The pool's own `Arc` stays.

`job.reply.clone()` is the thread's return value. The parent's vec is those strings.

The overlap test is the full function in `tests/supervisor.rs`. The helper above it builds `n` jobs.

```rust
fn jobs(n: usize, hold: Option<Duration>) -> Vec<Job> {
    (0..n)
        .map(|i| Job {
            name: "specialist",
            reply: format!("reply {i}"),
            hold,
        })
        .collect()
}

#[test]
fn eight_specialists_share_the_bound_and_the_context() {
    let pool = Pool::new(WORKER_BOUND);
    let context = pool.context();
    let hold = Duration::from_millis(40);
    let started = Instant::now();
    let replies = run_specialists(&pool, &jobs(WORKER_BOUND, Some(hold)));
    let elapsed = started.elapsed();
    assert_eq!(replies.len(), WORKER_BOUND);
    assert_eq!(pool.max_seen(), WORKER_BOUND);
    assert!(std::sync::Arc::ptr_eq(&context, &pool.context()));
    // Eight serial sleeps would be hold * 8. macOS CI measured about 186 ms,
    // which is still under seven holds and above a fully parallel 40 ms.
    assert!(elapsed < hold * 7, "elapsed {elapsed:?}");
    assert_eq!(HERMES_MAX_CONCURRENT_CHILDREN, 10);
    assert_ne!(pool.limit(), HERMES_MAX_CONCURRENT_CHILDREN);
}
```

`jobs` uses the range `0..n`. Each reply is `reply {i}`. The same `hold` is stored in every job, which is legal because `Option<Duration>` is `Copy` when `Duration` is `Copy`. `pool.context()` before the run and `pool.context()` after the run are two `Arc::clone` calls. `ptr_eq` asks whether the pool still holds the allocation it was born with. `run_specialists` only clones that `Arc` inside the worker and drops the clone. It does not replace `Pool.context`.

`Instant::now` and `elapsed` wrap the call. The clock includes thread start, the eight sleeps, and the join. It is not a Criterion mean. The Criterion mean for eight jobs with `hold: None` is `149.22 µs`, from `schedule_eight`. This test's job is the overlap assert, the peak of 8, and the side-by-side of 8 and 10.

The assertion is `elapsed < hold * 7`. Seven holds of 40 ms is 280 ms. Eight serial sleeps would be `hold * 8`, which is 320 ms. The comment in the test says macOS CI measured about 186 ms, still under seven holds and above a fully parallel 40 ms. The `assert!` is the upper bound. The comment is the comment. `replies.len()` is 8. `pool.max_seen()` is 8. `Arc::ptr_eq` on a context cloned before the run and `pool.context()` after the run is true. `HERMES_MAX_CONCURRENT_CHILDREN` equals 10. `pool.limit()` is not 10.

The ninth-job test uses `WORKER_BOUND + 1` jobs and a hold of 20 ms. `replies.len()` is 9. `pool.max_seen()` is 8. All nine strings come back. The peak stays at the limit.

The bench `schedule_eight` uses the same eight jobs with `hold: None`. No sleep. The mean is `149.22 µs`.

## 29.4 `child_call` is a `Result`

The tool check is two tests on the depth.

```rust
pub fn child_call(tool: &str, depth: u8) -> Result<(), TeamError> {
    if depth >= MAX_DEPTH && BLOCKED_TOOLS.contains(&tool) {
        return Err(TeamError::Blocked(tool.to_string()));
    }
    if depth > MAX_DEPTH {
        return Err(TeamError::Blocked(tool.to_string()));
    }
    Ok(())
}
```

> **Rust sidebar: `Result`**
>
> - `Result<(), TeamError>` has two sides. `Ok(())` is success with no payload. The unit value `()` means the call is allowed. `Err(TeamError::Blocked(...))` is the refusal. The string inside is an owned copy of the tool name, from `to_string()`, so the `Err` does not borrow the `&str` argument.
> - `return Err(...)` leaves the function immediately. The second `if` does not run when the first one matches. A caller uses `match` or `is_ok` or the `?` operator. The tests use `matches!`.
> - This `Result` is not `rusqlite::Error` and it is not a lock poison. Lock poison in this file still panics through `expect`. The tool policy is the `Result`.

`MAX_DEPTH` is 1. Walk the three cases the test uses.

Depth 1 and a name in `BLOCKED_TOOLS`: `depth >= 1` is true, `contains` is true, the function returns `Blocked` with that name. That is every entry: `delegate_task`, `clarify`, `memory`, `send_message`, `cronjob_manage`. This is M5. A child does not delegate, clarify, write memory, send a message, or manage a cron job.

Depth 1 and `read_file`: `read_file` is not in the slice, so the first `if` is false. `1 > 1` is false, so the second `if` is false. The result is `Ok(())`. A child may call a tool that is not on the list, at depth 1.

Depth 0 and `delegate_task`: `0 >= 1` is false, so the blocked list is not consulted. `0 > 1` is false. The result is `Ok(())`. The parent is allowed to call `delegate_task`. The same name at depth 1 is `Blocked`. The depth argument is the difference.

Depth 2 and `read_file`: the first `if` is false because `read_file` is not blocked. The second `if` is true because `2 > 1`. The result is `Blocked("read_file")`. Any tool at depth greater than 1 is denied, including a tool the child at depth 1 could call. The test writes `MAX_DEPTH + 1`, which is 2.

`BLOCKED_TOOLS.contains(&tool)` passes `&&str` because `tool` is already `&str` and `contains` wants a reference to the slice's item. The slice items are `&str`.

The test loops `for tool in BLOCKED_TOOLS` and expects `Err(TeamError::Blocked(name))` with `name == *tool`. Then it checks `read_file` at `MAX_DEPTH`, `delegate_task` at 0, and `read_file` at `MAX_DEPTH + 1`.

## 29.5 The worktree is a marker, and the reflection is a string

`worktree` is the opt-in flag.

```rust
/// `delegation.worktree_isolation` defaults to false. Opt-in returns a
/// marker. This function does not create a git worktree.
pub fn worktree(opt_in: bool) -> Option<&'static str> {
    if opt_in {
        Some("opt-in")
    } else {
        None
    }
}
```

`_get_worktree_isolation` defaults to false. `worktree(false)` returns `None`. `worktree(true)` returns `Some("opt-in")`, a static string. The function does not call git. `tools/subagent_worktree.py` exists on the Hermes side. This harness returns the marker and stops. A real git worktree is deferred.

The guardrails doc lists D6 as subagent worktree isolation, opt-in through `delegation.worktree_isolation`. The flag in this crate is the `bool`. The isolation the doc describes is not built here.

`enqueue_reflection` is the team-run note.

```rust
pub fn enqueue_reflection(&self, note: &str) {
    let mut state = self.state.lock().expect("state");
    if state.reflection.len() == QUEUE_CAP {
        state.reflection.pop_front();
    }
    state.reflection.push_back(note.to_string());
}
```

The lock is the same mutex as `inflight`. The function does not read `inflight` and does not write `max_seen`. If the deque length is already 64, `pop_front` removes the oldest string. `push_back` appends the new owned string. A call on an empty deque makes the length 1. The 65th call drops the oldest and the length stays 64.

`VecDeque` is a double-ended queue. `pop_front` takes from the head. `push_back` adds at the tail. The kept notes are the newest 64.

The test `the_worktree_is_off_unless_asked_and_reflection_is_a_queue` calls `enqueue_reflection("team run finished")` on a fresh pool and expects `reflection_len()` of 1 and `max_seen()` of 0. No specialist ran, so the peak stays at the initial zero. That is the sentence this chapter has to keep: the reflection push does not look like a worker.

Chapter 26's reviewer is still in this crate, in `reflection.rs`, copied forward. `spawn_reviewer` is the function a review fork uses.

```rust
pub fn spawn_reviewer(review_memory: bool) -> Fork {
    let tools = if review_memory {
        MEMORY_TOOLS
    } else {
        SKILL_TOOLS
    };
    Fork {
        tools: tools.to_vec(),
        memory_nudge_interval: 0,
        skill_nudge_interval: 0,
        review_memory,
        cancel: Arc::new(AtomicBool::new(false)),
    }
}
```

`MEMORY_TOOLS` is `memory`, `skill_view`, `skill_manage`, `read_file`, `search_files`. `SKILL_TOOLS` is that list without `memory`. `Loop::finalize` calls `spawn_reviewer` when a nudge is due. `enqueue_reflection` does not call `finalize` and does not call `spawn_reviewer`. I read the body. The statements are the lock, the optional `pop_front`, and the `push_back`. The milestone `chapter_29_reflection_starts_reviewer` is false for that reason. A string on the deque is not a `Fork`.

The bench `enqueue_reflection` times that push of `"team run finished"`. The mean is `18.4 ns`. A long Criterion sample will hit the cap of 64 and start dropping the front. The timed function is still the one above. No reviewer is constructed inside it.

`demo` joins two replies and then enqueues.

```rust
pub fn demo() -> String {
    let pool = Pool::new(WORKER_BOUND);
    let jobs = [
        Job {
            name: "outline",
            reply: "three steps".into(),
            hold: None,
        },
        Job {
            name: "draft",
            reply: "the draft".into(),
            hold: None,
        },
    ];
    let replies = run_specialists(&pool, &jobs);
    pool.enqueue_reflection("team run finished");
    format!("{} | {}", replies[0], replies[1])
}
```

`Pool::new(8)` is the bound. Two jobs, no hold. `run_specialists` returns the replies in spawn order, so index 0 is `three steps` and index 1 is `the draft`. The format string puts ` | ` between them. The reflection string is pushed and is not part of the return value. The test expects `three steps | the draft`.

The binary is `src/bin/chapter_demo.rs`.

```rust
use harness_core::supervisor::demo;

fn main() {
    println!("{}", demo());
}
```

The package binary is `ch29-supervisor`.

The bench file wires both samples.

```rust
fn schedule_eight(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    let jobs: Vec<Job> = (0..WORKER_BOUND)
        .map(|i| Job {
            name: "specialist",
            reply: format!("reply {i}"),
            hold: None,
        })
        .collect();
    c.bench_function("schedule_eight", |b| {
        b.iter(|| {
            let replies = run_specialists(black_box(&pool), black_box(&jobs));
            black_box(replies);
        });
    });
}

fn enqueue_reflection(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    c.bench_function("enqueue_reflection", |b| {
        b.iter(|| {
            pool.enqueue_reflection(black_box("team run finished"));
        });
    });
}

criterion_group!(benches, schedule_eight, enqueue_reflection);
criterion_main!(benches);
```

`schedule_eight` builds the pool and the eight jobs before `b.iter`. Replies are `reply 0` through `reply 7`. `hold` is `None`. The timed closure starts eight scoped threads, takes eight permits, clones the context `Arc`, and joins. `black_box` wraps the pool, the jobs, and the replies. No model is called. The 40 ms sleep from the overlap test is not in this sample.

`enqueue_reflection` builds a pool and times the push. No `run_specialists`. No `spawn_reviewer`.

`Cargo.toml` names the bench `supervisor` with `harness = false`. The workflow section is `29`, label `chapter 29 supervisor`, package `ch29-supervisor`.

## Tests

Five tests in `tests/supervisor.rs`.

Eight jobs that each sleep 40 ms share one context pointer. `max_seen` is 8. Elapsed stays under `hold * 7`. The Hermes constant is 10 and `pool.limit()` is not 10.

Nine jobs with a 20 ms hold still peak at `max_seen` 8, and nine replies come back.

Each blocked name at depth 1 is `TeamError::Blocked`. `read_file` at depth 1 is `Ok`. `delegate_task` at depth 0 is `Ok`. `read_file` at depth 2 is `Blocked`.

`worktree(false)` is `None`. `worktree(true)` is `Some("opt-in")`. One `enqueue_reflection` leaves `reflection_len` at 1 and `max_seen` at 0.

`demo` equals `three steps | the draft`.

The package also runs the tests copied forward from chapter 28. Nextest runs all of them.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch29-supervisor
```

```text
Starting 151 tests across 36 binaries
     Summary [   0.344s] 151 tests run: 151 passed, 0 skipped
```

```bash
cargo run --locked -p ch29-supervisor --bin ch29-supervisor
```

```text
three steps | the draft
```

The binary runs two scripted specialists and enqueues a reflection note that is not in the printed line. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `schedule_eight` | 149.22 µs | eight specialists, one context | `run_specialists` on eight jobs that do not sleep |
| `enqueue_reflection` | 18.4 ns | the note, without a reviewer | `push_back` of one string |

`schedule_eight` excludes pool setup and job setup. The timed work is the scope, the permits, the context clone, and the join.

`enqueue_reflection` excludes `Pool::new`. The timed work is the lock and the push. `max_seen` stays 0 across that call. The queue cap inside the function is 64.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `29`. The ids are `29/schedule_eight` and `29/enqueue_reflection`. The old ids `27/schedule_eight` and `27/enqueue_reflection` are aliases in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. The printed Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The two replies are `three steps` and `the draft`.

## Pitfalls

`WORKER_BOUND` is 8. `_DEFAULT_MAX_CONCURRENT_CHILDREN` is 10. Passing 10 to `Pool::new` would be the Hermes default. This chapter's tests construct the pool with `WORKER_BOUND` and assert the limit is not 10. The constant `HERMES_MAX_CONCURRENT_CHILDREN` is documentation in the type system. `acquire` reads `self.limit`.

`delegate_task` at depth 0 is `Ok`. The same string at depth 1 is `Blocked`. A caller who forgets the depth argument is not this function. The function takes the depth. The parent path and the child path are the two numbers.

`worktree(true)` returns the static string `opt-in`. Two opted-in callers still share the process and the pool's `Arc`. The marker is the decision. The git worktree is not created.

`enqueue_reflection` and `acquire` share one mutex. They do not share a meaning. A reflection push does not increment `inflight`. `max_seen` stays 0 when that is the only call. `spawn_reviewer` still exists in `reflection.rs` for chapter 26's nudge path. This queue does not call it.

`schedule_eight` includes thread start and join. It is not the overlap test. The overlap test is the 40 ms hold and the assert against `hold * 7`. The bench sets `hold` to `None`.

`QUEUE_CAP` of 64 here is the reflection deque. Chapter 31's `QUEUE_CAP` of 100 is a different const in `fanout.rs`. Mixing them up schedules the wrong refusal.

The ninth job blocks inside `acquire`. It does not fail the `Result`. `run_specialists` returns `Vec<String>`, not `Result`. Every job that can eventually take a permit returns its reply. The bound shows up as `max_seen`, not as an error.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). This chapter closes M5 and M6.

**M5.** `BLOCKED_TOOLS` is `delegate_task`, `clarify`, `memory`, `send_message`, `cronjob_manage`. That is `DELEGATE_BLOCKED_TOOLS` in `tools/delegate_tool_toolsets.py`. `child_call` returns `TeamError::Blocked` for those names when `depth >= MAX_DEPTH`.

**M6.** `MAX_DEPTH` is 1. Depth greater than 1 is `Blocked` for every tool, including `read_file`. The concurrency cap in Hermes is `_DEFAULT_MAX_CONCURRENT_CHILDREN` of 10, and the config key is `delegation.max_concurrent_children`. This pool's `limit` is `WORKER_BOUND`, 8. `acquire` waits on the condvar at that limit. `max_seen` records the peak.

**D6.** The guardrails doc names subagent worktree isolation as opt-in. `_get_worktree_isolation` defaults to false. `worktree(false)` is `None`. `worktree(true)` is the marker `opt-in`. `tools/subagent_worktree.py` is the Hermes module. This function does not create a git worktree.

**Reflection.** `enqueue_reflection` pushes a string. `chapter_29_reflection_starts_reviewer` stays false. The mean of that push is `18.4 ns`.

**Left for later.** Heartbeats are chapter 36. Chapter 31 is the queue of 100 and the gather in `task_index` order. Chapter 32's critic takes one permit from this pool. A real git worktree stays out.

## Chapter summary

- **Bound.** Eight permits. A ninth waits in `acquire`. Hermes's default concurrent children is 10. `max_seen` peaks at 8.
- **Context.** One `Arc` inside the pool. Specialists clone the pointer with `context()`.
- **Child.** The five blocked tools at depth 1. Any tool at depth greater than 1. The parent at depth 0 may call `delegate_task`.
- **Worktree.** `None` unless opt-in. The opt-in value is `opt-in`. No git worktree is created.
- **Reflection.** A `push_back` on a deque of 64. `max_seen` stays 0. `spawn_reviewer` is not called.
- **Time.** `schedule_eight` mean `149.22 µs`. `enqueue_reflection` mean `18.4 ns`. Both are `mean.point_estimate`. The Criterion `time:` line is the slope.

[personal note: the specialist that only worked because it could not call the parent's tools]

## What you implement

`my-harness/tests/ch29.rs` is the milestone.

```rust
use my_harness::{chapter_29_pool_limit, chapter_29_reflection_starts_reviewer};

#[test]
fn eight_specialists_share_one_pool_and_a_reflection_note_does_not_run_the_reviewer() {
    assert_eq!(chapter_29_pool_limit(), 8);
    assert!(!chapter_29_reflection_starts_reviewer());
}
```

The stubs panic.

```rust
pub fn chapter_29_pool_limit() -> usize {
    todo!("chapter 29")
}

pub fn chapter_29_reflection_starts_reviewer() -> bool {
    todo!("chapter 29")
}
```

`todo!("chapter 29")` is the panic. The test wants the limit equal to 8, and it wants `chapter_29_reflection_starts_reviewer()` to be false. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 30 moves ownership to the next agent. The transcript is filtered first. Role `scratch` is dropped. `MAX_HANDOFFS` is 4, and that cap is this harness. Platform re-bind stays in chapter 39.
