# Chapter 38: Profiles

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 36's harness can supervise a child. It still has one persona and one pile of secrets. A process that serves more than one user, or more than one role, needs several personas in one config, and it needs a secret that stays out of the system prompt. This chapter adds that map. The crate is `steps/ch38-profiles`, a copy of chapter 36 plus `profiles`. The package is `ch38-profiles`. It is the harness line. Chapter 37, the balancer, is not copied in.

`Profile` owns the persona text and the secret map. The store is built from config. `get_secret` fails closed when no scope is installed: the error is the unscoped-secret error, and the secret is not in the prompt. That is the multiplex rule. A profile that is selected resolves in well under the cost of a model call, and the resolved persona is what the prompt receives. The map stays beside it. `FakeModel` echoes the persona, so a test can see the secret did not ride along in the string.

Chapter 39's chat gateway is the first caller that needs more than one persona, because a platform session is bound to a user. Chapter 20's redaction still runs on text that is stored or sent. Profiles are a different control: a secret that never enters the prompt does not need to be scrubbed out of the prompt later. Chapter 49's external call will scrub again on the way out, because the other process is a reader. The profile map is why there is a scoped value to refuse in the first place.

A self-improving agent makes this sharper. Memory and skills are written by the model from whatever it can see. A secret that leaks into the persona becomes a fact in `USER.md` or a line in a skill, and chapter 21's scanner is looking for instructions, not for every credential shape. Keeping the secret out of the prompt is the earlier cut. Reflection then has nothing secret to memorize.

Rust, at the map: the store is a struct built once, the profile is an owned persona plus a map, and `get_secret` returns `Result` so the unscoped case is an error rather than an empty string that looks like a value. The prompt builder takes the persona by reference and never the map.

`cargo run -p ch38-profiles --bin ch38-profiles --offline` prints the persona. The exercise is `my-harness/tests/ch38.rs`: an unscoped secret read is the error, and the secret is not in the prompt.

**In this chapter**

- 38.1 The error types
- 38.2 `Profile` owns the persona and the map
- 38.3 Building the store
- 38.4 `get_secret` fails closed
- 38.5 The prompt is the persona
- 38.6 `FakeModel` and `demo`

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

[personal note: two personas in one process, and a key that never showed up in the prompt]

## What this chapter decides

`get_secret` with multiplex on and `scope` unset returns `Err(UnscopedSecretError)` named for the env var. It does not return the process map.

With a scope set, the lookup is that profile's map. Another profile's value is not visible. Under multiplex a missing name is `Ok(None)`. With multiplex off, a missing name falls through to the process map. With multiplex off and no scope at all, `get_secret` reads the process map.

`prompt` and `resolve` hand out the persona `Arc`. `resolve` sets `has_secret` from whether `get_secret` returned a string, then drops that string. `secret_leaked_into_prompt` is true only if a non-empty secret value is a substring of that persona. The demo's persona is `you write`. The key stays in the map.

`demo` scopes `writer`, resolves `API_KEY`, calls `FakeModel`, and returns `profile: writer`. No key is printed.

## 38.1 The error types

E4's failure is a type with one public field.

```rust
/// Hermes `UnscopedSecretError`: multiplex is on and no profile scope is installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnscopedSecretError {
    pub name: String,
}
```

Hermes's class subclasses `RuntimeError` and builds a sentence that tells the operator to run `hermes gateway restart`. This struct stores the name and derives `Debug`, `Clone`, `PartialEq`, and `Eq`. It does not implement `std::error::Error`, and it does not implement `Display`. Tests compare the struct. `multiplex_without_scope_fails_closed` expects `name` to be `"API_KEY"`. The secret value is not a field. Asking for `API_KEY` and failing cannot leak `sk-process` through the error, because the error never received the value.

