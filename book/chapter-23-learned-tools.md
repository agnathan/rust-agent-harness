# Chapter 23: Learned Tools

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 8 can reveal a tool after `search_tools` or `set_task`. Chapter 15 can refuse the 51st `web_search` and the 51st `delegate_task`. Neither chapter remembers which names a task used last time. The offered set is whatever the session gateway last revealed. This chapter installs a prediction before the request. `session_tools` reads that set. It does not score anything while it reads.

A composite and a macro are names that expand to steps. They do not run the steps. Code mode runs a short script of `call tool json` lines, and only against tools the session was offered and that sit on a fixed allowlist.

This is the server track. The step crate is `steps/ch23-learned-tools`. The package is `ch23-learned-tools`. The lib name stays `gw_learned`. `mcp-gateway` re-exports `LearnedGateway` as `Gateway`. The crate vendors chapter 15 instead of path-depending on it. It still binds no socket.

| Section | Step | What it adds |
|---|---|---|
| 23.1 | `steps/ch23-learned-tools` | chapter 15's catalog, session, and limits, copied in |
| 23.2 | the same crate | `predict`, off the request path |
| 23.3 | the same crate | `LearnedGateway` and the offered `Arc` |
| 23.4 | the same crate | `session_tools` reads the offer and does not score |
| 23.5 | the same crate | composite and macro names |
| 23.6 | the same crate | code mode, allowlist, offer, cap of 50 |

> **How Hermes does it**
>
> I read `toolsets.py` (the `code_execution` toolset entry, `create_custom_toolset`, `get_toolset_info`) and `tools/code_execution_tool.py` (`SANDBOX_ALLOWED_TOOLS`, `DEFAULT_MAX_TOOL_CALLS`, `generate_hermes_tools_module`, `MAX_STDOUT_BYTES`) on Hermes `main` at `ab3fe9dffb0a`.
>
> `code_execution` is a toolset entry whose description is "Run Python scripts that call tools programmatically (reduces LLM round trips)". Its tool list is `execute_code`. `debugging` includes `web` and `file`, so `get_toolset_info` sets `is_composite` when `includes` is non-empty. `create_custom_toolset` registers a runtime toolset from a name, a description, and lists of tools and includes.
>
> `SANDBOX_ALLOWED_TOOLS` is the frozenset `web_search`, `web_extract`, `read_file`, `write_file`, `search_files`, `patch`, `terminal`. `DEFAULT_MAX_TOOL_CALLS` is 50. `MAX_STDOUT_BYTES` is 50_000. `generate_hermes_tools_module` emits a stub for the intersection of the allowlist and the tools the session has enabled.
>
> I looked for a scorer that ranks tools from memory or past sessions. Hermes does not have one. The offered set changes through tool search, which chapter 8 already adapted.
>
> **Adopted.** The seven allowlist names. The cap of 50. A composite is a name whose steps are other tools. A macro is a named list saved ahead of the request, the same idea as `create_custom_toolset`.
>
> **Adapted.** `refresh_prediction` scores catalog names from a usage log, the memory text, and a session note, then stores an `Arc`. `session_tools` copies that list. It does not call `predict`. The script is lines of `call tool json`, not Python, and there is no `hermes_tools` socket.
>
> **Deferred.** The real interpreter, the UDS and file RPC transports, and the stdout caps (`MAX_STDOUT_BYTES` is 50_000, `MAX_STDERR_BYTES` is 10_000) stay out. A live embedder is still later.

> **Guardrails.** The new limit sits on the tool the script is allowed to call. The prediction is not a model call. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

[personal note: a tool the score kept offering after you had stopped using it]

## What this chapter decides

The score and the read are two functions. `refresh_prediction` is the offline step. `session_tools` is the request path. A test reads `predict_calls` before and after `session_tools` and the number is the same.

`predict` ranks names. A usage row whose tool is in the catalog and whose task shares a token of three or more characters with the current task adds 1. A mention of the tool name in memory or in the session note adds 3. A name that scores 0 is left out. The rest sort by score, then by name.

`save_composite` and `save_macro` store a name and a list of steps. `expand` returns that list. Neither one runs the steps.

`run_code_mode` reads `call tool json` lines. A tool outside `SANDBOX_ALLOWED` is `NotAllowed`. A tool that is allowed but not in the offer is `NotOffered`. The 51st call is `TooManyCalls`, and the backend is not called. A line the parser cannot read is `BadLine`.

