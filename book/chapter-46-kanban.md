# Chapter 46: The Kanban Board

Process A creates `board.lock`. Process B tries the same path and gets `Busy`. There's no version integer: `transact` locks, reads, checks status and owner, writes, and unlocks. The lock is a sibling file opened with `create_new`. A sees status `ready` and owner `None`, writes `running`. `Drop` deletes the lock, so B, once it can read, sees `running` and returns `Lost`. The claim binary prints `claimed` and `lost`, the in-process demo is one claim and then a complete. That demo prints `kanban: done`. Hermes's SQLite board is the compare-and-swap, and this chapter's board is the JSON file and the lock.

Inside one process, chapter 45's board is enough. A mutex doesn't cross a process boundary and two processes that both want the same task would each think they own it. This chapter adds that board in `steps/ch46-kanban`, a copy of chapter 45 plus `kanban`. The package is `ch46-kanban`. Pause is a field on the JSON, and it isn't the process-wide emergency stop.

A card is a `Task` on a `Board`, an `Op` is claim, complete, review, or block. `apply` checks `parents_done` before a dependent task can be claimed. `Unlock` releases the sibling lock, and paths stay inside the board file the test created.

Chapter 47 adds a message bus on this harness. The kanban board stays the durable task list. The bus is how agents talk. A card has an owner and a status while a message has a topic and a subscriber. Chapter 48 keeps a skill proposal on its own queue. A skill is a procedure and a card is a unit of work. So the proposal isn't a kanban card, and `skill_propose` doesn't call `publish`.

The durable claim is what lets a team keep learning across crashes, which is the piece a continuously improving harness needs from a board on disk. A process that dies mid-task doesn't take the task with it, another process claims it. Reflection on the parent can see that the task finished once. A claim both processes think they won would run the task twice. The transaction is why the task finishes once.

At the file, the board is serialized JSON. The write is a temp file plus a rename so a reader never sees a half board. The lock is a sibling file, not a version field. `TxError` is the conflict, and the two-process test spawns the claim binary and checks what it prints.

Running `cargo run -p ch46-kanban --bin ch46-kanban` runs the in-process demo. The exercise is `my-harness/tests/ch46.rs`, where two claims of one task must come back as one win.

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

## What this chapter decides

`transact` creates `board.lock` with `create_new`, reads `board.json`, applies one op, writes the file back, and deletes the lock in `Drop`. `acquire` closes the file handle before it returns the `Unlock`. The delete runs when `transact` returns, including when a later step returns an error. `_lock` drops on the way out.

A claim requires status `ready`, owner `None`, parents `done`, and a board that isn't paused. It sets `running` and the owner. A card that isn't `ready`, or that already has an owner, returns `Lost`.

Complete requires `running`, the same owner, and a non-empty result, `trim` decides empty, and whitespace is `Empty`. The success path sets `done`, stores the result, and clears the owner. Review requires the owner and a non-empty summary, then sets `review` and clears the owner, and block does the same on the way to `blocked`, with the reason stored in `result`.

A second claim of a `running` card returns `Lost`. Two processes run the binary `ch46_kanban_claim`, one prints `claimed`, and the other prints `lost`. `Parents`, `Paused`, and `Busy` also print `lost` from that binary, while `Empty`, `Missing`, and `Io` print the debug form on stderr and exit 1.

The demo uses a temp directory, claims `t1` for `worker`, checks the lock file is gone, runs `FakeModel`, completes with result `shipped`, and returns `kanban: done`.

## 46.1 `Task` and `Board`

The new module is `kanban.rs`, the chapter 45 copy is `blackboard.rs`. `pub mod kanban` is what `chapter_demo.rs` and `kanban_claim.rs` import.

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

A task is one card: `id` and `status` are required strings, `owner` and `result` are optional. `parents` is a vec of parent ids, with `serde(default)` on those three so a missing JSON field becomes `None`, `None`, and an empty vec while a present field deserializes as written. `ready` builds a card whose status is the string `ready`, whose owner and result are `None`. Whose parents vec is empty, and the id comes from `Into<String>`. So `Task::ready("t1")` stores `t1`. Status is a `String` rather than an enum, and `apply` compares it with `!=` to `"ready"`, `"running"`, and the other literals.

