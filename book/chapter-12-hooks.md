# Chapter 12: Hooks and Plugins

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 11 can refuse a command. It cannot refuse an arbitrary tool because a plugin said so, and it cannot rewrite the arguments on the way in. This chapter adds that slot. A hook runs before the tool, and another runs after. A third sees the messages that are about to be sent, by borrow. A memory backend records the turn on a queue. The queue is the trait chapter 21 will fill with files.

The step crate is `steps/ch12-hooks`. It is chapter 11's harness copied forward, plus `hooks`. The package name stays `ch12-hooks`. The library name is `harness_core`. There is no path dependency on chapter 11.

| Section | Step | What it adds |
|---|---|---|
| 12 | `steps/ch12-hooks` | `ToolHook`, `ApiHook`, `MemoryBackend` |

> **How Hermes does it**
>
> I read `agent/tool_executor.py` (`_pre_tool_block`, `_dispatch_authorized_once`) and `hermes_cli/plugins.py` (`_get_pre_tool_call_directive_details`, `_dispatch_pre_tool_call_hooks`, `_resolve_block_from_details`) on Hermes `main` at `cf71b1aa`. `_pre_tool_block` returns `(block_message, final_args)`. The directive scan is block, then approve, then modify. A block wins over an approve even when the approve was registered first. A hook exception does not block. An approve calls `tools/approval.py` `request_tool_approval`. If that gate errors, the call is blocked.
>
> `pre_api_request` is a lifecycle hook. I saw it invoked next to `pre_llm_call` in `tests/hermes_cli/test_shared_metrics_model_attribution.py`. `agent/memory_provider.py` `MemoryProvider.sync_turn` is the non-blocking write of one finished turn. `queue_prefetch` is the background half.
>
> **Adopted.** The `(block, args)` pair. Block over approve. Modify patches merge into the arguments the later hooks see. A hook error is skipped. Escalation that is not approved becomes a block, and a gate failure is `BLOCKED: plugin approval gate failed`. The memory call records the turn and returns.
>
> **Adapted.** `post_tool_call` returns `Option<String>`, a replacement result. I did not port Hermes's post-hook payload. `pre_api_request` takes `&[Message]` so the hook cannot be the thing that clones the conversation. `MemoryBackend::enqueue_turn` is a `Mutex<VecDeque>` capped at 64, not Honcho or Hindsight. A hook may name a toolset to enable. `set_thread_tool_whitelist` in `hermes_cli/plugins.py` narrows tools; this enable is the other direction, and a failed availability check still hides the tool.
>
> **Deferred.** The reviewer that runs after a turn, and `on_pre_compress`, are chapter 26. The file backend is chapter 21. Untrusted-output wrapping is chapter 34 (F4).

[personal note: a hook that fired on a tool you thought was still going to run]

## What this chapter decides

`dispatch_pre` walks the hooks in order.

| Action | Effect |
|---|---|
| `Modify` | Shallow-merge the patch into the arguments |
| `Block` | Return that message now. Later hooks do not run |
| `Approve` | Remember the first one. A later block still wins |
| error | Skip the hook |

`Approve` does not run the tool. The caller passes the reason through `escalation_block`, which calls chapter 11's `check`. Cron mode denies a dangerous command, so the hook's escalate becomes the denial text. `Ask` is also a block in this function: the synchronous path has no human. Chapter 11's `run_call` is the wait. `gate_failed` is the string used when the gate cannot be asked.

`dispatch_post` lets each hook replace the result string. The last `Some` wins.

`run_api_hook` passes the slice through. The test stores `messages.as_ptr()` and checks it against the vec the caller still owns.

`MemoryQueue::enqueue_turn` pushes `user assistant` and drops the oldest job past 64. It does not open a file.

`Registry::available` is true only when the tool's check passed and its toolset was enabled by a hook. Enabling `notes` does not make `secret` available when the check failed.

`run_script` is the fake model. A block ends the list.

## 12.1 The module is `hooks`, and `drive_loop` does not call it

The file is `steps/ch12-hooks/crates/harness-core/src/hooks.rs`. The module comment is the job of the file.

```rust
//! Book chapter 12. Hooks sit in front of a tool call. One of them can block,
//! rewrite the arguments, or escalate into the chapter 11 prompt. The
//! registry check from chapter 3 still decides whether a tool is available.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use crate::approval::{check, Policy, Presence, Session, Verdict};
use serde_json::{Map, Value};
```

`lib.rs` declares `pub mod hooks;`. A test writes `harness_core::hooks::dispatch_pre`. The `pub use` list re-exports the loop, the budget, and chapter 3's `ToolRegistry`. It does not re-export these names.

`registry.rs` has been here since chapter 3. `ToolRegistry` stores `Arc<dyn Tool>`. The `Registry` later in this file stores checks and toolset names. `drive_loop` still takes `&ToolRegistry`.

I searched `agent.rs`. `dispatch_pre`, `dispatch_post`, `run_api_hook`, and `scan_plugin_shape` do not appear. `drive_loop` is where `run_loop`, `run_loop_driven`, and `run_loop_with_inbox` end up.

```rust
#[allow(clippy::too_many_arguments)]
async fn drive_loop(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
    inbox: Option<&SessionInbox>,
    driver: &mut Driver,
) -> TurnOutcome {
```

That `Message` is `crate::message::Message`, with a `Role` enum, tool calls, and a tool-call id. `hooks::Message` is two strings. A function that takes `&[hooks::Message]` cannot take the vec this loop pushes.

