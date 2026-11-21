# Chapter 17: Built-in Tools II: Files, Terminal, Todo, and Clarify

An agent with a filesystem can clobber a file it never read, walk `..` out of the workspace, or run `rm -rf /` the moment a terminal exists. Chapter 16 can search and extract, with the URL checked first. It still can't read a file in the workspace, patch one, run a command through the approval floor, keep a todo list, or pause to ask the user a question. Those are the everyday tools, and each one has a way to go wrong that a bare function would miss.

The trick is that a path is canonicalized before it is trusted, and a write needs a stamp from a read. `Workspace::resolve` refuses a path that doesn't start with the workspace root, absolute paths included, and a relative `..` that leaves the root is an error. `read_file` records the mtime and the length. `write_file` of a path that was never read, or whose mtime moved, is refused as a stale write, so the model can't clobber a file it hasn't seen. `patch` replaces the one occurrence that matched. The read is capped. The terminal goes through chapter 11's `deny_tool` and then chapter 13's worker, rather than growing a second approval pipeline, with output cut head and tail and the foreground timeout bounded. `TodoStore` is agent-owned state, one list merged by id, with a revision so a stale write can be rejected, and it isn't a file in the user's repository. `clarify` pauses for a user, with a cap on questions and choices. A child agent may not call it. That block is the one chapter 29 will enforce again for a specialist: `clarify`, `delegate_task`, `memory`, `send_message`, and cron stay out of a child's hands.

The limit belongs up front. `read_file`, `write_file`, and `patch_file` aren't `Tool` implementations, so the registry doesn't gain those names for the model to call. The tests and the demo call the functions, the same way chapter 13's tests call the worker. `ClarifyTool` is the exception, because the pause has to sit inside `Tool::call` so the turn waits on it. A later wrapper that registers the file functions would call these same functions.

These tools are what a later skill actually does. A procedure that says "patch the file, then run the tests" is only real if patch and the terminal exist and are guarded, which is why chapter 25 will have something safe to call when it lets the agent write that procedure down. The stale-write refusal matters for a continuously improving harness in particular: an agent that "fixes" a file it didn't read will store a confident mistake, and reflection will then treat the mistake as experience.

Canonicalization is how `..` and symlinks stop being a string trick. A stamp is a struct you compare. The todo list is a `Vec` plus a revision counter. `clarify`'s pause is a future the test completes with a fake user reply.

The step is `steps/ch17-local-tools`, package `ch17-local-tools`, library `harness_core`, the module `local_tools` on a copy of chapter 16. `cargo run -p ch17-local-tools --bin ch17-local-tools` prints the escape, the stale write, the file still holding `one`, and the child refusal. `my-harness/tests/ch17.rs` is a write without a prior read.

**In this chapter**

- 17.1 The root is a canonical path
- 17.2 A write needs a stamp from a read
- 17.3 Read, write, and a patch that matches once
- 17.4 The terminal asks, then it runs
- 17.5 Truncation, in an order that matters
- 17.6 The list is not a file
- 17.7 Clarify is a pause inside `call`
- 17.8 A child does not get to ask
- 17.9 What the benches time

| Section | Step | What it adds |
|---|---|---|
| 17 | `steps/ch17-local-tools` | a workspace root, stale writes, one-match patches, a terminal through the existing gates, a todo list, a clarify pause |

