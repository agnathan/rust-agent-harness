# Chapter 7: Dynamic Catalog

A search is already running when a reload lands. The call that cloned the old `Arc` keeps the old index, finishes on the tool it started with, and never sees the plugin that just arrived. The next call clones the new pointer and sees the new catalog. That swap is this chapter.

Chapter 5's gateway could only list the tools it was given at startup, so a plugin or a second server meant a restart. Chapter 6 measured a 10,000-tool search that was fast and still ran inside the agent. Moving the index here means the agent asks and the server answers, which is the split the client chapters have been aiming at. The step crate is `steps/ch07-catalog`, package `ch07-catalog`. It binds no socket. A live connection to an upstream MCP server waits until chapter 37.

`Gateway` holds a `Snapshot` behind `RwLock<Arc<Snapshot>>`. The snapshot is the merged list: native tools, a `PluginSet`, and an in-memory `Upstream`. `Origin` says which of those a card came from. `Bm25Index` ranks the way chapter 6 did, with the same k1 and b, and an exact name match ranks first. `Embedder` is the trait chapter 6 introduced, and `HashEmbedder` is the stand-in that does not call a network. `reload` builds a new index, then replaces the `Arc`. A caller that already cloned the old `Arc` keeps it. `call` clones the `Arc` before it reads the card, so a reload during the body does not change that call's tool. A duplicate name is `CatalogError`, and the previous snapshot stays.

The server stays a separate program because the index is the catalog's job. The harness, from chapter 6, decides what may enter the prompt and keeps that prefix append-only. The gateway decides what the catalog contains, and it can search that catalog without making every agent process carry 10,000 schemas. Hermes rebuilds a client-side catalog on every assembly, on purpose, so a session-keyed copy can't drift from the live definitions. This server is the source of the list, so it can keep a snapshot and publish a new one by swapping an `Arc`. Chapter 8 will key a session view on that snapshot, and the notification chapter 4 already honors is how the client hears about the swap. `mcp-gateway` re-exports a later copy of this gateway and still opens no port.

A plugin load is a new snapshot, and the in-flight call keeps the old one. When skills and learned tools exist, the same publish-and-keep rule is how an offer can grow without mutating a call that already started.

Cloning an `Arc` copies a pointer and bumps a count. `RwLock` lets readers hold the current pointer while one writer stores a new one. The embedder is a trait object, and BM25 is the ranking, explained next to the scorer.

`cargo run -p ch07-catalog` runs the demo. Nothing listens.

**In this chapter**

- What this chapter decides
- Reload
- Tests
- Build, run, and benchmark
- Pitfalls
- Guardrails in this chapter
- Chapter summary
- What's next

| Section | Step | What it adds |
|---|---|---|
| 7 | `steps/ch07-catalog` | BM25, exact-name rank, plugin and upstream merge, `Arc` reload |

> **How Hermes does it**
>
> I read `tools/tool_search_catalog.py` and `tools/tool_search.py` on current Hermes `main`. `search_catalog` ranks by BM25 and puts an exact name match first (`score` is infinity for that row). The module doc in `tool_search.py` says the catalog is stateless: it is rebuilt from the live definitions on every assembly, because a session-keyed catalog drifts. `CHARS_PER_TOKEN` is 4.0. `_MAX_QUERIES_PER_CALL` is 7 and `_MAX_DESCRIBE_NAMES_PER_CALL` is 10. Those two caps are chapter 8. `tools/registry.py` says a same-name, same-toolset `register()` overwrites.
>
> **Adopted.** BM25 with k1 = 1.2 and b = 0.75, the same constants as chapter 6. An exact name match is the first hit, with an infinite score. A duplicate name is an error, and the previous snapshot stays.
>
> **Adapted.** Hermes rebuilds on every assembly. This server keeps a `Snapshot` behind `RwLock<Arc<Snapshot>>`. `reload` builds the new index, then replaces the `Arc`. A caller that already cloned the old `Arc` keeps it. `call` clones the `Arc` before it reads the card, so a reload during the body does not change that call's tool. The rarest-token admission in `search_catalog` (`_gate_token`, and `_required_term_coverage`) is not in this index. Chapter 6's client search does not have it either. I am not assigning it to a later chapter. `HashEmbedder` buckets bytes into eight floats. Search calls it when one is installed, and ranking stays BM25. That is the same seam as chapter 6.
>
> **Deferred.** A real embedder, a real upstream MCP connection, and hot reload across replicas wait. Federation of live servers is chapter 37. Per-session views are chapter 8. The 7-query and 10-describe caps are chapter 8.

## What this chapter decides

`Load` has three lists, `native`, `plugins`, and `upstreams`, because a merged catalog has to remember where a name came from when two sources disagree. `Snapshot::build` walks them in that order. The first time a name appears, it is kept, and its `Origin` is `Native`, `Plugin`, or `Upstream`. The second time, `reload` returns `Duplicate` and the gateway's current `Arc` is unchanged, so a bad load can't replace a catalog the process is already serving.

`search` tokenizes on non-alphanumeric runs, drops tokens shorter than 2 characters, and lowercases the rest. There is no stopword list in this crate, because a short synthetic catalog would lose real query words to a list copied from English prose. An exact case-insensitive name is pulled out and placed first. The rest are BM25 hits with a score above zero, tie-broken by name.

`call` captures `snapshot()` and then reads that snapshot, so the body and the name lookup agree even if `reload` runs halfway through. `echo` still returns `text`. Every other name returns the description from the captured card.

