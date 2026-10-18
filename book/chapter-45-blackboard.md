# Chapter 45: The Blackboard

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 44 ends a turn. One update becomes one mail, and the mail sits on that process's inbox. A second agent cannot read it. This chapter is the blackboard. Facts and waiters live on one `Board`. `tick` claims under the lock. `answer` runs `FakeModel` only after that lock is free.

The step crate is `steps/ch45-blackboard`. The book number is 45. The package is `ch45-blackboard`. The library name is `harness_core`. This is the chapter 44 harness copied forward, including `acp`, plus `blackboard`. `Cargo.toml` says "Chapter 45: the previous harness plus this chapter." Edition is 2021. `rust-version` is 1.88. There is no path dependency on `ch44-acp`. The sources are copied. `lib.rs` adds `pub mod blackboard`. The file comment at the top of `lib.rs` still describes chapter 10's loop: chapter 9's agent loop, plus the turn table that `run_loop` drives. `blackboard.rs` itself says book chapter 45. The binary that prints this chapter is `src/bin/chapter_demo.rs`, named `ch45-blackboard`. The package also builds `ch45_mcp_fixture` and `sandbox-worker`. `examples/demo.rs` is the inherited eval runner. It calls `run_saved_evals` and `run_stress_evals`. It does not print the blackboard line. `ch45-harness-cli` depends on `ch45-blackboard` by path. The bench target is `blackboard`, with `harness = false`.

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
> ```python
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
> ```python
> # The dispatch lock has been released here. Fire the tick observer strictly OUTSIDE the single-writer
> # critical section (#56066 sweeper finding / #64231 disposition): a slow subscriber must never extend the
> # lock hold and stall a sibling dispatcher's tick.
> ```
>
> The hook call I found next to the first comment is `_fire_dispatch_tick_hook` at line 2031, after the `with` block ends. The same function also calls the hook at line 2020 when resolving the database path throws, on a tick that never took the lock. I did not find a hook call on line 2348.
>
> Pause is `agent/estop.py`. `is_engaged` at line 52 is true when any candidate sentinel exists, and it fails safe:
>
> ```python
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
> ```python
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
> ```python
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

[personal note: the moment the lock had to end before the fake reply]

## What this chapter decides

`post` stores a key and a value, or returns `Threat` and stores nothing. The check is `is_injection` on the value after `to_ascii_lowercase`. The four needles are exact substrings. A value that only matches once filler words are allowed is stored.

`tick` locks and calls `claim_one`. A paused board returns `Claim::Paused` before the scan. Otherwise the first waiter whose `wakes` is under `MAX_WAKES` and whose key is present is woken. That waiter's `wakes` goes up by one. The fact is removed. The `MutexGuard` ends when `tick` returns. `Claim::Idle` means no waiter matched. The facts stay.

`answer` calls `try_lock` before `FakeModel::complete`. The guard from that `try_lock` is dropped before `complete`. If the scheduler still held the lock, `try_lock` would fail and `expect` would panic with `lock held across the model`.

`pause` sets the flag. `resume` clears it. A `Claim::Woke` already returned can still be passed to `answer`. The flag stops the next `tick`. It does not reach into a claim the caller already holds.

`demo` posts `notes` for the waiter `researcher` on the key `ready`, claims it, and returns `blackboard: researcher ok`.

## 45.1 `MAX_WAKES` and `Waiter`

`blackboard.rs` is the new module. `acp.rs` is the chapter 44 copy. `pub mod blackboard` is what `chapter_demo.rs` imports. The constant and the waiter sit at the top:

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

`MAX_WAKES` is a `u32` because `wakes` is a `u32`. The value is 8. `claim_one` compares `waiter.wakes < MAX_WAKES`. A waiter at 8 is skipped. The fact that waiter was watching can still sit in the map. The doc comment on the constant says a waiter that has already fired this many times is left asleep.