> **How Hermes does it**
>
> I read `tools/file_tools.py` (`read_file_tool`, `write_file_tool`, `patch_tool`), `tools/file_tools_paths.py` (`_authoritative_workspace_root`, `_path_resolution_warning`), `tools/file_state.py` (`FileStateRegistry::check_stale`), `tools/file_tools_write_guards.py` (`_stale_overwrite_blocker`), `tools/terminal_tool.py` (`FOREGROUND_MAX_TIMEOUT`), `tools/terminal_tool_result.py` (`_truncate_head_tail`), `tools/tool_output_limits.py` (`DEFAULT_MAX_BYTES`), `tools/todo_tool.py` (`TodoStore`, `todo_tool`), `tools/clarify_tool.py` (`clarify_tool`), and `tools/delegate_tool_toolsets.py` (`DELEGATE_BLOCKED_TOOLS`) on Hermes `main`.
>
> **Adopted.** `_DEFAULT_MAX_READ_CHARS` is 100,000. A `write_file` of a file the task has not read, or whose mtime moved, is `stale_write_blocked`. `patch` is the edit that lands instead of clobbering the file. `FOREGROUND_MAX_TIMEOUT` is 600 seconds (`TERMINAL_MAX_FOREGROUND_TIMEOUT`). Terminal output is cut with `DEFAULT_MAX_BYTES` of 50,000. The todo list merges by id, caps at `MAX_TODO_ITEMS` of 256 and `MAX_TODO_CONTENT_CHARS` of 4,000, and carries a monotonic revision so a stale write can be rejected. The list is agent-owned state, not a path in the repo. `clarify` takes at most `MAX_QUESTIONS` of 5 and `MAX_CHOICES` of 4. `DELEGATE_BLOCKED_TOOLS` contains `clarify`, and also `delegate_task`, `memory`, `send_message`, and `cronjob_manage`. A child must not call `clarify`.
>
> **Adapted.** Hermes warns when a relative path resolves outside the workspace root, and treats an absolute path as the caller's way to name a file on purpose. This harness refuses the escape. An absolute path is allowed only when `canonicalize` lands under the root. The terminal does not grow a second approval pipeline and does not port Docker or sudo. It calls `approval::deny_tool` and then chapter 13's worker. `sleep-ms` goes through `sandbox::run_until`, which kills the child on timeout. Any other command uses `Worker::spawn`, one `accept`, and `quit`, because the worker does not exit after a single line and `run_until` would wait until the timeout. The clarify pause in tests is a `VecDeque` of replies on the tool. The parent loop is `ScriptedProvider` plus that queue. There is no stdin.
>
> **Deferred.** SSH path resolution, container path mirrors, protected-instruction approval prompts, Docker and sudo backends, and the gateway lifecycle block. That block stays chapter 39. Chapter 29's supervisor keeps its own copy of the blocked-tool list. This chapter enforces `clarify` on the tool itself so a mistaken registration still cannot ask.

## What this chapter decides

A path is allowed only when `canonicalize` lands under the workspace root. An existing file can be written or patched only after a read in this task, and only while the mtime and length still match. A patch that doesn't match exactly once leaves the file untouched. The terminal asks chapter 11 before it asks chapter 13's worker. The todo list never becomes a file. `clarify` waits inside `call`, and `depth >= 1` refuses before the queued reply moves.

## 17.1 The root is a canonical path

`Workspace` holds one `PathBuf`. `resolve` is the only way a caller names a file.

```rust
pub fn resolve(&self, raw: &str) -> Result<PathBuf, String> {
    let root = fs::canonicalize(&self.root).map_err(|err| err.to_string())?;
    let candidate = {
        let path = Path::new(raw);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            root.join(path)
        }
    };
    let resolved = if candidate.exists() {
        fs::canonicalize(&candidate).map_err(|err| err.to_string())?
    } else {
        let parent = candidate
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .ok_or_else(|| format!("path escapes the workspace root: {raw}"))?;
        let name = candidate
            .file_name()
            .ok_or_else(|| format!("path escapes the workspace root: {raw}"))?;
        fs::canonicalize(parent)
            .map_err(|err| err.to_string())?
            .join(name)
    };
    if !resolved.starts_with(&root) {
        return Err(format!("path escapes the workspace root: {raw}"));
    }
    Ok(resolved)
}
```

The root is canonicalized first, so a relative root and a symlink root become the same prefix. A relative request is joined onto that prefix. `..` is not interpreted by string chopping. `canonicalize` resolves it, and `starts_with` then answers whether the result is still inside the root.

A path that does not exist yet cannot be canonicalized. The file's parent is canonicalized instead, and the final component is joined back on. `inside/new.txt` works when `inside` exists. `../outside.txt` canonicalizes the parent to a directory above the root, `starts_with` fails, and the error contains `escapes`. That is the demo's first line.

> **Rust sidebar.** `Path::is_absolute` is true for a path that starts at the filesystem root. `root.join(path)` on a relative path appends components. `starts_with` on `Path` compares components, not a string prefix, so `/workspace` does not count as a prefix of `/workspace-other`. `canonicalize` returns an error if the path is missing, which is why the new-file branch exists. `map_err(|err| err.to_string())` turns `std::io::Error` into the `String` this module uses for every refusal. The model reads that string. It does not read the `io::Error` type.