`ScriptedModel` counts `complete` calls. The demo does not construct one. There is no API key.

## 23.1 The crate vendors chapter 15

Chapter 15 used to be a path dependency. A reader who opened only `ch23-learned-tools` still needed `ch15-limits` on disk. This chapter copies the sources in, the same way chapter 15 copied the catalog and the session gateway.

`lib.rs` declares three modules and re-exports the names the rest of the book already calls.

```rust
pub mod catalog;
pub mod limits;
pub mod session;

pub use limits::{Catalog, LimitedGateway, SessionError, SessionView};
```

`catalog` is the chapter 7 gateway: native tools, plugin tools, an in-memory upstream list, and BM25 search. `session` is the chapter 8 view: one session, `search_tools`, `set_task`, `describe`. `limits` is the chapter 15 wrapper: per-tool permits, an idempotent retry, a breaker, and the caps of 50 on `web_search` and `delegate_task`.

`limits.rs` still says `use crate::session::SessionGateway` and `pub use crate::catalog::Gateway as Catalog`. Those paths work because the modules sit at the crate root. Nothing in this file imports `gw_limits`. `Cargo.toml` does not list `ch15-limits`.

`mcp-gateway` depends on this package and does `pub use gw_learned::LearnedGateway as Gateway`. That type is still here. The re-export of `Catalog`, `LimitedGateway`, `SessionError`, and `SessionView` keeps the names a later server chapter will call when it vendors this crate the same way.

`LimitedGateway::session_tools` is a one-line forward.

```rust
pub fn session_tools(&self, session_id: &str) -> Option<SessionView> {
    self.inner.session_tools(session_id)
}
```

`open_session` and `catalog` forward the same way. The learned layer calls those three. It does not reimplement the permit, the breaker, or the cap of 50. A code-mode script that names `web_search` still has to pass chapter 15's cap when something actually calls the tool. This chapter's cap of 50 is a different counter. It counts lines in one script. It does not increment `web_search`.

### Rust: `pub use` and `pub mod`

`pub mod catalog` makes the module visible to other crates as `gw_learned::catalog`. `mod catalog` without `pub` would keep it private to this crate. I made `catalog`, `limits`, and `session` public so a reader of this step can open the vendored code, and so `pub` items inside them are not dead-code warnings. `pub use limits::SessionView` puts `SessionView` at the crate root. Callers write `gw_learned::SessionView`. They do not have to know it was defined in `session.rs` and re-exported through `limits.rs`.

A path dependency would have been `ch15-limits = { path = "../ch15-limits" }` and `use gw_limits::LimitedGateway`. That compiles, and it is what the previous draft of this chapter did. It also means the step is not a snapshot. Vendoring is the rule for the server line from chapter 15 on.

## 23.2 The score is a plain function

`Use` is one row of history. `PredictInput` borrows the task, the rows, the memory text, the session note, and the catalog. The lifetime `'a` says those borrows last as long as the call. `predict` does not store them.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Use {
    pub task: String,
    pub tool: String,
}

#[derive(Debug, Clone)]
pub struct PredictInput<'a> {
    pub task: &'a str,
    pub usage: &'a [Use],
    pub memory: &'a str,
    pub session_note: &'a str,
    pub catalog: &'a [String],
}
```

### Rust: a lifetime on a struct

`'a` is a name for "how long these references are valid." `PredictInput<'a>` cannot outlive the strings it points at. The caller builds the catalog `Vec` and the usage `Vec` in the test, then passes `&catalog` and `&usage`. When `predict` returns, the input is gone and the owned `Vec<String>` of names is what remains. I used borrows so the score does not clone the log before it has decided which names to keep.

`predict` walks the catalog once.

```rust
pub fn predict(input: &PredictInput<'_>) -> Vec<String> {
    let task_tokens = tokens(input.task);
    let memory = input.memory.to_ascii_lowercase();
    let note = input.session_note.to_ascii_lowercase();
    let mut scored: Vec<(i32, &str)> = input
        .catalog
        .iter()
        .map(|name| {
            let mut score = 0;
            for row in input.usage {
                if row.tool == *name && shares_token(&task_tokens, &row.task) {
                    score += 1;
                }
            }
            if contains_name(&memory, name) || contains_name(&note, name) {
                score += 3;
            }
            (score, name.as_str())
        })
        .filter(|(score, _)| *score > 0)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
    scored
        .into_iter()
        .map(|(_, name)| name.to_string())
        .collect()
}
```

`tokens` splits on anything that is not an ASCII letter or digit and drops words shorter than three characters. `read` does not count. `note` does. The set is a `BTreeSet`, so the same word twice is one token.

```rust
fn tokens(text: &str) -> BTreeSet<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| word.len() >= 3)
        .map(|word| word.to_ascii_lowercase())
        .collect()
}

