# Chapter 50: Seeing Inside

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 49 can call an external agent and cancel the call. When a team run goes wrong, the transcript says what the model said. It does not say which span was open, which field was safe to log, or whether the hundred agents in a research team finished. This chapter adds spans on the events the harness already emits, and it adds the research team that uses them. The crate is `steps/ch50-tracing`, a copy of chapter 49 plus `spans`. The package is `ch50-tracing`. Chapter 51 is not this team. Chapter 51 is the server's load and token comparison. Chapter 52 copies this harness, not chapter 51.

`CountingSubscriber` counts the spans a test opened, so the bench can compare a turn with tracing and a turn without it. `safe_field` decides what may be attached to a span. A secret is not a span field. `emit` records one event. `mock_turn` runs the scripted model under that subscriber. `Memory` is the small store the research team reads across runs, the harness memory, not a new database. `research_team` runs 100 mock agents and then one more. `demo` prints the team. A span per event still finishes the mock turn, which is the requirement: tracing is not allowed to change the outcome, and it is not allowed to blow the chapter 1 budget of a millisecond of overhead once the model is excluded.

`research_team` runs 100 threads that each append the scripted reply, then one more call that can see those notes. The demo prints `trace: ok events=3 spans=5 team=101`. Sessions, memory, skills, and the team patterns are modules in this copy because every harness chapter copied the previous one. This demo does not drive them, and it does not replace chapter 26's reviewer or chapter 48's librarian. What this chapter locks is the span field. A span that included the raw prompt would undo chapter 20's scrub by writing the secret to a collector. `safe_field` is that refusal.

Chapter 52 re-runs chapter 9's saved evals on this copy of `run_loop`, with the spans still in the crate. The evals do not require a trace to pass. They require the loop to still return the outcomes chapter 9 named. Tracing that changed a verdict would make the suite measure the tracer.

Rust, at the subscriber: the `tracing` crate's subscriber is a trait you implement, and the counting one is a few atomics. A field is a key and a value you filter before it is recorded. The team is a loop of tasks, the same fan the earlier chapters bounded, now with a span entered and exited around the mock turn.

`cargo run -p ch50-tracing --bin ch50-tracing` runs the demo. The exercise is `my-harness/tests/ch50.rs`: a span per event still finishes the mock turn.

**In this chapter**

