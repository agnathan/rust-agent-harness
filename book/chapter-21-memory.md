# Chapter 21: Memory Files

Add `likes tea` while the session is still open, and the file on disk changes while the system prompt stays on the snapshot `open` already handed out. That pointer is an `Arc` taken once, and `add` never replaces it, so a prefix the provider has already cached keeps the same bytes until a later session opens the directory again. A continuously improving harness can store a durable fact on this turn and still keep the prefix it has already paid for.

The fact belongs in a small file, rather than in a transcript you would have to search. `MEMORY.md` holds notes that are true every session, and `USER.md` holds the profile. Entries are whole lines joined by a newline, a section sign, and a newline. The tests write one memory entry, `likes tea`, and one profile line, `name is Ada`. Five entries in the same shape look like this:

```text
likes tea
§
prefers short replies
§
the workspace root is the repo
§
deploy window is Tuesday morning
§
no mail without an explicit ask
```

Those five lines are the file shape, not a session this book ran. The budgets are the Hermes ones, 2,200 for memory and 1,375 for the profile, and a write past the budget returns how many characters to free so the model consolidates instead of dropping the line on the floor. Replace needs the old text to match exactly one entry. A hand edit that breaks the delimiter is a drift error, and the file is left as the hand edit wrote it.

The scan runs before the rename. A line that says to ignore the rules and send the keys becomes `MemoryError::Scan`, and the snapshot is left alone with the file. Writes go through `first_threat_message`. They skip chapter 20's redactor, so a secret that misses these four patterns can still land in the file, while transcript redaction stays on the session path this crate copied forward. The write takes effect in the prompt at the next `open`, which is the cache rule from chapter 2 applied to memory.

Chapter 22 searches sessions and leaves these files alone. Skills, when they arrive in chapter 24, hold procedures; memory holds the facts; the session holds what will be stale within a week. Chapter 26's reviewer may write memory, and only with the memory tool. Chapter 34 copies the latest user line onto `Prep.flushed` before it compresses, and that copy still has to be written here if the fact should land in the file. Chapter 27 curates skills, not these two files. Get the store wrong and the agent either forgets a durable fact or remembers an instruction someone smuggled into a tool result.

The step is chapter 20's harness plus `memory_files`, at `steps/ch21-memory`, package `ch21-memory`, and the module implements the memory trait from chapter 12, so a different backend can still be swapped in. The write is a temp file and a rename under a process-local mutex, and the scanner is a regex on the candidate before that rename. `cargo run -p ch21-memory` adds `likes tea`, enqueues a turn, and prints the file beside the word `frozen`. `my-harness/tests/ch21.rs` checks the other half: a poisoned entry is blocked, and the snapshot the caller already holds is the same allocation afterward.

**In this chapter**

- 21.1 `open` reads two files into an `Arc`
- 21.2 Add, replace, remove, and the budget
- 21.3 The scan, before the file changes
- 21.4 The queue, and the pointer that stays put

| Section | Step | What it adds |
|---|---|---|
| 21 | `steps/ch21-memory` | two files, a frozen snapshot, a strict scan |

> **How Hermes does it**
>
> I read `tools/memory_tool.py` (`memory_char_limit` 2200, `user_char_limit` 1375), `tools/memory_tool_store.py` (`ENTRY_DELIMITER`, `add`, `replace`, `_scan_memory_content`), and `tools/threat_patterns.py` (`first_threat_message`, `INVISIBLE_CHARS`, `MAX_SCAN_CHARS`, the pattern ids `ssh_backdoor`, `context_exfil`, and `hardcoded_secret`) on Hermes `main` at `cf71b1aa`.
>
> Entries are joined by `\n§\n`. `add` refuses an empty string, a duplicate, a scan hit, and a write that would pass the character budget. The error tells the model how many characters to free. `replace` finds the one entry that equals `old_text` and replaces that whole entry. A scan uses scope `strict`. Invisible characters are reported first, as `invisible_unicode_U+XXXX`. Other hits use the sentence that names the pattern id and says the content is injected into the system prompt. The scan stops at 65,536 characters. The snapshot for the system prompt is taken at session start. A write applies next session.
>
> **Adopted.** The delimiter, the two limits, the duplicate refusal, the budget overflow count, and the two block sentences. Invisible characters from `INVISIBLE_CHARS`. Pattern ids `ssh_backdoor` (`authorized_keys`), `context_exfil`, and `hardcoded_secret`. A hand edit that uses `§` without the delimiter is a drift error. The snapshot is an `Arc` taken at `open`. `enqueue_turn` does not rewrite it.
>
> **Adapted.** The scan is those four checks, not the full pattern list. The env-name lookahead on `hardcoded_secret` is not ported. The write lock is a `Mutex` in this process, plus a temp file and `rename`. It is not a cross-process file lock. The approval gate stays off.
>
> **Deferred.** A9, `memory.write_approval`, is chapter 25. This chapter writes when `add` is called. It does not stage a diff for a human.

