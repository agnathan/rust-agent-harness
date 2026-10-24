# Chapter 6: Thousands of Tools

Chapter 3 can hold a few dozen tools, and the model was shown every schema that passed its gate. That stops working when the catalog is the size of a real MCP workspace: several servers, each with its own list, easily a thousand tools and sometimes ten thousand. Three costs arrive together. The prompt grows by the schema text, so the provider bills more input tokens. The model has more near-matches to choose from, so it calls the wrong tool more often. And every turn pays that cost even when the user asked for one rename.

This chapter is the client half of the catalog. It keeps the chapter 3 shape — a name, a description, a JSON Schema — and it does not open an MCP connection. Chapter 4 is the client that does. Chapter 5 is the separate server that will hold the list. The benches here still need a catalog of 1,000 and 10,000 tools, so a deterministic generator fills it. The five copies are `steps/ch06-1-schema-cost`, `ch06-2-tool-index`, `ch06-3-search-tool`, `ch06-4-routing`, and `ch06-5-lazy-list`. Section numbers stay 6.1 through 6.5 so they match those crates.

`SyntheticSource::list` is the stand-in the benches call. Section 6.1 counts schema bytes and reuses chapter 2's token estimate. Section 6.2 builds a BM25 index over names and descriptions, and an `Embedder` trait for a later vector. Section 6.3 adds `search_tools`: a hit is revealed on the next turn, the prompt prefix is append-only so a cached prefix stays valid, and a cap bounds how many schemas appear. Section 6.4 routes by server, then trims names and descriptions so the model is not told about a tool it cannot call. Section 6.5 caches `tools/list`, drops the cache on `list_changed`, filters to one agent's servers, and feeds a real chapter 4 list through `cards_from_listed`, `ListCache::from_cards`, and `replace_cards`. The chapter 3 dispatcher stays.

The server track exists so this work does not have to live inside the agent forever. Chapter 7 moves the BM25 index into the gateway, where a 10,000-tool search is not riding along in the harness process. This chapter keeps the part the harness must own: what is allowed into the prompt, in an order that does not rewrite the prefix. That rule is the same one chapter 2 locked for the system prompt. A self-improving agent that searches for a tool every turn, and rewrites the front of the prompt when it finds one, pays the cache again on every lesson. Revealing append-only, and evicting at a checkpoint, is how the harness stays cheap while the catalog grows. Learned prediction of which tools a task tends to use is chapter 23, on the server, and it changes the offered set rather than the middle of a request.

Rust, as it shows up: an inverted index is a map from token to postings. A trait hides the embedder so the tests use a hash. Append-only is a `Vec` you only push to. The cache is an ordinary struct you replace.

The runnable demo is offline. `cargo run -p ch06-5-harness-cli -- --mock ask "rename a file"` is the end state. A live OpenRouter call is optional on that CLI and was not run for this chapter. The missing-key error is a real run with the key unset.

**In this chapter**

- 6.1 Why many tools hurt
- 6.2 A searchable index
- 6.3 search_tools and lazy loading
- 6.4 Route, then shrink
- 6.5 The registry at scale

| Section | Step | What it adds |
|---|---|---|
| 6.1 | `steps/ch06-1-schema-cost` | schema bytes and the chapter 2 token estimate for 1,000 and 10,000 tools |
| 6.2 | `steps/ch06-2-tool-index` | a BM25 index over names and descriptions, and an `Embedder` trait |
| 6.3 | `steps/ch06-3-search-tool` | `search_tools`, reveal on the next turn, an append-only prefix, a cap |
| 6.4 | `steps/ch06-4-routing` | route by server, then trim names and descriptions |
| 6.5 | `steps/ch06-5-lazy-list` | a cached list, invalidation, a per-agent toolset, and `ask` |

> **How Hermes does it**
>
> Hermes keeps the full registry in `tools/registry.py` and then decides what the model is allowed to see. A toolset plus a `check_fn` drops tools that are not available. Descriptions are rewritten so they only mention tools that passed that check. This chapter's index takes the same gate: `build_available` never inserts a card whose check returns false. Section 6.4's `rewrite_description` drops a sentence that names `tool:` plus a name outside the available set. The model is not told about a tool it cannot call, and it is not told to go look at one either.

