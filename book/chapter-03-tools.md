# Chapter 3: Tools

A conversation that can only talk is still a chat. I want the model to look something up, write something down, and read the result on the next turn, and this chapter is the shape of that, but it stops before any model is asked to choose. The loop that asks is chapter 9. The wire protocol is chapter 4. The catalog of thousands of schemas is chapter 6. What lands here, in three copies (`steps/ch03-1-registry`, `steps/ch03-2-toolsets`, and `steps/ch03-3-dispatch`), is the trait, the gate, and the scheduler those chapters share.

A tool has a name, a description, a JSON Schema for its arguments, and a content result. That's the same shape as an MCP tool, chosen now while every tool is still a function in this process. If a native tool and a remote tool were different types, chapter 9's loop would grow a match arm for each, so one trait means the loop asks for a name and a JSON object and the registry answers. `Tool` carries the schema, a `ToolKind`, and `call`. The registry rejects a second registration of the same name, so a later plugin can't quietly replace a built-in. `Catalog` turns named toolsets on only when an environment map contains the expected value and a check accepts it. The library never reads `std::env` itself, so a test builds the map it wants. The list a later chapter shows the model contains only the tools that passed.

`Dispatcher` runs the calls. Read-only calls in one batch overlap, up to 8 at a time, and a write runs alone. The kind is a field on the tool, not a hand-maintained list of names, so a new read that forgets to join a list can't sneak into the parallel set, and a new write can't either. The bound of 8 is the number the rest of the book shares for in-flight tool work. The session inbox from chapter 2 is a different bound, 64, and it queues turns, while this one queues calls inside a turn. Chapter 11 will put an approval decision in front of this dispatcher. Chapter 4 will wrap an MCP tool in the same trait and the same kind. Chapter 13's sandbox and chapter 17's files and terminal sit on this scheduler rather than inventing another one.

`ask --mock` still prints the scripted reply from chapter 1. Nothing here sends a schema to OpenRouter, which is why the registry tests need no key. The CLI copies still know how to resolve a key, and a missing key is the chapter 1 error.

Rust shows up where the design needs it: a trait is the tool, `serde_json` and a JSON Schema are the arguments, `Arc` shares one tool with the registry and the dispatcher, a semaphore is the bound of 8, and `async` is how eight reads overlap.

The demos are offline. `cargo run -p ch03-3-harness-core --example demo` is the dispatcher. `cargo run -p ch03-3-harness-cli -- --mock ask "Say hello"` still takes the chapter 1 path. The tests for this chapter live in the step crates. There is no `my-harness/tests/ch03.rs`.

| Section | Step | What it adds |
|---|---|---|
| 3.1 | `steps/ch03-1-registry` | the `Tool` trait, a JSON Schema, a registry |
| 3.2 | `steps/ch03-2-toolsets` | named toolsets gated on env vars and checks |
| 3.3 | `steps/ch03-3-dispatch` | concurrent reads, exclusive writes, a bound of 8 |

> **How Hermes does it**
>
> `tools/registry.py` stores a `ToolEntry` per tool: name, toolset, JSON schema, handler, and `check_fn`. `get_definitions` walks the names the agent asked for and keeps a tool only when `check_fn()` returns true, or when the tool has no check. Those checks probe things like a terminal backend or a browser, and the results are cached for about 30 seconds so a flaky probe does not strip tools on every turn. A second registration of the same name is rejected, so a plugin cannot quietly replace a built-in.
>
> Parallelism is a separate list. `_PARALLEL_SAFE_TOOLS` in `agent/tool_dispatch_helpers.py` is the read-only set (`read_file`, `web_search`, `skills_list`, and the rest of that frozenset). Anything outside it runs one at a time, unless it is path-scoped. `_PATH_SCOPED_TOOLS` is `read_file`, `write_file`, and `patch`. Two of those may overlap in time when their normalized paths do not overlap. The same path, or a path that contains the other, waits.
>
> This chapter's `Tool` trait is the entry: schema, kind, and `call`. `Catalog` is the toolset gate, and the check looks at an env value the caller passed in, not at a process probe. `Dispatcher` is the pool. The kind is coarser than Hermes: every `Write` runs alone. There is no path check yet, so two writes to different files still wait for each other. I want that rule in the type before I teach the dispatcher about paths.

