# Chapter 18: Providers

The first fake model answers `no`, the route moves on, and the turn comes back `pong` from the fallback, with the client id still `1` after the key rotates from `sk-one` to `sk-two`. That is the part this chapter builds, and it isn't wired into the loop yet, and `drive_loop` still calls the chapter 9 provider. `agent.rs` never names `adapters`. The tests and the demo call the module, and a later edit is what connects the route, which is worth saying here so the gap at the end of the walk isn't a surprise.

Chapter 1 can call OpenRouter, and every later harness copy still can. A harness that is serious about more than one model needs a native body beside that client, a way to fall through when a route fails, a way to rotate a key without building a new HTTP client, and a cheaper model for the summaries it will soon want. `Msg` is the message this module sends; two JSON bodies cover the OpenAI-compatible shape and the native Anthropic shape, and Bedrock sits beside them. The side model is for summaries, not the user turn, unless that turn asked for one.

Chapter 19 will stream these adapters; Chapter 34 asks the side model for a summary when it compresses. Reflection and compression should be the cheap calls, not a second run of the user-facing model, which is how a continuously improving harness keeps the lesson cheaper than the task.

Another impl of the same trait is how a new provider would appear, once something implements it. This file returns `serde_json::Value` instead, so the two request shapes can differ without two HTTP stacks; an enum is the TLS decision. The client id is the value you can assert stayed put across rotation, which is the test that the pool was reused.

The step is `steps/ch18-providers`, package `ch18-providers`, library `harness_core`, the module `adapters` on a copy of chapter 17. `cargo run -p ch18-providers` prints `reply: pong` and `client: 1`. `my-harness/tests/ch18.rs` is an invalid route falling through to the fallback, with rotation keeping the same client.

| Section | Step | What it adds |
|---|---|---|
| 18 | `steps/ch18-providers` | Anthropic and Bedrock bodies, a route, key rotation, outbound repair |

> **How Hermes does it**
>
> I read `agent/anthropic_adapter.py` (`build_anthropic_kwargs`), `agent/bedrock_adapter.py` (`build_converse_kwargs`), `agent/chat_completion_helpers.py` (`_provider_preferences_for_agent`, `_validated_openrouter_provider_sort`, `_OPENROUTER_PROVIDER_SORT_VALUES`), `agent/agent_init.py` (`fallback_model`), `agent/credential_pool.py` (`PooledCredential`, `mark_exhausted_and_rotate`), `agent/message_sanitization.py` (`uniquify_tool_call_ids`, `close_interrupted_tool_sequence`), `agent/agent_runtime_helpers_placeholders.py` (`_INTERRUPTED_PLACEHOLDER`), and `agent/ssl_verify.py` (`resolve_httpx_verify`, `install_truststore`) on Hermes `main` at `cf71b1aa`.
>
> `build_anthropic_kwargs` sends `model`, `messages`, and `max_tokens`. `system` is set only when it is non-empty. `max_tokens` has to be positive. `build_converse_kwargs` sends `modelId`, `messages`, and `inferenceConfig`, and the default `max_tokens` in that signature is 4096. Provider preferences keep `only`, `ignore`, `order`, `sort`, `require_parameters`, and `data_collection`, and drop empty values. Sort is `throughput`, `latency`, or `price`. Anything else is ignored. `require_parameters` is kept only when it is true. Duplicate tool-call ids gain a `_dN` suffix starting at 2. A transcript that ends on a tool row gets an assistant row whose text is `[interrupt: no assistant output for this turn]`. TLS uses the OS trust store through `truststore`. `ssl_verify: false` is for local development.
>
> **Adopted.** The Anthropic field names, including omitting an empty system prompt. The Bedrock field names `modelId` and `inferenceConfig.maxTokens`. The six preference keys, the three sort values, and dropping a false `require_parameters`. The `_d2` suffix. The interrupted-placeholder sentence. Refusing to disable TLS unless a dev flag is set.
>
> **Adapted.** There is no HTTP client in this crate. The per-model output-ceiling table is not ported. A `max_tokens` of 0 is an error. Fallbacks are a `models` array on the route object. I did not find Hermes writing that array inside `build_anthropic_kwargs` or `build_converse_kwargs`. Hermes keeps the list on `fallback_model`. Rotation advances an index in one `Pool` and the client id stays `1`. The side model is a string choice, not a second request.
>
> **Deferred.** OAuth transforms, thinking budgets, and cache points stay out. Live certificate verification stays out with the socket. Chapter 19 strips think blocks from a stream.

## What this chapter decides

`anthropic_body("claude", "be brief", messages, 128)` has `model`, `max_tokens`, `system`, and `messages`, and an empty system string is omitted, and `max_tokens` of 0 is `ProviderError::MaxTokens`.

`bedrock_body` uses `modelId` and `inferenceConfig.maxTokens`; the test passes 4096, which is the default on `build_converse_kwargs`; an empty system string is omitted here too.

`provider_preferences` drops empty lists, and `sort: Some("Price")` becomes `"price"`, and `sort: Some("fastest")` is omitted, and `require_parameters: false` is omitted. `data_collection: Some("deny")` is kept.

`openrouter_route` puts the model list on `models` and the preference map on `provider`. `choose_next` with no failure returns the first model; `choose_next` after `"primary"` returns `"fallback"`.

