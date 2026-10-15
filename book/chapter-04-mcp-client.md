# Chapter 4: MCP Client

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 3 can call a tool that lives in this process. A real workspace does not keep every tool there. A file server, a browser, a notes database: each one is its own program, and the way those programs advertise tools is MCP. This chapter is the client. It speaks MCP over a child process's stdin and stdout, and over streamable HTTP on `127.0.0.1`. The tools that come back are the same `Tool` values chapter 3 already dispatches.

The server in the tests is a fixture in this repo, `steps/ch04-fixture`. It does not dial an external host. Linux, macOS, and Windows all run it. `echo` returns its text argument and is marked read-only. `note` stores a line and is a write. `grow` reveals a fourth tool, `extra`, and sends `notifications/tools/list_changed`. `FIXTURE_DELAY_MS` makes `echo` sleep, so a test can see overlapping calls.

| Section | Step | What it adds |
|---|---|---|
| 4.1 | `steps/ch04-1-stdio` | an `rmcp` session over stdio, list and call |
| 4.2 | `steps/ch04-2-http` | the same session over streamable HTTP |
| 4.3 | `steps/ch04-3-registry` | remote tools in the chapter 3 registry and dispatcher |
| 4.4 | `steps/ch04-4-cache` | a `tools/list` cache dropped by `list_changed` |

Each step is its own copy. Section 4.1 still builds after 4.4 exists. The library crate in every copy is still named `harness_core`. Two of those crates cannot be dependencies of one package, which is why chapter 6 does not link `rmcp`. It takes a list of cards. The end of this chapter is where that list comes from.

`rmcp` 3.5.1 needs rustc 1.88. These four crates, and the fixture, set `rust-version = "1.88"`. The earlier chapters stay on 1.85. CI installs stable, which is past 1.88 on the runners I use.

> **How Hermes does it**
>
> `tools/mcp_tool.py` opens a server from a config block. Stdio is a `command` and `args`. HTTP and Streamable HTTP are a `url` and optional `headers`. SSE is a third transport in that same file. Discovered tools are registered into the registry from chapter 3's box, `tools/registry.py`, under the same `ToolEntry` shape as a built-in. A second registration of a built-in name is still rejected.
>
> `notifications/tools/list_changed` triggers a refresh: list again, drop tools that disappeared, register tools that appeared. A burst of those notifications is coalesced in `tools/mcp_tool_health.py`, in `_schedule_tools_refresh`: one refresh runs, and at most one follow-up is queued behind it. Prompt and resource list-changed notifications are received and not fully acted on. The embedded `hermes mcp serve` command is stdio-only. The client speaks both stdio and HTTP.
>
> This chapter's client is the same split. `McpSession` is the connection. `McpTool` is the registry entry. `readOnlyHint` becomes `ToolKind`. A duplicate name is `ToolError::Duplicate`. The cache in 4.4 drops on every notification. It does not coalesce a burst. That difference is a pitfall at the end of the chapter.

[personal note: the first MCP server you pointed a client at, and what it listed]

## What this chapter decides

The fixture is source, included into each step. Cargo's `artifact = "bin"` dependency needs `-Z bindeps` on the 1.99 toolchain this repo builds with, so a step cannot depend on the fixture binary that way. Each step crate has a `[[bin]]` named `mcp_fixture` whose file is one line:

```rust
include!("../../../../../ch04-fixture/src/main.rs");
```

Tests then see `CARGO_BIN_EXE_mcp_fixture`. Examples and benches do not, so `fixture_binary()` falls back to the binary sitting next to the current executable, and on Windows it appends `.exe`. The fixture's own crate comments are `//`, not `//!`, because an inner doc comment is illegal once `include!` is not the first token of a crate that already has its own comment.

`readOnlyHint` absent means `Write`. MCP lets a server omit the hint. Treating the omission as a read would let a mutating tool overlap its neighbors. `note` omits the hint on purpose, and the registry records it as a write. `echo` sets the hint, and eight of those calls share the chapter 3 bound.

The list cache is per session, and one notification drops it. Hermes coalesces. I want the notification to be visible in a test before I hide a burst. Chapter 6's `ListCache` is the place a large catalog is snapshotted for many agents. This chapter's `ToolListCache` is the place one connection remembers the last `tools/list`.

