# Chapter 46: The Kanban Board

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 45's board works inside one process. Two processes that both want the same task will both think they own it, because a mutex does not cross a process boundary. A team of agent processes needs a board on disk, and a claim that one of them wins. This chapter adds that board. The crate is `steps/ch46-kanban`, a copy of chapter 45 plus `kanban`. The package is `ch46-kanban`. The claim binary is `ch46_kanban_claim`, not the chapter demo binary. Cargo turns hyphens into underscores in the environment variable, and a shared binary name would overwrite `target/debug`.

`Task` and `Board` are the JSON document. `Op` is claim, complete, review, or block. `apply` checks `parents_done` before a task can be claimed, so a dependent task waits. `transact` locks, reads, checks status and owner, writes, and unlocks. The lock is a sibling file opened with `create_new`. There is no version integer. Two claims of one task: one creates the lock and writes, the other gets `Busy` while the lock exists or `Lost` once the status is no longer `ready`. `Unlock` is the guard that releases it. Paths stay inside the board file the test created. `FakeModel` and the demo run one claim. Hermes's SQLite board is a compare-and-swap on status. This chapter's board is the JSON file and the lock. Pause is a field on the JSON. It is not the process-wide emergency stop.

Chapter 47 adds a message bus on this harness. The kanban board stays the durable task list. The bus is how agents talk. They are different couplings: a card has an owner and a status, a message has a topic and a subscriber. Chapter 48's team skills are proposed on the bus, not stored as kanban cards. A skill is a procedure. A card is a unit of work.

The durable claim is what lets a team keep learning across crashes. A process that dies mid-task does not take the task with it. Another process claims it. The work product can still be reviewed, and reflection on the parent can see that the task finished, once. A claim that both processes think they won would run the task twice and might write two skills for one lesson. The transaction is the reason that does not happen.

Rust, at the file: the board is serialized JSON, and the write is a temp file plus a rename so a reader never sees a half board. The lock is a sibling file, not a version field. `TxError` is the conflict. The two-process test spawns the claim binary and checks the exit.

`cargo run -p ch46-kanban --bin ch46-kanban` runs the in-process demo. The exercise is `my-harness/tests/ch46.rs`: two claims of one task, one wins.

**In this chapter**

- 46.1 `Task` and `Board`
- 46.2 `Op` and `TxError`
- 46.3 `Unlock`, paths, `write_board`, and `read_board`
- 46.4 `acquire`
- 46.5 `parents_done` and `apply`
- 46.6 `transact` and `task_status`
- 46.7 `FakeModel`, `demo`, and the claim binary
- 46.8 The SQLite compare-and-swap

| Section | What it walks |
|---|---|
| 46.1 | `Task` and `Board` |
| 46.2 | `Op` and `TxError` |
| 46.3 | `Unlock`, paths, `write_board`, `read_board` |
| 46.4 | `acquire` |
| 46.5 | `parents_done` and `apply` |
| 46.6 | `transact` and `task_status` |
| 46.7 | `FakeModel`, `demo`, and the claim binary |
| 46.8 | the SQLite compare-and-swap |

> **How Hermes does it**
>
> I looked at the kanban database. On Hermes `main` at `9e01b44`, `hermes_cli/kanban_db.py` opens with this module docstring:
>
> ```python
> """SQLite-backed Kanban board shared across profiles (the cross-profile coordination primitive).
>
> Lives under the shared Hermes root: ``default`` board DB at ``<root>/kanban.db`` (pre-boards
> back-compat), other boards at ``<root>/kanban/boards/<slug>/``; a worker on one board never sees
> another. Board resolution: ``board=`` arg > ``HERMES_KANBAN_BOARD`` > ``HERMES_KANBAN_DB`` (pins the
> file path) > ``<root>/kanban/current`` > ``default`` — but only for unfenced callers; the dispatcher
> injects these into workers, and dispatched workers (``HERMES_KANBAN_TASK``), delegated children and
> board-enumerating machine flows (gateway notifier/watcher/dispatcher, ``pin_first_board_resolution``)
> always resolve through the pin, so workers physically cannot see other boards.
> Concurrency: WAL + ``BEGIN IMMEDIATE`` + compare-and-swap on ``tasks.status``/``claim_lock`` —
> SQLite serializes writers so one claimer wins, losers see zero rows (no retries, no distributed
> locks). Schema: tasks, task_links, task_comments, task_events, task_runs, attachments, notify subs.
> """
> ```
>
> WAL plus `BEGIN IMMEDIATE` plus a compare-and-swap on `tasks.status` and `claim_lock`. SQLite serializes writers. One claimer wins. Losers see zero rows.
>
> `_claim_and_open_run` at line 2143 is that swap. The docstring says it does CAS from `source_status` to `running`, opens a run row, emits `claimed`, and returns `None` when the CAS lost. The caller holds the transaction. The update I read:
>
> ```python
> def _claim_and_open_run(
>     conn: sqlite3.Connection, task_id: str, source_status: str, lock: str, expires: int, now: int,
>     *, event_extra: Optional[dict] = None,
> ) -> Optional[int]:
>     """CAS ``source_status -> running``, open a run row, emit ``claimed``; None
>     when the CAS lost. Caller holds the txn."""
>     cur = conn.execute(
>         f"""
>         UPDATE tasks
>            SET status        = 'running',
>                claim_lock    = ?,
>                claim_expires = ?,
>                started_at    = COALESCE(started_at, ?)
>          WHERE id = ?
>            AND status = '{source_status}'
>            AND claim_lock IS NULL
>         """,
>         (lock, expires, now, task_id),
>     )
>     if cur.rowcount != 1:
>         return None
> ```
>
> The `WHERE` clause requires the id, the source status, and `claim_lock IS NULL`. `rowcount != 1` returns `None`. That `None` is the loser. `claim_task` in the same file calls this with source status `ready`. Before the call, if `_parents_satisfied` is false, `claim_task` demotes a `ready` card to `todo`, appends a `claim_rejected` event, and returns `None`.
>
> **Adopted.** One lock file via `OpenOptions` `create_new`. Compare-and-swap in `apply`: status must be `ready` and owner `None`, else `Lost`. An empty result is `TxError::Empty`. A stranger owner loses. Parents must be `done`. Pause blocks `Claim` and still allows `Complete`.
>
> **Adapted.** JSON file plus `create_new` lock, not SQLite. Windows `PermissionDenied` is a retry. The comment in `acquire` says so. 2000 tries, 100 microseconds, then `Busy`. `Unlock`'s `Drop` removes the lock file.
>
> **Deferred.** WAL, `task_runs`, and claim expiry. The model runs after the lock file is gone. `demo` asserts `!lock_path.exists()` before `FakeModel`.