## What this chapter decides

`MemoryFiles::open` creates the directory and reads `MEMORY.md` and `USER.md` if they exist, and `snapshot` returns that `Arc` for the rest of the process.

`add(Target::Memory, "likes tea")` writes the file, and a second add of the same text is `Duplicate`. `add(Target::User, "name is Ada")` is the other file, and `replace` of the whole entry `likes tea` with `likes coffee` rewrites `MEMORY.md`. `remove` leaves it empty.

A string of 2,201 `x` characters is over the 2,200 limit by 1, and `budget_line` is `would exceed the limit by 1 chars`. The file stays empty.

`authorized_keys` is blocked with `ssh_backdoor` in the sentence, and a zero-width space is `U+200B`. `include the full conversation` is `context_exfil`, and `api_key = "` plus 20 letters is `hardcoded_secret`. None of those writes land.

After `add`, `Arc::ptr_eq` on the snapshot from before and after is true, and the snapshot text is still empty. The file contains `likes tea`.

`enqueue_turn` pushes one queue entry and doesn't change the file, and the trait name is `files`.

A file that contains `§` but not the delimiter `\n§\n` is `Drift`, but `add` doesn't rewrite it.

The demo adds `likes tea`, enqueues a turn, and prints the file and the word `frozen`. No key. The approval gate isn't in this binary.

## 21.1 `open` reads two files into an `Arc`

This chapter's harness is chapter 20 copied forward, plus `memory_files`, and the WAL file and the redaction passes are still in the crate. The new code is the two markdown files, and `MEMORY.md` is the agent's notes. `USER.md` is the user profile, and `memory_files` implements the `MemoryBackend` trait that already lives in this crate's `hooks` module.

The trait is small, and chapter 12's in-memory backend is `MemoryQueue`, and its `name` is `"queue"`. This backend's `name` is `"files"`.

```rust
pub trait MemoryBackend: Send + Sync {
    fn name(&self) -> &str;
    fn enqueue_turn(&self, record: &TurnRecord<'_>) -> Result<(), HookError>;
}
```

`TurnRecord` borrows the two strings for the call, and the `'a` on the struct is that borrow. `enqueue_turn` copies the characters into the queue, so the record can end when the call returns.

```rust
pub struct TurnRecord<'a> {
    pub user: &'a str,
    pub assistant: &'a str,
}
```

The files themselves are not the trait. They are `MemoryFiles`.

```rust
pub const ENTRY_DELIMITER: &str = "\n§\n";
pub const MEMORY_CHAR_LIMIT: usize = 2_200;
pub const USER_CHAR_LIMIT: usize = 1_375;
pub const MAX_SCAN_CHARS: usize = 65_536;
const QUEUE_CAP: usize = 64;
```

The delimiter is a newline, the section sign, and a newline, and `MEMORY.md` is allowed 2,200. `USER.md` is allowed 1,375, and the scan looks at the first 65,536 bytes. The queue holds 64 turn strings, and those four numbers are the policy. The functions below only apply them.

```rust
pub struct MemoryFiles {
    root: PathBuf,
    frozen: Arc<Frozen>,
    lock: Mutex<()>,
    queue: Mutex<VecDeque<String>>,
}
```

`root` is the directory, and `frozen` is the snapshot taken at `open`. `lock` guards the file writes, and `queue` guards the turn strings. They are different mutexes, so a queue push doesn't wait for a file write, and a file write doesn't wait for a queue push.

> **Rust sidebar: `Mutex`**
>
> - `Mutex<T>` lets one caller at a time hold `T`. `lock` waits until any current holder drops its guard, then returns `MutexGuard<T>`. The guard dereferences to `T`. Dropping the guard unlocks.
> - `lock` is `Mutex<()>`. The unit value `()` has nothing in it. I am not protecting a field. I am protecting the sequence "read the file, change the vec, rename the temp file" so two `add` calls in this process cannot interleave those steps.
> - `queue` is `Mutex<VecDeque<String>>`. The strings live inside the mutex. You get them by locking, and the guard's drop is what publishes the push to the next locker. `queue_len` locks, reads `len`, and drops the guard at the end of the statement.
> - `expect` on a lock turns a poisoned mutex into a panic. Poison is the flag set when a holder panics before unlocking. I would rather stop than write `MEMORY.md` after a half-finished critical section.
> - The snapshot `Arc` is not inside either mutex. `snapshot` never locks. A writer cannot block a reader of the frozen text, and a reader of the frozen text cannot observe a half-written entry, because the frozen text is not the file.