## What this chapter decides

Four choices, before any of the code.

The tool shape is MCP's shape now, while every tool is still a function in this process. Chapter 4 will list tools from another process and call them over stdio or HTTP. If the native tool and the remote tool were different types, the agent loop in chapter 9 would grow a match arm for each, so one trait means the loop asks for a name and a JSON object and the registry answers.

`ask --mock` still prints the scripted reply from chapter 1, and nothing here sends a tool schema to OpenRouter. That split is what lets the registry tests run with no key and no wiremock. The CLI copies still know how to resolve an OpenRouter key, which is why each milestone includes the missing-key line, and why that line is an error from chapter 1 rather than a tool result.

The gate is data. `Catalog` takes an `EnvMap`. The library never calls `std::env::var`. A test builds the map it wants, and the CLI is the only place that reads `HARNESS_ECHO` and `HARNESS_NOTES`. Hermes's `check_fn` is a probe that can fail because Docker is busy. Caching that probe for 30 seconds is the right call over there. Caching an env string would hide a change the test just made, so this check runs every time `available` is called.

The scheduler's policy is a field on the tool, `ToolKind`, not a list of names. `_PARALLEL_SAFE_TOOLS` is a frozenset Hermes maintains by hand. A new read tool that is left off that list runs alone, and a new write that is added by mistake runs beside other calls. A field set by the `Tool` impl travels with the `Arc`, including an MCP tool chapter 4 will wrap. The cost of that simplicity is the path rule Hermes already has: `_PATH_SCOPED_TOOLS` lets `write_file` and `patch` overlap when the paths do not. This chapter's `Write` never overlaps anything. Two writes to different files wait. I would rather start strict and add a path set when a real file tool exists than start with a name list I have to keep in sync.

The bound is 8 because that is the number the rest of the book shares. `docs/performance.md` says native tools and MCP calls share one scheduler, and the default is 8 in flight. The session inbox from chapter 2 is a different bound, 64, and it queues turns. This one queues calls inside a turn.

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

The rest of the shape is small on purpose. `ToolSchema` is the three fields the model will eventually see. `ToolContent` is one text block. `ToolResult` is a list of those blocks plus `is_error`, which is how a tool reports a failure inside a successful return. MCP does the same split: the transport succeeded, and the tool says the operation did not. `ToolError` is the other path. The harness could not run the tool at all. A dispatcher task has to move that error out of the task, so the enum is `Clone`.

```rust
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

pub struct ToolResult {
    pub content: Vec<ToolContent>,
    pub is_error: bool,
}

pub enum ToolError {
    Unknown { name: String },
    Duplicate { name: String },
    InvalidArgs { name: String, reason: String },
    Failed { message: String },
}
```

`Failed` is the variant a tool uses when the work itself breaks: a missing file, a remote 500, a parse error inside the handler. Echo never returns it. `is_error: true` on a `ToolResult` is the milder form, a result the model should read. Chapter 9 is what turns either one into a `tool` message. This chapter only makes both representable, so a caller does not have to invent a third channel.

`check_arguments` is the whole validator. If the schema has no `required` array, every argument object is accepted, including one the tool will ignore. If `required` is present, the arguments must be a JSON object, and every string in that array must be a key. A required entry that is not a string is skipped. Types, `additionalProperties`, enums, and nested objects are stored in the schema and not enforced. `EchoTool` then reads `text` with `as_str`. A number under that key passes the required check and becomes an empty string, because `as_str` returns none and the call uses `unwrap_or("")`. That is a hole. The tests cover the missing key and the non-object. They do not cover a present key of the wrong type, and I am leaving the hole visible rather than pretending `required` is a schema validator.

`check_arguments` is short enough to read straight through. The early `Ok(())` is the "this schema names no required keys" path. The second early return is the "you passed a string or an array" path. The loop only cares that the key exists. It does not look at the value.