- 50.1 `CountingSubscriber`
- 50.2 `safe_field` and `emit`
- 50.3 `FakeModel` and `mock_turn`
- 50.4 `Memory`
- 50.5 `research_team`
- 50.6 `demo`

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
> ```python
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
> ```python
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
> ```python
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

[personal note: a span that held a name]

## What this chapter decides

`safe_field` discards the prompt and returns `"request"`.

`emit` pushes the name onto a sink and enters `tracing::info_span!("event", name)`.

`mock_turn` enters a turn span whose field is `safe_field(prompt)`, emits `request`, completes `FakeModel`, emits `response`, enters a gateway span, and emits `gateway`. The sink holds the three names. The prompt is not stored in the sink.

`CountingSubscriber::new_span` increments an `AtomicU64` that starts at 1. `spans_opened` is that counter minus one. `event`, `record`, `enter`, and `exit` are empty. `record_follows_from` is empty too. `enabled` returns `true`.

`research_team(n)` spawns `n` scoped threads. Each reads `memory.last()` on the spawning thread, then the scoped thread pushes `"{index}:{note}"`. `note` is `FakeModel`'s reply. The second call of `research_team(1)` runs after the first `thread::scope` has returned, so the earlier notes are already in the mutex. One hundred then one yields length 101.

`demo` returns `trace: ok events=3 spans=5 team=101`. The reply is `ok`. The three events are the sink. The five spans are the turn, the request event, the response event, the gateway span, and the gateway event. `team=101` is 100 notes plus the follow-up.

## 50.1 `CountingSubscriber`

`spans.rs` is the new module. `a2a.rs` is the chapter 49 copy. `pub mod spans` is what `chapter_demo.rs` imports, through `harness_core::spans::demo`. The subscriber is the first type:

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

The module comment says a span per event, and that the field is the event name, not the prompt. The second line says the research team writes into one memory across runs.

`new` stores an `AtomicU64` that starts at 1, inside an `Arc`. `Clone` on the subscriber clones that `Arc`. `demo` keeps `opened` as a clone and moves `subscriber` into `with_default`. Both see the same counter. `spans_opened` loads the counter with `Ordering::Relaxed` and subtracts 1 with `saturating_sub`. After `new`, before any span, the load is 1 and `spans_opened` is 0.

`enabled` returns `true`. The metadata argument is unused. Spans and events are enabled.

`new_span` ignores the `Attributes`. It does not read the field values. `fetch_add(1)` returns the previous counter and then adds 1. The first call returns 1 and leaves the counter at 2. `Id::from_u64` is called with `id.max(1)`. The first id is 1. `spans_opened` becomes 1. The fifth call returns 5 and leaves the counter at 6. `spans_opened` becomes 5. That is the demo's span count.

`record` takes a span id and a `Record` and does nothing. `record_follows_from` does nothing. `event` does nothing. `enter` does nothing. `exit` does nothing. The subscriber counts ids. It does not write the prompt. It does not open a socket. It does not call an exporter. `CountingSubscriber` is the whole exporter in this crate.

`#[derive(Default)]` is on the struct. `new` is what `demo` and the tests call, and `new` sets the counter to 1. `Default` would also build the struct. `AtomicU64`'s `Default` is 0, and `Arc`'s `Default` is a new allocation of that default. `demo` does not call `Default::default()`. It calls `new`, so the first id is 1 and `Id::from_u64` receives 1. `id.max(1)` still forces a 0 up to 1 if a counter ever handed one out.

### Rust: `Subscriber` and `Id`

`tracing::Subscriber` is the trait the `tracing` macros call on the current subscriber. `with_default` installs one for a closure. Outside that closure the macros have whatever subscriber the process already had, which in these tests is none until `with_default` runs. `mock_turn` is called inside `with_default` in the test and in `demo`. The bench installs the subscriber once around both bench functions.

`new_span` must return an `Id`. The `Attributes` are the span's metadata and fields at creation. This impl discards them. A field that `info_span!` built is not stored. `record` is the later path for values recorded after creation. That method is empty, so a value recorded there is not stored either.

`fetch_add` on `AtomicU64` is one atomic increment. Two threads opening spans get different ids because each `fetch_add` returns a distinct previous value. `research_team` does not open spans. `mock_turn` runs on the calling thread in `demo`. The five ids are 1 through 5 in order.

`Ordering::Relaxed` is what this subscriber writes. Chapter 49's pool uses `SeqCst`. This counter is a tally. `spans_opened` is read after `with_default` returns, on the same thread that opened the spans, in `demo`. The test reads it after `with_default` returns too.

## 50.2 `safe_field` and `emit`

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

`safe_field` takes the prompt and does not use it. `let _ = text` names the parameter. The return is the static string `"request"`. Every prompt shares that return. `safe_field("sk-live-0123456789abcdef")` is `"request"`. `safe_field("hello")` is `"request"`. The test asserts `safe_field(secret)` equals `"request"`.

The doc comment says the field is content-free and that the prompt is not a span field. `mock_turn` passes `safe_field(prompt)` into the turn span as the `event` field. The prompt string is not that field.