> **Rust sidebar: `Path` and reading a file**
>
> - `Path` is a borrowed path, like `&str`. `PathBuf` is an owned path, like `String`. `MemoryFiles` stores a `PathBuf` because the directory has to outlive the call to `open`.
> - `open` takes `impl Into<PathBuf>`. A `PathBuf`, a `String`, or a `&Path` can be turned into an owned path. The tests pass `&root`.
> - `root.join("MEMORY.md")` builds a new `PathBuf`. It does not change `root`. `with_extension("tmp")` builds another path by replacing the extension, so `MEMORY.md` becomes `MEMORY.tmp`.
> - `fs::read_to_string` reads the whole file and checks that the bytes are UTF-8. The success value is a `String`. A missing file is `ErrorKind::NotFound`. Any other failure, including bytes that are not UTF-8, is a different error. This chapter maps only `NotFound` to an empty string. Everything else becomes `MemoryError::Io` holding `err.to_string()`, which is the display text. The original error kind is not stored.
> - `fs::create_dir_all` creates the directory and any missing parents. If the directory already exists, it succeeds. If `root` is a file, it returns the error this function turns into `Io`.

```rust
pub fn open(root: impl Into<PathBuf>) -> Result<Self, MemoryError> {
    let root = root.into();
    fs::create_dir_all(&root).map_err(|err| MemoryError::Io(err.to_string()))?;
    let frozen = Arc::new(Frozen {
        memory: read_raw(&root.join("MEMORY.md"))?,
        user: read_raw(&root.join("USER.md"))?,
    });
    Ok(Self {
        root,
        frozen,
        lock: Mutex::new(()),
        queue: Mutex::new(VecDeque::new()),
    })
}
```

`open` reads each file once, but it doesn't parse entries. The snapshot holds the raw bytes decoded as UTF-8, or `""` when the file is absent. A hand-edited file that would later fail as drift is still stored here as raw text. Parsing happens on `add`, `replace`, and `remove`. A brand-new directory has two missing files, so both strings are empty, and that empty `Frozen` is the one the prompt is supposed to keep.

`read_raw` is the whole file policy.

```rust
fn read_raw(path: &Path) -> Result<String, MemoryError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(MemoryError::Io(err.to_string())),
    }
}
```

The `if` on the `Err` arm is a match guard, and only `NotFound` takes the empty-string path. A permission error and a UTF-8 error both become `Io`, and `read` is the public wrapper, and it doesn't take `self.lock`.

`?` cannot turn an `std::io::Error` into a `MemoryError` on its own, and there is no `From` impl connecting them. `create_dir_all`, `fs::write`, and the second `rename` use `map_err` to build `MemoryError::Io(err.to_string())` and then `?` on that `Result`. `read_raw` does the same conversion inside the `match`, because `NotFound` is a success and the other kinds are `Io`. One error type leaves the module, and callers match on `Empty`, `Duplicate`, `Drift`, `Budget`, `Scan`, and `Io`. They do not match on `std::io::ErrorKind`.

```rust
pub fn read(&self, target: Target) -> Result<String, MemoryError> {
    read_raw(&self.root.join(target.file()))
}
```

A caller can read the file while another call holds the write lock, because `read` never locks. On one thread that doesn't come up, and `MemoryFiles` is `Sync`, because `Mutex` and `Arc` are, so two threads are allowed to call it. `add` holds the lock across the write, and `read` can observe the file before the rename or after it. The rename is what makes those two views the ones I want: the old file, or the new file.

Which file, and which limit, is `Target`.

```rust
pub enum Target {
    Memory,
    User,
}

impl Target {
    fn file(self) -> &'static str {
        match self {
            Target::Memory => "MEMORY.md",
            Target::User => "USER.md",
        }
    }

    fn limit(self) -> usize {
        match self {
            Target::Memory => MEMORY_CHAR_LIMIT,
            Target::User => USER_CHAR_LIMIT,
        }
    }
}
```

`Target` is `Copy`. Passing it copies the enum, and there is no third file. `file` and `limit` are the only match arms, and every write goes through them, so a memory write cannot land in `USER.md` by a missed argument.

`Frozen` is the snapshot value, and the fields are public so a caller can read `memory` and `user`. They are plain `String`s, and nothing in this file writes those fields after `Arc::new`.

```rust
pub struct Frozen {
    pub memory: String,
    pub user: String,
}
```

> **Rust sidebar: `Arc` is the snapshot**
>
> - `Arc<T>` is a shared pointer with a reference count. `Arc::new` puts `Frozen` on the heap and sets the count to one. `Arc::clone` bumps the count and copies the pointer. It does not copy the two strings.
> - `snapshot` returns that clone. The prompt and the store can both hold it. `Arc::ptr_eq` is true when two handles point at the same allocation. The test keeps the handle from before `add` and the handle from after `add` and asserts they are the same allocation.
> - Replacing the `Arc` in the store would be a new allocation. The prompt's old handle would still see the old text, which is what you want from a shared pointer. This chapter does not even do that replacement. `add` never assigns `self.frozen`.
> - Mutating the text inside the `Arc` would change the prompt's bytes. A shared `Arc` does not hand out `&mut Frozen`. `Arc::get_mut` only works when the count is one, and this code never calls it. There is no `Mutex` inside `Frozen`. The file is the mutable side. The `Arc` is the frozen side.