> **Guardrails.** Two owners cannot both win the compare-and-swap. An empty result is refused. Pause blocks a new claim. Parents gate the claim. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

[personal note: a second process that had to lose]

## What this chapter decides

`transact` creates `board.lock` with `create_new`, reads `board.json`, applies one op, writes the file back, and deletes the lock in `Drop`. `acquire` closes the file handle before it returns the `Unlock`. The delete runs when `transact` returns, including when a later step returns an error, because `_lock` drops on the way out.

`Claim` requires status `ready`, owner `None`, parents `done`, and a board that is not paused. It sets `running` and the owner. A card that is not `ready`, or that already has an owner, returns `Lost`.

`Complete` requires `running`, the same owner, and a non-empty result. `trim` decides empty. Whitespace is `Empty`. The success path sets `done`, stores the result, and clears the owner. `Review` requires the owner and a non-empty summary. It sets `review` and clears the owner. `Block` does the same on the way to `blocked`, with the reason stored in `result`.

A second claim of a `running` card returns `Lost`. Two processes are the binary `ch46_kanban_claim`. One prints `claimed`. The other prints `lost`. `Parents`, `Paused`, and `Busy` also print `lost` from that binary. `Empty`, `Missing`, and `Io` print the debug form on stderr and exit 1.

`demo` uses a temp directory, claims `t1` for `worker`, checks the lock file is gone, runs `FakeModel`, completes with result `shipped`, and returns `kanban: done`.

## 46.1 `Task` and `Board`

`kanban.rs` is the new module. `blackboard.rs` is the chapter 45 copy. `pub mod kanban` is what `chapter_demo.rs` and `kanban_claim.rs` import.

```rust
//! Book chapter 46. One JSON board. A lock file is the transaction.
//! The status update is a compare-and-swap: a second owner loses.

use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub parents: Vec<String>,
}

impl Task {
    pub fn ready(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: "ready".into(),
            owner: None,
            result: None,
            parents: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board {
    #[serde(default)]
    pub paused: bool,
    pub tasks: Vec<Task>,
}

impl Board {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            paused: false,
            tasks,
        }
    }
}
```

`Task` is one card. `id` and `status` are required strings. `owner` and `result` are optional. `parents` is a vec of parent ids. `serde(default)` on those three means a missing JSON field deserializes as `None`, `None`, and an empty vec. A field that is present still deserializes as written. `ready` builds a card whose status is the string `ready`, whose owner and result are `None`, and whose parents vec is empty. The id comes from `Into<String>`, so `Task::ready("t1")` stores `t1`. Status is a `String`, not an enum. `apply` compares it to `"ready"`, `"running"`, and the other literals with `!=`.

`Board` is the file. `paused` defaults to false when the JSON omits it. `tasks` is a vec in file order. `new` sets `paused` false and stores the vec. There is no `Default` impl in this file. Callers pass the tasks they want. `demo` passes `vec![Task::ready("t1")]`.

The derives include `Serialize` and `Deserialize`. `write_board` and `read_board` are the only functions that use them. `PartialEq` is why a test could compare boards. The tests compare status strings and `TxError` values instead.

### Rust: `#[serde(default)]` on `Option` and `Vec`

`Option<String>` already deserializes from JSON `null` as `None` and from a string as `Some`. `default` covers the other case: the key is absent. Serde then uses `Default`, and `Option`'s default is `None`. `Vec<String>`'s default is an empty vec. `bool`'s default is `false`. Without `default`, a missing `owner` key would fail `from_slice` and `read_board` would return `TxError::Io`. The tests write boards through `write_board`, which always emits the fields, so the default is what a hand-edited file can rely on. This chapter's tests do not omit fields.

