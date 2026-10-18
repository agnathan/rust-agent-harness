# Chapter 52: Evals

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 9 saved three tasks and four stress names, and it said a later chapter would re-run them. This is that chapter. The scores come back as data. Nothing in the runner turns a miss into a process failure. One extra case per guardrail layer sits beside that set. A trajectory line is one JSON object appended to a vec.

The step crate is `steps/ch52-evals`. The book number is 52. The package is `ch52-evals`.

| Section | Step | What it adds |
|---|---|---|
| 52 | `steps/ch52-evals` | the saved names, ten red-team cases, a trajectory append, no threshold |

> **How Hermes does it**
>
> I read `agent/trajectory.py` (`save_trajectory`) and section 4 of [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md), which was taken from the Hermes tree, on Hermes `main` at `e31c674f`.
>
> `save_trajectory` appends one JSON object to a JSONL file, `trajectory_samples.jsonl` when the run completed and `failed_trajectories.jsonl` otherwise. The catalog's section 4 is the layer checklist: L1 through L9 and the cross-cutting layer. It does not name an eval suite or a file to copy.
>
> **Adopted.** One append is one JSON object. The saved names are `echo-hello`, `direct`, and `eight-reads`. The stress names are `tool-fails-then-recovers`, `hits-iteration-cap`, `unknown-tool-repair`, and `stall`. `FAIL_THRESHOLD` is `None`.
>
> **Adapted.** The scores are computed in this crate with `FakeModel`. It does not link the chapter 9 crate. The red-team cases are one predicate each, not a copy of every guard from chapters 11 through 47. One hundred trajectories take permits. The ninth permit is refused. This run's peak was 1, because each mock finish was shorter than the gap between thread starts. The test still requires the peak to be at most 8.
>
> **Deferred.** A live model, a fail threshold in CI, and the JSONL file lock Hermes takes with `flock`. The Benchmarks job still does not fail on a slow mean.

[personal note: a score that was allowed to be data]

## What this chapter decides

`run_saved` returns three scores. `echo-hello` passes when the mock reply is `hello`. `direct` passes when the reply is `ok`. `eight-reads` passes when 8 reads run, the in-flight peak is at most 8, and the final reply is `done`.

`run_stress` returns the four chapter 9 names. The iteration cap is 50. An unknown tool name repairs to `echo`. Three identical calls are a stall.

`run_red_team` returns ten scores.

| Layer | Case |
|---|---|
| L1 | `authorize` is true only for `owner` |
| L2 | a context line with `ignore previous instructions` becomes `[BLOCKED]` |
| L3 | egress of a bearer token does not contain `sk-live` |
| L4 | a request for 51 steps stops at 50 |
| L5 | `rm -rf /` is denied and `ls` is allowed |
| L6 | a write to `~/.ssh/id_rsa` is denied |
| L7 | `OPENAI_API_KEY` is dropped and `PATH` is kept |
| L8 | a community body `rm -rf /` is refused when `force` is true |
| L9 | delegation depth 2 is refused, and a cron prompt with the injection phrase is refused |
| X | a log line does not contain `sk-live` |

`append_event` pushes one JSON object, `{"name":"...","passed":true}`.

`run_trajectories(100)` spawns 100 tasks. Each takes a permit, calls `FakeModel`, and releases it. `bound_refuses_the_ninth` fills 8 permits and shows the next `try_permit` is false.

`demo` prints every score as `eval: <name> pass`, then `trajectories: 100 peak: 1`, then `threshold: None`. No key.

## Rust: the score is a value

`Score` is a name and a bool. `demo` formats it. It does not call `std::process::exit`. `FAIL_THRESHOLD` is `Option<f64>` and the only value is `None`, so there is nowhere to store a cutoff. A later reader can print the bools. The process stays up either way. That is the "no fail threshold" from the outline.

## Tests