`emit` pushes `name` onto the sink, then builds `tracing::info_span!("event", name)` and enters it. The span's name is the literal `event`. The field recorded by the macro is `name`, and the value is the `&'static str` argument: `"request"`, `"response"`, or `"gateway"`. The sink stores that same name. The sink does not store the prompt. `emit` does not take the prompt.

`_entered` holds the enter guard until `emit` returns. The guard drops at the end of the function, which exits the span. `CountingSubscriber::enter` and `exit` are empty, so the enter still counts as a `new_span` and does not store fields. The sink push happens before the span is built. A subscriber that refused `enabled` would still have the name in the sink. This subscriber returns `true` from `enabled`.

Hermes `redact_bounded` redacts `str(raw or "")` and slices it to 500 characters by default. `safe_field` does not call it. The prompt is discarded. The field is `"request"`.

### Rust: `info_span!` and a guard

`tracing::info_span!("event", name)` builds a span at info level. The first argument is the span name. `name` in field position is shorthand for a field named `name` whose value is the variable `name`. The macro looks up the current subscriber and calls `new_span`. `let _entered = span.enter()` calls `enter` and keeps the guard in a local. The underscore means the function does not read the guard. The drop at the end of the scope calls `exit`. Renaming the local to `_` would drop the guard at the end of the statement, which would exit the span before `emit` returns. The file binds `_entered` so the enter lasts for the rest of the function. The rest of the function is the end of the function. The span is open for no further work. The count still increments, because `new_span` ran.

`sink.push(name)` stores the `&'static str` itself. The vec does not allocate a `String` for the name. The three pushes in `mock_turn` are three statics. `format!("{sink:?}")` in the test prints those statics. The test asserts that debug text does not contain `sk-`.

## 50.3 `FakeModel` and `mock_turn`

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

`FakeModel` in this file is a second struct with the same shape as chapter 49's. `complete` ignores the prompt and returns `self.reply.to_string()`. `demo` uses `reply: "ok"` for the turn and for the team of 100, and `reply: "kept"` for the follow-up call. The test `a_turn_records_three_event_names_and_not_the_prompt` uses `reply: "ok"`. The test `one_hundred_agents_learn_across_two_runs` uses `"found"` and then `"kept"`.

`mock_turn` does six steps.

The turn span is `info_span!("turn", event = safe_field(prompt))`. The span name is `turn`. The field name is `event`. The field value is `safe_field(prompt)`, which is `"request"`. The prompt is an argument to `safe_field` and is not the field value. `_root` holds the enter guard until `mock_turn` returns.

`emit("request", sink)` pushes `"request"` and opens a span named `event` whose `name` field is `"request"`. That span's guard drops when `emit` returns. The turn guard is still held.

`model.complete(prompt)` returns the reply. The prompt is passed and ignored. For `demo` the reply is `ok`. For the secret test the reply is `ok` and the prompt is `sk-live-0123456789abcdef`.

`emit("response", sink)` pushes `"response"` and opens another `event` span.

The gateway span is `info_span!("gateway", event = "dispatch")`. The span name is `gateway`. The field `event` is the static `"dispatch"`. `_gateway` holds that enter until `mock_turn` returns. The turn guard is still held, so the gateway enter happens while the turn is entered. `CountingSubscriber` does not record parent links. `record_follows_from` is empty. The source text nests the enters: `_root` is still in scope when `_gateway` is created.

`emit("gateway", sink)` pushes `"gateway"` and opens a third `event` span. Then `mock_turn` returns the reply. The gateway guard drops, then the turn guard drops, as the locals go out of scope in reverse order.

The sink is `["request", "response", "gateway"]`. Three names. The prompt is not among them. The spans opened inside one `mock_turn` are five: the turn, the request event, the response event, the gateway span, and the gateway event. `demo` reads `spans_opened` after that call and prints 5. The test asserts `spans_opened() >= 3`. It does not assert equality with 5. The source opens five.

The doc comment says these are chapter 1 events, then a gateway child span, and that no prompt text is stored.

### Rust: a field binding in the macro

`event = safe_field(prompt)` is a field named `event` set to the expression. The expression runs in the caller, before the subscriber sees the span. `safe_field` has already returned `"request"` when `new_span` is invoked. `CountingSubscriber::new_span` drops the attributes, so the string `"request"` is not saved by the subscriber. It was still the value the macro put in the field. The prompt was not that value.

`event = "dispatch"` is the same form with a literal. The gateway span's field is the word `dispatch`. It is not the prompt and it is not the sink name `"gateway"`. The sink name `"gateway"` comes from `emit("gateway", sink)`, which is a different span, the one named `event`.