## 46.2 `Op` and `TxError`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Claim { owner: String },
    Complete { owner: String, result: String },
    Review { owner: String, summary: String },
    Block { owner: String, reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError {
    Lost,
    Empty,
    Parents,
    Paused,
    Busy,
    Missing,
    Io,
}
```

`Op` is the one verb `transact` applies. Each variant carries the owner. `Complete` carries `result`. `Review` carries `summary`. `Block` carries `reason`. Those three strings are the evidence `apply` trims. `Claim` has no evidence string. `transact` takes `Op` by value. The caller moves the owner string in. `apply` borrows the op as `&Op` and clones the owner into the task only on the success path of `Claim`.

`TxError` is every refusal and every IO failure this file returns. There is no `Display` impl and no `thiserror` derive in this file. The claim binary prints `{error:?}` on the variants it does not map to `lost`, so the stderr line is the `Debug` form. `Lost` is the compare-and-swap failure: wrong status, or an owner that does not match, or an owner already set. `Empty` is blank evidence. `Parents` is a parent that is not `done`. `Paused` is a claim against a paused board. `Busy` is the lock retry giving up. `Missing` is an id that is not in `tasks`. `Io` is every filesystem and JSON error this file maps. The variants are unit variants. They carry no path and no errno. `PartialEq` is why the tests write `Err(TxError::Empty)`.

### Rust: struct variants and a closed error set

`Op::Claim { owner: "worker".into() }` names the field at the call site. A match later binds `owner` by name. Adding a fifth op would fail to compile in `apply` until a new arm existed. `TxError` is the same kind of closed set. `transact` returns `Result<String, TxError>`. The `String` is the status after a successful write. The error means the file was left as it was, except for `Io` from `write_board` after `apply` mutated memory: that mutation is dropped with the local `board`, and the lock file is removed by `Drop`. The JSON on disk is whatever `write_board` last successfully renamed. A failed `write_board` returns before a successful rename completes, or returns the error from the second rename. The in-memory board is not what a later `read_board` sees unless the rename finished.

## 46.3 `Unlock`, paths, `write_board`, and `read_board`

```rust
struct Unlock {
    path: PathBuf,
}

impl Drop for Unlock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn board_path(dir: &Path) -> PathBuf {
    dir.join("board.json")
}

fn lock_path(dir: &Path) -> PathBuf {
    dir.join("board.lock")
}

pub fn write_board(dir: &Path, board: &Board) -> Result<(), TxError> {
    fs::create_dir_all(dir).map_err(|_| TxError::Io)?;
    let bytes = serde_json::to_vec(board).map_err(|_| TxError::Io)?;
    let tmp = dir.join("board.json.tmp");
    fs::write(&tmp, bytes).map_err(|_| TxError::Io)?;
    let dest = board_path(dir);
    if fs::rename(&tmp, &dest).is_err() {
        let _ = fs::remove_file(&dest);
        fs::rename(&tmp, &dest).map_err(|_| TxError::Io)?;
    }
    Ok(())
}

fn read_board(dir: &Path) -> Result<Board, TxError> {
    let bytes = fs::read(board_path(dir)).map_err(|_| TxError::Io)?;
    serde_json::from_slice(&bytes).map_err(|_| TxError::Io)
}
```

`Unlock` is private. Its only field is the lock path. `Drop` calls `fs::remove_file` and ignores the error with `let _`. A missing lock file at drop time does not panic and does not change the `Result` `transact` is about to return. The remove is the end of the transaction. The user-facing comment on `transact` says the lock file is gone before `transact` returns. That is this `Drop`, sequenced as the local `_lock` goes out of scope.

`board_path` joins `board.json`. `lock_path` joins `board.lock`. Both are pure. They do not create anything. `demo` and the model test both look at `board.lock` after `transact`. `task_status` reads `board.json` through `read_board`.

`write_board` is public because the tests and the bench build the starting file with it. `create_dir_all` makes the temp directory. `to_vec` serializes the board. The bytes go to `board.json.tmp`. `rename` moves that temp onto `board.json`. If the rename fails, the function removes `board.json` and renames again. The second failure is `TxError::Io`. The first failure is not returned on its own. `read_board` is private. It reads `board.json` and deserializes. Either a missing file or a bad payload is `Io`. Callers that want a status without taking the lock use `task_status`, which calls `read_board`.

`map_err(|_| TxError::Io)` drops the underlying error. A permission error and a JSON syntax error look the same to the caller. The claim binary then prints `Io` if it has not already mapped the variant to `lost`.

### Rust: `Drop` as the unlock

`Drop::drop` runs when the value is dropped, including on `?` early returns after the value exists. `transact` binds `let _lock = acquire(...)?`. The `?` on `acquire` means a failed acquire never builds an `Unlock`, so `Drop` does not run, and there is no lock file from this call to remove. `create_new` failed, or the retry loop returned `Busy`. After `Ok(Unlock)`, every later `?` in `transact` drops `_lock` first. The lock file is removed on `Lost`, `Empty`, `Parents`, `Paused`, `Missing`, and `Io` from the read or the write, the same way it is removed on success. The underscore on `_lock` means the binding is otherwise unused. The destructor is the use.

`Path::join` returns a new `PathBuf`. `dir` is borrowed. `board_path` and `lock_path` do not store `dir`. A caller can pass a temporary `PathBuf` and keep using the returned path after the call.

## 46.4 `acquire`

```rust
fn acquire(path: &Path) -> Result<Unlock, TxError> {
    for _ in 0..2_000 {
        match OpenOptions::new().write(true).create_new(true).open(path) {
            Ok(file) => {
                drop(file);
                return Ok(Unlock {
                    path: path.to_path_buf(),
                });
            }
            Err(error)
                if error.kind() == ErrorKind::AlreadyExists
                    || error.kind() == ErrorKind::PermissionDenied =>
            {
                // Windows can report a sharing violation while the other
                // process deletes the lock file. That is a retry, not a loss.
                thread::sleep(Duration::from_micros(100));
            }
            Err(_) => return Err(TxError::Io),
        }
    }
    Err(TxError::Busy)
}
```

The loop is `0..2_000`, two thousand attempts. Each attempt opens `path` with `write(true)` and `create_new(true)`. `create_new` succeeds only when the file did not exist. That success is the winner of the lock file. `drop(file)` closes the handle before `Unlock` is returned. `Unlock` remembers the path and does not hold the `File`. The later `remove_file` in `Drop` is why the handle was closed here.

`AlreadyExists` means another process, or another call in this process, already created `board.lock`. `PermissionDenied` is the case the comment names: Windows can report a sharing violation while the other process deletes the lock file. The comment says that is a retry, not a loss. Both kinds sleep `Duration::from_micros(100)` and try again. Any other `ErrorKind` returns `Io` immediately, without sleeping and without burning the rest of the attempts.

If all 2000 attempts hit `AlreadyExists` or `PermissionDenied`, the function returns `Busy`. It does not create the file. It does not return `Lost`. `Lost` is the status compare-and-swap inside `apply`. `Busy` is the lock file. The descriptions file says the 100 microsecond retry loop runs only when the lock file already exists. An uncontended `transact`, which is the bench, takes the `Ok(file)` arm on the first open. The sleep does not run.

`acquire` is private. Tests reach it through `transact`. The two-process test is what contends. The bench is one process, and its setup writes a fresh directory, so `board.lock` is absent when `transact` starts.

### Rust: `create_new` and a match guard

`OpenOptions::create_new(true)` sets the create-new flag. `open` then fails with `AlreadyExists` if the path is there. That is the compare-and-swap on the file name. The status compare-and-swap is a second check, inside the lock, in `apply`. Winning the file is not the same as winning the card. A process can create `board.lock`, read a card that is already `running`, and return `Lost`. `Drop` still removes the lock file.

The `if` on the `Err` arm is a match guard. It runs after the pattern matches `Err(error)`. Both error kinds share one body. A guard that was false would fall through to the next arm. The next arm is `Err(_)`, which is `Io`. `Ok(file)` is first so a successful create never sleeps.

## 46.5 `parents_done` and `apply`

```rust
fn parents_done(board: &Board, task: &Task) -> bool {
    task.parents.iter().all(|parent| {
        board
            .tasks
            .iter()
            .find(|candidate| candidate.id == *parent)
            .is_some_and(|candidate| candidate.status == "done")
    })
}

fn apply(board: &mut Board, id: &str, op: &Op) -> Result<(), TxError> {
    let Some(index) = board.tasks.iter().position(|task| task.id == id) else {
        return Err(TxError::Missing);
    };
    if board.paused && matches!(op, Op::Claim { .. }) {
        return Err(TxError::Paused);
    }
    let parents_ok = parents_done(board, &board.tasks[index]);
    let task = &mut board.tasks[index];
    match op {
        Op::Claim { owner } => {
            if !parents_ok {
                return Err(TxError::Parents);
            }
            if task.status != "ready" || task.owner.is_some() {
                return Err(TxError::Lost);
            }
            task.status = "running".into();
            task.owner = Some(owner.clone());
        }
        Op::Complete { owner, result } => {
            if result.trim().is_empty() {
                return Err(TxError::Empty);
            }
            if task.status != "running" || task.owner.as_deref() != Some(owner) {
                return Err(TxError::Lost);
            }
            task.status = "done".into();
            task.result = Some(result.clone());
            task.owner = None;
        }
        Op::Review { owner, summary } => {
            if summary.trim().is_empty() {
                return Err(TxError::Empty);
            }
            if task.status != "running" || task.owner.as_deref() != Some(owner) {
                return Err(TxError::Lost);
            }
            task.status = "review".into();
            task.result = Some(summary.clone());
            task.owner = None;
        }
        Op::Block { owner, reason } => {
            if reason.trim().is_empty() {
                return Err(TxError::Empty);
            }
            if task.status != "running" || task.owner.as_deref() != Some(owner) {
                return Err(TxError::Lost);
            }
            task.status = "blocked".into();
            task.result = Some(reason.clone());
            task.owner = None;
        }
    }
    Ok(())
}
```

`parents_done` is true when every parent id is a task on this board whose status is `done`. `all` on an empty iterator is true, so a task with no parents passes. `find` locates the parent by id. `is_some_and` requires the parent to exist and to be `done`. A missing parent id fails the check. A parent in `ready`, `running`, `review`, or `blocked` fails the check. The function does not look through grandparents. It reads `task.parents` only.

`apply` finds the task index by id. No match is `Missing`. The pause check is next, and only for `Op::Claim`. `matches!(op, Op::Claim { .. })` ignores the owner. A paused board returns `Paused` before parents are considered and before the status is read for the claim. `Complete`, `Review`, and `Block` do not take this branch. A paused board still accepts those three when the rest of their checks pass. That is the split: pause blocks a new claim and still allows complete.

`parents_ok` is computed before the mutable borrow of the task. The parent walk needs to read `board.tasks` while the later `let task = &mut board.tasks[index]` borrows one element. The bool is a copy. The borrow of the slice ends when `parents_done` returns.

`Claim` returns `Parents` when `parents_ok` is false, before it writes status or owner. It returns `Lost` when status is not `ready` or owner is `Some`. Both checks are the compare-and-swap. The Hermes update required `status = source` and `claim_lock IS NULL`. Here the source is the string `ready` and the empty lock is `owner.is_none()`, written as `is_some()` in the failure condition. On success the status becomes `running` and the owner becomes `Some` of a clone of the op's owner.

`Complete` trims `result`. An empty trim is `Empty`, before the owner check. The test uses `"  "`, which trims to empty. A non-empty result then requires status `running` and `owner.as_deref() == Some(owner)`. A stranger's owner fails that equality and returns `Lost`. Success writes `done`, stores the result, and sets owner to `None`. `Review` is the same shape with `summary` and status `review`. `Block` is the same shape with `reason` and status `blocked`. All three clear the owner. A later `Claim` can take the card only from `ready`. Moving to `review` or `blocked` or `done` means a second `Claim` sees a status other than `ready` and returns `Lost`. This file has no op that moves `review` back to `ready`.

`Ok(())` means the in-memory board was updated. `transact` still has to write it. `apply` does not touch the disk.

### Rust: `is_some_and` and `as_deref`

`is_some_and` takes the `Option` from `find` and applies a predicate only on `Some`. `None` is false. The predicate sees `&Task`. `candidate.status == "done"` is a string compare. A parent whose status is `Done` with a capital D would fail. `ready` builds the lowercase spelling, and `apply` writes lowercase spellings.

`as_deref` on `Option<String>` yields `Option<&str>`. In the `Complete` arm, `op` is `&Op`, so `owner` is `&String`. `PartialEq` compares that option with `Some(owner)`. The comparison does not clone the stored owner. A mismatch is `Lost` and does not clear the owner. The stranger test relies on that: after `Err(Lost)`, `task_status` is still `running`.

## 46.6 `transact` and `task_status`

```rust
/// One transaction. The lock file is gone before this returns.
pub fn transact(dir: &Path, id: &str, op: Op) -> Result<String, TxError> {
    let _lock = acquire(&lock_path(dir))?;
    let mut board = read_board(dir)?;
    apply(&mut board, id, &op)?;
    let status = board
        .tasks
        .iter()
        .find(|task| task.id == id)
        .map(|task| task.status.clone())
        .ok_or(TxError::Missing)?;
    write_board(dir, &board)?;
    Ok(status)
}

pub fn task_status(dir: &Path, id: &str) -> Result<Option<String>, TxError> {
    let board = read_board(dir)?;
    Ok(board
        .tasks
        .iter()
        .find(|task| task.id == id)
        .map(|task| task.status.clone()))
}
```

`transact` is the public transaction. The doc comment says the lock file is gone before the function returns. The order is fixed. Acquire `board.lock`. Read `board.json`. Apply the op. Read the status string back out of the mutated board. Write the board. Return that status. `_lock` drops at the final brace, and also drops if `read_board`, `apply`, the status find, or `write_board` returns an error through `?`.

The status find after `apply` is a second lookup by id. `apply` already found the id or returned `Missing`. A successful `apply` leaves the task in the vec, so `find` succeeds. `ok_or(TxError::Missing)` is the arm if it does not. The cloned status is what the caller sees: `running`, `done`, `review`, or `blocked`, depending on the op. `Claim` of `t1` in the tests expects `running`.

`write_board` rewrites the whole file, not a single field. Every task on the board is serialized again. The bench has one task. A board with a parent and a child rewrites both, which is what a claim of the child does when it fails: it does not rewrite, because `apply` returns `Parents` before `write_board`.

`task_status` does not acquire the lock. It reads `board.json` and clones the status, or returns `Ok(None)` when the id is absent. `Ok(None)` and `Err(Missing)` are different. `transact` uses `Missing` for an id it was asked to change. `task_status` uses `None` for an id it was asked to read. A missing file is `Io` for both, because both go through `read_board`. The two-process test calls `task_status` after both children have exited, and expects `Some("running")`.

### Rust: `ok_or` and a lock that is not a data guard

`ok_or` converts `Option<String>` into `Result<String, TxError>`. `?` then returns `Missing`. The clone happens inside `map`, before `ok_or`, so the board borrow ends before `write_board` needs `&board` again. That is why the status is cloned instead of returned as a borrow. The function returns a `String` the caller owns after the board local is dropped.

The lock file is not a `MutexGuard` around the JSON. Nothing in `task_status` checks `board.lock`. A reader during `transact` can observe the previous `board.json`, because the write is a rename of `board.json.tmp` onto `board.json` at the end. A reader cannot observe a half-applied `Task` inside `apply`, because `apply` edits memory and the file changes only in `write_board`. Two `transact` calls contend on `create_new`, not on the JSON parse.

## 46.7 `FakeModel`, `demo`, and the claim binary

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

pub fn demo() -> String {
    let dir = std::env::temp_dir().join(format!("ch46-kanban-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).expect("board");
    transact(
        &dir,
        "t1",
        Op::Claim {
            owner: "worker".into(),
        },
    )
    .expect("claim");
    assert!(!lock_path(&dir).exists());
    let _ = FakeModel { reply: "ok" }.complete("t1");
    let status = transact(
        &dir,
        "t1",
        Op::Complete {
            owner: "worker".into(),
            result: "shipped".into(),
        },
    )
    .expect("complete");
    let _ = fs::remove_dir_all(&dir);
    format!("kanban: {status}")
}
```

`FakeModel` is the same shape as the blackboard fake, and it is a different type. `complete` discards the prompt and returns `reply`. The bench does not construct it. `descriptions.json` says `FakeModel` is not constructed in the sample.

`demo` builds a directory named `ch46-kanban-` plus the process id, under the temp dir. `remove_dir_all` clears a leftover from a previous run of the same process. The error is ignored. `write_board` creates the directory and writes one ready task `t1`. `transact` claims it for `worker`. `expect("claim")` panics if the claim fails. `assert!(!lock_path(&dir).exists())` runs before `FakeModel`. The lock file from the claim is already gone, because `transact` returned and `Drop` removed it. `complete("t1")` returns `ok`, and `let _` drops that string. The prompt is the task id, not a model prompt from a provider. The second `transact` completes as `worker` with result `shipped`. The status comes back `done`. `remove_dir_all` deletes the directory. `format!("kanban: {status}")` is `kanban: done`.

`chapter_demo.rs` prints that string:

```rust
use harness_core::kanban::demo;

fn main() {
    println!("{}", demo());
}
```

`cargo run` needs `--bin ch46-kanban`. The example binary prints eval lines. No key is read.

The claim binary is `src/bin/kanban_claim.rs`. The test points at it with `env!("CARGO_BIN_EXE_ch46_kanban_claim")`. Cargo sets that env var when building an integration test, from the bin name `ch46_kanban_claim`. The bin name uses underscores so it cannot collide with the demo bin `ch46-kanban`, whose env var would be `CARGO_BIN_EXE_ch46_kanban`.

```rust
use std::env;
use std::path::Path;
use std::process::ExitCode;

use harness_core::kanban::{transact, Op, TxError};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        eprintln!("usage: kanban-claim claim <dir> <id> <owner>");
        return ExitCode::from(2);
    };
    if command != "claim" {
        eprintln!("usage: kanban-claim claim <dir> <id> <owner>");
        return ExitCode::from(2);
    }
    let Some(dir) = args.next() else {
        return ExitCode::from(2);
    };
    let Some(id) = args.next() else {
        return ExitCode::from(2);
    };
    let Some(owner) = args.next() else {
        return ExitCode::from(2);
    };
    match transact(
        Path::new(&dir),
        &id,
        Op::Claim {
            owner: owner.clone(),
        },
    ) {
        Ok(_) => {
            println!("claimed");
            ExitCode::SUCCESS
        }
        Err(TxError::Lost | TxError::Parents | TxError::Paused | TxError::Busy) => {
            println!("lost");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::from(1)
        }
    }
}
```

Args are `claim`, the directory, the id, and the owner. A missing command prints the usage line and exits 2. A command other than `claim` does the same. A missing directory, id, or owner exits 2 without the usage line. `transact` of `Op::Claim` is the whole body. `Ok` prints `claimed` and exits 0. `Lost`, `Parents`, `Paused`, and `Busy` print `lost` and exit 0. The two-process test looks at stdout for those two words. It does not check the exit code. Any other error prints `Debug` to stderr and exits 1. The test message includes stderr if the stdout words are wrong, which is how a surprise `Io` shows up.

The two-process test spawns two threads. Each thread runs `Command::new` on that executable with args `claim`, the same directory, `t1`, and owner `a` or `b`. One create_new wins. That process's `apply` sees `ready` and owner `None`, writes `running`, and prints `claimed`. The other process either retries the lock file and then sees status `running` and returns `Lost`, or loses in some other `Lost` / `Busy` way. Both of those print `lost`. The test asserts the two stdout lines contain both words, in either order, and that `task_status` is `running`.

### Rust: `env!` and `ExitCode`

`env!("CARGO_BIN_EXE_ch46_kanban_claim")` is a compile-time read. The integration test binary does not look up `PATH`. Cargo fails the build if that bin was not built for the test. The string is the absolute path of the executable. `Command::new` takes it.

`ExitCode::from(2)` and `ExitCode::from(1)` are the failure codes. `ExitCode::SUCCESS` is 0. `main` returns `ExitCode`, so the process status is that value. `println!("claimed")` writes stdout with a newline. The test trims whitespace before comparing to `claimed` and `lost`.

## 46.8 The SQLite compare-and-swap

The module docstring in `hermes_cli/kanban_db.py` states the concurrency rule in one sentence. WAL, `BEGIN IMMEDIATE`, and a compare-and-swap on `tasks.status` and `claim_lock`. SQLite serializes writers. One claimer wins. Losers see zero rows. There are no retries and no distributed locks in that sentence. This crate retries the lock file. It does not retry the status swap. A `Lost` returns to the caller. The claim binary prints `lost` and exits 0. It does not loop back into `transact`.

`_claim_and_open_run` updates one row to `status = 'running'` and sets `claim_lock`, `claim_expires`, and `started_at`, and only where `id` matches, `status` is the source status, and `claim_lock IS NULL`. `rowcount != 1` returns `None` before it inserts a `task_runs` row. The winner then inserts the run, stores `current_run_id`, and appends a `claimed` event. This crate's `apply` has the status check and the empty-owner check. It does not insert a run row. It does not set an expiry. It does not append an event. Those are the deferred pieces: WAL, `task_runs`, and claim expiry.

`claim_task` calls `_claim_and_open_run` with source `ready`. Parents are checked first. `_parents_satisfied` is false when any direct parent is not in `done` or `archived`. On that failure `claim_task` updates the card from `ready` to `todo` and returns `None`. `parents_done` in this file requires status `done` only. `archived` is not a status `apply` writes. A parent that is not `done` makes `Claim` return `Parents` without changing the child. `write_board` is skipped. The child stays `ready` on disk. Hermes demotes. This crate refuses and leaves the status.

`demo` asserts the lock file is absent and then constructs `FakeModel`. The model is the subscriber that must not run under the lock. Chapter 45 did that with `try_lock` after `tick`. This chapter does it with `Drop` of `Unlock` before `complete`. The assertion is `!lock_path(&dir).exists()`. The complete op is a second transaction, after the model, with its own lock file that is also gone before `demo` formats the line.

The blackboard module is still in the crate. `kanban.rs` does not call `tick` or `post`. The chapter 45 tests are copied forward and still run. The kanban tests do not construct a `Board` from `blackboard`.

Count `two_processes_cannot_hold_the_same_task`. One directory. One ready task `t1`. Two processes. The stdout set is `claimed` and `lost`. The status is `running`. One owner is stored. The test does not read the owner field, so it does not assert whether `a` or `b` won. `apply` stored whichever owner passed the `ready` / `None` check. The other owner hit `Lost` or `Busy` and printed `lost`.

Count the bench. One uncontended claim of `t1`. The setup directory is outside the sample. `remove_dir_all` of that directory is inside the sample, after `transact`. No second process. No `FakeModel`.

### Rust: `rowcount` has no twin here

SQLite's `rowcount` is the number of rows the `UPDATE` changed. Zero means the `WHERE` missed and the loser returns `None`. `apply` does not have a row count. The miss is a boolean: status is not `ready`, or owner is `Some`. Both become `TxError::Lost`. A missing id is `Missing` instead of `Lost`, because the task was never a candidate row. Hermes would also change zero rows for a bad id, and `rowcount != 1` would return `None` for that too. This file splits those two outcomes.

## Tests

The new tests are `steps/ch46-kanban/crates/harness-core/tests/kanban.rs`. The package also builds the copied harness tests, including `blackboard.rs`.

`fresh` builds a directory under the temp dir. The name is `ch38-`, the label, the process id, and the current time in nanoseconds. It `remove_dir_all`s that path before returning it. The `ch38-` prefix is the string in the source. The bench uses `ch38-bench-` plus the process id and a counter.

`claim_complete_review_and_block_are_one_status_each` uses three directories from `fresh("verbs")`, `fresh("block")`, and `fresh("done")`. On the first, claim of `t1` by `worker` returns `running`, and review with summary `look` returns `review`. On the second, claim then block with reason `needs a human` returns `blocked`. On the third, claim then complete with result `shipped` returns `done`. The last line removes `fresh("noop")`, which is a new empty directory, not the three boards. Each successful `transact` is one status. The test compares the returned `String`, not a second read, except that the return value is the status `transact` cloned after `apply`.

`an_empty_result_and_a_stranger_lose` claims `t1` for `worker`. Complete with result `"  "` equals `Err(TxError::Empty)`. Complete with owner `other` and result `shipped` equals `Err(TxError::Lost)`. `task_status` is `Some("running")`. The empty check runs first, so the whitespace result never reaches the owner compare. The stranger passes the trim check, fails the owner compare, and does not write `done`. The owner on the card stays `worker` because `Lost` returns before the assignment. The test checks the status, not the owner field.

`a_parent_that_is_not_done_blocks_the_claim` builds a child whose `parents` vec holds `parent`, and a parent from `Task::ready("parent")`, so the parent status is `ready`. Claim of `child` by `worker` equals `Err(TxError::Parents)`. The test does not read the status back. `apply` returns before it writes, and `transact` skips `write_board` on that error, so the child stays `ready` in the file `write_board` wrote at the start.

`pause_refuses_a_new_claim_and_complete_still_runs` writes a ready `t1`, claims it, then builds a paused board in memory with `t1` still `running` and owned by `worker`, and writes that file. Claim of `t2` on that directory equals `Err(TxError::Missing)`, because `t2` is not in `tasks`. A second directory from `fresh("pause-claim")` holds a paused board whose only task is ready `t2`. Claim of `t2` by `other` equals `Err(TxError::Paused)`. The function name says complete still runs. The body does not call `Op::Complete`. The pause split is in `apply`, which returns `Paused` only for `Op::Claim`. This test locks the `Paused` return and the `Missing` return.

`two_processes_cannot_hold_the_same_task` is the race. `fresh("race")`, one ready `t1`, two threads, one executable from `env!("CARGO_BIN_EXE_ch46_kanban_claim")`, args `claim`, the directory, `t1`, and owner `a` or `b`. Stdout must contain `claimed` and `lost`. The assertion message includes both stderr strings. `task_status` is `Some("running")`.

`the_model_runs_after_the_lock_file_is_gone` claims `t1` for `worker`, asserts `board.lock` does not exist, and expects `FakeModel { reply: "ok" }.complete("t1")` to equal `ok`.

`demo_finishes_the_card` expects `demo()` to equal `kanban: done`.

The seven functions, in file order:

```rust
fn claim_complete_review_and_block_are_one_status_each()
fn an_empty_result_and_a_stranger_lose()
fn a_parent_that_is_not_done_blocks_the_claim()
fn pause_refuses_a_new_claim_and_complete_still_runs()
fn two_processes_cannot_hold_the_same_task()
fn the_model_runs_after_the_lock_file_is_gone()
fn demo_finishes_the_card()
```

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch46-kanban
```

```text
    Starting 243 tests across 52 binaries
     Summary [   0.444s] 243 tests run: 243 passed, 0 skipped
```

I am not typing a pass count for the new-test file. The package run is the line above.

```bash
cargo run -p ch46-kanban --bin ch46-kanban
```

```text
kanban: done
```

The binary writes a temp board, claims it, and completes it. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `claim_transaction` | 328.02 µs | one claim transaction under 5 ms | lock file, JSON rewrite, `remove_dir_all` |

The function name is `claim_transaction`. The bench id is `46/claim_transaction`. The old ids `44/claim_transaction` and `42/claim_transaction` are aliases in `benchmarks/descriptions.json`. The function name did not change when the id moved.

```rust
fn claim_transaction(c: &mut Criterion) {
    let mut n = 0u64;
    c.bench_function("claim_transaction", |b| {
        b.iter_batched(
            || {
                n += 1;
                let dir =
                    std::env::temp_dir().join(format!("ch38-bench-{}-{n}", std::process::id()));
                let _ = std::fs::remove_dir_all(&dir);
                write_board(&dir, &Board::new(vec![Task::ready("t1")])).expect("board");
                dir
            },
            |dir: PathBuf| {
                let status = transact(
                    black_box(&dir),
                    "t1",
                    Op::Claim {
                        owner: "worker".into(),
                    },
                )
                .expect("claim");
                let _ = std::fs::remove_dir_all(&dir);
                black_box(status);
            },
            BatchSize::SmallInput,
        );
    });
}
```

`iter_batched` runs the setup closure outside the timer. The counter `n` gives each sample its own directory, `ch38-bench-` plus the process id plus `n`. The setup removes that path, writes one ready task `t1`, and is not timed. Directory creation is that `write_board`, which calls `create_dir_all`, and it is excluded. The timed routine calls `transact` of `Op::Claim` with owner `worker`, then `remove_dir_all`, then `black_box` of the status. `create_new` of `board.lock`, the JSON read, the status swap, and the JSON write are inside the sample. `remove_dir_all` is inside the sample too. The second process is not spawned. `FakeModel` is not called. Complete, review, and block are not this sample. The 100 microsecond retry loop runs only when the lock file already exists. The descriptions file states those timing facts in the `timing` field of `46/claim_transaction`. The `inputs` field says one task, id `t1`, status `ready`, no owner, no parents, owner string `worker`, one lock file in a private temp directory. The `fake` field says the board is a JSON file in the temp directory, there is no SQLite and no worker process, and `FakeModel` is not constructed. The `why` field says claim, complete, review, or block has to be one transaction under 5 milliseconds, and two processes must not both own the task. This sample is one uncontended claim. The race is the test.

The mean is 328.02 µs. It is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). The id is `46/claim_transaction`. The aliases are `44/claim_transaction` and `42/claim_transaction`. `.github/workflows/benchmarks.yml` runs `bash .github/scripts/run_bench_section.sh 46 "chapter 46 kanban" ch46-kanban kanban`. That script runs `cargo bench -p ch46-kanban --bench kanban --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`. The same flags are in `steps/ch46-kanban/README.md`. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `kanban: done`.

### Rust: the setup counter sits outside the timer

`n` is captured mutably by the setup closure. `iter_batched` calls that closure before it starts timing the routine. The increment and the directory creation are on the untimed side. The routine takes `PathBuf` by value, so the path moves into the timed closure and the setup does not keep a second owner. `remove_dir_all` inside the routine deletes the directory the next setup will not reuse, because `n` has moved on. The `expect("claim")` panics the bench if `transact` returns an error. An uncontended claim of a ready card returns `running`.

## Pitfalls

The bench is one uncontended claim, plus `remove_dir_all` of the temp directory. A number that includes two processes and the 100 microsecond retry is `two_processes_cannot_hold_the_same_task`. The descriptions file says the retry loop runs only when the lock file already exists. The bench's setup creates a fresh directory and does not create `board.lock`.

The lock file is removed in `Drop`. A crash between `create_new` and that drop leaves `board.lock` behind. The next `acquire` retries 2000 times at 100 microseconds and returns `Busy`. This crate does not delete a stale lock based on age. Hermes stores `claim_expires` on the row inside `_claim_and_open_run`. That expiry is not here.

On Windows, `PermissionDenied` is a retry, per the comment in `acquire`. An immediate `Io` on that kind would skip the sleep. `AlreadyExists` is the other retry kind. Every other kind is `Io` on the first sighting.

Complete from a process that is not the owner is `Lost`. The card stays `running`. An empty or whitespace result is `Empty`, and that check happens before the owner check. Clearing the owner on review and on block is what the success arms do. A later claim still needs status `ready`. This file never writes `ready` except `Task::ready` at construction.

Parents gate `Claim` only. A child with a parent still in `ready` gets `Parents` and the file is not rewritten. Pause gates `Claim` only. The test `pause_refuses_a_new_claim_and_complete_still_runs` asserts `Paused` and `Missing`. It does not call `Complete`. `apply` still allows `Complete` on a paused board when the owner and the result check pass.

`task_status` does not take `board.lock`. It can read the JSON a `transact` has not replaced yet. After both processes in the race have joined, the status is `running`.

`demo` deletes its directory. The printed line is `kanban: done`. The lock-file assertion is inside `demo`, before the model, and it is not part of the printed line. `examples/demo.rs` does not call `kanban::demo`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The catalog's N2 row is the ESTOP in `agent/estop.py`. This module does not call `is_engaged`. Pause is the `paused` field on the JSON board. The behaviors below are the ones `kanban.rs` implements.

**Compare-and-swap.** `create_new` makes one lock file. `apply` for `Claim` requires status `ready` and owner `None`, and otherwise returns `Lost`. Two owners cannot both pass that check. `two_processes_cannot_hold_the_same_task` expects one `claimed` and one `lost`, and a card left `running`.

**Empty result.** `Complete`, `Review`, and `Block` return `Empty` when `trim` is empty. They return before they write the next status. `an_empty_result_and_a_stranger_lose` keeps the card `running` after a whitespace result.

**Pause blocks new claims.** A paused board returns `Paused` for `Op::Claim`. `Complete` is not that match arm. The pause test expects `Paused` on a ready card on a paused board.

**Parents gate the claim.** `parents_done` requires every parent id to be a task whose status is `done`. Otherwise `Claim` returns `Parents` and `transact` does not rewrite the file. `a_parent_that_is_not_done_blocks_the_claim` expects `Parents` while the parent is still `ready`.

## What you implement

`my-harness/tests/ch46.rs` is the milestone.

```rust
use my_harness::chapter_46_claims_won;

#[test]
fn two_claims_of_one_task_one_wins() {
    assert_eq!(chapter_46_claims_won(2), 1);
}
```

The stub panics.

```rust
pub fn chapter_46_claims_won(attempts: usize) -> usize {
    let _ = attempts;
    todo!("chapter 46")
}
```

`todo!("chapter 46")` is the panic. The test passes `2` and wants `1` back: two claims, one winner. In the step crate that is `two_processes_cannot_hold_the_same_task`. Two processes call `transact` of `Op::Claim` on one ready task. One stdout line is `claimed`. The other is `lost`. The status is `running`. The stub stays `todo!`. Do not paste `kanban.rs` into `my-harness/`.

## Chapter summary

- **File.** `board.json` for the cards. `board.lock` for the transaction.
- **Swap.** `create_new`, then status `ready` and owner `None`, or `Lost`.
- **Verbs.** Claim, complete, review, block. Each call to `transact` is one of them.
- **Race.** Two processes. One `claimed`. One `lost`.
- **Model.** `FakeModel` runs after `!lock_path.exists()`. The demo line is `kanban: done`.
- **Time.** `claim_transaction` mean 328.02 µs. That is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). The id is `46/claim_transaction`. The aliases are `44/claim_transaction` and `42/claim_transaction`. The Criterion `time:` line is the slope.

[personal note: the lock file that had to disappear before the reply]

## What's next

Chapter 47 is the message bus. `redact_for_egress` runs on every body.
