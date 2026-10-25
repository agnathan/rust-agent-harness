# Chapter 51: Load and Token Capstone

A smaller offer only counts when the task still succeeds. This is the MCP server program, the same line as chapters 5, 7, 8, 15, 23, 37, and 42, and it isn't the harness. Chapter 52 doesn't copy this directory. The sample is 32 cards, each schema `{"name":"tool_NN"}` is 18 bytes. `approx_tokens` is integer `div_ceil` of that length by 4. So each card is 5 tokens. Every-tool is 160, the static arm of the first eight is 40. The dynamic arm on `tool_07` is 5. `tool_07` sits inside the first eight. So with no deadline all three arms succeed, while a deadline equal to the dynamic total fails the wide arms and keeps the dynamic arm. That is this toy catalog's arithmetic, not a tokenizer and not chapter 6's 10,000-tool list.

A self-improving harness that adds tools without an offer policy eventually puts every schema in the prompt. The dynamic arm is the counterexample, and it has to succeed at the task or it is only a smaller prompt. Continuously improving is this book's name for that harness, which is why the session toolset, the learned offer. The limits sit beside the comparison.

The crate is `steps/ch51-capstone`, a copy of chapter 42, with library name `gwservice` and a new module `capstone`. It binds no socket, so `oha` isn't run. There's no listener to point it at, and `mcp-gateway` re-exports `LearnedGateway` from this crate as `Gateway` beside chapter 8's session types. A `Card` is one tool in the sample catalog, the struct `Arm` is one row of the comparison. `sample_catalog` builds the fixed set the three arms share. `tokens_of` and `offers_needed` are the arithmetic behind `compare`, `task_arms` is the rename task in the same shape as chapter 8's fake task. `load` admits 100 in-memory dispatches whose percentile stays inside the process. `admit` is the gate that load passes through, and `demo` prints the arms and the load line.

Integer `div_ceil` is the token estimate, a struct per arm keeps the three results from collapsing into one number. The percentile is a sort plus an index. The load is a loop of in-memory calls with no socket, no HTTP framework, and no key.

Running `cargo run -p ch51-capstone` prints the arms. The exercise is `my-harness/tests/ch51.rs`, where the dynamic, static, and every-tool arms each report tokens and task success.

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

## What this chapter decides

The opening already fixed the token totals and which arms succeed with no deadline. So this section keeps the other decisions. `compare` returns three `Arm` values in the order `every`, `static`, `dynamic`, and success means the needed name is offered and the token count sits inside the deadline when a deadline is set, with `None` meaning no deadline. `tool_31` misses the static set, and the name `missing` fails all three.

`task_arms` calls `session::run_rename_task` for the session enum's three variants, and all three succeed. Those reports carry schema tokens from the session telemetry, which are a different measurement from the compare totals. The demo prints both results.

`load(100)` builds a `SessionGateway`, reloads `catalog::synthetic(32)`, searches once, and calls the revealed tool 100 times, with `CALL_BOUND` at 8. The wave counter lands on 13, which section 51.7 states without walking every call. `p99` runs on the per-call samples while the demo leaves that field off the printed line. `oha` isn't invoked, because this crate binds no socket.

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

`CHARS_PER_TOKEN` documents the Hermes rule. `schema_tokens` doesn't read this constant, because the division lives in `session::approx_tokens`, which has its own `CHARS_PER_TOKEN` of 4. Both numbers match, so changing only the capstone constant wouldn't change a token count, which is why a drift in the unused constant would slip past the test that pins `schema_tokens`.

`CALL_BOUND` is 8, and both `admit` and the width of a wave in `load` use it. `CATALOG_LEN` is 32 and `STATIC_LEN` is 8, so `sample_catalog`, `compare`, and `task_arms` take the catalog length from that constant. The static slice is `take(STATIC_LEN)`.

A `Card` is a name and a schema string, and both fields are public. The struct `Arm` is one row of the comparison: `name` is a static string, `every` or `static` or `dynamic`, `tokens` is the sum, `success` is the bit. `offered` is how many cards that arm kept. The session module has an enum with the same name, and section 51.5 is where a search hits both. `LoadReport` is the load, `requests` and `successes` are the counts the demo prints as `load=100/100`, `waves` is the counter the demo prints. `p99_ns` and `elapsed_ns` are filled and then left off the printed line.