An absolute path skips the join. `/etc/passwd` does not start with the workspace root, so it is refused. An absolute path that does land inside the root is allowed. That is the adaptation of Hermes: absolute is not a free pass, and a relative escape is an error rather than a warning.

A symlink inside the root that points outside fails the same check. `canonicalize` follows it. The followed path does not start with the root. The unit test does not build a symlink. The `..` case is the one it asserts.

## 17.2 A write needs a stamp from a read

`FileState` is a `HashMap` from the canonical path to a `Stamp`. The stamp is the mtime and the length.

```rust
pub fn check_stale(&self, path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    let Some(seen) = self.reads.get(path) else {
        return Err("stale_write_blocked: file was not read in this task".into());
    };
    let current = stamp_of(path)?;
    if current != *seen {
        return Err("stale_write_blocked: file changed since it was read".into());
    }
    Ok(())
}
```

A path that does not exist yet has nothing to clobber. Creating it is allowed, and that is why the missing-file arm returns `Ok` before the map lookup. An existing file with no entry is `stale_write_blocked: file was not read in this task`. An entry whose mtime or length moved is `stale_write_blocked: file changed since it was read`.

> **Rust sidebar.** `if let Some(seen) = ... else { return Err(...) }` is the form for "I need the value, and the other case is a return." `Stamp` derives `PartialEq`, so `current != *seen` compares both fields. `*seen` dereferences the reference `get` returned. `SystemTime` equality is the mtime compare. A length change with the same mtime still fails, because both fields are in the struct.

`stamp_of` reads `metadata`. A platform that cannot report mtime contributes `UNIX_EPOCH` rather than failing the read. Two such stamps still compare equal to each other, and a length change is still visible.

`read_file` resolves, reads the bytes as UTF-8, and calls `note`, which stores the stamp. `write_file` and `patch_file` call `check_stale` before they touch the file. After a successful write or patch, `note` runs again, so the next write in the same task sees the new stamp and is not stale.

The unit test writes `inside/note.txt` with the contents `one`, then calls `write_file` with `two` on a fresh `FileState`. The error contains `stale_write_blocked`. It then reads, writes `two`, and patches. The file on disk ends as `three`.

## 17.3 Read, write, and a patch that matches once

`read_file` returns the text after a character cap. `MAX_READ_CHARS` is 100,000, Hermes `_DEFAULT_MAX_READ_CHARS`.

```rust
pub fn read_file(
    workspace: &Workspace,
    state: &mut FileState,
    raw: &str,
) -> Result<String, String> {
    let path = workspace.resolve(raw)?;
    let text = fs::read_to_string(&path).map_err(|err| err.to_string())?;
    state.note(&path)?;
    Ok(cap_chars(&text, MAX_READ_CHARS, "read"))
}
```

`cap_chars` collects `chars()` so the cut cannot land in the middle of a UTF-8 character. Over the cap, the footer is `[TRUNCATED read at 100000 chars]`. The stamp is taken on the file, not on the truncated string. A later write still compares mtime and length against the real file.

`write_file` checks stale, creates the parent with `create_dir_all` when there is one, writes the bytes, and notes the new stamp.

`patch_file` is the edit. An empty `old` is refused before any path work, because replacing the empty string would match every file in a way that is not an edit. The path is resolved, the stale check runs, and the file is read. `matches(old).count()` must be exactly one. Zero matches and two matches both return `patch expected one match, found N` and do not call `fs::write`. One match uses `replacen(old, new, 1)`, writes, and notes.

> **Rust sidebar.** `str::matches` yields non-overlapping hits. `count` consumes that iterator. `replacen` stops after the first replacement, which matches the count we just required. The file is left untouched on the error path because `fs::write` is below the `if count != 1` return.

These three functions are not `Tool` implementations. The registry in this chapter does not gain `read_file` as a name the model calls. The tests and the demo call the functions, the same way chapter 13's tests call the worker. `ClarifyTool` is the exception, because the pause has to sit inside `Tool::call` so `turn_from_inbox` waits on it. A later wrapper that registers the file functions would call these same functions. It would not grow a second path check.

