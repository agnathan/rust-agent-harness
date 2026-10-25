# Chapter 32: The Critic

If the same agent that did the work also grades it, the grade tends to agree with the work, and workers work, and someone else checks. That split is what keeps a continuously improving harness from storing its own mistakes as the procedure. The demo hands `judge` the line `verdict: done` and prints `accepted: done`, and a missing word, or a polite sentence that never says `verdict`, comes back `Reject`.

`judge` is a function. It isn't the `Tool` trait, and it isn't chapter 29's `Job`, so the fix for the mismatch is to stop calling it a tool. `as_tool` is only the name of the wrapper that calls `judge`, and one permit comes off the chapter 29 pool. A judge that ran wide would starve the workers it's judging, which is why the summary quotes that limit again. The parser scans for one word, and the snippet cuts on a character boundary so a slice never lands in the middle of a UTF-8 sequence. The notes lock is a read, and the fan-out queue doesn't gain a task from the grade.

Chapter 33 will loop on this verdict: a continuation, a `wait` target, and the quality-gate commands deferred from earlier. They stay out of this file so one judgment is testable without a state machine; a bad grade is information for the parent. Turning every rejection into a new procedure is how curation inherits a pile of "don't do the thing we just did" skills. Chapter 26 can review the run afterward, with the skill tools, off the hot path.

The same rule shows up in smaller form elsewhere: the reviewer's tool slice, and the curator's enqueue, both spend a bounded resource so the check can't become the work.

Indexing a `&str` is by byte, and `is_char_boundary` is the check before the slice. The verdict parser is a scan, not a JSON schema, because the model returns a sentence. The permit is the same guard type as chapter 29, so the pool's accounting stays one type.

The step is chapter 31's harness plus `critic`, at `steps/ch32-critic`, package `ch32-critic`. `cargo run --locked -p ch32-critic --bin ch32-critic` prints `accepted: done`. `my-harness/tests/ch32.rs` checks that the critic reads the summary and leaves the work alone.

| Section | Step | What it adds |
|---|---|---|
| 32.1 | `steps/ch32-critic` | the constants, `Verdict`, `Outcome` |
| 32.2 | `steps/ch32-critic` | `parse_verdict` scans the reply |
| 32.3 | `steps/ch32-critic` | `snippet_of` stops at 4,000 bytes on a char boundary |
| 32.4 | `steps/ch32-critic` | `judge`, `as_tool`, `final_gate` |
| 32.5 | `steps/ch32-critic` | the demo `accepted: done` and `judge_one_permit` |

> **How Hermes does it**
>
> I read `hermes_cli/goals.py` (`_parse_judge_response`, `DEFAULT_MAX_TURNS`, `DEFAULT_JUDGE_TIMEOUT`, `_JUDGE_RESPONSE_SNIPPET_CHARS`, `JUDGE_SYSTEM_PROMPT`, `DEFAULT_GATE_TIMEOUT_SECONDS`, `DEFAULT_GATE_MAX_RETRIES`) on Hermes `main` at `ab3fe9dffb0a`.
>
> `DEFAULT_MAX_TURNS` is 20. `DEFAULT_JUDGE_TIMEOUT` is 30.0. `_JUDGE_RESPONSE_SNIPPET_CHARS` is 4000. `JUDGE_SYSTEM_PROMPT` is the judge prompt. `DEFAULT_GATE_TIMEOUT_SECONDS` is 300. `DEFAULT_GATE_MAX_RETRIES` is 3. `_parse_judge_response` is the parser.
>
> **Adopted.** The turn cap of 20. The timeout constant of 30 seconds. The cap of 4,000. `done` is the accept this gate returns.
>
> **Adapted.** The parser is a line scan of the reply for the word `verdict`. A missing verdict word is `Reject`. `as_tool` and `final_gate` both call `judge`. `judge` takes one permit with `pool.acquire`, locks the notes, and does not push. The 30 second timeout is not slept. `SNIPPET_CHARS` is 4,000. The cut uses `raw.len()`, which is bytes, then steps back to a char boundary. Hermes's `_JUDGE_RESPONSE_SNIPPET_CHARS` is a character count.
>
> **Deferred.** The continuation loop, the wait target, and the quality-gate shell commands, to chapter 33. `DEFAULT_GATE_TIMEOUT_SECONDS` and `DEFAULT_GATE_MAX_RETRIES` are not stored in this file. `JUDGE_SYSTEM_PROMPT` is not embedded here.