`Pool::new` has client id 1, and `mark_exhausted_and_rotate` moves from `sk-one` to `sk-two` and the client id is still 1.

`side_model(false, "primary", "side")` is `"primary"`; `side_model(true, ...)` is `"side"`.

`uniquify_tool_call_ids` turns `["call_0", "call_0", "call_1"]` into `["call_0", "call_0_d2", "call_1"]`. `close_interrupted` appends the placeholder when the last row is `tool`, and does nothing when the last row is already `assistant`.

`resolve_tls(true, _)` is `OsTrust`; `resolve_tls(false, false)` is an error; `resolve_tls(false, true)` is `DevInsecure`, and this crate does not build an `SslContext`.

The demo fails the first fake model and takes the fallback, and the reply is `pong`, and the client id is 1. No key.

## 18.1 The module is `adapters`, and `drive_loop` does not call it

The file is `steps/ch18-providers/crates/harness-core/src/adapters.rs`. Its module comment is the whole job of the file.

```rust
//! Book chapter 18. One turn can build an Anthropic body, a Bedrock body,
//! or an OpenRouter route. Rotating a key keeps the same client id.
//! The side model stays off unless the turn asked for a summary.
```

`lib.rs` declares `pub mod adapters;`; the module is public, so an integration test can write `harness_core::adapters::Pool`. The `pub use` list at the bottom of `lib.rs` does not re-export these names; you go through the module.

`provider.rs` is already in the same crate, declared as `mod provider;` with no `pub`, and it has been there since chapter 9. Leave that file alone, and do not delete `provider.rs` to "make room": that file is the OpenRouter client the inherited tests call. The saved evals build a `ScriptedProvider` from it; removing the file would take the trait out from under `drive_loop`.

The trait the loop already depends on is small.

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    async fn complete(&self, messages: &[Message]) -> Vec<Event>;
    fn calls(&self) -> u64;
}
```

> **Rust sidebar: a trait is a list of methods**
>
> `ScriptedProvider` and the live client in `live.rs` both implement `Provider`, and `drive_loop` takes `&dyn Provider` so the function doesn't name which type. `async_trait` makes `async fn complete` callable through that pointer, the same reason `Tool` used it in [chapter 3](chapter-03-tools.md). `adapters.rs` defines no trait: Anthropic and Bedrock return a `serde_json::Value`, and there is no `impl Provider for` an Anthropic type in this file.

As the opening said, `agent.rs` never names `adapters`. `drive_loop` is the private function `run_loop`, `run_loop_driven`, and `run_loop_with_inbox` all end up in, and its signature still takes the chapter 9 trait object.

```rust
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

Inside the loop the model call is `provider.complete(&send)`, in the `tokio::select!` that also watches the cancel token. `budget_wrap_up` and `soft_wrap_up` each call `provider.complete` the same way. Those three calls are every model call in `agent.rs`. None of them call `anthropic_body`, `choose_next`, `mark_exhausted_and_rotate`, or `uniquify_tool_call_ids`.

That gap is intentional. The inherited evals look at `Message` values from `ScriptedProvider`, so I am leaving the module unwired and the loop keeps the client those evals already assert on. The new tests construct the bodies themselves.

`Message` and `Msg` are different types; `Message` in `message.rs` uses the `Role` enum and also carries `tool_calls` and `tool_call_id`. `Msg` is local to `adapters.rs`: a `String` role and a `content` string. A function that accepts `&[Msg]` cannot take the vec `drive_loop` pushes into; the repair functions repair that smaller transcript.

The callers of `adapters` in this step are `tests/providers.rs`, `benches/providers.rs`, and `src/bin/chapter_demo.rs`, and those crates see `pub` items. `wire_messages` and `chosen_static` stay private, and the public functions call them.

Two constants sit at the top of the file.

```rust
pub const INTERRUPTED_PLACEHOLDER: &str = "[interrupt: no assistant output for this turn]";
pub const CLIENT_ID: u64 = 1;
```

`INTERRUPTED_PLACEHOLDER` is the sentence `close_interrupted` appends; `CLIENT_ID` is the only client id `Pool::new` ever stores, and nothing in the file adds one to it.

## 18.2 `Msg`, two enums, and the TLS decision

A message in this module is two owned strings.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Msg {
    pub role: String,
    pub content: String,
}

impl Msg {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }
}
```

> **Rust sidebar: `impl Into<String>`**
>
> `Msg::new` takes `impl Into<String>`, so a literal allocates and an existing `String` moves. The test calls `Msg::new("user", "hi")`. The same bound is on `open_session` in [chapter 15](chapter-15-limits.md).

`Debug` is what `{:?}` prints. `Clone` is an explicit copy of both strings, and `PartialEq` and `Eq` are `==`. The tests compare `Msg` values with `assert_eq!` on the content of the last row, which needs `PartialEq` on the string, and the error enum below derives the same pair so a test can compare `ProviderError::InsecureTls` by value.

The failures this file can report are an enum with two variants and no data.

```rust
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProviderError {
    #[error("max_tokens must be positive")]
    MaxTokens,
    #[error("tls verification disabled without the dev flag")]
    InsecureTls,
}
```

> **Rust sidebar: an enum is one of several shapes**
>
> `ProviderError` is either `MaxTokens` or `InsecureTls`. `#[derive(Error)]` from `thiserror` is the text `Display` prints, and the TLS test compares the variant with `assert_eq!`. A later error that needs a status code would add `{ status: u16 }` inside the variant. Enums are in [chapter 1](chapter-01-first-contact.md).