`Waiter` has three public fields. `name` is the string that shows up in `Claim::Woke` and in the demo line. `key` is the fact key this waiter wants. `wakes` starts at 0 inside `new`. The derives are `Debug`, `Clone`, `PartialEq`, and `Eq`. There is no `Copy`. A `String` cannot be copied by bitwise copy. Cloning a waiter copies both strings and the counter.

`new` takes `impl Into<String>` for both arguments. `Waiter::new("researcher", "ready")` passes two `&str` values. `name.into()` builds the owned `String`. `Waiter::new(format!("w{n}"), format!("k{n}"))` in the bench passes `String` values, and `Into<String> for String` is the identity. The same function serves both call sites. `wakes` is not a parameter. Every new waiter starts at zero wakes. A test that wants a waiter already at the cap has to write the field, or claim eight times. The tests claim eight times.

### Rust: `impl Into<String>`

`impl Into<String>` is a generic bound written in argument position. The caller chooses the type. The function body calls `.into()` and stores a `String`. `&str`, `String`, and anything else that implements `Into<String>` are legal. The bound is on each argument separately, so the name and the key can be different source types in one call. `new` does not borrow the caller after it returns. The `Waiter` owns both strings.

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

`Threat` is a unit struct. It has no fields. `post` returns `Result<(), Threat>`. The error carries no message and no phrase id. `Copy` is legal because there is nothing to clone. `Clone` is derived as well. A test checks `is_err()` and does not match on a payload.

`Claim` is the result of one scheduler pass. `Woke` holds the waiter's name and the fact value, both owned `String`s. `Idle` and `Paused` hold nothing. The enum does not derive `Copy`, because `Woke` owns strings. It does derive `Clone` and `PartialEq`. `assert_eq!` in the tests compares a whole `Claim` to a struct literal. `matches!` in other tests only checks the variant.

`answer` borrows a `&Claim`. `demo` matches on `&claim` to read the name, then uses the same claim again as the argument to `answer`. The borrow is why both can run. Moving the claim into `answer` would have forced `demo` to clone it or to split the fields first.

### Rust: a unit struct and a data-carrying variant

`pub struct Threat;` is a type with one value, written `Threat`. It is not `()`. `Result<(), Threat>` uses `()` as the success value and `Threat` as the error. Callers cannot confuse a refused post with a successful one that stored an empty string. Success is `Ok(())` only after `insert`.

`Claim::Woke { name, value }` is a struct variant. The fields are named. `Claim::Woke { value, .. }` in `answer` binds `value` and ignores `name`. `Claim::Idle` and `Claim::Paused` are unit variants. A `match` that names all three is exhaustive. Adding a fourth variant would fail to compile in `answer` and in `demo` until those matches learned an arm.

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

The fields are private. Callers outside the module use the methods. `facts` is a `HashMap<String, String>`. The key and the value are both owned. `waiters` is a `Vec<Waiter>` in insertion order. `claim_one` scans that vec from the front. `paused` starts false in both constructors.

`new` builds an empty map, an empty vec, and `paused: false`. `waiter_count` on that board is 0. The test `demo_names_the_waiter` asserts `Board::new().waiter_count() == 0` after it checks the demo line. `with_waiters` takes the vec by value and stores it. The map still starts empty. The flag still starts false. Passing a vec of 100 waiters does not post a fact. The bench posts after `with_waiters`.

`pause` sets the flag. `resume` clears it. `is_paused` reads it. None of these three touch `facts` or `waiters`. A fact posted before `pause` is still in the map. `claim_one` is what refuses to take it.

`has_fact` is `contains_key`. It borrows the key as `&str`. It does not remove. The condvar test loops on `while !guard.has_fact("ready")`. The claim test asserts `has_fact("ready")` is false after a successful claim, and true after the ninth post that `tick` refuses.

`post` converts the value first, then checks `is_injection`, then inserts. The key is converted only on the success path. `key.into()` does not run when the function returns `Err(Threat)`. An injection value is dropped with the local `value` binding. The map is unchanged. The doc comment says the fact is not stored. `insert` on a clean value replaces any previous value for that key and returns the old value, which this function discards. The tests post each key once.