## What this chapter decides

`limits` returns `(20, 30)`, and those are `DEFAULT_MAX_TURNS` and `DEFAULT_JUDGE_TIMEOUT_SECS`, and nothing in this crate counts turns, and nothing sleeps for 30 seconds.

`parse_verdict` lowercases the reply, finds `verdict`, and reads the token after the separators. `done`, `blocked`, `wait`, and `continue` map to those variants, and any other token, and a reply with no `verdict` substring, is `Reject`.

`judge` acquires one permit, clones the pool's context `Arc`, locks `notes`, parses, and builds an `Outcome`. `accepted` is `verdict == Verdict::Done`, and the snippet is the reply cut at 4,000 bytes on a character boundary, and the notes vec isn't pushed.

`as_tool` returns that `Outcome`; `final_gate` returns `Ok(snippet)` when `accepted` is true and `Err(verdict)` otherwise.

`demo` judges `verdict: done` and returns `accepted: done`, and it reads no key.

## 32.1 The constants are named, and two of them are not looped

```rust
use crate::supervisor::Pool;

pub const DEFAULT_MAX_TURNS: u32 = 20;
pub const DEFAULT_JUDGE_TIMEOUT_SECS: u64 = 30;
pub const SNIPPET_CHARS: usize = 4_000;
```

`Pool` is the only import from the supervisor; `WORKER_BOUND` isn't imported at the top. `demo` refers to it as `crate::supervisor::WORKER_BOUND` at the call, and the pool's limit is still 8, because `Pool::new` receives that const. The critic doesn't build a second pool type.

`DEFAULT_MAX_TURNS` is 20, a `u32`, matching the Hermes turn cap, and `DEFAULT_JUDGE_TIMEOUT_SECS` is 30, a `u64`. Hermes stores `DEFAULT_JUDGE_TIMEOUT` as 30.0, and this file stores whole seconds. `SNIPPET_CHARS` is `4_000`, which is 4000, and the underscore is a digit separator. Hermes names the same size `_JUDGE_RESPONSE_SNIPPET_CHARS`.

`limits` returns the pair the tests assert.

```rust
pub fn limits() -> (u32, u64) {
    (DEFAULT_MAX_TURNS, DEFAULT_JUDGE_TIMEOUT_SECS)
}
```

The tuple is `(20, 30)`, and there is no third field for 300 and no fourth field for 3. `DEFAULT_GATE_TIMEOUT_SECONDS` is 300 and `DEFAULT_GATE_MAX_RETRIES` is 3 in `goals.py`. Chapter 33 is where a gate retry belongs, and this function doesn't mention them. A caller who only has `limits()` has the turn cap and the judge timeout.

`Verdict` is the five outcomes the parser can return.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Done,
    Blocked,
    Wait,
    Continue,
    Reject,
}
```

`Copy` means a verdict is copied into `Outcome` and into `Err` without a clone method. `PartialEq` is what `verdict == Verdict::Done` and the tests' `assert_eq!` use. `Reject` is the harness variant for a reply the scan doesn't recognize, and the four names above it are the words the scan looks for.

`Outcome` is what `as_tool` returns.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub accepted: bool,
    pub verdict: Verdict,
    pub snippet: String,
}
```

`accepted` isn't a second opinion; `judge` sets it from the verdict. `snippet` is an owned prefix of the raw reply, and it isn't a new sentence the critic wrote.

## 32.2 `parse_verdict` scans for one word

The parser is the adaptation; Hermes has `_parse_judge_response`, and this function scans the reply text.

