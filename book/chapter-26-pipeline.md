# Chapter 26: The Sequential Pipeline

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 25 maintains one agent's skills. It does not hand a piece of work from one agent to the next. This chapter runs four stages in order: outline, draft, critique, revise. Each stage is a scripted reply. The next stage receives the same `Arc<Context>`. The message vec is not cloned on the way.

The step crate is `steps/ch26-pipeline`. The book number is 26. The package is `ch26-pipeline`.

| Section | Step | What it adds |
|---|---|---|
| 26 | `steps/ch26-pipeline` | outline, draft, critique, revise, one shared `Arc` |

> **How Hermes does it**
>
> I read `agent/turn_request_assembly.py` (the function that runs mixture-of-agents reference models and appends their aggregated context to the last user message) on Hermes `main` at `8dcf6d5b`.
>
> Hermes does not have an outline, draft, critique, revise pipeline. The closest pass is that mixture-of-agents assembly: reference models run, then an aggregator adds their context onto the user message the main model sees.
>
> **Adopted.** One context object is extended. Later work reads what earlier work wrote. The aggregator idea is the critique stage's note sitting on the same context the revise stage reads.
>
> **Adapted.** Four named stages, each a value with a scripted reply. There is no reference-model fan-out in this chapter. `hand_off` returns the same `Arc`. A separate bench clones 100 messages so the cost of a copy is visible. The pipeline does not call it.
>
> **Deferred.** A live model per stage. Chapter 27 calls specialists as tools under a bound. Those specialists stay scripted too. The mixture-of-agents reference models are not ported.

[personal note: a draft you wish had gone through a critique stage before you saw it]

## What this chapter decides

`Context` holds the messages and a `Mutex` of notes. `Stage` is a name and a reply. `STAGE_NAMES` is `outline`, `draft`, `critique`, `revise`.

`run` walks the stages in the slice order. Each one pushes `name: reply` onto the notes. It returns the same `Arc`. `Arc::ptr_eq` is true for the input and the output. The message vec's length does not change.

`hand_off` is the pointer the next stage receives. It is the function the bench times. `run` is what the tests call.

`demo` builds one user message, runs the four scripted replies, and returns the last note, `revise: the revision`. No key.

## Rust: Arc::ptr_eq

`Arc::clone` copies the pointer and bumps a count. It does not copy `messages`. `Arc::ptr_eq` is true when both handles point at the same allocation. The test keeps the original `Arc`, passes a clone into `run`, and checks the returned handle.

`black_box` on the clone and on the `ptr_eq` result stops the bench from deleting the work. The 100 messages are built before the timer.

## Tests

Three tests. Four stages append four notes in order, the message vec stays length 3, and the two `Arc`s are the same pointer. `hand_off` of a 100-message context is that same pointer. The demo's last note is `revise: the revision`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch26-pipeline
```

```text
Starting 3 tests across 3 binaries
     Summary [   0.003s] 3 tests run: 3 passed, 0 skipped
```

```bash
cargo run -p ch26-pipeline
```

```text
revise: the revision
```

The binary runs four scripted stages. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `arc_pass` | 10.46 ns | the next stage receives `Arc` | clone the context pointer |
| `copy_transcript` | 6.568 µs | the copy the pipeline does not do | clone 100 messages |

`arc_pass` builds a context of 100 messages before the bench. The timed closure is `b.iter`. Inside: `hand_off` of an `Arc::clone` passed through `black_box`, then `Arc::ptr_eq` passed through `black_box`. No stage reply is formatted. No model is called.

`copy_transcript` uses the same 100 messages. The timed closure clones `messages`. The pipeline does not call this function. The number is what a hand-off would cost if it copied the transcript.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38986915984](https://github.com/agnathan/rust-agent-harness/actions/runs/38986915984), section `26`.

### Milestone demo

The `cargo run` above. The last note is `revise: the revision`.

## Pitfalls

The notes mutex is not the transcript. A stage that pushed into `messages` would grow the prompt. This chapter only appends notes.

`run` locks once per stage. Four stages are four lock acquisitions. That cost is not `arc_pass`. `arc_pass` is only the pointer.

A scripted reply does not read the previous note. The next chapter's specialists are still scripted. The shared `Arc` is the part that is real.

`copy_transcript` is in the bench file so the copy has a number. Calling it from `run` would miss the point of the chapter.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). This chapter passes context. It does not write a skill or a memory file.

**Context.** The next stage gets `Arc`. The 100 messages stay in one allocation. A copy of those messages measured 6.568 µs and is not on the path.

**Order.** The stage slice is the order. Critique cannot run before draft unless the caller rearranges the slice.

**Left for later.** Chapter 27 bounds how many specialists run at once and blocks a child from calling `delegate_task`. Chapter 28 filters a transcript before a handoff. E1 and E2 are still absent.

## Chapter summary

- **Stages.** Outline, draft, critique, revise, in that order.
- **Context.** One `Arc`. `ptr_eq` holds. Messages are not cloned between stages.
- **Replies.** Scripted. No key.
- **Time.** Handing off the `Arc` measured **10.46 ns**. Copying 100 messages measured **6.568 µs**.

[personal note: a pipeline stage that only made sense because it could see the previous note]

## What's next

Chapter 27 is the supervisor. Specialists are tools. Up to eight run at once, on the same `Arc`. A child cannot delegate again.
