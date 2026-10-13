# Chapter 1: First Contact

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Every agent framework I've looked at comes down to one repeated act: send a list of messages to a language model and get a message back. This chapter builds that act in four small steps. Each step compiles, has its own tests, and has a criterion benchmark. Later steps copy the earlier code forward, so section 1.1 still builds after 1.4 exists.

[personal note: where I was with Rust when I started this chapter]

The code lives in four directories:

| Section | Step | What it adds |
|---|---|---|
| 1.1 | `steps/ch01-1-messages` | `Message` and `Role`, serialized to OpenAI JSON |
| 1.2 | `steps/ch01-2-provider` | `Provider`, one `Event` enum, and a latency-configurable mock |
| 1.3 | `steps/ch01-3-cli` | OpenRouter by default, the same events, usage and cost |
| 1.4 | `steps/ch01-4-errors` | a request timeout, and the edges of a buffered reply |

I'm copying three ideas from Hermes Agent (Nous Research): OpenAI-style messages are the internal format, "talk to a model" sits behind one interface, and the rest of the harness never needs to know which provider it is talking to. The client speaks that one JSON shape, and OpenRouter is the default server, so a model id such as `openai/gpt-4o-mini` or `anthropic/claude-3.5-sonnet` is a flag, not a new adapter. Native Anthropic and Bedrock adapters, OpenRouter routing, and key rotation wait until the providers chapter. Streaming folds the events this chapter already emits.

> **How Hermes does it**
>
> Hermes puts each provider behind one profile (`providers/base.py`). The agent loop (`agent/conversation_loop.py`) is that call, repeated: messages in, a message out, then tools, until the model stops. This chapter's `Provider` is that profile. The Rust side adds one `Event` enum (`TextDelta`, `ToolCall`, `Usage`, `Done`, `Error`). The mock and the buffered OpenRouter client emit it now, so later streaming, the gateway, and tracing consume the same values. Hermes's credential pools and failover stay in the providers chapter.

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

CI runs this step's tests on Ubuntu, macOS, and Windows. Criterion runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml). Each run uploads a JSON artifact of the means and, when the job can publish, charts them. The table above is from this machine.

### Milestone demo

Mock is the whole demo. There is no model call in this section.

```bash
cargo run -p ch01-1-harness-core --example demo
```

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/demo`
{"model":"openai/gpt-4o-mini","messages":[{"role":"system","content":"You are a helpful assistant."},{"role":"user","content":"Say hello"}]}
```

## 1.2 Provider

Section 1.2 copies the message types and adds the trait every later chapter will call. The library still does not open a socket. tokio is a dependency only for `sleep` in the mock. The tests and the benchmark start the runtime.

