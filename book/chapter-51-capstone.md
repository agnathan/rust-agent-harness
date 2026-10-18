# Chapter 51: Load and Token Capstone

*Draft. Bracketed notes like [personal note] are where my own story goes.*

*(server)*

Chapter 50 can see a turn. This chapter asks a smaller question on the server line. How many schema tokens does a finished task spend if the offer is every card, a fixed eight, or the one card the task named? The same crate then runs 100 calls through the session gateway that already lives in memory. Nothing listens.

The step crate is `steps/ch51-capstone`. The book number is 51. The package is `ch51-capstone`. The library name stays `gwservice`, so `gwservice::` still resolves. This is a copy of `steps/ch42-service`, the MCP gateway service, plus `src/capstone.rs`. It is not in the harness copy chain. There is no path dependency on `ch42-service`. The sources are copied. Edition is 2021. `rust-version` is 1.85. `Cargo.toml` says the chapter 42 tree vendored, plus the three token arms and an in-memory load. `lib.rs` still opens with the chapter 23 comment about a prediction installed before the request. `pub mod capstone` is the new module. The other modules are the vendored ones: `catalog`, `federation`, `limits`, `service`, and `session`. The bench target is named `capstone`. The binary is `ch51-capstone`, and `src/main.rs` prints `gwservice::capstone::demo()`.

`mcp-gateway` now depends on `ch51-capstone` and on `ch08-session-toolsets`. The chapter 8 library name is `gw_session`. `mcp-gateway/src/lib.rs` re-exports `gwservice::LearnedGateway` as `Gateway`. It also re-exports the chapter 8 session types. The file comment says this process still binds no socket. The crate has no `[[bin]]`. The smoke test builds `Gateway::new()` and reads an empty catalog. It does not call `capstone::demo`.

| Section | What it walks |
|---|---|
| 51.1 | `CHARS_PER_TOKEN`, the bounds, `Card`, `Arm`, `LoadReport` |
| 51.2 | `schema_tokens` and `session::approx_tokens` |
| 51.3 | `sample_catalog` and the 18-byte schema |
| 51.4 | `tokens_of`, `offers_needed`, `compare`, `arm` |
| 51.5 | `task_arms` and `session::run_rename_task` |
| 51.6 | `admit` and `p99` |
| 51.7 | `load` |
| 51.8 | `demo` |
| 51.9 | the package tests, including the ones this crate inherited |

> **How Hermes does it**
>
> I read `tools/tool_search_catalog.py` on Hermes `main` at `9e01b44`. Lines 22–23 set `CHARS_PER_TOKEN = 4.0`. The comment on that line says "slightly underestimates (fewer false activations)." Around line 311, `assemble_if_fits` keeps a rendered catalog only when `math.ceil(len(text) / CHARS_PER_TOKEN)` is within `max_tokens`. The three modes there are full schemas, names, and a collapsed summary of the largest groups. I did not find this chapter's three arms in that file. Hermes does not score `every`, `static`, and `dynamic` against one needed name, and it does not run an in-memory load of 100 calls.
>
> **Adopted.** Four characters per token. Three arms: every card, a static set of 8, and a dynamic set of the one name the task needs.
>
> **Adapted.** This crate uses the vendored `session::approx_tokens`. That function is integer `div_ceil` of the byte length by 4. It is not `math.ceil` of a float division. The load is `SessionGateway` in memory. There is no `oha` run. There is no listening socket.
>
> **Deferred.** A real socket, `oha`, and a remote catalog. A live embedder stays out of this file. The Benchmarks job still measures the in-process functions.

> **Guardrails.** The arms report tokens and a success bit. A deadline the wide arms exceed fails those arms and leaves the dynamic arm successful. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

[personal note: three token counts on a task that still finished]

## What this chapter decides

`sample_catalog` builds 32 cards. Each schema is the JSON object `{"name":"tool_NN"}`. That string is 18 bytes. `schema_tokens` calls `approx_tokens`. `div_ceil(18 / 4)` is 5. The every arm is `32 * 5`, which is 160. The static arm is the first 8 cards, `8 * 5`, which is 40. The dynamic arm is the one matching name, which is 5.

`compare` returns three `Arm` values in that order: `every`, `static`, `dynamic`. Success is the needed name being offered, and the token count sitting inside the deadline when a deadline is set. `None` means no deadline. `tool_07` is inside the first 8, so all three arms succeed when the deadline is `None`. `tool_31` misses the static set. The name `missing` fails all three.

`task_arms` calls `session::run_rename_task` for `Arm::Every`, `Arm::Static`, and `Arm::Dynamic`. All three succeed. Those reports carry schema tokens from the session telemetry. They are not the 160, 40, and 5 figures. The demo prints both results.

`load(100)` builds a `SessionGateway`, reloads `catalog::synthetic(32)`, searches once, and calls the revealed tool 100 times. `CALL_BOUND` is 8. The wave counter increments when `in_flight` is 0 at the start of a call, and it resets `in_flight` when the count hits 8. One hundred divided by 8 is 12 full waves, and the leftover calls open one more wave, so the report says 13. `p99` runs on the per-call samples. The demo does not print it. The number moves from run to run. `oha` is not invoked.

