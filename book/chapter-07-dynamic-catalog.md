# Chapter 7: Dynamic Catalog

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 5 can list the tools it was given at startup. A gateway that only does that cannot pick up a plugin, or a second server's tools, without restarting the process. Chapter 6 already measured what a 10,000-tool list costs when the client holds it. This chapter moves the catalog into the server: native tools, plugin tools, and an in-memory upstream list, one BM25 index, and a reload that swaps an `Arc`.

The step crate is `steps/ch07-catalog`. The package is `ch07-catalog`. It still binds no socket. A live connection to an upstream MCP server is chapter 37.

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

[personal note: a reload that surprised a call still in flight]

## What this chapter decides

`Load` has three lists: `native`, `plugins`, and `upstreams`. `Snapshot::build` walks them in that order. The first time a name appears, it is kept, and its `Origin` is `Native`, `Plugin`, or `Upstream`. The second time, `reload` returns `Duplicate` and the gateway's current `Arc` is unchanged.

`search` tokenizes on non-alphanumeric runs, drops tokens shorter than 2 characters, and lowercases the rest. There is no stopword list in this crate. An exact case-insensitive name is pulled out and placed first. The rest are BM25 hits with a score above zero, tie-broken by name.

`call` captures `snapshot()` and then reads that snapshot. `echo` still returns `text`. Every other name returns the description from the captured card.

Plugins and upstreams in this chapter are values in the `Load`. Nothing is spawned and nothing is dialed.

## Reload

```rust
pub fn reload(&self, load: Load) -> Result<(), CatalogError> {
    let snapshot = Snapshot::build(load)?;
    *self.current.write().expect("catalog") = Arc::new(snapshot);
    Ok(())
}
```

`Snapshot::build` runs before the write lock is taken. The lock is held only for the pointer store. A `call` that already holds an `Arc` does not take that lock again.

### Rust: cloning an `Arc` is not cloning the index

`Arc::clone` increments a count and copies a pointer. The index stays where it is. Two calls can hold two different snapshots after a reload. The old index is freed when the last `Arc` drops. You do not lock the index for the whole search. You lock long enough to clone the pointer, then search the clone.

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

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38863938100](https://github.com/agnathan/rust-agent-harness/actions/runs/38863938100), section `7` (recorded as `gw7`).

### Milestone demo

The `cargo run` above is the demo.

## Pitfalls

`reload` of a duplicate returns the error and leaves the old snapshot. Code that ignores the `Result` and then searches will see the old catalog and think the load landed.

The write lock is short, but a search that forgets to clone the `Arc` and instead holds the read guard for the whole scan will block reload. `search` clones, drops the guard, then scores.

`HashEmbedder` is not a ranking signal. A test that expects nearest-neighbor order from those eight floats is testing a model this chapter does not have.

Exact-name ranking uses the trimmed query as the whole name. A query of `rename a file` does not exact-match `files_rename_file`. It takes the BM25 path.

## Guardrails in this chapter

The catalog boundary is still not a security boundary. [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Duplicate names fail closed.** A second `echo` does not overwrite. The previous `Arc` stays. Hermes overwrites a same-toolset duplicate (`tools/registry.py`). This chapter refuses.

**Reload does not edit a snapshot a call already holds.** That is the consistency rule. It is not authentication.

Plugin tools are whatever the `Load` contains. Hermes scans plugins at install (`tools/plugin_guard.py`, H3: 400 files, 10 MB total, 1 MB per file). This chapter does not scan. No later chapter in the outline owns H3, so I am not pointing at one. Upstream tools are an in-memory list. The OSV check before an npx or uvx launch (H4, `tools/osv_check.py`) is noted on chapter 42. Live federation is chapter 37.

Context-file injection scanning (F2) and the 20,000-character cap (F3) are chapter 34. They are not applied to tool descriptions here.

## Chapter summary

- **Load.** Native, then plugins, then upstreams. A duplicate name aborts the reload.
- **Search.** BM25, exact name first. 10,000 tools, top 8, measured **299.0 µs**.
- **Reload.** The index is built, then the `Arc` is stored. Rebuilding 10,000 tools measured **27.42 ms**. Swapping a one-tool snapshot while holding the old `Arc` measured **2.467 µs**.
- **Embedder.** Optional. Ranking does not use it.

[personal note: whether the reload or the index was the part you had to reread]

## What's next

Chapter 8 gives each session its own revealed set, `search_tools`, `set_task`, and a token count for three ways of showing tools.
