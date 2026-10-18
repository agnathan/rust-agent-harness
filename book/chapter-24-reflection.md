# Chapter 24: The Reflection Loop

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 23 can write a skill when something asks it to. It does not look back at a finished turn. This chapter counts user turns and tool calls. Every tenth user turn, and every tenth tool iteration, turn finalize builds a reviewer fork. The fork may call skill tools. It may call memory only when the memory nudge is what fired. Cancel sets a flag. The turn does not sit out a two-second wait.

The step crate is `steps/ch24-reflection`. The book number is 24. The package is `ch24-reflection`.

| Section | Step | What it adds |
|---|---|---|
| 24 | `steps/ch24-reflection` | nudge counters, a forked reviewer, cancel on return |

> **How Hermes does it**
>
> I read `agent/agent_init.py` (the assignments of `_memory_nudge_interval` and `_skill_nudge_interval`), `agent/background_review.py` (`spawn_background_review_thread`, `_BACKGROUND_REVIEW_CANCEL_TIMEOUT_SECONDS`, the fork setup that zeroes both nudge intervals, `_review_tool_whitelist`), and `hermes_cli/config_defaults.py` (`memory.nudge_interval`) on Hermes `main` at `8dcf6d5b`.
>
> Both intervals start at 10. A missing config key stays at 10. `background_review.py` sets the fork's `_memory_nudge_interval` and `_skill_nudge_interval` to 0 so the reviewer does not nudge itself. The whitelist is `memory` and `skills` when the memory nudge fired and memory is enabled, and `skills` alone otherwise. The comment on that branch says a skill-nudge review never gets the memory tool. `read_file` and `search_files` are added. Write tools and `terminal` stay off. Cancel waits up to `_BACKGROUND_REVIEW_CANCEL_TIMEOUT_SECONDS` (2.0) for the thread to notice.
>
> **Adopted.** The interval of 10, a fork whose intervals are 0, memory only on a memory nudge, and a whitelist that includes skill tools plus `read_file` and `search_files`. `terminal` is refused.
>
> **Adapted.** Turn finalize builds the fork record. It does not start the review thread. The thread in the cancel bench is a spinner that sleeps 1 ms and checks a flag. This crate does not wait 2.0 seconds. `HERMES_CANCEL_TIMEOUT` names that constant and is not the join timeout. Memory here is a `Vec` of note lines, not `MEMORY.md`.
>
> **Deferred.** The forked agent, the shared session id, and prompt-cache parity of the advertised tool list. Chapter 27 runs the reflection note from a team run off the hot path. The curator that consumes a review is chapter 25.

[personal note: a turn you wished the agent had reviewed after you left]

## What this chapter decides

`Loop::note_user_turn` increments a counter. On multiples of 10 it marks a memory nudge due. `note_tool_iter` does the same for a skill nudge. `finalize` consumes those flags. No flag means `None`. A flag means `spawn_reviewer`.

`spawn_reviewer(true)` copies the memory tool list: `memory`, `skill_view`, `skill_manage`, `read_file`, `search_files`. `spawn_reviewer(false)` omits `memory`. Both intervals on the fork are 0.

`execute` reads the scripted model one line at a time. The first word is the tool. A tool outside the list is `NotAllowed`, and later lines are not run. `memory` pushes the rest of the line onto the note vec. `skill_view` calls chapter 22's loader when a root is passed. Cancel is checked before each line.

`ReviewRun::spawn_spinner` starts a thread that sleeps 1 ms until the flag is set. `cancel_and_join` sets the flag and joins. That join is the "user returned" path.

`ScriptedModel` holds the lines. The demo takes 10 user turns, finalizes, and runs `memory prefer short notes`. No key.

## Rust: a flag the other thread can see

`AtomicBool` with `Ordering::Release` on the store and `Ordering::Acquire` on the load is the cancel signal. The reviewer thread does not share a `Mutex` with the turn. The turn does not need the reviewer's result to finish.

`iter_custom` in the spawn bench starts the timer, builds `iters` forks, stops the timer, then drops the vec. The drop is outside the sample. `iter_batched` builds the spinner in the setup closure, which Criterion does not time, and times `cancel_and_join`.

## Tests

Six tests. Nine user turns do not fork. The tenth does, the tools include `memory` and `skill_view`, both intervals are 0, and the next finalize is empty. Ten tool iterations fork without `memory`. `terminal ls` is `NotAllowed` and writes no note. `memory prefer short notes` lands. Cancel before `execute` returns `Cancelled` and writes nothing. A spinning review joins in under 50 ms. The demo note is `prefer short notes`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch24-reflection
```

```text
Starting 6 tests across 3 binaries
     Summary [   0.006s] 6 tests run: 6 passed, 0 skipped
```

```bash
cargo run -p ch24-reflection
```

```text
note: prefer short notes
forked: after 10 user turns
```

The binary counts ten user turns, forks, and runs the scripted memory line. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `spawn_reviewer` | 60.16 ns | under 1 ms added to turn finalize | build one reviewer fork |
| `cancel_review` | 819.8 µs | under 50 ms | set the cancel flag and join the spinner |

`spawn_reviewer` times `spawn_reviewer(true)` inside `iter_custom`. The `Vec` of forks is dropped after the timer stops. No thread is started. No model is called. The memory nudge flag is the `true` argument, passed through `black_box`.

`cancel_review` uses `iter_batched`. The setup closure calls `ReviewRun::spawn_spinner`, which starts the thread. That setup is not timed. The timed closure is `cancel_and_join`: one atomic store and a join. The worker sleeps 1 ms between flag checks, so the join waits out at most one sleep.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38986915984](https://github.com/agnathan/rust-agent-harness/actions/runs/38986915984), section `24`.

### Milestone demo

The `cargo run` above. The note is `prefer short notes`. The fork happens after 10 user turns.

## Pitfalls

A skill nudge and a memory nudge on the same turn produce one fork, and that fork has memory. The skill-only list is used when the memory flag is clear.

`finalize` clears the flags. Forgetting to call it leaves them set, and the next finalize still forks. Calling it twice does not fork twice.

The 2.0 second Hermes wait is not a budget. A reviewer that ignores the flag would hold `cancel_and_join` until the process ends. This spinner checks the flag.

`skill_manage` is on the list and returns the line. It does not write a file. A real patch still goes through chapter 23, including the read-before-write rule.

The note vec is not `MEMORY.md`. Chapter 19's scan does not run on it.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Tool list.** The fork cannot call `terminal`. A skill-nudge fork cannot call `memory`, so an unattended review cannot delete a memory entry. That is the same split as `_review_tool_whitelist`.

**Nudge the fork.** Intervals on the fork are 0. The reviewer does not schedule another reviewer.

**Cancel.** The turn sets an `AtomicBool`. It does not wait `_BACKGROUND_REVIEW_CANCEL_TIMEOUT_SECONDS`.

**Left for later.** The fork does not share the parent's session row, so it cannot write into the live transcript. Chapter 25 is the pass that archives skills. Chapter 27 enqueues a reflection note after a team run, off the request path. E1 and E2 are still absent.

## Chapter summary

- **Counters.** Memory every 10 user turns. Skills every 10 tool iterations.
- **Fork.** Intervals are 0. Memory is on the list only for a memory nudge.
- **Refuse.** `terminal` is not a review tool.
- **Cancel.** A flag, then a join. Measured **819.8 µs**.
- **Finalize.** Building the fork measured **60.16 ns**.

[personal note: a review that was worth running after the turn, not during it]

## What's next

Chapter 25 is the curator. It marks a skill stale, then archives it, and it does that work off the request path. A write still goes through chapter 23's gate and scan.
