# Chapter 28: The Sequential Pipeline

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 27 can mark a skill stale and archive it. It does not hand a piece of writing from one stage to the next. This chapter runs four stages in order: outline, draft, critique, revise. Each stage is a name and a scripted reply. The next stage receives the same `Arc<Context>`. `hand_off` returns that same `Arc`. The message vec is one allocation for the whole run.

The step crate is `steps/ch28-pipeline`. The package is `ch28-pipeline`. The library name is `harness_core`. It is chapter 27's harness copied forward, plus `pipeline`. `lib.rs` adds `pub mod pipeline`. The comment at the top of `lib.rs` is still the chapter 10 comment about the turn table. That sentence stays in the copied file. The new module is the one this chapter reads. There is no path dependency on the `ch27-curation` package.

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

[personal note: a draft you wish had gone through a critique stage before you saw it]

## What this chapter decides

`Context` holds `messages` and a `Mutex` of notes. `Stage` is a name and a reply. `STAGE_NAMES` is `outline`, `draft`, `critique`, `revise`.

`run` walks the slice. Each stage locks the notes, pushes `name: reply`, and drops the lock at the end of that statement. The function returns the `Arc` it was given. `hand_off` returns its argument. `Arc::ptr_eq` is true for the value that went in and the value that came out.

`demo` builds one user message, runs the four stages, checks the pointer, and returns the last note: `revise: the revision`. No key. No model.

The bench `arc_pass` times `hand_off`. The bench `copy_transcript` times a clone of the message vec. `run` does not call that clone.

## 28.1 The context is one allocation

The module opens with the standard-library pieces this file uses.

```rust
use std::sync::{Arc, Mutex};
```

`Arc` and `Mutex` live in `std::sync`. This file does not use `Condvar`, atomics, or threads. Those arrive when a later chapter shares the context across workers. Here one thread walks four stages, and the sharing is the pointer the next stage receives.

`Message` is the transcript row.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub text: String,
}
```

`Debug` is the print derive. `Clone` is what makes `messages.clone()` legal in the bench. `PartialEq` and `Eq` let a test compare two messages. Both fields are owned `String`s. A role of `"user"` in the demo is turned into a `String` with `.into()`. The text is the same. Cloning a `Message` clones both strings. That is a heap copy of the characters, not a copy of a pointer.

`Context` is the object the stages share.

```rust
pub struct Context {
    pub messages: Vec<Message>,
    pub notes: Mutex<Vec<String>>,
}
```

There is no `Clone` derive on `Context`. `Mutex` does not implement `Clone`, so a derived `Clone` would not be available even if I asked for one. The way to hand this value to the next stage is an `Arc`. The way to put a note on it is to lock `notes`.

`messages` is a `Vec<Message>`. `run` never writes it. The tests check that a three-message vec is still three messages, and that the first text is still `message 0`. The transcript stays the vec that was built before `run`.

`notes` is `Mutex<Vec<String>>`. The mutex is the lock. The vec inside is the log of stage replies. It starts empty in the demo and in the tests. After four stages it has four strings.

> **Rust sidebar: `Arc`**
>
> - `Arc::new` puts the `Context` in a heap allocation and returns a pointer with a strong count of one. That pointer is cheap to clone. `Arc::clone` increments the strong count and returns another pointer to the same allocation. It does not clone `messages` and it does not clone the notes vec.
> - Moving an `Arc` into a function does not increment the count. The move hands the pointer over. Returning that same pointer hands it back. The allocation stays.
> - `Arc::ptr_eq` compares the address of the allocation. Two `Arc`s that came from the same `Arc::new` compare equal. Two `Arc::new` calls compare unequal, even when the messages inside look the same.
> - Dropping an `Arc` decrements the count. The `Context` is freed when the last `Arc` drops. `run` returns the `Arc`, so the caller still holds it.

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

`name` is `&'static str`. The string is baked into the binary. `STAGE_NAMES` and the demo both use those literals. `reply` is an owned `String`, because a reply is data the caller built. `Stage` has no `Clone` derive. The loop in `run` borrows each stage. It does not need to own a copy.