The TLS result is a second enum; it is the success type, so it does not implement `Error`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlsDecision {
    OsTrust,
    DevInsecure,
}

pub fn resolve_tls(ssl_verify: bool, dev_insecure: bool) -> Result<TlsDecision, ProviderError> {
    if ssl_verify {
        return Ok(TlsDecision::OsTrust);
    }
    if dev_insecure {
        Ok(TlsDecision::DevInsecure)
    } else {
        Err(ProviderError::InsecureTls)
    }
}
```

Three branches, and the first one returns before the second `if` runs.

`ssl_verify == true` returns `Ok(OsTrust)` immediately; the `dev_insecure` argument is not read, and `resolve_tls(true, true)` would still be `OsTrust`. The test calls `resolve_tls(true, false)` and expects `OsTrust`. It does not call the pair where both flags are true. The early return is what makes that pair the same answer.

`ssl_verify == false` and `dev_insecure == true` is `Ok(DevInsecure)`. This is the only way the function records that verification was turned off on purpose.

`ssl_verify == false` and `dev_insecure == false` is `Err(InsecureTls)`. The `else` is attached to the second `if`, so it runs only when we already failed the first `if`, and there is no fourth combination.

The function returns the enum and stops; it does not build a certificate store, an `SslContext`, or a client. The imports at the top of the file are `serde_json::{json, Map, Value}` and `thiserror::Error`; no TLS crate is named, and `DevInsecure` means "the caller set the flag.". It does not mean a socket was opened with verification off. The live handshake stays out with the socket, which is what the deferred line in the Hermes box already says.

## 18.3 Two request bodies, both a `Value`

Anthropic and Bedrock want different keys; I still want one return type, and the return type is `serde_json::Value`.

```rust
pub fn anthropic_body(
    model: &str,
    system: &str,
    messages: &[Msg],
    max_tokens: u32,
) -> Result<Value, ProviderError> {
    if max_tokens == 0 {
        return Err(ProviderError::MaxTokens);
    }
    let mut body = json!({
        "model": model,
        "max_tokens": max_tokens,
        "messages": wire_messages(messages),
    });
    if !system.is_empty() {
        body["system"] = json!(system);
    }
    Ok(body)
}
```

`model` and `system` are borrowed `&str`; the function does not take ownership of the caller's strings. `messages` is a slice, `&[Msg]`, so the caller keeps the vec; `max_tokens` is a `u32`; a `u32` cannot be negative. Zero is the only non-positive value the type can hold, and zero is the error. The per-model ceiling table is not in this function. Any positive `u32` is accepted, including a number far past what an API would allow. The check is the sign, written as `== 0` because the type has no sign bit.

`json!` builds the object with three keys already filled. `wire_messages` runs before the object exists, because its return value is one of the fields. Then, only when `system` is non-empty, the function writes a fourth key.

```rust
pub fn bedrock_body(
    model: &str,
    system: &str,
    messages: &[Msg],
    max_tokens: u32,
) -> Result<Value, ProviderError> {
    if max_tokens == 0 {
        return Err(ProviderError::MaxTokens);
    }
    let mut body = json!({
        "modelId": model,
        "messages": wire_messages(messages),
        "inferenceConfig": { "maxTokens": max_tokens },
    });
    if !system.is_empty() {
        body["system"] = json!(system);
    }
    Ok(body)
}
```

The shape of the function is the same, and the keys aren't. Bedrock's model field is `modelId`, the cap is nested as `inferenceConfig.maxTokens`, and the same zero check returns `ProviderError::MaxTokens`. The interesting decision is the empty system string: it is omitted, which the Bedrock test locks by passing `""` and checking the key is absent. `4096` is the value that test passes for `maxTokens`, the default on `build_converse_kwargs`.

> **Rust sidebar: a `Value` is an enum, and the body is one on purpose**
>
> One function returns `serde_json::Value` for both APIs, because a struct with `max_tokens` couldn't also have `inferenceConfig.maxTokens`, and Bedrock has no `max_tokens` key. JSON as a value is in [chapter 3](chapter-03-tools.md). `body["max_tokns"]` compiles, and the test's `anthropic["max_tokens"]` is what fails when the key is wrong. Assigning `body["system"]` inserts a missing key, while `get("system")` returns `None` when the key was left out, which is what the Bedrock test checks. `let mut body` is required because that assignment changes the object.

`wire_messages` is the private helper both bodies share.

```rust
fn wire_messages(messages: &[Msg]) -> Vec<Value> {
    messages
        .iter()
        .map(|message| json!({"role": message.role, "content": message.content}))
        .collect()
}
```

> **Rust sidebar: an iterator is a trait you usually don't call**
>
> `.iter().map(...).collect()` walks the slice when `collect` asks, `json!` allocates one object per message, and the `Vec<Value>` is owned while the original `&[Msg]` is only borrowed. `choose_next` uses `position`, which stops at the first match and returns `Option<usize>`.

Each object has exactly two keys, `role` and `content`. Tool-call ids, raw arguments, and the `Role` enum never enter this vec, because `Msg` doesn't have those fields. The Anthropic test builds one message with `Msg::new("user", "hi")` and asserts `messages[0].role == "user"`, which is enough to prove `wire_messages` ran. An empty slice collects into an empty `Vec`, and `json!` stores that as an empty array.

## 18.4 Preferences, the route, and the next model

The preference object is a struct first, then a map, then one field on the route.

```rust
#[derive(Debug, Clone, Default)]
pub struct Prefs {
    pub only: Vec<String>,
    pub ignore: Vec<String>,
    pub order: Vec<String>,
    pub sort: Option<String>,
    pub require_parameters: bool,
    pub data_collection: Option<String>,
}
```

`Default` fills this in without a handwritten constructor, and `Vec::default()` is empty, and `Option::default()` is `None`, and `bool::default()` is `false`. The bad-sort case in the test writes `sort: Some("fastest".into())` and then `..Prefs::default()`, so the other five fields are those empty values. `..` is struct update, and every field not named in the literal comes from the value after the dots.

Sort is the only field with an allow-list.

```rust
pub fn validated_sort(raw: &str) -> Option<&'static str> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "throughput" => Some("throughput"),
        "latency" => Some("latency"),
        "price" => Some("price"),
        _ => None,
    }
}
```

`trim` drops leading and trailing whitespace; `to_ascii_lowercase` allocates a new `String` and changes only `A`–`Z`. `as_str` borrows that temporary so `match` can compare it to the literals. The three `Some` arms return string literals, which have type `&'static str`; they do not borrow `raw`. The temporary lowercase string dies at the end of the statement, and that is fine, because the returned `&str` points at the binary, not at the temporary.