`_root` and `_gateway` both live until the end of `mock_turn`. Two spans are entered at the return. Their `exit` calls run on drop. Those methods are empty. The ids were allocated at `new_span` and stay allocated from the counter's point of view. The counter does not go backwards on exit.

## 50.4 `Memory`

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

`Memory` is a mutex around a vec of notes. `new` starts empty. `len` locks and returns the vec length. `is_empty` calls `len` and compares to 0. `demo` and the tests do not call `is_empty`. `last` locks, clones the last `String`, and returns it as `Some`. An empty vec returns `None`, because `slice::last` does. `push` locks and pushes an owned `String`. `expect("memory")` panics if the mutex is poisoned. The tests do not poison it.

`Default` calls `new`. `demo` calls `Memory::new()` and does not use `Default`. The notes field is private. Callers use `len`, `last`, and `push`.

`last` clones so the lock guard can drop before the caller uses the string. The `String` the caller holds is a copy. A later `push` does not invalidate it. `research_team` reads `last` on the parent thread and moves that `Option<String>` into the scoped thread.

### Rust: `Mutex<Vec<String>>` and `Option`

`lock` returns `Result<MutexGuard<Vec<String>>, PoisonError<_>>`. `expect` unwraps the guard. `last()` on the vec returns `Option<&String>`. `cloned` turns that into `Option<String>` by cloning the inner string. `None` stays `None`. The guard drops at the end of the statement, which unlocks. `push` needs the guard only for the `Vec::push`. The note is moved into the vec.

Two threads can call `push` on the same `Memory`. The mutex serializes the pushes. `research_team` shares one `Memory` across the scoped threads. The length after `thread::scope` returns is the number of pushes that finished, which is `n` for a single call that pushes once per index.

## 50.5 `research_team`

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

The doc comment says `n` mock agents append one note each, and that the second call sees the earlier notes.

`thread::scope` gives a scope that joins every spawned thread before `scope` returns. The function then returns `memory.len()`. The joins finish before that length is read. A second call on the same `Memory` therefore sees the notes the first call pushed.

The loop is `for index in 0..n`. On each iteration the parent calls `memory.last()` and then `scope.spawn`. The parent read happens before that iteration's thread runs, and it can happen while earlier iterations' threads are already running. `last` is not taken inside the thread. The thread receives the `Option<String>` that the parent cloned.

Inside the thread, `known.as_deref()` turns `Option<String>` into `Option<&str>`. `unwrap_or("first")` uses `"first"` when `last` was `None`. `model.complete(prompt)` ignores that prompt and returns the model's reply. `memory.push(format!("{index}:{note}"))` stores the index, a colon, and the reply. The user prompt is not the note. The note is the mock reply.

`demo` calls `research_team(100, &memory, &model)` with `reply: "ok"`. One hundred threads each push a note shaped `{index}:ok`. `thread::scope` joins them. The length is 100. The next line calls `research_team(1, &memory, &FakeModel { reply: "kept" })`. The parent `last()` reads one of the earlier notes, because the first scope has returned and the vec holds 100 strings. The new thread ignores that string, completes `"kept"`, and pushes `{index}:kept`. The index is 0, because the loop is `0..1`. The stored note is `0:kept`. `memory.len()` is 101. `last()` ends with `kept`.

The test `one_hundred_agents_learn_across_two_runs` uses `FakeModel { reply: "found" }` for the hundred and `FakeModel { reply: "kept" }` for the one. It asserts the first return equals 100, `memory.len()` equals 100, the second return equals 101, and `memory.last().unwrap()` ends with `kept`. It does not assert the text `found` and it does not assert the index.

Within one `research_team(100)` call, each parent `last()` can observe pushes from threads spawned on earlier iterations, or it can still see `None` if those threads have not pushed yet. The test does not assert which prompt each of the hundred threads received. It asserts the length. The second call is the one the doc comment describes. It runs after the first scope, so `last` sees the earlier notes. `FakeModel` ignores the prompt it is given. The new note is still the reply.

