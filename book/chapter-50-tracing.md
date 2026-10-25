# Chapter 50: Seeing Inside

A span that kept the raw prompt would hand a secret to the collector and undo chapter 20's scrub. That refusal is the payoff of the chapter: the test passes `sk-live-0123456789abcdef` into `mock_turn`, `safe_field` returns `request`, and the sink holds `request`, `response`, and `gateway`. The prompt isn't a span field and isn't in the sink, and `format!("{sink:?}")` doesn't contain `sk-`, so the key the test used never becomes something a collector could store.

The demo prints exactly `trace: ok events=3 spans=5 team=101`. That line is three event names (`request`, `response`, `gateway`) and five spans on a mock turn, then `research_team(100)` notes of `ok` and one follow-up `0:kept`. It isn't a readable trace of a memory, a skill, a delegate, and a tool as one harness. Those modules sit in the crate because every harness chapter copied the previous one, and this demo doesn't drive them and doesn't replace chapter 26's reviewer or chapter 48's librarian.

The second `research_team` call runs after the first `thread::scope` returns, so `last()` can see that the notes are already there. The new thread ignores that string, `FakeModel` ignores the prompt, and the stored text is `{index}:{reply}`.

Tracing that changed a verdict would make chapter 52 measure the tracer, which a continuously improving harness can't accept, because the evals don't require a trace to pass and they require the loop to still return the outcomes chapter 9 named. The crate is `steps/ch50-tracing`, a copy of chapter 49 plus `spans`, the package is `ch50-tracing`, and `cargo run -p ch50-tracing --bin ch50-tracing` prints the demo line. The exercise is `my-harness/tests/ch50.rs`, where a span per event still has to finish the mock turn.

At the subscriber, the `tracing` crate's subscriber is a trait you implement, and the counting one is a shared counter plus methods that store nothing. A field is a key and a value you filter before it is recorded, which is what `safe_field` does to the prompt. The team is a loop of tasks, the same fan the earlier chapters bounded, and the spans in this demo wrap the mock turn only.

| Section | What it walks |
|---|---|
| 50.1 | `CountingSubscriber` |
| 50.2 | `safe_field` and `emit` |
| 50.3 | `FakeModel` and `mock_turn` |
| 50.4 | `Memory` |
| 50.5 | `research_team` |
| 50.6 | `demo` |

> **How Hermes does it**
>
> I read `agent/monitoring/otlp_exporter.py`, `agent/monitoring/redaction.py`, and `agent/moa_trace.py` on Hermes `main` at `9e01b44`. I read `redact_bounded` before describing it. I read `_traces_enabled_and_dir` before describing the flag.
>
> The module doc on `otlp_exporter.py` says it maps gateway monitoring events to OTel spans for the operator-configured `monitoring.export.otlp` endpoint, and that no default destination ships. The OTel SDK is an optional extra (`hermes-agent[otlp]`) imported lazily. `headers_env` values are read at export time and never logged or stored. The streaming subscriber runs fail-isolated on the emitter thread. `event_filter` keeps other planes off it. The first lines are:
>
> 

```python
> """Export monitoring events to an OpenTelemetry Collector over OTLP/HTTP.
>
> Maps gateway monitoring events to OTel spans for the operator-configured ``monitoring.export.otlp``
> endpoint (no default destination ships) and hosts the OTLP plumbing shared with
> ``gateway_health_export``.  The OTel SDK is an optional extra (``hermes-agent[otlp]``) imported
> lazily; ``headers_env`` values are read at export time and never logged or stored.  The streaming
> subscriber runs fail-isolated on the emitter thread; ``event_filter`` keeps other planes off it.
> """
> ```

>
> `redact_bounded` in `agent/monitoring/redaction.py` is:
>
> 

```python
> def redact_bounded(raw: Any, *, limit: int = 500, empty: str = "[redacted]", unavailable: str = UNAVAILABLE) -> str:
>     """Redact ``str(raw or "")`` and length-bound it; ``empty`` replaces an empty
>     result, ``unavailable`` is returned if redaction itself raises."""
>     try:
>         return (redact_for_export(str(raw or "")) or empty)[:limit]
>     except Exception:
>         return unavailable
> ```

