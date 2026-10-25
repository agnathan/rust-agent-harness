# Chapter 38: Profiles

A secret that never enters the prompt doesn't have to be scrubbed out of it later. Chapter 20 still redacts text that is stored or sent. This map is the earlier cut, and it matters to a continuously improving harness because memory and skills are written from whatever the model can see.

A scoped `resolve` is `96.7 ns`, and a model call is on the order of milliseconds, so the lookup isn't where a turn spends its time. The resolved persona is what the prompt receives, and the secret map stays beside that string rather than inside it.

The store keeps one mutable `scope` field, and two overlapping calls share that field, so this is the single-user toy. A safer shape would pass a scope token into `get_secret` instead of storing one field on the store. This file doesn't do that. Hermes uses a `ContextVar`, which this chapter doesn't have, and that is why two tasks there can hold two mappings while this struct can't.

There is a second gap in the same struct, and the tests don't trip it. `resolve(profile, secret_name)` reads the secret from `self.scope`, not from the `profile` argument. A scope of `reader` with `resolve("writer", ...)` would put writer's persona on the prompt and reader's secret in the bool. The tests set the scope to the same name they resolve, so the persona and the bool stay on one profile.

The crate is `steps/ch38-profiles`, the package is `ch38-profiles`, and the tree is a copy of chapter 36 plus `profiles`. It doesn't copy chapter 37, because this chapter stays on the harness line and the balancer stays on the server. `Profile` owns the persona text and the secret map, the store is built from config, and `get_secret` fails closed when no scope is installed. The error is the unscoped-secret error, and the secret stays out of the prompt, which is the multiplex rule. At the map, the store is a struct built once, and `get_secret` returns `Result` so the unscoped case is an error rather than an empty string that could pass for a real value. The prompt builder takes the persona by reference and leaves the map where it is. `cargo run -p ch38-profiles --bin ch38-profiles --offline` prints `profile: writer`. The exercise is `my-harness/tests/ch38.rs`, where an unscoped secret read is the error and the secret stays out of the prompt.

| Section | Step | What it adds |
|---|---|---|
| 38.1 | `steps/ch38-profiles` | `UnscopedSecretError` and `ResolveError` |
| 38.2 | `steps/ch38-profiles` | `Profile` and `Resolved` |
| 38.3 | `steps/ch38-profiles` | `Profiles::new`, `add`, `set_process`, `set_scope` |
| 38.4 | `steps/ch38-profiles` | `get_secret` |
| 38.5 | `steps/ch38-profiles` | `prompt`, `resolve`, `secret_leaked_into_prompt` |
| 38.6 | `steps/ch38-profiles` | `FakeModel`, `demo`, the tests in `tests/profiles.rs` |