`ResolveError` is the error `resolve` returns.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    Unscoped(UnscopedSecretError),
    UnknownProfile,
}
```

`Unscoped` wraps the secret error. `UnknownProfile` means the profile name passed to `resolve` is not in the map. The two variants are different failures. An unknown profile is reported before `get_secret` runs. A known profile with multiplex on and no scope becomes `Unscoped`, because `get_secret` fails and `map_err` wraps it. There is no third variant for "secret missing". A missing name under a legal scope is `Ok` with `has_secret: false`.

> **Rust sidebar: `Result` instead of a raise**
>
> - Hermes raises `UnscopedSecretError`. Rust returns `Result<Option<String>, UnscopedSecretError>` from `get_secret`. The caller writes `?` or `unwrap_err`.
> - `Option` and `Result` are nested here. `Ok(None)` means "this read was allowed, and the name is not set". `Err(...)` means "this read is not allowed". Collapsing those into one `None` would hide E4.
> - `ResolveError` is a separate enum because `resolve` can also fail by profile name. `map_err(ResolveError::Unscoped)` turns the inner error into the outer one. The `Ok` path of `get_secret` is untouched by `map_err`.

## 38.2 `Profile` owns the persona and the map

```rust
pub struct Profile {
    name: String,
    persona: Arc<str>,
    secrets: HashMap<String, String>,
}
```

All three fields are private. There is no `secrets()` getter. A secret leaves the profile only through `Profiles::get_secret`, which looks up by the installed scope. `name` and `persona` have accessors. For the writer profile in the tests, `name()` is `writer` and `persona()` is `you write`. `persona()` borrows the `Arc<str>` as `&str`. It does not clone the characters. The tests never call the accessors. They go through `resolve`, which clones the `Arc` instead of returning a borrow, so the prompt can outlive the `&Profile` lookup.

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

`new` accepts anything convertible into `String` for the name and the persona, and any iterator of pairs for the secrets. The tests pass a name, a persona, and an array of one pair: `[("API_KEY", "sk-writer")]`. Each pair is mapped into owned strings and collected into the `HashMap`.

The persona is stored as `Arc<str>`, not as `String`. `Arc::<str>::from(persona.into())` allocates the text once and puts it behind a reference count. Later clones of the `Arc` bump the count and share the bytes. `persona()` returns `&str`, which the `Arc<str>` derefs to. The secret map stays a `HashMap<String, String>`. Cloning the `Arc` does not clone the map.

`Resolved` is the success value of `resolve`.

```rust
pub struct Resolved {
    pub prompt: Arc<str>,
    pub has_secret: bool,
}
```

Both fields are public. `prompt` is the persona pointer. `has_secret` is a bool. There is no `secret: String` field. A caller who only has a `Resolved` can print the prompt and can branch on the bool. They cannot format the key, because this struct never held it.

> **Rust sidebar: `Arc<str>`**
>
> - `Arc<str>` is a shared, immutable string. `clone` copies the pointer and the count, not the characters. `resolve` does that clone. The bench includes it.
> - `HashMap<String, String>` owns each key and each value. `get` returns `Option<&String>`. `cloned()` on that option allocates a new `String` for the caller. That allocation is inside `get_secret`, and `resolve` drops it after reading `is_some()`.
> - Private fields are the boundary. `Profile` can be constructed with `new` and inspected with `name` and `persona`. The map is not part of the public shape.

## 38.3 Building the store

```rust
pub struct Profiles {
    profiles: HashMap<String, Profile>,
    process_env: HashMap<String, String>,
    multiplex: bool,
    scope: Option<String>,
}
```

`profiles` is the persona table, keyed by profile name. `process_env` stands in for `os.environ`. `multiplex` is fixed at construction. `scope` starts as `None` and changes through `set_scope`. One store has one scope. Hermes keeps the scope in a `ContextVar`, so two tasks can install two mappings. This struct cannot. A second profile is a second entry in the map, visible only when `scope` is set to that name.

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

`new(true)` is the multiplex store the fail-closed test and the bench use. `new(false)` is the single-profile store. Nothing in the file flips `multiplex` after `new`. `add` clones the profile's name for the key and moves the profile into the map. `insert` replaces a previous profile of the same name. The tests add `writer` and `reader` once each. `set_process` inserts one process value. There is no method that deletes a process key. `set_scope(Some("writer"))` stores `Some("writer".to_string())`. `set_scope(None)` stores `None`, which is the cleared scope. `Option<&str>` is the argument so the caller can pass a literal or `None` without building a `String` at the call. `map(str::to_string)` allocates only for the `Some` arm.

`add`, `set_process`, and `set_scope` take `&mut self`. `get_secret`, `prompt`, `resolve`, and `secret_leaked_into_prompt` take `&self`. The bench builds the store, then calls `resolve` inside `iter`, which only needs the shared borrow. The maps are not rebuilt per sample.

## 38.4 `get_secret` fails closed

This is the E4 function.

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

The branches, in the order the source tries them:

A scope is installed. The lookup is `profiles.get(scope)`, then that profile's `secrets.get(name)`, then `cloned()` so the result is `Option<String>`. The scope string is the map key, not the `name` argument of `resolve`. If the scope names a profile that is not in the map, `get` is `None`, `and_then` is `None`, and `value` is `None`. If the profile exists and the secret name does not, `value` is also `None`. Those two misses look the same to this function.

If `value` is `Some`, or multiplex is on, the function returns `Ok(value)` immediately. A hit returns the string. A multiplex miss returns `Ok(None)` and does not read `process_env`. That is the Hermes rule the docstring states: under multiplexing a miss does not fall through to another profile's environment. `scope_reads_only_that_profile` sets the process map to `sk-process`, scopes `writer`, and reads `API_KEY` as `sk-writer`. It then scopes `reader` and reads `sk-reader`. It then asks for `MISSING` and gets `None`, not `sk-process`. Writer's key is not reader's key.

If a scope is installed, the value is absent, and multiplex is off, the function falls through to `process_env`. `single_profile_scope_falls_through_on_a_miss` scopes `writer`, whose map has `API_KEY` and not `OTHER`, and `get_secret("OTHER")` returns `from-process`. The scope was consulted. The miss was allowed to continue because this store was built with `Profiles::new(false)`.

No scope is installed. If multiplex is on, the function returns `Err` with `name` set to the requested env var. It does not look at `profiles` and it does not look at `process_env`. `multiplex_without_scope_fails_closed` adds a writer profile that holds `sk-writer`, sets the process map to `sk-process`, and calls `get_secret("API_KEY")` with the scope still unset. The result is the error named `API_KEY`. Neither stored value comes back. This is the branch Hermes raises on. Hermes would still have served a global name such as `PATH` from `os.environ` before that raise, because `_is_global_env` runs first. This function has no such list. Every name, including names Hermes treats as process settings, fails closed on this branch. `API_KEY` is not a global name in the set I read, so for the test's key the two implementations agree: no scope, multiplex on, error.

No scope, multiplex off. The last line reads `process_env` and wraps it in `Ok`. A missing process key is `Ok(None)`. `inactive_multiplex_reads_the_process_map` checks this line first, before any profile is added, and gets `sk-process`.

`get_secret` clones the value out. The map keeps its own `String`. The clone is the caller's. `resolve` is the caller that drops it.

The same five inputs, written as a table, are the tests. `P` is the process map. `W` is writer's map. Scope is the installed name.

| Store | Scope | Name | Result |
|---|---|---|---|
| multiplex, `W` holds `sk-writer`, `P` holds `sk-process` | unset | `API_KEY` | `Err`, name `API_KEY` |
| multiplex, writer and reader both present, `P` holds `sk-process` | `writer` | `API_KEY` | `Ok(Some("sk-writer"))` |
| same | `reader` | `API_KEY` | `Ok(Some("sk-reader"))` |
| same | `writer` | `MISSING` | `Ok(None)` |
| multiplex off, `P` holds `sk-process`, no profile yet | unset | `API_KEY` | `Ok(Some("sk-process"))` |
| multiplex off, writer scoped, `P` holds `OTHER=from-process` | `writer` | `OTHER` | `Ok(Some("from-process"))` |

The fourth row is the one that protects the other profile. `MISSING` is not in writer's map, multiplex is on, and the function returns `Ok(None)` while `sk-process` sits unused in `process_env`. The last row is the same miss with multiplex off, and the process value comes back. The flag on `Profiles::new` is the whole difference between those two rows.

Hermes's `get_secret` is the function this table adapts. I read it at lines 219–247 of `agent/secret_scope.py`. The body is short enough to follow branch for branch. First, if `_is_global_env(name)` then return `os.environ` or the default. This Rust function has no matching first branch. Second, if a scope is bound, read `bound.mapping.get(name)`. A hit returns the value. A miss returns `default` when `serves_routed_profile()` is true, and otherwise `_environ_or`. `serves_routed_profile` returns true immediately when `is_multiplex_active()` is true, so a multiplex miss returns `default` and does not call `_environ_or`. That is the `Ok(None)` row, with Hermes's `default` of `None` when the caller omitted one. Third, if no scope is bound and `is_multiplex_active()`, raise `UnscopedSecretError` with the name and a developer sentence. The sentence says the read must run inside `set_secret_scope`, and that reading `os.environ` here would risk leaking another profile's value. Fourth, no scope and multiplex inactive: `_environ_or`. The Rust `Err` is the third branch. The Rust process-map reads are the second branch's fall-through and the fourth branch.

The developer sentence is not stored on `UnscopedSecretError` here. Hermes puts the user sentence in the exception message and the developer sentence on `__notes__` via `add_note`. The constructor also has a compatibility quirk I read at lines 125–130: if the first argument contains whitespace and no developer detail was passed, it treats that argument as the detail and clears the secret name. A secret name, the comment says, never contains whitespace. The Rust struct has one field, `name`, and the tests pass `API_KEY`, which has no whitespace. There is no second string to mis-classify.

`_GLOBAL_ENV_EXACT` is the list the first Hermes branch consults. I read the names. Runtime settings include `HERMES_HOME`, `HERMES_PROFILE`, `HERMES_MAX_ITERATIONS`, `HERMES_REDACT_SECRETS`, and `_HERMES_GATEWAY`. OS settings include `PATH`, `HOME`, `USER`, `LANG`, `SHELL`, `TMPDIR`. The API-server block includes `API_SERVER_ENABLED`, `API_SERVER_HOST`, `API_SERVER_PORT`, and `API_SERVER_CORS_ORIGINS`, and the comment says `API_SERVER_KEY` is a credential and is not in the set. Prefixes cover `HERMES_KANBAN_`, `HERMES_TELEGRAM_`, and `TERMINAL_`. `API_KEY` matches neither the exact set nor a prefix. On the Hermes function, an unscoped multiplex read of `API_KEY` raises. On this function, the same read returns `Err`. An unscoped multiplex read of `PATH` would still return `os.environ` in Hermes and would return `Err` here. That gap is the missing allowlist, not a disagreement about `API_KEY`.

`set_secret_scope(None)` clears the `ContextVar`. `set_scope(None)` clears the field. Both put the next read on the no-scope path. The difference is who else can see the installation. A `ContextVar` is visible to the task that set it and to a context copied onto a worker. The field is visible to every caller of this `Profiles` value until the next `set_scope`. `inactive_multiplex_reads_the_process_map` relies on that: it sets the scope, reads writer's key, clears the scope, and reads the process key, all on one store, in order. Two overlapping calls would share that same field. The file does not add a lock around it. The tests run on one thread.

> **Rust sidebar: `if let Some(scope)`**
>
> - `self.scope` is `Option<String>`. `if let Some(scope) = &self.scope` borrows the inner string when it is present. The `else` of this pattern is the rest of the function, the no-scope path. There is no `else` keyword. Code after the `if let` block runs only when the scope was `None`, because both arms inside the block `return`.
> - `and_then` chains optional lookups. `profiles.get(scope)` yields `Option<&Profile>`. The closure's `secrets.get(name)` yields `Option<&String>`. `and_then` flattens that into one option. `cloned()` turns `&String` into `String`.
> - `value.is_some() || self.multiplex` returns the option as `Ok` when either side is true. A multiplex miss is `Ok(None)`, which is still `Ok`. The `Err` path is not this line.

## 38.5 The prompt is the persona

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

`prompt` looks up the profile by the argument and clones the persona `Arc`. An unknown name is `None`. It does not read secrets. `resolve` uses the same lookup and fails with `UnknownProfile` when the name is absent. That check happens before `get_secret`. An unknown profile on a multiplex store with no scope is `UnknownProfile`, not `Unscoped`. The fail-closed test calls `resolve("writer", "API_KEY")` after `add` of writer, so the profile exists, `get_secret` runs, and the error is `Unscoped`.

`get_secret` uses `self.scope`, not the `profile` argument. `resolve("writer", "API_KEY")` attaches writer's persona as the prompt, and the secret bool comes from whatever scope is installed. The tests set the scope to the same name they resolve. A scope of `reader` combined with `resolve("writer", ...)` would put writer's persona on the prompt and reader's secret in the bool. The source allows it. The tests do not combine them that way.

On the `Ok` path, `secret` is `Option<String>`. `has_secret` stores `secret.is_some()`. The `String`, when there was one, drops at the end of `resolve`. It is not assigned to `Resolved`. `prompt` is `Arc::clone` of the persona. For the demo profile that persona is `you write`. `sk-writer` is not a substring of `you write`.

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

An unknown profile returns false. For a known profile, every secret value is tested. An empty string does not count, because `!secret.is_empty()` fails and `contains("")` would have been true for every persona. A non-empty value that appears anywhere in the persona makes the function true. The check is a substring, not the chapter 20 redactor. A persona of `you write` and a secret of `sk-writer` returns false. A persona that had been written as `you write with sk-writer` would return true. The function does not remove the substring. It reports it. `prompt_does_not_carry_the_secret` asserts the bool is false, and also asserts `resolved.prompt` does not contain `sk-writer` and does not contain `sk-`.

`resolve` of a scoped writer with `API_KEY` returns `has_secret: true` and a prompt of `you write`. The bool says the key was present. The prompt does not contain it. That split is the point of `Resolved`.

> **Rust sidebar: `let Some` and `?`**
>
> - `let Some(found) = ... else { return Err(...) }` is the same idea as `if let`, written so the `Some` binding lives for the rest of the function. `found` is `&Profile`. The `Arc` clone at the bottom borrows `found.persona` before `found` goes out of scope.
> - `?` after `map_err` returns the `ResolveError` on failure. On success, `secret` is `Option<String>`. The question mark unwraps the `Result`, not the `Option`.
> - `contains` on `Arc<str>` uses the `str` inside. It does not clone the persona. The substring search is over the shared bytes.

## 38.6 `FakeModel` and `demo`

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

`FakeModel` holds a `'static` reply. `complete` takes the prompt, binds it to `_`, and returns `reply.to_string()`. The model does not echo the prompt. It does not read `API_KEY`. The test calls `complete` and expects `"ok"`, and it checks the prompt again after the call. The prompt is unchanged because `complete` does not write it. `resolved.prompt` is an `Arc<str>`, and the argument is `&str`.