`demo` prints one line:

```text
capstone: every=160 static=40 dynamic=5 success=1/1/1 task=1/1/1 load=100/100 waves=13
```

## 51.1 The constants, `Card`, `Arm`, and `LoadReport`

`capstone.rs` is the new module. The file starts with the constants and the three structs the rest of the functions pass around.

```rust
//! Chapter 51. Three token arms and an in-memory load. No socket.
//! The arms and the load call the chapter 8 session gateway that this crate vendors.

use std::time::Instant;

use serde_json::json;

use crate::catalog::{synthetic, Load};
use crate::session::{self, approx_tokens, SessionGateway};

/// Hermes `tools/tool_search_catalog.py` uses 4 characters per token.
pub const CHARS_PER_TOKEN: usize = 4;
pub const CALL_BOUND: usize = 8;
pub const CATALOG_LEN: usize = 32;
pub const STATIC_LEN: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub name: String,
    pub schema: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arm {
    pub name: &'static str,
    pub tokens: u64,
    pub success: bool,
    pub offered: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadReport {
    pub requests: u32,
    pub successes: u32,
    pub p99_ns: u128,
    pub elapsed_ns: u128,
    pub waves: u32,
}
```

`CHARS_PER_TOKEN` documents the Hermes rule. `schema_tokens` does not read this constant. The division lives in `session::approx_tokens`, which has its own `CHARS_PER_TOKEN` of 4. Both numbers match. Changing only the capstone constant would not change a token count.

`CALL_BOUND` is 8. `admit` uses it. `load` uses it as the width of a wave. `CATALOG_LEN` is 32. `STATIC_LEN` is 8. `sample_catalog` and `compare` and `task_arms` all take the catalog length from that constant. The static slice is `take(STATIC_LEN)`.

`Card` is a name and a schema string. Both fields are public. `Arm` is one row of the comparison. `name` is a static string, `every` or `static` or `dynamic`. `tokens` is the sum. `success` is the bit. `offered` is how many cards that arm kept. `LoadReport` is the load. `requests` and `successes` are the counts the demo prints as `load=100/100`. `waves` is the counter the demo prints. `p99_ns` and `elapsed_ns` are filled and then left off the printed line.

`PartialEq` is why a test can compare arms by field. The tests in this chapter compare fields one at a time.

### Rust: a public constant the function does not read

A `pub const` is part of the crate's API. The compiler does not warn that `capstone::CHARS_PER_TOKEN` is unread, because a downstream crate could name it. The function that divides is `approx_tokens`, and it names `session::CHARS_PER_TOKEN`. Two constants with the same value can drift. The test pins the token count through `schema_tokens`, so a drift in the session constant would fail `three_arms_share_a_success_and_not_a_token_count`. A drift in the unused capstone constant would not.

## 51.2 `schema_tokens` and `approx_tokens`

```rust
pub fn schema_tokens(schema: &str) -> u64 {
    approx_tokens(schema)
}
```

That is the whole function. It forwards the schema string. The session module is the one that knows the character rule. I am quoting only `approx_tokens` from `session.rs`. The rest of that module is the vendored chapter 8 gateway, and this chapter does not paste it.

```rust
pub fn approx_tokens(text: &str) -> u64 {
    (text.len().div_ceil(CHARS_PER_TOKEN)) as u64
}
```

`text.len()` is the byte length. The schemas in this chapter are ASCII, so bytes and characters match. `div_ceil` rounds a remainder up. `18.div_ceil(4)` is 5, because 18 is `4 * 4 + 2`. Integer division without the ceil would be 4. Hermes uses `math.ceil` of a float. `4.0` in Python makes `len(text) / CHARS_PER_TOKEN` a float, and `math.ceil` of that float is the same integer for these lengths. The Rust function never builds a float. An empty string is `0.div_ceil(4)`, which is 0.

`schema_tokens` is what `tokens_of` calls for each card. The session gateway uses `approx_tokens` on other strings too: a search listing, a describe block, call arguments, and the call result. Those calls are inside `load` and inside `run_rename_task`. They do not change the `Arm.tokens` figures. Those figures are only the card schemas.

### Rust: `usize::div_ceil`

`div_ceil` is a method on the integer. `a.div_ceil(b)` is the smallest integer greater than or equal to `a / b`, for a positive divisor. The cast to `u64` happens after the rounding. A schema longer than `usize` is not a case this function sees. The cards here are 18 bytes.

## 51.3 `sample_catalog`

```rust
pub fn sample_catalog() -> Vec<Card> {
    (0..CATALOG_LEN)
        .map(|index| {
            let name = format!("tool_{index:02}");
            Card {
                schema: format!(r#"{{"name":"{name}"}}"#),
                name,
            }
        })
        .collect()
}
```

The range is `0..32`. The name format is `tool_` plus the index in width 2, so the names are `tool_00` through `tool_31`. `tool_07` is index 7. `tool_31` is the last card.

The schema format is a raw string whose doubled braces are the `format!` escape. The value that lands in `Card.schema` is `{"name":"tool_NN"}`. Count it. The opening brace is 1. `"name"` is 6. The colon is 1. `"tool_00"` is 9, because the name itself is 7 characters and the quotes add 2. The closing brace is 1. That is 18 bytes. Every index from 0 to 31 has a two-digit suffix, so every schema in this catalog is 18 bytes. `approx_tokens` of one schema is 5.