No step calls OpenRouter. `--mock` on the 4.4 CLI runs the fixture. Without `--mock`, a missing `OPENROUTER_API_KEY` is the chapter 1 error, and a key that is present is refused with "this demo does not place a live OpenRouter call". The refusal is the whole live path.

## 4.1 Stdio

`McpSession::stdio` builds a `tokio::process::Command`, wraps it in `rmcp`'s `TokioChildProcess`, and serves a small handler.

```rust
impl ClientHandler for Watch {
    fn on_tool_list_changed(
        &self,
        _context: NotificationContext<RoleClient>,
    ) -> impl std::future::Future<Output = ()> + Send + '_ {
        self.changed.store(true, Ordering::SeqCst);
        std::future::ready(())
    }
}
```

> **Rust sidebar: a future returned from a trait**
> - `on_tool_list_changed` is required by `ClientHandler`. The return type is "some `Future` that is `Send`". The caller does not name the concrete type.
> - The body stores a flag and returns `std::future::ready(())`, a future that is already done. The notification handler does not await the network.
> - `Ordering::SeqCst` on that flag is more synchronization than the flag needs. A later `list` on the same session reads it with the same ordering, and the test polls until `extra` appears. I would rather the test be obvious than the atomic be clever.

`list_tools` calls `list_all_tools`, copies the name, description, and input schema, and sorts by name so a test can compare lists. `read_only` is `annotations.read_only_hint`, and `unwrap_or(false)` is the write default. `call_text` sends `CallToolRequestParams` and returns the first text content block. `list_calls` counts lists, which section 4.4 asserts. `shutdown` takes `self` and calls `RunningService::cancel`. Drop of the child transport kills the process if cancel never runs. `TokioChildProcess`'s `Drop` does that kill.

The fixture's `grow` cannot notify from inside the handler that is still borrowing the peer, so it spawns the notify:

```rust
if request.name == "grow" {
    self.extra.store(true, Ordering::SeqCst);
    let peer = context.peer.clone();
    tokio::spawn(async move {
        let _ = peer.notify_tool_list_changed().await;
    });
    return Ok(CallToolResult::success(vec![ContentBlock::text("grew")]).into());
}
```

`list_tools` on the server drops the tool named `extra` until that flag is set. HTTP mode shares one `Arc<AtomicBool>` across requests, so a `grow` on one request is visible to the next. Stdio mode is one `Fixture` for the life of the process, and the same flag lives on that value.

### The stdio tests

`steps/ch04-fixture` has three tests: list and call `echo` over stdio, `grow` delivers `list_changed` and a later list contains `extra`, and the same list-and-call over HTTP. Section 4.1's crate repeats the two stdio cases against `McpSession`.

### Build, run, and benchmark

```bash
cargo nextest run -p ch04-mcp-fixture -p ch04-1-harness-core --locked
cargo run -p ch04-1-harness-core --example demo --quiet
cargo bench -p ch04-1-harness-core --bench stdio_call -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Fixture, 3 tests. Library, 2 tests.

```text
     Summary [   0.021s] 3 tests run: 3 passed, 0 skipped
     Summary [   0.010s] 2 tests run: 2 passed, 0 skipped
```

The example lists the three tools that exist before `grow`, then calls `echo`.

```text
tools: echo, grow, note
echo: hello
```

The bench is one `echo` round trip on a session that stays open. The `time:` line below is Criterion's slope. The table uses the mean from `estimates.json`.

```text
stdio_echo_round_trip   time:   [70.240 µs 70.665 µs 71.238 µs]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `stdio_echo_round_trip` | 71.29 µs | one local stdio call, no model | 4 vCPU, sample size 10, warm-up 1 s, measurement 2 s |

CI runs the fixture and this step on Ubuntu, macOS, and Windows. The bench runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml). A push to `main` appends the medians to the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/).

### Milestone demo

```bash
cargo run -p ch04-1-harness-core --example demo --quiet
```

```text
tools: echo, grow, note
echo: hello
```

The optional live command was not run. With the key unset, the section 4.4 binary is the one that prints the chapter 1 error. This section has no CLI.

