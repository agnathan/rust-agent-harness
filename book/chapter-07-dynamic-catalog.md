# Chapter 7: Dynamic Catalog

A search is already running when a reload lands. The call that cloned the old `Arc` keeps the old index, finishes on the tool it started with, and never sees the plugin that just arrived. The next call clones the new pointer and sees the new catalog. That swap is this chapter.

Chapter 5's gateway could only list the tools it was given at startup, so a plugin or a second server meant a restart. Chapter 6 measured a 10,000-tool search that was fast and still ran inside the agent. Moving the index here means the agent asks and the server answers, which is the split the client chapters have been aiming at. The step crate is `steps/ch07-catalog`, package `ch07-catalog`. It binds no socket. A live connection to an upstream MCP server waits until chapter 37.

`Gateway` holds a `Snapshot` behind `RwLock<Arc<Snapshot>>`. The snapshot is the merged list: native tools, a `PluginSet`, and an in-memory `Upstream`. `Origin` says which of those a card came from. `Bm25Index` ranks the way chapter 6 did, with the same k1 and b, and an exact name match ranks first. `Embedder` is the trait chapter 6 introduced, and `HashEmbedder` is the stand-in that does not call a network. `reload` builds a new index, then replaces the `Arc`. A caller that already cloned the old `Arc` keeps it. `call` clones the `Arc` before it reads the card, so a reload during the body does not change that call's tool. A duplicate name is `CatalogError`, and the previous snapshot stays.

The server stays a separate program because the index is the catalog's job. The harness, from chapter 6, decides what may enter the prompt and keeps that prefix append-only. The gateway decides what the catalog contains, and it can search that catalog without making every agent process carry 10,000 schemas. Hermes rebuilds a client-side catalog on every assembly, on purpose, so a session-keyed copy can't drift from the live definitions. This server is the source of the list, so it can keep a snapshot and publish a new one by swapping an `Arc`. Chapter 8 will key a session view on that snapshot, and the notification chapter 4 already honors is how the client hears about the swap. `mcp-gateway` re-exports a later copy of this gateway and still opens no port.

A plugin load is a new snapshot, and the in-flight call keeps the old one. When skills and learned tools exist, the same publish-and-keep rule is how an offer can grow without mutating a call that already started. That is the catalog half of a continuously improving harness. The harness, later, will write a skill and want a new tool offer. The offer has to appear for the next call. It must not rewrite the call that already cloned a snapshot and started work. Chapter 23 is where a learned offer sits on this idea. This chapter is the swap the offer will need.

Cloning an `Arc` copies a pointer and bumps a count. `RwLock` lets readers hold the current pointer while one writer stores a new one. The embedder is a trait object, and BM25 is the ranking, explained next to the scorer.

I expected the embedder to change the order of hits, because a type named `Embedder` sounds like a ranker. `Gateway::search` can call it, and the chapter's Hermes box says ranking stays BM25. Installing `HashEmbedder` still returns the exact-name hit. The eight floats are a seam, the same seam chapter 6 left, so a later model can be dropped in without this chapter pretending to have one. A test that wants nearest-neighbor order is testing a model the file lacks.

The demo is the small version of the same swap. It loads one native tool and one upstream tool, searches `rename a file`, and prints `tools=2` and `files_rename_file (files)`. No key. No socket. The 10,000-tool numbers are the bench, not this printout. Chapter 6's client search of that size was 146.8 µs. This server's search measured 299.0 µs on the Benchmarks runner. Different crates, different machines, and the chapter keeps both figures so neither one gets asked to stand for the other.

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

Walk a query through the index the way the tests do, because the constants are easy to nod at and then misuse. `search` splits on runs of characters that are not ASCII letters or digits, drops any token shorter than two characters, and lowercases the rest. The query `rename a file` loses `a`. What remains is `rename` and `file`. There is no stopword list, so a two-letter word that would have been thrown out of an English list is still a token. The chapter says why: a short synthetic catalog would lose real query words to a list copied from prose. I expected `a` to be dropped and `file` to be dropped with it, as if the tokenizer knew which words were boring. It only knows length.

An exact match is a different path. The trimmed query is compared to the whole name, case-insensitive. `calendar_rename_file` as the query is that name, so the hit's score is infinity and it is placed first. `rename a file` is not that name. It takes the BM25 path, which is what the demo's search is doing. k1 is 1.2 and b is 0.75, the same constants chapter 6 used. Hits need a score above zero. Ties break by name. The rarest-token admission Hermes uses in `search_catalog` is absent here, and it was absent in chapter 6's client search too. The box says that, and it leaves the gap unassigned. Leaving it unnamed would let a reader think chapter 37 or chapter 51 quietly added it. They leave it out.

`Load` is three lists because a merged catalog has to remember where a name came from. `Snapshot::build` walks native, then plugins, then upstreams. The first time a name appears, it is kept, and its `Origin` is `Native`, `Plugin`, or `Upstream`. The second time, `reload` returns `Duplicate` and the gateway's current `Arc` stays the one it already had. A bad load cannot replace a catalog the process is serving. I expected the second `echo` to win, the way a config file later in a list usually wins. The test `a_duplicate_name_does_not_replace_the_catalog` reloads a second `echo` and checks the length is still 1. The error is the result. The old snapshot is the state.