```rust
pub fn parse_verdict(raw: &str) -> Verdict {
    let lower = raw.to_ascii_lowercase();
    let Some(pos) = lower.find("verdict") else {
        return Verdict::Reject;
    };
    let rest = lower[pos + "verdict".len()..].trim_start_matches(|ch: char| {
        ch == ':' || ch == '"' || ch == ' ' || ch == '=' || ch == '\''
    });
    if rest.starts_with("done") {
        Verdict::Done
    } else if rest.starts_with("blocked") {
        Verdict::Blocked
    } else if rest.starts_with("wait") {
        Verdict::Wait
    } else if rest.starts_with("continue") {
        Verdict::Continue
    } else {
        Verdict::Reject
    }
}
```

`to_ascii_lowercase` allocates a new `String`; ASCII letters fold, and other bytes stay as they are. `Verdict: Done` becomes a string that contains `verdict: done`, and the original `raw` isn't modified. The snippet helper cuts `raw`, not `lower`, so the snippet keeps the caller's casing.

`find("verdict")` returns `Option<usize>`, the byte index of the first match. `let Some(pos) = ... else` is the pattern that returns `Reject` when the word is absent, and the search is a substring search on the lowercased text. It isn't a JSON parse, and it doesn't require the word to sit at column 0. A reply that never contains those seven letters is `Reject`.

`pos + "verdict".len()` is the index just after the word. `"verdict".len()` is 7 bytes, and the range `lower[pos + 7..]` is the suffix. `trim_start_matches` drops a prefix of characters while the closure returns true, and the characters are colon, double quote, space, equals, and single quote. A run of them is removed, and `verdict: done`, `verdict:done`, `verdict = done`, and `verdict "done"` all leave a suffix that starts with `done`. The trim stops at the first other character, and it doesn't remove a quote that appears after the word `done`.

The `if` chain uses `starts_with`. `done` is tested first, then `blocked`, then `wait`, then `continue`, and a suffix that starts with `done` is `Done` even when more characters follow. The tests pass exact tokens: `verdict: done`, `verdict: blocked`, `verdict: wait`, `verdict: continue`, and they also pass three rejects: the empty string, `looks fine`, and `verdict: maybe`.

Empty string: `find` returns `None`. `Reject`.

`looks fine`: the lowercased text has no `verdict`. `Reject`.

`verdict: maybe`: the suffix after the trim is `maybe`, and it doesn't start with any of the four words; the final `else` is `Reject`.

`verdict: done` is `Done`; `verdict: blocked` is `Blocked`. `verdict: wait` is `Wait`. `verdict: continue` is `Continue`, and the interesting failures are the ones the unit tests don't pass. `verdict: donut` takes the done branch, because `starts_with("done")` is true of `donut`, so a nonsense word that happens to share those four letters is accepted. `The verdict is done` finds `verdict` and then sees `is done`, and the trim only strips colon, quote, space, equals, and single quote, so the suffix doesn't start with `done` and the function returns `Reject`. A polite critic that writes a sentence, instead of `verdict: done`, fails closed, and that is what the function returns on those two strings. Neither string is in `done_accepts_and_a_missing_verdict_rejects`.

`Wait` parses and doesn't mean the function slept, and there is no session id, no pid, and no number of seconds read out of the suffix. The word is the variant, and the continuation loop and the wait target are chapter 33.

The scan is the whole reply, including newlines; the demo string is `verdict: done\nThe note is short enough.` The word sits on the first line. `find` returns its index, and the suffix starts at the colon. After the trim, `starts_with("done")` matches, and the newline and the sentence stay in the suffix, past the matched prefix. The function doesn't split on `\n` and it doesn't require the verdict to be the only thing on the line. The adaptation is that scan, in place of a structured judge object.

Chapter 31's demo string is `a\nb\nc`, and put that string through `parse_verdict` and `find("verdict")` returns `None`, and the verdict is `Reject`. `accepted` would be false. `final_gate` would return `Err(Reject)`, and the gather output is three replies and no verdict word. The critic doesn't treat a non-empty summary as finished. Someone has to put the word `verdict` in the text the critic reads, and the demo in this chapter does that with a literal. `judge` doesn't call `fanout::gather`. The two modules sit in one crate and meet only if a caller passes a string from one into the other.