`sample_catalog` does not call the session gateway. It does not search. The bench builds it once, outside the timed loop, and passes the vec into `compare`.

### Rust: `{index:02}` and `{{` in a format string

`:02` is the fill and the width. A value under 10 is padded with a zero. `0` prints as `00`. `31` already has two digits, so it prints as `31`. A raw string does not treat `\` as an escape. `format!` still treats `{{` as a literal `{` and `}}` as a literal `}`. The `{name}` in the middle is the placeholder. The string that comes out is JSON with one field.

## 51.4 `tokens_of`, `offers_needed`, `compare`, and `arm`

```rust
fn tokens_of(cards: &[&Card]) -> u64 {
    cards.iter().map(|card| schema_tokens(&card.schema)).sum()
}

fn offers_needed(cards: &[&Card], needed: &str) -> bool {
    cards.iter().any(|card| card.name == needed)
}

pub fn compare(catalog: &[Card], needed: &str, deadline: Option<u64>) -> [Arm; 3] {
    let every: Vec<&Card> = catalog.iter().collect();
    let static_set: Vec<&Card> = catalog.iter().take(STATIC_LEN).collect();
    let dynamic: Vec<&Card> = catalog.iter().filter(|card| card.name == needed).collect();
    [
        arm("every", &every, needed, deadline),
        arm("static", &static_set, needed, deadline),
        arm("dynamic", &dynamic, needed, deadline),
    ]
}

fn arm(name: &'static str, cards: &[&Card], needed: &str, deadline: Option<u64>) -> Arm {
    let tokens = tokens_of(cards);
    let present = offers_needed(cards, needed);
    let within = deadline.is_none_or(|limit| tokens <= limit);
    Arm {
        name,
        tokens,
        success: present && within,
        offered: cards.len(),
    }
}
```

`compare` borrows the catalog. It builds three slices of references. `every` is the whole catalog, 32 references. `static_set` is `take(8)`, which is `tool_00` through `tool_07`. `dynamic` keeps the cards whose name equals `needed`. For `tool_07` that is one card. For `missing` that is an empty vec.

`arm` sums tokens, checks presence, and checks the deadline. `deadline.is_none_or(...)` is true when the option is `None`. A `Some(limit)` is true only when `tokens <= limit`. Success is both bits. An arm can offer the tool and still fail, because the deadline failed. An arm can be under the deadline and still fail, because the name was absent. The dynamic arm for a missing name has zero cards, zero tokens, and `success` false. Zero tokens would pass any non-negative deadline. Presence still fails it.

The return type is `[Arm; 3]`. Index 0 is every. Index 1 is static. Index 2 is dynamic. The bench reads `arms[2].tokens`.

For `tool_07` and `deadline = None` the numbers are fixed. Every offers 32 and sums to 160. Static offers 8 and sums to 40. Dynamic offers 1 and sums to 5. `tool_07` is one of the first 8 names, so presence is true on all three. All three succeed.

For `tool_31` and no deadline, every still offers it, and dynamic keeps that one card. Static stopped at `tool_07`. Presence is false on the static arm, so that arm's success bit is false. Its token count is still 40. The failure is the missing name, not the tokens.

For `missing`, every and static do not contain that name. Dynamic's filter is empty. All three success bits are false.

The deadline test takes the dynamic token count, which is 5, and passes `Some(5)` back into `compare` for `tool_07`. Every is 160, which is above 5. Static is 40, which is above 5. Dynamic is 5, which is not above 5. Presence is still true on all three, because the name did not change. The wide arms fail on the deadline alone. The dynamic arm stays successful.

### Rust: `Option::is_none_or` and a fixed array

`is_none_or` takes a predicate. `None` skips the predicate and returns true. `Some` runs it. Writing `deadline.map(|limit| tokens <= limit).unwrap_or(true)` is the same branch. The method is the short form.

`[Arm; 3]` is a value with a length the compiler knows. The caller cannot push a fourth arm without changing the type. A `Vec` would make `arms[2]` a runtime check. The array makes the three-arm shape part of the signature. `compare` always returns three, including when the dynamic filter is empty.

## 51.5 `task_arms` and the rename task

```rust
/// The three arms on the real session gateway. Each report has tokens and a success bit.
pub fn task_arms(count: usize) -> [session::TaskReport; 3] {
    [
        session::run_rename_task(session::Arm::Every, count),
        session::run_rename_task(session::Arm::Static, count),
        session::run_rename_task(session::Arm::Dynamic, count),
    ]
}
```

`task_arms` does not call `compare`. It calls the vendored session function three times. `count` is `CATALOG_LEN` when `demo` calls it, so each call builds a synthetic catalog of 32 tools. The return type is `[TaskReport; 3]`. A `TaskReport` has `success`, `schema_tokens`, `call_tokens`, and `result_tokens`. The demo prints only the three success bits, as `task=1/1/1`.

I read `run_rename_task` in `steps/ch51-capstone/src/session.rs`. It builds a `SessionGateway`, reloads `synthetic(count)` as the native cards, and opens a session named `task`. It finds the card whose server is `files` and whose name contains `rename_file`. That card is the target.

`synthetic` cycles eight servers, eight verbs, and eight nouns. For a count of 32 the noun index stays on `file`, because the noun only advances every 64 tools. The files server's rename card is in that set. `catalog::call` returns the card's description for every name other than `echo`. The description is the sentence that says to use that server to perform that verb on that noun. It contains `rename` and `file`, and it is not empty.

The `Every` arm reveals every card, sets `schema_tokens` from `approx_tokens` of `schema_blob` over the whole catalog, and calls the target with `{"text":"a.txt"}`. Success is the result containing `rename`, or the result being non-empty. The description satisfies both.

The `Static` arm reveals the target, then reveals up to seven more cards whose server is `files`. It sets `schema_tokens` from the blob of the cards that were actually revealed, and it calls the target with `{"to":"b.txt"}`. Success is the result containing `rename` or `file`. The description satisfies that too.

The `Dynamic` arm calls `search_tools` with the query `rename a file`. It takes the first listing line and splits on ` — `, the space-emdash-space separator `search_tools` writes between the name and the blurb. If that name is non-empty it `describe`s the name, calls it with `{"to":"b.txt"}`, and succeeds when the name contains both `rename` and `file`. The search hits the rename card. The name check passes.

The schema tokens on these reports are the telemetry totals. `schema_blob` joins name, server, and description. Those strings are longer than `{"name":"tool_NN"}`. The test checks that each report succeeded and that `schema_tokens` is greater than 0, and that the every arm's schema tokens are at least the dynamic arm's. It does not assert 160, 40, or 5. Those three numbers belong to `compare`.

`task_arms` is not what the bench times. `dynamic_tokens` calls `compare`. `load_100` calls `load`.

### Rust: two arm types

`capstone` has a struct named `Arm`. `session` has an enum named `Arm`, with variants `Every`, `Static`, and `Dynamic`. `task_arms` spells the enum as `session::Arm`. A bare `Arm` inside `capstone.rs` is the struct. The array it returns is `TaskReport`, not the struct. The demo reads `.success` on both. The fields behind that name differ.

## 51.6 `admit` and `p99`

```rust
pub fn admit(in_flight: usize) -> bool {
    in_flight < CALL_BOUND
}