`demo` builds a multiplex store, adds writer, and installs the writer scope before `resolve`. That order is the difference between this function and the fail-closed test. The test never calls `set_scope`. `demo` does. `resolve` succeeds. `has_secret` would be true. `demo` does not read the bool and does not read the prompt into the return value. It discards `complete`'s string with `let _`. The return is the literal `"profile: writer"`. The characters `sk-writer` do not appear in that literal. They remain in the profile map, which drops when `demo` returns.

`expect("writer is in scope")` panics if `resolve` returned `Err`. With this input it does not. No key is printed. The chapter 36 supervision code is still in the crate. `demo` does not call `is_stale` or `restart_child`.

Put the demo and the fail-closed test next to each other. Both start with `Profiles::new(true)` and both `add` a writer whose persona is `you write` and whose map holds `API_KEY` = `sk-writer`. The test also calls `set_process("API_KEY", "sk-process")` and then calls `get_secret` and `resolve` immediately. The demo never calls `set_process`. It calls `set_scope(Some("writer"))` and then `resolve`. The process map is irrelevant to the demo because the scope hits. The scope is the whole reason the demo's `expect` returns. Remove that `set_scope` line and `resolve` takes the same path as the test: profile `writer` exists, so the error is `Unscoped`, not `UnknownProfile`, and `expect` panics. The printed line would not happen. The test exists so that path is asserted without a panic. The demo exists so the binary has a line to print when the scope is installed.