## 17.4 The terminal asks, then it runs

`run_terminal` takes the chapter 11 `Session`, the command, a timeout, and the path of the sandbox worker.

```rust
pub fn run_terminal(
    session: &mut Session,
    command: &str,
    timeout: Duration,
    worker: &Path,
) -> TerminalOutput {
    let arguments = json!({"command": command});
    if let Some(message) = approval::deny_tool(session, "terminal", &arguments) {
        return TerminalOutput {
            denied: true,
            text: message,
        };
    }
    let timeout = if timeout > FOREGROUND_MAX_TIMEOUT {
        FOREGROUND_MAX_TIMEOUT
    } else {
        timeout
    };
    // ... worker, then truncate_terminal
}
```

`deny_tool` returns `None` when the command may run, and `Some(message)` when it may not. The name passed here is `terminal`. Chapter 11 only inspects `terminal` and `execute_code`. Any other name skips the floor. That is why this call uses `terminal` and not a new name. `execute_code` still goes through `check_execute_code` inside `deny_tool`. This function does not replace that path.

A denial returns immediately. `denied` is true. The worker path is never opened. The integration test points the worker at `/no/such/sandbox-worker` and sends `rm -rf /`. Chapter 11's hardline answers with a message that starts `BLOCKED (hardline)`. The test asserts `denied`, asserts the text contains `BLOCKED`, and asserts it does not contain `echo hello ran`, which is the line the worker prints for a command it actually ran. A bug that skipped `deny_tool` would try to spawn a binary that is not there. It would not print the worker's success line, and `denied` would be false.

`FOREGROUND_MAX_TIMEOUT` is 600 seconds. A caller who passes a longer `Duration` is clamped. The timeout argument is still what `sleep-ms` waits on after that clamp.

The worker split is the adaptation. `sandbox::run_until` waits until the worker process exits. The chapter 13 worker exits on `quit`, not after one job. Sending `echo hello` through `run_until` would sit until the timeout and then report a timeout for a command that had already answered. So `run_terminal` branches:

- A command that starts with `sleep-ms ` uses `run_until`. That is the worker's slow job, and `run_until` kills the child when the timeout fires. `SandboxError::Timeout` becomes the text `timeout`.
- Every other command spawns a `Worker`, calls `accept` with the line, then `quit`. `accept` reads one reply. `quit` tells the process to exit so the test does not leave it behind.

> **Rust sidebar.** `Worker::spawn(worker).and_then(|mut child| { ... })` is "spawn, and if that succeeded, run this closure." `and_then` on `Result` does not run the closure on `Err`. `child.accept(command)?` returns the `Err` from the closure, which `and_then` then returns as the outer `Err`. `let _ = child.quit()` ignores a quit failure. The process has already given us the line. A failed quit is not a failed command.

The allowed-command test uses `env!("CARGO_BIN_EXE_sandbox-worker")`, the path Cargo sets when the integration test builds the bin. `echo hello` comes back containing `ok echo hello`, and `denied` is false. `sleep-ms 5000` with a 200 millisecond timeout comes back as the text `timeout`.

## 17.5 Truncation, in an order that matters

`truncate_terminal` runs three cuts. Line length, then line count, then bytes.

`DEFAULT_MAX_LINE_LENGTH` is 2,000 characters. A longer line keeps that many `char`s and appends `…`. `DEFAULT_MAX_LINES` is 2,000. Over that, the first half and the last half stay, and the middle becomes `… [N lines omitted] …`. `DEFAULT_MAX_BYTES` is 50,000. Over that, the string is split near the middle on character boundaries and the gap is `… [N bytes omitted] …`.

```rust
fn floor_char_boundary(text: &str, mut index: usize) -> usize {
    if index > text.len() {
        return text.len();
    }
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}
```

`str::len` is bytes. Slicing `&joined[..head]` panics when `head` is in the middle of a UTF-8 character. `is_char_boundary` is the standard-library check. `floor_char_boundary` walks backward to a safe index. `ceil_char_boundary` walks forward. The omitted count is the byte gap between those two indexes.