The board is the file: `paused` defaults to false when the JSON omits it, `tasks` is a vec in file order. `new` stores that vec with `paused` false. There's no `Default` impl in this file, so callers pass the tasks they want. `demo` passes `vec![Task::ready("t1")]`. The derives include `Serialize` and `Deserialize`, which only `write_board` and `read_board` use. The tests compare status strings and `TxError` values rather than whole boards.

> **Rust sidebar: `#[serde(default)]` on `Option` and `Vec`**
>
> `Option<String>` already deserializes from JSON `null` as `None` and from a string as `Some`, and `default` covers a missing key, where serde uses `Default` so an `Option` becomes `None`, a `Vec<String>` becomes empty, and a `bool` becomes `false`. Without `default`, a missing `owner` key would fail `from_slice` and `read_board` would return `TxError::Io`. The tests write boards through `write_board`, which always emits the fields, so the default is what a hand-edited file can rely on, and this chapter's tests don't omit fields.

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

`Op` is the one verb `transact` applies: each variant carries the owner, `Complete` also carries `result`, `Review` carries `summary`, and `Block` carries `reason`. Those three strings are the evidence `apply` trims. Claim has no evidence string; `transact` takes `Op` by value, the caller moves the owner string in. `apply` borrows the op as `&Op` and clones the owner into the task only on the success path of `Claim`.

| Variant | When it is returned |
|---|---|
| `Lost` | status or owner check failed inside the lock |
| `Empty` | evidence trims to blank |
| `Parents` | a parent isn't `done` |
| `Paused` | `Claim` on a paused board |
| `Busy` | lock retry gave up |
| `Missing` | the id isn't in `tasks` |
| `Io` | filesystem or JSON error this file maps |

`Busy` is the lock file giving up after retries. `Lost` is the status or owner check inside the lock, and they aren't the same failure. There's no `Display` impl and no `thiserror` derive in this file; the claim binary prints `{error:?}` on the variants it doesn't map to `lost`, and the variants carry no path and no errno, which is why the tests can write `Err(TxError::Empty)`.

> **Rust sidebar: a closed error set**
>
> `Op::Claim { owner: "worker".into() }` names the field at the call site, a later match binds `owner` by name, and adding a fifth op would fail to compile in `apply` until a new arm existed. `transact` returns `Result<String, TxError>`, the `String` is the status after a successful write, and an error leaves the file as it was, except for `Io` from `write_board` after `apply` mutated memory: that mutation drops with the local `board`, `Drop` removes the lock file, and the JSON on disk is whatever `write_board` last successfully renamed.

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

`Unlock` is private, and its only field is the lock path. `Drop` calls `fs::remove_file` and ignores the error with `let _`. So a missing lock file at drop time doesn't panic and doesn't change the `Result` `transact` is about to return. That remove is the end of the transaction, which is what the comment on `transact` means when it says the lock file is gone before the function returns.

`board_path` joins `board.json` and `lock_path` joins `board.lock`, and both are pure: they don't create anything. The demo and the model test both look at `board.lock` after `transact`. `task_status` reads `board.json` through `read_board`.

`write_board` is public because the tests and the bench build the starting file with it. `create_dir_all` makes the directory, `to_vec` serializes the board, the bytes go to `board.json.tmp`. `rename` moves that temp onto `board.json`. If the rename fails, the function removes `board.json` and renames again. The second failure is `TxError::Io`, while the first failure isn't returned on its own. `read_board` is private, it reads `board.json` and deserializes, and either a missing file or a bad payload is `Io`. Callers that want a status without taking the lock use `task_status`, which calls `read_board`.

`map_err(|_| TxError::Io)` drops the underlying error. So a permission error and a JSON syntax error look the same to the caller. The claim binary then prints `Io` if it hasn't already mapped the variant to `lost`.

> **Rust sidebar: `Drop` as the unlock**
>
> `Drop::drop` runs when the value is dropped, including on `?` early returns after the value exists. `transact` binds `let _lock = acquire(...)?`, and the `?` on `acquire` means a failed acquire never builds an `Unlock`, so `Drop` doesn't run and there's no lock file from this call to remove. After `Ok(Unlock)`, every later `?` in `transact` drops `_lock` first, on `Lost`, `Empty`, `Parents`, `Paused`, `Missing`, and `Io` the same way as on success. The underscore means the binding is otherwise unused, and the destructor is the use.

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