The conventional order is a const.

```rust
pub const STAGE_NAMES: [&str; 4] = ["outline", "draft", "critique", "revise"];
```

The type is an array of four string slices. `run` does not read this const. The test builds a `Vec<Stage>` by mapping over it, and the demo writes the same four names by hand. If a caller passed `revise` first, `run` would push the revise note first. The order is the slice order. The const is the order this chapter's tests use.

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

The first parameter is `&[Stage]`, a shared borrow of a slice. The function cannot push a new stage onto the caller's vec, and it cannot replace a reply. It can read `name` and `reply`. The second parameter is `Arc<Context>` by value. The caller moves an `Arc` in, or moves a clone of one. The return type is the same `Arc<Context>`. There is no `Result`. A poisoned lock panics in `expect`. An empty slice is a successful return of the same `Arc` with the notes left as they were.

`for stage in stages` yields `&Stage`. `stage.name` is `&str`. `stage.reply` is `&String` in the format arguments, which is enough for `Display`. Nothing in the loop writes the stage.

`format!("{}: {}", stage.name, stage.reply)` allocates a new `String`. The demo's last call formats `revise` and `the revision` into `revise: the revision`. The test formats each name against a reply that is the same word, so the four notes are `outline: outline`, `draft: draft`, `critique: critique`, and `revise: revise`. The colon and the space are the format string. They are not stored on the `Stage`.

`push` appends that string to the vec inside the mutex. If the vec's buffer is full, `push` allocates a larger buffer and moves the existing note strings into it. That reallocation is the notes vec's heap buffer. It is not the `Context` allocation. `Arc::ptr_eq` stays true. The `messages` vec is a different field and is not touched, so its buffer stays where `Arc::new` put it.

Then the function returns `context`. That is a move of the `Arc` the parameter already holds. The strong count does not go up on the return. It went up only if the caller had called `Arc::clone` before the call.

I read this function looking for a second transcript. The statements are the lock, the format, the push, and the return. There is no `messages.clone()`. There is no `Arc::new(Context { ... })` inside the loop. The last user message is the one the caller stored. The stage notes sit beside it, on `notes`, which is the harness stand-in for context attached to the same object.

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

`(0..n)` is the range. `map` builds one `Message` per index. `collect` turns that iterator into `Vec<Message>`. `Mutex::new(Vec::new())` is an unlocked empty log. For `n` of 3 the texts are `message 0`, `message 1`, and `message 2`.

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

`STAGE_NAMES.iter()` yields `&&str`. The field `name` wants `&str`. Rust's deref coercion lets that `&&str` sit in a `&str` field here through the pattern the compiler accepts for this map. The reply is `(*name).into()`, an owned copy of the same word. I am not going to pretend the stage read the previous note. The reply was set before `run`. What `run` proves is that the note landed on the shared object, in slice order, without replacing the messages.

`Arc::clone(&context)` is the second pointer. `out` is that pointer after `run` returns it. `Arc::ptr_eq(&context, &out)` is the assertion that they are one allocation. `out.messages.len()` is 3. The first text is still `message 0`. The notes lock in the test uses `unwrap`, which panics on poison the same way `expect` does, with a different panic message. `as_slice()` borrows the four strings for `assert_eq!`. The expected array is the four formatted lines, in `STAGE_NAMES` order.

That is the pipeline. Four pushes. One `Arc`. The messages stay.

## 28.3 `hand_off` is the pointer the next stage is given

The function the bench times is smaller than `run`.

```rust
/// What the next stage is given. The pointer is the same allocation.
pub fn hand_off(context: Arc<Context>) -> Arc<Context> {
    context
}
```

The doc comment is the behavior. The body is the parameter. Returning `context` moves the `Arc` out. There is no lock, no format, and no loop. A reader who wants the notes to grow calls `run`. A reader who wants to know what "the next stage receives" means calls `hand_off` and checks `Arc::ptr_eq`.

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

`context(100)` allocates one `Context` whose vec holds `message 0` through `message 99`. `Arc::clone` makes a second pointer. `hand_off` moves that second pointer into `next`. `ptr_eq` is true. The length is 100 because `hand_off` did not rebuild the vec and did not drop it. The original `context` binding is still alive, so the strong count is two until the test function returns and both drop.