>
> The module doc above that function says every string that leaves the process passes through `redact_for_export`: secrets via `agent/redact.py::redact_for_egress`, then PII. `redact_bounded` calls `redact_for_export` on `str(raw or "")`, slices to `limit`, and returns `unavailable` if redaction raises.
>
> `agent/moa_trace.py` is opt-in via `moa.save_traces`. The module doc says it is off by default, and that when it is off the only overhead is the config read. A turn that runs appends one JSON line to `<hermes_home>/moa-traces/<session_id>.jsonl`. The same doc says the side channel never enters the `messages` table, history, or replay. `_traces_enabled_and_dir` returns `None` when the flag is off:
>
> 

```python
> def _traces_enabled_and_dir() -> Optional[Path]:
>     """Trace directory if ``moa.save_traces`` is on, else None. Reads config per
>     call (once per cache-MISS turn); ``moa.trace_dir`` overrides the default."""
>     try:
>         from hermes_cli.config import load_config
>         moa_cfg = (load_config() or {}).get("moa") or {}
>     except Exception:  # pragma: no cover - never break a turn over tracing
>         return None
>     if not moa_cfg.get("save_traces"):
>         return None
> ```

>
> The side channel is a file under the trace dir. I am not claiming the prompt is stored in the model messages. The module doc says the trace never enters the `messages` table.
>
> **Adopted.** A span per event. The field is a safe name, not the prompt. `CountingSubscriber` counts spans and does not export.
>
> **Adapted.** There is no OTLP socket. `CountingSubscriber` is the whole exporter. A research team of mock agents writes one shared `Memory` across two runs. The dependency is the `tracing` crate, not the OTel SDK.
>
> **Deferred.** A collector. Headers. `moa.save_traces` files.

> **Guardrails.** The prompt is not a span field. No socket. Memory notes are the mock reply, which is `ok` or `kept`, not the user prompt, because `FakeModel` ignores the prompt. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

## What this chapter decides

`safe_field` discards the prompt and returns `"request"`, and that return is the only field value the turn span is allowed to carry.

`emit` pushes the name onto a sink and enters `tracing::info_span!("event", name)`, so the sink and the span share a name and neither of them receives the prompt.

`mock_turn` enters a turn span whose field is `safe_field(prompt)`, emits `request`, completes `FakeModel`, emits `response`, enters a gateway span, and emits `gateway`. The sink holds the three names, and the prompt isn't stored in the sink.

The counter on `CountingSubscriber` exists so a test can see how many spans a turn opened. `new_span` moves that counter, `spans_opened` is the counter minus one, and `event`, `record`, `enter`, and `exit` record nothing. `record_follows_from` records nothing too, and `enabled` returns `true`.

`research_team(n)` spawns `n` scoped threads, and each parent read of `memory.last()` happens on the spawning thread before that iteration's thread runs. The scoped thread pushes `"{index}:{note}"`, where `note` is `FakeModel`'s reply, and the second call of `research_team(1)` runs after the first `thread::scope` has returned, so the earlier notes are already in the mutex. One hundred then one yields length 101, and the new thread ignores the string `last()` returned.

`demo` returns `trace: ok events=3 spans=5 team=101`, where the reply is `ok`, the three events are the sink, and the five spans are the turn, the request event, the response event, the gateway span, and the gateway event. `team=101` is 100 notes of `ok` plus the follow-up `0:kept`.

## 50.1 `CountingSubscriber`

The counter exists so a test can see how many spans a turn opened, and the count is an `AtomicU64` that `new` starts at 1 inside an `Arc`. `spans_opened` subtracts one from that count, `demo` keeps a clone so the closure and the later read share it, and after `mock_turn` the five spans leave `spans_opened` at 5. `demo` calls `new`, so the first id is 1, and it doesn't call `Default::default()`.

`spans.rs` is the new module, `a2a.rs` is the chapter 49 copy, and `pub mod spans` is what `chapter_demo.rs` imports, through `harness_core::spans::demo`. The subscriber is the first type:

```rust
//! Book chapter 50. A span per event. The field is the event name, not the prompt.
//! A research team of mock agents writes into one memory across runs.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

#[derive(Clone, Default)]
pub struct CountingSubscriber {
    next: Arc<AtomicU64>,
}

impl CountingSubscriber {
    pub fn new() -> Self {
        Self {
            next: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn spans_opened(&self) -> u64 {
        self.next.load(Ordering::Relaxed).saturating_sub(1)
    }
}

impl Subscriber for CountingSubscriber {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &Attributes<'_>) -> Id {
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        Id::from_u64(id.max(1))
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, _event: &Event<'_>) {}

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}
```