`PartialEq` is why a test can compare arms by field, and the tests in this chapter compare fields one at a time. A `pub const` is part of the crate's API, and the compiler doesn't warn that `capstone::CHARS_PER_TOKEN` is unread. A downstream crate could name it. The function that divides is `approx_tokens`, and it names `session::CHARS_PER_TOKEN`. So two constants with the same value can drift. The test pins the token count through `schema_tokens`, which means a drift in the session constant would fail `three_arms_share_a_success_and_not_a_token_count`, while a drift in the unused capstone constant would not.

## 51.2 `schema_tokens` and `approx_tokens`

```rust
pub fn schema_tokens(schema: &str) -> u64 {
    approx_tokens(schema)
}
```

That forwarder is the whole function, and the session module is the one that knows the character rule. The rest of `session.rs` is the vendored chapter 8 gateway. This chapter quotes only `approx_tokens`.

```rust
pub fn approx_tokens(text: &str) -> u64 {
    (text.len().div_ceil(CHARS_PER_TOKEN)) as u64
}
```

`text.len()` is the byte length, and the schemas in this chapter are ASCII, so bytes and characters match. `div_ceil` rounds a remainder up, which is the rule the opening already applied to the 18-byte schema. Integer division without the ceil would drop that remainder. Hermes uses `math.ceil` of a float. `4.0` in Python makes `len(text) / CHARS_PER_TOKEN` a float. `math.ceil` of that float is the same integer for these lengths. The Rust function never builds a float, and an empty string is `0.div_ceil(4)`, which is 0.

`schema_tokens` is what `tokens_of` calls for each card. The session gateway also uses `approx_tokens` on a search listing, a describe block, call arguments, and the call result. Those calls sit inside `load` and inside `run_rename_task`. They don't change the `Arm.tokens` figures, because those figures are only the card schemas. `div_ceil` is a method on the integer, `a.div_ceil(b)` is the smallest integer greater than or equal to `a / b` for a positive divisor. The cast to `u64` happens after the rounding. A schema longer than `usize` isn't a case this function sees.

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

The range is `0..32`, and the name format is `tool_` plus the index in width 2. So the names run from `tool_00` through `tool_31`. `tool_07` is index 7, and `tool_31` is the last card. The schema format is a raw string whose doubled braces are the `format!` escape. The value that lands in `Card.schema` is `{"name":"tool_NN"}`, which the opening counted at 18 bytes. Every index from 0 to 31 has a two-digit suffix, so every schema in this catalog has that same length. `sample_catalog` doesn't call the session gateway or run a search.

The bench builds the catalog once, outside the timed loop, and passes the vec into `compare`. `:02` is the fill and the width. So a value under 10 is padded with a zero, `0` prints as `00`. `31` already has two digits so it prints as `31`. A raw string doesn't treat `\` as an escape, while `format!` still treats `{{` as a literal `{` and `}}` as a literal `}`. `{name}` in the middle is the placeholder that becomes the tool name.

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

`compare` borrows the catalog and builds three slices of references. `every` is the whole catalog, 32 references, `static_set` is `take(8)`, which is `tool_00` through `tool_07`. `dynamic` keeps the cards whose name equals `needed`. For `tool_07` that filter is one card, and for `missing` it is an empty vec.

`arm` sums tokens, checks presence, and checks the deadline. `deadline.is_none_or(...)` is true when the option is `None`, a `Some(limit)` is true only when `tokens <= limit`, and success is both bits. An arm can offer the tool and still fail. The deadline failed, and an arm can sit under the deadline and still fail, because the name was absent. The dynamic arm for a missing name has zero cards, zero tokens, and `success` false. So zero tokens would pass any non-negative deadline while presence still fails it.

The return type is `[Arm; 3]`, index 0 is every, index 1 is static, and index 2 is dynamic, which is the slot the bench reads as `arms[2].tokens`. For `tool_07` and `deadline = None` the totals are the ones the opening fixed. `tool_07` is one of the first eight names, so presence is true on all three and all three succeed. For `tool_31` and no deadline, every still offers it and dynamic keeps that one card, while static stopped at `tool_07`. So presence is false on the static arm, that arm's success bit is false. Its token count is still the static total. The failure is the missing name, not the tokens.

