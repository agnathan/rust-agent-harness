# Chapter 46: Shared Team Skills

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 45 moves an envelope. It does not change what the team knows how to do. A shared skill library has one writer, or two workers will each think they installed the file. This chapter is that library: a proposal queue, a librarian, and an `Arc` the workers already hold.

The step crate is `steps/ch46-team-skills`. The book number is 44. The package is `ch46-team-skills`.

| Section | Step | What it adds |
|---|---|---|
| 46 | `steps/ch46-team-skills` | `skill_propose`, one librarian, an `Arc` swap, `TEAM.md` |

> **How Hermes does it**
>
> I read `tools/skills_sync.py` (the module docstring), `tools/skills_guard.py` (`should_allow_install`), `tools/write_approval.py` (`evaluate_gate`), and `optional-skills/creative/kanban-video-orchestrator/scripts/bootstrap_pipeline.py` (the `TEAM.md` note) on Hermes `main` at `e31c674f`.
>
> `skills_sync.py` copies bundled skills into `~/.hermes/skills/` and updates a user copy only when that copy still matches the origin hash. `should_allow_install` lets `force` override a block except a dangerous verdict on a community or trusted source. The refusal text says `--force` does not override a dangerous verdict. `evaluate_gate` stages a skill write when the approval gate is on. It does not commit it inline. The video bootstrap docstring names `TEAM.md` as the task-graph convention for that pipeline.
>
> **Adopted.** A community body that contains `rm -rf /` is refused, including when `force` is true. Workers do not write the index. `TEAM.md` is rendered from the skill list. A swap publishes `skills.updated`.
>
> **Adapted.** The queue is a `VecDeque` of 64, not the bus from chapter 45. The index is an `Arc` behind a `Mutex`. The swap is `mem::replace` of that `Arc`. The librarian calls `FakeModel` before it takes the swap lock. Builtin trust still allows a dangerous body, which is the Hermes builtin row.
>
> **Deferred.** The full Skills Guard regex set, the manifest hash, and the background-review fork. Chapter 25 already has the single-skill gate. This chapter is the team copy.

[personal note: an Arc the worker got to keep]

## What this chapter decides

`skill_propose` pushes onto a `VecDeque`. The 65th returns `Full`.

`librarian_accept` is the only writer. It rejects an empty body. It rejects a dangerous community proposal even when `force` is set. It calls `FakeModel` first. Then it clones the current skill list, appends a `Team` skill, builds a new `Index`, and `swap_index` replaces the `Arc`. The version increments. A `skills.updated` notice is pushed after the swap.

`load` clones the `Arc` and drops the mutex. A worker that loaded version 1 still sees version 1 after the librarian publishes version 2.

`render_team` writes `# TEAM` and one line per skill, `bundled` or `team`.

`demo` proposes `cite`, accepts it, and prints `skills: v2 3`. The 3 is the line count of `TEAM.md`. No key.

## Rust: the swap is the pointer

`Library` holds `Mutex<Arc<Index>>`. `swap_index` locks, `mem::replace`s the `Arc`, and unlocks. The new index was built before that lock. A worker's `Arc` is a refcount, not a borrow of the librarian's scan. The worker does not wait on `FakeModel`. The model ran before `swap_index`.

## Tests

Five tests. The queue holds 64 and the 65th is `Full`. After accept, the worker's old `Arc` is still version 1, the library is version 2, `TEAM.md` names `cite (team)`, and the notice is `skills.updated`. `force: true` on `rm -rf /` from a community proposal returns `Guard` and does not bump the version. A builtin dangerous body is allowed through `allow_install`. A worker can `load` and a later `swap_index` leaves that `Arc` at version 1. The demo is `skills: v2 3`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch46-team-skills
```

```text
     Summary [   0.006s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch46-team-skills
```

```text
skills: v2 3
```

The binary proposes one skill and the librarian accepts it. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `skill_propose` | 73.7 ns | enqueue a proposal under 1 ms | one push on an empty deque |
| `index_swap` | 199.9 ns | an `Arc` swap, off the worker path | `mem::replace` of the index pointer |

`skill_propose` builds the `Proposal` before the bench. `iter_batched` setup is `VecDeque::new` and is not timed. The timed routine calls `skill_propose` and `black_box` of the result. The push is inside the sample. The librarian, the guard, and `FakeModel` are not called. The `Full` path is not taken.

`index_swap` setup builds a `Library` at version 1 and a new `Arc<Index>` at version 2. That is not timed. The timed routine calls `swap_index` and `black_box` of the previous `Arc`. The lock and the pointer replace are inside the sample. Cloning the skill list and rendering `TEAM.md` are not.

The table is `mean.point_estimate` from Benchmarks run [38991844877](https://github.com/agnathan/rust-agent-harness/actions/runs/38991844877) on ubuntu-latest (`bench-run.json`, 73.69873963432929 ns and 199.94539724084487 ns). The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `skills: v2 3`.

## Pitfalls

The enqueue bench is one push. A number that includes `librarian_accept` is the scan and the model, which this budget keeps off the worker path.

`force` on a community dangerous body does not write. A test that only checks builtin trust would miss that.

The worker must `load` before the swap if the test wants to show the old version. Loading after the swap sees the new one, which is also correct and is not that assertion.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Skills Guard (H1).** `should_allow_install` blocks a dangerous community verdict, and `force` does not override it. This crate treats `rm -rf /` and `ignore previous instructions` as that verdict. The rest of the scanner is not copied.

**Write gate (A9).** Workers enqueue. They do not call `swap_index`. `evaluate_gate` stages a skill write when the gate is on. Here the librarian is the stage: nothing reaches the `Arc` until `librarian_accept` returns.

**Single writer.** `librarian_accept` is the function that bumps `version`. A worker's `Arc` does not see a half-updated list.

**Left for later.** The manifest hash, the 400-file plugin caps, and H5's read-before-write on a patch. Chapter 25 still owns a single skill file.

## Chapter summary

- **Queue.** `skill_propose` holds 64.
- **Librarian.** The only writer. The model runs before the swap.
- **Index.** An `Arc` replace. Old readers keep the old version.
- **Team.** `TEAM.md` lists bundled and team skills. `skills.updated` follows the swap.
- **Model.** `FakeModel` answers `ok`. The demo needs no key.
- **Time.** One enqueue measured **73.7 ns**. One swap measured **199.9 ns**.

[personal note: a version number the old Arc did not see]

## What's next

Chapter 47 is A2A, and only for an agent that lives somewhere else. Two agents in this process still use the patterns from part III.