> **Rust sidebar: the methods that record nothing.** `event`, `record`, `enter`, and `exit` record nothing, and `record_follows_from` records nothing either, so this subscriber keeps none of the values the macros built. `enabled` returns true and ignores its metadata, and `new_span` is the only method that moves the count, which is why a test can see how many spans a turn opened and still know the prompt was never stored. `with_default` installs the subscriber for a closure, `mock_turn` runs inside that closure in the test and in `demo`, and outside it these tests have no subscriber until `with_default` runs.

## 50.2 `safe_field` and `emit`

`safe_field` is the refusal the opening promised, and it returns `request` for every prompt, including the key the test passes and the word `hello` the demo passes.

```rust
/// Content-free field. The prompt is not a span field.
pub fn safe_field(text: &str) -> &'static str {
    let _ = text;
    "request"
}

pub fn emit(name: &'static str, sink: &mut Vec<&'static str>) {
    sink.push(name);
    let span = tracing::info_span!("event", name);
    let _entered = span.enter();
}
```

`safe_field` takes the prompt and doesn't use it. `let _ = text` names the parameter so the compiler accepts it, and the return is the static string `"request"`, so every prompt shares that return. `safe_field("sk-live-0123456789abcdef")` is `"request"`, and `safe_field("hello")` is `"request"`. The test asserts `safe_field(secret)` equals `"request"`.

The doc comment says the field is content-free and that the prompt isn't a span field. `mock_turn` passes `safe_field(prompt)` into the turn span as the `event` field, and the prompt string isn't that field.

`emit` pushes `name` onto the sink, then builds `tracing::info_span!("event", name)` and enters it. The span's name is the literal `event`. The field recorded by the macro is `name`, and the value is the `&'static str` argument, `"request"`, `"response"`, or `"gateway"`. The sink stores that same name, the sink doesn't store the prompt, and `emit` doesn't take the prompt.

`_entered` holds the enter guard until `emit` returns, and the guard drops at the end of the function, which exits the span. The subscriber's `enter` and `exit` record nothing, so the enter still counts as a `new_span` and doesn't store fields. The sink push happens before the span is built, which means a subscriber that refused `enabled` would still have the name in the sink, and this subscriber returns `true` from `enabled`.

Hermes `redact_bounded` redacts `str(raw or "")` and slices it to 500 characters by default, and `safe_field` doesn't call it. The prompt is discarded, and the field is `"request"`.

> **Rust sidebar: the guard has to outlive the statement.** `tracing::info_span!("event", name)` builds a span at info level, the first argument is the span name, and `name` in field position is shorthand for a field named `name` whose value is the variable `name`. The macro looks up the current subscriber and calls `new_span`. `let _entered = span.enter()` calls `enter` and keeps the guard in a local, the underscore means the function doesn't read the guard, and the drop at the end of the scope calls `exit`. Binding the local to `_` would drop the guard at the end of the statement, which would exit the span before `emit` returns, so the file binds `_entered` and the span stays entered for the rest of the function. The rest of the function is the end of the function, the span is open for no further work, and the count still increments because `new_span` ran. `sink.push(name)` stores the `&'static str` itself, the vec doesn't allocate a `String` for the name, and the three pushes in `mock_turn` are three statics. `format!("{sink:?}")` in the test prints those statics, and the test asserts that debug text doesn't contain `sk-`.

## 50.3 `FakeModel` and `mock_turn`

The turn is five spans and three names, and the model still ignores whatever prompt it was given:

```rust
pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

/// Chapter 1 events, then a gateway child span. No prompt text is stored.
pub fn mock_turn(model: &FakeModel, prompt: &str, sink: &mut Vec<&'static str>) -> String {
    let root = tracing::info_span!("turn", event = safe_field(prompt));
    let _root = root.enter();
    emit("request", sink);
    let reply = model.complete(prompt);
    emit("response", sink);
    let gateway = tracing::info_span!("gateway", event = "dispatch");
    let _gateway = gateway.enter();
    emit("gateway", sink);
    reply
}
```

`FakeModel` in this file is a second struct with the same shape as chapter 49's. `complete` ignores the prompt and returns `self.reply.to_string()`. `demo` uses `reply: "ok"` for the turn and for the team of 100, and `reply: "kept"` for the follow-up call. The test `a_turn_records_three_event_names_and_not_the_prompt` uses `reply: "ok"`. The test `one_hundred_agents_learn_across_two_runs` uses `"found"` and then `"kept"`, and what it checks is the length and the suffix, which the next sections walk.