fn shares_token(task: &BTreeSet<String>, other: &str) -> bool {
    tokens(other).iter().any(|token| task.contains(token))
}
```

`shares_token` tokenizes the old task again. A row whose task is "read the note" shares `read` and `note` with a current task of the same words. A row whose task is "unrelated" shares nothing, so that row adds 0 even when the tool name matches.

The memory bonus is a whole-name match, not a token match. `contains_name` lowercases the tool name and looks for it as its own word. The character before and after must not be a name character. A name character is an ASCII letter, a digit, or `_`.

```rust
fn contains_name(haystack: &str, name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    haystack.match_indices(&name).any(|(at, _)| {
        let before_ok = at == 0 || !is_name_char(haystack.as_bytes()[at - 1]);
        let after = at + name.len();
        let after_ok = after >= haystack.len() || !is_name_char(haystack.as_bytes()[after]);
        before_ok && after_ok
    })
}

fn is_name_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
```

`read_file` inside "prefer read_file for notes" matches. `read` inside `read_file` does not, because the next byte is `_`, and `_` is a name character. That is why the underscore is part of the tool name. I did not split on it.

The sort is score descending, then name ascending. `b.0.cmp(&a.0)` flips the integer compare. `then_with` only looks at the name when the scores are equal. Two tools that both score 3 come out in alphabetical order, so the test can name the exact slice.

A zero score is filtered out before the sort. An empty log, empty memory, and an empty note offer nothing, even when the catalog has four names. The function returns an owned `Vec`. It does not touch a gateway, a session, or a model.

## 23.3 The gateway stores an Arc

`LearnedGateway` holds the chapter 15 gateway plus the offer.

```rust
pub struct LearnedGateway {
    inner: LimitedGateway,
    offered: Mutex<Arc<Vec<String>>>,
    pending_change: AtomicBool,
    predict_calls: AtomicU64,
    composites: Mutex<BTreeMap<String, Vec<String>>>,
    macros: Mutex<BTreeMap<String, Vec<String>>>,
}
```

`new` starts with an empty offer, a clear change flag, a zero counter, and two empty maps.

```rust
pub fn new() -> Self {
    Self {
        inner: LimitedGateway::new(),
        offered: Mutex::new(Arc::new(Vec::new())),
        pending_change: AtomicBool::new(false),
        predict_calls: AtomicU64::new(0),
        composites: Mutex::new(BTreeMap::new()),
        macros: Mutex::new(BTreeMap::new()),
    }
}
```

### Rust: `Arc`, `Mutex`, and `AtomicU64`

`Arc<Vec<String>>` is a pointer with a count. Cloning the `Arc` bumps the count. It does not clone the names. Dropping a clone decrements the count. The `Vec` is freed when the last clone goes away. I store the `Arc` inside a `Mutex` because a refresh replaces the pointer and a read clones it, and those can happen on different tasks. The mutex is held only for the swap or the clone. The score runs before the lock, in `predict`, which does not touch `self`.

`AtomicU64` and `AtomicBool` are counters that do not need the mutex. `Ordering::SeqCst` is the strictest order. I used it so a test that reads `predict_calls` after `session_tools` sees the value the refresh stored, not a stale register. This crate is not chasing a weaker order.

`BTreeMap` keeps composite names sorted. I do not iterate the map on the request path. `expand` is a lookup. The order matters when a test prints the keys, and it does not matter for `get`.

`refresh_prediction` is the install.

```rust
pub fn refresh_prediction(&self, input: &PredictInput<'_>) -> Arc<Vec<String>> {
    self.predict_calls.fetch_add(1, Ordering::SeqCst);
    let names = predict(input);
    let arc = Arc::new(names);
    *self.offered.lock().expect("offered") = Arc::clone(&arc);
    self.pending_change.store(true, Ordering::SeqCst);
    arc
}
```

The counter moves first, then the score, then the store, then the flag. A caller that panics inside `predict` has already counted the attempt. Nothing in `predict` panics on ordinary strings. `lock().expect` panics only if another thread poisoned the mutex by panicking while holding it. I treat that as a bug, not as an empty offer.

The returned `Arc` is a second clone of the same allocation the field now holds. The caller can keep it. A later refresh allocates a new `Vec` and a new `Arc`. The old one stays alive only while someone still holds a clone.

`offered_arc` is the cheap read.

```rust
pub fn offered_arc(&self) -> Arc<Vec<String>> {
    Arc::clone(&self.offered.lock().expect("offered"))
}
```

It does not call `predict`. It does not look at memory. The bench times this path through `session_tools`, which calls `offered_arc` and then copies the names into the view.

## 23.4 `session_tools` reads and does not score

```rust
pub fn session_tools(&self, session_id: &str) -> Option<SessionView> {
    let mut view = self.inner.session_tools(session_id)?;
    let offered = self.offered_arc();
    if !offered.is_empty() {
        view.tools = offered.as_ref().clone();
        view.list_changed = self.pending_change.swap(false, Ordering::SeqCst);
    }
    Some(view)
}
```

`?` on `Option` returns `None` when the session was never opened. The inner view is the chapter 8 list, which is empty until `search_tools` or `set_task` reveals names. If the installed offer is empty, this function leaves that view alone and does not touch `pending_change`. An empty prediction means "I have not installed a set," not "offer zero tools and mark the list changed."

When the offer is non-empty, the view's `tools` field becomes a clone of the names. `SessionView` is a plain struct.

```rust
pub struct SessionView {
    pub session_id: String,
    pub tools: Vec<String>,
    pub list_changed: bool,
}
```

The caller can keep the view. A later refresh does not change a view you already took. That is the point of copying out of the `Arc`. Holding `offered_arc()` is the cheaper read if you only need the names and you are willing to see a later refresh when you clone again.

`swap(false)` writes `false` and returns the previous value. The first `session_tools` after a refresh sees `list_changed == true`. The next one sees `false`. The names stay. `predict_calls` is not in this function. I checked by reading it. The test checks the same thing from the outside.

`catalog` and `open_session` are the two other forwards a caller needs before that read.

```rust
pub fn catalog(&self) -> &Catalog {
    self.inner.catalog()
}