I keep `hand_off` as its own function so the bench has a name that is only the pointer. Folding it into `run` would time the four `format!` calls as well. Those allocations are real work. They are not the question this chapter is measuring. The question is what it costs to give the next stage the same context.

`copy_transcript` answers the other question: what it costs to clone the vec. That function is not in `pipeline.rs`. It lives in the bench file. `run` and `hand_off` do not call it. I want the mean on the page so a later change that adds `messages.clone()` inside `hand_off` is a change you can see.

The user's message in the demo is one `Message` in that vec. Attaching a stage note does not push a second copy of "Write a note on rename." onto `messages`. The Hermes function I read appends aggregated context onto the last user message. This harness puts the stage line on `notes` instead of editing that message's `text`. The shared object is the idea I took. The field I write is the adaptation.

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

`Arc::new` builds the context. The vec macro builds one message. The role is `user`. The text is `Write a note on rename.` The notes mutex starts empty.

The stage array is four `Stage` values. `reply` uses `.into()` so a `&str` literal becomes a `String`. The names match `STAGE_NAMES`. The replies are the demo's own words: `three steps`, `the draft`, `shorter`, `the revision`.

`run(&stages, Arc::clone(&context))` borrows the array as a slice and passes a second `Arc`. After the call, the notes are exactly these four strings, in this order:

`outline: three steps`

`draft: the draft`

`critique: shorter`

`revise: the revision`

`assert!(Arc::ptr_eq(&context, &out))` runs inside `demo`. If a later edit made `run` return a new `Arc`, the binary would panic before it printed. The unit test below does not repeat that assert. It checks the string `demo` returns. The pointer check is in the function the binary calls.

`last()` on the notes vec is `Option<&String>`. `cloned()` turns that into `Option<String>` by cloning the last note, so the owned string does not borrow the guard. `unwrap_or_default()` on `Option<String>` is `""` when the vec is empty. This demo always pushes four notes, so the value is `revise: the revision`. The `Result` of the lock is handled by `expect` before `last` runs. `demo` itself returns `String`, not `Result`. The binary prints that string.

The binary is `src/bin/chapter_demo.rs`.

```rust
use harness_core::pipeline::demo;

fn main() {
    println!("{}", demo());
}
```

`Cargo.toml` names the package `ch28-pipeline` and the binary `ch28-pipeline`, with `path = "src/bin/chapter_demo.rs"`. Two other binaries live in the same package: `ch28_mcp_fixture` and `sandbox-worker`. The run line names `--bin ch28-pipeline` so Cargo does not have to guess.

The test is one equality.

```rust
#[test]
fn the_demo_ends_on_the_revision() {
    assert_eq!(harness_core::pipeline::demo(), "revise: the revision");
}
```

No key. The four replies are literals. Nothing in `demo` reads an environment variable or opens a socket.

The critique reply is `shorter`. The revise reply is `the revision`. The revise stage does not read the string `shorter` in code. Both strings were written in the array before `run`. What the shared `Arc` makes true is that a later stage could lock `notes` and read `outline: three steps` before it pushed its own line. This chapter's scripted replies do not do that read. The pointer is ready for a stage that does. Live models that would read it are deferred.

## 28.5 The benches time the pointer and the copy

The bench file is `crates/harness-core/benches/pipeline.rs`. `Cargo.toml` registers it as `[[bench]] name = "pipeline"` with `harness = false`, so the file brings its own `criterion_main`. The workflow runs this bench as section `28`, label `chapter 28 pipeline`, package `ch28-pipeline`.

`sample` builds the context both benches share. It is called before `b.iter`, so the 100 messages are not part of the timed closure.

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

Index 0's text is `message 0 about the rename procedure`. Index 99 is the same shape. One hundred owned strings, one `Arc`.

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