`name()` and `persona()` are the borrows a caller uses when they do not want a `Resolved`. `persona()` on writer is `you write`, the same characters `resolve` will wrap in an `Arc` clone. Neither accessor accepts a secret name. There is no way, through them, to format `sk-writer` into a prompt. The only public function that reads a secret value and returns it to the caller is `get_secret`. `resolve` reads it and keeps the bool. `secret_leaked_into_prompt` reads every value and keeps a bool. `demo` calls `resolve` and then throws away even the bool, by not naming `resolved.has_secret`. The model call receives `&resolved.prompt`, which is `you write`. `FakeModel::complete` binds that reference to `_` and returns `ok`. The test after the model call checks the prompt again, because a model that wrote into the prompt would be a different `complete`. This `complete` cannot write into an `&str`.

Chapter 36's `is_stale` is still public as `harness_core::supervision::is_stale`, with the same 15 and 40. This chapter does not change those constants and does not pass a profile name into a pulse. A child running as `writer` does not, in this crate, automatically install `writer` as the secret scope. The two modules are neighbors. The binary for this chapter calls `profiles::demo` only.

> **Rust sidebar: `let _` drops the reply**
>
> - `complete` returns a `String`. `let _ = ...` drops it immediately. The demo's return value is a different string, built by `"profile: writer".to_string()`.
> - `reply: &'static str` means the script lives for the whole program. `"ok"` is a literal. `to_string` allocates the returned `String` from it.
> - `expect` is the demo's assertion that the scope was installed. A library caller uses `?` or a `match`. The demo is allowed to panic because a missing scope here would be a bug in these four statements, and the fail-closed path is the test, not this function.