```rust
pub fn check_arguments(schema: &ToolSchema, arguments: &Value) -> Result<(), ToolError> {
    let Some(required) = schema
        .input_schema
        .get("required")
        .and_then(Value::as_array)
    else {
        return Ok(());
    };
    let Some(object) = arguments.as_object() else {
        return Err(ToolError::InvalidArgs {
            name: schema.name.clone(),
            reason: "arguments must be a JSON object".into(),
        });
    };
    for key in required {
        let Some(key) = key.as_str() else {
            continue;
        };
        if !object.contains_key(key) {
            return Err(ToolError::InvalidArgs {
                name: schema.name.clone(),
                reason: format!("missing {key}"),
            });
        }
    }
    Ok(())
}
```

`EchoTool` is the first real tool. The schema is a unit struct's associated function, so the registry and the call share one definition. `kind` is `Read`. `call` checks, then pulls `text`.

```rust
impl EchoTool {
    pub fn schema_value() -> ToolSchema {
        ToolSchema {
            name: "echo".into(),
            description: "Return the text argument unchanged.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "text": { "type": "string" } },
                "required": ["text"],
                "additionalProperties": false
            }),
        }
    }
}

#[async_trait]
impl Tool for EchoTool {
    fn schema(&self) -> ToolSchema {
        Self::schema_value()
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
        let schema = self.schema();
        check_arguments(&schema, &arguments)?;
        let text = arguments
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        Ok(ToolResult::text(text))
    }
}
```

`schema()` builds a new `ToolSchema` on every call. Echo's schema is a handful of strings, and the 13.96 ns lookup does not call `schema()`. A tool with a large schema would want to store it. Echo does not, because a stored schema would be a second copy to keep identical to `schema_value`.

`StaticTool` is a named stub for filling a registry in tests and benches. It has an empty `properties` object, so `check_arguments` accepts anything, and the reply is the tool's own name. `StaticTool::arc` wraps that in `Arc<dyn Tool>`, which is the type `register` takes. `LatencyTool` arrives in section 3.3. It sleeps, then records `(start, end)` behind a `Mutex` so a test can see whether two calls overlapped. The mutex is the standard-library one, so the tool stays `Sync`, and the lock is taken only to push the span. The sleep happens outside the lock.

There is also a blanket impl:

```rust
#[async_trait]
impl Tool for Arc<dyn Tool> {
    fn schema(&self) -> ToolSchema {
        (**self).schema()
    }

    fn kind(&self) -> ToolKind {
        (**self).kind()
    }

    async fn call(&self, arguments: Value) -> Result<ToolResult, ToolError> {
        (**self).call(arguments).await
    }
}
```

`Arc<dyn Tool>` is a `Tool`. A function that takes `impl Tool` can take the handle the registry stores, and `call` on the handle forwards to the concrete tool. The double deref is the `Arc`, then the trait object. Cloning the `Arc` shares the tool. It does not clone the handler.

The registry is a `HashMap` from name to `Arc<dyn Tool>`. I picked a hash map over a `Vec` because dispatch looks up by the name the model will send, and a linear scan would show up once chapter 6 puts thousands of tools behind one registry. I picked it over an enum of every concrete tool because chapter 4 adds tools whose types do not exist in this crate.

```rust
pub fn register(&mut self, tool: Arc<dyn Tool>) -> Result<(), ToolError> {
    let name = tool.schema().name;
    if self.tools.contains_key(&name) {
        return Err(ToolError::Duplicate { name });
    }
    self.tools.insert(name, tool);
    Ok(())
}

pub fn schemas(&self) -> Vec<ToolSchema> {
    let mut schemas: Vec<ToolSchema> = self.tools.values().map(|tool| tool.schema()).collect();
    schemas.sort_by(|left, right| left.name.cmp(&right.name));
    schemas
}
```

`register` reads the name from `schema()`, and refuses a second tool with the same name. The first one stays. Hermes does the same rejection so a plugin cannot overwrite a built-in. The name is owned twice: once as the map key, once inside the schema. That is deliberate. `get` borrows the key the caller passed and does not ask the tool for its schema again. `schemas` clones every schema, then sorts by name. HashMap iteration order is not the order the model should see, and a later chapter's prompt has to be stable. Name order is that stability. The lookup bench does not pay for the sort. It calls `get("tool-25")` and then `kind()` on a 50-tool map built before the timer starts. The names are `tool-00` through `tool-49`, zero-padded so name order and numeric order match.

