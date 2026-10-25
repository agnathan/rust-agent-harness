# Chapter 28: The Sequential Pipeline

Passing the next stage the same context costs `9.15 ns`. Cloning the hundred messages instead costs `6.12 µs`, and dividing those two published means makes the copy about 670 times the pointer. `hand_off` returns the `Arc` it was given, `run` pushes each note onto that same allocation, and the pipeline takes the cheap path. A continuously improving harness can only review the run it actually had, so outline, draft, critique, and revise have to share one object.

The demo's task is one sentence, "Write a note on rename." Outline answers `three steps`, the draft answers `the draft`, the critique answers `shorter`, and the revision answers `the revision`, which is the line the binary prints: `revise: the revision`. I expected the critique to be a reading of the draft. The replies are written into the stage array before `run`, so the critique never reads it, but the four notes still land on one `Arc`, in that order, where a later stage could look. One agent can already remember and write a skill. What changes here is that more than one stage has to see the same work.

`Context` holds the messages and a mutex of notes, `Stage` is a name plus a reply, and `STAGE_NAMES` is outline, draft, critique, revise. `run` follows the slice it's given. The step is chapter 27's harness plus `pipeline`, at `steps/ch28-pipeline`, package `ch28-pipeline`, and skill files and memory files stay where chapters 21 and 25 put them. Reflection can still be enqueued after the run, while curation stays chapter 27's job, off to the side, because this chapter attaches the context and leaves the decision about what is worth keeping to that job. Chapter 29 calls specialists under a bound. Chapter 30's handoff is a filtered transfer of ownership, a different operation from this pointer, and chapter 33 is where a supervisor fans out and hands off only when ownership should move.

`cargo run --locked -p ch28-pipeline --bin ch28-pipeline` prints `revise: the revision`. `my-harness/tests/ch28.rs` checks that the next stage receives the `Arc`.

| Section | Step | What it adds |
|---|---|---|
| 28.1 | `steps/ch28-pipeline` | `Message`, `Context`, `Stage`, `STAGE_NAMES` |
| 28.2 | `steps/ch28-pipeline` | `run` appends one note per stage on the shared `Arc` |
| 28.3 | `steps/ch28-pipeline` | `hand_off` returns the same `Arc` |
| 28.4 | `steps/ch28-pipeline` | the demo's four scripted replies |
| 28.5 | `steps/ch28-pipeline` | `arc_pass` and `copy_transcript` |

> **How Hermes does it**
>
> I read `agent/turn_request_assembly.py` (`_append_moa_context`) on Hermes `main` at `ab3fe9dffb0a`.
>
> `_append_moa_context` runs MoA reference models and appends aggregated context to the last user message. It calls `agent.moa_loop.aggregate_moa_context`. There is no sequential outline, draft, critique, revise pipeline in that file.
>
> **Adopted.** Extra context is attached. The user's message is not copied into a second transcript. Later work reads what earlier work put on the same context.
>
> **Adapted.** Four scripted stages share one `Arc<Context>`. `hand_off` returns the same `Arc`. The replies are values in the stage slice. A separate bench clones 100 messages so the copy has a mean. The pipeline does not call that clone.
>
> **Deferred.** Live reference models. Chapter 29 runs specialists under a bound. Those specialists stay scripted too.

## What this chapter decides

`Context` holds `messages` and a `Mutex` of notes. `Stage` is a name and a reply. `STAGE_NAMES` is `outline`, `draft`, `critique`, `revise`.

`run` walks the slice, and each stage locks the notes, pushes `name: reply`, and drops the lock at the end of that statement before the function returns the `Arc` it was given. `hand_off` returns its argument, so `Arc::ptr_eq` holds for the value that went in and the value that came out.

`demo` builds one user message, runs the four stages, checks the pointer, and returns the last note, `revise: the revision`. The replies are literals, and nothing in `demo` reads an environment variable or opens a socket.

`arc_pass` times `hand_off`. `copy_transcript` times a clone of the message vec, which is the other version, and section 28.3 is where that contrast sits. `run` calls the pointer.

## 28.1 The context is one allocation

The module opens with the standard-library pieces this file uses.

```rust
use std::sync::{Arc, Mutex};
```

`Arc` and `Mutex` come from `std::sync`, and this file stays on one thread: four stages, one after another, with the next stage receiving the pointer. `Condvar`, atomics, and extra threads arrive when a later chapter shares that context across workers.