```bash
cargo run -p ch04-4-harness-cli -- ask "hello"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## 4.2 Streamable HTTP

`HttpFixture::spawn` starts `mcp_fixture --http`, reads one stderr line, and keeps the child. The line is `MCP_URL=http://127.0.0.1:<port>/mcp`. The port is `0`, so the OS picks it. Drop calls `start_kill`.

The server side is `StreamableHttpService` with a local session manager, JSON responses, and legacy session mode off. Axum nests that service at `/mcp` and binds `127.0.0.1:0`. The factory closes over the shared `extra` flag, so `grow` survives from one HTTP request to the next even though each request builds a `Fixture`.

```rust
pub async fn streamable_http(url: &str) -> Result<Self, SessionError> {
    let changed = Arc::new(AtomicBool::new(false));
    let transport = rmcp::transport::StreamableHttpClientTransport::from_uri(url.to_string());
    let inner = Watch { changed: Arc::clone(&changed) }
        .serve(transport)
        .await
        .map_err(|error| SessionError::Protocol(error.to_string()))?;
    Ok(Self { inner, changed, list_calls: AtomicU64::new(0) })
}
```

`stdio` now delegates to `stdio_with`, which sets environment variables on the child. Section 4.3 uses that to pass `FIXTURE_DELAY_MS`.

### Build, run, and benchmark

```bash
cargo nextest run -p ch04-2-harness-core --locked
cargo run -p ch04-2-harness-core --example demo --quiet
cargo bench -p ch04-2-harness-core --bench stdio_call -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The bench target is still named `stdio_call`. The function inside it is `http_echo_round_trip`. Library, 3 tests: the two stdio cases plus one HTTP list-and-call.

```text
     Summary [   0.034s] 3 tests run: 3 passed, 0 skipped
```

```text
tools: echo, grow, note
echo: hello
url: http://127.0.0.1:44691/mcp
```

That port was the one the kernel assigned on this run. The next run prints a different port.

```text
http_echo_round_trip    time:   [213.70 µs 214.94 µs 216.48 µs]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `http_echo_round_trip` | 215.7 µs | one local HTTP call, no model | same machine as 4.1 |

The HTTP trip is about three times the stdio trip (215.7 µs against 71.29 µs). Both are loopback. The difference is the HTTP framing, not a network. The series is on the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/).

### Milestone demo

```bash
cargo run -p ch04-2-harness-core --example demo --quiet
```

The tools line and the echo line match section 4.1. The `url:` line is the ephemeral port from the run above. The missing-key command is the same 4.4 CLI command as in 4.1.

## 4.3 The same registry

`McpTool` holds `Arc<McpSession>` and the `RemoteTool` from the list. `schema()` copies the name, description, and input schema. `kind()` is `Read` when `read_only` is set and `Write` otherwise. `call` is `call_text`, and a protocol error becomes `ToolError::Failed`.

> **Rust sidebar: Arc on the session**
> - The registry stores `Arc<dyn Tool>`. Each remote tool needs the session, and eight reads call it at once.
> - `Arc<McpSession>` is that share. Cloning it copies the pointer. `call_text` takes `&self`, so the tasks do not need `&mut`.
> - `shutdown` takes `self` by value. An `Arc` cannot be moved into it. Dropping the last `Arc` drops the `RunningService`, and the child transport's `Drop` kills the process.

`register_remote` lists once and registers each tool. A native `EchoTool` plus the fixture's `echo` returns `ToolError::Duplicate` on the first remote name, which sorts to `echo`. A native `StaticTool` named `ping` plus the remote list succeeds. The dispatcher is the chapter 3 dispatcher, bound 8. `LatencyTool` from the chapter 3 copy is gone in this crate. Clippy treats it as dead code, and the fixture's delay is the latency now.

### The merge tests

`remote_tools_share_the_registry_with_a_native_tool` expects the duplicate, then registers `ping` and checks `echo` is `Read`, `note` is `Write`, and a call returns `from-registry`.

`eight_remote_reads_overlap_and_a_write_does_not` sets `FIXTURE_DELAY_MS=40`. Eight `echo` calls through `Dispatcher::run` finish in under 200 ms. The same eight through `run_sequential` take more than 250 ms. A batch of `echo`, `note`, `echo` keeps the write exclusive: the test checks the outputs and the order. Missing names still group with the reads, the same rule as chapter 3.

### Build, run, and benchmark