The order is why a single 60,000-character line does not contain `bytes omitted`. The line-length cut runs first, the result is under 50,000 bytes, and the byte cut never fires. The test asserts an ellipsis and a character count under 3,000. Six hundred lines of one hundred `y`s are under both the line-length cap and the line-count cap, and over the byte cap. That fixture is what contains `bytes omitted`.

## 17.6 The list is not a file

`TodoStore` holds a `Vec<TodoItem>` and a `revision` that starts at 0. Nothing in `apply` calls `fs::write`. The workspace root is not an argument.

```rust
pub fn apply(
    &mut self,
    incoming: Vec<TodoItem>,
    merge: bool,
    expected_revision: Option<u64>,
) -> Result<u64, String> {
    if let Some(expected) = expected_revision {
        if expected != self.revision {
            return Err(format!(
                "stale todo revision: expected {expected}, have {}",
                self.revision
            ));
        }
    }
    let incoming: Vec<TodoItem> = incoming.into_iter().map(clip_item).collect();
    if merge {
        for item in incoming {
            if let Some(existing) = self.items.iter_mut().find(|row| row.id == item.id) {
                *existing = item;
            } else {
                self.items.push(item);
            }
        }
    } else {
        self.items = incoming;
    }
    if self.items.len() > MAX_TODO_ITEMS {
        self.items.truncate(MAX_TODO_ITEMS);
    }
    self.revision = self.revision.saturating_add(1);
    Ok(self.revision)
}
```

`expected_revision` is checked before any item moves. A mismatch returns `stale todo revision` and does not bump the counter. The caller's list is unchanged. `None` skips the check, which is how a caller who does not care about lost updates can still write. The tests pass `Some`.

`merge` false replaces the vector. `merge` true walks the incoming items. A matching `id` is overwritten in place. An unknown `id` is pushed. `clip_item` keeps at most 4,000 characters of `content`, counted with `chars()`, not bytes. After the merge or replace, a list longer than 256 items is `truncate`d. Items past the cap are dropped. Then `revision` becomes `saturating_add(1)`, so it cannot wrap to 0 on overflow.

> **Rust sidebar.** `iter_mut().find` returns `Option<&mut TodoItem>`. `*existing = item` writes through that mutable reference and moves `item` in. The `else` branch pushes because `find` did not take ownership of `item` on the `None` path. `Vec::truncate` keeps the prefix and drops the tail. It does not allocate.

The unit test applies one item at revision 0 with `merge` false. The revision is now 1. A merge that still says `Some(0)` errors, and the content is still `first`. A merge that says `Some(store.revision())` updates that one id to `second` and leaves the length at 1. The revision is 2.

## 17.7 Clarify is a pause inside `call`

`parse_questions` reads a JSON object with a `questions` array. Empty is an error. More than 5 is an error. Each entry's `text` is a string, defaulting to empty when the field is missing. `choices` keeps at most 4 strings, each clipped to `MAX_CHOICE_CHARS` (8,000), and drops empty ones. Extra choice strings past the fourth are ignored rather than failing the call. A sixth question fails the whole call. That matches the Hermes caps on questions, and it is stricter than Hermes only in that this parser does not try to salvage a longer list.

`ClarifyTool` stores three things: the `depth`, a `Mutex<VecDeque<String>>` of replies, and an `AtomicUsize` counting how many times `call` got past the child check.

```rust
async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
    let schema = self.schema();
    check_arguments(&schema, &arguments)?;
    if let Err(message) = child_may_call("clarify", self.depth) {
        return Err(ToolError::Failed { message });
    }
    let questions =
        parse_questions(&arguments).map_err(|message| ToolError::Failed { message })?;
    self.asked.fetch_add(1, Ordering::SeqCst);
    let answer = self
        .answers
        .lock()
        .expect("answers")
        .pop_front()
        .unwrap_or_else(|| "(no user reply)".into());
    let rendered = questions
        .iter()
        .map(|question| question.text.clone())
        .collect::<Vec<_>>()
        .join(" | ");
    Ok(ToolResult::text(format!("{rendered} => {answer}")))
}
```

`Tool::call` takes `&self`. The registry holds `Arc<dyn Tool>`, so the tool cannot ask for `&mut self`. The queue has to move anyway. `Mutex` is that interior mutability. `pop_front` takes the oldest reply. An empty queue becomes the text `(no user reply)`, so a test that forgot to enqueue an answer still finishes the turn and the missing reply is visible in the tool message.