`Message` is the transcript row.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub text: String,
}
```

`Debug` is the print derive. `Clone` is what makes `messages.clone()` legal in the bench. `PartialEq` and `Eq` let a test compare two messages, and both fields are owned `String`s. A role of `"user"` in the demo is turned into a `String` with `.into()`, and the text is the same. Cloning a `Message` clones both strings, and that is a heap copy of the characters, not a copy of a pointer.

`Context` is the object the stages share.

```rust
pub struct Context {
    pub messages: Vec<Message>,
    pub notes: Mutex<Vec<String>>,
}
```

`Context` has no `Clone` derive, because `Mutex` doesn't implement `Clone`, so the way to hand the value to the next stage is an `Arc` and the way to put a note on it's to lock `notes`.

`messages` is a `Vec<Message>`, and `run` leaves it alone. The tests check that a three-message vec is still three messages and that the first text is still `message 0`, which means the transcript stays the vec that was built before `run`.

`notes` is `Mutex<Vec<String>>`, and the mutex is the lock. The vec inside is the log of stage replies, and it starts empty in the demo and in the tests. After four stages it has four strings.

> **Rust sidebar: `Arc`**
>
> - `Arc::new` puts the `Context` in a heap allocation and returns a pointer with a strong count of one. `Arc::clone` increments the strong count and returns another pointer to the same allocation. It does not clone `messages` and it does not clone the notes vec.
> - `Arc::ptr_eq` compares the address of the allocation. Two `Arc`s that came from the same `Arc::new` compare equal. Two `Arc::new` calls compare unequal, even when the messages inside look the same.
> - The `Context` is freed when the last `Arc` drops. `run` returns the `Arc`, so the caller still holds it.
> - `Option`, `Result`, enums, and `Arc` are taught in [chapter 1](chapter-01-first-contact.md).

> **Rust sidebar: `Mutex`**
>
> - `notes.lock()` returns a `Result`. The `Ok` value is a `MutexGuard`. The guard lets you use the `Vec<String>` as if you held `&mut` to it. When the guard drops, the mutex unlocks.
> - `expect("notes")` turns that `Result` into the guard. If the mutex is poisoned, `expect` panics with the message `notes`. Poison means a thread panicked while holding the guard. This chapter's `run` is one thread. The `expect` is still the unlock-or-panic path the source uses.
> - The guard in `run` is a temporary. It lives until the end of the statement that calls `push`. The next stage locks again. Four stages are four lock and unlock pairs. They do not hold the mutex at the same time, because they do not run at the same time.

`Stage` is the unit `run` walks.

```rust
pub struct Stage {
    pub name: &'static str,
    pub reply: String,
}
```

`name` is `&'static str`, baked into the binary, and both `STAGE_NAMES` and the demo use those literals. `reply` is an owned `String` because a reply is data the caller built. `Stage` has no `Clone` derive, and the loop in `run` only borrows each stage.

The conventional order is a const.

```rust
pub const STAGE_NAMES: [&str; 4] = ["outline", "draft", "critique", "revise"];
```

The type is an array of four string slices. The test builds a `Vec<Stage>` by mapping over the const, and the demo writes the same four names by hand, while `run` itself follows whatever slice it was given, so a caller who passed `revise` first would see the revise note first.

The module comment above the `use` line is the contract in one place: four stages, the same `Arc`, the transcript not cloned between stages. The functions below are that contract.

## 28.2 `run` appends a note and returns the same `Arc`

`run` is the whole pipeline.

```rust
pub fn run(stages: &[Stage], context: Arc<Context>) -> Arc<Context> {
    for stage in stages {
        context
            .notes
            .lock()
            .expect("notes")
            .push(format!("{}: {}", stage.name, stage.reply));
    }
    context
}
```

The first parameter is `&[Stage]`, a shared borrow, so the function can read `name` and `reply` while the caller's vec stays the caller's. The second parameter is `Arc<Context>` by value: the caller moves an `Arc` in, or moves a clone of one, and the return type is that same `Arc<Context>`. A poisoned lock panics in `expect`, and an empty slice returns the same `Arc` with the notes left as they were.

`for stage in stages` yields `&Stage`. `stage.name` is `&str`. `stage.reply` is `&String` in the format arguments, which is enough for `Display`, and nothing in the loop writes the stage.

`format!("{}: {}", stage.name, stage.reply)` allocates a new `String`, and the demo's last call formats `revise` and `the revision` into `revise: the revision`. The test formats each name against a reply that is the same word, so the four notes are `outline: outline`, `draft: draft`, `critique: critique`, and `revise: revise`. The colon and the space are the format string, and they are not stored on the `Stage`.

