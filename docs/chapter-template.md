# Chapter template

Copy this shape. The Hermes box is filled only after the files have been read. See [Writing a chapter](writing-a-chapter.md).

```markdown
# Chapter N: Title

Two or three paragraphs: what the previous chapter cannot do, what this one adds, what it leaves for later.

| Section | Step | What it adds |
|---|---|---|
| N.1 | `steps/chNN-1-slug` | … |

> **How Hermes does it**
>
> Cite `path/in/hermes-agent`. **Adopted:** … **Adapted:** … **Deferred:** … (and the chapter that will take it).

> **Guardrails.** Layer from [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md): which catalog ids this chapter guards, and which it leaves for a later chapter.

## What this chapter decides

## N.1 Section title

Walk the key function. A Rust sidebar sits where the type or the trait is first needed.

### The tests

### Build, run, and benchmark

A real command, the real nextest summary, the real demo output, the Criterion slope, and a table whose Mean column is `estimates.json`.

### Milestone demo

`--mock` or the example. The optional `--live` line, and the missing-key output if that command was run without a key. Do not invent a model transcript.

## Pitfalls

## Chapter summary

## What's next
```

Social drafts live in `book/social/chapter-NN.md`.