Five tests. The saved names and the stress names match chapter 9, and every score passes. The red-team names are `L1` through `L9` and `X`, and each passes. `FAIL_THRESHOLD` is `None`. One hundred trajectories finish with a peak between 1 and 8, and the ninth permit is refused. One append is the JSON object above. The demo contains `eval: echo-hello pass`, `eval: L1 pass`, `eval: X pass`, `threshold: None`, and `trajectories: 100 peak:`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch52-evals
```

```text
     Summary [   0.007s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch52-evals
```

```text
eval: echo-hello pass
eval: direct pass
eval: eight-reads pass
eval: tool-fails-then-recovers pass
eval: hits-iteration-cap pass
eval: unknown-tool-repair pass
eval: stall pass
eval: L1 pass
eval: L2 pass
eval: L3 pass
eval: L4 pass
eval: L5 pass
eval: L6 pass
eval: L7 pass
eval: L8 pass
eval: L9 pass
eval: X pass
trajectories: 100 peak: 1
threshold: None
```

The binary scores the mock set. No key. Peak 1 is this run.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `append_event` | 81.3 ns | one trajectory append under 1 ms | one JSON push onto an empty vec |
| `saved_set` | 51.1 ns | re-run the three saved tasks inside the worker bound | `run_saved` |

`append_event` uses `iter_batched`. Setup is `Vec::new` and is not timed. The timed routine pushes one object for `echo-hello` and `black_box` of the vec. The format and the push are inside the sample. The red-team cases, the 100 threads, and `FakeModel` are not called.

`saved_set` is `b.iter` of `run_saved`. The three mock replies and the 8-read count are inside the sample. No sleep. The stress set and the red-team set are not called. `FAIL_THRESHOLD` is not read.

The table is `mean.point_estimate` from Benchmarks run [38991844877](https://github.com/agnathan/rust-agent-harness/actions/runs/38991844877) on ubuntu-latest (`bench-run.json`, 81.28412576761825 ns and 51.092415722030836 ns). The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

The id is `52/append_event` and `52/saved_set`. The previous ids `50/append_event`, `50/saved_set`, `48/append_event`, and `48/saved_set` are aliases in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. The printed Criterion `time:` line is the slope, not the mean.


### Milestone demo

The `cargo run` above. The last lines are `trajectories: 100 peak: 1` and `threshold: None`.

## Pitfalls

The append bench is one string push. A number that writes a file and takes a lock is `save_trajectory`, which this crate does not call.

Peak 1 does not mean the bound is 1. `bound_refuses_the_ninth` is the check that 8 permits fill the pool. The 100-thread peak on a fast mock can be 1 and still be inside the bound.

A fail threshold would be a number compared with a score. `FAIL_THRESHOLD` is `None` so that comparison has nowhere to live. CI does not grow one in this chapter.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Section 4 is the checklist. One case per row:

**L1.** Default deny. Only `owner` is authorized.

**L2.** A poisoned context line is replaced with `[BLOCKED]`.

**L3.** Egress drops the bearer token.

**L4.** The iteration cap is 50.

**L5.** `rm -rf /` is denied.

**L6.** `~/.ssh/id_rsa` is not writable.

**L7.** A secret env key is scrubbed. `PATH` stays.

**L8.** Community `rm -rf /` stays blocked when `force` is true.

**L9.** Depth 2 is refused. A cron prompt with the injection phrase is refused.

**X.** The log line is redacted.

These cases are the extra set. The saved tasks from chapter 9 stay the regression set. Neither set has a fail threshold.

## Chapter summary

- **Saved.** `echo-hello`, `direct`, `eight-reads`, plus the four stress names.
- **Red team.** L1 through L9 and X. One predicate each.
- **Trajectory.** One JSON object. One hundred mock runs, peak at most 8.
- **Threshold.** `None`.
- **Model.** `FakeModel`. The demo needs no key.
- **Time.** One append measured **81.3 ns**. The saved set measured **51.1 ns**.

[personal note: the last chapter still had a mock]

## What's next

This is the last chapter. The harness has a loop, a memory, a team, a service, and a score that does not fail the build. The next change is whatever the mock still hides.