### Rust: `thread::scope`

`thread::scope` borrows `memory` and `model` for the life of the scope. The spawned closures can use those references because the scope joins them before the references would end. A `thread::spawn` that outlived `research_team` could not borrow `memory`. `scope.spawn` can. `move` moves `known` and `index` into the closure. `index` is `usize`, which is `Copy`. `known` is `Option<String>` and moves. `memory` and `model` are references captured from the environment. The signature takes `&Memory` and `&FakeModel`, so the pushes share one vec and one reply.

`as_deref` on `Option<String>` borrows the string as `&str` without cloning it again. `unwrap_or("first")` does not allocate. `format!` allocates the note that `push` stores. The reply `ok` becomes a note like `0:ok`. The reply `kept` becomes `0:kept` on the follow-up, whose index range is `0..1`.

`thread::scope` returns when every spawned thread has returned. A panic in a scoped thread panics the scope. These threads call `complete` and `push`. `complete` does not panic. `push` panics if the mutex is poisoned. The tests get `100` and `101` back.

## 50.6 `demo`

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

`demo` builds one model whose reply is `ok`. The sink starts empty. The subscriber starts at 1. `opened` clones the `Arc`. `with_default` moves `subscriber` into the closure's subscriber slot and runs `mock_turn` with the prompt `"hello"`.

Inside that call the sink becomes `["request", "response", "gateway"]`. Five spans are opened. `complete` ignores `"hello"` and returns `ok`. `reply` is `ok`. `sink.len()` is 3. `opened.spans_opened()` is 5, because the clone shares the counter and the counter sits at 6 after five `fetch_add` calls. `6 - 1` is 5.

`Memory::new()` is empty. `research_team(100, ...)` pushes 100 notes of the form `{index}:ok` and returns 100. That return value is discarded. `research_team(1, ...)` with a new `FakeModel { reply: "kept" }` pushes `0:kept` and returns 101. `again` is 101.

The format string is `trace: ok events=3 spans=5 team=101`.

The notes in this demo are `ok` and `kept`. They are the mock replies. They are not the user prompt. `FakeModel` ignores the prompt. The prompt of the turn was `"hello"`. The prompt of a team thread is `memory.last()` or `"first"`. Neither string is what `push` stores. `push` stores the index and the reply.

`research_team` does not enter a span. The subscriber is not installed around the team. The span count stays 5. The test `demo_names_the_team` asserts the line starts with `trace: ok events=3 ` (the prefix includes the space after `3`) and that the line contains `team=101`. It does not assert `spans=5` by itself. The format string and the counts produce `spans=5` in the returned line.

The binary prints that line. No key. No socket. No file under a trace directory.

### Rust: `with_default` and a cloned counter

`tracing::subscriber::with_default(subscriber, closure)` sets the thread-local subscriber for the closure and restores the previous one afterward. The subscriber is moved in. The clone `opened` is how `demo` reads the count after the closure. Without the clone, the `Arc` would have moved and `spans_opened` would be unreachable. `Clone` on `CountingSubscriber` is derived, and the only field is `Arc<AtomicU64>`, so the clone is a new subscriber struct pointing at the same atomic.

`format!` uses `{reply}` by name and `{}` for `sink.len()` and `opened.spans_opened()` in order. `team={again}` interpolates the length from the second `research_team`. The spaces in the format string are the spaces in `trace: ok events=3 spans=5 team=101`.

## The turn, walked with the secret from the test

The test builds `CountingSubscriber::new`, clones it into `watched`, and uses an empty sink. The prompt is `sk-live-0123456789abcdef`. The model reply is `ok`. `with_default` runs `mock_turn`.

`safe_field` returns `"request"`. The turn span's `event` field is that word. `emit("request")` pushes `"request"`. `complete` returns `ok` and drops the secret on the floor inside `let _ = prompt`. `emit("response")` pushes `"response"`. The gateway span's `event` field is `"dispatch"`. `emit("gateway")` pushes `"gateway"`.