The turn span is `info_span!("turn", event = safe_field(prompt))`. The span name is `turn`, the field name is `event`, and the field value is `safe_field(prompt)`, which is `"request"`. The prompt is an argument to `safe_field` and isn't the field value. `_root` holds the enter guard until `mock_turn` returns.

`emit("request", sink)` pushes `"request"` and opens a span named `event` whose `name` field is `"request"`, and that span's guard drops when `emit` returns while the turn guard is still held. `model.complete(prompt)` returns the reply, the prompt is passed and ignored, and for `demo` the reply is `ok`. For the secret test the reply is `ok` and the prompt is `sk-live-0123456789abcdef`.

`emit("response", sink)` pushes `"response"` and opens another `event` span. The gateway span is `info_span!("gateway", event = "dispatch")`, the span name is `gateway`, and the field `event` is the static `"dispatch"`. `_gateway` holds that enter until `mock_turn` returns, and the turn guard is still held, so the gateway enter happens while the turn is entered. `CountingSubscriber` records no parent links, because `record_follows_from` is empty, and the source text nests the enters: `_root` is still in scope when `_gateway` is created.

`emit("gateway", sink)` pushes `"gateway"` and opens a third `event` span, then `mock_turn` returns the reply. The gateway guard drops, then the turn guard drops, as the locals go out of scope in reverse order.

The sink is `["request", "response", "gateway"]`, three names, and the prompt isn't among them. The spans opened inside one `mock_turn` are five: the turn, the request event, the response event, the gateway span, and the gateway event. `demo` reads `spans_opened` after that call and prints 5. The test asserts `spans_opened() >= 3` and doesn't assert equality with 5, while the source opens five. The doc comment says these are chapter 1 events, then a gateway child span, and that no prompt text is stored.

> **Rust sidebar: the field is set before `new_span`.** `event = safe_field(prompt)` is a field named `event` set to the expression, and the expression runs in the caller before the subscriber sees the span, so `safe_field` has already returned `"request"` when `new_span` is invoked. `CountingSubscriber::new_span` drops the attributes, so the string `"request"` isn't saved by the subscriber, and it was still the value the macro put in the field. The prompt wasn't that value. `event = "dispatch"` is the same form with a literal, the gateway span's field is the word `dispatch`, and it isn't the prompt and it isn't the sink name `"gateway"`. The sink name `"gateway"` comes from `emit("gateway", sink)`, which is a different span, the one named `event`. `_root` and `_gateway` both live until the end of `mock_turn`, two spans are entered at the return, and their `exit` calls run on drop. Those methods record nothing. The ids were allocated at `new_span` and stay allocated from the counter's point of view, and the counter doesn't go backwards on exit.

## 50.4 `Memory`

The notes live in one mutex, and the second call can see them only because the first scope has already returned:

```rust
pub struct Memory {
    notes: Mutex<Vec<String>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            notes: Mutex::new(Vec::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.notes.lock().expect("memory").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn last(&self) -> Option<String> {
        self.notes.lock().expect("memory").last().cloned()
    }

    pub fn push(&self, note: String) {
        self.notes.lock().expect("memory").push(note);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
```

`Memory` is a mutex around a vec of notes. `new` starts empty, `len` locks and returns the vec length, and `is_empty` calls `len` and compares to 0. `demo` and the tests don't call `is_empty`. `last` locks, clones the last `String`, and returns it as `Some`, and an empty vec returns `None` because `slice::last` does. `push` locks and pushes an owned `String`. `expect("memory")` panics if the mutex is poisoned, and the tests don't poison it.

`Default` calls `new`, `demo` calls `Memory::new()` and doesn't use `Default`, and the notes field is private so callers use `len`, `last`, and `push`. `last` clones so the lock guard can drop before the caller uses the string, the `String` the caller holds is a copy, and a later `push` doesn't invalidate it. `research_team` reads `last` on the parent thread and moves that `Option<String>` into the scoped thread.

`lock` returns a guard that `expect` unwraps, `last()` on the vec returns `Option<&String>`, and `cloned` turns that into `Option<String>`. The guard drops at the end of the statement, which unlocks, and `push` needs the guard only for the `Vec::push`. Two threads can call `push` on the same `Memory`, the mutex serializes the pushes, and `research_team` shares one `Memory` across the scoped threads. The length after `thread::scope` returns is the number of pushes that finished, which is `n` for a single call that pushes once per index.