## 6.1 Why many tools hurt

The generator builds MCP-shaped cards. Eight server names, eight verbs, eight nouns, and an index, so tool 24 is always `files_rename_file_24`. Every fifth description mentions another tool as `tool:some_name`, which section 6.4 will have to scrub. The server name stays on the card for routing. It is not part of the JSON the model would see.

The token count is the chapter 2 estimate: characters divided by 4, rounded up. It is not a tokenizer, and it is not a bill. Providers charge for prompt tokens, so this number is the input those tokens would be.

```rust
pub fn approx_tokens(text: &str) -> usize {
    text.chars().count().div_ceil(4)
}

pub fn schema_cost(cards: &[ToolCard]) -> SchemaCost {
    let body = schema_json(cards);
    SchemaCost {
        tools: cards.len(),
        bytes: body.len(),
        tokens: approx_tokens(&body),
    }
}
```

`mention_count` is the selection problem stated as a count. It asks how many cards contain every token of the query. For "rename a file" that is 192 tools out of 10,000. Stuffing every schema into the prompt does not make those 192 into one.

### Build, run, and benchmark

```bash
cargo nextest run -p ch06-1-harness-core --locked
cargo run -p ch06-1-harness-core --example demo
cargo bench -p ch06-1-harness-core --bench schema_cost -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Nextest, 6 tests:

```text
     Summary [   0.158s] 6 tests run: 6 passed, 0 skipped
```

The demo. Generation is part of this run. The bench below times `schema_cost` only, with the cards already built.

```text
tools=50 bytes=10030 tokens=2508 mentions_rename_file=9
tools=1000 bytes=204909 tokens=51228 mentions_rename_file=21
tools=10000 bytes=2070285 tokens=517572 mentions_rename_file=192
```

Ten thousand schemas are about 2.07 MB and 517,572 estimated tokens. A 50-tool registry was 2,508 tokens. The 10,000-tool prompt is about 206 times that, which tracks the tool count (the extra comes from longer numeric suffixes). 192 cards match the words "rename" and "file". The model would still have to pick among them, after reading the other 9,808 schemas too.

```text
schema_tokens_10000     time:   [21.036 ms 21.061 ms 21.084 ms]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `schema_tokens_1000` | 923.1 µs | under the 10,000-tool budget | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0, sample size 10 |
| `schema_tokens_10000` | 21.06 ms | < 50 ms | same |

The `time:` line is the slope. The table is the mean point estimate. Counting the tokens is cheap next to sending half a million of them on every turn.

### Milestone demo

The example prints the token estimate. It does not call a model.

```bash
cargo run -p ch06-1-harness-core --example demo
```

```text
tools=50 bytes=10030 tokens=2508 mentions_rename_file=9
tools=1000 bytes=204909 tokens=51228 mentions_rename_file=21
tools=10000 bytes=2070285 tokens=517572 mentions_rename_file=192
```

This section has no HTTP client. The chapter's optional OpenRouter command is the section 6.5 binary. I did not make that call. With the key unset it stops first:

```bash
cargo run -p ch06-5-harness-cli -- ask "rename a file"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## 6.2 A searchable index

Build the index once. A query then walks the terms it actually used, instead of scanning every description in the prompt.

> **Rust sidebar: an inverted index**
> - Each word points at the tools that contain it, plus how many times. That map is the inverted index. BM25 is a formula on top of it.
> - `idf` is higher when the word is rare. `tf` is how often the word appears in one tool. Short tools are not punished as hard as long ones (`k1` is 1.2, `b` is 0.75). The name is counted twice, because that is what a person types.
> - This is the Lucene form: `ln((N - df + 0.5) / (df + 0.5) + 1)`.

`build_available` is the Hermes check. A card that fails the closure is not inserted, so a later turn cannot reveal it.

Embeddings sit behind a trait. `HashedBag` hashes tokens into a fixed vector. It is not a model. Search still ranks with BM25. A test installs a counting embedder and checks that `search` calls it once and returns the same names BM25 would have returned alone. The bench does not install an embedder.

On 10,000 tools, "rename a file" ties. Every server has a `rename_file` tool, the scores match, and the tie breaks by name, so the first rows are `calendar_rename_file_*`. That is the reason section 6.4 routes before it searches. Keyword search finds the words. It does not know that "file" belongs to the files server.

### Build, run, and benchmark

```bash
cargo nextest run -p ch06-2-harness-core --locked
cargo run -p ch06-2-harness-core --example demo
cargo bench -p ch06-2-harness-core --bench tool_index -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

