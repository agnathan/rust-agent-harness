# How to read this book

Each chapter adds one piece of the harness and shows the tests for that piece. Read in order if you are learning Rust. The sidebars are optional if you already know the language.

## The two trees

`steps/chNN-…` is the finished reference for that chapter. `my-harness/` is the crate you fill in. The milestone in `my-harness/tests/chNN.rs` compiles, then panics on `todo!` until your function makes the assertion pass. The step crate is the reference. Do not copy it into `my-harness/`.

The MCP gateway is a separate program, `mcp-gateway/`, not a module of the harness. Chapters 5, 7, 8, 15, 23, 37, 42, and 51 are that server. The harness calls it. It does not grow inside `my-harness/`.

## The boxes

**How Hermes does it** names files in [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) and says what this chapter adopted, what it changed, and what it left for later. The commit named in the box is the tree those paths were read against. A later chapter may cite a later commit of the same repository.

**Guardrails** names the catalog ids from [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md) that this chapter's code calls, and the ids it leaves for a later chapter. The box is the catalog, not a second implementation.

## Rust sidebars

A sidebar is a blockquote labeled **Rust sidebar**. It explains the language feature the code next to it just used. `Option`, `Result`, enums, and `Arc` are taught in [chapter 1](chapter-01-first-contact.md). Matching on an enum is in [chapter 2](chapter-02-conversations.md). Later chapters point back there when the same feature shows up again, and they keep a sidebar only for what is new in that chapter.

## The numbers

The commands and the measured means stay in the chapter. What Criterion's `time:` line means, which machine produced the early tables, and how a later mean was taken from a Benchmarks run is [How the numbers were measured](appendix-benchmark-method.md).