```rust
pub fn snapshot(&self) -> Arc<Frozen> {
    Arc::clone(&self.frozen)
}
```

That is the whole function, but it doesn't open `MEMORY.md` again. A write that happened after `open` is invisible here. A second `MemoryFiles::open` of the same directory is a new store and a new `Arc`, and that one reads whatever is on disk now. That second open is the next session, and I don't call `open` again in the middle of a turn.

## 21.2 Add, replace, remove, and the budget

The errors are one enum, and tests match on it.

```rust
pub enum MemoryError {
    Empty,
    Duplicate,
    Drift,
    Budget { over: usize },
    Scan(String),
    Io(String),
}
```

`Empty` covers a blank write, a replace that doesn't find exactly one entry, and a remove that finds nothing. Those three causes share a variant, and `Duplicate` is only `add`. `Drift` is a file that contains `§` without the real delimiter, and `Budget` carries how many the joined text went over by. `Scan` carries the sentence the model would see, and `Io` carries the display string from the filesystem.

`add` is the write the tests use first.

```rust
pub fn add(&self, target: Target, content: &str) -> Result<&'static str, MemoryError> {
    let content = content.trim();
    if content.is_empty() {
        return Err(MemoryError::Empty);
    }
    if let Some(message) = first_threat_message(content) {
        return Err(MemoryError::Scan(message));
    }
    let _guard = self.lock.lock().expect("memory");
    let mut entries = load_entries(&self.root.join(target.file()))?;
    if entries.iter().any(|entry| entry == content) {
        return Err(MemoryError::Duplicate);
    }
    entries.push(content.to_string());
    check_budget(target, &entries)?;
    write_entries(&self.root.join(target.file()), &entries)?;
    Ok("Entry added.")
}
```

`content.trim()` borrows a substring with the surrounding whitespace removed, and a string of spaces becomes empty, and that returns `Empty` before the scan and before the lock. The file isn't created by this failure, and `read_raw` still reports `""` because a missing file and an empty file are the same result.

The scan runs next, still without the lock, but a poisoned entry doesn't wait on a writer, and it doesn't read the file. `MemoryError::Scan` carries the message. The function returns before `load_entries`, so a drift file isn't even checked if the new text itself is blocked.

`let _guard` is the lock. `Mutex::lock` returns a guard that unlocks when it is dropped, and the underscore means the guard is bound so it stays alive, not read. A temporary that isn't bound would drop at the end of the statement and unlock before `load_entries`. The binding keeps the lock until `add` returns, including the early `?` returns after it. `expect("memory")` panics if the mutex is poisoned, which means a thread panicked while holding the lock, and I'd rather stop than write the file after a half-finished critical section.

`load_entries` reads and parses, and `any` walks the entries and stops at the first one equal to the trimmed text. The stored entries were trimmed when they were parsed, so `"likes tea"` matches `"likes tea"`. A second `add` of that text returns `Duplicate` and doesn't call `write_entries`, and the file stays as it was. The `_guard` is still in scope on that return, so the lock drops as `add` leaves. The next writer can take it. Nothing was renamed.

The success type is `&'static str`, and `"Entry added."`, `"Entry replaced."`, and `"Entry removed."` are literals in the binary. I don't allocate a sentence for the happy path, and `budget_line` is the function that allocates, and it allocates only when a caller wants the overflow sentence. The test matches `MemoryError::Budget { over }` and then calls `budget_line(over)`, and the `add` error itself is the struct, not that sentence.

A new entry is `content.to_string()`, an owned copy, pushed onto the local vec, and `check_budget` then measures that vec. If it is over the limit, `?` returns `Budget` and the local vec is dropped. The file is untouched, and there is no partial line left behind. Only the success path calls `write_entries`, and the success string is `"Entry added."`, a `&'static str`, not a formatted message. The budget detail lives in the error, not in the success text.

`replace` finds one whole entry and swaps it.

```rust
pub fn replace(
    &self,
    target: Target,
    old_text: &str,
    new_content: &str,
) -> Result<&'static str, MemoryError> {
    let new_content = new_content.trim();
    if old_text.trim().is_empty() || new_content.is_empty() {
        return Err(MemoryError::Empty);
    }
    if let Some(message) = first_threat_message(new_content) {
        return Err(MemoryError::Scan(message));
    }
    let _guard = self.lock.lock().expect("memory");
    let mut entries = load_entries(&self.root.join(target.file()))?;
    let hits: Vec<usize> = entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.as_str() == old_text.trim())
        .map(|(index, _)| index)
        .collect();
    if hits.len() != 1 {
        return Err(MemoryError::Empty);
    }
    entries[hits[0]] = new_content.to_string();
    check_budget(target, &entries)?;
    write_entries(&self.root.join(target.file()), &entries)?;
    Ok("Entry replaced.")
}
```