> **Rust sidebar: `Arc`**
>
> - `Arc::clone` bumps a counter.
> - They share one `EchoTool`.
> - `Arc<dyn Tool>` needs `Tool: Send + Sync`, which the trait already requires. A tool that kept a `RefCell` could not be in this map.
> - `Option`, `Result`, enums, and `Arc` are taught in [chapter 1](chapter-01-first-contact.md).

### The registry tests

`cargo test -p ch03-1-harness-core --test registry` is six tests, and together they protect the schema shape, the refusal, and the order the model will see. `echo_schema_is_mcp_shaped` checks the name, a non-empty description, `type: object`, `required[0] == "text"`, and `ToolKind::Read`. `echo_returns_the_text_argument` calls echo with `{"text":"hello"}` and expects one content block, `hello`, and `is_error == false`. `echo_rejects_a_missing_required_field` calls echo with `{}` and expects `InvalidArgs { name: "echo", reason: "missing text" }`. `arguments_that_are_not_an_object_fail_when_the_schema_requires_keys` passes the string `"nope"`, and the error text contains `JSON object`. The one that surprised me is `registry_lookup_and_sorted_schemas`: it registers `zeta` (read), `EchoTool`, and `alpha` (write), `get("missing")` is none, `alpha`'s kind is `Write`, and `schemas()` comes back `alpha`, `echo`, `zeta`, which is name order rather than insertion order. I had assumed the map would remember who registered first. `a_second_tool_with_the_same_name_is_refused` registers `echo` twice, the second `register` is `Duplicate`, and `len()` stays 1.

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

The bench looks up `tool-25` in a registry of 50 tools. The mean is the point estimate from `target/criterion/registry_lookup/new/estimates.json`.

```text
registry_lookup         time:   [13.845 ns 13.901 ns 14.012 ns]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `registry_lookup` | 13.96 ns | < 1 µs | [bench host](appendix-benchmark-method.md) |

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

This copy of the CLI still speaks OpenRouter. With the key unset it stops on the chapter 1 error:

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

`EnvMap` is a `HashMap<String, String>` with `insert` and `get`. A missing key and a stored empty string are different: `get` returns `None` for the first and `Some("")` for the second, and `available` rejects both. Hermes's `check_fn` is a zero-argument probe (`check_terminal_requirements` and the browser checks) whose result is cached. The check here is `Fn(&str) -> bool` on the value the caller already resolved. There is no TTL, because the value is data, not a socket probe. If a later chapter needs a probe, it belongs in the closure the CLI builds, and the library still only sees the bool.

```rust
pub fn available(&self, env: &EnvMap) -> Result<ToolRegistry, ToolError> {
    let mut registry = ToolRegistry::new();
    for toolset in &self.sets {
        let Some(value) = env.get(&toolset.env_var) else {
            continue;
        };
        if value.is_empty() || !(toolset.check)(value) {
            continue;
        }
        for tool in &toolset.tools {
            registry.register(Arc::clone(tool))?;
        }
    }
    Ok(registry)
}
```

The `?` on `register` means one duplicate name fails the whole catalog. I would rather refuse to start the turn than offer two tools called `echo` and let the model guess which handler runs. The tools that already passed stay unregistered, because the `ToolRegistry` under construction is dropped with the error.

`gated` stores a check of `|_| true`. `with_check` replaces that box. The notes set in the demo only accepts `"1"`. Time accepts any non-empty value, so `"yes"` turns it on. The tool list inside a set is `Vec<Arc<dyn Tool>>`, cloned into the registry with `Arc::clone` when the set passes. A hidden set does not clone its arcs into the result. The tools still exist on the `Toolset`, which is what makes a second `available` call with a different map cheap to rebuild.

### The toolset tests

Four tests share one catalog: `time` holds `clock` and watches `HARNESS_TIME`, and `notes` holds `note` (a write) and watches `HARNESS_NOTES` with the `"1"` check. `only_toolsets_that_pass_are_listed` sets time to `"1"` and notes to `"0"`, and the schemas are just `clock` because the notes check rejected `"0"`. `an_empty_env_var_hides_the_toolset` sets time to `""` and notes to `"1"`, so the schemas are just `note`. `both_sets_pass` sets time to `"yes"` and notes to `"1"`, and the registry contains `clock` and `note`. `a_duplicate_name_across_passing_sets_is_an_error` builds two sets that both register `echo` and both pass, and `available` returns an error whose text contains `duplicate`.

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

The bench builds the available list for one set of 50 tools whose env var is set. `available_schemas` is the whole path: the check, fifty `Arc::clone`s into a new registry, fifty `schema()` calls, and the name sort. The catalog and the env map are built before the timer. Target: under 1 ms. Lookup in section 3.1 was 13.96 ns because it never did this work.

```text
available_tools_50      time:   [21.902 µs 21.945 µs 21.970 µs]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `available_tools_50` | 21.89 µs | < 1 ms | [bench host](appendix-benchmark-method.md) |

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