> **How Hermes does it**
>
> I read `agent/secret_scope.py` on Hermes `main` at `9e01b44`. The module docstring says the multiplexing gateway serves many profiles from one process, and each profile's `.env` keys cannot be unioned into `os.environ`, because profile A's keys would leak into profile B. `get_secret` reads the active scope. When multiplexing is active and no scope is set, it raises rather than falling back to `os.environ`.
>
> `UnscopedSecretError` is a `RuntimeError` at line 115. The docstring says a secret was read in multiplex mode with no scope installed, and the fix is to wrap the call in `set_secret_scope`, not to widen the global allowlist. The constructor stores `secret_name`. The user-facing sentence names `this profile's {secret_name}` when a name was passed.
>
> `get_secret` is at line 219. Global names, `_is_global_env`, read `os.environ` first and return. I read `_GLOBAL_ENV_EXACT`. It includes `PATH`, `HOME`, `HERMES_HOME`, `HERMES_PROFILE`, and the API-server listener settings. The comment next to those listener settings says `API_SERVER_KEY` is a credential and is not in the set. `API_KEY` is not in the set either. With a scope installed, a hit returns that mapping's value. A miss returns `default` when `serves_routed_profile()` is true, and otherwise falls through to `os.environ`. `is_multiplex_active()` makes `serves_routed_profile()` true, so a multiplex miss does not read another profile's environment. With no scope, multiplex active raises `UnscopedSecretError`. Multiplex inactive reads `os.environ`.
>
> `set_multiplex_active` is the process flag. `is_multiplex_active` is that flag or a context-local override. The scope itself is a `ContextVar`, installed with `set_secret_scope` and cleared with `None`.
>
> The guardrail catalog calls this E4, profile secret scope: `get_secret` fails closed with `UnscopedSecretError` when no scope is installed in multiplex mode. The file I opened matches that row.
>
> **Adopted.** Multiplex on and no scope is an error. The error carries the env name that was asked for. A scope reads that profile's map and does not read another profile's map. Under multiplex, a miss stays absent and does not fall through to the process map. Outside multiplex, a miss falls through to the process map. No scope and multiplex off reads the process map. The secret is not copied into the prompt.
>
> **Adapted.** The maps are `HashMap`s. The error is a `Result`, not a raise. There is no `ContextVar`: one `Profiles` value holds one `scope` field. There is no global-env allowlist, so a name Hermes would still read from `os.environ` also fails closed here when multiplex is on and no scope is installed. The process environment is a `HashMap` filled by `set_process`, not `os.environ`. The prompt is an `Arc<str>` of the persona. `resolve` reports `has_secret` and drops the `String`. `secret_leaked_into_prompt` is a substring check. `FakeModel` ignores the prompt bytes and returns its scripted reply.
>
> **Deferred.** `_is_global_env`, the `.env` parser further down `secret_scope.py`, task-local scope, and the multiplexed gateway process. Chapter 39 is the chat gateway. The copied harness already contains chapter 20's redaction module. `profiles.rs` does not call it.

## What this chapter decides

`get_secret` with multiplex on and `scope` unset returns `Err(UnscopedSecretError)` named for the env var, and it leaves the process map unread. With a scope set, the lookup is that scope's map, and another profile's value stays invisible. Under multiplex a missing name is `Ok(None)`. With multiplex off, a missing name falls through to the process map. With multiplex off and no scope at all, `get_secret` reads the process map directly.

`prompt` and `resolve` hand out the persona `Arc`. `resolve` sets `has_secret` from whether `get_secret` returned a string, then drops that string, and the secret read follows `self.scope` rather than the profile argument, which is why the tests set those two names to the same string. `secret_leaked_into_prompt` is true only when a non-empty secret value is a substring of that persona. The demo's persona is `you write`, and the key stays in the map.

`demo` scopes `writer`, resolves `API_KEY`, calls the scripted model, and returns `profile: writer`. No key is printed.

## 38.1 The error types

E4's failure is a struct with one public field, and the value of the secret never enters it.

```rust
/// Hermes `UnscopedSecretError`: multiplex is on and no profile scope is installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnscopedSecretError {
    pub name: String,
}
```

Hermes's class subclasses `RuntimeError` and builds a sentence that tells the operator to run `hermes gateway restart`. This struct stores the name and derives `Debug`, `Clone`, `PartialEq`, and `Eq`. It doesn't implement `std::error::Error`, and it doesn't implement `Display`, because the tests compare the struct itself. `multiplex_without_scope_fails_closed` expects `name` to be `"API_KEY"`. The secret value isn't a field, so asking for `API_KEY` and failing can't leak `sk-process` through the error, because the error never received the value.

