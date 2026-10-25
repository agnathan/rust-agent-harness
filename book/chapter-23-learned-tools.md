# Chapter 23: Learned Tools

A task that always ends on `read_file` should be offered `read_file` before the model starts searching the catalog. Hermes, at the commit this chapter cites, has no scorer that ranks tools from memory or from past sessions. The offered set there changes through tool search, which chapter 8 already adapted. This chapter adds the score, on the server, where two agents can share one usage log without each of them embedding the index.

The harness still decides what may enter the prompt, and the server decides which names the session is shown. Scoring inside the model call would make every request wait on a ranking, so `refresh_prediction` installs an `Arc` beforehand and `session_tools` only reads that pointer. A test counts `predict_calls` across the read and expects the same number, and the offered set can change before the request, and it stays put for the length of `session_tools`. For a continuously improving harness, use can tighten the catalog without sitting on the turn's latency.

This is the server program, kept separate from the harness, the same line as chapters 5, 7, 8, and 15. The crate is `steps/ch23-learned-tools`, library `gw_learned`. It vendors chapter 15 the way chapter 15 vendored chapters 7 and 8: `catalog`, `session`, and `limits` are modules, with no path dependency on `ch15-limits`. `LearnedGateway` binds no socket, and `mcp-gateway` will re-export a later copy, from chapter 51, under the name `Gateway`. The Hermes pin moves here from `cf71b1aa`, the commit chapters 11 through 22 cite, to `ab3fe9dffb0a`, a later commit of the same repository. The notes below were read against that later commit, and the walk of the crate stays in the order the sections already have.

A composite or a macro is a name that expands to steps. Code mode runs a line script against the sandbox allowlist, capped at 50 calls, and the new limit sits on the tool the script is allowed to call. The score itself is a plain function you can bench without a session, and the installed set is swapped the way chapter 7 swaps a snapshot. Chapter 37 vendors this gateway and adds pools and federation, and chapters 42 and 51 vendor it again. Chapter 24, back on the harness, starts skills: a procedure the agent reads, where a macro is a procedure the server can run. They stay in different crates, so a skill file stays a skill file and a gateway composite stays a composite. Once the server says which names this session gets, the harness keeps chapter 6's append-only reveal and leaves the prefix alone.

`cargo run --locked -p ch23-learned-tools` runs the demo in-process, and `my-harness/tests/ch23.rs` checks that `session_tools` reads the predicted set and leaves the scorer uncalled.

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
> I read `toolsets.py` (the `code_execution` toolset entry, `create_custom_toolset`, `get_toolset_info`) and `tools/code_execution_tool.py` (`SANDBOX_ALLOWED_TOOLS`, `DEFAULT_MAX_TOOL_CALLS`, `generate_hermes_tools_module`, `MAX_STDOUT_BYTES`) on Hermes `main` at `ab3fe9dffb0a`. Chapters 11 through 22 cite `cf71b1aa`. This chapter cites `ab3fe9dffb0a`, a later commit of the same repository.
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

## What this chapter decides

The score and the read are two functions, and `refresh_prediction` is the offline step. `session_tools` is the request path, and a test reads `predict_calls` before and after `session_tools` and the number is the same.

`predict` ranks names. A usage row whose tool is in the catalog and whose task shares a token of three or more characters with the current task adds 1. A mention of the tool name in memory or in the session note adds 3. A name that scores 0 is left out, and the rest sort by score, then by name.

`save_composite` and `save_macro` store a name and a list of steps, and `expand` returns that list. Neither one runs the steps.

`run_code_mode` reads `call tool json` lines, and a tool outside `SANDBOX_ALLOWED` is `NotAllowed`. A tool that is allowed but not in the offer is `NotOffered`, and the 51st call is `TooManyCalls`, and the backend isn't called. A line the parser cannot read is `BadLine`.

`ScriptedModel` counts `complete` calls, but the demo doesn't construct one. There is no API key.

## 23.1 The crate vendors chapter 15

Chapter 15 used to be a path dependency, and a reader who opened only `ch23-learned-tools` still needed `ch15-limits` on disk. This chapter copies the sources in, the same way chapter 15 copied the catalog and the session gateway.

`lib.rs` declares three modules and re-exports the names the rest of the book already calls.

```rust
pub mod catalog;
pub mod limits;
pub mod session;

pub use limits::{Catalog, LimitedGateway, SessionError, SessionView};
```

