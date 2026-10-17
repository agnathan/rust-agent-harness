# Chapter 30: The Critic

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 29 can gather a summary. It does not decide whether that summary is finished. This chapter is the critic. It reads a reply, takes one permit from the chapter 27 pool, and looks at the shared context. `verdict: done` accepts. `blocked`, `wait`, and `continue` do not. A missing verdict, or a verdict this crate does not know, rejects. The critic does not write the answer into the context notes.

The step crate is `steps/ch26-critic`. The book number is 30. The package is `ch26-critic`. It depends on `ch23-supervisor`.

| Section | Step | What it adds |
|---|---|---|
| 30 | `steps/ch26-critic` | a verdict, one permit, accept only on done |

> **How Hermes does it**
>
> I read `hermes_cli/goals.py` (`_parse_judge_response`, `DEFAULT_MAX_TURNS`, `DEFAULT_JUDGE_TIMEOUT`, `_JUDGE_RESPONSE_SNIPPET_CHARS`, `JUDGE_SYSTEM_PROMPT`) on Hermes `main` at `8dcf6d5b`.
>
> The judge returns `done`, `blocked`, `continue`, or `wait`. `_parse_judge_response` says it fails open. An empty reply becomes `continue`. A reply that is not JSON becomes `continue`. An unknown verdict string becomes `continue`. `DEFAULT_MAX_TURNS` is 20. `DEFAULT_JUDGE_TIMEOUT` is 30 seconds. `_JUDGE_RESPONSE_SNIPPET_CHARS` is 4,000. The prompt says `DONE` requires the deliverable to exist, and an explanation of why the goal cannot be reached is `BLOCKED`, not `DONE`. Quality gates are shell commands that must pass before the judge may declare `DONE`.
>
> **Adopted.** The four verdict names, a 4,000-character snippet, and a judge that is not the worker. `done` is the only accept. The pool's context `Arc` is the one the critic reads.
>
> **Adapted.** A missing or unknown verdict is `Reject`. Hermes continues. This gate has to accept before the answer leaves, so a reply with no verdict does not pass. `DEFAULT_MAX_TURNS` and `DEFAULT_JUDGE_TIMEOUT` are named by `limits`. This crate does not loop and does not sleep for 30 seconds. One permit is taken from the supervisor pool of 8. `as_tool` and `final_gate` both call that path.
>
> **Deferred.** The continuation loop, the `wait` target (a session, a pid, or a number of seconds), and the quality-gate shell commands. Chapter 31 composes the patterns and is where a loop around this gate belongs.

[personal note: a result that looked finished until something had to say so]

## What this chapter decides

`parse_verdict` finds the word `verdict` and reads the token after the separators `:`, `"`, space, `=`, and `'`. `done`, `blocked`, `wait`, and `continue` map to those variants. Anything else, including no `verdict` at all, is `Reject`.

`as_tool` calls `judge`. `judge` acquires one permit, clones the pool's context `Arc`, locks the notes, and does not push. `accepted` is true only for `Done`. The snippet is the reply cut at 4,000 bytes on a char boundary.

`final_gate` uses the same `judge`. `Ok` is the snippet when the verdict is `Done`. Any other verdict is `Err`.

`limits` returns `(20, 30)`. Those are the Hermes turn cap and the judge timeout. Nothing in this crate counts turns or waits.

`demo` judges `verdict: done` and returns `accepted: done`. No key.

## Rust: one permit, then the lock

`pool.acquire` returns a `Permit`. It lives until `judge` returns, then `Drop` releases the slot. A second call sees `max_seen` still at 1, because the first permit was already gone.

The notes lock is a read. The guard is dropped with the function. The answer string is not inserted. The test records the length before and after.

## Tests

Four tests. The four known verdicts parse. An empty string, `looks fine`, and `verdict: maybe` are `Reject`. `limits` is `(20, 30)`. `as_tool` on `verdict: done` accepts, the context pointer matches, `max_seen` is 1, and the notes length is unchanged. `final_gate` on `blocked` is `Err(Blocked)` and `max_seen` stays 1. An empty gate is `Reject`. A 5,000-x reply plus the verdict prefix snippets to 4,000 characters and still accepts. The demo is `accepted: done`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch26-critic
```

```text
Starting 4 tests across 3 binaries
     Summary [   0.003s] 4 tests run: 4 passed, 0 skipped
```

```bash
cargo run -p ch26-critic
```

```text
accepted: done
```

The binary judges one scripted reply. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `judge_one_permit` | 175.3 ns | one permit from the shared bound | judge `verdict: done` |

`judge_one_permit` builds a pool with limit 8 before the bench. The timed closure is `b.iter`. Inside: `as_tool` with the pool and the reply `verdict: done\nThe note is short enough.` passed through `black_box`. That acquires one permit, locks the notes, parses the verdict, copies the short snippet, and drops the permit. No model is called. The 30 second timeout is not slept. The notes vec is not appended.

The table is `mean.point_estimate` from Criterion on this machine, with the Benchmarks flags `--warm-up-time 1 --measurement-time 2 --sample-size 10`. The workflow mean on ubuntu-latest replaces this number when that run is green.

### Milestone demo

The `cargo run` above. The line is `accepted: done`.

## Pitfalls

Hermes treats a broken judge reply as `continue`. This crate treats it as `Reject`. A caller that expected the Hermes fail-open will stop a turn that Hermes would have kept going. That is the point of a gate that has to accept.

`wait` parses and does not accept. It also does not wait. There is no session id and no sleep. The continuation loop is chapter 31.

`final_gate` and `as_tool` each take their own permit. Calling both on one reply takes two permits in series, not two at once. `max_seen` stays 1.

The snippet cut is 4,000 bytes of the raw reply, verdict line included. It is not a summary the critic wrote.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Gate.** `Done` is the only accept. `Blocked`, `Wait`, `Continue`, and `Reject` do not leave through `final_gate`. A missing verdict is `Reject`, not Hermes's fail-open `continue`.

**Permit.** `judge` takes one permit from the chapter 27 pool. The bound stays 8. The critic does not start an extra pool.

**Context.** The critic clones the pool's `Arc` and locks the notes. It does not push the answer. The work stays where chapter 29 left it.

**Left for later.** The continuation loop, the `wait` target, and the quality-gate shell commands are chapter 31. E1 and E2 are still absent. The snippet is not redacted.

## Chapter summary

- **Verdicts.** `done` accepts. `blocked`, `wait`, and `continue` do not. Missing is `Reject`.
- **Permit.** One, from the pool of 8. The context `Arc` is the pool's.
- **Work.** The critic does not write the notes.
- **Named, unused.** 20 turns and a 30 second timeout. This crate does not loop or sleep.
- **Time.** One judgment measured **175.3 ns**.

[personal note: a gate you wanted to fail closed]

## What's next

Chapter 31 composes these patterns: a supervisor that fans out, and a handoff only when ownership should move. The continuation loop around this critic belongs there.