`"Price"` trims to itself, lowercases to `"price"`, and hits the third arm, and `"fastest"` hits `_` and returns `None`. Whitespace and `"LATENCY"` would hit the first and second arms because of `trim` and the lowercase, and `_` is every other string, including `""`; the match stays exhaustive.

`match` is how you take an enum, or a string, apart; here the thing being matched is `&str`, and each arm is one pattern. The first matching arm runs, and the rest are skipped; there is no fall-through.

```rust
pub fn provider_preferences(prefs: &Prefs) -> Map<String, Value> {
    let mut out = Map::new();
    if !prefs.only.is_empty() {
        out.insert("only".into(), json!(prefs.only));
    }
    if !prefs.ignore.is_empty() {
        out.insert("ignore".into(), json!(prefs.ignore));
    }
    if !prefs.order.is_empty() {
        out.insert("order".into(), json!(prefs.order));
    }
    if let Some(sort) = prefs.sort.as_deref().and_then(validated_sort) {
        out.insert("sort".into(), json!(sort));
    }
    if prefs.require_parameters {
        out.insert("require_parameters".into(), json!(true));
    }
    if let Some(policy) = prefs
        .data_collection
        .as_ref()
        .filter(|value| !value.is_empty())
    {
        out.insert("data_collection".into(), json!(policy));
    }
    out
}
```

`Map<String, Value>` is the object variant before it is wrapped in a `Value`; `Map::new()` is empty. A key enters the map only from one of these six `if`s, and an empty list never becomes `"ignore": []`. A missing key is how the wire stays quiet.

`only`, `ignore`, and `order` share one pattern: skip the empty vec, otherwise `insert` the whole vec as a JSON array. `json!(prefs.only)` clones the strings into the `Value`, and the `Prefs` borrow stays put. The test builds `only` as `["anthropic"]`, `ignore` as `[]`, and `order` as `["anthropic", "amazon"]`, and the asserts that exist are on `sort`, `require_parameters`, and `data_collection`. The empty-`ignore` path runs and isn't locked by its own `get`. I am leaving that visible, because an assert on `sort` doesn't prove the empty list was dropped, and `if !is_empty()` is the whole policy for those three keys.

The sort line is a chain, and `prefs.sort` is `Option<String>`. `as_deref()` turns `Option<String>` into `Option<&str>` by borrowing the string inside. `and_then(validated_sort)` calls `validated_sort` only on `Some`, and flattens the `Option` that `validated_sort` itself returns; `Some("Price")` becomes `Some("price")`. `Some("fastest")` becomes `None`, and the `if let` does not insert, and `None` stays `None` and also does not insert. The inserted JSON is `json!(sort)` where `sort` is the `&'static str` from the match, so the map owns a fresh JSON string and does not keep a borrow of `Prefs`.

`require_parameters` inserts only when the bool is true, and the inserted value is `json!(true)`, not the bool variable, so `false` leaves the key out. The test locks that absence with `map.get("require_parameters").is_none()`.

`data_collection` uses `as_ref` so the `Option<String>` is borrowed, then `filter` drops the `Some` when the string is empty. `Some("deny")` is kept, which is what the test locks, and an empty string becomes `None` the same way a missing value does. `filter` keeps `Some` only when the closure returns true.

```rust
pub fn openrouter_route(models: &[String], prefs: &Prefs) -> Value {
    json!({
        "models": models,
        "provider": provider_preferences(prefs),
    })
}
```

The route is always both keys; `provider_preferences` returns the map, and `json!` accepts it because `Map<String, Value>` can be serialized as a JSON object. An all-default `Prefs` still produces `"provider": {}`: the key is present and the inner object is empty. The preference asserts happen on the map before it is wrapped. Choosing the next name doesn't look at that map.

```rust
pub fn choose_next<'a>(models: &'a [String], failed: Option<&str>) -> Option<&'a str> {
    let Some(failed) = failed else {
        return models.first().map(String::as_str);
    };
    let index = models.iter().position(|model| model == failed)?;
    models.get(index + 1).map(String::as_str)
}
```