```bash
cargo nextest run -p ch04-3-harness-core --locked
cargo run -p ch04-3-harness-core --example demo --quiet
cargo bench -p ch04-3-harness-core --bench stdio_call -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Library, 4 tests.

```text
     Summary [   0.485s] 4 tests run: 4 passed, 0 skipped
```

The example registers `ping`, then the remote tools.

```text
tools: echo, grow, note, ping
echo: hello
```

The bench is 50 `echo` calls with no sleep, concurrent against sequential. This is protocol overhead. It is not the 40 ms overlap test. Fifty calls through a bound of 8 are faster together, and they are not eight times faster, because each call is a short stdio trip and the child still has to answer them.

```text
mcp_50_concurrent       time:   [1.8977 ms 1.9080 ms 1.9171 ms]
mcp_50_sequential       time:   [3.5651 ms 3.5902 ms 3.6127 ms]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `mcp_50_concurrent` | 1.908 ms | 50 calls, bound 8, no sleep | same machine as 4.1 |
| `mcp_50_sequential` | 3.603 ms | the same 50, one at a time | same |

The 40 ms test is the one that matches "about one round trip": eight delayed reads finished together, and eight in a row did not. The bench above is the overhead of the protocol when the tool itself does not wait. Both rows are on the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/).

### Milestone demo

```bash
cargo run -p ch04-3-harness-core --example demo --quiet
```

```text
tools: echo, grow, note, ping
echo: hello
```

## 4.4 A cache, and the chapter 6 seam

`ToolListCache` holds the session and `Option<Arc<[RemoteTool]>>`. `list` calls `take_list_changed`. A `true` drops the cached `Arc`. A hit clones the `Arc` and does not call `list_tools`. A miss lists, stores the slice, and returns it.

```rust
pub async fn list(&mut self) -> Result<Arc<[RemoteTool]>, SessionError> {
    if self.session.take_list_changed() {
        self.cached = None;
    }
    if let Some(cached) = &self.cached {
        return Ok(Arc::clone(cached));
    }
    let tools = self.session.list_tools().await?;
    let cached = Arc::<[RemoteTool]>::from(tools);
    self.cached = Some(Arc::clone(&cached));
    Ok(cached)
}
```

The test calls `grow`, then polls `list` until `extra` is present. `list_calls` is at least 2. The poll is there because the notify is spawned, so the flag can land a few milliseconds after `grew` returns.

Chapter 6 does not depend on this crate. Two `harness_core` libraries cannot sit in one package. The seam is data. `ListedTool` is a name, a description, and a schema. `cards_from_listed(server, tools)` builds `ToolCard`s and sets `server`, which stays off the schema the model sees. `ListCache::from_cards` and `replace_cards` store that vec and mark the cache dirty. The synthetic generator is still what the 10,000-tool benches measure. A real list enters through the same two methods. The test `a_real_tools_list_uses_the_same_cache` builds an `echo` card, sees one `list` call and a shared `Arc` on the second read, then `replace_cards` adds `extra` and the next `list` returns both.

### Build, run, and benchmark

```bash
cargo nextest run -p ch04-4-harness-core --locked
cargo run -p ch04-4-harness-core --example demo --quiet
cargo run -p ch04-4-harness-cli -- --mock ask "hello"
cargo bench -p ch04-4-harness-core --bench stdio_call -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Library, 5 tests. The cold bench builds a new cache inside the async iter. The hit bench calls `block_on` inside `iter` because `list` needs `&mut self`, and Criterion's `to_async` could not hold that borrow across the future.

```text
     Summary [   0.476s] 5 tests run: 5 passed, 0 skipped
```

```text
tools: echo, grow, note
list_calls_after_two_reads=1
echo: hello
```

```text
mcp_list_cold           time:   [81.642 µs 81.968 µs 82.584 µs]
mcp_list_hit            time:   [167.80 ns 168.07 ns 168.46 ns]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `mcp_list_cold` | 82.87 µs | one `tools/list` over stdio | same machine as 4.1 |
| `mcp_list_hit` | 168.0 ns | < 1 µs | same |

Chapter 6's synthetic hit was 17.56 ns because it clones an `Arc` and does no protocol. This hit is the same clone plus the atomic flag check, measured with `block_on` inside the iter, so the runtime setup is inside the sample. 168.0 ns is still under the 1 µs budget. The cold and hit series are on the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/).