`HashMap::new` does not allocate a fact. `Vec::new` does not allocate a waiter. `with_waiters` moves the caller's buffer in, so the vec's existing capacity stays.

### Rust: private fields and `HashMap::insert`

Privacy is per module. `facts` has no `pub`. A test in `tests/blackboard.rs` cannot write `board.facts`. It calls `post`, `has_fact`, `pause`, and `resume`. The integration test is outside `harness_core`, so it sees only the public API. `claim_one` is public and is the one function that removes a key.

`insert` returns `Option<String>`, the previous value. `post` ignores that option. The `Result` is about the injection check, not about whether the key was new. `Ok(())` means the value passed the check and was stored. `Err(Threat)` means the check failed and `insert` did not run.

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

The pause check is first. `Claim::Paused` returns before `position` runs. Waiter counters stay where they were. Facts stay in the map. The test `a_paused_board_does_not_claim` posts `notes`, pauses, and expects `tick` to return `Paused`. After `resume`, the same fact is still there, and the next `tick` is `Woke`.

`position` walks `waiters` from index 0. The predicate needs both `wakes < MAX_WAKES` and a fact whose key equals `waiter.key`. The first waiter that passes is the one this call claims. Later waiters wait for a later call. The test `a_claim_wakes_the_matching_waiter_once` puts waiter `idle` on key `other` in front of waiter `researcher` on key `ready`, and posts only `ready`. `idle` fails `contains_key`. `researcher` matches. The claim's name is `researcher` and the value is `notes`.

`let Some(index) = index else` returns `Idle` when `position` returns `None`. That happens when the vec is empty, when no fact key matches, when every matching waiter is already at 8 wakes, or when the only matching waiters are at the cap. `Idle` does not remove a fact. The wake-cap test posts `notes` a ninth time, gets `Idle`, and `has_fact("ready")` is still true.

On a match, `wakes` increments before the clones. The increment is stored on the waiter in the vec. The name and the key are cloned so the function can mutably borrow `self.facts` afterward. `remove` takes the value out of the map. The doc comment says the fact leaves with the claim. `unwrap_or_default` yields an empty `String` if `remove` returns `None`. The predicate already required `contains_key` for that same key, on this same `&mut self`, so the key is present when `remove` runs. The `Woke` value is that removed string.

`Default` calls `new`. An empty board, a zero waiter count, `paused` false. `demo` does not use `Board::default`. It uses `with_waiters`.

### Rust: `let-else` and `Iterator::position`

`let Some(index) = index else { return Claim::Idle; }` is a let-else. The `else` arm must diverge. `return` diverges. After the statement, `index` is a `usize`, not an `Option`. The next line indexes `self.waiters[index]`.

`position` takes a predicate and returns `Option<usize>`. The closure borrows each `Waiter` and also reads `self.facts`. The closure and the later `&mut self.waiters[index]` are sequential. The `iter()` borrow ends when `position` returns. That is why the mutable borrow of the chosen waiter is legal on the next line. `wakes += 1` uses that mutable borrow. The clones end it before `remove`.

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

`Shared` is the pair a waiting thread needs. `board` is public. `cv` is public. The condvar test locks `worker.board` itself and calls `claim_one` on the guard. It does not call `tick`. `tick` takes a `&Mutex<Board>` and is the path `demo`, the bench, and the other tests use. `Shared::new` wraps `Board::with_waiters`. The condvar starts with no waiters registered. `Condvar::new` does not know about the vec of `Waiter` values. Those records are inside the board.

`Shared::post` locks, calls `Board::post`, notifies, and returns `Ok(())`. The `?` is on `board.post`. An injection returns `Err(Threat)` before `notify_all`. No waiter is woken for a fact that was refused, and the fact was not inserted. `lock().expect("blackboard")` panics if the mutex is poisoned. The string is `blackboard`. A panic inside another holder poisons the mutex. This function does not clear the poison.

`notify_all` wakes every thread currently in `cv.wait`. The test spawns one thread. A second waiter blocked in `wait` would also wake, recheck `has_fact`, and either claim or wait again. This file does not spawn that second thread. The test posts once.