## Tests

`tests/profiles.rs` is the new file. The package also carries the earlier harness suites, because the crate is a copy of chapter 36. The assertions for this chapter are the functions in that file. I am not going to invent a package-wide total.

`multiplex_without_scope_fails_closed` builds `Profiles::new(true)`, adds writer with persona `you write` and `API_KEY` of `sk-writer`, and sets the process value to `sk-process`. It does not call `set_scope`. `get_secret("API_KEY")` is `Err(UnscopedSecretError { name: "API_KEY" })`. `resolve("writer", "API_KEY")` matches `Err(ResolveError::Unscoped(_))`. The secret strings are in the maps. They are not in the error.

`scope_reads_only_that_profile` adds writer (`sk-writer`, persona `you write`) and reader (`sk-reader`, persona `you read`) on a multiplex store, with the process map set to `sk-process`. Scope `writer` reads `sk-writer`. Scope `reader` reads `sk-reader`. `MISSING` reads `None`.

`prompt_does_not_carry_the_secret` scopes writer and resolves `API_KEY`. `has_secret` is true. The prompt does not contain `sk-writer`. `secret_leaked_into_prompt("writer")` is false. `FakeModel { reply: "ok" }` returns `ok`. The prompt also does not contain `sk-`.

`inactive_multiplex_reads_the_process_map` uses `Profiles::new(false)`. Before any profile is added, `get_secret("API_KEY")` is `sk-process`. After writer is added and the scope is set, the same name is `sk-writer`. After `set_scope(None)`, it is `sk-process` again. Clearing the scope returns to the last branch of `get_secret`.