## 3.3 Concurrent reads, exclusive writes

Hermes, in `_should_parallelize_tool_batch`, asks whether the whole batch is safe. If any name sits outside `_PARALLEL_SAFE_TOOLS` and isn't a path-scoped tool with a non-overlapping path, the whole batch runs one at a time, so a single unsafe tool serializes its neighbors. I want the opposite cut. Eight read-only tools that each wait should finish in about the time of one of them, and a write should be a fence rather than a reason to hold up reads that never touch it. The bound is 8, the same number the performance budget uses for the rest of the book.

`Dispatcher::run` walks the batch. Consecutive reads are one group. A write ends the group, runs by itself, and the next reads start only after it. `run_sequential` is the same calls one at a time, so the bench has something to compare against. The loop peels the batch into maximal read runs, which means eight reads, one write, and eight more reads still overlap inside each read run.

> **Rust sidebar: a semaphore and a JoinSet**
> - `tokio::sync::Semaphore::new(8)` starts with 8 permits. `acquire_owned().await` waits until one is free. Dropping the permit gives it back.
> - `JoinSet` runs those calls as tasks and collects them as they finish. The output vec is filled by the call's original index, so the results stay in request order even when the tasks finish out of order.
> - The permit is held across the `.await` of `tool.call`. That is the point of an async lock: the task can sleep without tying up an OS thread, and the permit still counts.
> - A write does not take a permit from that group. It runs on the caller's task, alone, after the previous group has been joined.

`execute` is the whole policy. `parallel_reads` is true for `run` and false for `run_sequential`, and that flag is the only difference between them.

```rust
while index < calls.len() {
    if !parallel_reads || self.is_write(registry, &calls[index]) {
        outputs.push(self.call_one(registry, &calls[index]).await);
        index += 1;
        continue;
    }
    let start = index;
    index += 1;
    while index < calls.len() && !self.is_write(registry, &calls[index]) {
        index += 1;
    }
    let mut grouped = self.call_reads(registry, &calls[start..index]).await;
    outputs.append(&mut grouped);
}
```

A write in the middle splits the batch into three groups: the reads before it, the write, the reads after it. Two writes in a row are two groups of one. Order of the output vec is the order of the groups, and inside a read group it is the order of the calls, not the order the tasks finish.

`call_reads` is where the semaphore and the `JoinSet` meet. The slice is one wave. The semaphore is created for that wave and dropped when the function returns, which is why a later wave starts with a full set of permits.

```rust
let semaphore = Arc::new(Semaphore::new(self.bound));
let mut tasks = JoinSet::new();
for (offset, call) in calls.iter().cloned().enumerate() {
    let semaphore = Arc::clone(&semaphore);
    let tool = registry.get(&call.name).map(Arc::clone);
    tasks.spawn(async move {
        let _permit = semaphore.acquire_owned().await.expect("semaphore");
        let result = match tool {
            Some(tool) => tool.call(call.arguments).await,
            None => Err(ToolError::Unknown {
                name: call.name.clone(),
            }),
        };
        (offset, CallOutput { name: call.name, result })
    });
}
```