`tick` locks and calls `claim_one`. The doc comment says the guard ends before this function returns. The local `board` is a `MutexGuard`. It drops at the end of `tick`, after `claim_one` has built the `Claim`. The caller of `tick` receives the claim and does not hold the guard. `answer` is a different function. `demo` calls `tick` and then `answer`.

`is_injection` lowercases with `to_ascii_lowercase` and asks whether any of the four strings is a substring. The phrases are the four alternations from the Hermes pattern, each glued between `ignore ` and ` instructions` with a single space and no filler. `any` stops at the first hit. The test string `Please IGNORE previous instructions now` lowercases to a sentence that contains `ignore previous instructions`, so the function returns true. `ignore all instructions` is the value `an_injection_is_not_posted` passes to `post`, and `post` returns `Err`. The other two phrases are in the array. That test does not post them. `contains` is case-sensitive on the already-lowercased string, so the needles are stored in lowercase. A value of `ignore previous instructions` matches. A value of `ignore the previous instructions` does not, because the substring `ignore previous instructions` is not in it. The Hermes regex would allow the word `the` inside `_FILLER`. This function does not. The doc comment says filler words are not matched.

### Rust: `MutexGuard`, `?`, and `Condvar::notify_all`

`lock` returns `LockResult<MutexGuard<Board>>`. `expect` turns the poison error into a panic. The guard dereferences to `Board`, so `board.post` and `board.claim_one` are method calls on the board. The guard unlocks when it is dropped. In `tick` the drop is the end of the function. The returned `Claim` does not contain the guard. The caller cannot accidentally keep the lock by holding the claim.

`?` on `Result<(), Threat>` returns the error from `Shared::post` and drops `board` as the function unwinds that stack frame. `notify_all` is sequenced after `?`, so it runs only on `Ok`. `notify_all` does not take the fact as an argument. Waiters wake and look at the map themselves. `wait` is called in the test, not in this file. The library method `Condvar::wait` atomically unlocks and sleeps, then relocks on wake. The test assigns the returned guard back: `guard = worker.cv.wait(guard).unwrap()`.

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

`FakeModel` holds a static reply. `complete` reads `prompt` and discards it with `let _ = prompt`. The returned string is `self.reply.to_string()`. The bench does not construct this type. `descriptions.json` says `FakeModel` is not constructed in the sample. The tests that want a reply construct `FakeModel { reply: "ok" }`.

`answer` takes the mutex, a borrowed claim, and a borrowed model. The first line is `try_lock`. Success means this thread acquired the mutex, which means no other guard was alive, including a guard left over from `tick`. The type ascription `MutexGuard<'_, Board>` names the guard. `drop(_free)` unlocks before the `match`. `complete` then runs with the mutex free. The doc comment says the model runs only after `tick` has returned, so the lock is not held. `try_lock` failing hits `expect("lock held across the model")` and panics. That panic is the test that a holder still inside `tick` cannot be the caller of `answer`. `the_model_runs_after_the_lock_is_released` calls `tick` and then `answer` and expects `ok`. It does not hold a guard across the call.

The `match` passes the fact value into `complete` only for `Woke`. `Idle` and `Paused` pass an empty prompt. `complete` ignores the prompt either way and returns the reply. `answer` still branches, so a later model that read its prompt would see the fact only on `Woke`.

`demo` builds a mutex around one waiter, name `researcher`, key `ready`. It locks, posts `ready` / `notes`, and `expect("clean fact")` panics if `post` returns `Threat`. `notes` contains none of the four phrases. `tick` claims. The claim is `Woke` with that name and that value, the fact is gone, and the waiter's `wakes` is 1. `answer` try-locks, drops the guard, and completes. The reply is `ok`. The name match reads `name.as_str()` from the `Woke` arm. `format!("blackboard: {name} {reply}")` is the return value `blackboard: researcher ok`.

`chapter_demo.rs` prints that string:

```rust
use harness_core::blackboard::demo;

fn main() {
    println!("{}", demo());
}
```