Both sides are trimmed, and an empty `old_text` or an empty `new_content` is `Empty` before the scan. The scan looks at `new_content` only, so a blocked replacement leaves the old entry on disk. The real lesson is the length check: the hit list is every index whose whole entry equals the trimmed old text, a substring doesn't match, and both zero hits and two hits return `Empty`. The caller can't tell "missing" from "ambiguous" from the variant alone, and the function writes neither case. `hits[0]` is safe only because that check returns first. Exactly one hit is replaced by an owned copy of the new text, the budget is checked after the swap, and a longer replacement can still be `Budget` with the old entry left on disk. `"Entry replaced."` is the success string, and `replace` skips the duplicate check, so the new text may equal some other entry.

`remove` deletes every match, but it doesn't scan.

```rust
pub fn remove(&self, target: Target, old_text: &str) -> Result<&'static str, MemoryError> {
    let _guard = self.lock.lock().expect("memory");
    let mut entries = load_entries(&self.root.join(target.file()))?;
    let before = entries.len();
    entries.retain(|entry| entry != old_text.trim());
    if entries.len() == before {
        return Err(MemoryError::Empty);
    }
    write_entries(&self.root.join(target.file()), &entries)?;
    Ok("Entry removed.")
}
```

There is no empty check before the lock, and no call to `first_threat_message`, because the scan already ran on the way in and removing a poisoned string is allowed. `retain` keeps entries that aren't equal to the trimmed text, so two copies of the same entry both go away, which is the opposite of `replace`. If the length doesn't change, nothing matched and the function returns `Empty` without writing, and an empty `old_text` trims to `""`, parsed entries are never empty, and the result is `Empty` again. A successful remove of the last entry writes an empty body, and `read` returns `""`.

The test `add_replace_and_remove_round_trip_the_delimiter` opens a fresh directory, adds `"likes tea"` to memory, adds it again and expects `Duplicate`, adds `"name is Ada"` to the user file, replaces the whole memory entry with `"likes coffee"`, reads `"likes coffee"`, removes that entry, and reads `""`. One entry is stored as the entry text alone, and the delimiter shows up when two entries are joined. It isn't a trailing marker on a single entry.

Parsing is `load_entries` and `parse_entries`.

```rust
fn load_entries(path: &Path) -> Result<Vec<String>, MemoryError> {
    parse_entries(&read_raw(path)?)
}

fn parse_entries(raw: &str) -> Result<Vec<String>, MemoryError> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    if raw.contains('§') && !raw.contains(ENTRY_DELIMITER) {
        return Err(MemoryError::Drift);
    }
    Ok(raw
        .split(ENTRY_DELIMITER)
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect())
}
```

An empty file is an empty vec, not drift. Drift is the next branch: the raw text contains the character `§`, and it doesn't contain the three-character delimiter `\n§\n`. `one § two` is that case, and `§\n` is that case too, because the delimiter needs a newline on both sides. `add` returns `Drift` and doesn't rewrite the file, and the test writes `one § two` after `open`, then expects `MemoryError::Drift` from `add`. `open` itself would have succeeded even if that text had already been on disk, and the snapshot would hold it raw. The prompt would see the section sign, and the structured write would refuse.

If the real delimiter is present, the drift check fails its second half and the split runs. A stray `§` inside an entry stays in that entry, and `a\n§\nb § c` parses as `a` and `b § c`. A doubled delimiter produces an empty piece in the middle, and `filter` drops it, and surrounding spaces on an entry are trimmed. A piece that is only spaces trims to empty and is dropped with the other empty pieces. `  a  \n§\n  b  ` becomes `a` and `b`, and the vec owns `String`s. `split` only lent slices of the raw text, and `to_string` copies each kept piece before `read_raw`'s `String` is dropped.

The budget counts the joined text, not the vec length.

```rust
fn check_budget(target: Target, entries: &[String]) -> Result<(), MemoryError> {
    let total = entries.join(ENTRY_DELIMITER).len();
    let limit = target.limit();
    if total > limit {
        Err(MemoryError::Budget {
            over: total - limit,
        })
    } else {
        Ok(())
    }
}

pub fn budget_line(over: usize) -> String {
    format!("would exceed the limit by {over} chars")
}
```

> **Rust sidebar: `len` counts bytes**
>
> - `str::len` and `String::len` return the number of bytes, not the number of Unicode scalar values. `"likes tea".len()` happens to equal the number of characters because every one of those characters is one byte.
> - `§` is U+00A7. Its UTF-8 encoding is the two bytes `C2 A7`. `ENTRY_DELIMITER` is newline, `§`, newline: three characters, four bytes. A join of `ab` and `cd` is the eight bytes in `ab\n§\ncd`.
> - The constants are named `MEMORY_CHAR_LIMIT` and `USER_CHAR_LIMIT`. The comparison uses `.len()`. For the ASCII test those numbers match. A note full of non-ASCII characters fills the budget faster than a character count would suggest.
> - `budget_line` still says `chars`, because that is the sentence this function formats. The `over` value is `total - limit` from the byte length.