The loop is `0..2_000`, two thousand attempts, and each attempt opens `path` with `write(true)` and `create_new(true)`. `create_new` succeeds only when the file didn't exist, that success is the winner of the lock file. `drop(file)` closes the handle before `Unlock` is returned so the later `remove_file` can delete it. `Unlock` remembers the path and doesn't hold the `File`.

`AlreadyExists` means another process, or another call in this process, already created `board.lock`. `PermissionDenied` is the case the comment names: Windows can report a sharing violation while the other process deletes the lock file. The comment says that is a retry, not a loss, so both kinds sleep `Duration::from_micros(100)` and try again. Any other `ErrorKind` returns `Io` immediately, without sleeping and without burning the rest of the attempts.

If all 2000 attempts hit `AlreadyExists` or `PermissionDenied`, the function returns `Busy`. It doesn't create the file, and it doesn't return `Lost`. `Lost` is the status and owner check inside `apply` while `Busy` is the lock file, the split from the opening. The descriptions file says the 100 microsecond retry runs only when the lock file already exists. An uncontended `transact`, which is the bench, takes the `Ok(file)` arm on the first open so the sleep doesn't run. `acquire` is private, tests reach it through `transact`, the two-process test is what contends. The bench writes a fresh directory so `board.lock` is absent when `transact` starts.

> **Rust sidebar: `create_new` and a match guard**
>
> `OpenOptions::create_new(true)` sets the create-new flag, and `open` then fails with `AlreadyExists` if the path is there. Winning the file isn't the same as winning the card: a process can create `board.lock`, read a card that is already `running`, and return `Lost`, and `Drop` still removes the lock file.
>
> The `if` on the `Err` arm is a match guard. It runs after the pattern matches `Err(error)`, both error kinds share one body, and a guard that was false would fall through to `Err(_)`, which is `Io`. `Ok(file)` is first so a successful create never sleeps.

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

`parents_done` is true when every parent id is a task on this board whose status is `done`. `all` on an empty iterator is true. So a task with no parents passes, `find` locates the parent by id. `is_some_and` requires the parent to exist and to be `done`. A missing parent id fails the check, a parent in `ready`, `running`, `review`, or `blocked` fails the check. The function doesn't look through grandparents.

`apply` finds the task index by id, and no match is `Missing`. The pause check is next, and only for `Op::Claim`: `matches!(op, Op::Claim { .. })` ignores the owner. A paused board returns `Paused` before parents are considered and before the status is read for the claim. `Complete`, `Review`, and `Block` don't take that branch, so a paused board still accepts those three when the rest of their checks pass. Pause blocks a new claim and still allows complete.

`parents_ok` is computed before the mutable borrow of the task. The parent walk needs to read `board.tasks` while `let task = &mut board.tasks[index]` borrows one element. The bool is a copy, and the borrow of the slice ends when `parents_done` returns.

Claim returns `Parents` when `parents_ok` is false, before it writes status or owner. It returns `Lost` when status isn't `ready` or owner is `Some`. Those two checks are the status and owner test inside the lock. The Hermes update required `status = source` and `claim_lock IS NULL`. Here the source is the string `ready` and the empty owner is `owner.is_none()`, written as `is_some()` in the failure condition. On success the status becomes `running` and the owner becomes `Some` of a clone of the op's owner.

Complete trims `result`, an empty trim is `Empty` before the owner check. The test uses `"  "`, which trims to empty. A non-empty result then requires status `running` and `owner.as_deref() == Some(owner)`, a stranger's owner fails that equality and returns `Lost`. Success writes `done`, stores the result, and sets owner to `None`. Review is the same shape with `summary` and status `review`, block is the same shape with `reason` and status `blocked`, and all three clear the owner. A later claim can take the card only from `ready`. So moving to `review` or `blocked` or `done` makes a second claim return `Lost`. This file has no op that moves `review` back to `ready`.

`Ok(())` means the in-memory board was updated, and `transact` still has to write it. `apply` doesn't touch the disk.