`bench_function` names the sample `arc_pass`. `b.iter` is the timed loop. Inside, `Arc::clone` bumps the strong count, `black_box` keeps the compiler from deleting the clone, `hand_off` moves that `Arc`, and `Arc::ptr_eq` compares it with the original. `black_box` comes from `std::hint::black_box`. It is an identity function the optimizer has to treat as unknown. The result of `ptr_eq` is true for this sample. The bench does not assert that. It passes the bool through `black_box` so the compare stays in the sample.

No `Stage` is built. No `format!` runs. No note is pushed. The mean for this function is `9.15 ns`. That number is `mean.point_estimate` from the Benchmarks workflow. It is the mean. The `time:` line Criterion prints on the runner is the slope, and it is not the number in the table.

`copy_transcript` times the clone the pipeline does not do.

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

`&context.messages` is `&Vec<Message>`. `clone` on that vec clones every `Message`, and each `Message` clones two `String`s. `black_box` on the borrow and on the result keeps the clone alive for the measurement. The notes mutex is not cloned. `Message`'s `Clone` derive is why this line compiles. `Context` has no such derive, which is why the pipeline cannot "just clone the context" and why the bench has to name the vec.

The mean for this function is `6.12 µs`. Same workflow, same rule: `mean.point_estimate`, and the printed `time:` line is the slope.

```rust
criterion_group!(benches, arc_pass, copy_transcript);
criterion_main!(benches);
```

The group is both functions. A change to `hand_off` shows up in `arc_pass`. A change that made `hand_off` clone `messages` would be a different function from the one in `pipeline.rs`. I would see it because `copy_transcript` would no longer be the only clone in the chapter.

The 100 messages exist so the copy is a transcript-sized vec, not an empty one. The pipeline's demo uses one message. The test `hand_off_is_the_same_pointer` uses 100 and checks the length, so the bench size and the test size match.

The bench file's imports are the whole dependency list for those two functions.

```rust
use std::hint::black_box;
use std::sync::{Arc, Mutex};

use criterion::{criterion_group, criterion_main, Criterion};
use harness_core::pipeline::{hand_off, Context, Message};
```

`Criterion` is the argument `arc_pass` and `copy_transcript` take as `&mut Criterion`. `criterion_group!` and `criterion_main!` are the macros at the bottom of the file. `hand_off`, `Context`, and `Message` are the only items pulled from `harness_core::pipeline`. `run` is not imported here. The bench that times the pipeline's handoff does not call `run`. That split is deliberate. `arc_pass` measures the pointer. The four `format!` calls stay in the tests and in `demo`.

The test file imports the rest.

```rust
use std::sync::{Arc, Mutex};

use harness_core::pipeline::{hand_off, run, Context, Message, Stage, STAGE_NAMES};
```

`Stage` and `STAGE_NAMES` are for the four-stage test. `Mutex` is for the `context` helper. The integration tests sit in `tests/pipeline.rs`, so they are a separate crate that links `harness_core` the way the binary does. They can call `demo` through `harness_core::pipeline::demo` because `demo` is `pub`. They cannot call a private function. Nothing in `pipeline.rs` is private. `run`, `hand_off`, and `demo` are all `pub`.

One pass of `run` on the demo array is four iterations. I want the first one written out as values, because the later ones are the same statements with different fields.

The `Arc` passed in points at a `Context`. `messages` has length 1. `notes` is an empty vec behind a mutex. The stage borrow has `name` of `outline` and `reply` of `three steps`. `lock` returns the guard. `expect` unwraps it. `format!` allocates `outline: three steps`. `push` makes the notes length 1. The statement ends. The guard drops. The mutex is free. The `Arc` is still the parameter, waiting for the next iteration or for the return.

Iteration two pushes `draft: the draft`. Iteration three pushes `critique: shorter`. Iteration four pushes `revise: the revision`. Then `run` returns the same `Arc`. `demo` locks once more, clones the last string out of the guard, and drops the guard before the function returns the `String`. The clone is why the returned note does not borrow `out`. The caller in `main` only has the `String`. It does not keep the `Arc`. `demo`'s locals drop at the closing brace, the strong count hits zero, and the `Context` is freed after the last note has already been cloned into the return value.