For `missing`, every and static don't contain that name, dynamic's filter is empty, and all three success bits are false. The deadline test takes the dynamic token count and passes `Some` of that count back into `compare` for `tool_07`. Every and static sit above that limit, dynamic sits on it. Presence is still true on all three because the name didn't change. The wide arms fail on the deadline alone, and the dynamic arm stays successful.

`is_none_or` takes a predicate, `None` skips the predicate and returns true, and `Some` runs it. Writing `deadline.map(|limit| tokens <= limit).unwrap_or(true)` is the same branch. The method is the short form. `[Arm; 3]` is a value with a length the compiler knows, so the caller can't push a fourth arm without changing the type. A `Vec` would make `arms[2]` a runtime check. The array makes the three-arm shape part of the signature, which is why `compare` always returns three, including when the dynamic filter is empty.

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

`task_arms` doesn't call `compare`, because it calls the vendored session function three times. `count` is `CATALOG_LEN` when `demo` calls it. So each call builds a synthetic catalog of 32 tools. The return type is `[TaskReport; 3]`, a `TaskReport` has `success`, `schema_tokens`, `call_tokens`, and `result_tokens`. The demo prints only the three success bits, as `task=1/1/1`.

Two types share the name `Arm`, and a reader who searches the name hits both. The capstone module's `Arm` is a struct, one row of the comparison, with `name`, `tokens`, `success`, and `offered`. The session module's `Arm` is an enum with variants `Every`, `Static`, and `Dynamic`, which is what `task_arms` passes as `session::Arm`. A bare `Arm` inside `capstone.rs` is the struct, the array `task_arms` returns is `TaskReport` rather than that struct. The demo reads `.success` on both even though the fields behind that name differ, and neither name changes.

`run_rename_task` in `steps/ch51-capstone/src/session.rs` builds a `SessionGateway`, reloads `synthetic(count)` as the native cards, and opens a session named `task`. It finds the card whose server is `files` and whose name contains `rename_file`. That card is the target. `synthetic` cycles eight servers, eight verbs, and eight nouns, and for a count of 32 the noun index stays on `file`. The noun only advances every 64 tools. The files server's rename card is in that set. `catalog::call` returns the card's description for every name other than `echo`. The description is the sentence that says to use that server to perform that verb on that noun. It contains `rename` and `file`, and it isn't empty.

The `Every` arm reveals every card, sets `schema_tokens` from `approx_tokens` of `schema_blob` over the whole catalog, and calls the target with `{"text":"a.txt"}`. Success is the result containing `rename`, or the result being non-empty, and the description satisfies both. The `Static` arm reveals the target, then reveals up to seven more cards whose server is `files`, sets `schema_tokens` from the blob of the cards that were actually revealed, and calls the target with `{"to":"b.txt"}`. Success is the result containing `rename` or `file`, and the description satisfies that too.

The `Dynamic` arm calls `search_tools` with the query `rename a file`, takes the first listing line, and splits on ` — `, the space-emdash-space separator `search_tools` writes between the name and the blurb. If that name is non-empty it describes the name, calls it with `{"to":"b.txt"}`. Succeeds when the name contains both `rename` and `file`. The search hits the rename card, and the name check passes. The schema tokens on these reports are the telemetry totals. `schema_blob` joins name, server, and description. Those strings are longer than `{"name":"tool_NN"}`. The test checks that each report succeeded and that `schema_tokens` is greater than 0. That the every arm's schema tokens are at least the dynamic arm's. It doesn't assert the compare totals, because those three numbers belong to `compare`.

`task_arms` isn't what the bench times, because `dynamic_tokens` calls `compare` and `load_100` calls `load`.

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

`admit` is true while `in_flight` is below 8, so `admit(7)` is true and `admit(8)` is false. `load` calls `admit` before it increments. It resets the counter when the counter reaches 8, so the assert inside the loop stays true for a normal run. The function itself is the gate, and filling it to 8 and asking again returns false.

On the series `1..=100` the rank is index 98, because `100 * 99` is 9900, `div_ceil` by 100 is 99. The index is that rank minus one, which is the comment on the function. An empty vec returns 0 and doesn't index. `saturating_mul` together with `saturating_sub` clamp instead of panicking, which this load never needs because it passes 100 samples. `load` fills `p99_ns` from those per-call samples and the demo doesn't print it, since the samples move from run to run while the rank formula doesn't. `sort_unstable` can reorder equal keys without breaking ties by insertion order. The bench mean `122.77 µs` is the whole `load(100)`, not a p99.

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