pub fn open_session(&self, session_id: impl Into<String>) {
    self.inner.open_session(session_id);
}
```

`impl Into<String>` means the caller can pass `&str` or `String`. `open_session("desk")` allocates one `String` inside the session map. I did not add a second session table on the learned gateway. The session id has to exist in the inner gateway or `session_tools` returns `None`.

### The tests

`prediction_changes_the_offered_set_and_is_not_on_the_read` opens `desk`, reads an empty view, and checks `list_changed` is false. It then refreshes with two `read_file` rows on the task "read the note" and a session note that names `web_search`. The ranked `Arc` is `["web_search", "read_file"]`. Memory does not mention either name, so `web_search` gets 3 from the note and `read_file` gets 2 from the rows. 3 sorts first. `write_file` and `terminal` score 0 and drop out.

The test stores `predict_calls()`, calls `session_tools`, and expects the same number. It expects `list_changed` on that first read and not on the second. The tool list is the same both times.

`a_zero_score_tool_is_left_out` calls `predict` with an empty log and empty text. The catalog still has four names. The result is empty. That test does not construct a gateway. The score is a function.

## 23.5 A name expands to steps

```rust
pub fn save_composite(&self, name: &str, steps: Vec<String>) {
    self.composites
        .lock()
        .expect("composites")
        .insert(name.to_string(), steps);
}

pub fn save_macro(&self, name: &str, steps: Vec<String>) {
    self.macros
        .lock()
        .expect("macros")
        .insert(name.to_string(), steps);
}

