# Chapter 36: Profiles

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 35 can pick a backend. It still has one persona and one pile of secrets. This chapter puts several personas in one config. Each profile has its own secret map. The prompt is the persona. In multiplex mode, a secret read with no scope installed fails closed.

The step crate is `steps/ch31-profiles`. The book number is 36. The package is `ch31-profiles`.

| Section | Step | What it adds |
|---|---|---|
| 36 | `steps/ch31-profiles` | personas, a secret map per profile, fail closed with no scope |

> **How Hermes does it**
>
> I read `agent/secret_scope.py` (`UnscopedSecretError`, `get_secret`, `is_multiplex_active`, `set_multiplex_active`) on Hermes `main` at `e31c674f`.
>
> `get_secret` reads a global env name from `os.environ` always. With a scope installed, a hit comes from that scope's map. A miss under multiplexing returns the default and does not read another profile's `os.environ`. A miss outside multiplexing falls through to `os.environ`. With no scope, multiplex active raises `UnscopedSecretError`. Multiplex inactive reads `os.environ`.
>
> **Adopted.** Fail closed when multiplex is on and no scope is installed. A scope reads only that profile. Under multiplex a miss stays `None`. Outside multiplex a miss falls through to the process map.
>
> **Adapted.** The maps are `HashMap`s. There is no `ContextVar` and no `.env` file. The prompt is an `Arc<str>` of the persona. `resolve` reports whether the secret is present and does not copy it into the prompt. `FakeModel` sees that persona.
>
> **Deferred.** The global-env allowlist, the `.env` loader, and a real multiplexed gateway. Chapter 37 is the chat gateway. E1 and E2 are still absent.

[personal note: two personas in one process, and a key that never showed up in the prompt]

## What this chapter decides

`get_secret` with multiplex on and `scope` unset returns `UnscopedSecretError` named for the env var. It does not return the process map.

With a scope set, the lookup is that profile's map. Another profile's value is not visible. Under multiplex a missing name is `None`. With multiplex off, a missing name falls through to the process map, and no scope at all reads the process map.

`prompt` and `resolve` hand out the persona `Arc`. `secret_leaked_into_prompt` is true only if a secret value is a substring of that persona. The demo's persona is `you write`. The key stays in the map.

`demo` scopes `writer`, resolves `API_KEY`, calls `FakeModel`, and returns `profile: writer`. No key is printed.

## Rust: the prompt is an Arc, the secret is a map

The persona is built once, as `Arc<str>`. `resolve` clones the pointer. The secret `String` is cloned only inside `get_secret`, and that clone is dropped after `has_secret` is set. The bench keeps the store outside the sample, so the maps are not rebuilt each iteration.

## Tests

Six tests. Multiplex with no scope errors, including `resolve`. Writer and reader keep different `API_KEY` values, and a missing name is `None`. The resolved prompt does not contain `sk-writer`, and `FakeModel` returns `ok`. Multiplex off reads the process map, then the scoped profile, then the process map again when the scope is cleared. A single-profile miss falls through to the process map. The demo is `profile: writer`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch31-profiles
```

```text
     Summary [   0.006s] 6 tests run: 6 passed, 0 skipped
```

```bash
cargo run -p ch31-profiles
```

```text
profile: writer
```

The binary resolves one scoped profile. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `resolve_profile` | 95.95 ns | resolve a profile and its secret under 1 ms | persona `Arc` plus one secret lookup |

`resolve_profile` builds the multiplex store, the `writer` profile, and the scope before the bench. The timed closure is `b.iter`. Inside: `resolve` of `writer` and `API_KEY`, both passed through `black_box`, then `black_box` of the `Resolved`. The `HashMap` lookup, the secret clone, and the `Arc` clone are inside the sample. Building the profile is excluded. `FakeModel` is not constructed. The fail-closed path is not taken.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `36`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `profile: writer`.

## Pitfalls

The bench is the scoped hit. A number that includes building the `HashMap`, or the `UnscopedSecretError` path, is a different question.

`has_secret` is a bool. Printing the `String` from `get_secret` would put the key in the log. The demo does not.

A multiplex miss returns `None` on purpose. Falling through to the process map there is the leak Hermes refuses.

The persona is trusted text. This crate does not scan it. Chapter 32 scans context files. A secret pasted into the persona fails `secret_leaked_into_prompt` and still is not redacted.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Secrets (E4).** Multiplex with no scope raises `UnscopedSecretError`. The name on the error is the env var that was asked for. The process map is not consulted.

**Prompt.** `resolve` copies the persona `Arc`. The secret value is not written into that string. A test checks the substring.

**Scope.** One scope, one map. Reader's key is not writer's key.

**Left for later.** Global env names that Hermes still reads from the process, the `.env` file, and redaction of a secret that leaks into a tool result. E1 and E2 are still absent.

## Chapter summary

- **Personas.** Several profiles, one process.
- **Fail closed.** Multiplex and no scope is an error.
- **Prompt.** The persona `Arc`. The secret stays in the map.
- **Model.** `FakeModel` answers `ok`. The demo needs no key.
- **Time.** One resolve measured **95.95 ns**.

[personal note: a key that resolved, and a prompt that did not contain it]

## What's next

Chapter 37 is the chat gateway. Platform messages enter the session inbox. A stranger is denied. A pairing code is eight characters and expires.