> **Rust sidebar: the lifetime on the returned name**
>
> `'a` is a name for how long this borrow lasts. `models: &'a [String]` says the slice is borrowed for `'a`, and the return type `Option<&'a str>` says the string inside the `Option` is borrowed for that same `'a`. The returned `&str` points into one of the `String`s in the slice, so it doesn't point at `failed`. `failed` is `Option<&str>` with a lifetime the signature leaves unnamed, because nothing returned borrows it. It is only compared. When the function returns, the caller must still be holding `models`. Drop the vec and the `&str` would dangle, and the compiler rejects that. `run_turn` hits this and maps the name onto a `'static` literal before its local vec dies. That function is below. `let Some(failed) = failed else { ... }` is `let`-`else`. Matching `Some` binds the inner `&str` and continues. Matching `None` runs the `else` block, which returns. After that line, `failed` is a `&str`, not an `Option`. `?` on an `Option` means "if this is `None`, return `None` from the function." `position` returns `None` when no element equals `failed`. The function then returns `None` without looking at `index + 1`.

`failed == None` returns the first name, or `None` when the slice is empty, and `first()` is `Option<&String>`. `map(String::as_str)` turns that into `Option<&str>`. `String::as_str` is a function pointer passed to `map`; it borrows the `String`.

`failed == Some(name)` scans with `position`; the closure is `|model| model == failed`, and `model` is `&String`. Comparing it to `&str` works because `String` implements equality with `str`, and the scan stops at the first hit. `get(index + 1)` is `Option<&String>`, and `map(String::as_str)` borrows it. When `failed` is the last name, `index + 1` is past the end and the function returns `None`. When `failed` isn't in the list, `position` is `None` and `?` returns `None`. The test calls `choose_next` with `Some("primary")` on `["primary", "fallback"]` and expects `Some("fallback")`, and with `None` expects `Some("primary")`. The empty slice, a missing name, and the last name are the other `None` returns in the body.

`choose_next` picks the next name; it doesn't read `Prefs`, and it doesn't call a model, so a failure you haven't already classified still needs chapter 14.

## 18.5 One pool, one client id, and the side model

The pool owns the keys, and the client id is a field that `new` sets and no method writes again.

```rust
#[derive(Debug, Clone)]
pub struct Pool {
    client_id: u64,
    keys: Vec<String>,
    cursor: usize,
}

impl Pool {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            client_id: CLIENT_ID,
            keys,
            cursor: 0,
        }
    }

    pub fn client_id(&self) -> u64 {
        self.client_id
    }

    pub fn current(&self) -> &str {
        self.keys.get(self.cursor).map(String::as_str).unwrap_or("")
    }

    pub fn mark_exhausted_and_rotate(&mut self) -> &str {
        if self.keys.len() > 1 {
            self.cursor = (self.cursor + 1) % self.keys.len();
        }
        self.current()
    }
}
```

The three fields are private; another crate can construct a pool and call the methods. It cannot write `pool.cursor = 0` and it cannot swap the key vec; `new` takes ownership of the `Vec<String>`. The caller that built `vec!["sk-one".into(), "sk-two".into()]` gives that vec away. `cursor` starts at 0, so `current` is the first key.

`client_id` returns the `u64` by copy, and a `u64` is `Copy`. Returning it does not borrow `self`, which is why a test can call `client_id()` before and after a `&mut self` method without holding a borrow across the mutation.

`current` borrows. `get` returns `Option<&String>` and does not panic when `cursor` is out of range; `map(String::as_str)` borrows the key. `unwrap_or("")` substitutes an empty literal when the option is `None`. The signature `-> &str` with `&self` ties that borrow to the pool. The literal `""` is `'static`, so it is allowed to pretend it came from `self`. An empty `keys` vec hits that literal. A `cursor` past the end would too. `new` starts the cursor at 0, and `mark_exhausted_and_rotate` only moves it when `len() > 1`, using modulo `len()`, so a pool that `new` built stays in range. The empty-pool branch is still in `unwrap_or`. No test builds `Pool::new(vec![])`.

`mark_exhausted_and_rotate` needs `&mut self` because it writes `cursor`. The `if` has one condition: more than one key. A single key leaves `cursor` at 0 and returns that same key, and zero keys skip the `if` and `current` returns `""`. Two or more keys set `cursor` to `(cursor + 1) % len`. The `%` wraps the last index back to 0. Then the method returns `self.current()`, which is a borrow of whichever key the cursor now names.

That returned `&str` borrows the pool; you cannot hold it in a local and call `mark_exhausted_and_rotate` again. The second call wants `&mut self`, and the local is still borrowing `self`. The test does not hold the borrow. It compares the return value inside `assert_eq!`, and the temporary dies at the end of the statement. The bench does the same with `black_box(key)` inside the sample, so the borrow ends before the next sample.

The test builds two keys, checks `client_id()` is `CLIENT_ID` (the constant `1`), rotates once, expects `"sk-two"`, and checks `client_id()` again. The field was not assigned in the rotate method. Both reads see `1`. A third key, the wrap from last back to first, and the single-key `if` that does not fire are what the bench and the source cover beyond that test. The bench's pool is `sk-one`, `sk-two`, `sk-three`, and each sample calls this method. Across samples the cursor walks 1, 2, 0, 1, ... The client id field is never rewritten. No HTTP client is constructed. There is no `reqwest` type in this file.

