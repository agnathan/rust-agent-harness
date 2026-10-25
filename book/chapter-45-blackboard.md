# Chapter 45: The Blackboard

The researcher is already waiting on the key `ready` when the writer posts `notes`. An editor front door, the work of chapter 44, turns an update into one message on the inbox, and it doesn't make agents call each other. A team that shouldn't have to know who else is on the team needs a board instead: someone posts a fact, someone else waits until the fact they need appears, and a thin scheduler wakes the waiter. This chapter adds that board in `steps/ch45-blackboard`, a copy of chapter 44 plus `blackboard`, and the package is `ch45-blackboard`.

One waiter, named `researcher`, sits on the key `ready`. The call `post("ready", "notes")` stores the fact, and `tick` returns `Woke` for that waiter. Then `answer` `try_lock`s and drops the guard before `FakeModel` runs, so the line that comes back is `blackboard: researcher ok`. If `complete` ran while the mutex was held, every other agent would wait out the model, which is why that guard is gone before the call. The bench is the same claim at a larger size, 100 waiters and one fact.

The module checks a phrase on the way in and obeys a pause flag on the tick, which is the behavior in `blackboard.rs`, and it doesn't call the catalog's threat scanner or the emergency-stop flag by those names. Chapter 46 puts a durable board beside this one, a lock file plus a status and owner check that two processes can lose. This chapter's board lives in memory, which is the right first step and the wrong place to store a task that must survive a crash. Chapter 47's bus couples agents a third way, by topic, with backpressure, and the blackboard stays the pattern of writing a fact and waiting for a fact.

A learning team can publish a result on the board without publishing its scratch. A fact on the board is working state, not a skill, which is why this matters to a continuously improving harness: reflection and curation still decide what is worth keeping, and the scanner stays off this hot path. If every post were a write into chapter 25, the scanner would sit on the path the claim has to keep fast.

At the wait, a mutex guards the posts and a condition variable is what `claim_one` waits on. `Arc` is how tasks share `Shared`, and an injection comes back as `Threat` instead of a panic. The guard drop before `FakeModel` is the scene above.

Running `cargo run -p ch45-blackboard --bin ch45-blackboard` runs the demo. The exercise is `my-harness/tests/ch45.rs`, where 100 waiters must come back as one claim.

| Section | What it walks |
|---|---|
| 45.1 | `MAX_WAKES` and `Waiter` |
| 45.2 | `Threat` and `Claim` |
| 45.3 | `Board`, pause, and `post` |
| 45.4 | `claim_one` |
| 45.5 | `Shared`, `tick`, and `is_injection` |
| 45.6 | `FakeModel`, `answer`, and `demo` |
| 45.7 | the dispatcher tick, the ESTOP, and the four phrases |

> **How Hermes does it**
>
> I looked for a blackboard type. Hermes does not ship one. On Hermes `main` at `9e01b44` the thin scheduler is the kanban dispatcher tick. `hermes_cli/kanban_db_dispatch.py` defines `dispatch_once`. The body takes a non-blocking dispatch lock, runs the tick, and then fires the observer. The comment at line 2029 is the rule I copied the shape of:
>
> 

```python
>     with _kbc._dispatch_tick_lock(db_path) as held:
>         if not held:
>             result = DispatchResult(skipped_locked=True)
>         else:
>             result = _locked_tick()
>             # Still under the dispatch lock: periodic PASSIVE WAL checkpoint.
>             _kbc._maybe_checkpoint_wal(conn, db_path)
>     # Lock released. Fire the tick observer strictly OUTSIDE the critical
>     # section: a slow subscriber must never stall a sibling dispatcher's tick.
>     _kb._fire_dispatch_tick_hook(result, board=board, dry_run=dry_run)
>     return result
> ```

>
> Line 2348 repeats that the dispatch lock has been released before the observer. The comment sits above `def _dispatch_once_locked`, and the words are:
>
> 

```python
> # The dispatch lock has been released here. Fire the tick observer strictly OUTSIDE the single-writer
> # critical section (#56066 sweeper finding / #64231 disposition): a slow subscriber must never extend the
> # lock hold and stall a sibling dispatcher's tick.
> ```

>
> The hook call I found next to the first comment is `_fire_dispatch_tick_hook` at line 2031, after the `with` block ends. The same function also calls the hook at line 2020 when resolving the database path throws, on a tick that never took the lock. I did not find a hook call on line 2348.
>
> Pause is `agent/estop.py`. `is_engaged` at line 52 is true when any candidate sentinel exists, and it fails safe:
>
> 

```python
> def is_engaged() -> bool:
>     """True if ANY candidate sentinel exists; fail SAFE (True) on stat errors."""
>     saw_stat_error = False
>     for path in _candidate_sentinel_paths():
>         try:
>             if path.exists():
>                 return True
>         except OSError:
>             saw_stat_error = True
>     return saw_stat_error
> ```

