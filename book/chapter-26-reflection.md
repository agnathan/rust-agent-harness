# Chapter 26: The Reflection Loop

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 25 can write a skill when something calls `Writer`. It does not count turns, and it does not build a reviewer. This chapter counts user turns and tool iterations. On every tenth user turn, and on every tenth tool iteration, `Loop::finalize` returns a `Fork`. The fork's tool slice is the skill tools, plus `memory` only when the memory nudge is what fired. Both nudge intervals on the fork are 0. Cancel stores a flag. The turn does not sleep for two seconds. `run_loop` does not call `finalize`. The fork is a value the caller can run later, with `execute`.

The step crate is `steps/ch26-reflection`. It is chapter 25's harness copied forward, plus `reflection`. The package name is `ch26-reflection`. The library name is `harness_core`. `lib.rs` adds `pub mod reflection`. There is no path dependency on chapter 25. `execute` calls `crate::skills::skill_view` when the scripted line is `skill_view`. The file is `steps/ch26-reflection/crates/harness-core/src/reflection.rs`.

| Section | What it walks |
|---|---|
| 26.1 | The intervals, the timeout, the two tool slices |
| 26.2 | `Loop`, `note_user_turn`, `note_tool_iter` |
| 26.3 | `finalize` and `Default` |
| 26.4 | `spawn_reviewer` and `Fork` |
| 26.5 | `ScriptedModel` and `execute` |
| 26.6 | `ReviewRun`, cancel, and the two-second constant |
| 26.7 | `demo`, and `run_loop` leaving the fork alone |
| 26.8 | What a skill-index note still is |

> **How Hermes does it**
>
> I read `agent/agent_init.py` (the assignments of `_memory_nudge_interval` and `_skill_nudge_interval`), `agent/background_review.py` (`spawn_background_review_thread`, `_BACKGROUND_REVIEW_CANCEL_TIMEOUT_SECONDS`, the fork that sets both nudge intervals to 0, `_review_tool_whitelist`), and `hermes_cli/config_defaults.py` (`memory.nudge_interval`) on Hermes `main` at `ab3fe9dffb0a`.
>
> Both intervals in this crate are 10. The fork sets both nudge intervals to 0. A skill-nudge review does not get the memory tool. The whitelist adds `read_file` and `search_files`. Write tools stay denied. The advertised tools list stays the parent's for prompt-cache parity. This harness does not advertise a schema. It only has the fork's tool slice. `_BACKGROUND_REVIEW_CANCEL_TIMEOUT_SECONDS` is 2.0. `memory.nudge_interval` is the config key.
>
> **Adopted.** The zeroed intervals on the fork. The tool split. The cancel flag.
>
> **Adapted.** The 2 second wait. This crate does not sleep it.
>
> **Deferred.** Running the reviewer inside `drive_loop`. `Loop::finalize` returns the fork. The user turn's `run_loop` does not call it.

> **Guardrails.** The fork's tool slice is the whitelist. `terminal` is refused. A skill nudge omits `memory`. Intervals on the fork are 0, so the reviewer does not schedule another reviewer. Cancel is an `AtomicBool`. `HERMES_CANCEL_TIMEOUT` is two seconds and this crate does not sleep it. The user turn does not run the reviewer.

[personal note: a turn you wished the agent had reviewed after you left]

## What this chapter decides

`MEMORY_NUDGE_INTERVAL` and `SKILL_NUDGE_INTERVAL` are both 10. `note_user_turn` increments a counter and sets `memory_due` when the count is a multiple of 10. `note_tool_iter` does the same for `skill_due`. `finalize` clears both flags. When both are clear it returns `None`. When either is set it returns `spawn_reviewer(review_memory)`. A turn where both flags are set still passes `review_memory == true`, so the fork includes `memory`.

`spawn_reviewer(true)` copies `memory`, `skill_view`, `skill_manage`, `read_file`, and `search_files`. `spawn_reviewer(false)` copies the same list without `memory`. Both intervals on the fork are 0. The function does not start a thread. It stores a fresh `AtomicBool` set to false.

`execute` pulls one scripted line at a time. The first word is the tool. A tool absent from the fork's slice is `ReviewError::NotAllowed`, and later lines do not run. `memory` pushes the rest of the line onto a note vec. `skill_view` calls chapter 24's loader when a root is passed. `read_file`, `search_files`, and `skill_manage` return the line. They do not write a file. Cancel is checked after the line is taken from the model and before the tool runs.