A runnable call hits chapter 11, then the dispatcher.

```rust
for call in runnable {
    if let Some(message) =
        crate::approval::deny_tool(&mut approval_session, &call.name, &call.arguments)
    {
        denied.push((call.id.clone(), message));
        continue;
    }
```

`deny_tool` is the approval floor. Survivors go to `dispatcher.run_raced`. Nothing between those lines asks a `ToolHook`. The inherited evals still use `ScriptedProvider` and `ToolRegistry`. The new tests build hooks themselves.

`drive_loop` does not call the hook runner. The chapter tests do. `tests/hooks.rs` calls `dispatch_pre`, `dispatch_post`, `escalation_block`, `run_api_hook`, `run_script`, and `MemoryQueue`. `tests/plugin_guard.rs` calls the plugin scan. `benches/hooks.rs` times three functions. `src/bin/chapter_demo.rs` calls `run_script`.

## 12.2 `HookError` is a message the runner drops

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookError {
    pub message: String,
}
```

> **Rust sidebar: `derive` writes the obvious traits**
> - `HookError` owns one `String`. `String` owns its bytes.
> - `Debug` is `{:?}`. `Clone` is an explicit copy. `PartialEq` and `Eq` are `==`.
> - The struct is not `std::error::Error` and has no `Display`. `dispatch_pre` matches `Err(_)` and drops the value.

The failing-hook test builds `HookError { message: "plugin down".into() }`. The runner never reads `message`. A failure is a skip. The tool result is a `PreAction::Block` message, and a failure is not a block. The field is there so the hook author has a place to put the reason.

## 12.3 `PreAction` is one directive

One hook can ask for several things. Each thing is one variant. The hook returns them in a `Vec`, in the order it means.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreAction {
    Continue,
    Block { message: String },
    Approve { message: String, rule_key: String },
    Modify { patch: Value },
}
```

> **Rust sidebar: an enum is one of several shapes**
> - A struct always has the same fields. An enum is a choice. A `PreAction` is exactly one variant.
> - `Continue` carries nothing. `Block` owns the message. `Approve` owns a message and a `rule_key`. `Modify` owns a `serde_json::Value`: one JSON value.
> - `Clone` is why a test hook can store a `Vec<PreAction>` and return a copy. The runner then moves each action into `match`.

`Continue` and an empty `Vec` are both no-ops. `Block` is the veto: the message becomes the tool result, and later hooks do not run. `Approve` does not run the tool and does not stop the walk. The runner keeps the first pair. `Modify` is a patch merged into the arguments later hooks see. Section 12.7 covers a patch that is not an object.

The block-after-approve test returns `Modify` then `Approve` from the first hook. I apply that vec in order until a block returns. I do not sort it into the Hermes scan order.

## 12.4 `ToolHook` is the plugin

A plugin here is a type that implements `ToolHook`. The runner does not name the concrete struct.

```rust
pub trait ToolHook: Send + Sync {
    fn pre_tool_call(&self, name: &str, args: &Value) -> Result<Vec<PreAction>, HookError> {
        let _ = (name, args);
        Ok(Vec::new())
    }

    /// `Some` replaces the tool result. `None` leaves it.
    fn post_tool_call(&self, name: &str, result: &str) -> Option<String> {
        let _ = (name, result);
        None
    }

    fn enable_toolset(&self) -> Option<&str> {
        None
    }
}
```

> **Rust sidebar: a trait is a list of methods**
> - A trait promises methods. `dyn ToolHook` means some type that implements it, unnamed at the call. `Send` may move to another thread. `Sync` may be shared. `Arc<dyn ToolHook>` needs both.
> - A `dyn` method cannot be generic or return `impl SomeTrait`. These three take `&self` and return owned data or a borrow of `&self`. That is object-safe.

> **Rust sidebar: a default method is the body you get for free**
> - A body on the trait is the default. `let _ = (name, args);` marks the parameters used. The underscore is an intentional ignore.
> - `NopHook` writes nothing, so all three defaults apply. `BlockName` overrides `pre_tool_call` only. Post stays `None`. The toolset stays `None`.

```rust
pub struct NopHook;

impl ToolHook for NopHook {}
```

`NopHook` is a unit struct. No fields. The bench passes `&NopHook` where the parameter is `&dyn ToolHook`. A local reference is enough when the hook does not have to outlive the call. There is no `Arc` on that path.

> **Rust sidebar: `Result` is success or failure**
> - `Result<Vec<PreAction>, HookError>` is `Ok(actions)` or `Err(error)`. The default is `Ok(Vec::new())`. An empty list is still success.
> - `Err` means the plugin could not run. It is not a `Block`. `dispatch_pre` skips that hook. The next hook still runs. `"plugin down"` never appears on the `Dispatch`.

> **Rust sidebar: `Option` is a value or nothing**
> - `Some(string)` from `post_tool_call` replaces the result. `None` leaves it. `Some("notes")` from `enable_toolset` names a set. The default is `None`.
> - There is no `Result` on the way out. A post hook can replace the string. It cannot fail the call.

> **Rust sidebar: a slice is a pointer and a length**
> - `&str` and `&[T]` are a pointer plus a length. `name: &str` borrows the tool name. `args: &Value` borrows the JSON. The caller keeps both.
> - `dispatch_pre` takes `&[Arc<dyn ToolHook>]`. `pre_api_request` takes `&[Message]`. `scan_plugin_shape` takes `&[PluginFile]`.