`catalog` is the chapter 7 gateway: native tools, plugin tools, an in-memory upstream list, and BM25 search. `session` is the chapter 8 view: one session, `search_tools`, `set_task`, `describe`. `limits` is the chapter 15 wrapper: per-tool permits, an idempotent retry, a breaker, and the caps of 50 on `web_search` and `delegate_task`.

`limits.rs` still says `use crate::session::SessionGateway` and `pub use crate::catalog::Gateway as Catalog`, and those paths work because the modules sit at the crate root. Nothing in this file imports `gw_limits`, but `Cargo.toml` doesn't list `ch15-limits`.

`mcp-gateway` depends on this package and does `pub use gw_learned::LearnedGateway as Gateway`, and that type is still here. The re-export of `Catalog`, `LimitedGateway`, `SessionError`, and `SessionView` keeps the names a later server chapter will call when it vendors this crate the same way.

`LimitedGateway::session_tools` is a one-line forward.

```rust
pub fn session_tools(&self, session_id: &str) -> Option<SessionView> {
    self.inner.session_tools(session_id)
}
```

`open_session` and `catalog` forward the same way, and the learned layer calls those three. It doesn't reimplement the permit, the breaker, or the cap of 50. A code-mode script that names `web_search` still has to pass chapter 15's cap when something actually calls the tool. This chapter's cap of 50 is a different counter, and it counts lines in one script. It doesn't increment `web_search`.

> **Rust sidebar: `pub use` and `pub mod`**
>
> `pub mod catalog` makes the module visible to other crates as `gw_learned::catalog`. `mod catalog` without `pub` would keep it private to this crate. I made `catalog`, `limits`, and `session` public so a reader of this step can open the vendored code, and so `pub` items inside them are not dead-code warnings. `pub use limits::SessionView` puts `SessionView` at the crate root. Callers write `gw_learned::SessionView`. They do not have to know it was defined in `session.rs` and re-exported through `limits.rs`.
>
> A path dependency would have been `ch15-limits = { path = "../ch15-limits" }` and `use gw_limits::LimitedGateway`. That compiles, and it means the step is not a snapshot: a later edit to chapter 15 would change this chapter's tree. Vendoring is the rule for the server line from chapter 15 on.

## 23.2 The score is a plain function

`Use` is one row of history, and `PredictInput` borrows the task, the rows, the memory text, the session note, and the catalog. The lifetime `'a` says those borrows last as long as the call, but `predict` doesn't store them.

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

> **Rust sidebar: a lifetime on a struct**
>
> `'a` is a name for "how long these references are valid." `PredictInput<'a>` cannot outlive the strings it points at. The caller builds the catalog `Vec` and the usage `Vec` in the test, then passes `&catalog` and `&usage`. When `predict` returns, the input is gone and the owned `Vec<String>` of names is what remains. I used borrows so the score does not clone the log before it has decided which names to keep.
>
> `predict` walks the catalog once.

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

`tokens` splits on anything that isn't an ASCII letter or digit and drops words shorter than three characters. `read` doesn't count. `note` does. The set is a `BTreeSet`, so the same word twice is one token.

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

The memory bonus is a whole-name match, not a token match, and `contains_name` lowercases the tool name and looks for it as its own word. The character before and after must not be a name character, and a name character is an ASCII letter, a digit, or `_`.

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

`read_file` inside "prefer read_file for notes" matches, but `read` inside `read_file` doesn't, because the next byte is `_`, and `_` is a name character. That is why the underscore is part of the tool name, but I didn't split on it.

The sort is score descending, then name ascending, and `b.0.cmp(&a.0)` flips the integer compare. `then_with` only looks at the name when the scores are equal. Two tools that both score 3 come out in alphabetical order, so the test can name the exact slice.

A zero score is filtered out before the sort, and an empty log, empty memory, and an empty note offer nothing, even when the catalog has four names. The function returns an owned `Vec`, but it doesn't touch a gateway, a session, or a model.

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

> **Rust sidebar: `Arc`, `Mutex`, and `AtomicU64`**
>
> `Arc<Vec<String>>` is a pointer with a count. Cloning the `Arc` bumps the count. It does not clone the names. Dropping a clone decrements the count. The `Vec` is freed when the last clone goes away. I store the `Arc` inside a `Mutex` because a refresh replaces the pointer and a read clones it, and those can happen on different tasks. The mutex is held only for the swap or the clone. The score runs before the lock, in `predict`, which does not touch `self`.
>
> `AtomicU64` and `AtomicBool` are counters that do not need the mutex. `Ordering::SeqCst` is the strictest order. I used it so a test that reads `predict_calls` after `session_tools` sees the value the refresh stored, not a stale register. This crate is not chasing a weaker order.
>
> `BTreeMap` keeps composite names sorted. I do not iterate the map on the request path. `expand` is a lookup. The order matters when a test prints the keys, and it does not matter for `get`.
>
> `refresh_prediction` is the install.

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