```text
     Summary [   0.015s] 5 tests run: 5 passed, 0 skipped
```

```text
indexed=10000
6.240 calendar_rename_file_1562 (calendar)
6.240 calendar_rename_file_2074 (calendar)
6.240 calendar_rename_file_2586 (calendar)
6.240 calendar_rename_file_26 (calendar)
6.240 calendar_rename_file_3098 (calendar)
```

The clone of the 10,000 cards is outside the build measurement. Search runs against an index that was built before the timer.

```text
index_build_10000       time:   [14.637 ms 14.659 ms 14.682 ms]
index_search_10000      time:   [146.60 µs 146.86 µs 147.12 µs]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `index_build_10000` | 14.66 ms | < 50 ms, once per catalog | same machine as 6.1 |
| `index_search_10000` | 146.8 µs | < 1 ms | same |

### Milestone demo

```bash
cargo run -p ch06-2-harness-core --example demo
```

```text
indexed=10000
6.240 calendar_rename_file_1562 (calendar)
6.240 calendar_rename_file_2074 (calendar)
6.240 calendar_rename_file_2586 (calendar)
6.240 calendar_rename_file_26 (calendar)
6.240 calendar_rename_file_3098 (calendar)
```

The optional live command is still the section 6.5 CLI, and it was not run. With the key unset:

```bash
cargo run -p ch06-5-harness-cli -- ask "rename a file"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## 6.3 search_tools and lazy loading

The model starts with one tool, `search_tools`. A call returns a short listing: name and a trimmed description. It does not attach the matched schemas to this turn. `begin_turn` appends those names, and that is the next turn's prompt. The core prefix is written once and left alone.

> **Rust sidebar: the stable prefix**
> - Chapter 2 kept the system prompt byte-identical so a provider can cache it. The same rule applies to tool schemas.
> - `core_prompt` is `search_tools` and a trailing newline. `visible_prompt` starts with those exact bytes and only then writes the revealed schemas.
> - `VecDeque` is the revealed list. New names go on the back. When the list is past the cap, `pop_front` drops the oldest revealed name. The core prefix is not in that deque, so eviction cannot rewrite it.

```rust
pub fn begin_turn(&mut self) {
    let staged = std::mem::take(&mut self.staged);
    for name in staged {
        if !self.is_visible(&name) {
            self.revealed.push_back(name);
        }
    }
    while self.revealed.len() > self.cap {
        self.revealed.pop_front();
    }
}
```

The cap on the demo is 8, and one search stages 5 names, so nothing is evicted yet. A test with a cap of 2 searches twice and checks that the core string is still the original bytes after the oldest revealed names are gone.

### Build, run, and benchmark

```bash
cargo nextest run -p ch06-3-harness-core --locked
cargo run -p ch06-3-harness-core --example demo
cargo bench -p ch06-3-harness-core --bench search_tool -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

```text
     Summary [   0.058s] 3 tests run: 3 passed, 0 skipped
```

Two thousand tools. The search result is the harness listing, not a model reply.

```text
tools=2000
all_schema_tokens=103063
visible_tokens_before=55
this_turn=search_tools
search:
calendar_rename_file_1562 — Use the calendar server to rename a file. Tool number 1562.
calendar_rename_file_26 — Use the calendar server to rename a file. Tool number 26.
calendar_rename_file_538 — Use the calendar server to rename a file. Tool number 538.
db_rename_file_1567 — Use the db server to rename a file. Tool number 1567.
db_rename_file_31 — Use the db server to rename a file. Tool number 31.
next_turn=search_tools,calendar_rename_file_1562,calendar_rename_file_26,calendar_rename_file_538,db_rename_file_1567,db_rename_file_31
visible_tokens_after=305
core_prompt_unchanged=true
```

The full catalog is 103,063 estimated tokens. The first turn shows 55, which is `search_tools` alone. After the next turn appends five schemas, the visible prompt is 305 tokens, and the core bytes are unchanged.

```text
full_schema_tokens_10000
                        time:   [18.553 ms 18.587 ms 18.618 ms]