`ReviewRun::spawn_spinner` starts a thread that sleeps 1 millisecond until the flag is true. `cancel_and_join` sets the flag and joins. The test expects that join to finish in under 50 milliseconds. `HERMES_CANCEL_TIMEOUT` names the two seconds Hermes waits. Nothing in this file calls `thread::sleep` with that duration.

`demo` takes 10 user turns, finalizes, and runs `memory prefer short notes`. The binary prints that note and the line `forked: after 10 user turns`. No key.

`drive_loop` is still the user turn. It does not construct a `Loop`. It does not call `finalize`.

The skill index from chapter 24 is still a note when the tree changes after the prompt. This chapter does not give the parent request a new copy of those bytes.

## 26.1 Two intervals, one timeout, two slices

The constants are the policy. The slices are the tools.

```rust
pub const MEMORY_NUDGE_INTERVAL: u32 = 10;
pub const SKILL_NUDGE_INTERVAL: u32 = 10;
/// Hermes waits this long for a review thread to notice cancel. This crate
/// does not sleep for it. The flag is the cancel.
pub const HERMES_CANCEL_TIMEOUT: Duration = Duration::from_secs(2);

const MEMORY_TOOLS: &[&str] = &[
    "memory",
    "skill_view",
    "skill_manage",
    "read_file",
    "search_files",
];
const SKILL_TOOLS: &[&str] = &["skill_view", "skill_manage", "read_file", "search_files"];
```

Both intervals are `u32`. The counters on `Loop` are `u32` too, so the comparison does not mix integer widths. Ten user turns is the memory nudge. Ten tool iterations is the skill nudge. The doc comment on `HERMES_CANCEL_TIMEOUT` is the adaptation in the source: Hermes waits this long, and this crate does not sleep for it. `Duration::from_secs(2)` is 2.0 seconds, the same number as `_BACKGROUND_REVIEW_CANCEL_TIMEOUT_SECONDS`. A search of this file finds no use of `HERMES_CANCEL_TIMEOUT` except the constant itself. The cancel path uses the flag.

`MEMORY_TOOLS` has five names. `SKILL_TOOLS` has four. The missing name is `memory`. That is the split: a skill-nudge review does not get the memory tool. Both slices include `skill_view`, `skill_manage`, `read_file`, and `search_files`. `read_file` and `search_files` are the names the whitelist adds. `terminal` is on neither slice. Write tools are absent. `skill_manage` is a name on the slice. `execute` handles it by echoing the line. It does not call `Writer`.

The types are `'static` string slices. `spawn_reviewer` copies them into a `Vec<&'static str>` on the fork. The vec owns the list of pointers. The string data stays in the binary.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewError {
    NotAllowed(String),
    Cancelled,
    Skill(String),
}
```

`NotAllowed` owns the tool name that was refused. `Cancelled` has no payload. `Skill` owns the `Debug` text of a `SkillError`, or the words `no root` when `skill_view` runs without a directory. `execute` returns `Result<Vec<String>, ReviewError>`.

## 26.2 `Loop` counts, and the flags are the nudge

`Loop` is the counter. It is not the agent loop in `agent.rs`. The name is the review schedule.

```rust
pub struct Loop {
    user_turns: u32,
    tool_iters: u32,
    memory_due: bool,
    skill_due: bool,
}

impl Loop {
    pub fn new() -> Self {
        Self {
            user_turns: 0,
            tool_iters: 0,
            memory_due: false,
            skill_due: false,
        }
    }

    pub fn note_user_turn(&mut self) {
        self.user_turns += 1;
        if self.user_turns.is_multiple_of(MEMORY_NUDGE_INTERVAL) {
            self.memory_due = true;
        }
    }

    pub fn note_tool_iter(&mut self) {
        self.tool_iters += 1;
        if self.tool_iters.is_multiple_of(SKILL_NUDGE_INTERVAL) {
            self.skill_due = true;
        }
    }