/// Nearest rank. For 100 samples the index is 98.
pub fn p99(mut samples: Vec<u128>) -> u128 {
    if samples.is_empty() {
        return 0;
    }
    samples.sort_unstable();
    let rank = samples.len().saturating_mul(99).div_ceil(100);
    let index = rank.saturating_sub(1);
    samples[index]
}
```

`admit` is true while `in_flight` is below 8. `admit(7)` is true. `admit(8)` is false. `load` calls `admit` before it increments, and it resets the counter when the counter reaches 8, so the assert inside the loop stays true for a normal run. The function itself is the gate. Filling it to 8 and asking again returns false.

`p99` sorts a copy. The argument is `mut` because `sort_unstable` needs it. An empty vec returns 0 and does not index. Otherwise the rank is `len * 99`, divided by 100 with `div_ceil`, then minus 1, saturating at 0. For 100 samples, `100 * 99` is 9900, and `9900.div_ceil(100)` is 99. The index is 98. That is the comment on the function. The test builds the series `1..=100` and expects the value at that index. This chapter does not print a `p99` from `load`. The samples in `load` are elapsed nanoseconds. They move. The rank formula does not.

`saturating_mul` and `saturating_sub` clamp instead of panicking on overflow. A length that overflows `usize` on the multiply is not a length this load produces. The load passes 100 samples.

### Rust: nearest rank on a sorted vec

The index is into the sorted vec, low to high. Index 0 is the smallest sample. Index 98 of 100 sorted samples is near the top. `sort_unstable` does not preserve the order of equal keys. The load's samples are times, and ties are possible. The function does not break ties by insertion order. The demo leaves the result off the line because a reader would treat one debug process's rank as a benchmark. The bench times the whole `load`, including this sort, and reports a Criterion mean of that whole call. The mean cell is `MEAN_PENDING_51_LOAD`. It is not a `p99`.

## 51.7 `load`

```rust
/// One hundred in-memory calls through `SessionGateway`. Nothing listens.
pub fn load(requests: u32) -> LoadReport {
    let gateway = SessionGateway::new();
    let cards = synthetic(CATALOG_LEN);
    gateway
        .reload(Load {
            native: cards,
            plugins: Vec::new(),
            upstreams: Vec::new(),
        })
        .expect("reload");
    gateway.open_session("load");
    let listing = gateway
        .search_tools("load", &["file".into()])
        .expect("search");
    let name = listing
        .lines()
        .next()
        .and_then(|line| line.split(" — ").next())
        .expect("tool")
        .to_string();
    let mut samples = Vec::with_capacity(requests as usize);
    let mut successes = 0u32;
    let mut waves = 0u32;
    let mut in_flight = 0usize;
    let started = Instant::now();
    for _ in 0..requests {
        if in_flight == 0 {
            waves += 1;
        }
        assert!(admit(in_flight));
        in_flight += 1;
        let one = Instant::now();
        let result = gateway
            .call("load", &name, &json!({"text": "a.txt"}))
            .expect("call");
        if !result.is_empty() {
            successes += 1;
        }
        samples.push(one.elapsed().as_nanos());
        if in_flight == CALL_BOUND {
            in_flight = 0;
        }
    }
    LoadReport {
        requests,
        successes,
        p99_ns: p99(samples),
        elapsed_ns: started.elapsed().as_nanos(),
        waves,
    }
}
```

`load` does not call `sample_catalog` and it does not call `compare`. The cards are `catalog::synthetic(32)`, the same generator `run_rename_task` uses. Plugins and upstreams are empty. The session id is `load`.

`search_tools` takes one query, the word `file`. That is under `MAX_QUERIES_PER_CALL`, which the session module sets at 7. The search reveals the hits and returns a listing. Each line is a name, then ` — `, then a trimmed description. The load keeps the first line's name. `search_tools` has already revealed that name, so `call` does not return `NotOffered`.

`catalog::call` returns the description for a synthetic tool. The description is non-empty. Each of the 100 calls increments `successes`. The test locks that at 100.

The wave counter is a plain integer. The loop is one thread. Nothing is spawned. At the start, `in_flight` is 0, so the first call increments `waves` to 1 and then increments `in_flight` to 1. Calls continue. When `in_flight` reaches 8, it is set back to 0 at the bottom of the loop. The next call sees 0 again and increments `waves`. Eight calls are one full wave. One hundred divided by 8 is 12 full waves, which is 96 calls. After call 96 the counter is 0 again. Call 97 increments `waves` to 13. Calls 97 through 100 are the partial wave. The code increments `waves` when `in_flight` returns to 0 at the start of a call, so the partial counts. The report's `waves` is 13.

`admit` is checked after the zero-test and before the increment. The counter is 0, 1, 2, 3, 4, 5, 6, or 7 at the check. All of those are below 8. The reset happens after the call, so the next check sees 0 rather than 8.

Each call records `elapsed` in nanoseconds and pushes it. After the loop, `p99` sorts that vec and stores the rank. `elapsed_ns` is the whole loop, including the search that happened before the loop. The search is outside the per-call samples and inside `elapsed_ns`. Catalog construction, `reload`, `open_session`, and the search are all inside `load`. The bench times `load`, so they are inside the sample.

There is no socket in this function. I searched the crate for a listener, a bind, and `oha`. None of those names are in the Rust sources. The calls are `SessionGateway::call` on the value built in this function.

### Rust: the assert is a bound check, not a benchmark

`assert!(admit(in_flight))` panics if a wave ever asks for a ninth in-flight call without resetting. A release build of this binary still keeps `assert!`. It is not `debug_assert!`. The panic path is not the measured path. The measured path resets at 8 and admits every call. The samples include the `call`, the emptiness check, and the push. They do not include `p99`. `p99` runs once, after the loop, and its sort is still inside `load`, so the bench pays for it once per `load(100)`.

## 51.8 `demo`

```rust
pub fn demo() -> String {
    let catalog = sample_catalog();
    let arms = compare(&catalog, "tool_07", None);
    let tasks = task_arms(CATALOG_LEN);
    let report = load(100);
    format!(
        "capstone: every={} static={} dynamic={} success={}/{}/{} task={}/{}/{} load={}/{} waves={}",
        arms[0].tokens,
        arms[1].tokens,
        arms[2].tokens,
        arms[0].success as u8,
        arms[1].success as u8,
        arms[2].success as u8,
        tasks[0].success as u8,
        tasks[1].success as u8,
        tasks[2].success as u8,
        report.successes,
        report.requests,
        report.waves
    )
}
```

`demo` builds the 32-card catalog, compares `tool_07` with no deadline, runs the three rename tasks at catalog length 32, and loads 100. The format string is the whole line. Bools print as `1` or `0` because of `as u8`. `report.p99_ns` is not a placeholder in that string. `report.elapsed_ns` is not either.

`main` is five lines:

```rust
use gwservice::capstone::demo;