`load` doesn't call `sample_catalog` and it doesn't call `compare`. The cards are `catalog::synthetic(32)`, the same generator `run_rename_task` uses, plugins and upstreams are empty. The session id is `load`. `search_tools` takes one query, the word `file`, which is under `MAX_QUERIES_PER_CALL`, the limit the session module sets at 7. The search reveals the hits and returns a listing, each line is a name, then ` — `, then a trimmed description. The load keeps the first line's name. `search_tools` has already revealed that name, so `call` doesn't return `NotOffered`.

`catalog::call` returns the description for a synthetic tool, the description is non-empty. Each of the 100 calls increments `successes`, which the test locks at 100. The wave counter is a plain integer on one thread, and nothing is spawned. One hundred calls with 8 in flight make 12 full waves, which is 96 calls. A partial wave, so `waves` is 13, and call 97 opens that partial wave. The `admit` check runs after the zero-test and before the increment, so the counter is 0 through 7 at the check. The reset happens after the call, which means the next check sees 0 rather than 8.

Each call records `elapsed` in nanoseconds and pushes it. After the loop `p99` stores that rank while `elapsed_ns` covers the whole function. The search sits outside the per-call samples and inside `elapsed_ns`. Catalog construction, `reload`, `open_session`, and the search are all inside `load`. So the bench that times `load` pays for them. There is no socket in this function. The crate's Rust sources have no listener, no bind, and no `oha`, so the calls are `SessionGateway::call` on the value built here.

`assert!(admit(in_flight))` panics if a wave ever asks for a ninth in-flight call without resetting. A release build of this binary still keeps `assert!`, because it isn't `debug_assert!`. The panic path isn't the measured path. The measured path resets at 8 and admits every call, the samples include the `call`, the emptiness check. The push, and they don't include `p99`, which runs once after the loop. That sort is still inside `load`, so the bench pays for it once per `load(100)`.

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

`demo` builds the 32-card catalog, compares `tool_07` with no deadline, runs the three rename tasks at catalog length 32, and loads 100. The format string is the whole line, bools print as `1` or `0` because of `as u8`. `report.p99_ns` isn't a placeholder in that string, nor is `report.elapsed_ns`. A bool casts to `0u8` or `1u8`, `format` then prints the number. The test looks for the substring `success=1/1/1` and for `task=1/1/1`. A failed arm would print `0` in that slot. The deadline path isn't what `demo` runs, because `demo` passes `None`.

`main` is five lines:

```rust
use gwservice::capstone::demo;

fn main() {
    println!("{}", demo());
}
```

Running `cargo run -p ch51-capstone --bin ch51-capstone` prints the line above. I ran it, and the line was `capstone: every=160 static=40 dynamic=5 success=1/1/1 task=1/1/1 load=100/100 waves=13`, with no key and no socket.

## 51.9 The package tests

The 23 tests are the package, five of them live in `tests/capstone.rs`. The other eighteen are the inherited files `balancing.rs`, `learned.rs`, and `service.rs`. Nextest starts six binaries: the lib harness, the `ch51-capstone` binary harness, and those four integration-test binaries. The lib and the binary have no `#[test]` functions, so the 23 tests sit in the four integration binaries.

```bash
cargo nextest run --locked -p ch51-capstone
```

```text
    Starting 23 tests across 6 binaries
     Summary [   0.087s] 23 tests run: 23 passed, 0 skipped
```

### `three_arms_share_a_success_and_not_a_token_count`

The test builds `sample_catalog`, checks the length against `CATALOG_LEN`, and takes `schema_tokens` of the first schema to confirm the count is greater than 0. It compares `tool_07` with no deadline, expects the names `every`, `static`, and `dynamic` with offered counts 32, 8, and 1, and expects every arm to succeed, with every's tokens above static's and static's above dynamic's. Every's tokens equal one card's tokens times 32, dynamic's tokens equal that one card. The test expresses static as the ordered inequality rather than as a second product, which is how the opening's totals show up without a second multiplication in the assertion. It then compares `tool_31`, where every and dynamic succeed and static fails. It compares `missing`, where every arm fails.

### `the_session_gateway_arms_each_report_tokens_and_success`

The test calls `task_arms(CATALOG_LEN)` and asserts that every `TaskReport` has `success` true and `schema_tokens` greater than 0. That the every report's schema tokens are at least the dynamic report's. It doesn't read `Arm.tokens`, so a failure here is a rename task that missed rather than a comparison of the two token measurements.