    pub fn user_turns(&self) -> u32 {
        self.user_turns
    }
```

`new` starts every field at zero or false. `note_user_turn` takes `&mut self` because it writes the counter and maybe the flag. The increment happens first. The first call takes the counter from 0 to 1. `is_multiple_of(10)` is false for 1 through 9 and true for 10, 20, and so on. Zero is a multiple of 10, and the increment-before-check is why a fresh loop does not mark itself due before any turn. The tenth call sets `memory_due`. The eleventh does not clear it. Only `finalize` clears it. Two more calls do not matter until the count hits 20, which sets the flag again if `finalize` had cleared it.

`note_tool_iter` is the same shape for `tool_iters` and `skill_due`. The two counters are independent. Ten tool iterations with no user turn set `skill_due` and leave `memory_due` false. Ten user turns with no tool iteration set `memory_due` and leave `skill_due` false.

`user_turns` is the read. The demo does not print it. The tests use `finalize` rather than this getter. The getter is there so a caller can see the count without taking `&mut self`.

The test `the_tenth_user_turn_forks_a_reviewer_with_memory` loops `MEMORY_NUDGE_INTERVAL - 1` times, which is 9, and expects `finalize()` to be `None`. The next `note_user_turn` is the tenth. `finalize` then returns a fork with `review_memory` true, `memory` in the tools, and `skill_view` in the tools. A second `finalize` is `None`, because the first one cleared the flag.

The test `a_skill_nudge_does_not_hand_the_reviewer_memory` calls `note_tool_iter` ten times and expects `review_memory` false, `memory` absent, and `skill_manage` present.

> **Rust sidebar: `&mut self` is the counter's owner**
>
> - `note_user_turn` needs a mutable borrow. A shared `&Loop` can call `user_turns` and cannot call `note_user_turn`. The compiler enforces that. There is no `Mutex` on these fields.
> - `is_multiple_of` is a method on the integer. `self.user_turns.is_multiple_of(10)` is true when the remainder of dividing by 10 is 0. The interval is the constant, so a reader can see the 10 in one place.
> - `bool` flags are not counters. They stay true until `finalize` writes false. Forgetting `finalize` leaves the flag set, and the next `finalize` still returns a fork.

## 26.3 `finalize` returns the fork and clears the flags

This is the function the chapter adds at the end of a turn. It is not called from `drive_loop`.

```rust
    /// Record the fork when a nudge is due. Returns `None` when this turn
    /// has nothing to review. Building the fork is the work added to
    /// turn finalize.
    pub fn finalize(&mut self) -> Option<Fork> {
        let review_memory = self.memory_due;
        let review_skill = self.skill_due;
        self.memory_due = false;
        self.skill_due = false;
        if !review_memory && !review_skill {
            return None;
        }
        Some(spawn_reviewer(review_memory))
    }
}

impl Default for Loop {
    fn default() -> Self {
        Self::new()
    }
}
```

The locals copy the flags before the fields are cleared. `bool` is `Copy`, so `review_memory` is a separate value. Clearing the fields first would make the locals follow the fields if they were assigned after. The order in the source is the copy, then the write of `false`.

Both locals false is `None`. The `Option` is how "no review" is represented. It is not an error. A caller matches `None` and continues the turn. `Some` is the fork.

`review_skill` is used only in the `if`. It does not select the tool list. `spawn_reviewer` receives `review_memory`. When the memory flag is true, the fork gets `MEMORY_TOOLS`, whether or not the skill flag is also true. When only the skill flag is true, `review_memory` is false and the fork gets `SKILL_TOOLS`. One turn, one fork. The function does not return two forks.

The doc comment says building the fork is the work added to turn finalize. The bench `spawn_reviewer` times that build. `finalize` itself is what the tests call, and it does the flag clear plus the build.

`Default` is the trait. `Loop::default()` calls `new`. A generic bound `T: Default` can construct a loop. The tests call `Loop::new()` directly.

> **Rust sidebar: `Option` and the `Default` trait**
>
> - `Option<Fork>` is one value. `None` has no fork. `Some(fork)` owns the fork. `finalize().expect("fork")` panics on `None` and moves the `Fork` out on `Some`. The demo uses `expect`. The nine-turn test uses `is_none()`.
> - `impl Default for Loop` is a trait implementation. The only required method is `default`. The body calls `new`, so there is one constructor and the trait is a second name for it.
> - After `finalize`, the flags are false even when the function returned `Some`. The fork does not hold a reference to `Loop`. The caller owns the `Fork`. The `Loop` can count the next ten turns.

## 26.4 `spawn_reviewer` builds the slice and zeroes the intervals

```rust
pub struct Fork {
    pub tools: Vec<&'static str>,
    pub memory_nudge_interval: u32,
    pub skill_nudge_interval: u32,
    pub review_memory: bool,
    cancel: Arc<AtomicBool>,
}