fn main() {
    println!("{}", demo());
}
```

`cargo run -p ch51-capstone --bin ch51-capstone` prints the line above. I ran it. The line was `capstone: every=160 static=40 dynamic=5 success=1/1/1 task=1/1/1 load=100/100 waves=13`. No key. No socket.

### Rust: `as u8` on a bool

A bool casts to `0u8` or `1u8`. `format` then prints the number. The test looks for the substring `success=1/1/1` and for `task=1/1/1`. A failed arm would print `0` in that slot. The deadline path is not what `demo` runs. `demo` passes `None`.

## 51.9 The package tests

The 23 tests are the package. Five of them are `tests/capstone.rs`. The other eighteen are the inherited files `balancing.rs`, `learned.rs`, and `service.rs`. Nextest starts six binaries: the lib harness, the `ch51-capstone` binary harness, and those four integration-test binaries. The lib and the binary have no `#[test]` functions. The 23 tests sit in the four integration binaries.

```bash
cargo nextest run --locked -p ch51-capstone
```

```text
    Starting 23 tests across 6 binaries
     Summary [   0.087s] 23 tests run: 23 passed, 0 skipped
```

### `three_arms_share_a_success_and_not_a_token_count`

The test builds `sample_catalog` and checks the length against `CATALOG_LEN`. It takes `schema_tokens` of the first schema and checks that the count is greater than 0. It compares `tool_07` with no deadline. The names are `every`, `static`, and `dynamic`. Offered counts are 32, 8, and 1. Every arm succeeds. Every's tokens are greater than static's, and static's are greater than dynamic's. Every's tokens equal one card's tokens times 32. Dynamic's tokens equal that one card. With the 18-byte schema, that one card is 5, every is 160, and static is 40 because 8 times 5 is 40. The test expresses static as the ordered inequality rather than as a second product. It then compares `tool_31`. Every and dynamic succeed. Static fails. It compares `missing`. Every arm fails.