> **Rust sidebar: `is_some_and` and `as_deref`**
>
> `is_some_and` takes the `Option` from `find` and applies a predicate only on `Some`, `None` is false, and the predicate sees `&Task`. `candidate.status == "done"` is a string compare, so a parent whose status is `Done` with a capital D would fail. `ready` builds the lowercase spelling, and `apply` writes lowercase spellings.
>
> `as_deref` on `Option<String>` yields `Option<&str>`. In the `Complete` arm, `op` is `&Op`, so `owner` is `&String`, and `PartialEq` compares that option with `Some(owner)` without cloning the stored owner. A mismatch is `Lost` and doesn't clear the owner, which is what the stranger test relies on: after `Err(Lost)`, `task_status` is still `running`.

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

`transact` is the public transaction, and the doc comment says the lock file is gone before the function returns. The order is fixed: acquire `board.lock`, read `board.json`, apply the op, read the status string back out of the mutated board, write the board, and return that status. `_lock` drops at the final brace, and it also drops if `read_board`, `apply`, the status find, or `write_board` returns an error through `?`.

The status find after `apply` is a second lookup by id. `apply` already found the id or returned `Missing`, a successful `apply` leaves the task in the vec so `find` succeeds. `ok_or(TxError::Missing)` is the arm if it doesn't. The cloned status is what the caller sees, `running`, `done`, `review`, or `blocked`, depending on the op. A claim of `t1` in the tests expects `running`.

`write_board` rewrites the whole file, not a single field, so every task on the board is serialized again. The bench has one task; a board with a parent and a child rewrites both when a claim of the child succeeds. It doesn't rewrite when the claim fails. `apply` returns `Parents` before `write_board`.

`task_status` doesn't acquire the lock: it reads `board.json` and clones the status, or returns `Ok(None)` when the id is absent. `Ok(None)` and `Err(Missing)` are different: `transact` uses `Missing` for an id it was asked to change, while `task_status` uses `None` for an id it was asked to read. A missing file is `Io` for both, because both go through `read_board`. The two-process test calls `task_status` after both children have exited and expects `Some("running")`.

> **Rust sidebar: `ok_or` and a lock that isn't a data guard**
>
> `ok_or` converts `Option<String>` into `Result<String, TxError>`, `?` then returns `Missing`, and the clone happens inside `map` before `ok_or`, so the board borrow ends before `write_board` needs `&board` again. The function returns a `String` the caller owns after the board local is dropped.
>
> The lock file isn't a `MutexGuard` around the JSON, and nothing in `task_status` checks `board.lock`. A reader during `transact` can observe the previous `board.json`, because the write is a rename of `board.json.tmp` onto `board.json` at the end, and a reader can't observe a half-applied `Task` inside `apply`, because `apply` edits memory and the file changes only in `write_board`. Two `transact` calls contend on `create_new`, not on the JSON parse.

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

`FakeModel` is the same shape as the blackboard fake, and it is a different type. `complete` discards the prompt and returns `reply`, the bench doesn't construct it, and `descriptions.json` says `FakeModel` isn't constructed in the sample.

The demo builds a directory named `ch46-kanban-` plus the process id, under the temp dir. `remove_dir_all` clears a leftover from a previous run of the same process, and the error is ignored. `write_board` creates the directory and writes one ready task `t1`, `transact` claims it for `worker`. `expect("claim")` panics if the claim fails. `assert!(!lock_path(&dir).exists())` runs before `FakeModel`. The lock file from the claim is already gone once `transact` has returned and `Drop` has removed it, which is the release the opening walked. `complete("t1")` returns `ok`, `let _` drops that string, and the prompt is the task id rather than a provider prompt. The second `transact` completes as `worker` with result `shipped`, the status comes back `done`, `remove_dir_all` deletes the directory. `format!("kanban: {status}")` is `kanban: done`, which the chapter binary prints.

```rust
use harness_core::kanban::demo;

fn main() {
    println!("{}", demo());
}
```

Running the package needs `--bin ch46-kanban`, because the example binary prints eval lines, and no key is read.

The claim binary is `src/bin/kanban_claim.rs`. The test points at it with `env!("CARGO_BIN_EXE_ch46_kanban_claim")`, which Cargo sets from the bin name `ch46_kanban_claim`.

> **Gotcha.** The claim binary is `ch46_kanban_claim`, not the demo binary `ch46-kanban`. Cargo turns hyphens into underscores in `CARGO_BIN_EXE_...`, and a shared name would collide with `CARGO_BIN_EXE_ch46_kanban`.

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