impl Fork {
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Acquire)
    }
}

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

`tools` is public so tests can call `contains`. The intervals are public so the test can assert they are 0. `review_memory` is public so the test can assert the flag that selected the slice. `cancel` is private. Callers use `cancel()` and `is_cancelled()`.

`tools.to_vec()` allocates a `Vec` and copies the `&str` pointers. `MEMORY_TOOLS` is a slice. The vec is owned by the fork. Changing the vec later does not change the constant.

Both intervals are the integer 0. The fork is not a `Loop`. Nothing in this file calls `is_multiple_of` on these fields. The zeroes are the fields the fork stores so a reviewer that read them would see 0. The test asserts both fields are 0 on the memory fork. The skill fork is built by the same function, so both of its intervals are 0 as well.

`review_memory` is stored as the argument. `execute` does not read it. The tool slice is what `execute` checks. The flag is for the caller and the test.

`Arc::new(AtomicBool::new(false))` is the cancel flag. The `Arc` is so `ReviewRun` can share a flag with a thread. `Fork` holds one `Arc`. `cancel()` takes `&self` and stores `true`. The method does not need `&mut self`, because `AtomicBool::store` takes `&self`. That is interior mutability, like the `Mutex` in chapter 25, with a smaller operation: one boolean, no guard, no poison.

`Ordering::Release` on the store and `Ordering::Acquire` on the load pair. A thread that `Acquire`-loads `true` sees the writes the storing thread made before the `Release` store. This flag has no other payload. The ordering is still the one the source writes. `Relaxed` would be the wrong copy of this code. The source says `Release` and `Acquire`.

The spawn bench calls `spawn_reviewer(true)` inside `iter_custom`. It collects the forks in a vec, stops the timer, then drops the vec. No thread is started. The `true` is passed through `black_box`.

Hermes keeps the advertised tools list as the parent's, for prompt-cache parity. This function does not build a schema. The fork's `tools` vec is the slice `execute` will allow. There is no second list hiding beside it.

## 26.5 `execute` runs the script, one tool at a time

The model in this file is a different type from chapter 24's `ScriptedModel`. This one holds many lines and uses an atomic counter so `next` can take `&self`.

```rust
pub struct ScriptedModel {
    lines: Vec<String>,
    calls: AtomicUsize,
}

impl ScriptedModel {
    pub fn new(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            lines: lines.into_iter().map(Into::into).collect(),
            calls: AtomicUsize::new(0),
        }
    }

    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::Relaxed)
    }

    fn next(&self) -> Option<String> {
        let index = self.calls.fetch_add(1, Ordering::Relaxed);
        self.lines.get(index).cloned()
    }
}
```

`new` takes any iterator of items that convert into `String`. The demo passes `["memory prefer short notes"]`, an array of one `&str`. `IntoIterator` yields the `&str`. `Into::into` owns it. `collect` builds the vec. `calls` starts at 0.

`next` is private. `execute` is the caller. `fetch_add(1, Relaxed)` returns the previous value and adds one. The first call gets index 0 and leaves the counter at 1. `Vec::get` returns `Option<&String>`. `cloned` turns that into `Option<String>`. An index past the end is `None`, and the counter has still moved. A second `execute` on the same model continues at the next index. The lines are not rewound.

`calls()` loads with `Relaxed`. The test that refuses `terminal` asserts `calls() == 1`. The refused line was consumed. The following line was not, because `execute` returned.

```rust
pub fn execute(
    fork: &Fork,
    model: &ScriptedModel,
    notes: &mut Vec<String>,
    skills_root: Option<&Path>,
) -> Result<Vec<String>, ReviewError> {
    let mut done = Vec::new();
    while let Some(line) = model.next() {
        if fork.is_cancelled() {
            return Err(ReviewError::Cancelled);
        }
        let mut parts = line.splitn(2, ' ');
        let tool = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("").trim();
        if !fork.tools.contains(&tool) {
            return Err(ReviewError::NotAllowed(tool.to_string()));
        }
        let result = match tool {
            "memory" => {
                notes.push(arg.to_string());
                arg.to_string()
            }
            "skill_view" => {
                let root = skills_root.ok_or_else(|| ReviewError::Skill("no root".into()))?;
                crate::skills::skill_view(root, arg, None)
                    .map_err(|err| ReviewError::Skill(format!("{err:?}")))?
            }
            "read_file" | "search_files" | "skill_manage" => format!("{tool} {arg}"),
            other => return Err(ReviewError::NotAllowed(other.to_string())),
        };
        done.push(result);
    }
    Ok(done)
}
```

