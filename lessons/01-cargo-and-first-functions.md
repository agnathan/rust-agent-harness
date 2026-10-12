# Lesson 01: Cargo, functions, strings, and tests

**Goal:** Set up the workspace, learn the cargo edit → test loop, and write your first Rust functions.

## Concepts
- **Cargo** is Rust's build tool and package manager.
  - `cargo build` compiles the code.
  - `cargo test` compiles and runs the tests.
  - `cargo check` only type-checks, which is fast.
- **A workspace** (`starter/Cargo.toml`) groups crates. A **crate** (`harness/`) is a library or a binary.
- **Modules.** `src/lib.rs` declares one module per lesson (`pub mod l01_hello;`), and each lives in its own file.
- **Functions.** `fn name(arg: Type) -> Ret { ... }`. The last expression without a `;` is the return value.
- **Strings.** `&str` is a borrowed view into text. `String` is owned, growable text. Functions usually take `&str` and return `String`.
- **Macros.** Calls ending in `!` are macros: `format!`, `println!`, `assert_eq!`, `todo!`.
- **`todo!()`** compiles fine but panics when it runs. That's how every stub in the starter fails until you implement it.
- **Tests.** Files in `harness/tests/` are integration tests. Each `#[test] fn` is one test.

## How Hermes does it
Hermes is Python, run with `hermes` (`hermes_cli/main.py:main`), and tested with pytest via `scripts/run_tests.sh`. `hermes_cli/cli.py`-style slash commands start with `/`, which is why `is_slash_command` exists.

## Tasks
1. `cd starter && cargo test --test l01_hello`. Watch it fail with "not yet implemented".
2. Open `harness/src/l01_hello.rs` and implement `agent_banner`.
   - Hint: `format!("{name} v{version} ready")`.
3. Implement `count_words`.
   - Hint: `text.split_whitespace().count()`.
4. Implement `is_slash_command`.
   - Hint: `trim_start()` then `starts_with('/')`.
5. Re-run the test until it's green. Run `cargo clippy` too if you have it.

## Milestone
`cargo test --test l01_hello` passes 3/3.

## Exercises
- Write your own `#[test]` for `count_words("\n\t")`.
- Add `fn shout(s: &str) -> String` that upper-cases its input. Why must it return `String` and not `&str`?
- Read the docs for `str` with `rustup doc --std` (search "split_whitespace").