### `a_tight_deadline_fails_the_wide_arm_only`

This is the deadline guard, and the test reads the dynamic token count with no deadline, then compares `tool_07` again with `Some(dynamic)`.

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

`arms[0]` is every, `arms[1]` is static. `arms[2]` is dynamic, so the assertions fail the first two and keep the third. The limit equals the dynamic total, which means a token count sitting on that total is inside the limit and the wide totals are outside it. The needed name is still `tool_07`, which all three arms offer, and the wide arms fail because they are past the deadline.

### `the_load_stays_inside_the_bound_and_ranks_p99`

`admit(CALL_BOUND - 1)` is true and `admit(CALL_BOUND)` is false. The test then builds `1..=100` as `u128` samples and checks `p99` of that series, which is the rank from the `p99` section rather than a timed load. It calls `load(100)` and checks `successes == 100`, `requests == 100`, `waves == 13`, and `p99_ns > 0`. That last check says the rank ran and produced a non-zero sample. It doesn't pin a nanosecond count, so this chapter doesn't quote one.

### `demo_reports_three_successes`

The test calls `demo` and looks for `success=1/1/1`, `task=1/1/1`, `load=100/100`, and `waves=13`. It doesn't look for a `p99` field, because the printed line has no place for one.

### `least_outstanding_picks_the_quietest_backend`

The inherited balancer starts four backends at outstanding counts 3, 1, 4, and 2. `pick(LeastOutstanding)` returns index 1, the backend named `b`, whose `call` returns `echo via b`. A second pick is still index 1, because this test is about the copied pool rather than the token arms.

### `power_of_two_picks_the_quieter_of_the_pair`

Three backends sit at outstanding counts 5, 1, and 4, power-of-two between index 0 and 1 picks 1. Power-of-two between index 2 and 0 picks 2. Least-outstanding on that pool returns some index, and an empty pool returns `None`.

### `federation_groups_a_tool_across_upstreams`

Two upstreams, `alpha` with `echo` and `time` and `beta` with `echo`, are what `federate` groups, mapping `echo` to both servers with alpha then beta. `pool_for("echo")` has length 2 and calls `echo via alpha`, and `pool_for("missing")` is empty.

### `discovery_stays_at_four`

`discover(6, Duration::from_millis(40))` returns `DISCOVERY_CONCURRENCY`. That constant is 4, which is why the sleep makes this the slow test in the package, while the capstone load doesn't call `discover`.

### `the_demo_picks_alpha`

`gwservice::federation::demo()` equals `pick: echo via alpha`, which is the inherited federation demo, and `capstone::demo` is a different function. So the binary prints the capstone line.

### `prediction_changes_the_offered_set_and_is_not_on_the_read`

`LearnedGateway` opens `desk`, the first `session_tools` view is empty, and `list_changed` is false. A usage log of two `read_file` rows, memory that mentions notes in files. A session note that mentions `web_search` produce the ranked offer `web_search`, then `read_file`. `session_tools` returns that offer and sets `list_changed`, `predict_calls` doesn't increase on the read. A second `session_tools` clears `list_changed` while keeping the same names. This is the chapter 23 prediction path, still in the copied `lib.rs`, and the capstone doesn't call `refresh_prediction`.

### `a_zero_score_tool_is_left_out`

`predict` on a catalog of `read_file`, `web_search`, `write_file`, and `terminal`, with an unrelated task and empty memory, returns an empty vec. A score of 0 is dropped.

### `composite_and_macro_expand_to_steps`

`save_composite("rename_flow", [read_file, write_file])` and `save_macro("morning", [web_search, read_file])` round-trip through `expand`, and `expand("missing")` is `None`.

### `code_mode_refuses_a_tool_outside_the_allowlist_or_the_offer`

`run_code_mode` of `call memory {}` returns `NotAllowed("memory")`, `call web_search {}` against an offer of only `read_file` returns `NotOffered`. `call read_file {"path":"a"}` runs once and returns `ok`, while `SANDBOX_ALLOWED` contains `terminal` and doesn't contain `memory`.

### `code_mode_stops_at_fifty_calls`

A script of `CODE_MODE_MAX_CALLS + 1` lines, and that constant is 50, returns `TooManyCalls`. The check happens before the calls run.