> **Rust sidebar.** `Mutex<T>` lets many shared references take turns mutating `T`. `lock` returns a guard. The guard's destructor unlocks. `expect` on the lock turns a poisoned mutex into a panic. Poison means another thread panicked while holding the lock. This harness treats that as a bug. `AtomicUsize` is a counter that can be read with `&self` and no lock. `Ordering::SeqCst` is the strongest ordering. The counter is a test probe, not a hot statistic, and the strong ordering keeps the assertion boring. `VecDeque` is a queue. `pop_front` is O(1). A `Vec` would also work at this size. The deque matches "the next reply."

`kind` is `ToolKind::Read`. Clarify does not write the workspace. The schema name is `clarify`, the required key is `questions`, and `additionalProperties` is false, so a stray field fails in `check_arguments` before the child check.

The parent integration test builds `ClarifyTool::new(0, vec!["the left one"])`, registers it, and scripts two replies: a tool call named `clarify` with one question, then the text `done`. `turn_from_inbox` runs. The stop reason is `Finished`. `asked` is 1. `pending` is 0. The tool message in the session contains `the left one`. The rendered shape is `which side? => the left one`. The fake model never blocks on a person. The fake user is the string that was in the queue when `call` ran.

That is the loop pause. `call` is a future the turn awaits. The future ends when the queue yields. In a process with a real user, the same spot is where a prompt would wait. This chapter's tests do not open a prompt. Chapter 11 already owns prompts, and this tool does not install a second one.

## 17.8 A child does not get to ask

```rust
pub const BLOCKED_CHILD_TOOLS: &[&str] = &[
    "delegate_task",
    "clarify",
    "memory",
    "send_message",
    "cronjob_manage",
];

pub fn child_may_call(tool: &str, depth: u8) -> Result<(), String> {
    if depth >= 1 && BLOCKED_CHILD_TOOLS.contains(&tool) {
        return Err(format!("BLOCKED: a child agent may not call {tool}"));
    }
    Ok(())
}
```

`depth` is a field on the tool, chosen when the tool is built. It is not an argument the model can pass. A parent is `ClarifyTool::new(0, ...)`. A child is `ClarifyTool::new(1, ...)`. `child_may_call` runs after the schema check and before `parse_questions` and before `pop_front`. The child's queue stays put. `asked` stays 0.

The integration test builds that child with the reply `should-not-be-read` and calls it with a valid question. The error contains `child agent may not call clarify`. `pending()` is still 1. The same test then runs the parent loop, so one function proves both sides: the child does not consume, and the parent does.

`BLOCKED_CHILD_TOOLS` lists the other Hermes names so a later tool in this process can call `child_may_call` without inventing a second list. Chapter 29's supervisor still has its own `DELEGATE_BLOCKED_TOOLS` and still refuses to hand those names to a child. This chapter does not delete that. A child who never receives the tool, and a tool that refuses when `depth >= 1`, are both checks. The demo prints the refusal:

```text
child: BLOCKED: a child agent may not call clarify
```

`contains` takes `&&str` because the array's items are already `&str`. `tool` is `&str`. The extra `&` is "a reference to this string," which is what `[&str]::contains` wants.

## 17.9 What the benches time

`patch_apply` creates a temp file `alpha beta`, reads it so the stamp exists, and then, inside the sample, writes the original bytes back, builds a fresh `FileState`, reads again, and patches `alpha` to `gamma`. The rewrite is inside the timer because a patch of an already-patched file would miss on the second iteration and the bench would time an error path. `black_box` wraps the workspace, the state, the path, and both strings.

`todo_merge` builds a store, applies one item at revision 0, then merges an update at revision 1. The store is built inside the sample so each iteration starts from an empty list. The merged content is `black_box`ed.

Both means below are `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39114427940](https://github.com/agnathan/rust-agent-harness/actions/runs/39114427940), section `17`.

| Bench | Mean |
|---|---|
| `17/patch_apply` | 255.1 µs |
| `17/todo_merge` | 131.2 ns |

The samples do not spawn the worker and do not read an API key. The terminal test is a nextest case, not a bench, because it starts a process.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch17-local-tools
```