The `expect` on `acquire_owned` fires only if the semaphore is closed. Nothing in this dispatcher closes it. The permit is bound to `_permit` so it stays alive until the task finishes the call. Dropping it at the end of the task is what lets a queued read start.

Path-scoped writes are the piece I am not copying yet. `_extract_parallel_scope_path` reads a `path` argument, expands `~`, and turns a relative path into an absolute one without `resolve()`, because the file may not exist yet. `_paths_overlap` is a prefix compare on the path parts: the same path overlaps, and a directory overlaps a file inside it. Echo has no path. A future `write_file` will. Until that tool exists, `ToolKind::Write` is the fence, and two writes wait even when their paths would have been independent.

Take a batch of four calls, in order: read `a.txt`, read `b.txt`, write `c.txt`, read `d.txt`. Hermes looks at the whole batch. All four names sit in `_PATH_SCOPED_TOOLS`, the paths do not overlap, and the batch may run together, write included. This dispatcher splits on kind. The two reads share a wave of at most 8. The write runs on the caller after that wave joins. The last read starts only then. The write to `c.txt` waited on files it does not touch. That wait is the pitfall I am accepting in this chapter, and it is the reason the kind is on the tool where a later chapter can add a path exception without changing `execute`'s shape. The exception would be "a write whose paths are disjoint may join the wave." It is not in this code.

One more difference on the gate. Hermes `check_fn` returns a bool and takes no arguments. `get_definitions` skips the tool when the cached probe is false, and a transient false within a minute of the last true is ignored so a busy Docker socket does not hide `terminal` mid-session. `Catalog::available` has no such grace. The closure sees the string, and a false hides the set for that call only. The next `available` with a new map sees the new string immediately, which is what the empty-string test relies on. A probe that flickers belongs in the CLI, with whatever grace that probe needs, before the bool is written into the `EnvMap`.

`is_write` is true only when the registry has the name and `kind()` is `Write`. A missing name is therefore grouped with the reads. The unknown error still comes back in that slot. It does not block the other reads, and it does not get the exclusive treatment a write gets. That is the behavior I want for a typo. A tool that mutates state and is tagged `Read` will overlap its neighbors. The kind is the whole policy. Hermes decides the same question from two sets of names, `_PARALLEL_SAFE_TOOLS` and `_PATH_SCOPED_TOOLS`, instead of a field on the tool. I kept the field because a tool from chapter 4's MCP client will not be on either of those Python lists, and the kind travels with the `Arc`.

`call_reads` builds a new `Semaphore` for that slice. The bound is not a process-wide cap. Two overlapping `Dispatcher::run` calls each get `bound` permits. `bound.max(1)` turns a zero into one, so a caller cannot ask for a semaphore with no permits and stall the batch. Each task clones the `Arc<dyn Tool>` before it is spawned, so the registry borrow does not have to live inside the task. The permit is acquired inside the task, which is what lets the ninth read wait while eight are in flight. Results land in a `Vec<Option<_>>` at the call's offset. `flatten` then drops the `Option`. Every slot is filled, because `join_next` runs until the set is empty.

`run_sequential` is the same `execute` with `parallel_reads` false. The bench needs that comparison, and a test can use it without a second scheduler.

The demo uses eight read tools that each sleep 20 ms. That sleep stands in for tool latency, and the run stays inside the process.

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

### The dispatch tests

Four async tests.

`eight_reads_finish_in_about_one_call` uses eight `LatencyTool`s at 30 ms, on a runtime with 8 worker threads. Concurrent time must be under `4 * 30 ms`. Sequential time must be over `6 * 30 ms`. Concurrent must also be less than half of sequential. The factors are loose because `tokio::time::sleep` on a small guest is not a stopwatch. They still fail if the semaphore accidentally serializes the wave, or if `run_sequential` accidentally shares it.

`a_write_does_not_overlap_other_calls` registers `before` (read), `write`, and `after` (read), each at 20 ms. After `run`, the output names are that same order. `before`'s end is at or before `write`'s start, and `write`'s end is at or before `after`'s start. The spans come from the `Mutex` inside each `LatencyTool`, recorded around the sleep.