### `the_scripted_model_is_optional_and_keyless`

A fresh `ScriptedModel` reports 0 calls, and `gwservice::demo()` returns the offered name `read_file` and the body `ok`. That `demo` is the learned-gateway demo in `lib.rs`, not `capstone::demo`. Completing the scripted model once returns `unused` and bumps the call count to 1, with no key read.

### `kill_moves_the_session_to_the_next_replica`

`Service::new` on replicas `a` and `b` places session `s1` on `a`, `reload` bumps the generation from 1 to 2. The held config arc's strong count is 1 after the reload. `kill("a")` routes `s1` to `b`, and the service stays healthy and ready.

### `quota_audit_and_drain`

`bearer_ok` accepts `Bearer token` and rejects `token` and `Bearer ` with the trailing space and nothing after it. The first `accept` returns an id, a second session for the same tenant hits `Quota`, a bad token hits `BadToken`, and the audit line starts with `accept `. The span list is `gateway.call 1`, `start_drain` clears ready, a new accept hits `Draining`. `finish` drains the inflight count to 0. A `FakeModel` completes `a` to `ok`, this service is the copied chapter 42 router, and `load` doesn't construct it.

### `a_dead_pool_refuses_the_call`

Killing the only replica makes `healthy` false, and `accept` returns `NoReplica`.

### `sampling_stops_at_ten_per_minute`

`SAMPLING_MAX_RPM` is 10, the loop allows that many calls at timestamp 0, the next `allow(0)` is false, and `allow(60)` is true.

### `url_elicitation_is_declined`

`elicit(false)` is an approval with `ELICIT_TIMEOUT_SECONDS`, `elicit(true)` is `Declined`, and a URL elicitation doesn't proceed.

### `osv_blocks_malware_and_fails_open_on_the_network`

An `echo` check is `Skipped` and leaves the cache empty, a clean `npx` result is `Clean`. A later check inside the TTL that can't reach the network stays `Clean`. A `uvx` package with a malware id and no network is `FailOpen` and occupies one cache slot, the same package with a network and `MAL-9` is `Blocked`. A check one second inside `OSV_CACHE_TTL_SECONDS` still returns the blocked malware id.

### `demo_routes_to_the_first_replica`

`service::demo()` equals `route: a`, and the capstone binary doesn't print that line.

## Build, run, and benchmark

The run command and the line are in the sections above, and the bench command is the one the workflow uses.

```bash
cargo bench -p ch51-capstone --bench capstone --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `dynamic_tokens` | `264.8 ns` | the dynamic arm's token count, which is 5 | `compare` of `tool_07`, then `arms[2].tokens` |
| `load_100` | `122.77 µs` | 100 in-memory calls, the sort, and `p99` | `load(100)` |

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

`load_100` has no setup outside the iteration, so every sample calls `load(100)` and then `black_box` of the report. The means are `mean.point_estimate` from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), section `51`. The bench ids are `51/dynamic_tokens`, `51/load_100`, `49/dynamic_tokens`, `47/dynamic_tokens`, `49/load_100`, and `47/load_100`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The two token totals are different measurements. `compare` sums the 18-byte schemas into the totals from the opening while `run_rename_task` sums `schema_blob` text through the session telemetry. Printing the task schema tokens on the demo line would show a different triple. So the demo prints the `compare` triple and the task success bits.

Integer `div_ceil` makes an 18-byte schema into 5 tokens, and dropping the ceil would make 4. So the arms would be 128, 32, and 4. Hermes calls `math.ceil` on a float. For this length the two roundings agree, but they aren't the same function. A length that is a multiple of 4 agrees either way and a length with a remainder is where `div_ceil` and truncating division split.

`assert!(admit(...))` inside `load` aborts the process if the bound is broken, and the bench assumes the happy path. A change that stopped resetting at 8 would fail the bench by panicking. It would fail `the_load_stays_inside_the_bound_and_ranks_p99` only if that panic happened inside `load(100)`. `admit(8)` is checked on its own before `load`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The catalog does not have a row for this capstone. The behaviors below are the ones `capstone.rs` actually implements.

**Tokens.** Four characters per token, through `session::approx_tokens`. The arms report the sum and a success bit. Success requires the needed tool to be in the offered set.

**Deadline.** A limit equal to the dynamic total fails every and static and keeps dynamic. That is `a_tight_deadline_fails_the_wide_arm_only`. The wide arms are past the limit. The dynamic arm is on the limit, and `<=` admits it. The tool is still offered on the arms that fail. The failure is the deadline.

**Load bound.** `CALL_BOUND` is 8. `admit(8)` is false. The load resets at 8, so 100 calls are 13 waves and 100 successes. No socket is opened. `oha` is not run.

**Left for later.** A listening socket, an `oha` run, and a remote catalog. Chapter 52 does not add them. Chapter 52 copies the harness, not this crate.

## What you implement

`my-harness/tests/ch51.rs` is the milestone for this chapter.

```rust
use my_harness::chapter_51_arm_succeeded;