`cargo run` needs `--bin ch45-blackboard`. The example binary prints eval lines. No key is read. `FakeModel::complete` ignores the prompt. This type is `harness_core::blackboard::FakeModel`. The `acp` module in the same crate has its own `FakeModel`. They are different types.

### Rust: `try_lock` and implicit format arguments

`try_lock` returns immediately. `lock` would wait. `answer` uses `try_lock` so a held mutex is a panic, not a stall behind the model. The guard's name is `_free`. The underscore prefix keeps the unused-binding warning quiet after the type has done its job. `drop(_free)` is explicit so the unlock happens before `complete`, even though the guard would also drop at the end of the function. Without that `drop`, the guard would live across `complete` and the function would hold the lock for the call it exists to keep unlocked.

`format!("blackboard: {name} {reply}")` captures the local variables by name. The colon and the spaces are literal. `name` is `&str`. `reply` is `String`. Both implement `Display`. The format string does not include the fact value. The fact was the model's prompt, and the model discarded it. The printed line is the waiter name and the reply.

## 45.7 The dispatcher tick, the ESTOP, and the four phrases

`dispatch_once` in `hermes_cli/kanban_db_dispatch.py` is the tick I read. Its docstring says it runs one dispatcher tick under the board's single-writer lock, wrapping `_dispatch_once_locked` in `_dispatch_tick_lock`, so two dispatchers on one `kanban.db` do not race a write tick. The loser returns a `DispatchResult` with `skipped_locked=True` and writes nothing. The lock is keyed on the resolved database path. After the `with` block, the comment at line 2029 says the lock is released and the tick observer fires outside the critical section, because a slow subscriber must never stall a sibling dispatcher's tick. `_fire_dispatch_tick_hook` is the next statement. The model, in that design, is whatever the subscriber does, and it does that work after the lock.

This chapter's subscriber is `answer`. `tick` is the critical section. It is a `Mutex` around a `HashMap`, not a database lock, and it claims one waiter rather than spawning a process. The guard drops when `tick` returns. `answer` then `try_lock`s. A slow `complete` cannot stall the next `tick`, because `answer` does not hold the mutex. The line 2348 comment says the same thing again, above `_dispatch_once_locked`: the lock has been released, and a slow subscriber must not extend the hold. I am treating that comment as the statement of the rule. The hook call I can point at is line 2031.

`agent/estop.py` is the pause. `is_engaged` returns true if any candidate sentinel exists. A stat error sets `saw_stat_error` and the function returns that flag at the end, so an `OSError` from `path.exists` fails safe as engaged. `check_paused` returns true when `is_engaged` is true. The module docstring says in-flight work is never killed. `Board::pause` is a bool on the struct, not a sentinel file, and it does not fail safe on a stat error because it does not stat. `claim_one` sees the flag and returns `Paused` without incrementing `wakes` and without `remove`. A claim value the caller already holds is in-flight in the same sense: `answer` will still call `complete` on it. `resume` clears the flag. The next `tick` can claim. Hermes re-arms the log line by discarding the component from `_logged_components` when the sentinel is gone. This board has no log line. `resume` only writes `false`.

`tools/threat_patterns.py` line 27 matches `ignore`, then bounded filler, then `previous|all|above|prior`, then filler, then `instructions`. The pattern id is `prompt_injection` and the scope is `all`. Line 8 says the filler is bounded. `_FILLER` is `(?:\w+\s+){0,8}`, zero to eight words. `is_injection` keeps the four words and drops the filler. The check is `str::contains` of the whole phrase. `post` refuses those four, case-insensitive, and returns before `insert`. `Shared::post` then skips `notify_all` because of `?`. A refused fact cannot wake the condvar test's thread.

`MAX_WAKES` is 8. I did not find that constant in `estop.py`, `threat_patterns.py`, or the dispatcher comment I quoted. The wake cap is this file's limit so one waiter cannot be chosen forever while its key keeps being posted. On the ninth attempt `position` fails the `wakes < MAX_WAKES` clause, `tick` returns `Idle`, and `remove` does not run. The fact stays for a waiter that is not at the cap. The tests never add that second waiter. They only assert the fact is still there.