### `the_session_gateway_arms_each_report_tokens_and_success`

The test calls `task_arms(CATALOG_LEN)`. It asserts that every `TaskReport` has `success` true and `schema_tokens` greater than 0. It asserts that the every report's schema tokens are at least the dynamic report's. It does not read `Arm.tokens`. A failure here is a rename task that missed, not a 160-versus-40 comparison.

### `a_tight_deadline_fails_the_wide_arm_only`

This is the deadline guard. The test reads the dynamic token count with no deadline, then compares `tool_07` again with `Some(dynamic)`.

```rust
fn a_tight_deadline_fails_the_wide_arm_only() {
    let catalog = sample_catalog();
    let dynamic = compare(&catalog, "tool_07", None)[2].tokens;
    let arms = compare(&catalog, "tool_07", Some(dynamic));
    assert!(!arms[0].success);
    assert!(!arms[1].success);
    assert!(arms[2].success);
}
```

`arms[0]` is every. `arms[1]` is static. `arms[2]` is dynamic. The assertions fail the first two and keep the third. The limit equals the dynamic total, so a token count of 5 is inside the limit and a token count of 40 or 160 is outside it. The needed name is still `tool_07`, which all three arms offer. The wide arms fail because they are past the deadline.

### `the_load_stays_inside_the_bound_and_ranks_p99`

`admit(CALL_BOUND - 1)` is true. `admit(CALL_BOUND)` is false. The test builds `1..=100` as `u128` samples and checks `p99` of that series. That check is the rank formula on a known series. It is not a timed load. Then it calls `load(100)` and checks `successes == 100`, `requests == 100`, `waves == 13`, and `p99_ns > 0`. The last check says the rank ran and produced a non-zero sample. It does not pin a nanosecond count. This chapter does not quote one.

### `demo_reports_three_successes`

The test calls `demo` and looks for `success=1/1/1`, `task=1/1/1`, `load=100/100`, and `waves=13`. It does not look for a `p99` field. The printed line has no place for one.

### `least_outstanding_picks_the_quietest_backend`

This is the inherited balancer. Four backends carry outstanding counts 3, 1, 4, and 2. `pick(LeastOutstanding)` returns index 1, the backend named `b`. `call` returns `echo via b`. A second pick is still index 1. The test is about the copied pool, not about token arms.

### `power_of_two_picks_the_quieter_of_the_pair`

Three backends, outstanding 5, 1, and 4. Power-of-two between index 0 and 1 picks 1. Power-of-two between index 2 and 0 picks 2. Least-outstanding on that pool returns some index. An empty pool returns `None`.

### `federation_groups_a_tool_across_upstreams`

Two upstreams, `alpha` with `echo` and `time`, `beta` with `echo`. `federate` maps `echo` to both servers, alpha then beta. `pool_for("echo")` has length 2 and calls `echo via alpha`. `pool_for("missing")` is empty.

### `discovery_stays_at_four`

`discover(6, Duration::from_millis(40))` returns `DISCOVERY_CONCURRENCY`, and that constant is 4. The sleep is why this test is the slow one in the package. The capstone load does not call `discover`.

### `the_demo_picks_alpha`

`gwservice::federation::demo()` equals `pick: echo via alpha`. That is the inherited federation demo. `capstone::demo` is a different function. The binary prints the capstone line.

### `prediction_changes_the_offered_set_and_is_not_on_the_read`

`LearnedGateway` opens `desk`. The first `session_tools` view is empty and `list_changed` is false. A usage log of two `read_file` rows, memory that mentions notes in files, and a session note that mentions `web_search` produce the ranked offer `web_search`, then `read_file`. `session_tools` returns that offer and sets `list_changed`. `predict_calls` does not increase on the read. A second `session_tools` clears `list_changed` and keeps the same names. This is the chapter 23 prediction path, still in the copied `lib.rs`. The capstone does not call `refresh_prediction`.

### `a_zero_score_tool_is_left_out`

`predict` on a catalog of `read_file`, `web_search`, `write_file`, and `terminal`, with an unrelated task and empty memory, returns an empty vec. A score of 0 is dropped.

### `composite_and_macro_expand_to_steps`

`save_composite("rename_flow", [read_file, write_file])` and `save_macro("morning", [web_search, read_file])` round-trip through `expand`. `expand("missing")` is `None`.

### `code_mode_refuses_a_tool_outside_the_allowlist_or_the_offer`