`ResolveError` is the error `resolve` returns when the profile is missing or the secret read is refused.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    Unscoped(UnscopedSecretError),
    UnknownProfile,
}
```

`Unscoped` wraps the secret error, and `UnknownProfile` means the profile name passed to `resolve` isn't in the map. The two variants are different failures. An unknown profile is reported before `get_secret` runs, and a known profile with multiplex on and no scope becomes `Unscoped`, because `get_secret` fails and `map_err` wraps it. There is no third variant for a missing secret. A missing name under a legal scope is `Ok` with `has_secret: false`, so absence and refusal stay different results.

> **Rust sidebar: `Result` instead of a raise**
>
> Hermes raises `UnscopedSecretError`, and Rust returns `Result<Option<String>, UnscopedSecretError>` from `get_secret`, so the caller writes `?` or `unwrap_err`. `Option` and `Result` are nested here, `Ok(None)` means the read was allowed and the name isn't set, and `Err(...)` means the read isn't allowed, so collapsing both into one `None` would hide E4. `ResolveError` is a separate enum because `resolve` can also fail by profile name. `map_err(ResolveError::Unscoped)` turns the inner error into the outer one, and the `Ok` path of `get_secret` is left alone by `map_err`.

## 38.2 `Profile` owns the persona and the map

The profile keeps the persona and the secrets private, and a secret leaves only through the store.

```rust
pub struct Profile {
    name: String,
    persona: Arc<str>,
    secrets: HashMap<String, String>,
}
```

All three fields are private, and there is no `secrets()` getter. A secret leaves the profile only through `Profiles::get_secret`, which looks up by the installed scope. `name` and `persona` have accessors. For the writer profile in the tests, `name()` is `writer` and `persona()` is `you write`. `persona()` borrows the `Arc<str>` as `&str`, and it doesn't clone the characters. The tests never call the accessors. They go through `resolve`, which clones the `Arc` instead of returning a borrow, so the prompt can outlive the `&Profile` lookup.

```rust
impl Profile {
    pub fn new(
        name: impl Into<String>,
        persona: impl Into<String>,
        secrets: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        Self {
            name: name.into(),
            persona: Arc::<str>::from(persona.into()),
            secrets: secrets
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn persona(&self) -> &str {
        &self.persona
    }
}
```

`new` accepts anything convertible into `String` for the name and the persona, and any iterator of pairs for the secrets. The tests pass a name, a persona, and an array of one pair, `[("API_KEY", "sk-writer")]`. Each pair is mapped into owned strings and collected into the `HashMap`, so the profile owns every key and every value.

The persona is stored as `Arc<str>`, not as `String`. `Arc::<str>::from(persona.into())` allocates the text once and puts it behind a reference count, and later clones of the `Arc` bump the count and share the bytes. `persona()` returns `&str`, which the `Arc<str>` derefs to. The secret map stays a `HashMap<String, String>`, and cloning the `Arc` doesn't clone the map, which is why handing out the persona is cheap next to a model call.

`Resolved` is the success value of `resolve`, and it is deliberately smaller than the profile.

```rust
pub struct Resolved {
    pub prompt: Arc<str>,
    pub has_secret: bool,
}
```

Both fields are public. `prompt` is the persona pointer, and `has_secret` is a bool. There is no `secret: String` field, so a caller who only has a `Resolved` can print the prompt and can branch on the bool, and they can't format the key, because this struct never held it.

> **Rust sidebar: `Arc<str>`**
>
> `Arc<str>` is a shared, immutable string, and `clone` copies the pointer and the count rather than the characters. `resolve` does that clone, which the bench includes. `HashMap<String, String>` owns each key and each value, `get` returns `Option<&String>`, and `cloned()` on that option allocates a new `String` for the caller. That allocation sits inside `get_secret`, and `resolve` drops it after reading `is_some()`. Private fields are the boundary: `Profile` can be constructed with `new` and inspected with `name` and `persona`, and the map stays off the public shape.

## 38.3 Building the store

One store holds the persona table, a stand-in for the process environment, the multiplex flag, and a single scope.

```rust
pub struct Profiles {
    profiles: HashMap<String, Profile>,
    process_env: HashMap<String, String>,
    multiplex: bool,
    scope: Option<String>,
}
```

`profiles` is the persona table, keyed by profile name. `process_env` stands in for `os.environ`. `multiplex` is fixed at construction, and `scope` starts as `None` and changes through `set_scope`. One store has one scope. Hermes keeps the scope in a `ContextVar`, so two tasks can install two mappings. This struct can't, because a second profile is only a second entry in the map, visible when `scope` is set to that name.

```rust
impl Profiles {
    pub fn new(multiplex: bool) -> Self {
        Self {
            profiles: HashMap::new(),
            process_env: HashMap::new(),
            multiplex,
            scope: None,
        }
    }

    pub fn add(&mut self, profile: Profile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn set_process(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.process_env.insert(name.into(), value.into());
    }

    pub fn set_scope(&mut self, name: Option<&str>) {
        self.scope = name.map(str::to_string);
    }
}
```

`new(true)` is the multiplex store the fail-closed test and the bench use, and `new(false)` is the single-profile store. Nothing in the file flips `multiplex` after `new`. `add` clones the profile's name for the key and moves the profile into the map, and `insert` replaces a previous profile of the same name. The tests add `writer` and `reader` once each. `set_process` inserts one process value, and there is no method that deletes a process key. `set_scope(Some("writer"))` stores `Some("writer".to_string())`, and `set_scope(None)` stores `None`, which is the cleared scope. `Option<&str>` is the argument so the caller can pass a literal or `None` without building a `String` at the call, and `map(str::to_string)` allocates only for the `Some` arm.

`add`, `set_process`, and `set_scope` take `&mut self`. `get_secret`, `prompt`, `resolve`, and `secret_leaked_into_prompt` take `&self`. The bench builds the store, then calls `resolve` inside `iter`, which only needs the shared borrow, so the maps aren't rebuilt on each sample.

## 38.4 `get_secret` fails closed

The function below is the E4 check, and the branches run in the order the source tries them.

```rust
pub fn get_secret(&self, name: &str) -> Result<Option<String>, UnscopedSecretError> {
    if let Some(scope) = &self.scope {
        let value = self
            .profiles
            .get(scope)
            .and_then(|profile| profile.secrets.get(name))
            .cloned();
        if value.is_some() || self.multiplex {
            return Ok(value);
        }
        return Ok(self.process_env.get(name).cloned());
    }
    if self.multiplex {
        return Err(UnscopedSecretError {
            name: name.to_string(),
        });
    }
    Ok(self.process_env.get(name).cloned())
}
```

A scope is installed, and the lookup is `profiles.get(scope)`, then that profile's `secrets.get(name)`, then `cloned()` so the result is `Option<String>`. The scope string is the map key, not the `name` argument of `resolve`. If the scope names a profile that isn't in the map, `get` is `None`, `and_then` stays `None`, and `value` is `None`. If the profile exists and the secret name doesn't, `value` is also `None`. Those two misses look the same to this function, so a bad scope and a missing key share one arm.

If `value` is `Some`, or multiplex is on, the function returns `Ok(value)` immediately. A hit returns the string, and a multiplex miss returns `Ok(None)` without reading `process_env`. That is the rule the Hermes notes state: under multiplexing, a miss doesn't fall through to another profile's environment. `scope_reads_only_that_profile` sets the process map to `sk-process`, scopes `writer`, and reads `API_KEY` as `sk-writer`. It then scopes `reader` and reads `sk-reader`, then asks for `MISSING` and gets `None`, not `sk-process`. Writer's key isn't reader's key.

If a scope is installed, the value is absent, and multiplex is off, the function falls through to `process_env`. `single_profile_scope_falls_through_on_a_miss` scopes `writer`, whose map has `API_KEY` and not `OTHER`, and `get_secret("OTHER")` returns `from-process`. The scope was consulted, and the miss was allowed to continue because this store was built with `Profiles::new(false)`.

No scope is installed. If multiplex is on, the function returns `Err` with `name` set to the requested env var, and it consults neither `profiles` nor `process_env`. `multiplex_without_scope_fails_closed` adds a writer profile that holds `sk-writer`, sets the process map to `sk-process`, and calls `get_secret("API_KEY")` with the scope still unset. The result is the error named `API_KEY`, and neither stored value comes back. This is the branch Hermes raises on. Hermes would still serve a global name such as `PATH` from `os.environ` before that raise, because `_is_global_env` runs first, and this function has no such list, so every name fails closed here. `API_KEY` isn't one of those global names, so for the test's key both sides agree: no scope, multiplex on, and an error.

No scope, and multiplex off. The last line reads `process_env` and wraps the option in `Ok`. A missing process key is `Ok(None)`. `inactive_multiplex_reads_the_process_map` checks this line first, before any profile is added, and gets `sk-process`.

`get_secret` clones the value out, the map keeps its own `String`, and the clone belongs to the caller. `resolve` is the caller that drops it after the bool is set.

The same five inputs, written as a table, are the tests. `P` is the process map, `W` is writer's map, and the scope column is the installed name.

| Store | Scope | Name | Result |
|---|---|---|---|
| multiplex, `W` holds `sk-writer`, `P` holds `sk-process` | unset | `API_KEY` | `Err`, name `API_KEY` |
| multiplex, writer and reader both present, `P` holds `sk-process` | `writer` | `API_KEY` | `Ok(Some("sk-writer"))` |
| same | `reader` | `API_KEY` | `Ok(Some("sk-reader"))` |
| same | `writer` | `MISSING` | `Ok(None)` |
| multiplex off, `P` holds `sk-process`, no profile yet | unset | `API_KEY` | `Ok(Some("sk-process"))` |
| multiplex off, writer scoped, `P` holds `OTHER=from-process` | `writer` | `OTHER` | `Ok(Some("from-process"))` |

The fourth row is the one that protects the other profile. `MISSING` isn't in writer's map, multiplex is on, and the function returns `Ok(None)` while `sk-process` sits unused in `process_env`. The last row is that same miss with multiplex off, and the process value comes back, so the flag passed to `Profiles::new` is the whole difference between those two rows.

Hermes and this function share the scoped miss and the unscoped failure, and they split where an allowlist would have saved a name. There is no global-env allowlist here, so an unscoped multiplex read of `PATH` returns `Err` and would still read `os.environ` in Hermes. `API_KEY` isn't in that allowlist, so both sides fail closed for the test's key, and the gap is the missing list rather than a fight over that name. Hermes can treat a first argument that contains whitespace as developer detail and clear the secret name. The Rust struct has one field, `name`, and the tests pass `API_KEY`, so nothing here has whitespace to mis-classify. Hermes parks a developer sentence on `__notes__`, and this error keeps only the name. Two overlapping calls share the one field, the file adds no lock around it, and the tests run on one thread.

> **Rust sidebar: `if let Some(scope)`**
>
> `self.scope` is `Option<String>`, and `if let Some(scope) = &self.scope` borrows the inner string when it is present. The rest of the function is the no-scope path, there is no `else` keyword, and code after the `if let` block runs only when the scope was `None`, because both arms inside the block return. `and_then` chains the optional lookups, `profiles.get(scope)` yields `Option<&Profile>`, the closure's `secrets.get(name)` yields `Option<&String>`, and `and_then` flattens that into one option before `cloned()` turns `&String` into `String`. `value.is_some() || self.multiplex` returns the option as `Ok` when either side is true. A multiplex miss is `Ok(None)`, which is still `Ok`, and the `Err` path is a later branch.

## 38.5 The prompt is the persona

The prompt is a clone of the persona, and the secret string is allowed to exist only long enough to set a bool.

```rust
pub fn prompt(&self, profile: &str) -> Option<Arc<str>> {
    self.profiles
        .get(profile)
        .map(|profile| Arc::clone(&profile.persona))
}

pub fn resolve(&self, profile: &str, secret_name: &str) -> Result<Resolved, ResolveError> {
    let Some(found) = self.profiles.get(profile) else {
        return Err(ResolveError::UnknownProfile);
    };
    let secret = self
        .get_secret(secret_name)
        .map_err(ResolveError::Unscoped)?;
    Ok(Resolved {
        prompt: Arc::clone(&found.persona),
        has_secret: secret.is_some(),
    })
}
```

`prompt` looks up the profile by the argument and clones the persona `Arc`, and an unknown name is `None`. It doesn't read secrets. `resolve` uses the same lookup and fails with `UnknownProfile` when the name is absent, and that check happens before `get_secret`. An unknown profile on a multiplex store with no scope is `UnknownProfile`, not `Unscoped`. The fail-closed test calls `resolve("writer", "API_KEY")` after adding writer, so the profile exists, `get_secret` runs, and the error is `Unscoped`.

`get_secret` uses `self.scope`, not the `profile` argument. `resolve("writer", "API_KEY")` attaches writer's persona as the prompt, and the secret bool comes from whatever scope is installed. The tests set the scope to the same name they resolve. A scope of `reader` combined with `resolve("writer", ...)` would put writer's persona on the prompt and reader's secret in the bool. The source allows that pairing, and the tests don't combine the names that way.

On the `Ok` path, `secret` is `Option<String>`, and `has_secret` stores `secret.is_some()`. The `String`, when there was one, drops at the end of `resolve`, and it isn't assigned to `Resolved`. `prompt` is `Arc::clone` of the persona. For the demo profile that persona is `you write`, and `sk-writer` isn't a substring of `you write`.

```rust
pub fn secret_leaked_into_prompt(&self, profile: &str) -> bool {
    let Some(found) = self.profiles.get(profile) else {
        return false;
    };
    found
        .secrets
        .values()
        .any(|secret| !secret.is_empty() && found.persona.contains(secret.as_str()))
}
```

An unknown profile returns false. For a known profile, every secret value is tested, and an empty string doesn't count, because `!secret.is_empty()` fails and `contains("")` would have been true for every persona. A non-empty value that appears anywhere in the persona makes the function true. The check is a substring, not the chapter 20 redactor. A persona of `you write` and a secret of `sk-writer` returns false, and a persona written as `you write with sk-writer` would return true. The function reports the substring, and it doesn't remove it. `prompt_does_not_carry_the_secret` asserts the bool is false, and it also asserts that `resolved.prompt` contains neither `sk-writer` nor `sk-`.

`resolve` of a scoped writer with `API_KEY` returns `has_secret: true` and a prompt of `you write`. The bool says the key was present, and the prompt doesn't contain it, which is the point of `Resolved`.

> **Rust sidebar: `let Some` and `?`**
>
> `let Some(found) = ... else { return Err(...) }` is the same idea as `if let`, written so the `Some` binding lives for the rest of the function, and `found` is `&Profile`. The `Arc` clone at the bottom borrows `found.persona` before `found` goes out of scope. `?` after `map_err` returns the `ResolveError` on failure, and on success `secret` is `Option<String>`, because the question mark unwraps the `Result` and leaves the `Option` in place. `contains` on `Arc<str>` uses the `str` inside and doesn't clone the persona, and the substring search runs over the shared bytes.

## 38.6 `FakeModel` and `demo`

The binary's line is a profile name, and the model call is there so a test can watch the prompt.

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

pub fn demo() -> String {
    let mut store = Profiles::new(true);
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.set_scope(Some("writer"));
    let resolved = store
        .resolve("writer", "API_KEY")
        .expect("writer is in scope");
    let _ = FakeModel { reply: "ok" }.complete(&resolved.prompt);
    "profile: writer".to_string()
}
```

`FakeModel` echoes a scripted reply and ignores the prompt, so a test can see the secret didn't ride along.

`demo` builds a multiplex store and adds writer with persona `you write` and key `sk-writer`. It installs the writer scope before `resolve`, which is why `expect` returns. The return is the literal `"profile: writer"`. `demo` drops the model's string with `let _`, and it never copies `has_secret` into that literal, so the characters `sk-writer` stay in the profile map until `demo` returns and the map drops. `expect("writer is in scope")` would panic if `resolve` returned `Err`, and with this input it doesn't. The chapter 36 supervision code is still in the crate, and `demo` doesn't call `is_stale` or `restart_child`.

Put the demo and the fail-closed test next to each other. Both start with `Profiles::new(true)` and both add a writer whose persona is `you write` and whose map holds `API_KEY` set to `sk-writer`. The test also calls `set_process("API_KEY", "sk-process")` and then calls `get_secret` and `resolve` immediately. While the demo never calls `set_process` and calls `set_scope(Some("writer"))` before `resolve`. The process map doesn't decide the demo, because the scope hits, and that `set_scope` is why the demo's `expect` returns. Remove that line and `resolve` takes the same path as the test, since profile `writer` exists, so the error is `Unscoped` rather than `UnknownProfile`, and `expect` panics. The printed line wouldn't happen. The test asserts that path without a panic, and the demo exists so the binary has a line to print once the scope is installed.

`name()` and `persona()` borrow the profile when the caller doesn't want a `Resolved`, and neither accessor takes a secret name, so a secret value leaves only through `get_secret`.

Chapter 36's `is_stale` is still public as `harness_core::supervision::is_stale`, with the same thresholds of 15 and 40. This chapter doesn't change those constants, and it doesn't pass a profile name into a pulse. A child running as `writer` doesn't install `writer` as the secret scope anywhere in this crate. The two modules are neighbors, and the binary for this chapter calls `profiles::demo` only.

> **Rust sidebar: `let _` drops the reply**
>
> `complete` returns a `String`, and `let _ = ...` drops it immediately, while the demo's return value is a different string built by `"profile: writer".to_string()`. `reply: &'static str` means the script lives for the whole program, `"ok"` is a literal, and `to_string` allocates the returned `String` from it. `expect` is the demo's check that the scope was installed, and a library caller uses `?` or a `match`. The demo may panic because a missing scope here would be a bug in these few statements, while the fail-closed path stays in the test.

## Tests

`tests/profiles.rs` is the new file. The package also carries the earlier harness suites, because the crate is a copy of chapter 36, and the assertions for this chapter are the functions named below.

`multiplex_without_scope_fails_closed` is the unset-scope path set beside the demo above, and it expects an `Err` named `API_KEY` from both `get_secret` and `resolve`.

`scope_reads_only_that_profile` adds writer, with `sk-writer` and persona `you write`, and reader, with `sk-reader` and persona `you read`. The store is multiplex, and its process map is `sk-process`. Scope `writer` reads `sk-writer`, scope `reader` reads `sk-reader`, and `MISSING` reads `None`.

`prompt_does_not_carry_the_secret` scopes writer and resolves `API_KEY`. `has_secret` is true, the prompt contains neither `sk-writer` nor `sk-`, `secret_leaked_into_prompt("writer")` is false, and the scripted reply the test asks for is `ok`.

`inactive_multiplex_reads_the_process_map` uses `Profiles::new(false)`. Before any profile is added, `get_secret("API_KEY")` returns `sk-process`. After writer is added and the scope is set, the same name is `sk-writer`, and after `set_scope(None)` it is `sk-process` again, because clearing the scope returns to the last branch of `get_secret`.

`single_profile_scope_falls_through_on_a_miss` uses multiplex off, with writer scoped and the process key `OTHER` set to `from-process`. `get_secret("OTHER")` returns `from-process`, the profile map doesn't contain `OTHER`, and the fall-through is the non-multiplex miss under a scope.

`demo_names_the_profile` expects `demo()` to equal `profile: writer`, which is the line the binary prints once the writer scope is installed.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch38-profiles
```

```text
    Starting 196 tests across 44 binaries
     Summary [   0.460s] 196 tests run: 196 passed, 0 skipped
```

```bash
cargo run -p ch38-profiles --bin ch38-profiles --offline
```

```text
profile: writer
```

The package has more than one binary, so the run names `ch38-profiles`. That binary is `src/bin/chapter_demo.rs`, it prints the profile name, and it prints no key.

```bash
cargo bench -p ch38-profiles --bench profiles --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `resolve_profile` | 96.7 ns | one scoped resolve | the persona `Arc` plus one secret lookup |

`resolve_profile` is `benches/profiles.rs`. It builds the multiplex store, the `writer` profile, and the scope before the timed loop, and the mean above is that scoped resolve.

The means are `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `38`, and the bench ids are `38/resolve_profile` and `36/resolve_profile`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The bench is the scoped hit. A measurement that builds the `HashMap` inside the sample, or that takes the `UnscopedSecretError` path, times a different question. The fail-closed path is the test `multiplex_without_scope_fails_closed`, which is why the bench installs a scope before it starts.

`has_secret` is a bool. Printing the `String` from `get_secret` would put the key in the log, `demo` doesn't print it, and `Resolved` has nowhere to store it.

A multiplex miss returns `Ok(None)` on purpose. Falling through to the process map on that path would be the leak the Hermes notes refuse, and the fall-through line in this file runs only when a scope is set, the value is absent, and `multiplex` is false.

Hermes serves `_is_global_env` names from `os.environ` even when multiplex is on and no scope is installed. This `get_secret` has no allowlist, so a later port of `PATH` or `HERMES_HOME` through this function, under multiplex with no scope, gets `UnscopedSecretError`. `API_KEY` sits outside that allowlist, so the milestone's key is the fail-closed case on both sides.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Secrets (E4).** Multiplex with no scope returns `UnscopedSecretError`. The name on the error is the env var that was asked for. The process map is not consulted. `resolve` of a profile that exists turns that into `ResolveError::Unscoped`. The catalog row and `get_secret` in `agent/secret_scope.py` are this rule. Hermes raises. This function returns `Err`.

**Prompt.** `resolve` clones the persona `Arc`. The secret `String` is dropped after `has_secret` is set. A test checks that `sk-writer` is not a substring of the prompt. `demo` prints `profile: writer`.

**Scope.** One scope, one map. Reader's key is not writer's key. Under multiplex, a name that is not in the scoped map is `Ok(None)`.

**Left for later.** The global env names Hermes still reads from the process, the `.env` loader, a task-local scope, and redaction of a secret that leaks into a tool result. Chapter 39 is where a platform message meets a persona. E1 and E2 live in the copied `redact` module. This file does not call them.

## What you implement

`my-harness/tests/ch38.rs` is the milestone, and the stub stays `todo!` until the reader fills the function in.

```rust
use my_harness::chapter_38_unscoped_secret_in_prompt;

#[test]
fn an_unscoped_secret_is_the_error_and_stays_out_of_the_prompt() {
    assert!(!chapter_38_unscoped_secret_in_prompt());
}
```

The test wants `chapter_38_unscoped_secret_in_prompt()` to be false. An unscoped secret read is the error, and the secret stays out of the prompt. A function that returns the secret inside the prompt text, or that returns the process-map value instead of the error, is the case the assertion rejects.

The stub is the panic the reader replaces.

```rust
pub fn chapter_38_unscoped_secret_in_prompt() -> bool {
    todo!("chapter 38")
}
```

`todo!("chapter 38")` panics when the test calls it. Filling the function in is the reader's work, and copying `steps/ch38-profiles` into `my-harness/` isn't the milestone the assertion is checking.

## Chapter summary

- **Personas.** Several profiles share one process, and the prompt the caller receives is the persona `Arc`.
- **Fail closed.** Multiplex with no scope is `UnscopedSecretError`, and `resolve` surfaces that failure as `ResolveError::Unscoped`.
- **Scope.** The installed profile's map is what `get_secret` reads, a multiplex miss is `Ok(None)`, and a single-profile miss can fall through to the process map.
- **Prompt.** The secret `String` isn't stored on `Resolved`, and `demo` prints `profile: writer`.
- **Bench.** The `resolve_profile` mean is `96.7 ns`, taken as `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).

## What's next

Chapter 39 is the chat gateway, back on this harness, and it is the first caller that needs more than one persona. A platform session is bound to a user, which is why this map has to exist before that gateway asks who is speaking.