`INBOX` from chapter 44 is still in this crate, in `acp.rs` and in `inbox.rs`. `blackboard.rs` does not submit a mail. `demo` does not call `translate`. The copied ACP tests still build. The blackboard tests do not call them.

Count `a_claim_wakes_the_matching_waiter_once`. Two waiters. One post of `notes` under `ready`. First `tick` equals `Woke { name: "researcher", value: "notes" }`. `has_fact("ready")` is false. Second `tick` matches `Idle`. The fact left with the first claim. There is nothing left to wake on.

Count `a_waiter_stops_after_the_wake_cap`. The loop is `for _ in 0..MAX_WAKES`. `MAX_WAKES` is 8, so the range is 0 through 7, eight posts and eight `Woke` claims. Each claim removes the fact, which is why the loop posts again. After the loop, `wakes` is 8. One more `post` of `notes` stores the fact. `tick` returns `Idle`. `has_fact("ready")` is true. The ninth fact is the one the cap left on the board.

Count the bench setup, which is the milestone's shape. `for n in 0..99` pushes `w0` through `w98` on keys `k0` through `k98`. One more `Waiter::new("chosen", "ready")` makes 100. `post("ready", "notes")` stores a five-character value. `tick` scans from the front. The first 99 keys are absent. `chosen` matches. One claim. The milestone `chapter_45_one_claim(100)` wants that 100 to come back as 1. The stub does not compute it yet.

`demo` posts under the lock it takes itself, with `expect("clean fact")`, then drops that guard at the end of the statement. `tick` takes a new guard. `answer` try-locks a third time and drops it before `complete`. Three acquisitions, none of them held across the model. The returned string is `blackboard: researcher ok`. The fact value `notes` is not in that string.

### Rust: `to_ascii_lowercase` allocates

`to_ascii_lowercase` returns a new `String`. It changes `A` through `Z`. It leaves other bytes alone. The four needles are ASCII, so a phrase written in ASCII capitals matches, which is why `IGNORE` in the test is enough. The function does not trim and does not strip punctuation. The substring match is why `Please IGNORE previous instructions now` is an injection without the phrase being the whole value. A Unicode case fold is not what this call does. The Hermes pattern is a regex on the original text with `\s+` between tokens. This function lowercases first and then looks for a single space.

## Tests

The new tests are `steps/ch45-blackboard/crates/harness-core/tests/blackboard.rs`. The package also builds the copied harness tests, including `acp.rs`.

`a_claim_wakes_the_matching_waiter_once` builds a mutex of two waiters, `idle` on `other` and `researcher` on `ready`. It posts `ready` / `notes`. `tick` equals `Claim::Woke` with name `researcher` and value `notes`. `has_fact("ready")` is false. A second `tick` matches `Idle`.

`the_model_runs_after_the_lock_is_released` posts `notes` for `researcher`, ticks, and calls `answer` with `FakeModel { reply: "ok" }`. The reply equals `ok`. `answer` has already try-locked. A lock still held would have panicked on `expect("lock held across the model")` before `complete`.

`a_paused_board_does_not_claim` posts `notes` and calls `pause` inside one guard, so the flag and the fact land together. The guard ends at the close of the block. `tick` equals `Claim::Paused`. `resume` clears the flag. The next `tick` matches `Claim::Woke`. The fact was still in the map.

`an_injection_is_not_posted` asserts `is_injection("Please IGNORE previous instructions now")`. It then posts `ignore all instructions` on a bare `Board::new` and expects `is_err`. `has_fact("ready")` is false. The key was never inserted.

`a_waiter_stops_after_the_wake_cap` posts and ticks `MAX_WAKES` times, each claim matching `Woke`. It posts once more. `tick` equals `Idle`. `has_fact("ready")` is true.