`single_profile_scope_falls_through_on_a_miss` uses multiplex off, writer scoped, process key `OTHER` set to `from-process`. `get_secret("OTHER")` is `from-process`. The profile map does not contain `OTHER`. The fall-through is the non-multiplex miss under a scope.

`demo_names_the_profile` expects `demo()` to equal `profile: writer`.

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

The package has more than one binary, so the run names `ch38-profiles`. That binary is `src/bin/chapter_demo.rs`. It prints the profile name. No key.

```bash
cargo bench -p ch38-profiles --bench profiles --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `resolve_profile` | 96.7 ns | one scoped resolve | the persona `Arc` plus one secret lookup |

`resolve_profile` is `benches/profiles.rs`. It builds the multiplex store, the `writer` profile, and the scope before the bench. The timed closure is `b.iter`. Inside: `resolve` of `writer` and `API_KEY`, both passed through `black_box`, then `black_box` of the `Resolved`. The `HashMap` lookup, the secret clone, and the `Arc` clone are inside the sample. Building the profile is outside it. `FakeModel` is not constructed. The fail-closed path is not taken. `secret_leaked_into_prompt` is not called. The sample is the scoped hit.

The mean is `96.7 ns`. It is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `38`. The id is `38/resolve_profile`. The old id `36/resolve_profile` is an alias in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `profile: writer`.

## Pitfalls

The bench is the scoped hit. A measurement that builds the `HashMap` inside the sample, or that takes the `UnscopedSecretError` path, times a different question. The fail-closed path is the test `multiplex_without_scope_fails_closed`.

`has_secret` is a bool. Printing the `String` from `get_secret` would put the key in the log. `demo` does not print it. `Resolved` has nowhere to put it.

A multiplex miss returns `Ok(None)` on purpose. Falling through to the process map on that path is the leak the Hermes docstring refuses. The fall-through line in this file runs only when a scope is set, the value is absent, and `multiplex` is false.

`resolve` checks the profile argument, then reads the secret from the scope field. They are the same string in the tests and in `demo`. They are two lookups in the source.

The persona is trusted text. This function does not scan it for the threats chapter 34 scans in context files. A secret pasted into the persona makes `secret_leaked_into_prompt` true and is still sitting in the `Arc`. The check reports the leak. It does not redact it. `profiles.rs` does not call the chapter 20 redactor that this copied crate already contains.

Hermes serves `_is_global_env` names from `os.environ` even when multiplex is on and no scope is installed. This `get_secret` has no allowlist. A later port of `PATH` or `HERMES_HOME` through this function, under multiplex with no scope, will get `UnscopedSecretError`. `API_KEY` is outside that allowlist in the file I read, so the milestone's key is the fail-closed case on both sides.

`set_scope` is one field. Two tasks that need two profiles at the same time need two `Profiles` values, or they need to take turns with `&mut self`. The `ContextVar` in `secret_scope.py` is the piece this struct does not have.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Secrets (E4).** Multiplex with no scope returns `UnscopedSecretError`. The name on the error is the env var that was asked for. The process map is not consulted. `resolve` of a profile that exists turns that into `ResolveError::Unscoped`. The catalog row and `get_secret` in `agent/secret_scope.py` are this rule. Hermes raises. This function returns `Err`.

**Prompt.** `resolve` clones the persona `Arc`. The secret `String` is dropped after `has_secret` is set. A test checks that `sk-writer` is not a substring of the prompt. `demo` prints `profile: writer`.

**Scope.** One scope, one map. Reader's key is not writer's key. Under multiplex, a name that is not in the scoped map is `Ok(None)`.

**Left for later.** The global env names Hermes still reads from the process, the `.env` loader, a task-local scope, and redaction of a secret that leaks into a tool result. Chapter 39 is where a platform message meets a persona. E1 and E2 live in the copied `redact` module. This file does not call them.

## What you implement

`my-harness/tests/ch38.rs` is the milestone. The step crate is the reference. The stub stays `todo!`.

```rust
use my_harness::chapter_38_unscoped_secret_in_prompt;