The separators in the trim closure are five characters; colon is the demo's separator, in `verdict: done`. Space is what a line like `verdict done` would need, and the closure includes it. Equals is included, and a double quote is included, and a single quote is included, and a comma isn't. A suffix that starts with a comma after the word `verdict` doesn't lose that comma, and `,done` doesn't start with `done`. The tests don't pass a comma. The closure is the list of characters the source strips, and it's only a prefix strip. Characters later in the suffix stay, which is why `done\nThe note is short enough.` still starts with `done`.

`to_ascii_lowercase` copies the whole reply before `find`. A 5,000-`x` reply allocates that lowercase string inside `parse_verdict`, and `snippet_of` allocates another string of 4,000 bytes from the original. Both allocations happen inside `judge`, while the notes guard is held, and the lowercase `String` is a local inside `parse_verdict`. It drops when `parse_verdict` returns, before `Outcome` is built. The snippet is moved into the `Outcome` and lives as long as that value does. The lowercase copy isn't stored on `Outcome`. The bench's reply is one short line plus a second short line, so both allocations are small. The mean `278.9 ns` includes them, plus the permit and the lock, and it doesn't include a 5,000-character reply. That longer reply is the unit test `the_snippet_stops_at_four_thousand_chars`.

`Outcome` for the demo input, if you kept it, would have `accepted` true, `verdict` `Done`, and a snippet equal to the whole raw string, because that string is far shorter than 4,000 bytes. `demo` doesn't read `snippet` or `verdict`, and it reads `accepted` and returns a different literal, `accepted: done`. The `else` literal `rejected` is what a `Reject`, a `Blocked`, a `Wait`, or a `Continue` would print. One boolean, and four non-accepting verdicts share that branch. `final_gate` keeps them distinct, because `Err` carries the variant. `demo` uses `as_tool` and collapses them, and the test `the_demo_accepts` only requires the accepting line.

`limits` is asserted beside the parser in the same test.

```rust
#[test]
fn done_accepts_and_a_missing_verdict_rejects() {
    assert_eq!(parse_verdict("verdict: done"), Verdict::Done);
    assert_eq!(parse_verdict("verdict: blocked"), Verdict::Blocked);
    assert_eq!(parse_verdict("verdict: wait"), Verdict::Wait);
    assert_eq!(parse_verdict("verdict: continue"), Verdict::Continue);
    assert_eq!(parse_verdict(""), Verdict::Reject);
    assert_eq!(parse_verdict("looks fine"), Verdict::Reject);
    assert_eq!(parse_verdict("verdict: maybe"), Verdict::Reject);
    let (turns, timeout) = limits();
    assert_eq!(turns, 20);
    assert_eq!(timeout, 30);
}
```

Twenty and thirty are the constants, and the test doesn't start a timer. `DEFAULT_JUDGE_TIMEOUT` of 30.0 is the Hermes number this `u64` stands for. The sleep isn't part of the assertion because it isn't part of the function.

## 32.3 The snippet stops on a character boundary

`snippet_of` is private. `judge` is the only caller.

```rust
fn snippet_of(raw: &str) -> String {
    let mut end = SNIPPET_CHARS.min(raw.len());
    while !raw.is_char_boundary(end) {
        end -= 1;
    }
    raw[..end].to_string()
}
```

`SNIPPET_CHARS.min(raw.len())` is the smaller of 4,000 and the byte length, so a short reply is copied whole and a longer one starts with `end` at 4,000. `is_char_boundary` is true when that index is a legal cut. The test uses ASCII `x`, so byte 4,000 is already a boundary and the loop body doesn't run. The interesting cut is a multibyte character. `é` is the two bytes `C3 A9`. If `end` landed on the `A9`, `is_char_boundary` would be false and the loop would step back to the `C3`, which is a boundary, so the slice ends at the start of `é` and the partial character is left out instead of panicking. An emoji such as `👍` is four bytes, and the same loop walks back to the first byte of that character. The unit test doesn't include `é` or `👍`, and it locks the ASCII length.