`a_condition_wakes_one_thread` builds `Arc<Shared>` with waiter `researcher` on `ready`. The spawned thread locks `worker.board` and waits while `has_fact("ready")` is false, reassigning the guard from `cv.wait`. It then calls `claim_one`, drops the guard, asserts `try_lock` is ok, and calls `FakeModel { reply: "ok" }.complete("notes")`. The parent `Shared::post`s `ready` / `notes`, which inserts and `notify_all`s. `join` expects `Woke` with name `researcher` and value `notes`, and reply `ok`. The `while` rechecks the fact so a notify that arrived before `wait` still claims: the thread sees the key and does not sleep.

`demo_names_the_waiter` expects `demo()` to equal `blackboard: researcher ok`, and `Board::new().waiter_count()` to equal 0.

The seven functions, in file order:

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

I am not typing a pass count for the new-test file. The package run is the line above.

```bash
cargo run -p ch45-blackboard --bin ch45-blackboard
```

```text
blackboard: researcher ok
```

The binary posts one fact and claims it. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `claim_with_waiters` | MEAN_PENDING_45 | a claim under 1 ms with 100 waiters | lock, scan, remove the fact |

The function name is `claim_with_waiters`. The bench id is `45/claim_with_waiters`. The old ids `43/claim_with_waiters` and `41/claim_with_waiters` are aliases in `benchmarks/descriptions.json`. The function name did not change when the id moved.

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

`iter_batched` runs the setup closure outside the timer. That closure builds waiters `w0` through `w98` on keys `k0` through `k98`, then `chosen` on `ready`, posts `notes`, wraps the board in a `Mutex`, and is not timed. The timed routine calls `tick` with that mutex passed through `black_box`, then `black_box` of the `Claim`. The lock, the scan of 100 waiters, the wake increment, the name and key clones, and the removal of the fact are inside the sample. Building the waiters is excluded. `FakeModel`, `answer`, `pause`, and the `Condvar` are not called. The descriptions file states those timing facts in the `timing` field of `45/claim_with_waiters`. The `inputs` field says 100 waiters, the matching key `ready` last, the fact value `notes` at 5 characters, `MAX_WAKES` at 8, and the board not paused. The `fake` field says the waiters are records, not threads, `FakeModel` is not constructed, and there is no condvar wait and no sleep. The `why` field says a claim has to stay under a millisecond with 100 waiters, and the lock must not be held across a model call. This sample is the claim. The model is not in it.

The mean is MEAN_PENDING_45. It is `mean.point_estimate` from this PR's Benchmarks workflow. The id is `45/claim_with_waiters`. The aliases are `43/claim_with_waiters` and `41/claim_with_waiters`. `.github/workflows/benchmarks.yml` runs `bash .github/scripts/run_bench_section.sh 45 "chapter 45 blackboard" ch45-blackboard blackboard`. That script runs `cargo bench -p ch45-blackboard --bench blackboard --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`. The same flags are in `steps/ch45-blackboard/README.md`. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `blackboard: researcher ok`.

### Rust: `black_box` and `BatchSize::SmallInput`

`black_box(&board)` keeps the compiler from proving the mutex is unused and deleting `tick`. `black_box(claim)` keeps it from deleting the call once it sees that nobody reads the `Claim`. `BatchSize::SmallInput` tells Criterion the setup value is small enough to clone across a batch of iterations. The setup value here is a `Mutex<Board>`. Each timed iteration receives its own mutex from a setup call. The next iteration does not see the removed fact. `Submit`-style fullness does not apply. Every sample starts from a board that still holds `notes`.

## Pitfalls

The bench is the claim. A number that spawns 100 threads and waits on a condvar measures `a_condition_wakes_one_thread`, not `claim_with_waiters`. The descriptions file says the waiters are records. `Condvar` is not in the sample. `FakeModel` is not in the sample.

The lock has to end before the model. `answer` panics if `try_lock` fails. Holding a guard across `complete` would fail that call, and it would make every other `tick` wait until the model returned. `drop(_free)` runs before the match so `answer` itself does not hold the guard across `complete` either.

`post` of an injection returns `Threat` and leaves the map unchanged. `Shared::post` does not `notify_all` on that error. A later claim cannot wake on a fact that was refused. Filler words that the Hermes regex allows are stored here. `ignore the previous instructions` does not contain the exact needle.