pub fn expand(&self, name: &str) -> Option<Vec<String>> {
    self.composites
        .lock()
        .expect("composites")
        .get(name)
        .cloned()
        .or_else(|| self.macros.lock().expect("macros").get(name).cloned())
}
```

A composite is checked first. A macro is the fallback. The same name in both maps returns the composite. I split the maps because Hermes splits "a toolset that includes other toolsets" from "a custom toolset the user saved." In this crate both are `Vec<String>`. The difference is which map you wrote. Neither map is consulted by `session_tools`. Expanding a name is a separate call. Running the steps would be a later loop, and this chapter does not have one.

`or_else` on `Option` runs the closure only when the composite lookup was `None`. The macro mutex is not locked when the composite hit. That keeps the two locks from nesting. Nested locks in the other order, on another thread, would be a deadlock if I ever locked macros and then composites. I do not.

The test saves `rename_flow` as `read_file` then `write_file`, and `morning` as `web_search` then `read_file`. `expand("missing")` is `None`. `Option` is the right type. An error would mean the name was illegal. A missing name is ordinary.

### Rust: `Option` and `cloned`

`get` returns `Option<&Vec<String>>`. `cloned` turns that into `Option<Vec<String>>` by cloning the vector when it is `Some`. The map keeps its own copy. The caller can push to the returned vector without changing the saved steps. I clone because the lock guard dies at the end of the statement, and a reference into the map would not be allowed to escape the lock.

## 23.6 Code mode filters names, then calls

The allowlist and the cap are constants.

```rust
pub const SANDBOX_ALLOWED: &[&str] = &[
    "web_search",
    "web_extract",
    "read_file",
    "write_file",
    "search_files",
    "patch",
    "terminal",
];

pub const CODE_MODE_MAX_CALLS: usize = 50;
```

`CodeError` is the four ways a script stops. It derives `PartialEq` so a test can write `assert_eq!(result, Err(CodeError::TooManyCalls))`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeError {
    TooManyCalls,
    NotAllowed(String),
    NotOffered(String),
    BadLine(String),
}
```

### Rust: `enum` and `Result`

An `enum` is one value that is exactly one of its variants. `NotAllowed` carries the tool name. `TooManyCalls` carries nothing. `Result<Vec<String>, CodeError>` is either the bodies the backend returned or one of those variants. The `?` operator in `run_code_mode` returns the error from the function immediately. I used `?` on `ok_or_else` so a bad line does not keep parsing the rest of the script.

```rust
pub fn run_code_mode(
    &self,
    script: &str,
    offered: &[String],
    mut call: impl FnMut(&str, &str) -> String,
) -> Result<Vec<String>, CodeError> {
    let offered_set: BTreeSet<&str> = offered.iter().map(String::as_str).collect();
    let mut lines = Vec::new();
    for raw in script.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let rest = line
            .strip_prefix("call ")
            .ok_or_else(|| CodeError::BadLine(line.to_string()))?;
        let (tool, args) = rest
            .split_once(char::is_whitespace)
            .ok_or_else(|| CodeError::BadLine(line.to_string()))?;
        if !SANDBOX_ALLOWED.contains(&tool) {
            return Err(CodeError::NotAllowed(tool.to_string()));
        }
        if !offered_set.contains(tool) {
            return Err(CodeError::NotOffered(tool.to_string()));
        }
        lines.push((tool.to_string(), args.trim().to_string()));
        if lines.len() > CODE_MODE_MAX_CALLS {
            return Err(CodeError::TooManyCalls);
        }
    }
    Ok(lines
        .into_iter()
        .map(|(tool, args)| call(&tool, &args))
        .collect())
}
```

The parse finishes before any call. A script of 51 `read_file` lines returns `TooManyCalls` and the closure is never invoked. I checked that by reading the `return` before the `Ok(lines...)` line. The test uses a closure that would count calls, and the count stays 0 because the function never gets there. The closure in the success test counts 1.

`impl FnMut(&str, &str) -> String` is the backend. `FnMut` means the closure can change its own captured variables, which is how the test increments `calls`. The gateway does not care what the closure does. The demo passes `|_tool, _args| "ok".into()`. No process is spawned. No socket is opened.

Blank lines and `#` comments are skipped. A line that does not start with `call ` is `BadLine`. A `call` with no argument word is `BadLine`, because `split_once` needs whitespace between the tool and the JSON. The JSON is not parsed. It is the rest of the line, trimmed, and handed to the closure as `&str`. A later chapter that wants a real object can parse it. This chapter's job is the allowlist, the offer, and the cap.

`SANDBOX_ALLOWED.contains(&tool)` works because the constant is `&[&str]` and `tool` is `&str`. The seven names match Hermes `SANDBOX_ALLOWED_TOOLS` in the order I wrote them. The order is not load-bearing. Membership is.