Plugins and upstreams in this chapter are values in the `Load`. Nothing is spawned and nothing is dialed, because a real upstream connection is chapter 37 and I want the snapshot swap tested before any socket can fail it.

## Reload

The scene from the opening is three lines. `Snapshot::build` finishes before the write lock is taken, the lock is held only long enough to store the pointer, and a `call` that already holds an `Arc` never takes that lock again. A search in flight and a reload can overlap because they don't share the index, only the pointer that names it.

```rust
pub fn reload(&self, load: Load) -> Result<(), CatalogError> {
    let snapshot = Snapshot::build(load)?;
    *self.current.write().expect("catalog") = Arc::new(snapshot);
    Ok(())
}
```

The write in `reload` is that pointer store. The build has already finished, which is why a search holding the old `Arc` and a reload storing a new one don't wait on each other.

> **Rust sidebar: cloning an `Arc` is not cloning the index**
>
> `Arc::clone` increments a count and copies a pointer. The index stays where it is. Two calls can hold two different snapshots after a reload. The old index is freed when the last `Arc` drops. You do not lock the index for the whole search. You lock long enough to clone the pointer, then search the clone.

## Tests

`reload_swaps_the_arc_and_keeps_the_old_snapshot` holds the first `Arc`, reloads three tools from native, plugin, and upstream sources, and checks the held `Arc` still has `echo` while the gateway has three tools. `a_duplicate_name_does_not_replace_the_catalog` reloads a second `echo` and checks the length is still 1. `an_exact_name_ranks_first` searches `calendar_rename_file` and expects that name with an infinite score. `call_uses_the_snapshot_it_captured` checks `echo` returns `hello` and that installing `HashEmbedder` still returns the exact-name hit. `ten_thousand_tools_find_rename` builds the synthetic catalog and searches `rename a file`.

The synthetic generator uses eight server names, eight verbs, and eight nouns. Tool `i` is `{server}_{verb}_{noun}_{i}`. It is the same shape as chapter 6's generator, in this crate, so the server bench does not depend on the client crate.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch07-catalog
```

```text
Starting 5 tests across 3 binaries
     Summary [   0.140s] 5 tests run: 5 passed, 0 skipped
```

The 10,000-tool test is the slow one in that summary.

```bash
cargo run -p ch07-catalog
```

```text
tools=2
files_rename_file (files)
```

The binary loads one native tool and one upstream tool and searches `rename a file`. No key, no socket.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `catalog_search_10k` | 299.0 µs | search under 1 ms | top-8 BM25 over 10,000 tools |
| `catalog_reload_10k` | 27.42 ms | rebuild plus the `Arc` store | clone 10,000 cards, build the index, store it |
| `catalog_arc_swap` | 2.467 µs | the pointer swap on a 1-tool catalog | hold the old `Arc`, reload one card |

`catalog_search_10k` builds the gateway and the 10,000-card index outside the sample. The timed closure is `search("rename a file", 8)` and `black_box` of the hits. No embedder is installed. `catalog_reload_10k` clones the 10,000 cards and rebuilds the index inside the sample. That clone is part of the reload the caller pays. `catalog_arc_swap` times a reload of one card while an `Arc` from before the reload is still held. The search number is the server. Chapter 6's client search of 10,000 tools was 146.8 µs. This server search measured 299.0 µs on the Benchmarks runner. They are different crates and different machines.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38863938100](https://github.com/agnathan/rust-agent-harness/actions/runs/38863938100), section `7` (recorded as `gw7`). How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`reload` of a duplicate returns the error and leaves the old snapshot. Code that ignores the `Result` and then searches will see the old catalog and think the load landed.

The write lock is short, but a search that forgets to clone the `Arc` and instead holds the read guard for the whole scan will block reload. `search` clones, drops the guard, then scores.

`HashEmbedder` is not a ranking signal. A test that expects nearest-neighbor order from those eight floats is testing a model this chapter does not have.

Exact-name ranking uses the trimmed query as the whole name. A query of `rename a file` does not exact-match `files_rename_file`. It takes the BM25 path.

## Guardrails in this chapter

The catalog boundary is still not a security boundary. The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Duplicate names fail closed.** A second `echo` does not overwrite. The previous `Arc` stays. Hermes overwrites a same-toolset duplicate (`tools/registry.py`). This chapter refuses.

**Reload does not edit a snapshot a call already holds.** That is the consistency rule. It is not authentication.

Plugin tools are whatever the `Load` contains. Hermes scans plugins at install (`tools/plugin_guard.py`, H3: 400 files, 10 MB total, 1 MB per file). This chapter does not scan. No later chapter in the outline owns H3, so I am not pointing at one. Upstream tools are an in-memory list. The OSV check before an npx or uvx launch (H4, `tools/osv_check.py`) is noted on chapter 42. Live federation is chapter 37.

Context-file injection scanning (F2) and the 20,000-character cap (F3) are chapter 34. They are not applied to tool descriptions here.

## Chapter summary

- **Load.** Native, then plugins, then upstreams. A duplicate name aborts the reload.
- **Search.** BM25, exact name first. 10,000 tools, top 8, measured **299.0 µs**.
- **Reload.** The index is built, then the `Arc` is stored. Rebuilding 10,000 tools measured **27.42 ms**. Swapping a one-tool snapshot while holding the old `Arc` measured **2.467 µs**.
- **Embedder.** Optional. Ranking does not use it.

## What's next

Chapter 8 gives each session its own revealed set, `search_tools`, `set_task`, and a token count for three ways of showing tools.