#[test]
fn an_unscoped_secret_is_the_error_and_stays_out_of_the_prompt() {
    assert!(!chapter_38_unscoped_secret_in_prompt());
}
```

The test wants `chapter_38_unscoped_secret_in_prompt()` to be false. An unscoped secret read is the error, and the secret is not in the prompt. A function that returns the secret inside the prompt text, or that returns the process-map value instead of the error, is the case the assertion rejects.

The stub panics.

```rust
pub fn chapter_38_unscoped_secret_in_prompt() -> bool {
    todo!("chapter 38")
}
```

`todo!("chapter 38")` is the panic. Filling the function in is the reader's work. Copying `steps/ch38-profiles` into `my-harness/` is not the milestone.

## Chapter summary

- **Personas.** Several profiles, one process. The prompt is the persona `Arc`.
- **Fail closed.** Multiplex and no scope is `UnscopedSecretError`. `resolve` surfaces it as `ResolveError::Unscoped`.
- **Scope.** The installed profile's map. A multiplex miss is `Ok(None)`. A single-profile miss can fall through to the process map.
- **Prompt.** The secret `String` is not stored on `Resolved`. `demo` prints `profile: writer`.
- **Bench.** `resolve_profile` mean `96.7 ns`. That is `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The Criterion `time:` line is the slope.

[personal note: a key that resolved, and a prompt that did not contain it]

## What's next

Chapter 39 is the chat gateway, back on this harness. It knows which persona is speaking only after this chapter. It decides which person is allowed to speak. A stranger is denied. A pairing code is eight characters and expires. A platform message is a push onto the session inbox.