```bash
cargo test -p ch01-2-harness-core
cargo bench -p ch01-2-harness-core --bench mock_calls
```

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError>;

    async fn complete_events(&self, messages: &[Message]) -> Result<Vec<Event>, ProviderError> {
        let message = self.complete(messages).await?;
        Ok(events_for_reply(&message, None, None))
    }
}
```

`complete` is still the reply. `complete_events` is the same call as the events every later chapter folds. The mock emits two: `TextDelta` with the canned text, then `Done`. It does not invent usage or a model id. `ToolCall` is in the enum so tool-calling does not create a second event type. `Error` is how a failure is reported to the same consumer. Nothing in this section emits those two yet.

```rust
pub enum Event {
    TextDelta(String),
    ToolCall { id: String, name: String, arguments: String },
    Usage(Usage),
    Done { message: Message, model: Option<String> },
    Error(String),
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
> - **Measured:** `mock_turn_overhead` mean **235.7 ns** (the bench now builds the event list). `mock_100_concurrent` mean **108.9 µs**. Both are inside the targets.

### Build, run, and benchmark

From the repository root. Still a library crate: `ch01-2-harness-core`.

```bash
cargo build -p ch01-2-harness-core
cargo nextest run -p ch01-2-harness-core
cargo run -p ch01-2-harness-core --example demo
cargo bench -p ch01-2-harness-core --bench mock_calls -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The mock never contacts a network, so there is no API key to set.

The milestone demo is the mock. It prints the two events and does not call OpenRouter.

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/demo`
event: text-delta (mock) I heard you.
event: done
```

Nextest:

```text
   Compiling ch01-2-harness-core v0.1.0 (/workspace/steps/ch01-2-provider/crates/harness-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
────────────
 Nextest run ID 0ce6d5a2-f8b6-492c-a3f5-a61396c8fe3f with nextest profile: default
    Starting 4 tests across 2 binaries
        PASS [   0.003s] (1/4) ch01-2-harness-core::provider mock_provider_answers_and_records
        PASS [   0.003s] (2/4) ch01-2-harness-core::provider mock_emits_text_then_done
        PASS [   0.044s] (3/4) ch01-2-harness-core::provider mock_latency_is_configurable
        PASS [   0.085s] (4/4) ch01-2-harness-core::provider one_hundred_mock_calls_run_concurrently
────────────
     Summary [   0.086s] 4 tests run: 4 passed, 0 skipped
```

The bench. `mock_turn_overhead` builds the event list. The `time:` line is the slope. The means are the point estimates from this run: 235.7 ns and 108.9 µs.

```text
   Compiling ch01-2-harness-core v0.1.0 (/workspace/steps/ch01-2-provider/crates/harness-core)
    Finished `bench` profile [optimized] target(s) in 1.46s
     Running benches/mock_calls.rs (target/release/deps/mock_calls-d656576a78ecd511)
Gnuplot not found, using plotters backend
Benchmarking mock_turn_overhead
Benchmarking mock_turn_overhead: Warming up for 1.0000 s
Benchmarking mock_turn_overhead: Collecting 10 samples in estimated 2.0000 s (7.0M iterations)
Benchmarking mock_turn_overhead: Analyzing
mock_turn_overhead      time:   [216.00 ns 230.26 ns 242.65 ns]
                        change: [+10.092% +23.448% +42.103%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking mock_100_concurrent
Benchmarking mock_100_concurrent: Warming up for 1.0000 s
Benchmarking mock_100_concurrent: Collecting 10 samples in estimated 2.0026 s (19k iterations)
Benchmarking mock_100_concurrent: Analyzing
mock_100_concurrent     time:   [107.23 µs 110.51 µs 112.81 µs]
                        change: [+1.1333% +4.9393% +8.5429%] (p = 0.02 < 0.05)
                        Performance has regressed.
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `mock_turn_overhead` | 235.7 ns | < 1 ms | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0 |
| `mock_100_concurrent` | 108.9 µs | < 50 ms | same machine |

CI runs these tests on Ubuntu, macOS, and Windows. Criterion runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml), and the means land in the job summary and in a per-commit JSON artifact. The table is from this machine.

## 1.3 OpenRouter, the client, and `harness ask`

Section 1.3 copies 1.2 and adds the HTTP provider plus a thin binary. The package names are `ch01-3-harness-core` and `ch01-3-harness-cli`. The executable file is `ch01-harness` so it does not collide with chapter 2 in the shared workspace. The clap command name is still `harness`.

```bash
cargo test -p ch01-3-harness-core -p ch01-3-harness-cli
cargo run -p ch01-3-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-3-harness-core --bench request_parse
```

The default server is OpenRouter, `https://openrouter.ai/api/v1`. The default model id is `openai/gpt-4o-mini`. `--model` takes any OpenRouter id (`anthropic/claude-3.5-sonnet`, `meta-llama/llama-3.1-8b-instruct`, and the rest of that catalog). The request body is still the OpenAI chat-completions shape, and it does not set `stream`. `--base-url` points the same client at another OpenAI-compatible server: OpenAI, a local Ollama at `http://127.0.0.1:11434/v1`, or the wiremock server in the tests.

Which key is required depends on the host:

- `openrouter.ai` reads `OPENROUTER_API_KEY`. A missing key is an error that names that variable. `--mock` is the offline path.
- Any other host reads `OPENAI_API_KEY`, which may be empty. Ollama and the local fake accept that. An empty key omits `Authorization`.
- `--api-key` overrides both variables.

OpenRouter also accepts two optional attribution headers, `HTTP-Referer` and `X-Title`, from `--http-referer` / `OPENROUTER_HTTP_REFERER` and `--title` / `OPENROUTER_TITLE`. They are sent only when set.

`OpenAiCompatible` owns one `reqwest::Client` for its whole life. reqwest pools idle connections per host (`pool_max_idle_per_host(8)`), so the second call can reuse the first connection. The `http2` feature and `http2_adaptive_window(true)` turn HTTP/2 on for servers that negotiate it: ALPN when the connection is TLS. A local fake speaks HTTP/1.1, and the same client talks to it without a separate code path. This section does not set a request timeout. Section 1.4 does. Retries are chapter 12.

```rust
pub fn from_endpoint(endpoint: &Endpoint) -> Self {
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(8)
        .http2_adaptive_window(true)
        .build()
        .expect("HTTP client");
    Self {
        client,
        base_url: endpoint.base_url.trim_end_matches('/').to_string(),
        api_key: endpoint.api_key.clone(),
        model: endpoint.model.clone(),
        http_referer: endpoint.http_referer.clone(),
        title: endpoint.title.clone(),
    }
}
```

`complete_detailed` POSTs `{base_url}/chat/completions`. A non-success status becomes `ProviderError::Api`. An empty `choices` array becomes `ProviderError::EmptyResponse`. The reply text is `choices[0].message`. The model that actually answered is the response's `model` field, which can differ from the id we sent. Token counts are the optional `usage` object. serde still ignores the other fields (`id`, `finish_reason`).

```rust
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}
```

`complete_events` on the HTTP client is one request, then the same enum the mock uses: `TextDelta` for the whole reply, `Usage` when the server sent token counts, and `Done` carrying the answered model id. OpenRouter's `usage.cost` is USD. Other servers omit it, and the field stays `None`. `AgentUsage` adds those numbers across the calls one agent has made. A call that reports no cost does not wipe a cost already recorded.

The CLI prints the content, then `model: …` when `Done` has one, then the ledger (`tokens:`, `cost:`, `agent:`), then one line per event. The mock prints the canned reply and two event lines. It does not invent a model id, token counts, or a cost.

> **Rust sidebar: `Result` and `?`**
> - A fallible function returns `Result<T, E>`. There are no exceptions.
> - `?` returns the error from the current function, or unwraps the value. `#[from]` on `ProviderError::Network` converts `reqwest::Error` so `.send().await?` type-checks.

> **Rust sidebar: `Option`**
> - `Option<String>` is either `Some(value)` or `None`. It is the type for "the server might not have sent this."
> - `if let Some(model) = &detailed.model` prints the line only when the field is present. A missing `usage` object stays `None` and the reply text is still kept.

The integration tests do not use a real key. `wiremock` starts a local server. One test posts twice on the same client and checks the answered model and `usage`. Another checks that the request JSON has no `stream` field. Another checks the attribution headers. The CLI test points `ch01-harness ask` at `POST /v1/chat/completions` on that server and reads `model:` and `tokens:` from stdout. A missing `OPENROUTER_API_KEY` is a failed process whose stderr names the variable. All of this runs on Ubuntu, macOS, and Windows in CI.

> **Performance budget**
>
> - **Target:** build a 50-message request and parse a response in under 1 ms, with no network in the measurement.
> - **Measured:** `build_request_and_parse_response` mean **2.133 µs**. The body includes `model`, `usage`, and `cost`, and the bench builds the event list. The wiremock tests check behavior, not this number.

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

`--mock` stays on this machine and prints a canned reply. I did not make a live call. A live OpenRouter call is the same command without `--mock`, with `OPENROUTER_API_KEY` set. Another server is a different base URL and `OPENAI_API_KEY`:

```bash
OPENROUTER_API_KEY=sk-or-... HARNESS_MODEL=openai/gpt-4o-mini \
  cargo run -p ch01-3-harness-cli -- ask "Say hello"

OPENAI_API_KEY=sk-... \
  cargo run -p ch01-3-harness-cli -- \
  --base-url https://api.openai.com/v1 --model gpt-4o-mini ask "Say hello"

cargo run -p ch01-3-harness-cli -- \
  --base-url http://127.0.0.1:11434/v1 --model llama3.2 ask "Say hello"
```

Those three are the command shape. I did not run them. What I ran is `--help`, the mock, and one ask with no key.

```text
     Running `target/debug/ch01-harness --help`
A tiny agent harness

Usage: ch01-harness [OPTIONS] <COMMAND>

Commands:
  ask   Send one prompt and print the reply
  help  Print this message or the help of the given subcommand(s)

Options:
      --base-url <BASE_URL>
          Base URL of an OpenAI-compatible API. Default is OpenRouter
          
          [env: OPENAI_BASE_URL=]
          [default: https://openrouter.ai/api/v1]

      --api-key <API_KEY>
          API key. Overrides OPENROUTER_API_KEY or OPENAI_API_KEY

      --model <MODEL>
          OpenRouter model id, such as openai/gpt-4o-mini or anthropic/claude-3.5-sonnet
          
          [env: HARNESS_MODEL=]
          [default: openai/gpt-4o-mini]

      --http-referer <HTTP_REFERER>
          Optional OpenRouter HTTP-Referer attribution header
          
          [env: OPENROUTER_HTTP_REFERER=]

      --title <TITLE>
          Optional OpenRouter X-Title attribution header
          
          [env: OPENROUTER_TITLE=]

      --mock
          Use the offline mock provider instead of HTTP

  -h, --help
          Print help (see a summary with '-h')

The default server is OpenRouter (https://openrouter.ai/api/v1).
A live call reads OPENROUTER_API_KEY. Another --base-url reads OPENAI_API_KEY
instead, which is how OpenAI, Ollama, and a local fake server are configured.
--api-key overrides either variable. --mock never uses a key.
```

Nextest on the library, then on the binary. The HTTP tests talk to a local wiremock server and do not read a key.

```text
   Compiling ch01-3-harness-core v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.55s
────────────
 Nextest run ID 16330b2f-dc91-4458-a6b4-72fe703c8a60 with nextest profile: default
    Starting 9 tests across 2 binaries
        PASS [   0.003s] (1/9) ch01-3-harness-core::provider openrouter_cost_accumulates_on_one_agent
        PASS [   0.004s] (2/9) ch01-3-harness-core::provider another_base_url_uses_the_openai_key
        PASS [   0.005s] (3/9) ch01-3-harness-core::provider http_error_is_classified
        PASS [   0.005s] (4/9) ch01-3-harness-core::provider empty_choices_are_an_error
        PASS [   0.003s] (5/9) ch01-3-harness-core::provider openrouter_is_the_default_and_names_its_key
        PASS [   0.005s] (6/9) ch01-3-harness-core::provider openrouter_headers_are_sent_when_set
        PASS [   0.003s] (7/9) ch01-3-harness-core::provider response_deserializes_from_openai_shape
        PASS [   0.003s] (8/9) ch01-3-harness-core::provider request_body_does_not_ask_for_a_stream
        PASS [   0.005s] (9/9) ch01-3-harness-core::provider two_calls_share_the_client_against_a_local_server
────────────
     Summary [   0.013s] 9 tests run: 9 passed, 0 skipped
```

```text
   Compiling ch01-3-harness-core v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-core)
   Compiling ch01-3-harness-cli v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.18s
────────────
 Nextest run ID 9b078687-2c7e-4404-b6dd-d0ed567573eb with nextest profile: default
    Starting 3 tests across 2 binaries
        PASS [   0.005s] (1/3) ch01-3-harness-cli::ask missing_openrouter_key_is_a_clear_error
        PASS [   0.005s] (2/3) ch01-3-harness-cli::ask mock_ask_prints_the_canned_reply
        PASS [   0.007s] (3/3) ch01-3-harness-cli::ask ask_talks_to_a_local_fake_server
────────────
     Summary [   0.007s] 3 tests run: 3 passed, 0 skipped
```

The mock ask, then the same command with no key and no `--mock`. That second process is supposed to fail.

```text
   Compiling ch01-3-harness-core v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-core)
   Compiling ch01-3-harness-cli v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.03s
     Running `target/debug/ch01-harness --mock ask 'Say hello'`
(mock) I heard you.
event: text-delta (mock) I heard you.
event: done
```

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/ch01-harness ask ping`
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

Cost accumulation, with no network. The first call reports `cost: 0.25` USD. The second reports tokens and no cost, so the ledger keeps 0.25.

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/demo`
agent: calls=2 prompt=6 completion=3 total=9 cost=0.25
```

An optional live OpenRouter call is the same `ask` without `--mock`, with `OPENROUTER_API_KEY` set. I did not make that call.

And the bench. The `time:` line is Criterion's slope. The mean in the table is the mean point estimate from `target/criterion/build_request_and_parse_response/new/estimates.json` on this same run (2,133.0 ns).

```text
   Compiling ch01-3-harness-core v0.1.0 (/workspace/steps/ch01-3-cli/crates/harness-core)
    Finished `bench` profile [optimized] target(s) in 1.82s
     Running benches/request_parse.rs (target/release/deps/request_parse-359f4e4892eb2854)
Gnuplot not found, using plotters backend
Benchmarking build_request_and_parse_response
Benchmarking build_request_and_parse_response: Warming up for 1.0000 s
Benchmarking build_request_and_parse_response: Collecting 10 samples in estimated 2.0000 s (924k iterations)
Benchmarking build_request_and_parse_response: Analyzing
build_request_and_parse_response
                        time:   [2.1317 µs 2.1351 µs 2.1383 µs]
                        change: [+3.8229% +4.1668% +4.4221%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `build_request_and_parse_response` | 2.133 µs | < 1 ms | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0 |

CI runs these tests on Ubuntu, macOS, and Windows. Criterion runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml), and the means land in the job summary. The table is from this machine.

## 1.4 Timeouts and what the reply may omit

Section 1.4 copies 1.3 and names two things a buffered call can do besides succeed. The package names are `ch01-4-harness-core` and `ch01-4-harness-cli`. The executable is `ch01-4-harness`.

```bash
cargo test -p ch01-4-harness-core -p ch01-4-harness-cli
cargo run -p ch01-4-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-4-harness-core --bench edges
```

The client now has a whole-request timeout, 60 seconds unless `--timeout-secs` says otherwise. reqwest's `timeout` covers connect and read. When `send` fails and `err.is_timeout()` is true, the error is `ProviderError::Timeout` and it carries the `Duration` we set. Anything else stays `ProviderError::Network`. This is one deadline, not a retry. Chapter 12 classifies failures, retries, and falls back. Chapter 14 is where a reply becomes a stream of the same events. The request body still has no `stream` field.

```rust
#[error("request timed out after {timeout:?}")]
Timeout { timeout: Duration },
```

```rust
let response = builder.send().await.map_err(|err| {
    if err.is_timeout() {
        ProviderError::Timeout {
            timeout: self.timeout,
        }
    } else {
        ProviderError::Network(err)
    }
})?;
```

The parse edges are the ones a real OpenRouter body takes. `usage` may be absent, and the reply is still the assistant message. `model` may be a different id than the one we asked for (a router answered on another model). `choices` may be an empty array, which is `EmptyResponse` rather than a panic on index 0. A wiremock server that delays 2 seconds, against a client whose timeout is 200 ms, is the timeout test. It does not need a key.

> **Rust sidebar: `Duration`**
> - `std::time::Duration` is a span of time, not a clock reading. `Duration::from_secs(60)` and `Duration::from_millis(200)` are the two values this section uses.
> - `{:?}` on a `Duration` is the debug form (`200ms`, `60s`). That is what the timeout error prints, so a short test timeout is not reported as one second.

> **Performance budget**
>
> - **Target:** parse a response that has `usage` and one that does not, in under 1 ms, with no network in the measurement.
> - **Measured:** `parse_response_edges` mean **792.7 ns**. The bench now builds events for a body that has `cost` and one that has no usage.

### Build, run, and benchmark

```bash
cargo build -p ch01-4-harness-core
cargo build -p ch01-4-harness-cli
cargo nextest run -p ch01-4-harness-core
cargo nextest run -p ch01-4-harness-cli
cargo run -p ch01-4-harness-cli -- --mock ask "Say hello"
cargo bench -p ch01-4-harness-core --bench edges -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

Same rule as 1.3: the run below is the mock. `--help` on `ch01-4-harness` is the 1.3 help plus `--timeout-secs` (default 60). I did not call OpenRouter.

```text
   Compiling ch01-4-harness-core v0.1.0 (/workspace/steps/ch01-4-errors/crates/harness-core)
   Compiling ch01-4-harness-cli v0.1.0 (/workspace/steps/ch01-4-errors/crates/harness-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s
     Running `target/debug/ch01-4-harness --mock ask 'Say hello'`
(mock) I heard you.
event: text-delta (mock) I heard you.
event: done
```

Nextest. `slow_server_is_a_timeout` is the delayed wiremock. The other tests are the 1.3 cases plus a body that has a model and no usage.

```text
   Compiling ch01-4-harness-core v0.1.0 (/workspace/steps/ch01-4-errors/crates/harness-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.54s
────────────
 Nextest run ID a8486731-255f-4ad9-8edb-52d2afebabb8 with nextest profile: default
    Starting 11 tests across 2 binaries
        PASS [   0.003s] ( 1/11) ch01-4-harness-core::provider missing_usage_still_keeps_the_answered_model
        PASS [   0.004s] ( 2/11) ch01-4-harness-core::provider another_base_url_uses_the_openai_key
        PASS [   0.005s] ( 3/11) ch01-4-harness-core::provider http_error_is_classified
        PASS [   0.005s] ( 4/11) ch01-4-harness-core::provider empty_choices_are_an_error
        PASS [   0.003s] ( 5/11) ch01-4-harness-core::provider openrouter_cost_accumulates_on_one_agent
        PASS [   0.005s] ( 6/11) ch01-4-harness-core::provider openrouter_headers_are_sent_when_set
        PASS [   0.004s] ( 7/11) ch01-4-harness-core::provider openrouter_is_the_default_and_names_its_key
        PASS [   0.004s] ( 8/11) ch01-4-harness-core::provider request_body_does_not_ask_for_a_stream
        PASS [   0.003s] ( 9/11) ch01-4-harness-core::provider response_deserializes_from_openai_shape
        PASS [   0.005s] (10/11) ch01-4-harness-core::provider two_calls_share_the_client_against_a_local_server
        PASS [   0.205s] (11/11) ch01-4-harness-core::provider slow_server_is_a_timeout
────────────
     Summary [   0.215s] 11 tests run: 11 passed, 0 skipped
```

```text
   Compiling ch01-4-harness-core v0.1.0 (/workspace/steps/ch01-4-errors/crates/harness-core)
   Compiling ch01-4-harness-cli v0.1.0 (/workspace/steps/ch01-4-errors/crates/harness-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.89s
────────────
 Nextest run ID 2aae669c-eab5-434f-ad8f-38dd4878aac0 with nextest profile: default
    Starting 3 tests across 2 binaries
        PASS [   0.005s] (1/3) ch01-4-harness-cli::ask mock_ask_prints_the_canned_reply
        PASS [   0.006s] (2/3) ch01-4-harness-cli::ask missing_openrouter_key_is_a_clear_error
        PASS [   0.008s] (3/3) ch01-4-harness-cli::ask ask_talks_to_a_local_fake_server
────────────
     Summary [   0.008s] 3 tests run: 3 passed, 0 skipped
```

The bench. Mean point estimate from `target/criterion/parse_response_edges/new/estimates.json`: 792.7 ns.

```text
   Compiling ch01-4-harness-core v0.1.0 (/workspace/steps/ch01-4-errors/crates/harness-core)
    Finished `bench` profile [optimized] target(s) in 1.78s
     Running benches/edges.rs (target/release/deps/edges-c0d4ef48a4ad3704)
Gnuplot not found, using plotters backend
Benchmarking parse_response_edges
Benchmarking parse_response_edges: Warming up for 1.0000 s
Benchmarking parse_response_edges: Collecting 10 samples in estimated 2.0000 s (2.5M iterations)
Benchmarking parse_response_edges: Analyzing
parse_response_edges    time:   [785.72 ns 789.72 ns 797.38 ns]
                        change: [+49.730% +50.530% +51.381%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `parse_response_edges` | 792.7 ns | < 1 ms | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0 |

CI runs these tests on Ubuntu, macOS, and Windows. Criterion runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml), and the means land in the job summary. The table is from this machine.

## Chapter summary

The harness can send one prompt and print one reply. Messages are OpenAI JSON. `Provider` hides the server. Every call, including the mock, emits the same `Event` values. The HTTP client is created once, pools connections, and can speak HTTP/2. OpenRouter is the default endpoint, `--model` selects a model id, and another `--base-url` still works for OpenAI, Ollama, and a local fake. A successful reply can name the model that answered, the token counts, and a USD cost when the server sent one. Those numbers accumulate on one agent. A slow call fails once, as a timeout, and is not retried here.

[personal note: a moment where the compiler stopped me, and how I worked out what it wanted]

## What you implement

`my-harness/tests/ch01_1.rs`, `ch01_2.rs`, `ch01_3.rs`, and `ch01_4.rs` are the milestone specs. They compile, then panic in `todo!` until the stubs in `my-harness/src` match these steps. The snapshots under `steps/` stay as the reference.

## What's next

One question and one answer is not a conversation. Chapter 2 keeps history: a `Conversation` that owns its messages, a `chat` REPL, and one session inbox the CLI submits to before it calls the provider. The system prompt from this chapter stays byte-stable across those turns, which is what makes prompt caching possible later. That snapshot uses this chapter's OpenRouter client, the same `Event` values, and the same cost ledger.