lazy_visible_prompt_10000
                        time:   [12.230 µs 12.249 µs 12.277 µs]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `full_schema_tokens_10000` | 18.59 ms | the cost of rendering every schema | same machine as 6.1 |
| `lazy_visible_prompt_10000` | 12.26 µs | < 1 ms | same |

### Milestone demo

The listing is the harness, not a model reply.

```bash
cargo run -p ch06-3-harness-core --example demo
```

```text
tools=2000
all_schema_tokens=103063
visible_tokens_before=55
this_turn=search_tools
search:
calendar_rename_file_1562 — Use the calendar server to rename a file. Tool number 1562.
calendar_rename_file_26 — Use the calendar server to rename a file. Tool number 26.
calendar_rename_file_538 — Use the calendar server to rename a file. Tool number 538.
db_rename_file_1567 — Use the db server to rename a file. Tool number 1567.
db_rename_file_31 — Use the db server to rename a file. Tool number 31.
next_turn=search_tools,calendar_rename_file_1562,calendar_rename_file_26,calendar_rename_file_538,db_rename_file_1567,db_rename_file_31
visible_tokens_after=305
core_prompt_unchanged=true
```

The optional live command was not run. With the key unset:

```bash
cargo run -p ch06-5-harness-cli -- ask "rename a file"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## 6.4 Route, then shrink

A one-line summary per server is the first level: `files: list, read, write, rename, and move a file`. The query is scored against those summaries. "rename a file" picks `files` because that summary contains both `rename` and `file`. "rename a branch" picks `git`. The second level runs BM25 only inside that server's index.

`shrink_card` lowercases the name, turns other characters into underscores, and prefixes the server when the name does not already start with it. Descriptions are cut on a word boundary at 80 characters. `rewrite_description` splits on sentences and drops one that mentions `tool:` plus a name the available set does not contain. That is the Hermes rewrite: the text the model reads only names tools it can call.

On this generator the descriptions are already one sentence, plus a short "see also" on every fifth tool. Shrinking 2,000 schemas went from 412,249 bytes to 401,057 bytes. The large win in this section is the route, which stops the calendar tie from section 6.2. A catalog of paragraph-length descriptions would drop more under the 80-character cut. The rewrite still matters when a sentence points at a tool from another server.

### Build, run, and benchmark

```bash
cargo nextest run -p ch06-4-harness-core --locked
cargo run -p ch06-4-harness-core --example demo
cargo bench -p ch06-4-harness-core --bench routing -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

```text
     Summary [   0.012s] 4 tests run: 4 passed, 0 skipped
```

```text
route=files
files_rename_file_1048 (files)
files_rename_file_24 (files)
files_rename_file_536 (files)
files_rename_file_1560 (files)
files_read_file_520 (files)
full_bytes=412249
shrunk_bytes=401057
```

`files_read_file_520` is in the top five because the query token `file` is in its name. The four `rename` hits are the ones the route was for. All five are on the files server.

```text
routed_search_10000     time:   [15.972 µs 16.017 µs 16.056 µs]
shrink_schemas_10000    time:   [31.204 ms 31.351 ms 31.452 ms]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `routed_search_10000` | 16.00 µs | < 1 ms | same machine as 6.1 |
| `shrink_schemas_10000` | 31.22 ms | < 50 ms, when the catalog changes | same |

Routed search is about nine times faster than the full-index search in 6.2 (16.00 µs against 146.8 µs) because it scores one server, not eight.

### Milestone demo

```bash
cargo run -p ch06-4-harness-core --example demo
```

```text
route=files
files_rename_file_1048 (files)
files_rename_file_24 (files)
files_rename_file_536 (files)
files_rename_file_1560 (files)
files_read_file_520 (files)
full_bytes=412249
shrunk_bytes=401057
```

The optional live command was not run. With the key unset:

```bash
cargo run -p ch06-5-harness-cli -- ask "rename a file"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## 6.5 The registry at scale