`fork` is shared. `cancel` can be called on it during the loop because `cancel` takes `&self`. `notes` is `&mut Vec<String>` because `memory` pushes. `skills_root` is `Option<&Path>`. The memory demo passes `None`. A `skill_view` line with `None` is `ReviewError::Skill("no root")`.

The `while let` ends when `next` returns `None`. Each line is split on the first space, at most two pieces. `splitn(2, ' ')` keeps the rest of the line, spaces included, as the second piece. `trim` on `arg` drops surrounding whitespace. A line with no space has tool equal to the whole line and arg equal to `""`. `unwrap_or("")` covers an empty iterator. `splitn` on a non-empty line yields at least the first piece.

`fork.tools.contains(&tool)` refuses a name that is not on the slice. `terminal` is the test's name. The error is `NotAllowed("terminal")`. `notes` is unchanged. `done` is dropped with the error. The function returns before the match.

The match is a second check. The `contains` check already required the tool to be on the slice. The match arms are the five names the slices use, plus `other`. `other` returns `NotAllowed` again. A name that passed `contains` and fell through to `other` would be a slice entry `execute` does not implement. The constants and the match agree.

`memory` pushes `arg` onto `notes` and uses the same string as the result. The demo line `memory prefer short notes` has tool `memory` and arg `prefer short notes`. The note vec becomes `["prefer short notes"]`. The returned vec is the same words.

`skill_view` requires a root. `ok_or_else` turns `None` into the skill error. `skill_view` from chapter 24 returns `Result<String, SkillError>`. `map_err` formats the error with `Debug` and wraps `ReviewError::Skill`. The `?` returns that error. Success pushes the file text onto `done`. The call passes `None` as the supporting-file argument, so the view is `SKILL.md`.

`read_file`, `search_files`, and `skill_manage` format `{tool} {arg}` and push that string. They do not open a filesystem path and they do not call `Writer::patch`. A review that wants a real patch still goes through chapter 25, including `require_read`. This function does not call it.

Cancel is checked at the start of the loop body, after `next` has already incremented `calls`. A fork that was cancelled before `execute` consumes the first line, sees the flag, and returns `Cancelled`. The note vec stays empty. The test `cancel_before_the_next_tool_stops_the_review` asserts `Cancelled` and an empty notes vec. It uses the line `memory should not land`.

> **Rust sidebar: an atomic so `next` can share the model**
>
> - `execute` takes `&ScriptedModel`. It cannot call a method that needs `&mut self`. `fetch_add` on `AtomicUsize` takes `&self`. The counter moves. The vec of lines does not. Lines are only read, through `get`.
> - `Ordering::Relaxed` on the call counter is the source's choice. The cancel flag uses `Release` and `Acquire`. The two atomics are different cells with different orderings.
> - `notes: &mut Vec<String>` is a normal mutable borrow. `memory` pushes an owned `String`. The caller keeps the vec after `execute` returns. The demo pops it.
> - `Option<&Path>` borrows a path the caller owns. `execute` does not store it. The borrow lasts for the call.

## 26.6 `ReviewRun` is the thread the cancel bench joins

`spawn_reviewer` does not start a thread. The cancel measurement needs one. `ReviewRun` is that thread.

```rust
pub struct ReviewRun {
    cancel: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ReviewRun {
    /// A worker that sleeps in 1 ms steps until cancel. The spawn is the
    /// setup. `cancel_and_join` is the return path.
    pub fn spawn_spinner() -> Self {
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&cancel);
        let handle = thread::spawn(move || {
            while !flag.load(Ordering::Acquire) {
                thread::sleep(Duration::from_millis(1));
            }
        });
        Self {
            cancel,
            handle: Some(handle),
        }
    }

    pub fn cancel_and_join(mut self) {
        self.cancel.store(true, Ordering::Release);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
```