`push` appends that string to the vec inside the mutex. If the vec's buffer is full, `push` allocates a larger buffer and moves the existing note strings into it. That reallocation is the notes vec's heap buffer, and it isn't the `Context` allocation. `Arc::ptr_eq` stays true. The `messages` vec is a different field and isn't touched, so its buffer stays where `Arc::new` put it.

Then the function returns `context`, and that is a move of the `Arc` the parameter already holds. The strong count doesn't go up on the return, and it went up only if the caller had called `Arc::clone` before the call.

I read this function looking for a second transcript, and the statements are the lock, the format, the push, and the return. There is no `messages.clone()`, and there is no `Arc::new(Context { ... })` inside the loop, and the last user message is the one the caller stored. The stage notes sit beside it, on `notes`, which is the harness stand-in for context attached to the same object.

The test that pins this is `four_stages_share_the_arc_and_keep_message_order`. It builds a context of three messages with the helper below, maps `STAGE_NAMES` into stages whose reply is the name, clones the `Arc`, and calls `run`.

```rust
fn context(n: usize) -> Arc<Context> {
    Arc::new(Context {
        messages: (0..n)
            .map(|i| Message {
                role: "user".into(),
                text: format!("message {i}"),
            })
            .collect(),
        notes: Mutex::new(Vec::new()),
    })
}
```

`(0..n)` is the range. `map` builds one `Message` per index. `collect` turns that iterator into `Vec<Message>`. `Mutex::new(Vec::new())` is an unlocked empty log, and for `n` of 3 the texts are `message 0`, `message 1`, and `message 2`.

```rust
#[test]
fn four_stages_share_the_arc_and_keep_message_order() {
    let context = context(3);
    let stages: Vec<Stage> = STAGE_NAMES
        .iter()
        .map(|name| Stage {
            name,
            reply: (*name).into(),
        })
        .collect();
    let out = run(&stages, Arc::clone(&context));
    assert!(Arc::ptr_eq(&context, &out));
    assert_eq!(out.messages.len(), 3);
    assert_eq!(out.messages[0].text, "message 0");
    let notes = out.notes.lock().unwrap();
    assert_eq!(
        notes.as_slice(),
        [
            "outline: outline",
            "draft: draft",
            "critique: critique",
            "revise: revise",
        ]
    );
}
```

`STAGE_NAMES.iter()` yields `&&str`, and the field `name` wants `&str`. Rust's deref coercion lets that `&&str` sit in a `&str` field here through the pattern the compiler accepts for this map. The reply is `(*name).into()`, an owned copy of the same word. I am not going to pretend the stage read the previous note, and the reply was set before `run`. What `run` proves is that the note landed on the shared object, in slice order, without replacing the messages.

`Arc::clone(&context)` is the second pointer. `out` is that pointer after `run` returns it. `Arc::ptr_eq(&context, &out)` is the assertion that they are one allocation. `out.messages.len()` is 3, and the first text is still `message 0`. The notes lock in the test uses `unwrap`, which panics on poison the same way `expect` does, with a different panic message. `as_slice()` borrows the four strings for `assert_eq!`, and the expected array is the four formatted lines, in `STAGE_NAMES` order.

That is the pipeline: four pushes onto one `Arc`, and the messages stay.

## 28.3 `hand_off` is the pointer the next stage is given

The function the bench times is smaller than `run`.

```rust
/// What the next stage is given. The pointer is the same allocation.
pub fn hand_off(context: Arc<Context>) -> Arc<Context> {
    context
}
```

The doc comment is the behavior, and the body is the parameter: returning `context` moves the `Arc` out, with no lock, no format, and no loop. Call `run` when the notes should grow. Call `hand_off` and check `Arc::ptr_eq` when the question is what "the next stage receives" means.

The test uses a context of 100 messages, which is the same size the bench builds.

```rust
#[test]
fn hand_off_is_the_same_pointer() {
    let context = context(100);
    let next = hand_off(Arc::clone(&context));
    assert!(Arc::ptr_eq(&context, &next));
    assert_eq!(next.messages.len(), 100);
}
```

`context(100)` allocates one `Context` whose vec holds `message 0` through `message 99`. `Arc::clone` makes a second pointer, `hand_off` moves that pointer into `next`, and `ptr_eq` is true because the vec was neither rebuilt nor dropped. The original `context` binding is still alive, so the strong count stays two until the test returns and both drop.