`call` clones the snapshot first, then reads the card from that clone. `echo` still returns `text`. Every other name returns the description on the captured card. A reload that lands while the body is running changes what the next call sees. The description this call already copied stays. That is the whole consistency rule, and it is a consistency rule rather than authentication. The guardrails section says so. Plugin tools are whatever the `Load` contains. This chapter leaves them unscanned. The box and the guardrails both refuse to point that scan at a chapter with no claim on it.

Plugins and upstreams here are values. Nothing is spawned and nothing is dialed. A real upstream connection is chapter 37, and the snapshot swap is tested before any socket can fail it. Federation will add live servers. It inherits the rule that a caller keeps the `Arc` it cloned. If you can say which snapshot a call used, you can say which catalog a lesson was learned against. That sentence is the reason this chapter sits on the way to a harness that keeps what it learned. The skill is written later. The catalog the skill calls has to be a value with an identity, not a list that mutates under the call.

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

The tests are the spec, read as a sequence rather than as names. `reload_swaps_the_arc_and_keeps_the_old_snapshot` holds the first `Arc`, reloads three tools from native, plugin, and upstream sources, and checks the held `Arc` still has `echo` while the gateway has three tools. That is the opening scene with assertions on it. `a_duplicate_name_does_not_replace_the_catalog` is the failed load: a second `echo`, an error, length still 1. `an_exact_name_ranks_first` searches `calendar_rename_file` and expects that name with an infinite score. `call_uses_the_snapshot_it_captured` checks that `echo` returns `hello` and that `HashEmbedder` leaves the exact-name hit in place. `ten_thousand_tools_find_rename` builds the synthetic catalog and searches `rename a file`. The generator uses eight server names, eight verbs, and eight nouns. Tool `i` is `{server}_{verb}_{noun}_{i}`, the same shape as chapter 6's generator, living in this crate so the server bench stands without the client crate.

The three bench rows are three different costs, and mixing them is the mistake. `catalog_search_10k` builds the gateway and the 10,000-card index outside the sample. The timed closure is `search("rename a file", 8)` and a `black_box` of the hits. No embedder is installed. 299.0 µs is that search. `catalog_reload_10k` clones the 10,000 cards and rebuilds the index inside the sample, because that clone is part of the reload the caller pays. 27.42 ms is the rebuild plus the store. `catalog_arc_swap` times a reload of one card while an `Arc` from before the reload is still held. 2.467 µs is the pointer swap, not a search and not a rebuild. Chapter 6's 146.8 µs stays the client figure. This chapter's 299.0 µs stays the server figure. The run id and the section letter `gw7` are in the table, and the appendix is how to read the line.

A continuously improving harness will want this swap once a skill can name a tool that was not in the catalog at the start of the process. The swap is how the next call sees the new card while the current call keeps the old one. Chapter 8 puts a session in front of the snapshot. Chapter 23 changes which names a session is offered. Neither of those is a reason to mutate an `Arc` a call already holds. If you can point at the snapshot, you can say which catalog a later lesson was learned against. That is the sentence this chapter contributes to the introduction's promise, and it is a smaller sentence than the slogan. The code is `reload`, the lock around one pointer store, and the `Arc` the caller cloned before the body ran.

## Chapter summary

- **Load.** Native, then plugins, then upstreams. A duplicate name aborts the reload.
- **Search.** BM25, exact name first. 10,000 tools, top 8, measured **299.0 µs**.
- **Reload.** The index is built, then the `Arc` is stored. Rebuilding 10,000 tools measured **27.42 ms**. Swapping a one-tool snapshot while holding the old `Arc` measured **2.467 µs**.
- **Embedder.** Optional. Ranking does not use it.

One more pass over the lock, because the bug is the long guard. `search` clones the `Arc`, drops the read guard, then scores. A search that holds the guard for the whole scan blocks `reload`, even though the index itself is immutable once built. The write lock is only the pointer store. `Snapshot::build` finishes before that lock is taken. Two costs stay visible if you keep them apart: building the index, which `catalog_reload_10k` measured at 27.42 ms for 10,000 cards, and swapping the pointer, which `catalog_arc_swap` measured at 2.467 µs. The search at 299.0 µs is a third cost, on an index that already exists. A reader who adds those three numbers together has described no operation this crate performs.

The duplicate path is the other bug that looks like success. `reload` returns `Err` and leaves the old snapshot. Code that ignores the `Result` and then searches will see the old catalog and can believe the load landed. The test checks the length. The pitfall is what happens when the caller doesn't. Length still 1 means the second `echo` was refused. It also means the process is still serving the first catalog, which is the outcome you want from a bad load, and the outcome you will miss if you only log the error and assume the swap happened.

`Origin` is the field that makes the three lists more than a concatenation. A card remembers whether it came from native, a plugin, or an upstream. When two sources offer the same name, the walk stops and the old `Arc` stays, so you can say which source the surviving card came from. The demo's line `files_rename_file (files)` is a name and a server, which is the search hit, not the origin enum. The tests that reload three sources are where origin is doing work. Chapter 37 will attach live servers to the upstream list. The enum is already the place those servers will be named.

## What's next

Chapter 8 gives each session its own revealed set, `search_tools`, `set_task`, and a token count for three ways of showing tools. The snapshot this chapter swaps is the catalog that session reads.
