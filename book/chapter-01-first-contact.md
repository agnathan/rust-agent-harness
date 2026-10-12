# Chapter 1: First Contact

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Every agent framework I've looked at comes down to one repeated act: send a list of messages to a language model and get a message back. This chapter builds that act in three small steps. Each step compiles, has its own tests, and has a criterion benchmark. Later steps copy the earlier code forward, so section 1.1 still builds after 1.3 exists.

[personal note: where I was with Rust when I started this chapter]

The code lives in three directories:

| Section | Step | What it adds |
|---|---|---|
| 1.1 | `steps/ch01-1-messages` | `Message` and `Role`, serialized to OpenAI JSON |
| 1.2 | `steps/ch01-2-provider` | `Provider` and a latency-configurable `MockProvider` on tokio |
| 1.3 | `steps/ch01-3-cli` | a shared `reqwest` client and `harness ask` |

I'm copying three ideas from Hermes Agent (Nous Research): OpenAI-style messages are the internal format, "talk to a model" sits behind one interface, and the rest of the harness never needs to know which provider it is talking to. Hermes's breadth (credential pools, failover, streaming) waits until a later chapter needs it.

## 1.1 Messages

Rust projects are managed by cargo. Section 1.1 is one library crate, `ch01-1-harness-core`. The library name inside the crate is still `harness_core`, which is what the tests import. The package name carries the section so it can sit in the repo workspace next to later chapters.

```bash
cargo test -p ch01-1-harness-core
cargo bench -p ch01-1-harness-core --bench serde_messages
```

> **Rust sidebar: cargo**
> - Cargo is the build tool, package manager, and test runner.
> - `Cargo.toml` is the manifest. `[dependencies]` names crates from crates.io.
> - A workspace shares one `Cargo.lock` and one `target/` directory.
> - `resolver = "3"` prefers dependency versions that match the declared `rust-version` (`1.85`).

A message is a role and some text. This section needs three roles. `tool` arrives with tool calling.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}
```

The body I will eventually POST is a model name plus those messages. `ChatRequest` borrows them, so building the body does not clone the conversation. `OwnedChatRequest` is the same shape with owned data, which is what serde gives back when a test parses the JSON.

```rust
#[derive(Debug, Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [Message],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}
```

> **Rust sidebar: structs, enums, and serde**
> - A `struct` is a named bundle of fields. An `enum` is exactly one of its variants, so a role cannot be an arbitrary string.
> - `#[derive(...)]` asks the compiler to write `Debug`, `Clone`, `PartialEq`, and the serde impls.
> - `#[serde(rename_all = "lowercase")]` makes `Role::User` serialize as `"user"`.
> - `impl Into<String>` lets a caller pass `"hi"` or an owned `String`.
> - `'a` on `ChatRequest` is a lifetime: the request must not outlive the messages it borrows.

The test builds 50 messages, serializes a `ChatRequest`, and parses it back into an `OwnedChatRequest`. That is the same work the benchmark times.

> **Performance budget**
>
> - **Target:** serialize and deserialize a 50-message request in under 1 ms.
> - **Measured:** `serde_50_message_request` mean **6.937 µs** (criterion, sample size 10, warm-up 1 s, measurement 2 s). The run is pasted below. CI reprints the current mean in the Benchmarks job summary.

### Build, run, and benchmark

From the repository root. This step is the library crate `ch01-1-harness-core`. A binary shows up in section 1.3.