Args are `claim`, the directory, the id, and the owner. A missing command prints the usage line and exits 2, a command other than `claim` does the same. A missing directory, id, or owner exits 2 without the usage line. `transact` of `Op::Claim` is the whole body: `Ok` prints `claimed` and exits 0. `Lost`, `Parents`, `Paused`, and `Busy` print `lost` and exit 0. The two-process test looks at stdout for those two words and doesn't check the exit code. Any other error prints `Debug` to stderr and exits 1. The test message includes stderr if the stdout words are wrong, which is how a surprise `Io` shows up.

The two-process test spawns two threads. Each thread runs `Command::new` on that executable with args `claim`, the same directory, `t1`, and owner `a` or `b`, which is the race the opening walked. One `create_new` wins, that process's `apply` sees `ready` and owner `None`, writes `running`, and prints `claimed`. The other process either retries the lock and then sees status `running` and returns `Lost`, or loses as `Busy`, and both of those print `lost`. The test asserts the two stdout lines contain both words, in either order. That `task_status` is `running`.

> **Rust sidebar: `env!` and `ExitCode`**
>
> `env!("CARGO_BIN_EXE_ch46_kanban_claim")` is a compile-time read, the integration test binary doesn't look up `PATH`, and Cargo fails the build if that bin wasn't built for the test. The string is the absolute path of the executable, `Command::new` takes it, and the test trims whitespace before comparing stdout to `claimed` and `lost`.

## 46.8 The SQLite compare-and-swap

If the board were a database, the rule would be the one in Hermes: WAL, `BEGIN IMMEDIATE`. A compare-and-swap on status. This file retries the lock and doesn't retry a `Lost`.

The module docstring in `hermes_cli/kanban_db.py` states that rule in one sentence, and it adds that SQLite serializes writers, one claimer wins, losers see zero rows. There are no retries and no distributed locks in that sentence. A `Lost` returns to the caller; the claim binary prints `lost` and exits 0. It doesn't loop back into `transact`.

`_claim_and_open_run` updates one row to `status = 'running'` and sets `claim_lock`, `claim_expires`, and `started_at`. Only where `id` matches, `status` is the source status, and `claim_lock IS NULL`. `rowcount != 1` returns `None` before it inserts a `task_runs` row. The winner then inserts the run, stores `current_run_id`, and appends a `claimed` event. This crate's `apply` has the status check and the empty-owner check. It doesn't insert a run row, set an expiry, or append an event. Those are the deferred pieces: WAL, `task_runs`, and claim expiry.

`claim_task` calls `_claim_and_open_run` with source `ready`. Parents are checked first. `_parents_satisfied` is false when any direct parent isn't in `done` or `archived`. On that failure `claim_task` updates the card from `ready` to `todo` and returns `None`. `parents_done` in this file requires status `done` only, `archived` isn't a status `apply` writes. A parent that isn't `done` makes `Claim` return `Parents` without changing the child. `write_board` is skipped, the child stays `ready` on disk, and Hermes demotes where this crate refuses and leaves the status.

The demo asserts the lock file is absent and then constructs `FakeModel`. The model is the subscriber that must not run under the lock. Chapter 45 did that with `try_lock` after `tick`. This chapter does it with `Drop` of `Unlock` before `complete`, the assertion `!lock_path(&dir).exists()`. The complete op is a second transaction, after the model, with its own lock file that is also gone before `demo` formats the line.

The blackboard module is still in the crate. `kanban.rs` doesn't call `tick` or `post`, the chapter 45 tests are copied forward and still run. The kanban tests don't construct a `Board` from `blackboard`.

The race test `two_processes_cannot_hold_the_same_task` doesn't read the owner field. So it doesn't assert whether `a` or `b` won, only that stdout was `claimed` and `lost` and the card was left `running`.

The bench is one uncontended claim of `t1`, with the setup directory outside the sample and `remove_dir_all` inside the sample after `transact`. There's no second process and no `FakeModel`.

> **Rust sidebar: `rowcount` has no twin here**
>
> SQLite's `rowcount` is the number of rows the `UPDATE` changed, and zero means the `WHERE` missed, so the loser returns `None`, and `apply` doesn't have a row count. The miss is a boolean: status isn't `ready`, or owner is `Some`, and both become `TxError::Lost`. A missing id is `Missing` instead of `Lost`, because the task was never a candidate row. Hermes would also change zero rows for a bad id, and `rowcount != 1` would return `None` for that too, while this file splits those two outcomes.