`SyntheticSource::list` clones the generated vec and counts the call. That clone is what an MCP `tools/list` costs: the server sends the catalog again. `ListCache` keeps an `Arc<[ToolCard]>`. A second `list` returns the same allocation and does not call the source. `notify_list_changed` sets the dirty flag. The next `list` calls the source again.

Chapter 4's `ToolListCache` is the connection that sees the real notification. This crate does not link `rmcp`, because both libraries are named `harness_core`. The handoff is a vec of cards. `ListedTool` is a name, a description, and a schema. `cards_from_listed` sets `server` on each `ToolCard` and leaves that field off the schema the model sees. `ListCache::from_cards` stores the vec. `replace_cards` is the refresh after `list_changed`: it replaces the source and marks the cache dirty. The test `a_real_tools_list_uses_the_same_cache` loads an `echo` card from the fixture's description, sees one list call and a shared `Arc` on the second read, then replaces the list with `echo` and `extra` and sees both on the next read. The 10,000-tool benches still call `ListCache::new`.

> **Rust sidebar: Arc**
> - `Arc<[ToolCard]>` is a shared slice. Cloning the `Arc` copies the pointer, not the 10,000 cards.
> - Chapter 3 already stored tools behind `Arc`. Here the shared thing is the catalog snapshot, and each agent borrows it through a filter.

`AgentView` is the per-agent toolset: an agent name and the servers it may use. Two agents can hold the same cached `Arc` and see different servers. The loaded names, the core tool plus whatever `begin_turn` has revealed, are the ones chapter 3's registry would actually register. The other cards stay as data until a search reveals them.

Two groupings use this same filter later, and this chapter does not build them. Code mode is one code tool plus the sandbox in chapter 13: the model sees that tool, and the sandbox runs the code, instead of seeing thousands of file tools. A specialist is a sub-agent with its own server set. The supervisor in chapter 29 is where that sub-agent is called. Both are a view over one catalog.

### Build, run, and benchmark

```bash
cargo nextest run -p ch06-5-harness-core -p ch06-5-harness-cli --locked
cargo run -p ch06-5-harness-core --example demo
cargo run -p ch06-5-harness-cli -- --mock ask "rename a file"
cargo bench -p ch06-5-harness-core --bench lazy_list -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Library, 4 tests. CLI, 3 tests. The CLI run includes a wiremock server that checks the live request body contains `search_tools` and does not contain `files_list_file_0`. That fake replies `from-fake`. It is not an OpenRouter transcript.

```text
     Summary [   0.012s] 4 tests run: 4 passed, 0 skipped
     Summary [   0.011s] 3 tests run: 3 passed, 0 skipped
```

The example adds the cache counters to the section 6.3 demo:

```text
list_calls_after_two_reads=1
list_calls_after_invalidate=2
```

The cold cache builds 10,000 cards and clones them once. That setup sits inside `cache_cold_10000`. The hit that follows only clones the `Arc`.

```text
cache_cold_10000        time:   [12.051 ms 12.077 ms 12.103 ms]
cache_hit_10000         time:   [17.541 ns 17.571 ns 17.605 ns]
agent_filter_10000      time:   [76.699 µs 76.885 µs 77.268 µs]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `cache_cold_10000` | 12.07 ms | one `tools/list` worth of cloning | same machine as 6.1 |
| `cache_hit_10000` | 17.56 ns | < 1 µs | same |
| `agent_filter_10000` | 77.00 µs | < 1 ms | same |

CI runs these five steps on Ubuntu, macOS, and Windows. The benches run on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml). A push to `main` appends the medians to the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/).

### Milestone demo

`--mock` is the default. It does not call a model. The `event:` lines are the harness reporting the search.

```bash
cargo run -p ch06-5-harness-cli -- --mock ask "rename a file"
```