Eight wakes is the cap. The ninth `post` still stores the fact. `tick` leaves it there and returns `Idle`. `MAX_WAKES` is not a Hermes constant I found in the files I quoted.

`claim_one` picks the first matching waiter. A second waiter on the same key is not claimed in that call. The fact is already removed, so the second waiter needs a new `post`.

`Paused` does not remove the fact and does not increment `wakes`. `resume` is what makes the next `tick` able to claim it. `answer` on a `Paused` claim still calls `complete("")`. `demo` does not pass a paused claim.

The board is a `HashMap` in a `Mutex`. A crash drops it. Two processes do not share it. That durability is chapter 46.

`demo` returns the waiter name and the reply. It does not return the fact. `examples/demo.rs` does not call `blackboard::demo`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The catalog row for the shared threat patterns is F1, `tools/threat_patterns.py`. The catalog row for the emergency stop is N2, `agent/estop.py`. This module does not call `scan_for_threats` or `is_engaged`. The behaviors below are the ones `blackboard.rs` implements.

**Injection refusal at post.** `is_injection` matches four exact phrases, case-insensitive. `Board::post` returns `Err(Threat)` and does not `insert`. The test `an_injection_is_not_posted` checks the `IGNORE previous` sentence and the `ignore all instructions` post.

**Pause claims nothing.** `claim_one` returns `Claim::Paused` when the flag is set. The waiter is not incremented. The fact stays. `a_paused_board_does_not_claim` covers that return, then `resume`.

**Lock released before the model.** `tick` drops its guard on return. `answer` `try_lock`s and panics with `lock held across the model` if the mutex is held. It `drop`s that guard before `FakeModel::complete`. `the_model_runs_after_the_lock_is_released` and the condvar test both `try_lock` around the model.

**Wake cap.** `MAX_WAKES` is 8. The ninth `tick` of the same waiter returns `Idle` and leaves the fact on the board. `a_waiter_stops_after_the_wake_cap` asserts `has_fact("ready")` after that `Idle`.

**E1 and E2.** `blackboard.rs` does not call `redact_sensitive_text` or `redact_for_egress`. A fact value is stored as posted and handed to `complete` as posted. E1 and E2 are not called by this module.

## What you implement

`my-harness/tests/ch45.rs` is the milestone.

```rust
use my_harness::chapter_45_one_claim;

#[test]
fn one_hundred_waiters_one_claim() {
    assert_eq!(chapter_45_one_claim(100), 1);
}
```

The stub panics.

```rust
pub fn chapter_45_one_claim(waiters: usize) -> usize {
    let _ = waiters;
    todo!("chapter 45")
}
```

`todo!("chapter 45")` is the panic. The test passes `100` and wants `1` back: one hundred waiters, one claim. In the step crate that is the bench shape. Ninety-nine waiters on other keys, one waiter on `ready`, one `post` of `notes`, one `tick` that returns a single `Woke`. The stub stays `todo!`. Do not paste `blackboard.rs` into `my-harness/`.

## Chapter summary

- **Board.** Facts in a `HashMap`. Waiters in a `Vec`. One `Mutex`.
- **Scheduler.** `tick` claims one waiter and drops the guard before it returns.
- **Model.** `answer` try-locks, drops that guard, then `FakeModel` returns `ok`. The demo line is `blackboard: researcher ok`.
- **Pause.** `Claim::Paused`. The fact stays. A claim already in hand still reaches `answer`.
- **Injection.** Four exact phrases. `post` does not store them.
- **Cap.** Eight wakes. The next fact stays on the board.
- **Time.** `claim_with_waiters` mean MEAN_PENDING_45. That is `mean.point_estimate` from this PR's Benchmarks workflow. The id is `45/claim_with_waiters`. The aliases are `43/claim_with_waiters` and `41/claim_with_waiters`. The Criterion `time:` line is the slope.

[personal note: a guard that ended in the right function]

## What's next

Chapter 46 is the kanban board. One lock file. One winner.