```rust
pub fn side_model<'a>(asked: bool, primary: &'a str, side: &'a str) -> &'a str {
    if asked {
        side
    } else {
        primary
    }
}
```

Both inputs share the lifetime `'a`, and the return value uses that same lifetime. The function returns one of the two borrows. It does not allocate, and it does not format a new name. `asked == false` is the `else` branch, which the test calls as `side_model(false, "primary", "side")` and expects `"primary"`. `asked == true` returns `side`. The test locks that with the same two literals.

The bool is the whole policy. The function does not look at whether the turn already printed text. A summary runs when the caller passes `true`. The user path in this chapter passes `false`. Nothing in `drive_loop` passes either. The function sits next to the route helpers so a later turn can ask for it.

## 18.6 Duplicate ids, then a dangling tool row

K4 in this chapter is two functions. They share no types with the route. They repair a batch of id strings, and a small `Vec<Msg>`, before a request would be sent. Nothing sends it.

```rust
pub fn uniquify_tool_call_ids(ids: &mut [String]) {
    let mut seen: Vec<String> = Vec::new();
    for id in ids.iter_mut() {
        if id.is_empty() {
            continue;
        }
        if !seen.iter().any(|got| got == id) {
            seen.push(id.clone());
            continue;
        }
        let mut n = 2u32;
        let renamed = loop {
            let candidate = format!("{id}_d{n}");
            if !seen.iter().any(|got| got == &candidate) {
                break candidate;
            }
            n += 1;
        };
        seen.push(renamed.clone());
        *id = renamed;
    }
}
```

The argument is `&mut [String]`, a mutable slice. The caller keeps the `Vec`. The function writes through the elements. `iter_mut` yields `&mut String`, one slot at a time. `for` is the iterator loop. Each turn of the loop is one id.

An empty string hits `continue`. It is not pushed into `seen`, and it is not renamed. A second empty string also continues. The two empties stay empty. They are not treated as a collision. The test's ids are all non-empty, so this branch is uncalled there. It is the first branch in the loop.

`seen` is a `Vec<String>` this function owns. The check is `seen.iter().any(|got| got == id)`. `any` is an iterator method. It runs the closure until one element returns true, or until the vec ends. `got` is `&String`. Comparing it to `&mut String` (the `id`) uses string equality. On the first sight of a value, `any` is false, the id is cloned into `seen`, and `continue` moves to the next slot. The clone is deliberate. `seen` has to own its strings while the loop mutates later slots of the same slice. Borrowing the slot would freeze it.

A later slot whose value is already in `seen` falls through to the rename. `n` starts at `2u32`. The suffix is `_d2` for the first collision, which is what the test locks: `["call_0", "call_0", "call_1"]` becomes `["call_0", "call_0_d2", "call_1"]`. The first `call_0` was stored. The second compares equal, so it is renamed. `call_1` is new, so it is stored as itself. The `_d2` looks off by one if you expected `_d1`. The literal in the source is `2u32`. A second collision of the same id becomes `_d3`.

The `loop` is an expression. `format!` allocates `candidate`. `any(|got| got == &candidate)` asks whether that exact string is already in `seen`. The first free candidate `break`s out of the loop, and the value of the loop is that `String`. If `call_0_d2` is already one of the original ids, `n` becomes 3 and the candidate is `call_0_d3`. The test does not build that input. The inner `if` is why the suffix is a search and not a hard-coded `_d2`. There is no upper bound in the source. `n` is a `u32` that increments by one each miss.

After the loop, `seen.push(renamed.clone())` records the new name, and `*id = renamed` writes it into the slot. `*id` dereferences the `&mut String`. The move puts `renamed` into the caller's vec. `seen` keeps the clone. Two allocations for one rename: the `format!`, and the clone. The original id string in the slot is dropped by the assignment.

```rust
pub fn close_interrupted(messages: &mut Vec<Msg>) -> bool {
    let Some(last) = messages.last() else {
        return false;
    };
    if last.role != "tool" {
        return false;
    }
    messages.push(Msg::new("assistant", INTERRUPTED_PLACEHOLDER));
    true
}
```

`last()` returns `Option<&Msg>`. The `let`-`else` returns `false` on an empty vec. No row is appended. A non-empty vec binds `last` and continues. `last.role != "tool"` compares the owned `String` to a literal. `String` implements equality with `&str`, so this does not allocate. Any other role, including `"assistant"`, `"user"`, and `""`, returns `false` and leaves the vec alone.

When the last role is `"tool"`, the function pushes `Msg::new("assistant", INTERRUPTED_PLACEHOLDER)`. Both arguments are `&str`. `new` allocates two `String`s. The placeholder is the constant at the top of the file, which is the sentence `[interrupt: no assistant output for this turn]`. The push makes the new last row an assistant row. The function returns `true`.

Call it again on the same vec and the last role is now `"assistant"`. The second `if` returns `false`. The test does exactly that. It starts from an assistant row plus a tool row, expects `true`, expects the new last content to equal `INTERRUPTED_PLACEHOLDER`, then expects the second call to be `false`. An empty vec is the other `false`, and the test does not pass one.

This closes the transcript so the next row the caller appends is not `tool` followed later by a user row with a tool row still open at the end. The function does not insert a user row. It inserts one assistant row. It does not look at earlier tool rows, only `messages.last()`.