## 50.5 `research_team`

The second call can see that the notes are already there, and the note it stores is still the mock reply:

```rust
/// `n` mock agents append one note each. The second call sees the earlier notes.
pub fn research_team(n: usize, memory: &Memory, model: &FakeModel) -> usize {
    thread::scope(|scope| {
        for index in 0..n {
            let known = memory.last();
            scope.spawn(move || {
                let prompt = known.as_deref().unwrap_or("first");
                let note = model.complete(prompt);
                memory.push(format!("{index}:{note}"));
            });
        }
    });
    memory.len()
}
```

The doc comment says `n` mock agents append one note each, and that the second call sees the earlier notes. `thread::scope` joins every spawned thread before `scope` returns, the function then returns `memory.len()`, and the joins finish before that length is read. A second call on the same `Memory` therefore runs after those notes are already stored, which is why `last()` can see them.

The loop is `for index in 0..n`. On each iteration the parent calls `memory.last()` and then `scope.spawn`, and the parent read happens before that iteration's thread runs, though it can happen while earlier iterations' threads are already running. `last` isn't taken inside the thread. The thread receives the `Option<String>` that the parent cloned.

Inside the thread, `known.as_deref()` turns `Option<String>` into `Option<&str>`, and `unwrap_or("first")` uses `"first"` when `last` was `None`. `model.complete(prompt)` ignores that prompt and returns the model's reply. `memory.push(format!("{index}:{note}"))` stores the index, a colon, and the reply, so the stored text is `{index}:{reply}` and the user prompt stays out of the note.

`demo` calls `research_team(100, &memory, &model)` with `reply: "ok"`. One hundred threads each push a note shaped `{index}:ok`, `thread::scope` joins them, and the length is 100. The next line calls `research_team(1, &memory, &FakeModel { reply: "kept" })`. The parent `last()` reads one of the earlier notes, because the first scope has returned and the vec holds 100 strings. The new thread ignores that string, completes `"kept"`, and pushes `{index}:kept`. The index is 0, because the loop is `0..1`, the stored note is `0:kept`, `memory.len()` is 101, and `last()` ends with `kept`.

The test `one_hundred_agents_learn_across_two_runs` uses `FakeModel { reply: "found" }` for the hundred and `FakeModel { reply: "kept" }` for the one. It asserts the first return equals 100, `memory.len()` equals 100, the second return equals 101, and `memory.last().unwrap()` ends with `kept`. It doesn't assert the text `found` and it doesn't assert the index. The second call can see that the notes are already there, the new thread ignores that string, and `FakeModel` ignores the prompt it is given, so the new note is still the reply.

Within one `research_team(100)` call, each parent `last()` can observe pushes from threads spawned on earlier iterations, or it can still see `None` if those threads haven't pushed yet. The test doesn't assert which prompt each of the hundred threads received, and it asserts the length. The second call is the one the doc comment describes, because it runs after the first scope, so `last` can see the earlier notes.

`thread::scope` borrows `memory` and `model` for the life of the scope, and the spawned closures can use those references because the scope joins them before the references would end. A `thread::spawn` that outlived `research_team` couldn't borrow `memory`, and `scope.spawn` can. `move` moves `known` and `index` into the closure, `index` is `usize` and copies, and `known` is `Option<String>` and moves. `memory` and `model` are references captured from the environment. The signature takes `&Memory` and `&FakeModel`, so the pushes share one vec and one reply. `as_deref` on `Option<String>` borrows the string as `&str` without cloning it again, `unwrap_or("first")` doesn't allocate, and `format!` allocates the note that `push` stores. The reply `ok` becomes a note like `0:ok`, and the reply `kept` becomes `0:kept` on the follow-up, whose index range is `0..1`. `thread::scope` returns when every spawned thread has returned, a panic in a scoped thread panics the scope, and these threads call `complete` and `push`. `complete` doesn't panic, `push` panics if the mutex is poisoned, and the tests get `100` and `101` back.

## 50.6 `demo`

The printed line is the mock turn and the team length, and it isn't a trace of the rest of the harness:

```rust
pub fn demo() -> String {
    let model = FakeModel { reply: "ok" };
    let mut sink = Vec::new();
    let subscriber = CountingSubscriber::new();
    let opened = subscriber.clone();
    let reply =
        tracing::subscriber::with_default(subscriber, || mock_turn(&model, "hello", &mut sink));
    let memory = Memory::new();
    research_team(100, &memory, &model);
    let again = research_team(1, &memory, &FakeModel { reply: "kept" });
    format!(
        "trace: {reply} events={} spans={} team={again}",
        sink.len(),
        opened.spans_opened()
    )
}
```