`hand_off` stays its own function so the bench has a name that is only the pointer. Folding it into `run` would time the four `format!` calls as well, and those allocations are real work, but they aren't the question this chapter is measuring. The question is what it costs to give the next stage the same context.

The other version, the one this function doesn't do, is the clone the bench times:

```rust
let copied = black_box(&context.messages).clone();
```

That line lives in `copy_transcript`, in the bench file, and the full function is in section 28.5. `run` and `hand_off` never call it. The mean is on the page so a later edit that adds `messages.clone()` inside `hand_off` is a change you can see beside `9.15 ns`.

The user's message in the demo is one `Message` in that vec. Attaching a stage note leaves "Write a note on rename." as the single user row, because the stage line goes on `notes` instead of editing that message's `text`. The Hermes function appends aggregated context onto the last user message, and this harness keeps the shared object and writes the other field.

## 28.4 The demo runs the four names and prints the last note

`demo` is what the binary prints.

```rust
pub fn demo() -> String {
    let context = Arc::new(Context {
        messages: vec![Message {
            role: "user".into(),
            text: "Write a note on rename.".into(),
        }],
        notes: Mutex::new(Vec::new()),
    });
    let stages = [
        Stage {
            name: "outline",
            reply: "three steps".into(),
        },
        Stage {
            name: "draft",
            reply: "the draft".into(),
        },
        Stage {
            name: "critique",
            reply: "shorter".into(),
        },
        Stage {
            name: "revise",
            reply: "the revision".into(),
        },
    ];
    let out = run(&stages, Arc::clone(&context));
    assert!(Arc::ptr_eq(&context, &out));
    let last = out
        .notes
        .lock()
        .expect("notes")
        .last()
        .cloned()
        .unwrap_or_default();
    last
}
```

`Arc::new` builds the context, and the vec macro builds one message. The role is `user`, and the text is `Write a note on rename.` The notes mutex starts empty.

The stage array is four `Stage` values. `reply` uses `.into()` so a `&str` literal becomes a `String`, and the names match `STAGE_NAMES`. The replies are the demo's own words: `three steps`, `the draft`, `shorter`, `the revision`.

`run(&stages, Arc::clone(&context))` borrows the array as a slice and passes a second `Arc`, and after the call, the notes are exactly these four strings, in this order:

`outline: three steps`

`draft: the draft`

`critique: shorter`

`revise: the revision`

`assert!(Arc::ptr_eq(&context, &out))` runs inside `demo`. If a later edit made `run` return a new `Arc`, the binary would panic before it printed. The unit test below doesn't repeat that assert, and it checks the string `demo` returns. The pointer check is in the function the binary calls.

`last()` on the notes vec is `Option<&String>`. `cloned()` turns that into `Option<String>` by cloning the last note, so the owned string doesn't borrow the guard. `unwrap_or_default()` on `Option<String>` is `""` when the vec is empty, and this demo always pushes four notes, so the value is `revise: the revision`. The `Result` of the lock is handled by `expect` before `last` runs. `demo` itself returns `String`, not `Result`, and the binary prints that string.

The binary is `src/bin/chapter_demo.rs`.

```rust
use harness_core::pipeline::demo;

fn main() {
    println!("{}", demo());
}
```

`Cargo.toml` names the package `ch28-pipeline` and the binary `ch28-pipeline`, with `path = "src/bin/chapter_demo.rs"`, and two other binaries live in the same package: `ch28_mcp_fixture` and `sandbox-worker`. The run line names `--bin ch28-pipeline` so Cargo doesn't have to guess.

The test is one equality.

```rust
#[test]
fn the_demo_ends_on_the_revision() {
    assert_eq!(harness_core::pipeline::demo(), "revise: the revision");
}
```

The demo reads no key. The four replies are literals, and nothing in `demo` reads an environment variable or opens a socket.

The critique reply is `shorter`, and the revise reply is `the revision`. The revise stage doesn't read the string `shorter` in code, and both strings were written in the array before `run`. What the shared `Arc` makes true is that a later stage could lock `notes` and read `outline: three steps` before it pushed its own line. This chapter's scripted replies don't do that read, and the pointer is ready for a stage that does. Live models that would read it are deferred.

## 28.5 The benches time the pointer and the copy

The bench file is `crates/harness-core/benches/pipeline.rs`. `Cargo.toml` registers it as `[[bench]] name = "pipeline"` with `harness = false`, so the file brings its own `criterion_main`. The workflow runs this bench as section `28`, label `chapter 28 pipeline`, package `ch28-pipeline`.