## 18.7 The fake turn the binary runs

The demo has to show a fallback reply without a socket. `FakeModel` is a name and a scripted reply. `run_turn` picks a name with `choose_next` and copies that model's reply.

```rust
#[derive(Debug, Clone)]
pub struct FakeModel {
    pub name: String,
    pub reply: String,
}

pub fn run_turn(models: &[FakeModel], failed_first: bool) -> (String, &'static str) {
    let names: Vec<String> = models.iter().map(|model| model.name.clone()).collect();
    let failed = if failed_first {
        names.first().map(String::as_str)
    } else {
        None
    };
    let chosen = choose_next(&names, failed).unwrap_or("");
    let reply = models
        .iter()
        .find(|model| model.name == chosen)
        .map(|model| model.reply.clone())
        .unwrap_or_default();
    (reply, chosen_static(chosen))
}

fn chosen_static(chosen: &str) -> &'static str {
    match chosen {
        "side" => "side",
        "fallback" => "fallback",
        _ => "primary",
    }
}
```

`models.iter().map(...).collect()` is the same pipeline as `wire_messages`, copying names into a local `Vec<String>`. `choose_next` needs `&[String]`, and `FakeModel` is not a `String`. The clone is the price of that second list. `failed_first == true` sets `failed` to the first name. `failed_first == false` sets `failed` to `None`, so `choose_next` returns the first name. The demo and the test both pass `true`.

`unwrap_or("")` turns a missing choice into an empty string. That happens when the name list is empty, or when the failed name is the last one, or when it is absent. `find` then walks the models for `name == chosen`. On `""` it finds nothing, `map` is not called, and `unwrap_or_default()` is an empty `String`. `String::default()` is `""`. The test's list has a fallback, so this path returns `"pong"` and never hits the default.

The return type is `(String, &'static str)`. The reply can be an owned `String` because it was cloned out of the model. The route name cannot be the `&str` from `choose_next`. That borrow points into `names`, and `names` is a local that dies when the function returns. `chosen_static` maps the borrowed name onto a literal before `names` is dropped.

> **Rust sidebar: `'static` is the lifetime of a literal**
>
> `"fallback"` in the source is stored in the binary, so a `&'static str` can be returned from any function and it doesn't borrow the arguments. `choose_next` returns `Option<&'a str>` tied to the local vec. Returning that from `run_turn` would be a dangling pointer, and the compiler stops the build. `chosen_static` is the copy of the decision into a literal. The match arms are `"side"`, `"fallback"`, and `_`. Every other string, including `""`, `"primary"`, and `"last"`, becomes `"primary"`. A successful pick of the first model and a failed pick both come back as the same `'static` word `"primary"`. The reply string is how you tell those apart. An empty reply with route `"primary"` is the `unwrap_or` path. A real primary reply with route `"primary"` is `failed_first == false`, or a failed name the match doesn't know. The test passes `primary` plus `fallback` and `failed_first: true`, so `chosen` is `"fallback"` and the match hits the second arm. The route comes back `"fallback"`. The reply comes back `"pong"`.

The binary is `src/bin/chapter_demo.rs`. It is a separate crate. It can call `pub` functions only.

```rust
fn main() {
    let mut pool = Pool::new(vec!["sk-one".into(), "sk-two".into()]);
    let _ = pool.mark_exhausted_and_rotate();
    let (reply, _route) = run_turn(
        &[
            FakeModel {
                name: "primary".into(),
                reply: "no".into(),
            },
            FakeModel {
                name: "fallback".into(),
                reply: "pong".into(),
            },
        ],
        true,
    );
    println!("reply: {reply}");
    println!("client: {}", pool.client_id());
}
```