```bash
cargo build -p ch01-1-harness-core
cargo nextest run -p ch01-1-harness-core
cargo bench -p ch01-1-harness-core --bench serde_messages -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Nothing in this step reads an API key or opens a socket.

I ran those commands on a 4-vCPU Intel Xeon guest under KVM, with 16 GiB of RAM, Linux, rustc 1.99.0, and cargo-nextest 0.9.148. The build finished with:

```text
   Compiling ch01-1-harness-core v0.1.0 (/workspace/steps/ch01-1-messages/crates/harness-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.19s
```

Nextest, after it finished compiling the tests:

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.39s
────────────
 Nextest run ID 733dee03-0cb6-4c6c-ac97-14ec7ef9c8c6 with nextest profile: default
    Starting 2 tests across 2 binaries
        PASS [   0.003s] (1/2) ch01-1-harness-core::message message_serializes_to_openai_shape
        PASS [   0.003s] (2/2) ch01-1-harness-core::message request_round_trips_fifty_messages
────────────
     Summary [   0.004s] 2 tests run: 2 passed, 0 skipped
```

Criterion prints a slope on the `time:` line. The mean in the table is the mean point estimate from that same run. The "change" line is a comparison with a previous baseline on this machine.

```text
    Finished `bench` profile [optimized] target(s) in 0.04s
     Running benches/serde_messages.rs (target/release/deps/serde_messages-359541fb9d74961e)
Gnuplot not found, using plotters backend
Benchmarking serde_50_message_request
Benchmarking serde_50_message_request: Warming up for 1.0000 s
Benchmarking serde_50_message_request: Collecting 10 samples in estimated 2.0003 s (320k iterations)
Benchmarking serde_50_message_request: Analyzing
serde_50_message_request
                        time:   [6.9286 µs 6.9349 µs 6.9442 µs]
                        change: [-0.3709% +0.0062% +0.3353%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `serde_50_message_request` | 6.937 µs | < 1 ms | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0 |

CI runs this step's tests on Ubuntu, macOS, and Windows. Criterion runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml), and each run writes the means into the job summary. The green summary for this branch is [run 38213430828](https://github.com/agnathan/rust-agent-harness/actions/runs/38213430828).

## 1.2 Provider

Section 1.2 copies the message types and adds the trait every later chapter will call. The library still does not open a socket. tokio is a dependency only for `sleep` in the mock. The tests and the benchmark start the runtime.

```bash
cargo test -p ch01-2-harness-core
cargo bench -p ch01-2-harness-core --bench mock_calls
```

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    /// Send the conversation, get back the assistant's reply.
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError>;
}
```

> **Rust sidebar: traits and async**
> - A trait is a set of methods a type promises to implement. Code written against `Provider` works for the mock now and the HTTP client in section 1.3.
> - `Send + Sync` means a provider can move across threads and be shared. That is what lets section 1.2 spawn 100 calls.
> - An `async fn` returns a future. It runs when something `.await`s it. While it waits, the thread can run other tasks.
> - `#[async_trait]` rewrites the method so the trait can be used as `dyn Provider`. The cost is one allocation per call, which is noise next to a model round-trip.
> - tokio is the runtime. `#[tokio::test]` and `#[tokio::main]` start it. The benchmark builds a multi-thread runtime with four workers.

`ProviderError` is a small enum. Section 1.2 has no HTTP crate yet, so the network variant holds a `String`. Section 1.3 changes that variant to wrap `reqwest::Error`.

The mock records every conversation it sees and can wait before it answers. The wait is how later benches stand in for model latency. Overhead benches pass `Duration::ZERO` and do not count a sleep.

```rust
pub struct MockProvider {
    reply: String,
    latency: Duration,
    pub seen: Mutex<Vec<Vec<Message>>>,
}

impl MockProvider {
    pub fn new(reply: impl Into<String>) -> Self {
        Self::with_latency(reply, Duration::ZERO)
    }

    pub fn with_latency(reply: impl Into<String>, latency: Duration) -> Self {
        Self {
            reply: reply.into(),
            latency,
            seen: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        if !self.latency.is_zero() {
            tokio::time::sleep(self.latency).await;
        }
        self.seen.lock().expect("seen").push(messages.to_vec());
        Ok(Message::assistant(self.reply.clone()))
    }
}
```

`complete` takes `&self`, so the mock can be shared. `seen` sits behind a `Mutex` because shared data in Rust is not mutated without a lock. The sleep happens before the lock, so the lock is not held across the wait.

A blanket impl makes `Arc<MockProvider>` a provider too. The concurrency test wraps the mock in an `Arc`, then uses a `JoinSet` to spawn 100 tasks that each sleep 80 ms. If those calls ran one after another they would take about eight seconds. They finish in well under two, which is the test's assertion. The benchmark does the same shape with zero latency.

> **Rust sidebar: `Arc` and `JoinSet`**
> - `Arc` is a shared pointer. Cloning it bumps a counter. It does not clone the mock.
> - `JoinSet` is a tokio collection of spawned tasks. `join_next` waits for whichever task finishes first.
> - Spawned tasks must be `Send`. The mock is `Send` because its mutex is the standard-library one, and `sleep` is `Send`.

> **Performance budget**
>
> - **Target:** harness overhead for one zero-latency mock call under 1 ms. One hundred concurrent zero-latency calls under 50 ms.
> - **Measured:** `mock_turn_overhead` mean **190.9 ns**. `mock_100_concurrent` mean **103.8 µs** (same criterion settings as section 1.1). The single-call samples on this guest spread out; both means are still inside the targets.

### Build, run, and benchmark

From the repository root. Still a library crate: `ch01-2-harness-core`.

```bash
cargo build -p ch01-2-harness-core
cargo nextest run -p ch01-2-harness-core
cargo bench -p ch01-2-harness-core --bench mock_calls -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The mock never contacts a network, so there is no API key to set.

Same machine as section 1.1. The build finished with:

```text
   Compiling ch01-2-harness-core v0.1.0 (/workspace/steps/ch01-2-provider/crates/harness-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.88s
```

Nextest:

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.09s
────────────
 Nextest run ID 3efce048-4401-446d-9d56-6a660e627e55 with nextest profile: default
    Starting 3 tests across 2 binaries
        PASS [   0.003s] (1/3) ch01-2-harness-core::provider mock_provider_answers_and_records
        PASS [   0.044s] (2/3) ch01-2-harness-core::provider mock_latency_is_configurable
        PASS [   0.085s] (3/3) ch01-2-harness-core::provider one_hundred_mock_calls_run_concurrently
────────────
     Summary [   0.085s] 3 tests run: 3 passed, 0 skipped
```

The bench again. `mock_turn_overhead` moved relative to an older baseline on this machine; the mean below is from this run.

```text
    Finished `bench` profile [optimized] target(s) in 0.07s
     Running benches/mock_calls.rs (target/release/deps/mock_calls-d656576a78ecd511)
Gnuplot not found, using plotters backend
Benchmarking mock_turn_overhead
Benchmarking mock_turn_overhead: Warming up for 1.0000 s
Benchmarking mock_turn_overhead: Collecting 10 samples in estimated 2.0000 s (12M iterations)
Benchmarking mock_turn_overhead: Analyzing
mock_turn_overhead      time:   [148.23 ns 176.46 ns 211.46 ns]
                        change: [+26.886% +48.340% +64.246%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking mock_100_concurrent
Benchmarking mock_100_concurrent: Warming up for 1.0000 s
Benchmarking mock_100_concurrent: Collecting 10 samples in estimated 2.0058 s (19k iterations)
Benchmarking mock_100_concurrent: Analyzing
mock_100_concurrent     time:   [102.54 µs 103.34 µs 104.16 µs]
                        change: [-1.5468% +0.0114% +1.6063%] (p = 0.99 > 0.05)
                        No change in performance detected.
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `mock_turn_overhead` | 190.9 ns | < 1 ms | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0 |
| `mock_100_concurrent` | 103.8 µs | < 50 ms | same machine |

CI runs these tests on Ubuntu, macOS, and Windows. Criterion runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml), and the means land in the job summary ([run 38213430828](https://github.com/agnathan/rust-agent-harness/actions/runs/38213430828)).

## 1.3 The client and `harness ask`

Section 1.3 copies 1.2 and adds the HTTP provider plus a thin binary. The package names are `ch01-3-harness-core` and `ch01-3-harness-cli`. The executable file is `ch01-harness` so it does not collide with chapter 2 in the shared workspace. The clap command name is still `harness`.

```bash
cargo test -p ch01-3-harness-core -p ch01-3-harness-cli
cargo run -p ch01-3-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-3-harness-core --bench request_parse
```

`OpenAiCompatible` owns one `reqwest::Client` for its whole life. reqwest pools idle connections per host (`pool_max_idle_per_host(8)`), so the second call can reuse the first connection. The `http2` feature and `http2_adaptive_window(true)` turn HTTP/2 on for servers that negotiate it: ALPN when the connection is TLS. A local fake speaks HTTP/1.1, and the same client talks to it without a separate code path.

```rust
pub struct OpenAiCompatible {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAiCompatible {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(8)
            .http2_adaptive_window(true)
            .build()
            .expect("HTTP client");
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
}
```

`complete` POSTs `{base_url}/chat/completions`, parses `choices[0].message`, and turns a non-success status into `ProviderError::Api`. serde ignores the extra fields a real response carries (`id`, `usage`, `finish_reason`).

```rust
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}
```

> **Rust sidebar: `Result` and `?`**
> - A fallible function returns `Result<T, E>`. There are no exceptions.
> - `?` returns the error from the current function, or unwraps the value. `#[from]` on `ProviderError::Network` converts `reqwest::Error` so `.send().await?` type-checks.

The binary parses flags and calls the provider once. `--mock` uses `MockProvider`. Otherwise it builds `OpenAiCompatible` from `--base-url`, `--api-key`, and `--model` (or the `OPENAI_BASE_URL`, `OPENAI_API_KEY`, and `HARNESS_MODEL` environment variables).

The integration test does not use a real key. The `wiremock` crate starts a local server, mounts `POST /chat/completions`, and the provider calls it twice on the same client. A second test points the `ch01-harness ask` binary at `POST /v1/chat/completions` on that same kind of server. Both run on Ubuntu, macOS, and Windows in CI.

> **Performance budget**
>
> - **Target:** build a 50-message request and parse a response in under 1 ms, with no network in the measurement.
> - **Measured:** `build_request_and_parse_response` mean **1.746 µs** (same criterion settings). The wiremock test checks behavior, not this number.

### Build, run, and benchmark

From the repository root. Two packages: the library `ch01-3-harness-core` and the binary crate `ch01-3-harness-cli`. The executable name is `ch01-harness`.

```bash
cargo build -p ch01-3-harness-core
cargo build -p ch01-3-harness-cli
cargo nextest run -p ch01-3-harness-core
cargo nextest run -p ch01-3-harness-cli
cargo run -p ch01-3-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-3-harness-core --bench request_parse -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

`--mock` stays on this machine and prints a canned reply. A live call is the same command without `--mock`. The key and the optional base URL and model name come from the environment, or from the matching flags:

```bash
OPENAI_API_KEY=sk-... OPENAI_BASE_URL=https://api.openai.com/v1 HARNESS_MODEL=gpt-4o-mini \
  cargo run -p ch01-3-harness-cli -- ask "Say hello"
```

I did not make that live call. The run below is the mock. `--help` is how the binary reports the same variables:

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/ch01-harness --help`
A tiny agent harness

Usage: ch01-harness [OPTIONS] <COMMAND>

Commands:
  ask   Send one prompt and print the reply
  help  Print this message or the help of the given subcommand(s)

Options:
      --base-url <BASE_URL>  Base URL of an OpenAI-compatible API [env: OPENAI_BASE_URL=] [default: https://api.openai.com/v1]
      --api-key <API_KEY>    API key. A local fake server accepts any value [env: OPENAI_API_KEY=] [default: ""]
      --model <MODEL>        Model name [env: HARNESS_MODEL=] [default: gpt-4o-mini]
      --mock                 Use the offline mock provider instead of HTTP
  -h, --help                 Print help
```

The library build and the binary build:

```text
   Compiling ch01-3-harness-core v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.18s
```

```text
   Compiling ch01-3-harness-cli v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.64s
```

Nextest on the library, then on the binary. Both talk to a local wiremock server, still with no API key.

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.03s
────────────
 Nextest run ID 44f76dcf-ec41-4106-9bb9-b6969e129b69 with nextest profile: default
    Starting 3 tests across 2 binaries
        PASS [   0.004s] (1/3) ch01-3-harness-core::provider response_deserializes_from_openai_shape
        PASS [   0.005s] (2/3) ch01-3-harness-core::provider http_error_is_classified
        PASS [   0.005s] (3/3) ch01-3-harness-core::provider two_calls_share_the_client_against_a_local_server
────────────
     Summary [   0.006s] 3 tests run: 3 passed, 0 skipped
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.70s
────────────
 Nextest run ID 25c9fdc9-dcb0-4b40-a58d-e8e66ab2d5a8 with nextest profile: default
    Starting 2 tests across 2 binaries
        PASS [   0.005s] (1/2) ch01-3-harness-cli::ask mock_ask_prints_the_canned_reply
        PASS [   0.009s] (2/2) ch01-3-harness-cli::ask ask_talks_to_a_local_fake_server
────────────
     Summary [   0.009s] 2 tests run: 2 passed, 0 skipped
```

The mock ask:

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/ch01-harness --mock ask 'Say hello'`
(mock) I heard you.
```

And the bench. The `time:` line is Criterion's slope. The mean in the table is the mean point estimate from the same run.

```text
    Finished `bench` profile [optimized] target(s) in 0.12s
     Running benches/request_parse.rs (target/release/deps/request_parse-359f4e4892eb2854)
Gnuplot not found, using plotters backend
Benchmarking build_request_and_parse_response
Benchmarking build_request_and_parse_response: Warming up for 1.0000 s
Benchmarking build_request_and_parse_response: Collecting 10 samples in estimated 2.0001 s (1.1M iterations)
Benchmarking build_request_and_parse_response: Analyzing
build_request_and_parse_response
                        time:   [1.7411 µs 1.7474 µs 1.7568 µs]
                        change: [+0.0489% +0.4928% +0.9420%] (p = 0.05 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `build_request_and_parse_response` | 1.746 µs | < 1 ms | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0 |

CI runs these tests on Ubuntu, macOS, and Windows. Criterion runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml), and the means land in the job summary ([run 38213430828](https://github.com/agnathan/rust-agent-harness/actions/runs/38213430828)).

## What you implement

`my-harness/tests/ch01_1.rs`, `ch01_2.rs`, and `ch01_3.rs` are the milestone specs. They compile, then panic in `todo!` until the stubs in `my-harness/src` match these steps. The snapshots under `steps/` stay as the reference.

[personal note: a moment where the compiler stopped me, and how I worked out what it wanted]

## What's next

One question and one answer is not a conversation. Chapter 2 keeps history: a `Conversation` that owns its messages, and a `chat` REPL. The system prompt from this chapter stays byte-stable across those turns, which is what makes prompt caching possible later.