`run_code_mode` of `call memory {}` returns `NotAllowed("memory")`. `call web_search {}` against an offer of only `read_file` returns `NotOffered`. `call read_file {"path":"a"}` runs once and returns `ok`. `SANDBOX_ALLOWED` contains `terminal` and does not contain `memory`.

### `code_mode_stops_at_fifty_calls`

A script of `CODE_MODE_MAX_CALLS + 1` lines, and that constant is 50, returns `TooManyCalls`. The check happens before the calls run.

### `the_scripted_model_is_optional_and_keyless`

A fresh `ScriptedModel` reports 0 calls. `gwservice::demo()` returns the offered name `read_file` and the body `ok`. That `demo` is the learned-gateway demo in `lib.rs`, not `capstone::demo`. Completing the scripted model once returns `unused` and bumps the call count to 1. No key is read.

### `kill_moves_the_session_to_the_next_replica`

`Service::new` on replicas `a` and `b` places session `s1` on `a`. `reload` bumps the generation from 1 to 2. The held config arc's strong count is 1 after the reload. `kill("a")` routes `s1` to `b`. The service stays healthy and ready.

### `quota_audit_and_drain`

`bearer_ok` accepts `Bearer token` and rejects `token` and `Bearer ` with the trailing space and nothing after it. The first `accept` returns an id. A second session for the same tenant hits `Quota`. A bad token hits `BadToken`. The audit line starts with `accept `. The span list is `gateway.call 1`. `start_drain` clears ready. A new accept hits `Draining`. `finish` drains the inflight count to 0. A `FakeModel` completes `a` to `ok`. This service is the copied chapter 42 router. `load` does not construct it.

### `a_dead_pool_refuses_the_call`

Killing the only replica makes `healthy` false. `accept` returns `NoReplica`.

### `sampling_stops_at_ten_per_minute`

`SAMPLING_MAX_RPM` is 10. The loop allows that many calls at timestamp 0. The next `allow(0)` is false. `allow(60)` is true.

### `url_elicitation_is_declined`

`elicit(false)` is an approval with `ELICIT_TIMEOUT_SECONDS`. `elicit(true)` is `Declined`. A URL elicitation does not proceed.

### `osv_blocks_malware_and_fails_open_on_the_network`

An `echo` check is `Skipped` and leaves the cache empty. A clean `npx` result is `Clean`. A later check inside the TTL that cannot reach the network stays `Clean`. A `uvx` package with a malware id and no network is `FailOpen` and occupies one cache slot. The same package with a network and `MAL-9` is `Blocked`. A check one second inside `OSV_CACHE_TTL_SECONDS` still returns the blocked malware id.

### `demo_routes_to_the_first_replica`

`service::demo()` equals `route: a`. The capstone binary does not print that line.

## Build, run, and benchmark

The run command and the line are in the sections above. The bench command is the one the workflow uses.

```bash
cargo bench -p ch51-capstone --bench capstone --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

`.github/scripts/run_bench_section.sh` runs that shape. The workflow calls it for section 51 with package `ch51-capstone` and bench `capstone`. The same flags are in `steps/ch51-capstone/README.md`.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `dynamic_tokens` | `MEAN_PENDING_51_TOKENS` | the dynamic arm's token count, which is 5 | `compare` of `tool_07`, then `arms[2].tokens` |
| `load_100` | `MEAN_PENDING_51_LOAD` | 100 in-memory calls, the sort, and `p99` | `load(100)` |

The ids are `51/dynamic_tokens` and `51/load_100`. The old ids `49/dynamic_tokens` and `47/dynamic_tokens` alias the first. The old ids `49/load_100` and `47/load_100` alias the second. They are the `aliases` arrays in `benchmarks/descriptions.json`. I did not rewrite gh-pages history.

```rust
fn dynamic_tokens(c: &mut Criterion) {
    let catalog = sample_catalog();
    c.bench_function("dynamic_tokens", |b| {
        b.iter(|| {
            let arms = compare(black_box(&catalog), black_box("tool_07"), None);
            black_box(arms[2].tokens);
        });
    });
}