#[test]
fn each_arm_reports_success() {
    assert!(chapter_51_arm_succeeded("every"));
    assert!(chapter_51_arm_succeeded("static"));
    assert!(chapter_51_arm_succeeded("dynamic"));
}
```

The stub panics until the milestone is filled in.

```rust
pub fn chapter_51_arm_succeeded(name: &str) -> bool {
    let _ = name;
    todo!("chapter 51")
}
```

`todo!("chapter 51")` is the panic, and the test asks the function about `every`, `static`, and `dynamic`, wanting `true` for each. In the step crate that is `compare` of `tool_07` with no deadline, where all three `Arm.success` bits are true. `task_arms` reports the same three successes from `run_rename_task`, while the stub stays `todo!`.

## Chapter summary

- `compare` scores every card, the first 8, and the one named card. For `tool_07` the tokens are 160, 40, and 5, with all three succeeding.
- `approx_tokens` is `len.div_ceil(4)`. So an 18-byte schema is 5 tokens, which is the rule the opening applied to the sample.
- `task_arms` runs the rename task on `SessionGateway` three times, all three succeed. Their schema tokens are the telemetry blobs rather than the compare totals.
- One hundred in-memory calls at bound 8 produce 13 waves, and `p99` is computed and left off the printed line.
- `mcp-gateway` re-exports `LearnedGateway` as `Gateway`, and the process binds no socket.
- `dynamic_tokens` has mean `264.8 ns` and `load_100` has mean `122.77 µs`, and both figures are `mean.point_estimate`.

## Where the server line stands

The server line is a separate program from the harness, and the measurement of that program stops in this chapter. Chapter 5 is the minimal server, a fixed list and a call gate that binds no socket. Chapter 7 is the catalog, reloaded behind an `Arc` so an in-flight search keeps the index it started with. Chapter 8 is the session toolset, where a session sees a smaller set than the whole catalog and the three offer shapes first show up. Chapter 15 is the limit and the breaker on that gateway. Chapter 23 is the learned offer: usage, memory.

A session note can change the names `session_tools` returns, a zero score stays out of the set. The read itself doesn't call `predict`. Chapter 37 is pools and discovery, least-outstanding and power-of-two across backends, one tool name grouped across upstreams, and discovery held at four. Chapter 42 is the service front this crate copies, with `route` by session id, a drain that stops new accepts and lets accepted calls finish, sampling capped at ten a minute, URL elicitation declined. An in-memory malware cache that fails open when the network is down.

This chapter copies that crate, adds `capstone`, and compares the three offers on 32 cards of 18 bytes, then runs an in-memory load of 100 through `SessionGateway`. The dynamic arm is the small offer that still finishes the task. A deadline equal to that arm's total is what fails the wide arms while the dynamic arm stays successful. The load admits eight calls in a wave, reports 13 waves, and fills `p99_ns` without printing it. `mcp-gateway` re-exports `LearnedGateway` from this library as `Gateway`, beside the session types, and the library is still `gwservice`. The process still binds no socket. So there is still no `oha` run. The token count is still integer `div_ceil` on those schemas rather than a tokenizer or the 10,000-tool list from chapter 6.

That is the server line as the book leaves it: a catalog, a session toolset, limits, a learned offer, pools and discovery, a service front, and this comparison of three offers plus the load of 100. The limits that were already true stay true here, which means no socket, no `oha`, no tokenizer, and 32 cards of 18 bytes. The harness continues one chapter past the eval suite. Chapter 53 copies that harness, not this crate, and runs one task twice.

## What's next

Chapter 52 re-runs the saved tasks on the tracing harness, not on this crate. Chapter 53 is the harness after that, and it still doesn't copy this directory.