`demo` builds one model whose reply is `ok`, the sink starts empty, and the subscriber starts at 1, with `opened` as a clone of the `Arc`. `with_default` moves `subscriber` into the closure's subscriber slot and runs `mock_turn` with the prompt `"hello"`.

Inside that call the sink becomes `["request", "response", "gateway"]`, five spans are opened, and `complete` ignores `"hello"` and returns `ok`. `reply` is `ok`, `sink.len()` is 3, and `opened.spans_opened()` is 5, because the clone shares the counter and the counter sits at 6 after five `fetch_add` calls, so `6 - 1` is 5.

`Memory::new()` starts empty, `research_team(100, ...)` pushes 100 notes of the form `{index}:ok` and returns 100, and that return value is discarded. `research_team(1, ...)` with a new `FakeModel { reply: "kept" }` pushes `0:kept` and returns 101, so `again` is 101. The format string is `trace: ok events=3 spans=5 team=101`.

The notes in this demo are `ok` and `kept`, they are the mock replies, and they aren't the user prompt, because `FakeModel` ignores the prompt. The prompt of the turn was `"hello"`, the prompt of a team thread is `memory.last()` or `"first"`, and neither string is what `push` stores, because `push` stores the index and the reply.

`research_team` doesn't enter a span, the subscriber isn't installed around the team, and the span count stays 5. The test `demo_names_the_team` asserts the line starts with `trace: ok events=3 ` (the prefix includes the space after `3`) and that the line contains `team=101`. It doesn't assert `spans=5` by itself. The format string and the counts produce `spans=5` in the returned line. The binary prints that line, with no key, no socket, and no file under a trace directory.

`tracing::subscriber::with_default(subscriber, closure)` sets the thread-local subscriber for the closure and restores the previous one afterward. The subscriber is moved in, and the clone `opened` is how `demo` reads the count after the closure. Without the clone, the `Arc` would have moved and `spans_opened` would be unreachable. `Clone` on `CountingSubscriber` is derived, and the only field is the `Arc`, so the clone is a new subscriber struct pointing at the same count. `format!` uses `{reply}` by name and `{}` for `sink.len()` and `opened.spans_opened()` in order, and `team={again}` interpolates the length from the second `research_team`. The spaces in the format string are the spaces in `trace: ok events=3 spans=5 team=101`.

## The turn, walked with the secret from the test

The test builds `CountingSubscriber::new`, clones it into `watched`, and uses an empty sink. The prompt is `sk-live-0123456789abcdef`, the model reply is `ok`, and `with_default` runs `mock_turn`. `safe_field` returns `"request"`, the turn span's `event` field is that word, and `emit("request")` pushes `"request"`. `complete` returns `ok` and drops the secret inside `let _ = prompt`. `emit("response")` pushes `"response"`, the gateway span's `event` field is `"dispatch"`, and `emit("gateway")` pushes `"gateway"`.

The assertions are that `reply` equals `"ok"`, `sink` equals `vec!["request", "response", "gateway"]`, `watched.spans_opened()` is at least 3, `safe_field(secret)` equals `"request"`, and `format!("{sink:?}")` doesn't contain `sk-`. The secret isn't in the sink, the span field the source built for the turn is `"request"`, and five `new_span` calls happen in that `mock_turn`. The test's bound is `>= 3`, and a subscriber that counted only the three `emit` spans would pass that one assertion. This subscriber also counts the turn span and the gateway span, so the value `demo` prints is 5.

## The team, walked as 100 then 1

Start with `Memory::new`, where `len` is 0 and `last` is `None`. `research_team(100, &memory, &FakeModel { reply: "found" })` enters `thread::scope`, index 0 reads `last`, and the vec is still empty unless a previous program pushed, which it didn't. The scoped thread's prompt becomes `"first"` if that `Option` is `None`, `complete` returns `found` either way, and the push is `{index}:found`. Indexes 1 through 99 do the same parent `last` and the same push of their own index. `thread::scope` joins all 100, `memory.len()` is 100, the function returns 100, and the test asserts both.