fn load_100(c: &mut Criterion) {
    c.bench_function("load_100", |b| {
        b.iter(|| {
            let report = load(black_box(100));
            black_box(report);
        });
    });
}
```

`dynamic_tokens` builds the catalog before `bench_function`. That build is not timed. The timed closure is `b.iter`. Inside it: `compare` of the catalog and `tool_07`, both passed through `black_box`, with deadline `None`, then `black_box` of `arms[2].tokens`. That value is 5. Building the three slices and summing `approx_tokens` are inside the sample. `load` is not called. `run_rename_task` is not called. No socket is opened. `FakeModel` is not called. The descriptions file says a number around a tenth of a microsecond is three filters over 32 short strings, and a millisecond would mean the sample started a socket or read a catalog from disk. The token count 5 is a result, not a time.

`load_100` has no setup outside the iteration. Every sample calls `load(100)` and `black_box` of the report. Inside that call: `synthetic(32)`, `reload`, one `search_tools`, 100 `SessionGateway::call`s, the wave counter, the sample vec, the sort, and the `p99` index. There is no sleep. There is no socket. `compare` and `run_rename_task` are not called. The deadline path is not taken. All 100 succeed, and the report carries 13 waves. The Criterion mean is the whole load, not one call. The descriptions file says microseconds would be surprising and milliseconds are the honest band, and that this rate is not a server throughput number. `oha` is not measured.

The mean cells are `mean.point_estimate`. `MEAN_PENDING_51_TOKENS` and `MEAN_PENDING_51_LOAD` stand in those cells until the Benchmarks workflow publishes this sample. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `capstone: every=160 static=40 dynamic=5 success=1/1/1 task=1/1/1 load=100/100 waves=13`.

## Pitfalls

The two token totals are different measurements. `compare` sums 18-byte schemas and gets 160, 40, and 5. `run_rename_task` sums `schema_blob` text through the session telemetry. Printing the task schema tokens on the demo line would show a different triple. The demo prints the `compare` triple and the task success bits.

`p99` of the load is a rank of nanosecond samples. The test of `1..=100` checks the index math. `load`'s `p99_ns > 0` checks that a sample existed. Putting either number on the demo line would look like a benchmark. The benchmark is `load_100`'s mean of the whole function. Those are different jobs.

Integer `div_ceil` makes 18 bytes into 5 tokens. Dropping the ceil would make 4, and the arms would be 128, 32, and 4. The Hermes call is `math.ceil` of a float. For this length the two roundings agree. They are not the same function. A length that is a multiple of 4 agrees either way. A length with a remainder is where `div_ceil` and truncating division split.

The static set is the first 8 names in catalog order. `tool_07` is in it. `tool_31` is not. A static set that was "any 8" would not pin that failure.

The wave counter is not a thread pool. Resetting `in_flight` to 0 is what opens the next wave. Forgetting the increment when the counter is already 0 would report 12 for 100 calls and drop the partial. The test wants 13.

`assert!(admit(...))` inside `load` aborts the process if the bound is broken. The bench assumes the happy path. A change that stopped resetting at 8 would fail the bench by panicking, and it would fail `the_load_stays_inside_the_bound_and_ranks_p99` only if that panic happened inside `load(100)`. `admit(8)` is checked on its own, before `load`.

`mcp-gateway` re-exports `LearnedGateway`. It does not re-export `capstone::demo`. Running the process crate's tests does not print the capstone line. The binary that prints it is `ch51-capstone`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The catalog does not have a row for this capstone. The behaviors below are the ones `capstone.rs` actually implements.

**Tokens.** Four characters per token, through `session::approx_tokens`. The arms report the sum and a success bit. Success requires the needed tool to be in the offered set.

**Deadline.** A limit equal to the dynamic total fails every and static and keeps dynamic. That is `a_tight_deadline_fails_the_wide_arm_only`. The wide arms are past the limit. The dynamic arm is on the limit, and `<=` admits it. The tool is still offered on the arms that fail. The failure is the deadline.

**Load bound.** `CALL_BOUND` is 8. `admit(8)` is false. The load resets at 8, so 100 calls are 13 waves and 100 successes. No socket is opened. `oha` is not run.

**Left for later.** A listening socket, an `oha` run, and a remote catalog. Chapter 52 does not add them. Chapter 52 copies the harness, not this crate.

## What you implement

`my-harness/tests/ch51.rs` is the milestone.

```rust
use my_harness::chapter_51_arm_succeeded;

#[test]
fn each_arm_reports_success() {
    assert!(chapter_51_arm_succeeded("every"));
    assert!(chapter_51_arm_succeeded("static"));
    assert!(chapter_51_arm_succeeded("dynamic"));
}
```

The stub panics.

```rust
pub fn chapter_51_arm_succeeded(name: &str) -> bool {
    let _ = name;
    todo!("chapter 51")
}
```

`todo!("chapter 51")` is the panic. The test asks the function about `every`, `static`, and `dynamic`, and it wants `true` for each. In the step crate that is `compare` of `tool_07` with no deadline: all three `Arm.success` bits are true, and `task_arms` reports the same three successes from `run_rename_task`. The stub stays `todo!`. Do not paste `capstone.rs` into `my-harness/`.

## Chapter summary

- **Arms.** `compare` scores every card, the first 8, and the one named card. For `tool_07` the tokens are 160, 40, and 5, and all three succeed.
- **Tokens.** `approx_tokens` is `len.div_ceil(4)`. An 18-byte schema is 5.
- **Tasks.** `task_arms` runs the rename task on `SessionGateway` three times. All three succeed. Their schema tokens are the telemetry blobs.
- **Load.** 100 in-memory calls, bound 8, 13 waves. `p99` is computed and not printed.
- **Process.** `mcp-gateway` re-exports `LearnedGateway` as `Gateway`. It binds no socket.
- **Time.** `dynamic_tokens` mean `MEAN_PENDING_51_TOKENS`. `load_100` mean `MEAN_PENDING_51_LOAD`. Both are `mean.point_estimate`. The Criterion `time:` line is the slope.

[personal note: a wide arm that failed its deadline and a narrow arm that finished]

## What's next

Chapter 52 re-runs chapter 9's saved tasks on the full harness. Chapter 52 copies chapter 50, the tracing harness, not this server crate. The server line ends here.
