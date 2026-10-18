# Writing a chapter

Standing rule, from Daniel. Follow it before writing or revising any chapter.

## Research Hermes first

Before the Rust, read how [Hermes Agent](https://github.com/NousResearch/hermes-agent) implements the same area. Look at the current files on `main`, not a memory of an older tree. The layout moves: a behavior that used to live in `agent/conversation_loop.py` may now sit in `agent/turn_iteration_prep.py` or `tools/tool_output_truncate.py`.

From that reading, name the best and most important ideas. Implement the worthwhile ones in this chapter's Rust. Leave a short note for each idea you do not take.

## The "How Hermes does it" box

Every chapter has the box. Cite the specific Hermes files. For each idea say one of:

- **Adopted.** The Rust does the same thing, and why it belongs in this chapter.
- **Adapted.** The Rust keeps the idea and changes the shape, and why.
- **Deferred.** The idea is real and named, and which later chapter will take it. Add that sentence to the later chapter's outline row in `book/book-outline.md` in the same change.

Do not invent a file path, a function name, or a threshold. If the source was not opened, the box does not mention it.

The MCP gateway is a separate track from the harness. Its architecture, the harness/server split, and the notes from Hermes and the public gateways are in [mcp-gateway.md](mcp-gateway.md). Server chapters 5, 7, 8, 15, 21, 35, 38, and 47 are written. They bind no socket. From chapter 10 on, each harness step is a copy of the previous harness plus the new modules. The layout, the two lineages, and the per-chapter plan for chapters 11–48 are in [harness-architecture.md](harness-architecture.md).

## What else a chapter contains

Use [the chapter template](chapter-template.md). In short: a decision section, a walkthrough of the key function, a Rust sidebar at the moment the feature needs the language, the tests in words, a Build/run/benchmark section with a real command and the real output, a milestone demo that defaults to the mock, a summary, and social posts with a `[personal note]` placeholder. Criterion tables use the mean from `estimates.json` (`mean.point_estimate`). The printed `time:` line is the slope. No live OpenRouter transcript unless the command was actually run, and CI stays keyless.

## Step crates

Each section is its own directory under `steps/`, a copy of the previous section plus the new behavior. Register it in the workspace `Cargo.toml`, `.github/chapters.json`, `.github/path-filters.yml`, and `.github/workflows/benchmarks.yml`. Tests run on Ubuntu, macOS, and Windows. The bench for that section runs on Ubuntu.