`spawn_spinner` makes a new flag, clones the `Arc` for the thread, and spawns. The closure is `move`, so it owns `flag`. The `ReviewRun` owns the other `Arc` and the `JoinHandle`. The handle is `Option` so `cancel_and_join` can `take` it. `take` replaces `Some` with `None` and returns the handle. `join` waits until the closure returns. `let _ =` drops the `Result` of the join. A panic inside the worker would be `Err` on that result. This worker does not panic. It loads the flag, sleeps 1 millisecond, and loads again.

The sleep is `Duration::from_millis(1)`. It is not `HERMES_CANCEL_TIMEOUT`. The comment on `spawn_spinner` says the spawn is the setup and `cancel_and_join` is the return path. The bench follows that split. `iter_batched` calls `spawn_spinner` in the setup closure, which Criterion does not include in the sample, and times `cancel_and_join`.

`cancel_and_join` takes `self` by value. The caller gives up the `ReviewRun`. The store uses `Release`. The worker's load uses `Acquire`. After the flag is true, the worker finishes its current sleep, sees the flag, and returns. The join then returns. The wait is at most one 1-millisecond sleep, plus scheduling. The test `cancelling_a_running_review_returns_quickly` starts an `Instant`, calls `cancel_and_join`, and asserts the elapsed time is under 50 milliseconds.

`Fork::cancel` and `ReviewRun::cancel_and_join` are two flags. The fork's flag stops `execute`. The run's flag stops the spinner. The chapter does not wire them to the same `Arc`. A caller who wants one flag shares the `Arc` themselves. This file keeps the two paths separate: the scripted execute checks `Fork`, and the bench joins `ReviewRun`.

The adapted wait is the part this file refuses to do. Hermes waits up to 2.0 seconds for the thread to notice. `HERMES_CANCEL_TIMEOUT` records that number. `cancel_and_join` does not pass it to `sleep`, and it does not pass it to a timed join. The join returns when the worker returns. A worker that ignored the flag would hold the join until the process ends. This worker checks the flag.

## 26.7 `demo` forks after ten user turns

```rust
pub fn demo() -> String {
    let mut loop_ = Loop::new();
    for _ in 0..MEMORY_NUDGE_INTERVAL {
        loop_.note_user_turn();
    }
    let fork = loop_.finalize().expect("fork");
    let model = ScriptedModel::new(["memory prefer short notes"]);
    let mut notes = Vec::new();
    execute(&fork, &model, &mut notes, None).expect("review");
    notes.pop().unwrap_or_default()
}
```

`0..MEMORY_NUDGE_INTERVAL` is `0..10`, ten iterations. After the loop, `memory_due` is true and `user_turns` is 10. `finalize` returns the memory fork. `expect` panics if it were `None`. The model has one line. `execute` pushes `prefer short notes`. `pop` takes that string off the vec. `unwrap_or_default` on `Option<String>` is `""` when the vec was empty. The test `the_demo_note_is_the_scripted_line` expects `prefer short notes`.

The binary is shorter than the function. It prints the note, then a fixed second line.

```rust
let note = demo();
println!("note: {note}");
println!("forked: after 10 user turns");
```

The second line is a string in `main`. It is not computed from `user_turns()`. The interval in the function is the constant 10, and the printed line says 10. They match because both are written as 10. A change to `MEMORY_NUDGE_INTERVAL` would change when `demo` forks. The printed line would still say 10 until `main` changed.

`demo` calls `execute`. That is the chapter's way of showing the note. It is not `run_loop`. The user turn is a different function, in `agent.rs`, copied forward with the rest of the harness.

```rust
pub async fn run_loop(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
) -> TurnOutcome {
    run_loop_with_inbox(
        messages, provider, registry, dispatcher, budget, cancel, started, None,
    )
    .await
}
```

The parameters are the messages, the provider, the registry, the dispatcher, the budget, the turn's `CancelFlag`, and the start time. There is no `Loop` argument. `run_loop` calls `run_loop_with_inbox`, which builds a `Driver` and calls `drive_loop`.

```rust
async fn drive_loop(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
    inbox: Option<&SessionInbox>,
    driver: &mut Driver,
) -> TurnOutcome {
```