```text
Starting 106 tests across 26 binaries
     Summary [   0.275s] 106 tests run: 106 passed, 0 skipped
```

"Across 26 binaries" is nextest's count of test targets. Chapter 16's package is 25. This package adds `tests/local.rs`. The CLI package is separate and is not in this number.

```bash
cargo run -p ch17-local-tools --bin ch17-local-tools
```

```text
escape: true
stale: true
read: one
child: BLOCKED: a child agent may not call clarify
```

`escape` is the `../outside.txt` refusal. `stale` is a write of `note.txt` before any read. `read` is the seeded contents, still `one`, because the stale write did not land. `child` is `child_may_call("clarify", 1)`. The binary does not start the worker and does not ask a person.

## Pitfalls

Joining a relative path and then scanning the string for `..` misses a symlink, and it misses an absolute path such as `/etc/passwd`. `canonicalize` plus `starts_with` is the check. A warning that still returns the path isn't this function: Hermes warns, and this chapter refuses.

A patch that wrote anyway when the count wasn't one would edit the wrong occurrence, or invent a match that was never there. `fs::write` stays below the count check, so zero matches and two matches leave the file alone.

If the todo revision moved before the equality check, a stale writer would advance the counter and then fail, and the next honest writer would be holding the wrong expected number. The check returns first, and the list stays as it was.

`depth` in the tool arguments would let the model claim to be the parent. The depth is fixed when the tool is constructed, which is why a child built with `ClarifyTool::new(1, ...)` can't talk its way into the queue.

## Guardrails

**Workspace root.** `Workspace::resolve` canonicalizes and refuses a path that does not start with the root. Absolute paths are held to the same prefix. A relative `..` that leaves the root is an error.

**Stale write.** `write_file` and `patch_file` call `check_stale`. An existing file needs a stamp from `read_file` in this task, and the mtime and length have to match. A new file may be created.

**Size caps.** Reads stop at 100,000 characters. Terminal lines stop at 2,000 characters, 2,000 lines, and 50,000 bytes, in that order. Todo content stops at 4,000 characters and the list at 256 items. Clarify accepts at most 5 questions and 4 choices, with each choice clipped at 8,000 characters.

**Terminal floor.** `run_terminal` calls `approval::deny_tool` with the name `terminal` before any worker. The timeout cannot exceed 600 seconds. Output goes through `truncate_terminal`. `execute_code` is still chapter 13's script gate, reached from the same `deny_tool`.

**Todo is agent state.** `TodoStore::apply` does not write the workspace. A mismatched `expected_revision` changes nothing.

**Clarify is a pause, and a child may not call it.** The parent turn awaits `ClarifyTool::call`. Tests supply the reply on the tool. `depth >= 1` returns `BLOCKED: a child agent may not call clarify` before the queue moves. Chapter 29 keeps the same block on the supervisor side.

**Left for later.** SSH path resolution, container mirrors, protected-instruction prompts, Docker and sudo, and the gateway lifecycle block in chapter 39.

## Chapter summary

- **Root.** A path is allowed when canonicalization lands inside the workspace, and `..` that leaves is an error.
- **Stamp.** A write or a patch of an existing file needs a matching read from this task.
- **Terminal.** Chapter 11 sees the command before chapter 13's worker does. `rm -rf /` comes back denied.
- **Pause.** `clarify` waits inside `call`. A child at `depth >= 1` doesn't get to ask.
- **Time.** One patch measured **255.1 µs**. A todo merge measured **131.2 ns**.

## What you implement

`my-harness/tests/ch17.rs` calls `chapter_17_stale_write_refused(false)` and expects `true`. The stub in `my-harness/src/milestones.rs` is `todo!("chapter 17")`. The finished check refuses a write when `read_first` is false. A write after a read in the same `FileState` is the other arm. The step crate is the reference, as the preface says.

## What's next

Chapter 18 copies this harness and adds provider adapters: an Anthropic body, a Bedrock body, an OpenRouter route, and a key rotation that keeps the same client. `web` and `local_tools` stay on the crate. The model behind those adapters is still the scripted one in the tests that do not opt into a live key.
