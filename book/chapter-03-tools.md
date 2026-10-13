# Chapter 3: Tools

*Draft. Bracketed notes like [personal note] are where my own story goes.*

A conversation that can only talk is still just a chat. The next thing the model needs is a way to *act*: look something up, write something down, and get a result back. This chapter adds that, in three steps. It does not add the loop that asks the model which tool to call. That is chapter 6. It also does not speak MCP on the wire. That is chapter 4. Loading thousands of tool schemas on demand, instead of putting every one in the prompt, is chapter 5. What it does add is the shape those later chapters will share.

By the end a tool has:

- a name
- a description
- a JSON Schema for its arguments
- a content result

That is the same shape as an MCP tool. A registry holds them. Named toolsets turn on only when an environment variable is set and a check accepts the value. The list a later chapter will show the model contains only the tools that passed. Read-only calls in one batch run together, up to 8 at a time. A write runs alone.

| Section | Step | What it adds |
|---|---|---|
| 3.1 | `steps/ch03-1-registry` | the `Tool` trait, a JSON Schema, a registry |
| 3.2 | `steps/ch03-2-toolsets` | named toolsets gated on env vars and checks |
| 3.3 | `steps/ch03-3-dispatch` | concurrent reads, exclusive writes, a bound of 8 |

Each step is its own copy, the way chapter 1 was. Section 3.1 still builds after 3.3 exists. The runnable demos are offline. Nothing in this chapter calls OpenRouter.

> **How Hermes does it**
>
> Hermes registers each tool with a JSON schema, a handler, and an availability check (`tools/registry.py`). Toolsets decide which of those tools the model is actually told about. Parallel tool calls run in a pool. This chapter's `Tool` trait is that registration: schema, kind, and `call`. `Catalog` is the toolset gate. `Dispatcher` is the pool, with one extra rule I want in the type's behavior from the start: a write never shares the pool with anything else.

[personal note: the first tool you wanted the harness to have, and why]

## 3.1 A tool and a registry

A tool is a trait. The registry stores `Arc<dyn Tool>`, because the tools in one map are different types and the map has to own them.

> **Rust sidebar: trait objects**
> - `dyn Tool` means "some type that implements `Tool`, and I don't know which at compile time."
> - The compiler no longer knows the size, so the value has to live behind a pointer. `Arc` is the pointer that can be shared and called from more than one task.
> - Methods on the trait have to be usable through that pointer. `async fn call` needs `async_trait` for that, the same way `Provider` did in chapter 1.
> - `schema()` returns an owned `ToolSchema`. A trait object can't return a reference tied to `self` in a way every implementor could share, and the schema is small.

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn schema(&self) -> ToolSchema;
    fn kind(&self) -> ToolKind;
    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError>;
}
```

`ToolKind` is `Read` or `Write`. Section 3.3 is what uses it. The schema's `input_schema` is a `serde_json::Value`. I check the `required` array and nothing else. A full JSON Schema validator can wait.

`EchoTool` is the first real tool. Its schema requires a string `text`. The call returns that string as one content block. `StaticTool` is a named stub for filling a registry in tests.

The registry is a `HashMap` from name to `Arc<dyn Tool>`. `register` refuses a second tool with the same name. `schemas` returns the list in name order, which is the list a later chapter shows the model.

> **Rust sidebar: JSON as a value**
> - `serde_json::json!({...})` builds a `Value` in the shape of the schema.
> - `Value` is an enum: object, array, string, number, and so on. `as_object()` and `as_str()` are the ways back out.
> - I am not parsing the schema into a struct. The schema is data the model will see. Keeping it as JSON means I don't invent a second schema language.

### Build, run, and benchmark

```bash
cargo test -p ch03-1-harness-core --test registry
cargo run -p ch03-1-harness-core --example demo
cargo bench -p ch03-1-harness-core --bench registry -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The demo registers `echo` and calls it. No network:

```text
tools: 1
schema: {"additionalProperties":false,"properties":{"text":{"type":"string"}},"required":["text"],"type":"object"}
echo: hello
```

`cargo nextest run -p ch03-1-harness-core` runs 28 tests: the chapter 2 suite, still green in this copy, plus 6 registry tests.

```text
     Summary [   0.054s] 28 tests run: 28 passed, 0 skipped
```

The bench looks up `tool-25` in a registry of 50 tools. The `time:` line is the slope. The mean is the point estimate from `target/criterion/registry_lookup/new/estimates.json`.