`let _ = pool.mark_exhausted_and_rotate()` rotates and drops the returned key. After one rotate on two keys, `current` is `"sk-two"`. The binary prints `client_id`, which is still `1`. `run_turn(..., true)` marks the first fake model failed. The reply is the fallback's `"pong"`. `_route` binds the `'static` string and the `println` calls do not use it. The milestone lines are `reply: pong` and `client: 1`.

`failed_first` is a bool. `validated_sort` does not run in this binary. The invalid-sort check and the fallback pick share a test and do not share a function. `run_turn` calls `choose_next` only.

## 18.8 What each test locks

`tests/providers.rs` is six tests. Each one fails if the decision it names regresses.

| Test | What breaks if you remove it |
|---|---|
| `anthropic_and_bedrock_bodies_carry_the_fields_their_apis_require` | `anthropic_body("claude", "be brief", &hello(), 128)` can drop `model`, `max_tokens`, or `system`, and a zero `max_tokens` can succeed |
| `an_invalid_sort_is_dropped_and_the_next_model_is_the_fallback` | `sort: Some("Price")` can stay `"Price"`, or `"fastest"` can be inserted, and `choose_next` can stop returning `"fallback"` |
| `rotation_keeps_the_client_and_the_side_model_stays_off` | `client_id()` can leave `CLIENT_ID` after a rotate, or `side_model(false, ...)` can return the side model |
| `duplicate_tool_ids_and_a_dangling_tool_row_are_repaired` | `["call_0", "call_0", "call_1"]` can stay duplicated, or a trailing `tool` row can be left open |
| `tls_stays_on_the_os_store_unless_the_dev_flag_is_set` | `resolve_tls(false, false)` can stop being `Err(InsecureTls)`, or the dev flag can stop producing `DevInsecure` |
| `the_fake_turn_uses_the_fallback_model` | `run_turn` with `failed_first` true can stop returning `("pong", "fallback")` |

## 18.9 What the two benches time

The bench file is `benches/providers.rs`. The names `choose_route` and `rotate_key` are the `bench_function` strings, and they are the function names recorded for this chapter in `benchmarks/descriptions.json`. The measured means stay in the table further down. This section is the source those names time.

```rust
fn choose_route(c: &mut Criterion) {
    let models = vec![
        "primary".to_string(),
        "fallback".to_string(),
        "last".to_string(),
    ];
    c.bench_function("choose_route", |b| {
        b.iter(|| {
            let next = choose_next(black_box(&models), black_box(Some("primary")));
            black_box(next);
        });
    });
}
```

The three-name vec is built before the samples. The timed call passes `Some("primary")`, so the returned slot is `"fallback"`. `"last"` sits in the slice and is not the name that comes back. `black_box` is `std::hint::black_box`. It makes the optimizer treat the slice, the failed name, and the returned `Option<&str>` as used. A call this small can otherwise be computed once and deleted. The closure is what `b.iter` repeats. Sort validation and body building are outside this function.

```rust
fn rotate_key(c: &mut Criterion) {
    let mut pool = Pool::new(vec!["sk-one".into(), "sk-two".into(), "sk-three".into()]);
    c.bench_function("rotate_key", |b| {
        b.iter(|| {
            let key = pool.mark_exhausted_and_rotate();
            black_box(key);
        });
    });
}
```

`Pool::new` runs before the samples. One pool is reused. Each sample calls `mark_exhausted_and_rotate` and feeds the borrowed key to `black_box`. The borrow ends when the closure ends, which is what lets the next sample take `&mut self`. The cursor wraps from the last key to the first across samples. `client_id` is not assigned. `criterion_group!(benches, choose_route, rotate_key)` registers the two functions, and `criterion_main!` is the bench binary's entry. The measured means are the table in the build section.

## Tests

Six tests. Both bodies have the fields above, and a zero `max_tokens` fails. An invalid sort is dropped and the fallback model is the next name. Rotation keeps client id 1, and the side model stays off. Duplicate ids and a dangling tool row are repaired. TLS stays on the OS decision unless the dev flag is set. The fake turn answers `pong` from the fallback.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch18-providers
```

```text
Starting 100 tests across 25 binaries
     Summary [   0.183s] 100 tests run: 100 passed, 0 skipped
```

```bash
cargo run -p ch18-providers
```

```text
reply: pong
client: 1
```

The binary rotates once, then picks the second fake model because the first one failed. No HTTP.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `choose_route` | 5.990 ns | under 1 ms | the next model after `primary` |
| `rotate_key` | 3.465 ns | under 1 ms | advance the key, same client id |

`choose_route` builds a three-name list before the bench. The timed closure is `b.iter`. Inside: `choose_next` with that list and `Some("primary")` both passed through `black_box`. No failed HTTP call is inside the sample. The result is `fallback`.

`rotate_key` builds a pool of three keys before the bench. Each sample calls `mark_exhausted_and_rotate` and black-boxes the new key. The client id is not rebuilt. The pool wraps from the last key back to the first across samples, so the sample is the index update, not a client constructor.

The means are `mean.point_estimate` from [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `18`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`choose_next` does not call the model. A failure you have not already classified still needs chapter 14. This function only picks the next name.

The `_d2` suffix looks off by one if you expected `_d1`. Hermes starts the range at 2. A second collision of the same id becomes `_d3`.

`ssl_verify: false` without the dev flag is an error, not a quiet downgrade. The flag is the only way this crate records that the caller meant to turn verification off. It still does not install a certificate store.

A side-model summary does not run because the turn printed text. It runs when `asked` is true. Leave that false on the user path.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L3.

**K4.** Duplicate tool-call ids in one batch are renamed with `_dN` starting at 2. A transcript that ends on a tool row is closed with the interrupted placeholder so the next user message is not `tool` then `user`.

**TLS.** The decision is the OS trust store. `ssl_verify: false` is refused unless the dev flag is set. That matches `agent/ssl_verify.py`: verification off is for local development.

**Left for later.** Think-block stripping is chapter 19 (K5). The live TLS handshake is still absent, with the socket. Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Bodies.** Anthropic has `max_tokens`. Bedrock has `modelId` and `inferenceConfig.maxTokens`.
- **Route.** Invalid sorts are dropped. The fallback is the next name in `models`.
- **Client.** Rotation changes the key and keeps client id 1. The side model stays off.
- **Wire.** `call_0` twice becomes `call_0` and `call_0_d2`. A dangling tool row is closed.
- **Time.** Choosing the next route measured **5.990 ns**. Rotating a key measured **3.465 ns**.

## What you implement

`my-harness/tests/ch18.rs` is the milestone. It compiles, then panics in `todo!("chapter 18")` until `chapter_18_fallback_model` returns `"fallback"` when `sort_ok` is false. An invalid sort does not get to pick a model. 

## What's next

Chapter 19 folds a stream of deltas into one reply. Think blocks are stripped as the tokens arrive, including a tag that is split across two chunks. A read-only tool can start before the stream finishes.