`pre_tool_call` sees `args` as they stand. An earlier hook's patch is already there. This hook's own patches are merged after it returns, so the next hook sees them.

## 12.5 `Dispatch` is what the walk returns

The walk hands back three facts: whether to stop, what the arguments are now, and whether anyone asked for a human.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dispatch {
    pub block: Option<String>,
    pub args: Value,
    /// `(message, rule_key)` when a hook asked for the human gate and nothing blocked.
    pub approve: Option<(String, String)>,
}
```

`block: None` means no veto. `block: Some(message)` means stop, and `message` is the text. `Option` keeps an empty message distinct from "no block." The tests compare with `as_deref()`, which borrows the `String` as `&str`.

`args` is the JSON after every `Modify` that ran. The caller passed ownership in. The `Dispatch` owns it on the way out.

`approve` is the pair the comment names: message, then rule key. `None` means nobody asked, or a later block wiped the request. `Some(("first", "a"))` is what the second test asserts. I did not give the pair its own struct. Nothing else in the file passes it around except this field and the local inside `dispatch_pre`.

## 12.6 `dispatch_pre` walks hooks until a block

Block wins over approve. Modify patches accumulate until a block. A hook error is ignored, which is the same as "hook failures never block."

```rust
pub fn dispatch_pre(hooks: &[Arc<dyn ToolHook>], name: &str, mut args: Value) -> Dispatch {
    let mut first_approve: Option<(String, String)> = None;
    for hook in hooks {
        let actions = match hook.pre_tool_call(name, &args) {
            Ok(actions) => actions,
            Err(_) => continue,
        };
        for action in actions {
            match action {
                PreAction::Continue => {}
                PreAction::Modify { patch } => merge_args(&mut args, &patch),
                PreAction::Block { message } => {
                    return Dispatch {
                        block: Some(message),
                        args,
                        approve: None,
                    };
                }
                PreAction::Approve { message, rule_key } => {
                    if first_approve.is_none() {
                        first_approve = Some((message, rule_key));
                    }
                }
            }
        }
    }
    Dispatch {
        block: None,
        args,
        approve: first_approve,
    }
}
```

> **Rust sidebar: `Arc<dyn ToolHook>` is a shared hook**
> - `dyn ToolHook` has no fixed size, so the value lives behind a pointer. `Arc` is that pointer plus a reference count. The last drop frees the hook.
> - The argument is a slice of `Arc`s. The function borrows the list. `std::slice::from_ref(&hook)` builds a slice of length 1 without allocating a `Vec`. The demo and `sample_block` use it.
> - `mut args` means this function owns the JSON and may change it. `name` stays a borrow.

> **Rust sidebar: `match` picks one arm**
> - `match` runs the first pattern that fits. The arms have to cover every variant, or the file does not compile.
> - The outer match is on `Result`. `Ok(actions)` binds the vec. `Err(_)` discards the `HookError`. `continue` goes to the next hook. It does not return from the function.
> - The inner match has four arms because `PreAction` has four variants. `return` inside `Block` leaves the function. Later actions in that vec, and later hooks, are not visited.

`args` comes in by value. `run_script` clones first. The unit test passes `json!({})`. `first_approve` starts as `None`. The type is on the `let` because a bare `None` could be any `Option`. Each hook sees `&args` as merged so far.

`Ok` enters the action loop. An empty vec is still `Ok`, so the inner loop can run zero times. That is `NopHook`, and `BlockName` on any name but its own. `Err` skips to the next hook. The test puts a failing hook first, then `"first"` / `"a"`, then `"second"` / `"b"`. `block` is `None`. `approve` is `Some(("first", "a"))`. `"plugin down"` is dropped. Later hooks still run.

`Continue` changes nothing. `Modify` hands the patch to `merge_args`. The copied keys stay in `args`. `Block` returns at once with `approve: None`, so an earlier pair is discarded and later hooks are not called. The test's first hook sets `path` to `notes.md` and approves `"please"` / `"rm"`. The second returns `Block { message: "no" }`. The result is block `"no"`, no approve, and `args["path"] == "notes.md"`. The modify ran before the return.

`Approve` writes the pair only while `first_approve` is `None`. The second approve does not replace the first, and it does not return, so a later block can still win. The bottom return is the only path that keeps an approve. An empty hook slice returns the original `args` with neither a block nor an approve.

## 12.7 `merge_args` is a shallow patch

Only `dispatch_pre` calls this. It is private.

```rust
fn merge_args(args: &mut Value, patch: &Value) {
    let Some(base) = args.as_object_mut() else {
        *args = patch.clone();
        return;
    };
    let Some(extra) = patch.as_object() else {
        return;
    };
    for (key, value) in extra {
        base.insert(key.clone(), value.clone());
    }
}
```

> **Rust sidebar: `let`-`else` leaves on the other shape**
> - `as_object_mut()` is `Some(&mut Map<String, Value>)` when the JSON is an object, and `None` for an array, string, number, bool, or null.
> - `let Some(base) = ... else { ... }` binds `base` on `Some`. The `else` block must `return`, `break`, `continue`, or panic.
> - `*args = patch.clone()` writes through the `&mut Value` and replaces the whole value.

The arguments are not an object. The first `else` clones the patch over `args` and returns. There is no map to insert into.

The arguments are an object and the patch is not. The second `else` returns and `args` stays. A string patch does not wipe a good object.

Both are objects. `insert` adds the key or replaces the value. Both the key and the value are cloned. A nested object is replaced whole. The function does not call itself. That is the shallow merge. The test starts from `{}` and inserts `path`. A key the patch does not mention is left alone. The patch stays borrowed. The clone is what enters `args`.

## 12.8 `dispatch_post` lets the last `Some` win

The tool has already produced a string. Every hook may replace it. Nobody stops the walk.

```rust
pub fn dispatch_post(hooks: &[Arc<dyn ToolHook>], name: &str, mut result: String) -> String {
    for hook in hooks {
        if let Some(next) = hook.post_tool_call(name, &result) {
            result = next;
        }
    }
    result
}
```

`result` is owned and `mut`. `run_script` passes `format!("ran {}", call.name)`, so the first hook on `echo` sees `ran echo`. The test calls `dispatch_post` with `"ran echo"` and one hook that returns `Some("rewritten")`.

`if let Some(next)` assigns on `Some` and skips on `None`. The default method returns `None`, so `NopHook` leaves the string. A `Some` does not return. The next hook is called with `&result` after the assignment, so it sees the replacement. Two `Some` results stack, and the last one wins. A trailing `None` leaves that replacement in place. The test has one hook, so the string is `"rewritten"`.

There is no `Result` here. A post hook cannot skip itself by failing, and it cannot veto. The name is passed so a hook can rewrite one tool and leave another. `run_script` does not call `dispatch_post` when `dispatch.block` is `Some`. The block message is the result, and the post hooks do not see it.

## 12.9 `escalation_block` turns an unapproved escalate into a block

`Dispatch.approve` is a request. This function is what a caller does with it. `run_script` does not call it. The cron test does.

```rust
pub fn escalation_block(
    policy: &Policy,
    session: &mut Session,
    tool: &str,
    reason: &str,
) -> Option<String> {
    let command = format!("{tool} {reason}");
    match check(policy, session, &command) {
        Verdict::Allow { .. } => None,
        Verdict::Deny { message, .. } => Some(message),
        Verdict::Ask { .. } => Some(format!(
            "BLOCKED: plugin approval required for {tool}. The user has NOT consented."
        )),
    }
}
```

`format!("{tool} {reason}")` joins with one space. The test passes `"rm"` and `"-rf ./build"`, so `command` is `"rm -rf ./build"`. `session` is `&mut` because `check` updates the denial count. This function does not read the count afterward.

```rust
pub fn cron_policy() -> Policy {
    Policy {
        presence: Presence::Cron,
        ..Policy::manual()
    }
}
```

`Policy::manual()` is the chapter 11 default: manual mode, cron mode deny, YOLO off, empty lists. `..` copies every field except `presence`, set here to `Presence::Cron`. `check` treats `"rm -rf ./build"` as a dangerous recursive delete. The unattended deny arm returns `Verdict::Deny`. That text contains `BLOCKED` and the sentence that the user has not consented.

`Allow` returns `None`: do not block. This function still does not run the tool. `Deny` returns `Some(message)`, the pipeline's text unchanged. The test accepts `"has NOT consented"` or `"BLOCKED"`. `Ask` ignores the verdict's description and returns `BLOCKED: plugin approval required for {tool}. The user has NOT consented.` `Ask` is a block because this function does not wait. Chapter 11's `run_call` is the real wait. The `..` in each pattern leaves the unused fields unbound.

## 12.10 `gate_failed` is the string when the gate cannot run

A denial is a `Verdict`. A gate that cannot be asked is not one. Hermes blocks that call. This is the string.

```rust
pub fn gate_failed(tool: &str) -> String {
    format!("BLOCKED: plugin approval gate failed for {tool}")
}
```

It does not call `check`. The only input is the tool name. `gate_failed("rm")` is `BLOCKED: plugin approval gate failed for rm`. The test uses `contains("plugin approval gate failed")`. I split it from `escalation_block` so a log can tell "the pipeline said no" apart from "the pipeline could not be run." The adopted line in the Hermes box names this sentence.

## 12.11 `ApiHook` and `run_api_hook` borrow the messages

The third hook sees the messages about to be sent. It does not replace them.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub trait ApiHook: Send + Sync {
    fn pre_api_request(&self, messages: &[Message]);
}

pub struct NopApi;

impl ApiHook for NopApi {
    fn pre_api_request(&self, messages: &[Message]) {
        let _ = messages.len();
    }
}

pub fn run_api_hook(hook: &dyn ApiHook, messages: &[Message]) {
    hook.pre_api_request(messages);
}
```