One entry has no delimiter in the join, and `"x"` repeated 2,201 times has length 2,201. The memory limit is 2,200. `over` is 1. `budget_line(1)` is `would exceed the limit by 1 chars`, and the test expects that string, and it expects `read` to return `""`. The user limit is the other arm of `target.limit()`, and the same function measures it. A user entry of 1,376 ASCII bytes is over by 1, and the two files do not share a budget. `check_budget` joins only the vec it was given, and `add` loads only `target.file()`, and a full `USER.md` leaves `MEMORY.md` at its own 2,200. Two entries in one file pay for the delimiter as well: the four bytes of `\n§\n` count toward that file's limit. `check_budget` builds the joined string only to measure it, then drops it, and `write_entries` joins again on the way to disk.

```rust
fn write_entries(path: &Path, entries: &[String]) -> Result<(), MemoryError> {
    let body = entries.join(ENTRY_DELIMITER);
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, &body).map_err(|err| MemoryError::Io(err.to_string()))?;
    if fs::rename(&tmp, path).is_err() {
        let _ = fs::remove_file(path);
        fs::rename(&tmp, path).map_err(|err| MemoryError::Io(err.to_string()))?;
    }
    Ok(())
}
```

The body for one entry is the entry, and the body for none is `""`. The write goes to `MEMORY.tmp` or `USER.tmp`, then `rename` moves that path onto `MEMORY.md` or `USER.md`. On a platform where rename replaces the destination, the reader sees the old file or the new file. The `if` is the branch where the first rename fails, and the code deletes the destination and tries the rename again. That is the fallback for a platform that will not rename onto an existing file. `let _ = fs::remove_file(path)` ignores a failure to delete. If the second rename also fails, the error is `Io`, the temp file may still be there, and the destination may already have been deleted. I don't try a third strategy, and the lock around this is the process-local `Mutex`. It isn't a cross-process file lock, and two processes can write the same directory.

After a successful write the snapshot pointer is still the one from 21.1. The test holds both handles, asserts `Arc::ptr_eq`, asserts the snapshot text is empty, and asserts the file contains `likes tea`.

## 21.3 The scan, before the file changes

`first_threat_message` is the gate, and `add` and `replace` call it on the text that would be stored. A `Some` becomes `MemoryError::Scan`, and a `None` means the write may continue. The file never receives a cleaned-up copy of a blocked entry, but the write doesn't happen.

```rust
pub fn first_threat_message(content: &str) -> Option<String> {
    let content = &content[..MAX_SCAN_CHARS.min(content.len())];
    if let Some(ch) = content.chars().find(|ch| INVISIBLE.contains(ch)) {
        return Some(format!(
            "Blocked: content contains invisible unicode character U+{:04X} (possible injection).",
            ch as u32
        ));
    }
    if content.contains("authorized_keys") {
        return Some(block("ssh_backdoor"));
    }
    if context_exfil(content) {
        return Some(block("context_exfil"));
    }
    if hardcoded_secret(content) {
        return Some(block("hardcoded_secret"));
    }
    None
}
```

The slice is the first 65,536 bytes, or the whole string if it is shorter. A threat that begins past that cut isn't scanned, and the later write would store the full string if the prefix is clean. The index is a byte index, because `str::len` is bytes and the slice uses that number. Slicing a `&str` at a byte that is in the middle of a character panics. ASCII never hits that, and a multibyte character sitting across byte 65,536 would. This function doesn't call `floor_char_boundary`. The tests stay on short ASCII plus one zero-width space, which is well under the cut and on a character boundary.

The checks return on the first hit, and later checks do not run.

Invisible characters come first, and the list is the one in the source.

```rust
const INVISIBLE: &[char] = &[
    '\u{200b}', '\u{200c}', '\u{200d}', '\u{2060}', '\u{2062}', '\u{2063}', '\u{2064}', '\u{feff}',
    '\u{202a}', '\u{202b}', '\u{202c}', '\u{202d}', '\u{202e}', '\u{2066}', '\u{2067}', '\u{2068}',
    '\u{2069}',
];
```

`chars()` walks Unicode scalar values, and `find` stops at the first one that `INVISIBLE` contains. U+200B is a zero-width space, and U+200C and U+200D are the zero-width non-joiner and joiner. U+2060 is a word joiner, and U+2062, U+2063, and U+2064 are the invisible times, separator, and plus. U+FEFF is the zero-width no-break space, and U+202A through U+202E are the directional embeddings, the pop, and the overrides. U+2066 through U+2069 are the directional isolates and the pop, and `{:04X}` prints the code point in uppercase hex, at least four digits. The test string `"hi\u{200b}there"` reports `U+200B`. The sentence is fixed around that code: `Blocked: content contains invisible unicode character U+200B (possible injection).` A string that also contains `authorized_keys` still reports the invisible character, because that check returns first.