```rust
#[test]
fn the_snippet_stops_at_four_thousand_chars() {
    let pool = Pool::new(WORKER_BOUND);
    let raw = format!("verdict: done\n{}", "x".repeat(5_000));
    let outcome = as_tool(&pool, &raw);
    assert_eq!(outcome.snippet.len(), SNIPPET_CHARS);
    assert!(outcome.accepted);
}
```

`"x".repeat(5_000)` is five thousand `x` characters, and the prefix `verdict: done\n` makes `raw` longer than 4000. `snippet.len()` is 4000. `accepted` is true because `parse_verdict` sees `verdict` and `done` on the full `raw`, not on the cut snippet. The cut still contains the prefix, because the prefix is shorter than 4000 and the `x`s fill the rest. Acceptance and the snippet are two fields, and the parser's input is the caller's string.

`JUDGE_SYSTEM_PROMPT` is the Hermes prompt, and this function doesn't prepend it, and it doesn't search it. The reply text is the only input.

## 32.4 `judge` takes one permit and does not push

`judge` is private, and both public entry points call it.

```rust
fn judge(pool: &Pool, raw: &str) -> Outcome {
    let _permit = pool.acquire();
    let context = pool.context();
    let _notes = context.notes.lock().expect("notes");
    let verdict = parse_verdict(raw);
    Outcome {
        accepted: verdict == Verdict::Done,
        verdict,
        snippet: snippet_of(raw),
    }
}
```

`pool.acquire` is chapter 29's method, on the pool this crate copied forward, and one permit comes out. `_permit` holds it until `judge` returns. A second `judge` can't overlap this one on the same thread, because the first permit drops before the second call. `max_seen` becomes 1 on the first acquire. A later acquire on the same pool sees `inflight` go to 1 again and sets `max_seen` to `max(1, 1)`, which stays 1. The test reads `max_seen` after `as_tool` and again after `final_gate` and expects 1 both times.

`pool.context()` clones the `Arc`. `Arc::ptr_eq` against a clone taken before the call stays true. `judge` doesn't replace the pool's context.

`notes.lock()` returns the guard. `expect("notes")` panics if the mutex is poisoned, and the binding is `_notes`, and no method is called on it. `push` isn't in this function, and the guard's job is to take the lock and to drop it. Drop order is the reverse of declaration. `_notes` drops first, then `context`, then `_permit`, and the notes mutex unlocks before the permit is returned to the pool. The pool's own `Arc` is untouched by the drop of the local clone.

`accepted: verdict == Verdict::Done` is the only accept. `Blocked`, `Wait`, `Continue`, and `Reject` store `accepted: false`. The verdict field still carries the specific variant, so `final_gate` can put that variant in `Err`.

```rust
pub fn as_tool(pool: &Pool, raw: &str) -> Outcome {
    judge(pool, raw)
}

pub fn final_gate(pool: &Pool, raw: &str) -> Result<String, Verdict> {
    let outcome = judge(pool, raw);
    if outcome.accepted {
        Ok(outcome.snippet)
    } else {
        Err(outcome.verdict)
    }
}
```

> **Rust sidebar: `Result`**
>
> - `as_tool` returns `Outcome`. The caller reads `accepted`, `verdict`, and `snippet`.
> - `final_gate` returns `Result<String, Verdict>`. `Err` is the verdict that was not `Done`. `Verdict` does not have to implement `std::error::Error` for this signature.
> - Both functions call `judge`, so both take a permit. Calling `as_tool` and then `final_gate` takes two permits in series. `max_seen` stays 1 because the first `Drop` runs before the second `acquire`.
> - `Option`, `Result`, enums, and `Arc` are taught in [chapter 1](chapter-01-first-contact.md).

The test covers both doors.

```rust
#[test]
fn the_tool_and_the_gate_take_one_permit_and_do_not_write() {
    let pool = Pool::new(WORKER_BOUND);
    let context = pool.context();
    let before = context.notes.lock().unwrap().len();
    let outcome = as_tool(&pool, "verdict: done\nship it");
    assert!(outcome.accepted);
    assert_eq!(outcome.verdict, Verdict::Done);
    assert!(Arc::ptr_eq(&context, &pool.context()));
    assert_eq!(pool.max_seen(), 1);
    assert_eq!(context.notes.lock().unwrap().len(), before);
    let refused = final_gate(&pool, "verdict: blocked\nmissing file");
    assert_eq!(refused, Err(Verdict::Blocked));
    assert_eq!(pool.max_seen(), 1);
    assert_eq!(final_gate(&pool, "").unwrap_err(), Verdict::Reject);
}
```