`results_stay_in_call_order_and_unknown_tools_fail` runs echo, a missing name, then `note` (a write). Slot 0 is `hi`. Slot 1's error text contains `unknown`. Slot 2 is `note`. Echo and the missing name share a read wave. `note` runs after that wave, because it is a write.

`the_bound_limits_a_read_wave` runs four 20 ms reads with `Dispatcher::new(2)`. Elapsed time stays under `5 * 20 ms`, which is enough for two waves and not enough for four in a row. For each span it counts how many spans had already started and not yet ended at that span's start. The maximum of those counts is 2. A bound of 2 lets the next wave's task pass `acquire_owned` at the instant one permit drops, so the count is taken at start instants, not over the whole interval.

The CLI in this step is the first caller that actually reads the process environment.

```rust
fn tool_env() -> EnvMap {
    let mut env = EnvMap::new();
    for key in ["HARNESS_ECHO", "HARNESS_NOTES"] {
        if let Ok(value) = std::env::var(key) {
            env.insert(key, value);
        }
    }
    env
}
```

`demo_catalog` builds two sets, echo behind `HARNESS_ECHO` and note behind `HARNESS_NOTES`. `tool_env` copies those two variables into an `EnvMap` when `std::env::var` succeeds, and leaves them out when it does not. An empty value that *is* set still gets inserted, and `available` then rejects it for being empty. Unset and empty both hide the set. The library stays free of `std::env`, which is why the same `Catalog` type is what the tests construct by hand. The `tools` command prints `schemas()` one name per line, which is why the names come out sorted. The `call` command parses `--arguments` as JSON, builds a one-element batch, and runs it through `Dispatcher::default()`, whose bound is `DISPATCH_BOUND` (8). One call does not exercise the bound. It does exercise the unknown-tool path: a hidden tool is absent from the registry, so the dispatcher returns `Unknown` and the CLI prints that on stderr as `event: error`.

With nothing set, the list is empty. `HARNESS_ECHO=1` shows `echo`. Both variables show `echo` and `note`. Calling `note` without `HARNESS_NOTES` is `unknown tool note`. Calling echo returns the argument:

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

`LatencyTool::call` is the clock the tests and the bench share.

```rust
async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
    let started = Instant::now();
    if !self.delay.is_zero() {
        tokio::time::sleep(self.delay).await;
    }
    let ended = Instant::now();
    self.spans.lock().expect("spans").push((started, ended));
    Ok(ToolResult::text(self.schema.name.clone()))
}
```

The span includes the sleep. The lock does not. A zero delay, which this chapter's bench does not use, records a span and returns. The overlap test would be meaningless if the lock were held across the sleep: every call would queue on the mutex and the spans would never overlap, even when the dispatcher was correct.

The criterion bench sleeps 1 ms per tool, not 20. `iter_batched` with `BatchSize::SmallInput` builds a fresh registry of eight `LatencyTool`s before each sample, so registration is outside the measurement and the tools are not reused across samples. The runtime is a multi-thread tokio runtime with 8 workers, created once. Sample size 10, warm-up 1 s, measurement 2 s. The means are the point estimates.