`research_team(1, &memory, &FakeModel { reply: "kept" })` runs next, after that scope has returned. The parent `last()` clones one of the notes already stored, the thread ignores it, `complete` returns `kept`, and the push is `0:kept`. `len` is 101, the function returns 101, and `last().unwrap()` is `0:kept`, which ends with `kept`. The test asserts the 101 and the suffix `kept`. The second call can see that the notes are already there, and the stored text is still `{index}:{reply}`.

`demo` is the same shape with replies `ok` and `kept`, and with the turn recorded before the team. The team length is still 101, the turn's sink length is 3, the span count is 5, and the line is `trace: ok events=3 spans=5 team=101`.

## Tests

The new tests are `steps/ch50-tracing/crates/harness-core/tests/spans.rs`, and the package also builds the copied harness tests, including `a2a.rs`. The nextest line below is the package run.

`a_turn_records_three_event_names_and_not_the_prompt` installs `CountingSubscriber` with `with_default` and runs `mock_turn` on `FakeModel { reply: "ok" }` with the prompt `sk-live-0123456789abcdef`. It asserts the reply equals `"ok"`, `sink` equals `vec!["request", "response", "gateway"]`, `watched.spans_opened() >= 3`, `safe_field(secret)` equals `"request"`, and `format!("{sink:?}")` doesn't contain `sk-`.

`one_hundred_agents_learn_across_two_runs` calls `research_team(100, ...)` with `FakeModel { reply: "found" }`, asserts that return equals 100 and `memory.len()` equals 100, then calls `research_team(1, ...)` with `FakeModel { reply: "kept" }` and asserts that return equals 101 and `memory.last().unwrap()` ends with `kept`. The second call can see that the notes are already there, and the assertion on the text is the suffix `kept`.

`demo_names_the_team` binds `demo()` to `line`, asserts `line.starts_with("trace: ok events=3 ")`, and asserts `line.contains("team=101")`. The failure messages print `{line}`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch50-tracing
```

```text
    Starting 258 tests across 55 binaries
     Summary [   0.407s] 258 tests run: 258 passed, 0 skipped
```

```bash
cargo run -p ch50-tracing --bin ch50-tracing
```

```text
trace: ok events=3 spans=5 team=101
```

The binary records one turn and a team of 101 notes, with no key and no socket. `examples/demo.rs` is the eval runner, and the line above is `chapter_demo.rs`.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `event_without_span` | 18.1 ns | one `FakeModel::complete` with the subscriber installed and no span entered | `complete("hello")` |
| `span_per_event` | 236.4 ns | one `mock_turn` under that same subscriber | the turn, three event spans, and the gateway span |

The ids are `50/event_without_span` and `50/span_per_event`. The aliases are `48/event_without_span`, `46/event_without_span`, `48/span_per_event`, and `46/span_per_event` in `benchmarks/descriptions.json`.

```rust
fn both(c: &mut Criterion) {
    let model = FakeModel { reply: "ok" };
    let subscriber = CountingSubscriber::new();
    tracing::subscriber::with_default(subscriber, || {
        c.bench_function("event_without_span", |b| {
            b.iter(|| {
                let reply = model.complete(black_box("hello"));
                black_box(reply);
            });
        });
        c.bench_function("span_per_event", |b| {
            b.iter(|| {
                let mut sink = Vec::with_capacity(3);
                let reply = mock_turn(black_box(&model), black_box("hello"), &mut sink);
                black_box((reply, sink));
            });
        });
    });
}
```

Subscriber install sits outside both samples. `CountingSubscriber::new` and `with_default` run before either `bench_function` body is timed, `descriptions.json` says the same thing, and `FakeModel { reply: "ok" }` is built outside the samples.

`event_without_span` is `b.iter` of `complete` on `"hello"` passed through `black_box`, then `black_box` of the reply. The reply is `ok`. The function doesn't enter a span, `mock_turn`, `research_team`, and the sink allocation aren't called, and the prompt isn't written to a sink. The id is `50/event_without_span`, and the mean cell is `18.1 ns`.

`span_per_event` is `b.iter` of a `Vec` with capacity 3, then `mock_turn` of the model and `"hello"` passed through `black_box`, then `black_box` of the reply and the sink. The turn span, the three `emit` spans, the gateway span, and one `complete` are inside the sample, which is five spans: turn, request event, response event, gateway, gateway event. The sink receives `request`, `response`, and `gateway`. The span field on the turn is the word `request`, and the prompt stays out of that field. `research_team` isn't called, and no file is written. The id is `50/span_per_event`, and the mean cell is `236.4 ns`.

`descriptions.json` says a span per event has to add less than 0.1 milliseconds, and the chapter 1 budget of 1 millisecond still has to hold with tracing on. It says the 100-agent team isn't in the sample. It says around 100 nanoseconds above `event_without_span` is the span tax the reading expects, and that one millisecond would still meet the chapter 1 budget. Those sentences are the file's reading of the sample, and they aren't a measured mean. The means are `18.1 ns` and `236.4 ns`. They are `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), section `50`.