`before` is the notes length on a fresh pool, which is 0. `as_tool` on `verdict: done\nship it` accepts, and the pointer matches. `max_seen` is 1, and the length is still `before`. `final_gate` on `verdict: blocked\nmissing file` is `Err(Blocked)`. `max_seen` is still 1, and an empty string through `final_gate` is `Err(Reject)`, from `unwrap_err`, and the notes were locked in each `judge` and never pushed, and the critic reads. The work stays where the caller left it.

`Context` in the supervisor is `notes: Mutex<Vec<String>>`. Chapter 28's pipeline context also has messages, and the pool's context is the supervisor one. `judge` never sees `pipeline::Context`. Locking notes here doesn't filter a transcript and doesn't append a stage line.

The 30 second constant isn't a `thread::sleep` argument anywhere in `critic.rs`. The time values in the file are the two consts, the return of `limits`, and the number 4000. `Duration` isn't imported. A caller who wants to wait 30 seconds has to do that outside this module. This module records the number Hermes uses and then parses.

`DEFAULT_MAX_TURNS` is the same shape, and no loop in `judge` counts to 20. Twenty `Continue` verdicts would be twenty calls if some other function called `judge` twenty times. This file doesn't. Chapter 33 owns the continuation loop.

Quality-gate shell commands are the other deferral. `final_gate` doesn't spawn a process, and the name is the gate in this crate: `Ok` only for `Done`. A shell that must exit 0 before the judge runs is chapter 33.

## 32.5 The demo accepts, and the bench times one permit

```rust
pub fn demo() -> String {
    let pool = Pool::new(crate::supervisor::WORKER_BOUND);
    let outcome = as_tool(&pool, "verdict: done\nThe note is short enough.");
    if outcome.accepted {
        "accepted: done".to_string()
    } else {
        "rejected".to_string()
    }
}
```

`Pool::new` gets 8. `as_tool` runs `judge` on a two-line reply, and the first line is `verdict: done`. The second line is `The note is short enough.` `accepted` is true, and the function returns `accepted: done`. The `else` branch is `rejected`, and this input doesn't take it, and the snippet is computed and then dropped with `outcome`. The printed line is the literal, not the snippet, and the notes vec is still empty.

```rust
#[test]
fn the_demo_accepts() {
    assert_eq!(harness_core::critic::demo(), "accepted: done");
}
```

The binary prints that line.

```rust
use harness_core::critic::demo;

fn main() {
    println!("{}", demo());
}
```

The package binary is `ch32-critic`. It takes no key, the reply is a literal, and no prompt string from `JUDGE_SYSTEM_PROMPT` is loaded.

The bench is `benches/critic.rs`. `Cargo.toml` names it `critic` with `harness = false`, and the workflow section is `32`, label `chapter 32 critic`, package `ch32-critic`.

```rust
fn judge_one_permit(c: &mut Criterion) {
    let pool = Pool::new(WORKER_BOUND);
    let raw = "verdict: done\nThe note is short enough.";
    c.bench_function("judge_one_permit", |b| {
        b.iter(|| {
            let outcome = as_tool(black_box(&pool), black_box(raw));
            black_box(outcome);
        });
    });
}

criterion_group!(benches, judge_one_permit);
criterion_main!(benches);
```

The pool and the raw string are built before `b.iter`. The timed closure calls `as_tool`, which calls `judge`: one `acquire`, one context clone, one notes lock, one parse, one snippet of a short string, one `Drop` of the permit. `black_box` keeps the pool borrow, the reply, and the `Outcome` in the sample, and no model is called. The 30 second timeout isn't slept, and the notes vec isn't appended. Each iteration's permit is released before the next iteration, so the bench doesn't queue eight critics. It takes one permit, over and over.