`sample` builds the context both benches share. It's called before `b.iter`, so the 100 messages are not part of the timed closure.

```rust
fn sample() -> Arc<Context> {
    Arc::new(Context {
        messages: (0..100)
            .map(|i| Message {
                role: "user".into(),
                text: format!("message {i} about the rename procedure"),
            })
            .collect(),
        notes: Mutex::new(Vec::new()),
    })
}
```

Index 0's text is `message 0 about the rename procedure`. Index 99 is the same shape, and one hundred owned strings, one `Arc`.

`arc_pass` times the handoff.

```rust
fn arc_pass(c: &mut Criterion) {
    let context = sample();
    c.bench_function("arc_pass", |b| {
        b.iter(|| {
            let next = hand_off(black_box(Arc::clone(&context)));
            black_box(Arc::ptr_eq(&context, &next));
        });
    });
}
```

`bench_function` names the sample `arc_pass`. `b.iter` is the timed loop. Inside, `Arc::clone` bumps the strong count, `black_box` keeps the compiler from deleting the clone, `hand_off` moves that `Arc`, and `Arc::ptr_eq` compares it with the original. `black_box` comes from `std::hint::black_box`, and it's an identity function the optimizer has to treat as unknown. The result of `ptr_eq` is true for this sample, and the bench doesn't assert that. It passes the bool through `black_box` so the compare stays in the sample.

No `Stage` is built and no `format!` runs. No note is pushed. The mean for this function is `9.15 ns`, `mean.point_estimate` from the Benchmarks workflow. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

`copy_transcript` times the clone the pipeline doesn't do.

```rust
fn copy_transcript(c: &mut Criterion) {
    let context = sample();
    c.bench_function("copy_transcript", |b| {
        b.iter(|| {
            let copied = black_box(&context.messages).clone();
            black_box(copied);
        });
    });
}
```

`&context.messages` is `&Vec<Message>`. `clone` on that vec clones every `Message`, and each `Message` clones two `String`s. `black_box` on the borrow and on the result keeps the clone alive for the measurement. The notes mutex isn't cloned. `Message`'s `Clone` derive is why this line compiles. `Context` has no such derive, which is why the pipeline can't "just clone the context" and why the bench has to name the vec.

The mean for this function is `6.12 µs`. Same workflow, same rule: `mean.point_estimate`.

```rust
criterion_group!(benches, arc_pass, copy_transcript);
criterion_main!(benches);
```

The group is both functions, and a change to `hand_off` shows up in `arc_pass`. A change that made `hand_off` clone `messages` would be a different function from the one in `pipeline.rs`. I would see it because `copy_transcript` would no longer be the only clone in the chapter.

The 100 messages exist so the copy is a transcript-sized vec, not an empty one. The pipeline's demo uses one message. The test `hand_off_is_the_same_pointer` uses 100 and checks the length, so the bench size and the test size match.

The bench file's imports are the whole dependency list for those two functions.

```rust
use std::hint::black_box;
use std::sync::{Arc, Mutex};

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::pipeline::{hand_off, Context, Message};
```

`Criterion` is the argument `arc_pass` and `copy_transcript` take as `&mut Criterion`. `criterion_group!` and `criterion_main!` are the macros at the bottom of the file. `hand_off`, `Context`, and `Message` are the only items pulled from `harness_core::pipeline`. `run` isn't imported here, and the bench that times the pipeline's handoff doesn't call `run`. That split is deliberate. `arc_pass` measures the pointer, and the four `format!` calls stay in the tests and in `demo`.

The test file imports the rest.

```rust
use std::sync::{Arc, Mutex};

use harness_core::pipeline::{hand_off, run, Context, Message, Stage, STAGE_NAMES};
```

`Stage` and `STAGE_NAMES` are for the four-stage test. `Mutex` is for the `context` helper. The integration tests sit in `tests/pipeline.rs`, so they are a separate crate that links `harness_core` the way the binary does. They can call `demo` through `harness_core::pipeline::demo` because `demo` is `pub`, and they can't call a private function. Nothing in `pipeline.rs` is private. `run`, `hand_off`, and `demo` are all `pub`.

`demo` clones the last note out of the guard so the returned `String` doesn't borrow `out`, and that clone is one string, not the message vec. `copy_transcript` never builds the four notes. Its clone is the hundred `Message` values from `sample`, each containing `about the rename procedure`, which is longer than the test helper's `message {i}` on purpose: the test checks length and pointer equality, and the bench gives the clone a transcript-sized body. `9.15 ns` is only `arc_pass`. `6.12 µs` is only `copy_transcript`, and the opening's "about 670 times" is those two means divided, not a third measurement.