## Tests

The new tests live in `steps/ch46-kanban/crates/harness-core/tests/kanban.rs`. The package also builds the copied harness tests, including `blackboard.rs`.

`fresh` builds a directory under the temp dir, named `ch38-` plus the label, the process id, and the current time in nanoseconds, and it `remove_dir_all`s that path before returning it. The `ch38-` prefix is the string in the source, and the bench uses `ch38-bench-` plus the process id and a counter.

One status per verb is `claim_complete_review_and_block_are_one_status_each`, which uses three directories from `fresh("verbs")`, `fresh("block")`, and `fresh("done")`. On the first, a claim of `t1` by `worker` returns `running` and a review with summary `look` returns `review`. On the second, claim then block with reason `needs a human` returns `blocked`. On the third, claim then complete with result `shipped` returns `done`. The last line removes `fresh("noop")`, which is a new empty directory rather than one of the three boards. Each successful `transact` returns the status it cloned after `apply`.

Empty evidence and a stranger both lose in `an_empty_result_and_a_stranger_lose`, which claims `t1` for `worker`. Complete with result `"  "` equals `Err(TxError::Empty)`, complete with owner `other` and result `shipped` equals `Err(TxError::Lost)`, and `task_status` is `Some("running")`. The empty check runs first, so the whitespace result never reaches the owner compare, the stranger passes the trim check and fails the owner compare. The owner on the card stays `worker` because `Lost` returns before the assignment. The test checks the status, not the owner field.

A parent that isn't done blocks the claim in `a_parent_that_is_not_done_blocks_the_claim`, which builds a child whose `parents` vec holds `parent` and a parent from `Task::ready("parent")`. So the parent status is `ready`. Claim of `child` by `worker` equals `Err(TxError::Parents)`, `apply` returns before it writes. `transact` skips `write_board` on that error, so the child stays `ready` in the file `write_board` wrote at the start.

Pause refuses a new claim in `pause_refuses_a_new_claim_and_complete_still_runs`, which writes a ready `t1`, claims it, then builds a paused board in memory with `t1` still `running` and owned by `worker`, and writes that file. Claim of `t2` on that directory equals `Err(TxError::Missing)`, because `t2` isn't in `tasks`. A second directory from `fresh("pause-claim")` holds a paused board whose only task is ready `t2`, and claim of `t2` by `other` equals `Err(TxError::Paused)`. The function name says complete still runs, and the body doesn't call `Op::Complete`. The pause split is in `apply`, which returns `Paused` only for `Op::Claim`. So this test locks the `Paused` return and the `Missing` return.

Two processes can't hold the same task in `two_processes_cannot_hold_the_same_task`: `fresh("race")`, one ready `t1`, two threads, one executable from `env!("CARGO_BIN_EXE_ch46_kanban_claim")`. Args `claim`, the directory, `t1`, and owner `a` or `b`. Stdout must contain `claimed` and `lost`, the assertion message includes both stderr strings. `task_status` is `Some("running")`.

The model runs after the lock file is gone in `the_model_runs_after_the_lock_file_is_gone`, which claims `t1` for `worker`, asserts `board.lock` doesn't exist, and expects `FakeModel { reply: "ok" }.complete("t1")` to equal `ok`.

The demo finishes the card in `demo_finishes_the_card`, which expects `demo()` to equal `kanban: done`.

The seven test functions appear below in the order the file defines them.

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

The summary above is the whole package, copied tests included.

```bash
cargo run -p ch46-kanban --bin ch46-kanban
```

```text
kanban: done
```

The binary writes a temp board, claims it, and completes it, and no key is read.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `claim_transaction` | 328.02 µs | one claim transaction under 5 ms | lock file, JSON rewrite, `remove_dir_all` |

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

> **Rust sidebar: the setup counter sits outside the timer**
>
> `n` is captured mutably by the setup closure, `iter_batched` calls that closure before it starts timing the routine, and the increment and the directory creation are on the untimed side. The routine takes `PathBuf` by value, so the path moves into the timed closure and the setup doesn't keep a second owner. `remove_dir_all` inside the routine deletes the directory the next setup will not reuse, because `n` has moved on, and `expect("claim")` panics the bench if `transact` returns an error. An uncontended claim of a ready card returns `running`.