```text
registry_lookup         time:   [13.845 ns 13.901 ns 14.012 ns]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `registry_lookup` | 13.96 ns | < 1 µs | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0, sample size 10 |

### Milestone demo

The example registers `echo` and calls it. No model, no network.

```bash
cargo run -p ch03-1-harness-core --example demo
cargo run -p ch03-1-harness-cli -- --mock ask "Say hello"
```

```text
tools: 1
schema: {"additionalProperties":false,"properties":{"text":{"type":"string"}},"required":["text"],"type":"object"}
echo: hello
```

```text
(mock) I heard you.
event: text-delta (mock) I heard you.
event: done
```

This copy of the CLI still speaks OpenRouter. The optional live command is the same binary without `--mock`. I did not make that call. With the key unset:

```bash
cargo run -p ch03-1-harness-cli -- ask "Say hello"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## 3.2 Toolsets

A registry of everything the process *could* call is not the list the model should see. Hermes hides tools behind toolsets. I do the same. A `Toolset` has a name, the environment variable that enables it, a check, and its tools. `Catalog::available` builds a registry of the sets that passed. A missing variable, an empty value, or a check that returns false leaves that set out.

The library does not read the process environment. Tests pass an `EnvMap`. The CLI is what reads `HARNESS_ECHO` and `HARNESS_NOTES`.

> **Rust sidebar: a boxed closure**
> - `Fn(&str) -> bool` is a trait. The check is "some function that looks at the env value."
> - Different checks are different types, so the toolset stores `Box<dyn Fn(&str) -> bool + Send + Sync>`.
> - `with_check` replaces the default check, which accepts any non-empty value. The notes set in the demo only accepts `"1"`.
> - The box is heap allocated once, when the toolset is built. The hot path calls it. It does not allocate another box per lookup.

The demo turns time on and leaves notes off:

```text
available: 1
tool: clock — clock tool
omitted: note (HARNESS_NOTES is unset)
```

Two passing sets that both register `echo` are an error. The model must not be offered two tools with one name.

### Build, run, and benchmark

```bash
cargo test -p ch03-2-harness-core --test toolset
cargo run -p ch03-2-harness-core --example demo
cargo bench -p ch03-2-harness-core --bench toolset -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The toolset tests are 4, and the whole library copy is 32 tests, all passing.

```text
     Summary [   0.055s] 32 tests run: 32 passed, 0 skipped
```

The bench builds the available list for one set of 50 tools whose env var is set. Target: under 1 ms.

```text
available_tools_50      time:   [21.902 µs 21.945 µs 21.970 µs]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `available_tools_50` | 21.89 µs | < 1 ms | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0, sample size 10 |

### Milestone demo

The example turns the time toolset on and leaves notes off. `--mock ask` is the offline model path in this copy.

```bash
cargo run -p ch03-2-harness-core --example demo
cargo run -p ch03-2-harness-cli -- --mock ask "Say hello"
```

```text
available: 1
tool: clock — clock tool
omitted: note (HARNESS_NOTES is unset)
```

```text
(mock) I heard you.
event: text-delta (mock) I heard you.
event: done
```

The optional live command was not run. With `OPENROUTER_API_KEY` unset:

```bash
cargo run -p ch03-2-harness-cli -- ask "Say hello"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## 3.3 Concurrent reads, exclusive writes

Eight read-only tools that each wait on the network should finish in about the time of one of them, not eight. A write should wait until nothing else is in flight, and the next call should wait until the write returns. The bound is 8, the same number the performance budget uses for the rest of the book.

`Dispatcher::run` walks the batch. Consecutive reads are one group. A write ends the group, runs by itself, and the next reads start only after it. `run_sequential` is the same calls one at a time, so the bench has something to compare against.

> **Rust sidebar: a semaphore and a JoinSet**
> - `tokio::sync::Semaphore::new(8)` starts with 8 permits. `acquire_owned().await` waits until one is free. Dropping the permit gives it back.
> - `JoinSet` runs those calls as tasks and collects them as they finish. The output vec is filled by the call's original index, so the results stay in request order even when the tasks finish out of order.
> - The permit is held across the `.await` of `tool.call`. That is the point of an async lock: the task can sleep without tying up an OS thread, and the permit still counts.
> - A write does not take a permit from that group. It runs on the caller's task, alone, after the previous group has been joined.

The demo uses eight read tools that each sleep 20 ms. That sleep is the stand-in for tool latency. I did not call a model.

```text
read-0
read-1
read-2
read-3
read-4
read-5
read-6
read-7
concurrent_ms: 21.4
sequential_ms: 170.0
```

Twenty milliseconds, eight times, would be 160 ms if they ran one after another. They took 21.4 ms together and 170.0 ms one at a time.

A separate test puts a write between two reads, each sleeping 20 ms, and checks the recorded intervals. The write starts only after the earlier read has finished. The later read starts only after the write has finished. Another test uses a bound of 2 on four reads and checks that at most two are in flight at any start instant.

The CLI in this step can list the gated tools and call one. With nothing set, the list is empty. `HARNESS_ECHO=1` shows `echo`. Both variables show `echo` and `note`. Calling `note` without `HARNESS_NOTES` is `unknown tool note`, on stderr, with `event: error`. Calling echo returns the argument:

```text
$ HARNESS_ECHO=1 cargo run -p ch03-3-harness-cli -- tools
echo
```

```text
$ HARNESS_ECHO=1 cargo run -p ch03-3-harness-cli -- call echo --arguments '{"text":"hello"}'
hello
```

```text
$ cargo run -p ch03-3-harness-cli -- call note
event: error unknown tool note
error: unknown tool note
```

### Build, run, and benchmark

```bash
cargo test -p ch03-3-harness-core --test dispatch
cargo run -p ch03-3-harness-core --example demo
HARNESS_ECHO=1 cargo run -p ch03-3-harness-cli -- tools
cargo bench -p ch03-3-harness-core --bench dispatch -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The dispatch tests are 4. The whole library copy is 36 tests.