>
> `check_paused` at line 122 returns true when engaged. It logs once per component per engagement, then returns true. When the sentinel is gone it returns false and drops that component from the logged set:
>
> 

```python
> def check_paused(component: str, logger: logging.Logger) -> bool:
>     """Return True when engaged, logging once per engagement per component (re-armed after a resume)."""
>     if not is_engaged():
>         with _log_lock:
>             _logged_components.discard(component)
>         return False
>     with _log_lock:
>         first = component not in _logged_components
>         _logged_components.add(component)
>     if first:
>         reason = (get_state() or {}).get("reason")
>         suffix = f" (reason: {reason})" if reason else ""
>         logger.info(
>             "%s dispatch paused by global emergency stop%s — remove with `hermes resume` (%s)",
>             component, suffix, sentinel_path(),
>         )
>     return True
> ```

>
> The module docstring above those functions says `hermes pause` writes `$HERMES_HOME/ESTOP`, cron and kanban and new gateway turns skip work, and in-flight work is never killed. A stat error counts as engaged.
>
> The injection phrases come from `tools/threat_patterns.py`. The module comment at line 8 says filler between tokens is the bounded `_FILLER`. Line 19 defines that filler as `(?:\w+\s+){0,8}`. Line 27 is the pattern:
>
> 

```python
> (rf'ignore\s+{_FILLER}(previous|all|above|prior)\s+{_FILLER}instructions', "prompt_injection", "all"),
> ```

>
> The four alternations are `previous`, `all`, `above`, and `prior`. The scope on that tuple is `all`. `_FILLER` allows zero to eight words between `ignore` and the alternation, and again between the alternation and `instructions`.
>
> **Adopted.** The scheduler claims under the lock via `tick`, then `answer` `try_lock`s before `FakeModel` so the lock is not held across `complete`. A paused board returns `Claim::Paused` and claims nothing. `post` refuses four phrases, `ignore previous instructions`, `ignore all instructions`, `ignore above instructions`, and `ignore prior instructions`, case-insensitive, and does not store the fact.
>
> **Adapted.** Four exact substring phrases, no filler words. `Shared` uses `Condvar::notify_all`. `MAX_WAKES` is 8. That constant is this chapter's extra. It is not a Hermes constant I found.
>
> **Deferred.** Durable compare-and-swap is chapter 46. The board is a `HashMap` in a `Mutex`, not SQLite.

> **Guardrails.** Injection refusal at `post`. A paused board claims nothing. The lock is released before the model. The wake cap leaves the fact on the board. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

## What this chapter decides

A post stores a key and a value, or it returns `Threat` and stores nothing, because the check is `is_injection` on the value after `to_ascii_lowercase`. The four needles are exact substrings, so a value that would match only once filler words are allowed still gets stored.

The scheduler locks inside `tick` and calls `claim_one`; a paused board returns `Claim::Paused` before the scan, and otherwise the first waiter whose `wakes` is under `MAX_WAKES` and whose key is present is woken, that waiter's `wakes` goes up by one, and the fact is removed. The `MutexGuard` ends when `tick` returns, which is the opening's rule on the scheduler side, and `Claim::Idle` means no waiter matched, so the facts stay.

The reply path calls `try_lock` before `FakeModel::complete` and drops that guard before the call. If the scheduler still held the lock, `try_lock` would fail and `expect` would panic with `lock held across the model`.

Pause sets the flag and resume clears it, and a `Claim::Woke` already returned can still be passed to `answer`, because the flag stops the next `tick` and doesn't reach into a claim the caller already holds.

The demo posts `notes` for the waiter `researcher` on the key `ready`, claims it, and returns `blackboard: researcher ok`.

## 45.1 `MAX_WAKES` and `Waiter`

The new module is `blackboard.rs`, the chapter 44 copy is `acp.rs`, and `pub mod blackboard` is what `chapter_demo.rs` imports. The constant and the waiter sit at the top of the file.

```rust
//! Book chapter 45. Agents share one board. A condition wakes a waiter.
//! The scheduler claims under the lock, then drops it before the model.

use std::collections::HashMap;
use std::sync::{Condvar, Mutex, MutexGuard};

/// A waiter that has already fired this many times is left asleep.
pub const MAX_WAKES: u32 = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Waiter {
    pub name: String,
    pub key: String,
    pub wakes: u32,
}

impl Waiter {
    pub fn new(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            key: key.into(),
            wakes: 0,
        }
    }
}
```

The cap is a `u32` because `wakes` is a `u32`, the value is 8, and `claim_one` compares `waiter.wakes < MAX_WAKES`, so a waiter at 8 is skipped while the fact that waiter was watching can still sit in the map. The doc comment on the constant says a waiter that has already fired this many times is left asleep.