```text
$ cargo run -p ch06-5-harness-cli -- --mock ask "rename a file"
tools=2000
all_schema_tokens=103063
visible_tokens_before=55
this_turn=search_tools
search:
calendar_rename_file_1562 — Use the calendar server to rename a file. Tool number 1562.
calendar_rename_file_26 — Use the calendar server to rename a file. Tool number 26.
calendar_rename_file_538 — Use the calendar server to rename a file. Tool number 538.
db_rename_file_1567 — Use the db server to rename a file. Tool number 1567.
db_rename_file_31 — Use the db server to rename a file. Tool number 31.
next_turn=search_tools,calendar_rename_file_1562,calendar_rename_file_26,calendar_rename_file_538,db_rename_file_1567,db_rename_file_31
visible_tokens_after=305
core_prompt_unchanged=true
event: tool-call search_tools id=search args={"query":"rename a file"}
event: text-delta calendar_rename_file_1562 — Use the calendar server to rename a file. Tool number 1562.
event: done
```

Without `--mock`, and with `OPENROUTER_API_KEY` unset, the same binary stops before any HTTP call. This is that run:

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

A live call, when a key is present, posts to OpenRouter with the chapter 1 client and a single tool, `search_tools`. I did not make that call.

## Guardrails in this chapter

A size cap on what the model is shown, which is the L2 idea applied to the tool list. The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**A cap on revealed schemas.** `ToolSession::new` takes a `cap`. `search_tools` stages names and returns a short listing, not the schemas. `begin_turn` appends those names, then drops the oldest revealed name while `revealed.len() > cap`. The core prefix, the `search_tools` schema, is not rewritten. The cap is a constructor argument. There is no global constant, and nothing refuses a catalog of 10,000 tools. Ten thousand is the bench size.

**Short descriptions.** `trim_words` cuts on a character count, at the last whitespace inside the window. Search blurbs and routed descriptions both pass 80. That is the only description cap in this chapter.

Hermes F3 (`agent/prompt_builder.py`, `CONTEXT_FILE_MAX_CHARS`, 20,000 characters, keeping the head and a tail) caps context files such as `AGENTS.md`. That belongs to chapter 34. This chapter does not apply it to tool output either.

## Chapter summary

- **Schema cost.** 10,000 MCP-shaped schemas were 2,070,285 bytes and 517,572 estimated tokens. 192 of them mention both "rename" and "file". Counting those tokens took 21.06 ms.
- **Index.** BM25 over 10,000 tools built in 14.66 ms and answered a top-8 query in 146.8 µs. A `check_fn` keeps unavailable tools out of the index. `Embedder` is a seam. Ranking stays BM25.
- **`search_tools`.** The first turn is 55 tokens. Five revealed schemas on the next turn bring it to 305, against 103,063 for all 2,000 tools. The core prefix stays byte-identical, including when the cap evicts the oldest revealed names. Rendering the lazy prompt took 12.26 µs.
- **Routing.** "rename a file" stays on the files server. That search took 16.00 µs. Shrinking 2,000 short descriptions went from 412,249 bytes to 401,057. Sentences that name an unavailable tool are dropped.
- **Cache.** Two reads of the stand-in `tools/list` call the source once. A stand-in `tools/list_changed` makes the next read call it again. A real list uses the same cache through `from_cards` and `replace_cards`. The cache hit was 17.56 ns. One agent's server filter over 10,000 tools was 77.00 µs.

## What's next

Chapter 4 is the MCP client. A session there lists tools and hears `list_changed`. `cards_from_listed` is how that list becomes the cards this chapter already caches. The generator stays for the benches.

This chapter is the client half of the catalog. It decides what enters the prompt: the task, the budget, the cache, and which servers an agent may see. Chapter 5 is the process that holds a fixed list and a 32/8 call gate. It still binds no socket. Chapter 7 is the server-side catalog and search, on that process. Chapter 9 is the agent loop: the model chooses a tool, the dispatcher runs it, and the result comes back as a message. `search_tools` is one of those choices. The loop is also where a cancellation token and a budget show up, and where the saved eval set starts.