`drive_loop` checks `cancel.is_cancelled()` on the turn's `CancelFlag`. That flag is the chapter 15 cancel, the one that stops the user turn. It is not `Fork::cancel`. The body of `drive_loop` does not name `reflection`, `finalize`, or `spawn_reviewer`. I looked for those names in `agent.rs`. The only hit in that file is the `drive_loop` definition itself, and the calls that reach it. The reviewer stays outside the user turn.

`CancelFlag` and `Fork`'s `AtomicBool` are different types. Stopping the user turn does not by itself cancel a fork, because nothing in `drive_loop` holds a fork. Starting a fork does not by itself stop the user turn. The schedule and the turn are two objects.

> **Rust sidebar: `dyn Provider` is a trait object, and this chapter does not pass one to the fork**
>
> - `run_loop` takes `&dyn Provider`. `dyn Provider` is the trait object. The concrete provider is behind a pointer. The function can call the trait's methods and does not name the concrete type.
> - `execute` does not take a `Provider`. It takes `&ScriptedModel`, a concrete struct in this module. The reviewer in this chapter has no provider trait to implement.
> - `async fn` returns a future. `run_loop` is async. `finalize` and `execute` are not. They return `Option<Fork>` and `Result<Vec<String>, ReviewError>` directly. A caller can run them without an executor.

## 26.8 The skill index is still a note

Chapter 24 deferred prompt-cache byte identity with the parent model request, and said that stays a note in this chapter. The deferral holds. `reflection` does not call `render_index`. It does not replace a `SkillSession`'s `Arc`. A skill added after the prompt is still `change_note`'s `Some(String)`. The index bytes in the `Arc` stay.

Hermes keeps the advertised tool list equal to the parent's so the prompt cache sees the same schema. This crate does not advertise a schema on the fork. `Fork.tools` is the slice `execute` allows. There is no parent schema beside it to keep byte-identical. That is the adaptation of the parity rule: the parent's list stays the parent's, in Hermes, and this harness only has the fork's slice.

`skill_manage` on the slice does not write. A real write is chapter 25, and a reviewer there has to `view` first. This chapter's `execute` will call `skill_view` if the script says so and a root is provided. It will not call `Writer::patch`. The read-before-write rule is unchanged, because the patch function is unchanged. This file does not wrap it.

## Tests

Six tests, in `steps/ch26-reflection/crates/harness-core/tests/reflection.rs`. The package is the copied harness plus these tests. The nextest line is 139 passed.

`the_tenth_user_turn_forks_a_reviewer_with_memory` takes nine user turns and expects `finalize` to be `None`. The tenth turn's fork has `review_memory`, includes `memory` and `skill_view`, and has both intervals at 0. The next `finalize` is `None`.

`a_skill_nudge_does_not_hand_the_reviewer_memory` takes ten tool iterations. The fork's `review_memory` is false. `memory` is absent. `skill_manage` is present.

`the_reviewer_cannot_call_terminal_and_can_write_a_note` runs `terminal ls` then a memory line on one model. The result is `NotAllowed` for `terminal`. The notes vec is empty. `calls()` is 1. A second model with `memory prefer short notes` returns that argument and pushes it.

`cancel_before_the_next_tool_stops_the_review` cancels the fork, then `execute`s `memory should not land`. The result is `Cancelled`. The notes vec is empty.

`cancelling_a_running_review_returns_quickly` spawns the spinner and expects `cancel_and_join` to finish in under 50 milliseconds.

`the_demo_note_is_the_scripted_line` expects `demo()` to equal `prefer short notes`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch26-reflection
```

```text
Starting 139 tests across 33 binaries
     Summary [   0.295s] 139 tests run: 139 passed, 0 skipped
```

```bash
cargo run --locked -p ch26-reflection --bin ch26-reflection
```

```text
note: prefer short notes
forked: after 10 user turns
```

The binary counts ten user turns, forks, and runs the scripted memory line. No key.

| Benchmark | Mean | What it is |
|---|---|---|
| `spawn_reviewer` | MEAN_26_spawn_reviewer | build one reviewer fork |
| `cancel_review` | MEAN_26_cancel_review | set the cancel flag and join the spinner |

`spawn_reviewer` uses `iter_custom`. The closure receives `iters`. It allocates a `Vec` with that capacity, starts an `Instant`, pushes `spawn_reviewer(black_box(true))` in a loop, reads `elapsed`, drops the vec, and returns the duration. The drop sits after the instant is read. No thread starts. No model runs.

`cancel_review` uses `iter_batched` with `BatchSize::SmallInput`. The setup closure is `ReviewRun::spawn_spinner`. The timed closure is `cancel_and_join`. The worker sleeps 1 millisecond between flag checks, so the join waits out at most one of those sleeps.

The Mean cells are `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, run BENCH_RUN_URL, section `26`. The ids are `26/spawn_reviewer` and `26/cancel_review`. The old ids `24/spawn_reviewer` and `24/cancel_review` are aliases in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. Criterion also prints a `time:` line. That line is the slope. It is not the mean.