The assertions are `reply` equals `"ok"`, `sink` equals `vec!["request", "response", "gateway"]`, `watched.spans_opened()` is at least 3, `safe_field(secret)` equals `"request"`, and `format!("{sink:?}")` does not contain `sk-`. The secret is not in the sink. The span field the source built for the turn is `"request"`.

Five `new_span` calls happen in that `mock_turn`. The test's bound is `>= 3`. A subscriber that counted only the three `emit` spans would pass that one assertion. This subscriber also counts the turn span and the gateway span, so the value `demo` prints is 5.

## The team, walked as 100 then 1

Start with `Memory::new`. `len` is 0. `last` is `None`.

`research_team(100, &memory, &FakeModel { reply: "found" })` enters `thread::scope`. Index 0 reads `last`. The vec is still empty unless a previous program pushed, which it did not. The scoped thread's prompt becomes `"first"` if that `Option` is `None`. `complete` returns `found` either way. The push is `{index}:found`. Indexes 1 through 99 do the same parent `last` and the same push of their own index. `thread::scope` joins all 100. `memory.len()` is 100. The function returns 100. The test asserts both.

`research_team(1, &memory, &FakeModel { reply: "kept" })` runs next. The parent `last()` clones one of the notes already stored. The thread ignores it. `complete` returns `kept`. The push is `0:kept`. `len` is 101. The function returns 101. `last().unwrap()` is `0:kept`, which ends with `kept`. The test asserts the 101 and the suffix `kept`.

`demo` is the same shape with replies `ok` and `kept`, and with the turn recorded before the team. The team length is still 101. The turn's sink length is 3. The span count is 5. The line is `trace: ok events=3 spans=5 team=101`.

## Tests

The new tests are `steps/ch50-tracing/crates/harness-core/tests/spans.rs`. The package also builds the copied harness tests, including `a2a.rs`. The nextest line below is the package run.

`a_turn_records_three_event_names_and_not_the_prompt` installs `CountingSubscriber` with `with_default` and runs `mock_turn` on `FakeModel { reply: "ok" }` with the prompt `sk-live-0123456789abcdef`. It asserts the reply equals `"ok"`. It asserts `sink` equals `vec!["request", "response", "gateway"]`. It asserts `watched.spans_opened() >= 3`. It asserts `safe_field(secret)` equals `"request"`. It asserts `format!("{sink:?}")` does not contain `sk-`.

`one_hundred_agents_learn_across_two_runs` calls `research_team(100, ...)` with `FakeModel { reply: "found" }`. It asserts that return equals 100 and `memory.len()` equals 100. It calls `research_team(1, ...)` with `FakeModel { reply: "kept" }`. It asserts that return equals 101 and `memory.last().unwrap()` ends with `kept`.

`demo_names_the_team` binds `demo()` to `line`. It asserts `line.starts_with("trace: ok events=3 ")`. It asserts `line.contains("team=101")`. The failure messages print `{line}`.

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

The binary records one turn and a team of 101 notes. No key. No socket. `examples/demo.rs` is the eval runner. The line above is `chapter_demo.rs`.

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

Subscriber install is outside both samples. `CountingSubscriber::new` and `with_default` run before either `bench_function` body is timed. `descriptions.json` says the same thing. `FakeModel { reply: "ok" }` is built outside the samples.

`event_without_span` is `b.iter` of `complete` on `"hello"` passed through `black_box`, then `black_box` of the reply. The reply is `ok`. The function does not enter a span. `mock_turn`, `research_team`, and the sink allocation are not called. The prompt is not written to a sink. The id is `50/event_without_span`. The mean cell is `18.1 ns`.

`span_per_event` is `b.iter` of a `Vec` with capacity 3, then `mock_turn` of the model and `"hello"` passed through `black_box`, then `black_box` of the reply and the sink. The turn span, the three `emit` spans, the gateway span, and one `complete` are inside the sample. That is five spans: turn, request event, response event, gateway, gateway event. The sink receives `request`, `response`, and `gateway`. The span field on the turn is the word `request`, not the prompt. `research_team` is not called. No file is written. The id is `50/span_per_event`. The mean cell is `236.4 ns`.