## Tests

Three tests in `crates/harness-core/tests/pipeline.rs`. `four_stages_share_the_arc_and_keep_message_order` checks `Arc::ptr_eq`, the three messages, and the four notes. `hand_off_is_the_same_pointer` checks `ptr_eq` and a length of 100. `the_demo_ends_on_the_revision` expects `revise: the revision`.

The package also carries the tests copied forward from chapter 27. Nextest runs all of them.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch28-pipeline
```

```text
Starting 146 tests across 35 binaries
     Summary [   0.325s] 146 tests run: 146 passed, 0 skipped
```

```bash
cargo run --locked -p ch28-pipeline --bin ch28-pipeline
```

```text
revise: the revision
```

The binary builds one context, runs outline, draft, critique, and revise, and prints the last note. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `arc_pass` | 9.15 ns | the next stage receives the same `Arc` | `hand_off` of a cloned pointer, then `Arc::ptr_eq` |
| `copy_transcript` | 6.12 µs | the copy `run` does not do | `clone` of 100 messages |

`arc_pass` builds the 100-message context before `b.iter`, and the timed closure clones the `Arc`, calls `hand_off`, and compares pointers. Both calls go through `black_box`, and no stage reply is formatted.

`copy_transcript` uses the same context, and the timed closure clones `messages`. `run` doesn't call this. `hand_off` doesn't call this.

The last note is `revise: the revision`.

The means are `mean.point_estimate` from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `28`. Bench ids: `28/arc_pass`, `28/copy_transcript`, `26/arc_pass`, `26/copy_transcript`, and how to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`run` locks `notes` and grows that log, so the user's sentence stays the one string the demo stored. A stage that pushed a new `Message` would grow the transcript instead, which is a different chapter.

`hand_off` returns the pointer and leaves every role in place, including a `scratch` row. Chapter 30 is the filter, and this chapter's test messages are all role `user`.

`copy_transcript` lives in the bench file so the copy has a mean beside the pointer. Calling it from `run` would build the second transcript this chapter is here to avoid, which is the contrast in section 28.3.

`demo` asserts `ptr_eq` before it returns, so a broken `run` panics the binary and the string comparison never gets its turn. Fix the pointer, then read the note.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). This chapter attaches context, and skill files and memory files stay where chapters 21 and 25 put them.

**Context.** The next stage gets the same `Arc`, and the messages stay in the vec that was built before `run`. Handing that pointer across measures `9.15 ns`. Cloning the 100 messages measures `6.12 µs`, and the pipeline takes the first path.

**Order.** The stage slice is the order. `STAGE_NAMES` is outline, draft, critique, revise. `run` follows the slice it was given.

**Left for later.** Chapter 29 bounds how many specialists run at once and closes M5 and M6 for child tools. Chapter 30 filters a transcript before a handoff. Live MoA reference models stay out. `aggregate_moa_context` is the Hermes call this chapter didn't port.

## Chapter summary

- **Stages.** Outline, draft, critique, revise, in slice order, on the rename note the demo stores.
- **Context.** One `Arc<Context>`. `hand_off` returns it, `Arc::ptr_eq` holds, and `messages` stays the vec from before `run`.
- **Notes.** Each stage pushes `name: reply` under the mutex, and the demo's last line is `revise: the revision`.
- **Replies.** Scripted literals, with no key and no model.
- **Time.** `arc_pass` mean `9.15 ns`. `copy_transcript` mean `6.12 µs`, about 670 times the pointer when you divide the two means. Both are `mean.point_estimate`.

## What you implement

`my-harness/tests/ch28.rs` is the milestone.

```rust
use my_harness::chapter_28_same_context;

#[test]
fn the_next_stage_receives_arc_context() {
    assert!(chapter_28_same_context(1, 1));
}
```

The stub in `my-harness/src/milestones.rs` panics.

```rust
pub fn chapter_28_same_context(left: usize, right: usize) -> bool {
    let _ = (left, right);
    todo!("chapter 28")
}
```

`todo!("chapter 28")` is the panic, and the test wants `chapter_28_same_context(1, 1)` to be true.

## What's next

A supervisor is next: specialists are jobs on one pool, the pool's limit is the same 8 chapter 3 uses for in-flight tool work, and a child at depth 1 is refused the blocked tools. A reflection note is a string on a queue. It waits there, and chapter 26's reviewer still starts only when something outside the team turn asks.