The counter moves first, then the score, then the store, then the flag, and a caller that panics inside `predict` has already counted the attempt. Nothing in `predict` panics on ordinary strings, and `lock().expect` panics only if another thread poisoned the mutex by panicking while holding it. I treat that as a bug, not as an empty offer.

The returned `Arc` is a second clone of the same allocation the field now holds. The caller can keep it, and a later refresh allocates a new `Vec` and a new `Arc`. The old one stays alive only while someone still holds a clone.

`offered_arc` is the cheap read.

```rust
pub fn offered_arc(&self) -> Arc<Vec<String>> {
    Arc::clone(&self.offered.lock().expect("offered"))
}
```

It doesn't call `predict`, but it doesn't look at memory. The bench times this path through `session_tools`, which calls `offered_arc` and then copies the names into the view.

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

`?` on `Option` returns `None` when the session was never opened, and the inner view is the chapter 8 list, which is empty until `search_tools` or `set_task` reveals names. If the installed offer is empty, this function leaves that view alone and doesn't touch `pending_change`. An empty prediction means "I haven't installed a set," not "offer zero tools and mark the list changed."

When the offer is non-empty, the view's `tools` field becomes a clone of the names. `SessionView` is a plain struct.

```rust
pub struct SessionView {
    pub session_id: String,
    pub tools: Vec<String>,
    pub list_changed: bool,
}
```

The caller can keep the view, but a later refresh doesn't change a view you already took. That is the point of copying out of the `Arc`. Holding `offered_arc()` is the cheaper read if you only need the names and you are willing to see a later refresh when you clone again.

`swap(false)` writes `false` and returns the previous value, and the first `session_tools` after a refresh sees `list_changed == true`. The next one sees `false`. The names stay. `predict_calls` isn't in this function, and I checked by reading it. The test checks the same thing from the outside.

`catalog` and `open_session` are the two other forwards a caller needs before that read.

```rust
pub fn catalog(&self) -> &Catalog {
    self.inner.catalog()
}

pub fn open_session(&self, session_id: impl Into<String>) {
    self.inner.open_session(session_id);
}
```

`impl Into<String>` means the caller can pass `&str` or `String`, and `open_session("desk")` allocates one `String` inside the session map. I didn't add a second session table on the learned gateway, and the session id has to exist in the inner gateway or `session_tools` returns `None`.

### The tests

`prediction_changes_the_offered_set_and_is_not_on_the_read` opens `desk`, reads an empty view, and checks `list_changed` is false. It then refreshes with two `read_file` rows on the task "read the note" and a session note that names `web_search`. The ranked `Arc` is `["web_search", "read_file"]`. Memory doesn't mention either name, so `web_search` gets 3 from the note and `read_file` gets 2 from the rows. 3 sorts first. `write_file` and `terminal` score 0 and drop out.

The test stores `predict_calls()`, calls `session_tools`, and expects the same number, and it expects `list_changed` on that first read and not on the second. The tool list is the same both times.

`a_zero_score_tool_is_left_out` calls `predict` with an empty log and empty text, and the catalog still has four names. The result is empty, but that test doesn't construct a gateway. The score is a function.

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

A composite is checked first, and a macro is the fallback. The same name in both maps returns the composite. I split the maps because Hermes splits "a toolset that includes other toolsets" from "a custom toolset the user saved." In this crate both are `Vec<String>`. The difference is which map you wrote, and neither map is consulted by `session_tools`. Expanding a name is a separate call, and running the steps would be a later loop, and this chapter doesn't have one.

`or_else` on `Option` runs the closure only when the composite lookup was `None`, but the macro mutex isn't locked when the composite hit. That keeps the two locks from nesting. Nested locks in the other order, on another thread, would be a deadlock if I ever locked macros and then composites. I do not.

The test saves `rename_flow` as `read_file` then `write_file`, and `morning` as `web_search` then `read_file`. `expand("missing")` is `None`. `Option` is the right type, and an error would mean the name was illegal. A missing name is ordinary.