`descriptions.json` says a span per event has to add less than 0.1 milliseconds, and the chapter 1 budget of 1 millisecond still has to hold with tracing on. It says the 100-agent team is not in the sample. It says around 100 nanoseconds above `event_without_span` is the span tax the reading expects, and that one millisecond would still meet the chapter 1 budget. Those sentences are the file's reading of the sample. They are not a measured mean. The means are `18.1 ns` and `236.4 ns`. They are `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), section `50`.

`.github/scripts/run_bench_section.sh` runs `cargo bench -p ch50-tracing --bench spans --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10` for section 50. The same flags are in `steps/ch50-tracing/README.md`. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `trace: ok events=3 spans=5 team=101`.

## Pitfalls

The bench is one `complete` and one `mock_turn`. `research_team(100)` is the test and the demo. A number that spawns 100 threads is not `span_per_event`.

`event_without_span` still runs under `with_default`. The subscriber is installed. The sample does not enter a span. Quoting only `span_per_event` hides the floor. Both means are the comparison. The install is outside both.

`safe_field` returns `"request"` for every prompt. A later edit that puts `text` into the span field would put the secret into the attributes. `new_span` currently drops attributes. The test also checks the sink, which `emit` fills with names. The secret test fails if the sink debug text contains `sk-`.

`spans_opened` subtracts 1 from a counter that starts at 1. Reading it before any `new_span` returns 0. `demo` reads it after `mock_turn`, which opens five. The test bound is `>= 3`.

`research_team` reads `last` on the parent, inside the spawn loop. Sibling threads may already have pushed. The test does not assert those prompts. It asserts lengths, and it asserts that the note after the second call ends with `kept`. The second call is sequenced after the first `thread::scope` returns.

`FakeModel` ignores the prompt. A note that equaled the user prompt would be a different function. `format!("{index}:{note}")` stores the reply. In `demo` the replies are `ok` and `kept`.

`CountingSubscriber` does not export. Pointing it at a collector would be a socket this file does not open. `headers_env` in the Hermes exporter is read at export time and never logged or stored. This subscriber has no header map.

`moa.save_traces` writes a file under the trace dir when the flag is on, and `_traces_enabled_and_dir` returns `None` when the flag is off. `demo` does not read that flag and does not write that file. The sink is a `Vec` of three names on the stack of `demo`.

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

The stub panics.

```rust
pub fn chapter_50_span_finishes_turn() -> bool {
    todo!("chapter 50")
}
```

`todo!("chapter 50")` is the panic. The test asserts `chapter_50_span_finishes_turn()` is true. In the step crate that is `mock_turn` under `CountingSubscriber` returning the fake reply after the request, response, and gateway names are in the sink. The stub stays `todo!`. Do not paste `spans.rs` into `my-harness/`.

## Chapter summary

- **Subscriber.** `new_span` counts from 1. `spans_opened` is the counter minus one. The other methods do not store fields.
- **Field.** `safe_field` returns `request`. The prompt is not a span field and is not in the sink.
- **Turn.** The sink holds `request`, `response`, and `gateway`. Five spans open: turn, request event, response event, gateway, gateway event.
- **Team.** `research_team(100)` then `research_team(1)` leaves 101 notes. The second call sees the earlier notes. The note text is the mock reply.
- **Model.** `FakeModel` answers `ok` or `kept` in `demo`. The line is `trace: ok events=3 spans=5 team=101`.
- **Time.** `event_without_span` mean `18.1 ns`. `span_per_event` mean `236.4 ns`. Those cells are `mean.point_estimate`. The Criterion `time:` line is the slope.

[personal note: five spans, and the sink held three names]

## What's next

Chapter 51 is the server capstone. It measures load and tokens. Chapter 52 copies this harness, not chapter 51.