That last clone is one `String`. It is not a clone of the message vec. The message `Write a note on rename.` dies with the `Context`. The process prints the note and exits. The next chapter's pool keeps its `Arc<Context>` inside the `Pool` so specialists can clone the pointer for the whole join. This chapter's demo does not need the context after the last note is in hand.

`copy_transcript` never builds those four notes. Its clone is the hundred `Message` values from `sample`. Each text contains `about the rename procedure`, which is longer than the test helper's `message {i}`. The bench and the test use different sentences on purpose. The test checks length and pointer equality. The bench gives the clone a transcript-sized body so the mean is the mean of that clone. I do not substitute one number for the other. `9.15 ns` is only `arc_pass`. `6.12 µs` is only `copy_transcript`.

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

`arc_pass` builds the 100-message context before `b.iter`. The timed closure clones the `Arc`, calls `hand_off`, and compares pointers. Both calls go through `black_box`. No stage reply is formatted.

`copy_transcript` uses the same context. The timed closure clones `messages`. `run` does not call this. `hand_off` does not call this.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `28`. The ids are `28/arc_pass` and `28/copy_transcript`. The old ids `26/arc_pass` and `26/copy_transcript` are aliases in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. The printed Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The last note is `revise: the revision`.

## Pitfalls

`run` locks `notes`. It does not append to `messages`. A stage that pushed a new `Message` would grow the transcript. This chapter grows the note log. The user's sentence stays the one string the demo stored.

The notes mutex is not held across stages. Each `push` is its own statement, and the guard drops at the semicolon. That is the sequential pipeline. Chapter 29 is the chapter that has more than one thread in `acquire` at once.

`hand_off` does not filter. A `scratch` role would still be in `messages` after the call. Chapter 30 is the filter. This chapter's messages in the tests are all role `user`.

A scripted reply does not read the previous note. The revise reply is the literal `the revision` even if the critique note had said something else. The shared `Arc` is the part that is real. The reading model is deferred with the reference models.

`copy_transcript` is in the bench file so the copy has a mean. Calling it from `run` would build the second transcript this chapter is here to avoid.

`Arc::ptr_eq` checks the `Context` allocation. A reallocation inside the notes `Vec` does not change that address. If you log pointer equality and also expect the note buffer's address to stay put, those are two different allocations.

`demo` asserts `ptr_eq` before it returns. A broken `run` panics the binary. The test that only compares the string would never run its `assert_eq!` if `demo` panicked first. Fix the pointer, then read the note.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). This chapter attaches context. It does not write a skill and it does not write a memory file.

**Context.** The next stage gets the same `Arc`. The messages stay in the vec that was built before `run`. The mean for handing that pointer across is `9.15 ns`. The mean for cloning the 100 messages is `6.12 µs`. The pipeline takes the first path.

**Order.** The stage slice is the order. `STAGE_NAMES` is outline, draft, critique, revise. `run` follows the slice it was given.

**Left for later.** Chapter 29 bounds how many specialists run at once and closes M5 and M6 for child tools. Chapter 30 filters a transcript before a handoff. Live MoA reference models stay out. `aggregate_moa_context` is the Hermes call this chapter did not port.

## Chapter summary

- **Stages.** Outline, draft, critique, revise, in slice order.
- **Context.** One `Arc<Context>`. `hand_off` returns it. `Arc::ptr_eq` holds. `messages` is not cloned between stages.
- **Notes.** Each stage pushes `name: reply` under the mutex. The demo's last line is `revise: the revision`.
- **Replies.** Scripted. No key. No model.
- **Time.** `arc_pass` mean `9.15 ns`. `copy_transcript` mean `6.12 µs`. Both are `mean.point_estimate`. The Criterion `time:` line is the slope.

[personal note: a pipeline stage that only made sense because it could see the previous note]

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

`todo!("chapter 28")` is the panic. The test wants `chapter_28_same_context(1, 1)` to be true. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 29 is the supervisor. Specialists are jobs on one pool. `WORKER_BOUND` is 8. A child at depth 1 cannot call the blocked tools. A reflection note is a string on a queue, and it does not start chapter 26's reviewer.