```text
dispatch_8_reads_concurrent
                        time:   [2.3235 ms 2.3390 ms 2.3620 ms]
dispatch_8_reads_sequential
                        time:   [17.377 ms 17.719 ms 17.953 ms]
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `dispatch_8_reads_concurrent` | 2.362 ms | about one 1 ms call, not eight | [bench host](appendix-benchmark-method.md) |
| `dispatch_8_reads_sequential` | 17.55 ms | about eight 1 ms calls | same |

Tokio's 1 ms sleep is a bit more than 1 ms on this guest, so one wave measured 2.362 ms and eight in a row measured 17.55 ms. That is about 7.4 times, which is the "one call, not eight" budget. The 20 ms demo above is the same comparison with a sleep that dominates the overhead: 21.4 ms versus 170.0 ms.

Where these numbers were taken is in [How the numbers were measured](appendix-benchmark-method.md).

### Milestone demo

Eight reads that each wait 20 ms. This run finished in 21.5 ms together and 168.6 ms one at a time. The block above is another run of the same example, 21.4 ms and 170.0 ms. `--mock ask` prints the scripted reply and leaves these tools alone.

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

## Pitfalls

- **`required` is not validation.** A present key of the wrong JSON type passes `check_arguments`. Echo then turns a non-string `text` into `""`. `additionalProperties: false` is documentation until a real validator exists.
- **The kind is a label.** A write tagged `Read` shares the pool. Hermes will also parallelize a name that sits in `_PARALLEL_SAFE_TOOLS` or in `_PATH_SCOPED_TOOLS` with a non-overlapping path. This dispatcher has no path set, so two writes to different files wait, and two reads of the same file do not.
- **An empty env value hides the set, and so does a missing key.** `"0"` hides notes only because that set's check says so. Time would treat `"0"` as on.
- **The permit outlives the wait, on purpose.** Holding it across `tool.call` is what caps in-flight work. Holding a `Mutex` across that same wait would pin a worker. `LatencyTool` locks only to push the span, before and after the sleep.

## Guardrails in this chapter

Layer L5, the decision of whether a call may run, plus the file-safety sentence this chapter is not claiming. The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Argument check.** `check_arguments` requires every name in the schema's `required` array, and it requires the arguments to be a JSON object. Other schema keywords are stored and not enforced. `EchoTool::call` and `StaticTool::call` run that check. Hermes K2 (`tools/arg_coercion.py`) coerces strings such as `"42"` and `"true"` to the schema type. That coercion is not scheduled in this book.

**Duplicate names.** `ToolRegistry::register` returns `ToolError::Duplicate` when the name is already in the map. One duplicate fails `available`; earlier sets in the vec are not returned as a partial registry.

**Read and write as a scheduling guardrail.** `Dispatcher::is_write` is true only when the registry contains the tool and `kind()` is `ToolKind::Write`. A missing name is not a write. Consecutive reads share a semaphore of `DISPATCH_BOUND` (8). A write runs alone. This decides who shares the pool. It does not look at a path.

**Env-gated toolsets.** `available` registers a set only when its env var is present, non-empty, and its check returns true. The default check accepts any non-empty value. The model is told the schemas of the sets that passed.

**File guards are not here.** Hermes says it in `agent/file_safety.py`: "Every guard here is defense-in-depth, NOT a security boundary: the terminal tool runs as the same OS user." Category C (the write denylist, the read blocks, the rest) is that layer. This registry does not check paths. The boundary Hermes names is the sandbox backend, and that backend is off by default (`terminal.backend: "local"`, catalog D1). Chapter 13 is where this harness builds the sandbox.

The loop's tool-call validator (K1, `agent/turn_tool_validation.py`) is chapter 9. `check_arguments` is the required-key check above.

## Chapter summary

- **`Tool`** is MCP-shaped: name, description, JSON Schema, content result, and a read or write kind. This chapter does not open an MCP connection.
- **`ToolRegistry`** owns the tools behind `Arc<dyn Tool>` and refuses a duplicate name. Lookup of one tool in a 50-tool map was 13.96 ns.
- **`Catalog`** turns toolsets on from an env map plus a check. The available list for 50 tools was 21.89 µs. Tools that fail the gate are not in that list.
- **`Dispatcher`** runs reads together up to 8, and runs a write alone. Eight 1 ms reads took 2.362 ms together and 17.55 ms one at a time.

## What's next

Chapter 4 is the MCP client, built with `rmcp`. It connects over stdio and streamable HTTP, lists tools, and calls them. Those tools land in this same registry, and this dispatcher is the scheduler they share with `echo`.

Chapter 5 is a minimal MCP server in its own process. The skeleton offers no tools. Chapter 6 is what happens when that list is thousands of tools long. It keeps this registry. The 10,000-tool benches still use a synthetic generator. A real `tools/list` enters that cache through `cards_from_listed`. The loop that lets the model choose a tool, and the `tool` messages that carry the result, are chapter 9.