> **Rust sidebar: `Option` and `cloned`**
>
> `get` returns `Option<&Vec<String>>`. `cloned` turns that into `Option<Vec<String>>` by cloning the vector when it is `Some`. The map keeps its own copy. The caller can push to the returned vector without changing the saved steps. I clone because the lock guard dies at the end of the statement, and a reference into the map would not be allowed to escape the lock.

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

`CodeError` is the four ways a script stops, and it derives `PartialEq` so a test can write `assert_eq!(result, Err(CodeError::TooManyCalls))`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeError {
    TooManyCalls,
    NotAllowed(String),
    NotOffered(String),
    BadLine(String),
}
```

> **Rust sidebar: `enum` and `Result`**
>
> An `enum` is one value that is exactly one of its variants. `NotAllowed` carries the tool name. `TooManyCalls` carries nothing. `Result<Vec<String>, CodeError>` is either the bodies the backend returned or one of those variants. The `?` operator in `run_code_mode` returns the error from the function immediately. I used `?` on `ok_or_else` so a bad line does not keep parsing the rest of the script.

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

The parse finishes before any call, and a script of 51 `read_file` lines returns `TooManyCalls` and the closure is never invoked. I checked that by reading the `return` before the `Ok(lines...)` line. The test uses a closure that would count calls, and the count stays 0 because the function never gets there. The closure in the success test counts 1.

`impl FnMut(&str, &str) -> String` is the backend, and `FnMut` means the closure can change its own captured variables, which is how the test increments `calls`. The gateway doesn't care what the closure does, and the demo passes `|_tool, _args| "ok".into()`. No process is spawned, and no socket is opened.

Blank lines and `#` comments are skipped, but a line that doesn't start with `call ` is `BadLine`. A `call` with no argument word is `BadLine`, because `split_once` needs whitespace between the tool and the JSON. The JSON isn't parsed, and it is the rest of the line, trimmed, and handed to the closure as `&str`. A later chapter that wants a real object can parse it, and this chapter's job is the allowlist, the offer, and the cap.

`SANDBOX_ALLOWED.contains(&tool)` works because the constant is `&[&str]` and `tool` is `&str`, and the seven names match Hermes `SANDBOX_ALLOWED_TOOLS` in the order I wrote them. The order isn't load-bearing. Membership is.

`memory` isn't in the list, so it is `NotAllowed` before the offer is consulted. `web_search` is in the list and still `NotOffered` when the view doesn't name it. Both checks exist. The allowlist is the sandbox boundary, and the offer is the session boundary. Hermes intersects the allowlist with the enabled tools and, if that intersection is empty, falls back to the whole allowlist. I didn't take the fallback, and an empty offer means no code-mode call succeeds. That is stricter, and it matches "only tools the session was offered."

The 50 here is `DEFAULT_MAX_TOOL_CALLS`, but it isn't `MAX_WEB_SEARCHES` from chapter 15. A script of 50 `web_search` lines would pass this function and then each real call would still hit the chapter 15 cap if the caller wired the closure to `LimitedGateway`. The demo closure doesn't, and the two caps stay separate so a reader can see which one fired.

`ScriptedModel` is the stand-in I do not call from `predict` or from `session_tools`.

```rust
pub fn complete(&mut self, _prompt: &str) -> String {
    self.calls += 1;
    self.reply.clone()
}
```

`_prompt` starts with an underscore because the body ignores it, and the test constructs one, asserts `calls()` is 0, runs the demo, and only then calls `complete`. The demo's offer doesn't depend on the model, and if a later edit calls `complete` from `refresh_prediction`, that test fails.

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

Memory contains the whole name `read_file`, so the score is 1 from the usage row plus 3 from memory. `web_search` scores 0. The offer is the one name, and the script is one allowed, offered line. The body is `ok`, but the assert in the middle is the same check as the integration test: the read didn't score.

## Tests

Six tests, in `tests/learned.rs`, and they link `gw_learned` and call the gateway. They are not free functions in a crate that never opens a session, except `a_zero_score_tool_is_left_out` and the code-mode cases, which call `predict` or `run_code_mode` directly. The first test is the one that goes through `session_tools`.

The binary is `ch23-learned-tools`, and the bench is `learned`. There is no second bench, and chapter 15's `limits` bench stays in the chapter 15 crate.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch23-learned-tools
```

```text
Starting 6 tests across 3 binaries
     Summary [   0.007s] 6 tests run: 6 passed, 0 skipped