The mean is `278.9 ns`, and it's `mean.point_estimate`.

The bench imports `as_tool` from `harness_core::critic` and `Pool`, `WORKER_BOUND` from `harness_core::supervisor`. `final_gate` isn't in the sample, and the unit test is where both entry points run.

One critic on an idle pool doesn't wait. `acquire` locks state, sees `inflight` of 0, skips the `while`, sets `inflight` to 1, and sets `max_seen` to 1. The `Condvar` isn't waited on. `Drop` later sets `inflight` back with `saturating_sub` and calls `notify_one`, and if no thread is waiting, the notify wakes nobody. The bench does that pair on every iteration: take one, drop one, and eight critics at once would be a different program. The ninth would wait in `acquire` the way chapter 31's sixteenth task waits, and this file's tests never start that ninth critic. `max_seen` of 1 is the observation they record, and it's the observation that the critic took a permit at all. A `judge` that parsed without `acquire` would leave `max_seen` at 0, and the test would fail. The permit is how the critic shares the chapter 29 bound instead of running beside it with no limit.

`black_box(&pool)` is still a `&Pool`. `as_tool` takes `&Pool`. The opaque wrapper stops the optimizer from noticing that the pool is unmodified across iterations and deleting the call. The pool is modified: `max_seen` changes on the first iteration and `inflight` moves during each call. After the iteration, `inflight` is 0 again and `max_seen` stays 1, and the notes length stays 0. A benchmark that asserted those facts inside `b.iter` would still be measuring `judge`, and this bench doesn't assert. The unit test asserts. The mean `278.9 ns` is the timing of the call the test already checked for behavior.

`snippet_of` on the bench's raw string takes `min(4000, len)`. The len of `verdict: done\nThe note is short enough.` is the byte length of that literal, which is under 4000, so `end` starts at that length. `is_char_boundary` is true for the end of a string, and the `while` doesn't run. The snippet is a full copy. The `Outcome` the bench passes to `black_box` holds that copy, the bool `true`, and `Verdict::Done`. Then the iteration ends and the `Outcome` drops, and the next iteration repeats the copy. That repeated short copy, plus the lock and the permit, is the sample.

`final_gate` on the same raw string would return `Ok` of that snippet instead of an `Outcome`. The bench doesn't call it. The extra `Result` match is a branch the unit test covers when the verdict is `Blocked` and when the raw string is empty. Empty raw: `parse_verdict` returns `Reject` before any `starts_with` check, `accepted` is false, `snippet_of("")` is an empty `String` because the length is 0, and `final_gate` returns `Err(Reject)` and drops the empty snippet. The error path still took a permit. `max_seen` after the empty call in the test is 1, the same peak, because the earlier `as_tool` had already set it.

Chapter 33 can loop on `Continue` until 20 turns, park on `Wait` when a target exists, and run a shell gate `DEFAULT_GATE_MAX_RETRIES` times with `DEFAULT_GATE_TIMEOUT_SECONDS` of 300. None of those calls are in `critic.rs`. What this file gives that chapter is `parse_verdict`, `as_tool`, `final_gate`, `limits`, and the rule that `Done` is the only accept. The permit those calls take is already the supervisor's.

## Tests

Four tests in `tests/critic.rs`.

The four known verdict lines parse to `Done`, `Blocked`, `Wait`, and `Continue`. `""`, `looks fine`, and `verdict: maybe` are `Reject`. `limits` is `(20, 30)`.

`as_tool` on `verdict: done\nship it` accepts, the context pointer matches, `max_seen` is 1, and the notes length is unchanged. `final_gate` on the blocked reply is `Err(Blocked)`. `max_seen` stays 1, and an empty `final_gate` is `Reject`.

A reply of `verdict: done` plus 5,000 `x` characters snippets to `SNIPPET_CHARS` and still accepts.

`demo` equals `accepted: done`.

The package also runs the tests copied forward from chapter 31. Nextest runs all of them.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch32-critic
```

```text
Starting 163 tests across 39 binaries
     Summary [   0.359s] 163 tests run: 163 passed, 0 skipped