`authorized_keys` is a case-sensitive substring, but `contains` doesn't lowercase. `Authorized_Keys` doesn't match, and `drop a key in authorized_keys` does. The id in the sentence is `ssh_backdoor`.

```rust
fn block(pid: &str) -> String {
    format!(
        "Blocked: content matches threat pattern '{pid}'. Content is injected into the system prompt and must not contain injection or exfiltration payloads."
    )
}
```

That is the sentence for the three pattern ids, and the test looks for `ssh_backdoor` and for `injected into the system prompt`.

`context_exfil` compiles its regex on every call that reaches it, and an invisible hit or an `authorized_keys` hit returns before this line, so those strings never compile it. I didn't put this one in an `OnceLock`, and the pattern is fixed, and `expect("context")` panics only if that fixed pattern fails to compile.

```rust
fn context_exfil(content: &str) -> bool {
    Regex::new(
        r"(?i)(include|output|print|share)\s+(?:\w+\s+){0,8}(conversation|chat\s+history|previous\s+messages|full\s+context|entire\s+context)",
    )
    .expect("context")
    .is_match(content)
}
```

`(?i)` folds case. The first group is one of four verbs: `include`, `output`, `print`, `share`. Then whitespace. Then zero to eight words, each a `\w+` followed by whitespace, and then one of `conversation`, `chat history`, `previous messages`, `full context`, or `entire context`. `include the full conversation` matches: `include`, a space, the word `the`, a space, and `conversation` can be reached because `{0,8}` is allowed to consume `the ` and `full `. Eight words between the verb and `conversation` still match. Nine do not. `include conversation` matches with zero words in between, and `print previous messages`, `share the chat history`, and `output the entire context` match. `include tea` doesn't, but `likes tea` doesn't. The id is `context_exfil`.

`hardcoded_secret` is the last check, compiled the same way, once per call that reaches it.

```rust
fn hardcoded_secret(content: &str) -> bool {
    Regex::new(r#"(?i)(?:api[_-]?key|token|secret|password)\s*[=:]\s*["'][A-Za-z0-9+/=_-]{20,}"#)
        .expect("secret")
        .is_match(content)
}
```

The raw string is `r#"..."#` so the quotes inside the class are literal, and this is the same delimiter chapter 20 uses for the auth header. The pattern is case-insensitive. The name is `api_key`, `apikey`, `api-key` (the `[_-]?` is an optional underscore or hyphen), or `token`, `secret`, or `password`. Optional space, then `=` or `:`, then optional space, then a `"` or `'`, then at least twenty characters from letters, digits, and `+/=_-`. The test value `api_key = "abcdefghijklmnopqrst"` has twenty letters and matches. `token: 'abcdefghijklmnopqrst'` matches. `apikey="abcdefghijklmnopqrst"` and `api-key="abcdefghijklmnopqrst"` match, but `password="short"` is under twenty characters and doesn't. There is no lookahead in this pattern, but the env-name lookahead isn't ported. The id is `hardcoded_secret`.

`None` is the fall-through, and `likes tea` and `name is Ada` take it, and the write proceeds.

The poisoned-entry test calls `first_threat_message` on four strings, then tries to `add` the bare word `authorized_keys` and expects `MemoryError::Scan`. `read` of `MEMORY.md` is still `""`, and the scan is the gate. It isn't a filter that stores a cleaned copy.

## 21.4 The queue, and the pointer that stays put

`enqueue_turn` is the trait method, but it doesn't open either file.

```rust
fn enqueue_turn(&self, record: &TurnRecord<'_>) -> Result<(), HookError> {
    let mut queue = self.queue.lock().expect("queue");
    if queue.len() == QUEUE_CAP {
        queue.pop_front();
    }
    queue.push_back(format!("{} {}", record.user, record.assistant));
    Ok(())
}
```

This is the other mutex, and `QUEUE_CAP` is 64, the same number as the WAL id queue, and a different buffer. At 64 strings the oldest is dropped, then the new one is pushed, and the stored value is the user text, a space, and the assistant text. The test record `hi` / `ok` becomes `hi ok`, and the function returns `Ok(())`. It has no `Err` path other than a poisoned mutex, which panics inside `expect` instead of returning `HookError`. `name` is the other trait method.

```rust
fn name(&self) -> &str {
    "files"
}
```

`queue_len` locks only long enough to read the length.

```rust
pub fn queue_len(&self) -> usize {
    self.queue.lock().expect("queue").len()
}
```