This `Message` is two owned strings. `crate::message::Message` has a `Role` enum and tool-call fields. `run_api_hook` accepts the one in this file. The bench builds a thousand of them, role `"user"`, content `"hello"`, before the timer.

`ApiHook` has no default, so `NopApi` writes the method. `let _ = messages.len()` marks the parameter used. The return is `()`. The hook cannot hand back a new slice. `run_api_hook` takes `&dyn ApiHook`: one borrowed hook, one call. The vec is not cloned.

The test stores `messages.as_ptr() as usize` and the length, then compares the pointer with the vec the caller still holds. Same address, length 1. The method takes `&self` and the trait requires `Sync`, so the test uses `AtomicUsize`. The library function only forwards the slice.

## 12.12 `TurnRecord` and `MemoryQueue`

The memory hook records one finished turn and returns. It does not open a file. Chapter 21 is the backend that does.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnRecord<'a> {
    pub user: &'a str,
    pub assistant: &'a str,
}

pub trait MemoryBackend: Send + Sync {
    fn name(&self) -> &str;
    fn enqueue_turn(&self, record: &TurnRecord<'_>) -> Result<(), HookError>;
}
```

> **Rust sidebar: `'a` is how long a borrow lasts**
> - `TurnRecord<'a>` contains borrows. Both `&str` fields live at least as long as the struct.
> - A literal is `'static`. The test passes `"hello"` and `"ok"`. A local `String` also works if the record only lives for the `enqueue_turn` call. The type does not demand `'static`.
> - `&TurnRecord<'_>` lets the compiler pick the lifetime. `format!` copies the characters into a `String` the queue owns. After the call returns, the queue does not hold the `&str`.

`MemoryBackend` is the trait chapter 21 will implement with files. `enqueue_turn` returns `Result<(), HookError>`. `()` means success carries no payload. `Err` is there so a backend that can fail has a type. This queue always returns `Ok(())`.

```rust
#[derive(Debug, Default)]
pub struct MemoryQueue {
    jobs: Mutex<VecDeque<String>>,
}

impl MemoryQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.jobs.lock().expect("queue").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl MemoryBackend for MemoryQueue {
    fn name(&self) -> &str {
        "queue"
    }

    fn enqueue_turn(&self, record: &TurnRecord<'_>) -> Result<(), HookError> {
        let mut jobs = self.jobs.lock().expect("queue");
        if jobs.len() == 64 {
            jobs.pop_front();
        }
        jobs.push_back(format!("{} {}", record.user, record.assistant));
        Ok(())
    }
}
```

> **Rust sidebar: `Mutex` is the lock around the deque**
> - `lock()` returns a guard or a poison error. The guard unlocks when it is dropped, at the end of the function that holds it.
> - `expect("queue")` panics if the lock is poisoned. Poison means a thread panicked while holding it. This queue does not try to recover.
> - `enqueue_turn` holds one guard across the length check, the pop, and the push. Two threads cannot both see 63 and both push. `len` locks on its own. `is_empty` calls `len`, so it locks too.

`Default` is an unlocked mutex around an empty `VecDeque`. `new` is `Self::default()`. `push_back` adds at the tail. `pop_front` drops the head without sliding every later job. The cap is the literal `64`, the same number as `INBOX_BOUND`, which this function does not read. At length 64 it pops, then pushes, and the length stays 64. At 63 it only pushes. The comparison is `==`. This method is the only writer, and it holds the lock, so the length does not pass 64.

The test's stored string is `hello ok`. It asserts `len() == 1` and `name() == "queue"`. There is no public pop. Chapter 21 is the copy a caller can read. `name` returns the literal `"queue"`.

## 12.13 `Registry` in this file is not the chapter 3 tool registry

Chapter 3's registry is `ToolRegistry` in `registry.rs`. `drive_loop` takes `&ToolRegistry`. It stores tools. This `Registry` lives in `hooks.rs`. It stores a check bit, the names in each set, and the sets a hook enabled. The tests call this one. The loop does not. This is not the chapter 3 tool registry.

```rust
#[derive(Debug, Clone)]
pub struct Registry {
    checks: HashMap<String, bool>,
    sets: HashMap<String, Vec<String>>,
    enabled: HashSet<String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
            sets: HashMap::new(),
            enabled: HashSet::new(),
        }
    }

    pub fn insert(&mut self, set: impl Into<String>, name: impl Into<String>, check_ok: bool) {
        let set = set.into();
        let name = name.into();
        self.checks.insert(name.clone(), check_ok);
        self.sets.entry(set).or_default().push(name);
    }

    pub fn enable_from(&mut self, hooks: &[Arc<dyn ToolHook>]) {
        for hook in hooks {
            if let Some(set) = hook.enable_toolset() {
                self.enabled.insert(set.to_string());
            }
        }
    }

    pub fn available(&self, name: &str) -> bool {
        if self.checks.get(name).copied() != Some(true) {
            return false;
        }
        self.sets
            .iter()
            .any(|(set, tools)| self.enabled.contains(set) && tools.iter().any(|tool| tool == name))
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
```

> **Rust sidebar: `HashMap` stores a value under a key**
> - `HashMap<String, bool>` is the check bit. `HashMap<String, Vec<String>>` is the tools in a set. Lookup is by key.
> - `insert` on `checks` overwrites the previous bool for that name. One name has one bit, even if two sets list it.
> - `entry(set).or_default()` returns the existing vec, or inserts an empty one and returns that. `push` appends the tool. `HashSet::insert` of a name already enabled changes nothing. `contains` is the question `available` asks.

`impl Into<String>` lets the test pass `"notes"` and `"remember"`. `.into()` allocates from a literal, or moves an existing `String`. `name.clone()` goes into `checks`. The original `name` moves into the set's vec.

`enable_from` does not call `pre_tool_call`. It reads `enable_toolset` only. `to_string()` copies the borrowed name into the set, so the `HashSet` does not borrow the hook after the function returns. `None` skips that hook.

`get` yields `Option<&bool>`. `copied()` makes `Option<bool>` because `bool` is `Copy`. `None` and `Some(false)` both fail `!= Some(true)` and return before the sets are consulted. Enabling a set does not revive a failed check.

When the bit is true, `any` needs a set that is both enabled and lists the name. The test inserts `remember` with `true` and `secret` with `false`, both in `notes`. Before `enable_from`, `remember` is false. After a hook that enables `notes`, `remember` is true and `secret` is false.

`Default` calls `new()`. Chapter 3's `ToolRegistry` still decides availability on the loop. This map is the hook rule: a plugin may turn a set on, and a failed check still hides the tool. Hermes's whitelist narrows. This enable is the other direction.

## 12.14 `BlockName` blocks one tool

The demo needs a hook that stops `rm` and lets any other name through.

```rust
pub struct BlockName {
    pub name: String,
    pub message: String,
}

impl ToolHook for BlockName {
    fn pre_tool_call(&self, name: &str, _args: &Value) -> Result<Vec<PreAction>, HookError> {
        if name == self.name {
            Ok(vec![PreAction::Block {
                message: self.message.clone(),
            }])
        } else {
            Ok(Vec::new())
        }
    }
}
```

Both branches are `Ok`. This hook does not fail.

`name == self.name` compares `&str` with `String` and does not allocate. The true arm returns one `Block`. The message is cloned so the hook keeps its field and the action owns a separate copy. The false arm returns an empty vec, the same effect as the default, written out so the name check is visible. `_args` is unused. The trait requires the parameter. `rm` with any JSON is still `rm`.

`post_tool_call` and `enable_toolset` stay the defaults. The fields are `pub` so another crate can write `BlockName { name: "rm".into(), message: "BLOCKED: hook refused rm".into() }`. `.into()` allocates the `String`.

## 12.15 `run_script` is the fake model

The model in this chapter is a list of calls. No HTTP and no key.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptedCall {
    pub name: String,
    pub args: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallOutcome {
    pub name: String,
    pub result: String,
}

pub fn run_script(hooks: &[Arc<dyn ToolHook>], calls: &[ScriptedCall]) -> Vec<CallOutcome> {
    let mut out = Vec::new();
    for call in calls {
        let dispatch = dispatch_pre(hooks, &call.name, call.args.clone());
        if let Some(message) = dispatch.block {
            out.push(CallOutcome {
                name: call.name.clone(),
                result: message,
            });
            break;
        }
        let result = dispatch_post(hooks, &call.name, format!("ran {}", call.name));
        out.push(CallOutcome {
            name: call.name.clone(),
            result,
        });
    }
    out
}
```

`call` is a reference into the slice, so `args` is cloned to give `dispatch_pre` an owned `Value`. The name is cloned into the outcome for the same reason. The outcome has to own its strings after the function returns.

`dispatch.approve` is not read. An approve-only hook does not stop the script. The function still formats `ran {name}` and runs the post hooks. The escalation test calls `escalation_block` itself, outside this function. `Dispatch` keeps the approve so a caller can see it. `run_script` is not that caller.

On `Some(message)`, the outcome's result is the block text. `break` leaves the `for` loop. Later calls are not dispatched. The test's script is `rm` then `echo`, with `BlockName` aimed at `rm`. `outcomes.len()` is 1, and the result contains `"refused"`. `echo` never becomes an outcome. A `continue` would have skipped the post hooks for `rm` and still run `echo`. A block ends the list.

The other path formats `ran {name}`, runs every post hook, and pushes. The tool body does not run. `ran echo` is a stand-in, not `EchoTool`. An empty `calls` slice returns an empty vec.

The demo binary builds the same `BlockName` inside `Arc::new`, coerces it to `Arc<dyn ToolHook>`, and passes `std::slice::from_ref`. It prints `result: BLOCKED: hook refused rm` and `stopped: 1`. The `assert_eq` of length 1 is in `main`. No key.

## 12.16 `sample_block` is the one-call form of that script

```rust
pub fn object_map(entries: &[(&str, &str)]) -> Value {
    let mut map = Map::new();
    for (key, value) in entries {
        map.insert((*key).to_string(), Value::String((*value).to_string()));
    }
    Value::Object(map)
}

pub fn sample_block() -> String {
    let hook: Arc<dyn ToolHook> = Arc::new(BlockName {
        name: "rm".to_string(),
        message: "BLOCKED: hook refused rm".to_string(),
    });
    let calls = [ScriptedCall {
        name: "rm".to_string(),
        args: object_map(&[("path", "/")]),
    }];
    run_script(std::slice::from_ref(&hook), &calls)
        .pop()
        .expect("one call")
        .result
}
```

The doc comment says this touches the map so a denied escalation still type-checks in the binary. The body does not call `escalation_block`. It runs one scripted `rm` and returns the result string. The demo and the tests do not call `sample_block`. It is the library form of that one blocked call.

`object_map` copies each pair into an owned key and a `Value::String`, then wraps `Value::Object`. `object_map(&[])` is `{}`. `object_map(&[("path", "/")])` is the demo's object, without `json!`. `pop` takes the one outcome. `expect("one call")` panics if the vec is empty. `.result` is `BLOCKED: hook refused rm`. `BlockName` ignores `path`.

## 12.17 Three caps, and the verdict types

A plugin install is the moment a hook enters the process. The structural half of Hermes `tools/plugin_guard.py` is these constants and the two functions under them. The pattern engine stays chapter 25.

```rust
pub const MAX_PLUGIN_FILE_COUNT: usize = 400;
pub const MAX_PLUGIN_TOTAL_BYTES: u64 = 10 * 1024 * 1024;
pub const MAX_PLUGIN_SINGLE_FILE_BYTES: u64 = 1024 * 1024;
```

The file cap is 400, a `usize`, because it counts entries in the slice. The total cap is `10 * 1024 * 1024` bytes, 10 MB. The one-file cap is `1024 * 1024` bytes, 1 MB. Both byte constants are `u64`, and the multiplication is compile-time. The tests add one when they want to step over a cap, so the numbers live here.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginVerdict {
    Safe,
    Caution,
    Dangerous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallDecision {
    Allow,
    Confirm,
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginFile {
    pub path: String,
    pub bytes: u64,
}
```

`PluginVerdict` is the scan. `InstallDecision` is the install. Both are `Copy`, so a call copies the discriminant. `Dangerous` exists so the install function can refuse it. `scan_plugin_shape` does not construct it. A pattern hit is chapter 25.

`PluginFile` is the caller's measurement. `bytes` is the size already counted. The struct is not `Copy`, because `path` is a `String`. The scanner reads `bytes` and does not open `path`.

## 12.18 `scan_plugin_shape` never returns `Dangerous`

Oversized is caution, not a hard block.

```rust
pub fn scan_plugin_shape(files: &[PluginFile]) -> PluginVerdict {
    let mut count = 0usize;
    let mut total = 0u64;
    let mut caution = false;
    for file in files {
        count += 1;
        total = total.saturating_add(file.bytes);
        if file.bytes > MAX_PLUGIN_SINGLE_FILE_BYTES {
            caution = true;
        }
    }
    if count > MAX_PLUGIN_FILE_COUNT || total > MAX_PLUGIN_TOTAL_BYTES {
        caution = true;
    }
    if caution {
        PluginVerdict::Caution
    } else {
        PluginVerdict::Safe
    }
}
```

> **Rust sidebar: `saturating_add` sticks at the top**
> - `u64` `+` panics in debug on overflow and wraps in release. A wrapped total can look smaller than 10 MB.
> - `saturating_add` returns `u64::MAX` instead. That value is still over `MAX_PLUGIN_TOTAL_BYTES`, so the verdict stays caution.
> - File count uses `+= 1`. A slice cannot be longer than `usize::MAX`, so that increment does not use the saturating form.

`count`, `total`, and `caution` start at 0, 0, and false. A zero-byte file still counts. Every comparison is `>`, so exactly 400 files, exactly 10 MB, and exactly 1 MB stay clear.

One file of `MAX_PLUGIN_SINGLE_FILE_BYTES + 1` sets the flag inside the loop. Count is 1 and the total is under 10 MB, so that input trips only the single-file check. 401 one-byte files trip only the count check. Their total is 401. One file of `MAX_PLUGIN_TOTAL_BYTES + 1` is also over 1 MB, so both the in-loop check and the total check fire. Several files each at or under 1 MB that sum past 10 MB would take the total check alone. The loop does not return early. One flag, decided after the slice. Oversized is `Caution`.

`caution` true returns `Caution`. Otherwise the function returns `Safe`. There is no third arm. `PluginVerdict::Dangerous` is not constructed. `scan_plugin_shape` never returns `Dangerous`. An empty slice returns `Safe`. The 32-byte file `ok.txt` returns `Safe`. Setting the flag twice is still `Caution`.

## 12.19 `should_allow_plugin_install` lets force override caution only

The scan says what the shape looks like. This function says whether to install. `force` cannot override `Dangerous`.

```rust
pub fn should_allow_plugin_install(verdict: PluginVerdict, force: bool) -> InstallDecision {
    match verdict {
        PluginVerdict::Safe => InstallDecision::Allow,
        PluginVerdict::Caution if force => InstallDecision::Allow,
        PluginVerdict::Caution => InstallDecision::Confirm,
        PluginVerdict::Dangerous => InstallDecision::Block,
    }
}
```

> **Rust sidebar: a match guard is an extra condition**
> - `Caution if force` matches that variant only when `force` is true.
> - Arms run top to bottom. The plain `Caution` arm handles caution when the guard failed, which is `force == false`.
> - Put the plain arm first and the guarded arm would never run. The order in the file is what makes force do something.

`Safe` returns `Allow` and does not read `force`. `Caution if force` returns `Allow`. That is the only arm that reads `force`. Force overrides caution. The test expects `Allow` for `Caution` with `true`.

The plain `Caution` arm returns `Confirm`. 401 files scan as `Caution` because 401 is over 400, and force false is confirm. The milestone's `"confirm"` is this arm. The test expects `Confirm` for `Caution` with `false`.

`Dangerous` returns `Block` with no guard. `force` is not read. Force overrides caution only. Force does not override dangerous. The test expects `Block` for `Dangerous` with `true`. Force false is the same arm.

`scan_plugin_shape` never produces `Dangerous`. Chapter 25 will. The test passes the variant directly. The step crate returns the enum. The milestone words are `"confirm"` and `"block"`. I am not pasting a solution into `my-harness/`.

## 12.20 What the benches time

`empty_pre_post` is the helper the empty-hook bench and the empty-hook test share.

```rust
pub fn empty_pre_post(hook: &dyn ToolHook, args: &Value) -> usize {
    let pre = hook.pre_tool_call("echo", args).expect("nop");
    let post = hook.post_tool_call("echo", "ok");
    pre.len() + usize::from(post.is_some())
}
```

The parameter is `&dyn ToolHook`. The bench holds `NopHook` on the stack. The sample does not bump an `Arc`.

The name is the literal `"echo"`. `expect("nop")` panics on `Err`. `NopHook` returns an empty vec and `None`, so the sum is 0. `usize::from(false)` is 0 and `usize::from(true)` is 1. The test asserts 0, with `object_map(&[])` as the arguments.

`benches/hooks.rs` times that helper as `empty_pre_post`, plus `api_request_hook` and `memory_enqueue`. `json!({})`, the thousand `Message` values, and the `MemoryQueue` are built before `b.iter`. `black_box` keeps the optimizer from deleting a call that always returns 0. The API sample calls `run_api_hook` on the borrowed slice. `NopApi` reads `len`. The enqueue sample reuses one queue, so after length 64 each sample pops and pushes. The `format!` is inside the sample. Nothing is written to disk. The means below are `mean.point_estimate` for those three names.

## Tests

Eight hook tests, plus the plugin test. A modify sticks when a later hook blocks. The first approve is kept, and a failing hook is skipped. A post hook replaces the result. Cron escalation denies. A failed check stays hidden. The API hook sees the same pointer. The queue length is 1. A block stops the script before `echo`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch12-hooks
```

```text
Starting 76 tests across 19 binaries
     Summary [   0.137s] 76 tests run: 76 passed, 0 skipped
```

```bash
cargo run -p ch12-hooks
```

```text
result: BLOCKED: hook refused rm
stopped: 1
```

The binary installs one hook that blocks `rm`, then a script of `rm` and `echo`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `empty_pre_post` | 10.90 ns | under 0.1 ms | one empty before hook and one empty after hook |
| `api_request_hook` | 2.182 ns | under 0.1 ms | `pre_api_request` on 1,000 borrowed messages |
| `memory_enqueue` | 61.70 ns | under 1 ms | one turn pushed onto the queue |

`empty_pre_post` calls `NopHook::pre_tool_call` and `post_tool_call` inside `b.iter`. The JSON arguments `{}` are built before the bench. No tool runs.

`api_request_hook` builds a vec of 1,000 `Message` values before the bench. Each content is `hello`. The timed closure calls `run_api_hook`, which calls `pre_api_request` on that slice. The hook reads `len`. It does not clone the vec. Building the 1,000 messages is excluded.

`memory_enqueue` locks the queue, drops the front if the length is already 64, and pushes one formatted string. The queue is built before the bench. The string format is inside the sample. Nothing is written to disk.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `12`.

### Milestone demo

The `cargo run` above. The result is the block message. `stopped` is 1.

## Pitfalls

`dispatch_pre` returns on the first block. A modify that was registered on a hook after that block never runs. Put the rewrite on an earlier hook if the block should still see it. The test does it that way.

An approve stored in `Dispatch.approve` is not a block. `run_script` ignores it and runs the tool. The escalation test calls `escalation_block` on purpose. Forgetting that call is how a plugin's "ask the human" becomes a silent allow.

The queue cap drops the oldest job. A caller that treats `len()` as "every turn since the process started" will be wrong after 64 enqueues. Chapter 21 is the durable copy.

`available` requires both the check and the enabled set. A hook that enables a set the tool is not in does nothing for that tool.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L5.

**Plugin veto.** `PreAction::Block` is the `pre_tool_call` veto. The message is the tool result. The script does not continue.

**A8.** `PreAction::Approve` is the escalation. `escalation_block` runs chapter 11's pipeline. A denial keeps the "has NOT consented" text. `gate_failed` is the fail-closed string when the gate cannot be called.

**Hook failure.** `Err` from `pre_tool_call` is skipped. The next hook still runs. A broken plugin does not become a deny, and it does not become an allow of a later block.

**Registry check.** `Registry::available` stays false when the check failed. The hook cannot enable a tool chapter 3 would have hidden.

**H3, structural half.** `scan_plugin_shape` applies the caps from `tools/plugin_guard.py`: 400 files, 10 MB total, 1 MB for one file. Oversized is `Caution`. `should_allow_plugin_install` lets `force` override caution and does not let it override `Dangerous`. The pattern engine is chapter 25 (H1).

**Left for later.** The post-turn reviewer is chapter 26. Memory files and the threat scan are chapter 21 (F5, F6). Secret redaction (E1, E2) is chapter 20. The hook message is not scrubbed in this chapter.

## Chapter summary

- **Before.** Modify, then block wins, then the first approve is kept. A failed hook is skipped.
- **After.** A hook may replace the result string.
- **Borrow.** The API hook sees the message slice. It does not get its own copy.
- **Queue.** One turn enqueues. The file write is chapter 21.
- **Time.** The empty pair measured **10.90 ns**. The API hook measured **2.182 ns**. The enqueue measured **61.70 ns**.

[personal note: which hook you expected to be a block, and what it rewrote instead]

## What you implement

`my-harness/tests/ch12.rs` is the milestone. It compiles, then panics in `todo!("chapter 12")` until `chapter_12_plugin_decision` returns `"confirm"` for 401 files with force false, and `"block"` for a dangerous plugin with force true. Force does not override dangerous. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 13 runs code outside this process. The approval gate from chapter 11 still sees the script before the child starts. The sandbox is off unless you ask for it.