`.github/scripts/run_bench_section.sh` runs `cargo bench -p ch50-tracing --bench spans --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10` for section 50. The same flags are in `steps/ch50-tracing/README.md`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

### Milestone demo

The line is `trace: ok events=3 spans=5 team=101`.

## Pitfalls

The bench is one `complete` and one `mock_turn`, while `research_team(100)` is the test and the demo, so a number that spawns 100 threads isn't `span_per_event`.

`safe_field` returns `"request"` for every prompt, and a later edit that puts `text` into the span field would put the secret into the attributes. `new_span` currently drops attributes, the test also checks the sink, which `emit` fills with names, and the secret test fails if the sink debug text contains `sk-`.

`FakeModel` ignores the prompt, so a note that equaled the user prompt would be a different function. `format!("{index}:{note}")` stores the reply, and in `demo` the replies are `ok` and `kept`, with the follow-up stored as `0:kept`.

`CountingSubscriber` exports nothing, and pointing it at a collector would be a socket this file doesn't open. `headers_env` in the Hermes exporter is read at export time and never logged or stored, and this subscriber has no header map.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**The prompt is not a span field.** `safe_field` returns `"request"`. The turn span uses that return as `event`. `emit` stores `"request"`, `"response"`, and `"gateway"` in the sink. The test prompt `sk-live-0123456789abcdef` is passed to `mock_turn` and does not appear in `format!("{sink:?}")`.

**No socket.** `CountingSubscriber` increments an atomic and returns. `enabled` is `true`. `event`, `record`, `enter`, and `exit` are empty. There is no OTLP client. Hermes maps gateway events to OTel spans only for an operator-configured endpoint, and no default destination ships. The SDK is the optional extra `hermes-agent[otlp]`. This crate depends on `tracing = "0.1"`.

**Memory notes are the mock reply.** `demo` uses `ok` for the hundred and `kept` for the follow-up. `FakeModel::complete` ignores the prompt. The stored string is `{index}:{reply}`. The user prompt is not the note. The test's first reply is `found`, and the assertion on the second note is the suffix `kept`.

**Left for later.** A collector, headers, and `moa.save_traces` files. `redact_bounded` length-bounds a redacted export string at 500 by default. This file does not call it. The field is the safe name, so the prompt is not handed to that scrub on a span.

## What you implement

`my-harness/tests/ch50.rs` is the milestone.

```rust
use my_harness::chapter_50_span_finishes_turn;

#[test]
fn a_span_per_event_still_finishes_the_mock_turn() {
    assert!(chapter_50_span_finishes_turn());
}
```

The stub panics, because the body is still the `todo!` below.

```rust
pub fn chapter_50_span_finishes_turn() -> bool {
    todo!("chapter 50")
}
```

`todo!("chapter 50")` is the panic, and the test asserts `chapter_50_span_finishes_turn()` is true. In the step crate that is `mock_turn` under `CountingSubscriber` returning the fake reply after the request, response, and gateway names are in the sink, and the stub stays `todo!`.

## Chapter summary

- **Subscriber:** `new_span` counts from 1, `spans_opened` is the counter minus one, and `event`, `record`, `enter`, and `exit` record nothing.
- **Field:** `safe_field` returns `request`, and the prompt isn't a span field and isn't in the sink.
- **Turn:** the sink holds `request`, `response`, and `gateway`, and five spans open: turn, request event, response event, gateway, gateway event.
- **Team:** `research_team(100)` then `research_team(1)` leaves 101 notes, the second call can see that the notes are already there, and the note text is the mock reply `{index}:{reply}`.
- **Model:** `FakeModel` answers `ok` or `kept` in `demo`, and the line is `trace: ok events=3 spans=5 team=101`.
- **Time:** `event_without_span` mean `18.1 ns` and `span_per_event` mean `236.4 ns` are `mean.point_estimate`.

## What's next

Once the field stays a name, chapter 52 copies this harness, and the server's token comparison is chapter 51.