The guard is a temporary here. That is fine, because the only work is `.len()`, and the lock can drop at the end of the statement. The snapshot test enqueues one record, expects `queue_len() == 1`, expects `name() == "files"`, and expects the file to still be `likes tea`. Enqueue doesn't rebuild the snapshot and doesn't call `write_entries`.

The bench `memory_enqueue` is this path, and `open` has already happened. The timed call locks, maybe drops the oldest entry, formats one string, and pushes it. That is why it sits in nanoseconds, and `add` would read a file, scan, and rename a temp file. The bench doesn't call `add`.

The demo opens a temp directory, takes a snapshot, adds `likes tea`, takes the snapshot again, enqueues `hi` / `ok`, and prints the file. The word `frozen` is the branch where both snapshot strings are still empty, and `changed` is the other branch. A fresh directory takes `frozen`.

The guardrail catalog calls this snapshot F6 and the write-time scan F5, and F6's test is the `Arc::ptr_eq` from 21.1: empty snapshot text, `likes tea` on disk. F5 is the four blocked strings in 21.3, `authorized_keys` among them. A9, the write-approval gate, stays off until chapter 25, and L8 is the catalog layer these files sit in.

`queue_len`, `snapshot`, `add`, `replace`, `remove`, `read`, `name`, and `enqueue_turn` are the public surface, and `budget_line` and `first_threat_message` are public too, so the tests can check the sentence without going through a write. `open` is the only constructor, and the `frozen` field it stored is the one later calls keep handing out.

## Tests

Five tests. Add, replace, and remove round-trip the delimiter, and a duplicate is refused, and a write over the budget names the overflow and leaves the file empty. Four poisoned strings are blocked, and `authorized_keys` doesn't reach the file, and the snapshot pointer is unchanged after a write, and enqueue doesn't touch the file. A broken delimiter is drift.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch21-memory
```

```text
Starting 114 tests across 29 binaries
     Summary [   0.221s] 114 tests run: 114 passed, 0 skipped
```

```bash
cargo run -p ch21-memory
```

```text
file: likes tea
snapshot: frozen
```

The binary uses a temp directory, and the file has the new entry. The snapshot taken at open is still empty.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `memory_enqueue` | 54.59 ns | under 1 ms | push one turn onto the background queue |

`memory_enqueue` calls `MemoryFiles::open` on a temp directory before the bench, and the timed closure is `b.iter`. Inside: `enqueue_turn` of user `hi` and assistant `ok`, with the record passed through `black_box`. The sample locks the queue, drops the oldest entry if 64 are already queued, formats one string, and pushes it. Opening the directory and reading the files are excluded, and no file is written. The snapshot isn't rebuilt.

The file says `likes tea`, and the snapshot is `frozen`.

The means are `mean.point_estimate` from [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `21`, and how to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The snapshot in 21.1 is for this process. A second `open` of the same directory reads the file you just wrote, and that second open is the next session. Calling `open` again in the middle of a turn builds a new `Arc` and moves the prefix.

The scan is four checks, and a payload that Hermes would catch with a later pattern in `threat_patterns.py` can pass here. The ids that are implemented use the real names and the real sentence.

`replace` needs the whole entry, but a substring isn't enough. Hermes can locate an entry by a span and still replace the whole entry, and this crate only matches the whole entry, so a partial `old_text` returns `Empty`.

The queue isn't a write, and `enqueue_turn` will not create `MEMORY.md`. `add` is the write.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L8.

**Scan.** Strict scope, on the write. Invisible characters, `ssh_backdoor`, `context_exfil`, and `hardcoded_secret`. The block sentence is `first_threat_message`. The file does not change when the scan hits.

**Snapshot.** Built once. Shared by `Arc`. A write does not rebuild the system prompt.

**A9.** The write-approval gate stays off. Chapter 25 is the human gate for memory and skills. This chapter does not stage a pending diff.

**Left for later.** The rest of the threat-pattern list. A cross-process file lock. Transcript redaction (E1, E2) is chapter 20, already in this crate. `add` does not call it.

## Chapter summary

- **Files.** `MEMORY.md` at 2,200 characters. `USER.md` at 1,375. Delimiter `\n§\n`.
- **Snapshot.** The `Arc` from `open` doesn't change when `add` writes.
- **Scan.** `authorized_keys` and a zero-width space never reach the file.
- **Queue.** `enqueue_turn` doesn't write. The gate A9 stays off.
- **Time.** One enqueue measured **54.59 ns**.

## What you implement

`my-harness/tests/ch21.rs` is the milestone, and it compiles, then panics in `todo!("chapter 21")` until `chapter_21_snapshot_changes_on_write` returns `false`. A write must not replace the `Arc` the caller already holds.

## What's next

Chapter 22 is recall, and the session file from chapter 20 gets an FTS5 trigram index. `session_search` can discover, scroll, read, or browse, and subagent sessions stay out of the results. The search runs only when something calls it.