```

```bash
cargo run --locked -p ch32-critic --bin ch32-critic
```

```text
accepted: done
```

The binary judges one scripted reply and reads no key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `judge_one_permit` | 278.9 ns | one permit from the pool of 8 | `as_tool` on `verdict: done` |

`judge_one_permit` builds the pool before `b.iter`. The timed closure acquires one permit, locks the notes, parses, copies the short snippet, and drops the permit. The 30 second constant isn't a sleep in that closure.

The means are `mean.point_estimate` from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `32`. Bench ids: `32/judge_one_permit`, `30/judge_one_permit`, and how to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`starts_with("done")` matches a suffix that begins with those four letters, and the tests pass the token `done` and stop. A longer token that begins with `done` follows the same branch, because that is what `starts_with` checks. The chain tests `blocked`, `wait`, and `continue` the same way. `verdict: maybe` falls through to `Reject`, and the test covers that string.

`find("verdict")` searches for the substring. A reply that doesn't contain it's `Reject`, including the empty string and `looks fine`. The scan lowercases first, so `Verdict: Done` folds before the search.

`final_gate` and `as_tool` each call `judge`, and two calls take two permits one after another. `max_seen` stays 1. Calling them from two threads on one pool would share the eight permits, and the tests call them on one thread.

`Reject` on a missing word is this parser. I am not describing a second parser's default. `_parse_judge_response` is the Hermes function, and the body this chapter runs is `parse_verdict`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Gate.** `accepted` is true only for `Verdict::Done`. `final_gate` returns `Ok` only then. `Blocked`, `Wait`, `Continue`, and `Reject` are `Err`, and a missing verdict word is `Reject`.

**Permit.** `judge` calls `pool.acquire` once, and the pool limit stays `WORKER_BOUND`, 8, and the critic doesn't construct another pool. `max_seen` after one judgment is 1.

**Context.** `judge` clones the pool's `Arc` and locks `notes`, and it doesn't `push`. The notes length in the test is the same before and after, and the reply isn't written into the shared context.

**Named.** `DEFAULT_MAX_TURNS` is 20. `DEFAULT_JUDGE_TIMEOUT_SECS` is 30. `SNIPPET_CHARS` is 4000, and the timeout isn't slept, and the turn cap isn't looped. `DEFAULT_GATE_TIMEOUT_SECONDS` of 300 and `DEFAULT_GATE_MAX_RETRIES` of 3 stay in Hermes until chapter 33.

**Left for later.** The continuation loop, the wait target, and the quality-gate shell commands are chapter 33. `JUDGE_SYSTEM_PROMPT` isn't copied into this crate.

## Chapter summary

- **Verdicts.** `done` accepts. `blocked`, `wait`, and `continue` parse and don't accept, and a missing verdict word is `Reject`.
- **Permit.** One `acquire` per `judge`, because a judge that ran wide would starve the workers it's judging. `as_tool` and `final_gate` both call `judge`, and the pool's bound stays 8, and `max_seen` after one judgment is 1.
- **Notes.** The critic locks them and doesn't push.
- **Snippet.** At most 4,000 bytes of `raw`, on a character boundary.
- **Constants.** 20 turns and 30 seconds, returned by `limits`, and the 30 seconds are not slept.
- **Time.** `judge_one_permit` mean `278.9 ns`, and that is `mean.point_estimate`.

## What you implement

`my-harness/tests/ch32.rs` is the milestone.

```rust
use my_harness::chapter_32_critic_does_the_work;

#[test]
fn the_critic_reads_the_summary_and_does_not_do_the_work() {
    assert!(!chapter_32_critic_does_the_work());
}
```

The stub panics.

```rust
pub fn chapter_32_critic_does_the_work() -> bool {
    todo!("chapter 32")
}
```

`todo!("chapter 32")` is the panic, and the test wants `chapter_32_critic_does_the_work()` to be false.

## What's next

Chapter 33 composes these patterns, and a supervisor can fan out on the same pool. A handoff runs when ownership should move. The continuation loop, the wait target, and the quality-gate shell commands wrap this critic there. `done` stays the accept.