### Milestone demo

The `cargo run` above. The note is `prefer short notes`. The fork line says `after 10 user turns`.

## Pitfalls

A memory nudge and a skill nudge on the same `finalize` produce one fork, and that fork's slice includes `memory`. The skill-only slice is the one `spawn_reviewer(false)` builds. `finalize` passes `review_memory`, which is true whenever the memory flag was set.

`finalize` clears the flags before it returns. Calling it twice does not fork twice. Skipping it leaves the flags set, and a later `finalize` still returns a fork.

`execute` consumes a scripted line before it checks cancel. A cancelled fork returns `Cancelled` and does not push a note. The model's `calls` counter has moved.

`HERMES_CANCEL_TIMEOUT` is two seconds. `cancel_and_join` does not sleep that long. A worker that never loads the flag would not come back. The spinner loads it every millisecond.

`skill_manage` on the slice returns the formatted line. It does not write `SKILL.md`. A patch is still `Writer::patch`, and a reviewer there still has to `view`.

The note vec is a `Vec<String>` in the caller. It is not `MEMORY.md`. Chapter 21's scan does not run on it.

`run_loop` will not notice that you built a `Loop`. The counters move only when something calls `note_user_turn` and `note_tool_iter`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The new boundary is the fork's tool slice.

**Tool split.** `_review_tool_whitelist` is the slice. A skill-nudge fork does not include `memory`. `terminal` is `NotAllowed`. Write tools are not on either slice. `read_file` and `search_files` are on both.

**Intervals.** The fork stores 0 and 0. The reviewer does not carry the parent's interval of 10. `MEMORY_NUDGE_INTERVAL` and `SKILL_NUDGE_INTERVAL` stay on `Loop`, which is the parent schedule.

**Cancel.** `Fork::cancel` stores `true` with `Release`. `execute` loads it with `Acquire` before each tool. `ReviewRun::cancel_and_join` stores and joins. `HERMES_CANCEL_TIMEOUT` is `Duration::from_secs(2)`. This crate does not sleep it.

**The user turn.** `drive_loop` does not call `finalize`. The reviewer runs when something calls `execute`. The demo does. The user turn does not.

**Prompt cache.** The advertised list in Hermes stays the parent's. This harness does not advertise a schema. The fork only has its tool slice. The skill-index `Arc` from chapter 24 is still the index. A later skill is still a note.

**Left for later.** Running the reviewer inside `drive_loop` is deferred. E1 and E2 are still absent. Chapter 27 archives skills off the request path. It does not start this fork.

## Chapter summary

- **Counters.** Memory every 10 user turns. Skills every 10 tool iterations.
- **Fork.** `finalize` returns it. Both intervals on it are 0. `memory` is on the slice only when the memory nudge fired.
- **Refuse.** `terminal` is `NotAllowed`. A skill nudge has no `memory` tool.
- **Cancel.** A flag, then a join. The two-second Hermes wait is a constant this crate does not sleep.
- **Turn.** `run_loop` and `drive_loop` do not call `finalize`.
- **Time.** Building the fork measured **MEAN_26_spawn_reviewer**. Cancelling the spinner measured **MEAN_26_cancel_review**. Those are the means.

[personal note: a review that was worth running after the turn, not during it]

## What you implement

`my-harness/tests/ch26.rs` is the milestone. The test is `spawning_the_reviewer_does_not_run_it_on_the_user_turn`. It asserts `chapter_26_review_runs_on_user_turn()` is false. Spawning the reviewer does not run it on the user turn.

The function in `my-harness` is a stub. It ends in `todo!("chapter 26")` and panics until the assertion can pass. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 27 is the curator. It marks an agent-created skill stale after 14 days and archives it after 30. The request path only enqueues that pass. A write still goes through chapter 25's scan and approval gate.