```text
     Summary [   0.294s] 36 tests run: 36 passed, 0 skipped
```

The criterion bench sleeps 1 ms per tool, not 20. Setup of the registry is outside the measurement. Eight workers, sample size 10, warm-up 1 s, measurement 2 s. The `time:` line is the slope. The means are the point estimates.

```text
dispatch_8_reads_concurrent
                        time:   [2.3235 ms 2.3390 ms 2.3620 ms]
dispatch_8_reads_sequential
                        time:   [17.377 ms 17.719 ms 17.953 ms]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `dispatch_8_reads_concurrent` | 2.362 ms | about one 1 ms call, not eight | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0, sample size 10 |
| `dispatch_8_reads_sequential` | 17.55 ms | about eight 1 ms calls | same |

Tokio's 1 ms sleep is a bit more than 1 ms on this guest, so one wave measured 2.362 ms and eight in a row measured 17.55 ms. That is about 7.4 times, which is the "one call, not eight" budget. The 20 ms demo above is the same comparison with a sleep that dominates the overhead: 21.4 ms versus 170.0 ms.

CI runs these three steps on Ubuntu, macOS, and Windows. The benches run on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml).

### Milestone demo

Eight reads that each wait 20 ms. This run finished in 21.5 ms together and 168.6 ms one at a time. The earlier 21.4 ms / 170.0 ms figures are the previous run of the same example. `--mock ask` does not call those tools. It only prints the scripted reply.

```bash
cargo run -p ch03-3-harness-core --example demo
HARNESS_ECHO=1 cargo run -p ch03-3-harness-cli -- tools
cargo run -p ch03-3-harness-cli -- --mock ask "Say hello"
```

```text
read-0
read-1
read-2
read-3
read-4
read-5
read-6
read-7
concurrent_ms: 21.5
sequential_ms: 168.6
```

```text
echo
```

```text
(mock) I heard you.
event: text-delta (mock) I heard you.
event: done
```

The optional live command was not run. With the key unset:

```bash
cargo run -p ch03-3-harness-cli -- ask "Say hello"
```

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

## Chapter summary

- **`Tool`** is MCP-shaped: name, description, JSON Schema, content result, and a read or write kind. This chapter does not open an MCP connection.
- **`ToolRegistry`** owns the tools behind `Arc<dyn Tool>` and refuses a duplicate name. Lookup of one tool in a 50-tool map was 13.96 ns.
- **`Catalog`** turns toolsets on from an env map plus a check. The available list for 50 tools was 21.89 µs. Tools that fail the gate are not in that list.
- **`Dispatcher`** runs reads together up to 8, and runs a write alone. Eight 1 ms reads took 2.362 ms together and 17.55 ms one at a time.

[personal note: which of these three rules was the one you almost skipped, and what would have broken later]

## What's next

Chapter 4 is the MCP client, built with `rmcp`. It connects over stdio and streamable HTTP, lists tools, and calls them. Those tools land in this same registry, and this dispatcher is the scheduler they share with `echo`.

Chapter 5 is what happens when that list is thousands of tools long. It keeps this registry, and until chapter 4 exists it fills the catalog with a synthetic generator instead of `tools/list`. The loop that lets the model choose a tool, and the `tool` messages that carry the result, are chapter 6.