The setup closure runs outside the timer. The counter `n` gives each sample its own directory, `ch38-bench-` plus the process id plus `n`.

The means are `mean.point_estimate` from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), the bench ids are `46/claim_transaction`, `44/claim_transaction`, and `42/claim_transaction`. The command is `cargo bench -p ch46-kanban --bench kanban --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The bench is one uncontended claim, plus `remove_dir_all` of the temp directory. So a number that includes two processes and the 100 microsecond retry is `two_processes_cannot_hold_the_same_task` rather than this sample. The descriptions file says the retry loop runs only when the lock file already exists. The bench's setup creates a fresh directory and doesn't create `board.lock`.

The lock file is removed in `Drop`, so a crash between `create_new` and that drop leaves `board.lock` behind. The next `acquire` retries 2000 times at 100 microseconds and returns `Busy`, and this crate doesn't delete a stale lock based on age. Hermes stores `claim_expires` on the row inside `_claim_and_open_run`, and that expiry isn't here.

On Windows, `PermissionDenied` is a retry, per the comment in `acquire`, and an immediate `Io` on that kind would skip the sleep. `AlreadyExists` is the other retry kind, and every other kind is `Io` on the first sighting.

Complete from a process that isn't the owner is `Lost`, the card stays `running`. An empty or whitespace result is `Empty`, with that check before the owner check. Clearing the owner on review and on block is what the success arms do, a later claim still needs status `ready`. This file never writes `ready` except in `Task::ready` at construction.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The catalog's N2 row is the ESTOP in `agent/estop.py`. This module does not call `is_engaged`. Pause is the `paused` field on the JSON board. The behaviors below are the ones `kanban.rs` implements.

**Compare-and-swap.** `create_new` makes one lock file. `apply` for `Claim` requires status `ready` and owner `None`, and otherwise returns `Lost`. Two owners cannot both pass that check. `two_processes_cannot_hold_the_same_task` expects one `claimed` and one `lost`, and a card left `running`.

**Empty result.** `Complete`, `Review`, and `Block` return `Empty` when `trim` is empty. They return before they write the next status. `an_empty_result_and_a_stranger_lose` keeps the card `running` after a whitespace result.

**Pause blocks new claims.** A paused board returns `Paused` for `Op::Claim`. `Complete` is not that match arm. The pause test expects `Paused` on a ready card on a paused board.

**Parents gate the claim.** `parents_done` requires every parent id to be a task whose status is `done`. Otherwise `Claim` returns `Parents` and `transact` does not rewrite the file. `a_parent_that_is_not_done_blocks_the_claim` expects `Parents` while the parent is still `ready`.

## What you implement

The milestone the reader implements is `my-harness/tests/ch46.rs`. The test below is what has to pass.

```rust
use my_harness::chapter_46_claims_won;

#[test]
fn two_claims_of_one_task_one_wins() {
    assert_eq!(chapter_46_claims_won(2), 1);
}
```

The stub panics on purpose, with the body below.

```rust
pub fn chapter_46_claims_won(attempts: usize) -> usize {
    let _ = attempts;
    todo!("chapter 46")
}
```

The panic is `todo!("chapter 46")`, and the test passes `2` and wants `1` back: two claims, one winner. In the step crate that is `two_processes_cannot_hold_the_same_task`, where two processes call `transact` of `Op::Claim` on one ready task, one stdout line is `claimed`, the other is `lost`. The status is `running`, while the stub stays `todo!`.

## Chapter summary

- The file is `board.json` for the cards and `board.lock` for the transaction, a sibling created with `create_new`.
- A claim inside that lock needs status `ready` and owner `None`, and anything else comes back `Lost`.
- Claim, complete, review, and block are the four verbs, and each call to `transact` is one of them.
- Two processes print one `claimed` and one `lost`, and the card is left `running`.
- `FakeModel` runs after `!lock_path.exists()`, and the demo line is `kanban: done`.
- The `claim_transaction` mean is 328.02 µs, `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), id `46/claim_transaction`, aliases `44/claim_transaction` and `42/claim_transaction`.

## What's next

Once a card is `done`, the workers still need to say so without naming a listener, and chapter 47's bus is that publish, with backpressure when a subscriber is slow and `redact_for_egress` on every body.