```

Three binaries: the integration-test harness, the library's unit tests (there are none), and the doctests. nextest counts the test binaries it built for the package, and the sentence is the count from this tree.

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
| `apply_offered` | 76.1 ns | the offered set only | copy 32 names into a session view |
| `predict_offline` | 149.64 µs | off the request path | score 1,000 rows against 64 names |

`apply_offered` builds the gateway, opens `desk`, and calls `refresh_prediction` before the bench, and one `session_tools` also runs first, so `list_changed` is already clear. The timed closure is `b.iter`, and inside: `session_tools` with the session id passed through `black_box`. That clones the `Arc` and copies 32 names, but `predict` isn't called.

`predict_offline` builds 64 names and 1,000 usage rows before the bench, and the timed closure is `b.iter`. Inside: `predict` with the input passed through `black_box`, and no gateway, no session, no model.

The offer is `read_file`, and the code-mode body is `ok`.

The means are `mean.point_estimate` from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `23`, and bench ids: `23/apply_offered`, `23/predict_offline`, `21/apply_offered`, `21/predict_offline`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

A mention in memory is worth more than two past uses. The session note in the test names `web_search`, so that tool sorts first even though the log only contains `read_file`. If the note is stale, the offer is stale until something calls `refresh_prediction` again.

`session_tools` copies the names, and holding the `Arc` from `offered_arc` is the cheaper read. The view is a snapshot, but a later refresh doesn't change a view you already took.

An empty offer doesn't clear the chapter 8 list, and if you refresh to an empty `Vec` on purpose, the next `session_tools` still returns whatever `search_tools` revealed. "No prediction yet" and "predict nothing" are the same empty `Arc` today, and install a sentinel only if a later chapter needs to tell them apart.

`contains_name` uses byte indexes after a lowercase conversion, and the tool names in this crate are ASCII. A non-ASCII name would still be a byte search against the lowercased haystack, and I'm not adding Unicode word breaks in this chapter.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The new limit sits on the tool the script is allowed to call.

**Allowlist.** `SANDBOX_ALLOWED` is the seven names from `SANDBOX_ALLOWED_TOOLS`. `memory` is `NotAllowed` before the backend runs.

**Offer.** A name on the allowlist still has to be in the installed offer. `web_search` is `NotOffered` when the view does not list it. Hermes falls back to the whole allowlist when the intersection is empty. This crate does not.

**Cap.** `CODE_MODE_MAX_CALLS` is 50, from `DEFAULT_MAX_TOOL_CALLS`. The 51st line returns before any call. This is not chapter 15's cap of 50 web searches. That cap is still in the vendored `limits` module, under the tool name `web_search`.

**Prediction.** `session_tools` does not score, and it does not run a tool. The score is a background install. `predict_calls` is the counter a test uses to prove the read did not call it.

**Vendored limits.** Permits, the breaker, and the idempotent retry come from chapter 15 unchanged. This chapter does not grow a second breaker.

**Left for later.** This interpreter is not the Docker sandbox (D1, D2). Secret redaction (E1, E2) already lives on the harness line in chapter 20. This server crate does not log the script. The Python `hermes_tools` RPC is not ported. Stdout truncation stays with Hermes `MAX_STDOUT_BYTES`.

## Chapter summary

- **Vendor.** Chapter 15's catalog, session, and limits live in this crate. There is no path dependency. The lib name is `gw_learned`.
- **Offer.** Installed by `refresh_prediction`. Read by `session_tools`. Hermes has no ranker from memory or past sessions, so `predict` is the one this crate adds, and the request path only reads the `Arc`.
- **Rank.** A memory or session mention is 3. A matching use is 1. Zero stays out. Ties break by name.
- **Names.** A composite and a macro expand to steps. They do not run them.
- **Code mode.** `call tool json`, allowlist, offer, and 50 calls. The backend runs only after the script parses.
- **Time.** Applying 32 names measured **76.1 ns**. Scoring 1,000 rows measured **149.64 µs**.

## What you implement

`my-harness/tests/ch23.rs` calls `chapter_23_read_invokes_scorer` and asserts it is false, and the stub in `my-harness/src/milestones.rs` panics with `todo!("chapter 23")` until you fill it in. The finished behavior is `LearnedGateway::session_tools` in this step: it copies the installed names and doesn't call `predict`. Do not paste that function into `my-harness`, and the milestone only needs the boolean the test asserts.

## What's next

Chapter 24 is skills, on the harness line, and it copies chapter 22 and adds a directory of `SKILL.md` files. The index is rendered once into an `Arc`, and the full text loads when the model asks for it.