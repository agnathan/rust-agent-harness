# Chapter 24: Using Skills

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 23 can remember which tools a task used. It cannot remember a procedure. This chapter loads a directory of `SKILL.md` files and renders them once into an `<available_skills>` block. The block is an `Arc`. A skill added after that render becomes a note. The `Arc` bytes stay. `skill_view` loads one whole file when something asks for it.

The step crate is `steps/ch24-skills`. The book number is 22. The package is `ch24-skills`.

| Section | Step | What it adds |
|---|---|---|
| 24 | `steps/ch24-skills` | `SKILL.md`, a cache-stable index, `skills_list` / `skill_view` |

> **How Hermes does it**
>
> I read `agent/prompt_builder.py` (`_render_skills_index`), `agent/skills_index_delta.py` (`stage_skills_index_note`, `_render`, `_NOTE_PREFIX`), `tools/skills_tool.py` (`skills_list`, `skill_view`), and `tools/skill_manager_tool.py` (`MAX_NAME_LENGTH`, `MAX_DESCRIPTION_LENGTH`, `MAX_SKILL_CONTENT_CHARS`) on Hermes `main` at `8dcf6d5b`.
>
> The index is a `## Skills` block, then `<available_skills>`, one category line, then `    - name: description`, sorted. The opening sentence says that if a skill is even partially relevant you must load it with `skill_view(name)`. The closing sentence says to proceed without a skill only when none are relevant. `skills_index_delta.py` says the index lives in the system prompt for the life of the conversation so the prefix cache stays warm. A later change is a note that starts `[System: The skills available in this conversation changed after its system prompt was built.` Added names use the head `Now available (load with skill_view(name) when relevant, like the skills listed in the system prompt):`. Removed names use `No longer available:`.
>
> `skills_list` takes an optional category. `skill_view` reads `SKILL.md`, or one file inside the skill directory. The limits are name 64 characters, description 1,024, body 100,000.
>
> **Adopted.** The index shape, those two sentences, the note prefix and the two heads, the three limits, and a view that returns the whole file.
>
> **Adapted.** `requires_mcp` is a flat frontmatter key. The design doc nests it under `metadata.harness`. A missing server marks the row `[needs: github]`. The note lists names, not the full description rows `_render` builds. The index always names `web_search or terminal` in the source sentence I copied; Hermes drops `web_search` when that tool is absent. I did not port that switch.
>
> **Deferred.** Plugin skills, the `preprocess` shell snippets (`skills.inline_shell` defaults to false), and rebuilding the index at compaction. Writes are chapter 25.

[personal note: a skill you loaded late, and the step it would have saved]

## What this chapter decides

A skill is `<category>/<name>/SKILL.md`. Frontmatter is `name`, `description`, and an optional `requires_mcp`. The directory name is the fallback when `name` is empty.

`load_tree` walks categories and skill folders in sorted order. A second copy of the same name is skipped. A name over 64 characters, a description over 1,024, or a file over 100,000 characters is an error.

`render_index` sorts by category, then name. The same input renders the same bytes. `SkillSession::open` stores that string in an `Arc`. `index` clones the pointer.

`skills_list` returns the metas, or only one category. `skill_view` returns the whole `SKILL.md`. A supporting file has to live under `references/`, `templates/`, `scripts/`, or `assets/`. `..` is refused.

`change_note` compares the names in the stored index with the names a fresh render would have. An add or a remove returns the Hermes note. The stored `Arc` is not replaced.

`ScriptedModel` replies `skill_view rename`. The demo writes one skill, asks the model, and prints the last line of the file. No key.

## Rust: the index is an Arc

`Arc::clone` copies the pointer and bumps a count. It does not copy the string. Two calls to `index` compare equal with `Arc::ptr_eq` only if you still hold both, and the bytes compare equal either way. The test keeps one `Arc`, writes another skill, and checks the string is the one from `open`.

The note is a separate `String`. It is what a later user message would carry. The system prompt the provider cached is the first render.

## Tests

Four tests. The index is sorted, byte-identical across two renders, and marks `github` until that server is in the connected list. `skills_list` can filter to `files`. `skill_view` returns the frontmatter and the body, reads `references/notes.md`, and refuses `../SKILL.md` and a file that is not in a supporting directory. Adding `sync` produces a note and leaves the session `Arc` unchanged. A name of 65 characters fails `load_tree`. The demo skill is `rename` and the body line is `Rename a file by its path.`

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch24-skills
```

```text
Starting 4 tests across 3 binaries
     Summary [   0.005s] 4 tests run: 4 passed, 0 skipped
```

```bash
cargo run -p ch24-skills
```

```text
skill: rename
body: Rename a file by its path.
```

The binary writes one skill under a temp directory, renders the index, and views the file the scripted model names. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `load_render_500` | 2.162 ms | under 10 ms, once per session | load 500 skills and render the index |

`load_render_500` writes 500 `SKILL.md` files before the bench, ten categories, names `skill000` through `skill499`. The timed closure is `b.iter`. Inside: `load_tree` and `render_index`, both inputs passed through `black_box`. Writing the files is excluded. No model is called.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38986915984](https://github.com/agnathan/rust-agent-harness/actions/runs/38986915984), section `22`.

### Milestone demo

The `cargo run` above. The skill is `rename`. The body line is `Rename a file by its path.`

## Pitfalls

A description change that does not add or remove a name does not produce a note. The stored index still has the old description until the next session. Hermes diffs the rows. This chapter diffs the names.

The second skill with the same name is skipped. The first one in sorted directory order is the one `skill_view` finds, because both walks sort the same way. Two folders that disagree are a drift you will not see in the index.

`requires_mcp: github` on a connected server disappears from the row. The rest of the line stays. Connecting a server does not by itself rebuild the `Arc`. That is the same rule as an added skill: the change is a note, or it waits for the next session.

The view has no offset. A 100,000-character file comes back whole. That is the limit, not an invitation.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). This chapter only reads.

**Path.** `skill_view` refuses `..` and a file outside the four supporting directories. That is the read side of the path check. It is not a write gate.

**Size.** Name 64, description 1,024, body 100,000. A file over the body limit does not enter the index.

**Index.** The system-prompt block is one `Arc`. A later skill is a note, so the cached prefix is not rewritten.

**Left for later.** The skills-guard scan (H1) and the write-approval gate (A9) are chapter 25. Secret redaction (E1, E2) is still absent. Context-file injection scanning (F2) is chapter 34, and it is not applied to skill bodies here.

## Chapter summary

- **Layout.** `<category>/<name>/SKILL.md`, with `name`, `description`, and optional `requires_mcp`.
- **Index.** Sorted, byte-stable, stored in an `Arc`. A missing server is `[needs: …]`.
- **Tools.** `skills_list` filters by category. `skill_view` returns the whole file.
- **Note.** An added or removed name is announced. The `Arc` stays.
- **Time.** Loading and rendering 500 skills measured **2.162 ms**.

[personal note: a procedure that belonged in a skill instead of in the prompt]

## What's next

Chapter 25 writes skills. Create, patch, and delete go through a scanner and, when the gate is on, an approval hold. A reviewer has to read a file before it can patch it.