`memory` is not in the list, so it is `NotAllowed` before the offer is consulted. `web_search` is in the list and still `NotOffered` when the view does not name it. Both checks exist. The allowlist is the sandbox boundary. The offer is the session boundary. Hermes intersects the allowlist with the enabled tools and, if that intersection is empty, falls back to the whole allowlist. I did not take the fallback. An empty offer means no code-mode call succeeds. That is stricter, and it matches "only tools the session was offered."

The 50 here is `DEFAULT_MAX_TOOL_CALLS`. It is not `MAX_WEB_SEARCHES` from chapter 15. A script of 50 `web_search` lines would pass this function and then each real call would still hit the chapter 15 cap if the caller wired the closure to `LimitedGateway`. The demo closure does not. The two caps stay separate so a reader can see which one fired.

`ScriptedModel` is the stand-in I do not call from `predict` or from `session_tools`.

```rust
pub fn complete(&mut self, _prompt: &str) -> String {
    self.calls += 1;
    self.reply.clone()
}
```

`_prompt` starts with an underscore because the body ignores it. The test constructs one, asserts `calls()` is 0, runs the demo, and only then calls `complete`. The demo's offer does not depend on the model. If a later edit calls `complete` from `refresh_prediction`, that test fails.

`demo` is the milestone the binary prints.

```rust
pub fn demo() -> (String, String) {
    let gateway = LearnedGateway::new();
    gateway.open_session("desk");
    let catalog = vec!["read_file".to_string(), "web_search".to_string()];
    let usage = vec![Use {
        task: "read the note".into(),
        tool: "read_file".into(),
    }];
    let input = PredictInput {
        task: "read the note",
        usage: &usage,
        memory: "prefer read_file for notes",
        session_note: "",
        catalog: &catalog,
    };
    gateway.refresh_prediction(&input);
    let before = gateway.predict_calls();
    let view = gateway.session_tools("desk").expect("session");
    let after = gateway.predict_calls();
    assert_eq!(before, after);
    let offered = view.tools.join(",");
    let bodies = gateway
        .run_code_mode(
            "call read_file {\"path\":\"note\"}\n",
            &view.tools,
            |_tool, _args| "ok".into(),
        )
        .expect("code");
    (offered, bodies.join(","))
}
```

Memory contains the whole name `read_file`, so the score is 1 from the usage row plus 3 from memory. `web_search` scores 0. The offer is the one name. The script is one allowed, offered line. The body is `ok`. The assert in the middle is the same check as the integration test: the read did not score.

## Tests

Six tests, in `tests/learned.rs`. They link `gw_learned` and call the gateway. They are not free functions in a crate that never opens a session, except `a_zero_score_tool_is_left_out` and the code-mode cases, which call `predict` or `run_code_mode` directly. The first test is the one that goes through `session_tools`.

The binary is `ch23-learned-tools`. The bench is `learned`. There is no second bench. Chapter 15's `limits` bench stays in the chapter 15 crate.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch23-learned-tools
```

```text
Starting 6 tests across 3 binaries
     Summary [   0.007s] 6 tests run: 6 passed, 0 skipped