### Milestone demo

`--mock` runs the fixture. It does not call a model. The `event:` lines are the harness reporting the echo.

```bash
cargo run -p ch04-4-harness-cli -- --mock ask "hello"
```

```text
tools: echo, grow, note
echo: hello
event: text-delta hello
event: done
```

Without `--mock`, and with `OPENROUTER_API_KEY` unset, the same binary stops before any MCP session and before any HTTP call:

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

`--api-key` overrides the environment variable for that check. A key that is present, from the flag or from the variable, still does not place a live OpenRouter call. The process prints `this demo does not place a live OpenRouter call` and exits 1. I did not paste a live transcript.

## Pitfalls

The rustc floor is 1.88 for these crates only. `cargo add` on a crate that still says 1.85 resolves `rmcp` 2.x, and that API is not the one this chapter calls. Pin `3.5.1` after the rust-version bump.

A missing `readOnlyHint` is a write. A server that forgets the hint on a search tool will serialize that search behind every other call. The hint is the whole policy, the same way `ToolKind` was in chapter 3.

`grow` spawns the notification. Calling `notify_tool_list_changed` inline deadlocks against the handler that is still on the peer. The test polls for `extra` because the spawn can lose the race with the next `list`.

Every `list_changed` drops the cache. Hermes coalesces a burst into one refresh plus one follow-up. A server that notifies on a timer will make this client list on every timer tick. Coalescing belongs with the health loop, and this chapter does not have one yet.

`shutdown` needs the session by value. Code that has only an `Arc` should drop the `Arc`. The child dies in `Drop`. Calling `shutdown` and also dropping a second clone will not compile, which is the point of taking `self`.

The HTTP URL's port changes every run. A test that asserts a fixed port will fail. Read `MCP_URL=` from stderr.

## Guardrails in this chapter

Still layer L5, for tools that arrive from another process. The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**An absent `readOnlyHint` is a write.** `list_tools` reads `annotations.read_only_hint` and uses `unwrap_or(false)` when the hint is missing. `RemoteTool` records that bool, and its field comment says an absent hint means the tool is treated as a write. The chapter 3 dispatcher then runs it alone.

**A duplicate remote name is refused.** `register_remote` lists the session once and calls `registry.register` for each tool. The `?` returns `ToolError::Duplicate` and stops the rest of that registration.

Sampling and elicitation limits (M7, `tools/mcp_tool_sampling.py`) are not in this client. Chapter 38 is where a server-driven call gets a limit, next to the gateway's own auth.

## Chapter summary

- **Stdio.** `McpSession` lists `echo`, `grow`, and `note`, and `echo` returns its text. One round trip measured 71.29 µs.
- **HTTP.** The same three tools over streamable HTTP on `127.0.0.1`, JSON responses, port chosen by the OS. One round trip measured 215.7 µs.
- **Registry.** Remote tools are `Arc<dyn Tool>`. A duplicate `echo` is rejected. `note` is a write because it has no read-only hint. Eight delayed reads overlapped. Fifty zero-delay calls measured 1.908 ms concurrent and 3.603 ms sequential.
- **Cache.** Two reads of `tools/list` increment `list_calls` once. `grow` sends `list_changed`, and the next read sees `extra`. The cold list measured 82.87 µs. The hit measured 168.0 ns.
- **Chapter 6.** `cards_from_listed`, `ListCache::from_cards`, and `replace_cards` take a real list without linking `rmcp`. The 10,000-tool benches still use the generator. The section crates for that chapter stay `steps/ch05-*`.

[personal note: which part of MCP was harder than the protocol, the process lifetime or the list cache]

## What's next

Chapter 5 is a minimal MCP server, a separate process from this client. The skeleton is `mcp-gateway/`. It offers an empty catalog and binds no socket. The chapter is not written yet. That process is where the catalog will live.

Chapter 6 already measured what happens when the list is thousands of tools long. The generator stays. A session from this chapter can feed that cache through `cards_from_listed`. That chapter is the client half: it decides what enters the prompt.

Chapter 9 is the agent loop. The model returns text or tool calls, the dispatcher runs native tools and these MCP tools, and the results come back as messages. A cancellation token and a budget sit on that loop, and a small saved eval set starts there.