A waiter has three public fields: `name` is the string that shows up in `Claim::Woke` and in the demo line, `key` is the fact this waiter wants, and `wakes` starts at 0 inside `new`. The derives are `Debug`, `Clone`, `PartialEq`, and `Eq`, and there's no `Copy`, because a `String` can't be copied bitwise, so cloning a waiter copies both strings and the counter.

The constructor takes `impl Into<String>` for both arguments, the same bound chapter 43 uses when `Job::new` takes `impl Into<String>`, so `Waiter::new("researcher", "ready")` and the bench's `format!` strings share one function. Wakes isn't a parameter, every new waiter starts at zero, and the waiter owns both strings once `new` returns. A test that wants a waiter already at the cap has to write the field or claim eight times, which is what the tests do.

## 45.2 `Threat` and `Claim`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Threat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Claim {
    Woke { name: String, value: String },
    Idle,
    Paused,
}
```

The refusal is a unit struct, `Threat`, with no fields, and `post` returns `Result<(), Threat>`, so the error carries no message and no phrase id. `Copy` is legal because there's nothing to clone, `Clone` is derived as well, and a test checks `is_err()` without matching on a payload.

One scheduler pass returns `Claim`, where `Woke` holds the waiter's name and the fact value, both owned `String`s, while `Idle` and `Paused` hold nothing. The enum doesn't derive `Copy`, because `Woke` owns strings, and it does derive `Clone` and `PartialEq`, so `assert_eq!` compares a whole `Claim` to a struct literal while `matches!` in other tests only checks the variant.

The model helper borrows a `&Claim`, and the demo matches on `&claim` to read the name, then uses the same claim again as the argument to `answer`, which is why both can run. Moving the claim into `answer` would have forced the demo to clone it or to split the fields first.

> **Rust sidebar: a unit struct and a data-carrying variant**
>
> `pub struct Threat;` is a type with one value, written `Threat`, and it isn't `()`, so `Result<(), Threat>` uses `()` as the success value and `Threat` as the error. Callers can't confuse a refused post with a successful one that stored an empty string, because success is `Ok(())` only after `insert`.
>
> `Claim::Woke { name, value }` is a struct variant with named fields, and `Claim::Woke { value, .. }` in `answer` binds `value` and ignores `name`. `Claim::Idle` and `Claim::Paused` are unit variants, so a match that names all three is exhaustive, and adding a fourth variant would fail to compile in `answer` and in `demo` until those matches learned an arm.

## 45.3 `Board`, pause, and `post`

```rust
#[derive(Debug)]
pub struct Board {
    facts: HashMap<String, String>,
    waiters: Vec<Waiter>,
    paused: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
            waiters: Vec::new(),
            paused: false,
        }
    }

    pub fn with_waiters(waiters: Vec<Waiter>) -> Self {
        Self {
            facts: HashMap::new(),
            waiters,
            paused: false,
        }
    }

    pub fn waiter_count(&self) -> usize {
        self.waiters.len()
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn has_fact(&self, key: &str) -> bool {
        self.facts.contains_key(key)
    }

    /// Refuse a value that carries an injection phrase. The fact is not stored.
    pub fn post(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<(), Threat> {
        let value = value.into();
        if is_injection(&value) {
            return Err(Threat);
        }
        self.facts.insert(key.into(), value);
        Ok(())
    }
```

The fields are private, so callers outside the module use the methods. Facts live in a `HashMap<String, String>` with both sides owned, waiters live in a `Vec<Waiter>` in insertion order that `claim_one` scans from the front, and `paused` starts false in both constructors.

An empty board comes from `new`, with an empty map, an empty vec, and `paused: false`, so `waiter_count` on that board is 0. The test `demo_names_the_waiter` asserts `Board::new().waiter_count() == 0` after it checks the demo line. `with_waiters` takes the vec by value and stores it, the map still starts empty, and the flag still starts false. Passing a vec of 100 waiters doesn't post a fact, because the bench posts after `with_waiters`.

Pause sets the flag, resume clears it, and `is_paused` reads it. None of those three touch `facts` or `waiters`, so a fact posted before `pause` is still in the map, and `claim_one` is what refuses to take it.

A lookup is `has_fact`, which is `contains_key` on a borrowed `&str`, and it doesn't remove the entry. That is why the condvar test loops on `while !guard.has_fact("ready")`, and why the claim test asserts `has_fact("ready")` is false after a successful claim and true after the ninth post that `tick` refuses.

A post converts the value first, then checks `is_injection`, then inserts, and the key is converted only on the success path, so `key.into()` doesn't run when the function returns `Err(Threat)`. An injection value is dropped with the local `value` binding, the map is unchanged, and the doc comment says the fact isn't stored. On a clean value, `insert` replaces any previous value for that key and returns the old one, which this function discards, and the tests post each key once.

> **Rust sidebar: private fields and `HashMap::insert`**
>
> Privacy is per module, and `facts` has no `pub`, so a test in `tests/blackboard.rs` can't write `board.facts` and calls `post`, `has_fact`, `pause`, and `resume` instead. The integration test sits outside `harness_core`, so it sees only the public API, and `claim_one` is the one public function that removes a key.
>
> `insert` returns `Option<String>`, the previous value, and `post` ignores that option. The `Result` is about the injection check rather than about whether the key was new: `Ok(())` means the value passed the check and was stored, and `Err(Threat)` means the check failed and `insert` didn't run.

## 45.4 `claim_one`

```rust
    /// One waiter whose key is on the board, or idle. The fact leaves with the claim.
    pub fn claim_one(&mut self) -> Claim {
        if self.paused {
            return Claim::Paused;
        }
        let index = self
            .waiters
            .iter()
            .position(|waiter| waiter.wakes < MAX_WAKES && self.facts.contains_key(&waiter.key));
        let Some(index) = index else {
            return Claim::Idle;
        };
        let waiter = &mut self.waiters[index];
        waiter.wakes += 1;
        let name = waiter.name.clone();
        let key = waiter.key.clone();
        let value = self.facts.remove(&key).unwrap_or_default();
        Claim::Woke { name, value }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
```

The pause check is first, so `Claim::Paused` returns before `position` runs, waiter counters stay where they were, and facts stay in the map. The test `a_paused_board_does_not_claim` posts `notes`, pauses, and expects `tick` to return `Paused`, and after `resume` the same fact is still there and the next `tick` is `Woke`.

The scan walks `waiters` from index 0, and the predicate needs both `wakes < MAX_WAKES` and a fact whose key equals `waiter.key`. The first waiter that passes is the one this call claims, and later waiters wait for a later call. The test `a_claim_wakes_the_matching_waiter_once` puts waiter `idle` on key `other` in front of waiter `researcher` on key `ready` and posts only `ready`, so `idle` fails `contains_key`, `researcher` matches, and the claim's name is `researcher` with value `notes`.

The let-else returns `Idle` when `position` returns `None`, which happens when the vec is empty, when no fact key matches, or when every matching waiter is already at 8 wakes, and that `Idle` doesn't remove a fact. The wake-cap test posts `notes` a ninth time, gets `Idle`, and `has_fact("ready")` is still true.

On a match, `wakes` increments before the clones, the increment is stored on the waiter in the vec, and the name and the key are cloned so the function can mutably borrow `self.facts` afterward. Remove takes the value out of the map, the doc comment says the fact leaves with the claim, and `unwrap_or_default` yields an empty `String` if `remove` returns `None`. The predicate already required `contains_key` for that same key on this same `&mut self`, so the key is present when `remove` runs and the `Woke` value is that removed string, and the demo builds its board with `with_waiters`.

> **Rust sidebar: `let-else` and `Iterator::position`**
>
> `let Some(index) = index else { return Claim::Idle; }` is a let-else, the `else` arm must diverge, and `return` diverges, so after the statement `index` is a `usize` rather than an `Option` and the next line indexes `self.waiters[index]`.
>
> `position` takes a predicate and returns `Option<usize>`. The closure borrows each `Waiter` and also reads `self.facts`, and that borrow ends when `position` returns, which is why the mutable borrow of the chosen waiter is legal on the next line. The increment `wakes += 1` uses that mutable borrow, and the clones end it before `remove`.

## 45.5 `Shared`, `tick`, and `is_injection`

```rust
pub struct Shared {
    pub board: Mutex<Board>,
    pub cv: Condvar,
}

impl Shared {
    pub fn new(waiters: Vec<Waiter>) -> Self {
        Self {
            board: Mutex::new(Board::with_waiters(waiters)),
            cv: Condvar::new(),
        }
    }

    pub fn post(&self, key: impl Into<String>, value: impl Into<String>) -> Result<(), Threat> {
        let mut board = self.board.lock().expect("blackboard");
        board.post(key, value)?;
        self.cv.notify_all();
        Ok(())
    }
}

/// The thin scheduler. The guard ends before this function returns.
pub fn tick(lock: &Mutex<Board>) -> Claim {
    let mut board = lock.lock().expect("blackboard");
    board.claim_one()
}

/// Phrases adapted from the Hermes prompt-injection pattern. Filler words are not matched.
pub fn is_injection(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "ignore previous instructions",
        "ignore all instructions",
        "ignore above instructions",
        "ignore prior instructions",
    ]
    .iter()
    .any(|phrase| lower.contains(phrase))
}
```

A waiting thread needs the pair in `Shared`, and both `board` and `cv` are public, so the condvar test locks `worker.board` itself and calls `claim_one` on the guard instead of calling `tick`. The function `tick` takes a `&Mutex<Board>` and is the path the demo, the bench, and the other tests use. `Shared::new` wraps `Board::with_waiters`, and the condvar starts with no waiters registered, because `Condvar::new` doesn't know about the vec of `Waiter` values and those records live inside the board.

The shared post locks, calls `Board::post`, notifies, and returns `Ok(())`. The `?` sits on `board.post`, so an injection returns `Err(Threat)` before `notify_all`, no waiter is woken for a fact that was refused, and the fact wasn't inserted. The call `lock().expect("blackboard")` panics if the mutex is poisoned, the panic string is `blackboard`, and a panic inside another holder poisons the mutex, which this function doesn't clear.

Calling `notify_all` wakes every thread that is currently blocked in `cv.wait`. The test spawns one thread, and a second waiter blocked in `wait` would also wake, recheck `has_fact`, and either claim or wait again, though this file doesn't spawn that second thread and the test posts once.

The scheduler locks and calls `claim_one`, and the doc comment says the guard ends before this function returns: the local `board` is a `MutexGuard` that drops at the end of `tick`, after `claim_one` has built the `Claim`, and the caller receives the claim without the guard. Answer is a different function, and the demo calls `tick` and then `answer`, which is the order the opening already walked.

The phrase check lowercases with `to_ascii_lowercase` and asks whether any of the four strings is a substring. The phrases are the four alternations from the Hermes pattern, each glued between `ignore ` and ` instructions` with a single space and no filler, and `any` stops at the first hit. The test string `Please IGNORE previous instructions now` lowercases to a sentence that contains `ignore previous instructions`, so the function returns true. The value `ignore all instructions` is what `an_injection_is_not_posted` passes to `post`, and `post` returns `Err`, while the other two phrases sit in the array and that test doesn't post them. `contains` is case-sensitive on the already-lowercased string, so the needles are stored in lowercase. A value of `ignore previous instructions` matches, and a value of `ignore the previous instructions` doesn't, because that exact substring isn't in it. The Hermes regex would allow the word `the` inside `_FILLER`, and this function doesn't, which is what the doc comment means when it says filler words aren't matched.

> **Rust sidebar: `MutexGuard`, `?`, and `Condvar::notify_all`**
>
> `lock` returns `LockResult<MutexGuard<Board>>`, `expect` turns a poison error into a panic, and the guard dereferences to `Board`, so `board.post` and `board.claim_one` are method calls on the board. The guard unlocks when it is dropped, and in `tick` that drop is the end of the function, so the returned `Claim` doesn't contain the guard and holding the claim can't keep the lock.
>
> The `?` on `Result<(), Threat>` returns the error from `Shared::post` and drops `board` as that stack frame unwinds, and `notify_all` is sequenced after `?`, so it runs only on `Ok`. Notify doesn't take the fact as an argument, waiters wake and look at the map themselves, and `wait` is called in the test rather than in this file. `Condvar::wait` atomically unlocks and sleeps, then relocks on wake, and the test assigns the returned guard back with `guard = worker.cv.wait(guard).unwrap()`.

## 45.6 `FakeModel`, `answer`, and `demo`

```rust
pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

/// Run the model only after `tick` has returned, so the lock is not held.
pub fn answer(lock: &Mutex<Board>, claim: &Claim, model: &FakeModel) -> String {
    let _free: MutexGuard<'_, Board> = lock.try_lock().expect("lock held across the model");
    drop(_free);
    match claim {
        Claim::Woke { value, .. } => model.complete(value),
        Claim::Idle | Claim::Paused => model.complete(""),
    }
}

pub fn demo() -> String {
    let board = Mutex::new(Board::with_waiters(vec![Waiter::new(
        "researcher",
        "ready",
    )]));
    board
        .lock()
        .expect("blackboard")
        .post("ready", "notes")
        .expect("clean fact");
    let claim = tick(&board);
    let reply = answer(&board, &claim, &FakeModel { reply: "ok" });
    let name = match &claim {
        Claim::Woke { name, .. } => name.as_str(),
        Claim::Idle => "idle",
        Claim::Paused => "paused",
    };
    format!("blackboard: {name} {reply}")
}
```

The stand-in model holds a static reply, `complete` reads `prompt` and discards it with `let _ = prompt`, and the returned string is `self.reply.to_string()`. The bench doesn't construct this type, `descriptions.json` says `FakeModel` isn't constructed in the sample, and the tests that want a reply construct `FakeModel { reply: "ok" }`.

The reply function takes the mutex, a borrowed claim, and a borrowed model. The first line is `try_lock`, and success means this thread acquired the mutex, which means no other guard was alive, including a guard left over from `tick`. The type ascription `MutexGuard<'_, Board>` names the guard, `drop(_free)` unlocks before the `match`, and `complete` then runs with the mutex free, the same drop the opening walked. A failed `try_lock` hits `expect("lock held across the model")` and panics. The test `the_model_runs_after_the_lock_is_released` calls `tick` and then `answer` and expects `ok`, and it doesn't hold a guard across the call.

The match passes the fact value into `complete` only for `Woke`, while `Idle` and `Paused` pass an empty prompt. Complete ignores the prompt either way and returns the reply, and `answer` still branches, so a later model that read its prompt would see the fact only on `Woke`.

The demo builds a mutex around one waiter, name `researcher`, key `ready`. It locks, posts `ready` / `notes`, and `expect("clean fact")` panics if `post` returns `Threat`, which `notes` doesn't, because that string contains none of the four phrases. Tick returns `Woke` with that name and that value, the fact is gone, and the waiter's `wakes` is 1. Answer try-locks, drops the guard, and completes, so the reply is `ok`, and the name match reads `name.as_str()` from the `Woke` arm, which means `format!("blackboard: {name} {reply}")` returns `blackboard: researcher ok`, the string the chapter binary prints.

```rust
use harness_core::blackboard::demo;

fn main() {
    println!("{}", demo());
}
```

Running the package needs `--bin ch45-blackboard`, because the example binary prints eval lines. No key is read, and `FakeModel::complete` ignores the prompt. This type is `harness_core::blackboard::FakeModel`, and the `acp` module in the same crate has its own `FakeModel`, so they are different types.

> **Rust sidebar: `try_lock` and implicit format arguments**
>
> `try_lock` returns immediately, where `lock` would wait, and `answer` uses `try_lock` so a held mutex is a panic rather than a wait, the scene from the opening. The guard's name is `_free`, the underscore prefix keeps the unused-binding warning quiet once the type has done its job, and `drop(_free)` is explicit so the unlock happens before `complete`.
>
> `format!("blackboard: {name} {reply}")` captures the locals by name, the colon and the spaces are literal, `name` is `&str`, and `reply` is `String`, and both implement `Display`. The format string doesn't include the fact value, because the fact was the model's prompt and the model discarded it, so the printed line is the waiter name and the reply.

## 45.7 The dispatcher tick, the ESTOP, and the four phrases

The dispatcher function `dispatch_once` in `hermes_cli/kanban_db_dispatch.py` runs one dispatcher tick under the board's single-writer lock, wrapping `_dispatch_once_locked` in `_dispatch_tick_lock`, so two dispatchers on one `kanban.db` don't race a write tick. The loser returns a `DispatchResult` with `skipped_locked=True` and writes nothing, and the lock is keyed on the resolved database path. After the `with` block, the comment at line 2029 says the lock is released and the tick observer fires outside the critical section, because a slow subscriber must never stall a sibling dispatcher's tick, and `_fire_dispatch_tick_hook` is the next statement. In that design the model is whatever the subscriber does, and it does that work after the lock.

This chapter's subscriber is `answer`, and `tick` is the critical section: a `Mutex` around a `HashMap`, claiming one waiter rather than spawning a process. The guard drops when `tick` returns, and `answer` then `try_lock`s, which is the opening's release again. The line 2348 comment says the same thing above `_dispatch_once_locked`, and the hook call itself is at line 2031.

The pause in Hermes is `agent/estop.py`, where `is_engaged` returns true if any candidate sentinel exists, a stat error sets `saw_stat_error`, and the function returns that flag at the end, so an `OSError` from `path.exists` fails safe as engaged. `check_paused` returns true when `is_engaged` is true, and the module docstring says in-flight work is never killed. `Board::pause` is a bool on the struct rather than a sentinel file, and it doesn't fail safe on a stat error because it doesn't stat. `claim_one` sees the flag and returns `Paused` without incrementing `wakes` and without `remove`. A claim the caller already holds is in-flight in the same sense, so `answer` will still call `complete` on it, and `resume` clears the flag so the next `tick` can claim. Hermes re-arms its log line by discarding the component from `_logged_components` when the sentinel is gone. This board has no log line, and `resume` only writes `false`.

Line 27 of `tools/threat_patterns.py` matches `ignore`, then bounded filler, then `previous|all|above|prior`, then filler, then `instructions`, with pattern id `prompt_injection` and scope `all`. Line 8 says the filler is bounded, and `_FILLER` is `(?:\w+\s+){0,8}`, zero to eight words. `is_injection` keeps the four words and drops the filler, the check is `str::contains` of the whole phrase, and `post` refuses those four, case-insensitive, before `insert`. `Shared::post` then skips `notify_all` because of `?`, so a refused fact can't wake the condvar test's thread.

The wake cap is 8, and I didn't find that constant in `estop.py`, `threat_patterns.py`, or the dispatcher comment quoted above. It is this file's limit so one waiter can't be chosen forever while its key keeps being posted. On the ninth attempt `position` fails the `wakes < MAX_WAKES` clause, `tick` returns `Idle`, and `remove` doesn't run, so the fact stays for a waiter that isn't at the cap. The tests never add that second waiter, and they only assert that the fact is still there.

The inbox from chapter 44 is still in this crate, in `acp.rs` and in `inbox.rs`. The blackboard doesn't submit a mail, the demo doesn't call `translate`, the copied ACP tests still build, and the blackboard tests don't call them.

> **Rust sidebar: `to_ascii_lowercase` allocates**
>
> `to_ascii_lowercase` returns a new `String`, it changes `A` through `Z`, and it leaves other bytes alone. The four needles are ASCII, so a phrase written in ASCII capitals matches, which is why `IGNORE` in the test is enough. The function doesn't trim and doesn't strip punctuation, and the substring match is why `Please IGNORE previous instructions now` is an injection without the phrase being the whole value. A Unicode case fold isn't what this call does. The Hermes pattern is a regex on the original text with `\s+` between tokens, and this function lowercases first and then looks for a single space.

## Tests

The new tests live in `steps/ch45-blackboard/crates/harness-core/tests/blackboard.rs`, and the package also builds the copied harness tests, including `acp.rs`.

A matching waiter is claimed once in `a_claim_wakes_the_matching_waiter_once`, which builds a mutex of two waiters, `idle` on `other` and `researcher` on `ready`, posts `ready` / `notes`, and expects `tick` to equal `Claim::Woke` with name `researcher` and value `notes`. After that claim, `has_fact("ready")` is false, and a second `tick` matches `Idle`.

The model runs after the release in `the_model_runs_after_the_lock_is_released`, which posts `notes` for `researcher`, ticks, and calls `answer` with `FakeModel { reply: "ok" }`. The reply equals `ok`, because `answer` has already try-locked, and a lock still held would have panicked on `expect("lock held across the model")` before `complete`.

A paused board refuses the claim in `a_paused_board_does_not_claim`, which posts `notes` and calls `pause` inside one guard so the flag and the fact land together. The guard ends at the close of the block, `tick` equals `Claim::Paused`, `resume` clears the flag, and the next `tick` matches `Claim::Woke` because the fact was still in the map.

An injection never lands in `an_injection_is_not_posted`, which asserts `is_injection("Please IGNORE previous instructions now")`, then posts `ignore all instructions` on a bare `Board::new` and expects `is_err`. `has_fact("ready")` is false, because the key was never inserted.

The cap stops the waiter in `a_waiter_stops_after_the_wake_cap`, whose loop is `for _ in 0..MAX_WAKES`, and because `MAX_WAKES` is 8 the range is eight posts and eight `Woke` claims, each of which removes the fact, which is why the loop posts again. After the loop, `wakes` is 8, one more post of `notes` stores the fact, `tick` returns `Idle`, and `has_fact("ready")` is true, so the ninth fact is the one the cap left on the board.

One thread wakes in `a_condition_wakes_one_thread`, which builds `Arc<Shared>` with waiter `researcher` on `ready`. The spawned thread locks `worker.board` and waits while `has_fact("ready")` is false, reassigning the guard from `cv.wait`, then calls `claim_one`, drops the guard, asserts `try_lock` is ok, and calls `FakeModel { reply: "ok" }.complete("notes")`. The parent posts `ready` / `notes` through `Shared::post`, which inserts and calls `notify_all`, and `join` expects `Woke` with name `researcher` and value `notes`, plus reply `ok`. The `while` rechecks the fact so a notify that arrived before `wait` still claims: the thread sees the key and doesn't sleep.

The demo names its waiter in `demo_names_the_waiter`, which expects `demo()` to equal `blackboard: researcher ok` and `Board::new().waiter_count()` to equal 0.

The seven test functions appear below in the order the file defines them.

```rust
fn a_claim_wakes_the_matching_waiter_once()
fn the_model_runs_after_the_lock_is_released()
fn a_paused_board_does_not_claim()
fn an_injection_is_not_posted()
fn a_waiter_stops_after_the_wake_cap()
fn a_condition_wakes_one_thread()
fn demo_names_the_waiter()
```

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch45-blackboard
```

```text
    Starting 236 tests across 50 binaries
     Summary [   0.357s] 236 tests run: 236 passed, 0 skipped
```

The summary above is the whole package, copied tests included.

```bash
cargo run -p ch45-blackboard --bin ch45-blackboard
```

```text
blackboard: researcher ok
```

The binary posts one fact and claims it, and no key is read.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `claim_with_waiters` | 4.96 µs | a claim under 1 ms with 100 waiters | lock, scan, remove the fact |

```rust
fn claim_with_waiters(c: &mut Criterion) {
    c.bench_function("claim_with_waiters", |b| {
        b.iter_batched(
            || {
                let mut waiters = Vec::with_capacity(100);
                for n in 0..99 {
                    waiters.push(Waiter::new(format!("w{n}"), format!("k{n}")));
                }
                waiters.push(Waiter::new("chosen", "ready"));
                let mut board = Board::with_waiters(waiters);
                board.post("ready", "notes").expect("clean fact");
                Mutex::new(board)
            },
            |board| {
                let claim = tick(black_box(&board));
                black_box(claim);
            },
            BatchSize::SmallInput,
        );
    });
}
```

> **Rust sidebar: `black_box` and `BatchSize::SmallInput`**
>
> `black_box(&board)` keeps the compiler from proving the mutex is unused and deleting `tick`, and `black_box(claim)` keeps it from deleting the call once it sees that nobody reads the `Claim`. `BatchSize::SmallInput` tells Criterion the setup value is small enough to batch, and the setup value here is a `Mutex<Board>`, so each timed iteration receives its own mutex from a setup call and the next iteration doesn't see the removed fact. Every sample starts from a board that still holds `notes`.

The setup closure runs outside the timer, building waiters `w0` through `w98` on keys `k0` through `k98`, then `chosen` on `ready`, posting `notes`, and wrapping the board in a `Mutex`, and none of that construction is timed. The milestone `chapter_45_one_claim(100)` wants that 100 to come back as 1, and the stub doesn't compute it yet.

The means are `mean.point_estimate` from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), the bench ids are `45/claim_with_waiters`, `43/claim_with_waiters`, and `41/claim_with_waiters`, and the command is `cargo bench -p ch45-blackboard --bench blackboard --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The bench is the claim, so a number that spawns 100 threads and waits on a condvar measures `a_condition_wakes_one_thread` rather than `claim_with_waiters`. The descriptions file says the waiters are records, `Condvar` isn't in the sample, and `FakeModel` isn't in the sample either.

A guard that is still alive when `answer` runs trips `expect("lock held across the model")`, which is how the opening's rule shows up as a panic. `drop(_free)` runs before the match so `answer` itself doesn't hold the guard across `complete`.

Eight wakes is the cap, the ninth `post` still stores the fact, `tick` leaves it there and returns `Idle`, and that constant isn't one from the Hermes files quoted above.

The claim picks the first matching waiter, so a second waiter on the same key isn't claimed in that call, and because the fact is already removed that second waiter needs a new `post`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The catalog row for the shared threat patterns is F1, `tools/threat_patterns.py`. The catalog row for the emergency stop is N2, `agent/estop.py`. This module does not call `scan_for_threats` or `is_engaged`. The behaviors below are the ones `blackboard.rs` implements.

**Injection refusal at post.** `is_injection` matches four exact phrases, case-insensitive. `Board::post` returns `Err(Threat)` and does not `insert`. The test `an_injection_is_not_posted` checks the `IGNORE previous` sentence and the `ignore all instructions` post.

**Pause claims nothing.** `claim_one` returns `Claim::Paused` when the flag is set. The waiter is not incremented. The fact stays. `a_paused_board_does_not_claim` covers that return, then `resume`.

**Lock released before the model.** `tick` drops its guard on return. `answer` `try_lock`s and panics with `lock held across the model` if the mutex is held. It `drop`s that guard before `FakeModel::complete`. `the_model_runs_after_the_lock_is_released` and the condvar test both `try_lock` around the model.

**Wake cap.** `MAX_WAKES` is 8. The ninth `tick` of the same waiter returns `Idle` and leaves the fact on the board. `a_waiter_stops_after_the_wake_cap` asserts `has_fact("ready")` after that `Idle`.

**E1 and E2.** `blackboard.rs` does not call `redact_sensitive_text` or `redact_for_egress`. A fact value is stored as posted and handed to `complete` as posted. E1 and E2 are not called by this module.

## What you implement

The milestone the reader implements is `my-harness/tests/ch45.rs`, and the test below is what has to pass.

```rust
use my_harness::chapter_45_one_claim;

#[test]
fn one_hundred_waiters_one_claim() {
    assert_eq!(chapter_45_one_claim(100), 1);
}
```

The stub panics on purpose, with the body below.

```rust
pub fn chapter_45_one_claim(waiters: usize) -> usize {
    let _ = waiters;
    todo!("chapter 45")
}
```

The panic is `todo!("chapter 45")`, and the test passes `100` and wants `1` back: one hundred waiters and one claim. In the step crate that is the bench shape, ninety-nine waiters on other keys, one waiter on `ready`, one `post` of `notes`, and one `tick` that returns a single `Woke`, while the stub stays `todo!`.

## Chapter summary

- The board keeps facts in a `HashMap` and waiters in a `Vec`, behind one `Mutex`, so a post and a claim share the same map.
- The scheduler's `tick` claims one waiter and drops the guard before it returns, which is the rule the opening walked with `researcher`.
- The reply path try-locks, drops that guard, and then `FakeModel` returns `ok`, so the demo line is `blackboard: researcher ok`.
- Pause returns `Claim::Paused` and leaves the fact in place, while a claim already in hand still reaches `answer`.
- Injection is four exact phrases, and `post` refuses them instead of storing the value.
- The cap is eight wakes, and the next fact stays on the board once that waiter has hit it.
- The `claim_with_waiters` mean is 4.96 µs, `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), id `45/claim_with_waiters`, aliases `43/claim_with_waiters` and `41/claim_with_waiters`.

## What's next

Two processes can both want the same task, and chapter 46 is a lock file plus a status and owner check that lets one of them lose.