```

Three binaries: the integration-test harness, the library's unit tests (there are none), and the doctests. nextest counts the test binaries it built for the package. The sentence is the count from this tree.

```bash
cargo run --locked -p ch23-learned-tools
```

```text
offered: read_file
code: ok
```

The binary opens one session, refreshes from one usage row and a memory sentence, and runs one code-mode line against a closure. No key. No socket.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `apply_offered` | MEAN_23_apply_offered | the offered set only | copy 32 names into a session view |
| `predict_offline` | MEAN_23_predict_offline | off the request path | score 1,000 rows against 64 names |

`apply_offered` builds the gateway, opens `desk`, and calls `refresh_prediction` before the bench. One `session_tools` also runs first, so `list_changed` is already clear. The timed closure is `b.iter`. Inside: `session_tools` with the session id passed through `black_box`. That clones the `Arc` and copies 32 names. `predict` is not called.

`predict_offline` builds 64 names and 1,000 usage rows before the bench. The timed closure is `b.iter`. Inside: `predict` with the input passed through `black_box`. No gateway, no session, no model.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, run BENCH_RUN_URL. The printed Criterion `time:` line is the slope, not the mean. The ids stay `23/apply_offered` and `23/predict_offline`. The old ids `21/apply_offered` and `21/predict_offline` are aliases in `benchmarks/descriptions.json`. I did not rewrite gh-pages history.

### Milestone demo

The `cargo run` above. The offer is `read_file`. The code-mode body is `ok`.

## Pitfalls

A mention in memory is worth more than two past uses. The session note in the test names `web_search`, so that tool sorts first even though the log only contains `read_file`. If the note is stale, the offer is stale until something calls `refresh_prediction` again.

`session_tools` copies the names. Holding the `Arc` from `offered_arc` is the cheaper read. The view is a snapshot. A later refresh does not change a view you already took.

An empty offer does not clear the chapter 8 list. If you refresh to an empty `Vec` on purpose, the next `session_tools` still returns whatever `search_tools` revealed. "No prediction yet" and "predict nothing" are the same empty `Arc` today. Install a sentinel only if a later chapter needs to tell them apart.

Code mode is not a sandbox. The allowlist is a filter on names. The backend in the demo is a closure in this process. Chapter 13 is the boundary that runs code outside the harness. `MAX_STDOUT_BYTES` is not applied here. A closure that returns a megabyte of text will hand that string back.

The underscore is part of the tool name. `read_file` matches as one name, not as the two words `read` and `file`. A memory sentence that says `read` does not award the 3 points to `read_file`.

`contains_name` uses byte indexes after a lowercase conversion. The tool names in this crate are ASCII. A non-ASCII name would still be a byte search against the lowercased haystack. I am not adding Unicode word breaks in this chapter.

The two caps of 50 are easy to mix up. `CODE_MODE_MAX_CALLS` stops a script. Chapter 15's `MAX_WEB_SEARCHES` stops a tool named `web_search`. Wiring the closure to the inner gateway without thinking about both will surprise you on the 50th line, or on the 51st real search, depending on which path you hit.

`expand` does not check that the steps are on the allowlist. A composite can name `memory`. Code mode will still refuse `memory` if you later try to run that step as a `call` line. The maps are names. The filter is the script.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The new limit sits on the tool the script is allowed to call.

**Allowlist.** `SANDBOX_ALLOWED` is the seven names from `SANDBOX_ALLOWED_TOOLS`. `memory` is `NotAllowed` before the backend runs.

**Offer.** A name on the allowlist still has to be in the installed offer. `web_search` is `NotOffered` when the view does not list it. Hermes falls back to the whole allowlist when the intersection is empty. This crate does not.

**Cap.** `CODE_MODE_MAX_CALLS` is 50, from `DEFAULT_MAX_TOOL_CALLS`. The 51st line returns before any call. This is not chapter 15's cap of 50 web searches. That cap is still in the vendored `limits` module, under the tool name `web_search`.

**Prediction.** `session_tools` does not score, and it does not run a tool. The score is a background install. `predict_calls` is the counter a test uses to prove the read did not call it.

**Vendored limits.** Permits, the breaker, and the idempotent retry come from chapter 15 unchanged. This chapter does not grow a second breaker.

**Left for later.** This interpreter is not the Docker sandbox (D1, D2). Secret redaction (E1, E2) already lives on the harness line in chapter 20. This server crate does not log the script. The Python `hermes_tools` RPC is not ported. Stdout truncation stays with Hermes `MAX_STDOUT_BYTES`.

## Chapter summary

- **Vendor.** Chapter 15's catalog, session, and limits live in this crate. There is no path dependency. The lib name is `gw_learned`.
- **Offer.** Installed by `refresh_prediction`. Read by `session_tools`. The score does not run on that read.
- **Rank.** A memory or session mention is 3. A matching use is 1. Zero stays out. Ties break by name.
- **Names.** A composite and a macro expand to steps. They do not run them.
- **Code mode.** `call tool json`, allowlist, offer, and 50 calls. The backend runs only after the script parses.
- **Time.** Applying 32 names measured **MEAN_23_apply_offered**. Scoring 1,000 rows measured **MEAN_23_predict_offline**.

[personal note: the first time the offer was right without a search]

## What you implement

`my-harness/tests/ch23.rs` calls `chapter_23_read_invokes_scorer` and asserts it is false. The stub in `my-harness/src/milestones.rs` panics with `todo!("chapter 23")` until you fill it in. The finished behavior is `LearnedGateway::session_tools` in this step: it copies the installed names and does not call `predict`. Do not paste that function into `my-harness`. The milestone only needs the boolean the test asserts.

## What's next

Chapter 24 is skills, on the harness line. It copies chapter 22 and adds a directory of `SKILL.md` files. The index is rendered once into an `Arc`. The full text loads when the model asks for it.
