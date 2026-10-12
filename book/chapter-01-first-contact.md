# Chapter 1: First Contact

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Every agent framework I've looked at, however impressive, comes down to one repeated act: send a list of messages to a language model and get a message back. Tools, memory, subagents, schedulers, chat-platform gateways: all of it is built around that one call.

So that's where this harness starts. By the end of this chapter I'll have:
- a Rust workspace laid out for the long haul
- a `Provider` abstraction with two implementations: a real one for any OpenAI-compatible API, and a mock for tests
- a tiny command-line tool, `harness ask "..."`, that sends one prompt and prints the reply
- tests that run with no network and no API key

I'm new to Rust, so I'll explain each language feature at the moment I first need it, in short sidebars. If you already know Rust, skip them.

[personal note: where I was with Rust when I started this chapter: what I'd read, what I'd tried, what intimidated me]

## Why start with a single call?

It's tempting to begin with the exciting parts: tool calling, agents delegating to other agents. But the agent loop is just this one call, repeated:

1. Send the conversation to the model.
2. If it asks for tools, run them, add the results, and go back to step 1.
3. Otherwise, return its answer.

If step 1 is fragile, nothing built on it will be solid. So I want it to be a clean, swappable, testable building block from day one.

This is also how Hermes Agent, the open-source Python agent from Nous Research that I'm using as my reference, is organised:
- The public entry point is the `AIAgent` class in `run_agent.py`. A turn runs through `agent/conversation_loop.py::run_conversation`, a `while` loop that calls the model, runs tools, and stops when the model answers with plain text (or when the iteration budget runs out).
- Messages are OpenAI-style dictionaries: `{"role": "system" | "user" | "assistant" | "tool", "content": ...}`. Hermes uses that format internally for every provider and converts it only when talking to a provider with a different API.
- Providers are described in one place. `providers/base.py` defines a `ProviderProfile` dataclass, and each provider (OpenRouter, Anthropic, Ollama, Nous Portal and many more) is a small plugin under `plugins/model-providers/`. The rest of Hermes asks the profile how to build requests, instead of hard-coding provider quirks everywhere.

I'm copying three ideas from that:
1. OpenAI-style messages are the internal format.
2. "Talk to a model" sits behind one interface.
3. The rest of the harness never needs to know which provider it's talking to.

What I'm *not* copying yet is Hermes's breadth: credential pools, failover between providers, streaming, prompt-caching rules. Those come later, when the harness actually needs them.

## Laying out the workspace

Rust projects are managed by **cargo**. I want two pieces from the start:
- **`harness-core`**: a library holding all the real logic (messages, providers, and later the loop, tools and agents).
- **`harness-cli`**: a thin binary that turns command-line arguments into calls into the library.

Keeping the CLI thin pays off later: a chat REPL, an HTTP server, or a Slack bot can all reuse the core. (Hermes has the same split. The CLI, the messaging gateway, the terminal UI and the desktop app all drive the same `AIAgent`.)

```
ch01-code/
├── Cargo.toml                 # the workspace
└── crates/
    ├── harness-core/
    │   ├── Cargo.toml
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── message.rs
    │   │   └── provider.rs
    │   └── tests/
    │       └── provider.rs
    └── harness-cli/
        ├── Cargo.toml
        └── src/main.rs
```

I created it like this:

```bash
mkdir ch01-code && cd ch01-code
cargo new --lib crates/harness-core
cargo new crates/harness-cli
```

The root `Cargo.toml` just lists the members:

```toml
[workspace]
resolver = "3"
members = ["crates/harness-core", "crates/harness-cli"]
```

> **Rust sidebar: cargo**
> - Cargo is the build tool, package manager and test runner in one.
> - `Cargo.toml` is the manifest; the `[dependencies]` section names *crates* (packages) from crates.io.
> - `cargo build` compiles, `cargo test` builds and runs tests, `cargo run -- <args>` runs a binary, and `cargo check` only type-checks (the fastest way to see compiler errors).
> - A *workspace* shares one `Cargo.lock` and one `target/` directory across several crates.
> - `resolver = "3"` tells cargo to prefer dependency versions that work with the Rust version I declared (`rust-version = "1.85"`).

The core library's dependencies:

```toml
[package]
name = "harness-core"
version = "0.1.0"
edition = "2021"
rust-version = "1.85"

[dependencies]
async-trait = "0.1"
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"

[dev-dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

- `reqwest` is the HTTP client. I turned off its default features and enabled `rustls-tls`, a pure-Rust TLS implementation, so I don't depend on the system's OpenSSL.
- `serde` and `serde_json` handle JSON.
- `async-trait` and `thiserror` I'll explain when they come up.
- `tokio` is only a *dev-dependency* here, because the library itself doesn't start a runtime; the tests and the CLI do.

`lib.rs` declares the modules and re-exports the important names:

```rust
pub mod message;
pub mod provider;

pub use message::{Message, Role};
pub use provider::{MockProvider, OpenAiCompatible, Provider, ProviderError};
```

## Messages

Here's the internal format: a role and some text. This chapter needs only three roles; `tool` arrives with tool calling.

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
        Self { role: Role::System, content: content.into() }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self { role: Role::User, content: content.into() }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: Role::Assistant, content: content.into() }
    }
}
```

> **Rust sidebar: structs, enums and serde**
> - A `struct` is a named bundle of fields. An `enum` is a value that is exactly one of several variants, so a `Role` can't be anything outside that list. Python dicts can't guarantee that.
> - `#[derive(...)]` asks the compiler to generate code: `Debug` for printing, `Clone` for copying, `PartialEq` for `==`, and `Serialize`/`Deserialize` (from serde) to convert to and from JSON.
> - `#[serde(rename_all = "lowercase")]` makes `Role::User` serialize as `"user"`, which is exactly what the OpenAI API expects.
> - `impl Into<String>` lets callers pass either a `&str` literal or an owned `String`.

Because `Message` already serializes to `{"role": "user", "content": "..."}`, I can send it to the API directly, with no separate "wire format" type yet.

## The Provider trait

This is the key abstraction of the chapter:

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    /// Send the conversation, get back the assistant's reply.
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError>;
}
```

It says: any type that can turn a conversation into an assistant message is a provider.
- It's `async` because calling a model means waiting on the network.
- It returns a `Result` because the network, the API key and the server can all fail.
- `Send + Sync` means a provider can be shared between threads and tasks. I'll need that once many agents run at once.

> **Rust sidebar: traits**
> - A trait is a set of methods a type promises to implement, similar to an interface.
> - Code written against a trait works with any implementation. Later, `Box<dyn Provider>` will hold "some provider, decided at runtime".
> - Async methods in traits are newer to Rust. Plain `async fn` in a trait works for generic code, but not when the trait is used as `dyn Provider`. The `#[async_trait]` macro bridges that gap by rewriting the method to return a boxed future. It's a common crate, and it costs one small allocation per call, which is nothing next to a network round-trip.

The errors get their own enum:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("API returned {status}: {body}")]
    Api { status: u16, body: String },
    #[error("response had no choices")]
    EmptyResponse,
}
```

> **Rust sidebar: Result and `?`**
> - Rust has no exceptions. A function that can fail returns `Result<T, E>`: either `Ok(value)` or `Err(error)`, and the compiler makes you deal with both.
> - The `?` operator means "if this is an error, return it from the current function now; otherwise give me the value". It can also *convert* the error along the way: the `#[from]` above generates a conversion from `reqwest::Error` into `ProviderError::Network`, so `.send().await?` just works.
> - `thiserror` generates the human-readable messages from the `#[error("...")]` attributes.

Three error kinds is a deliberate start. Hermes's `agent/error_classifier.py` has a much richer `FailoverReason` enum (rate limits, auth failures, billing, overload, context overflow, ...) because the *kind* of failure decides the recovery. I'll grow mine the same way when I add retries.

## The real provider: anything OpenAI-compatible

The OpenAI chat completions API has become a common language. OpenAI, OpenRouter, Ollama, LM Studio, vLLM and many other servers accept the same `POST {base_url}/chat/completions` request. So one implementation covers a lot of models:

```rust
// ---------- OpenAI-compatible provider ----------

pub struct OpenAiCompatible {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAiCompatible {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [Message],
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[async_trait]
impl Provider for OpenAiCompatible {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        let url = format!("{}/chat/completions", self.base_url);
        let request = ChatRequest { model: &self.model, messages };

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api { status: status.as_u16(), body });
        }

        let parsed: ChatResponse = response.json().await?;
        parsed
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message)
            .ok_or(ProviderError::EmptyResponse)
    }
}
```

A few things to notice:
- `ChatRequest<'a>` *borrows* the model name and messages instead of copying them. The `'a` is a lifetime: it tells the compiler the request can't outlive the data it points to. I only build the request long enough to serialize it, so borrowing is free and safe.
- `ChatResponse` and `Choice` include only the fields I need. serde ignores the rest of the API's response (ids, usage numbers, finish reasons) unless I ask for it, so the structs stay tiny and keep working if the API adds fields.
- When the HTTP status isn't a success, I read the body and return it in `ProviderError::Api`. A raw "401 Unauthorized" with the server's explanation is much easier to debug than a generic parse failure.
- `trim_end_matches('/')` means both `.../v1` and `.../v1/` work as the base URL.

> **Rust sidebar: async/await (the minimum)**
> - An `async fn` doesn't run when you call it. It returns a *future*, a description of work that runs when something `.await`s it.
> - While a future waits on the network, the thread is free to do other work. That's what will later let a single process run many agents at once.
> - Something has to drive the futures: that's the *runtime*. I use **tokio**, the most common one. `#[tokio::main]` on `main` starts it.

## The mock provider

Tests that hit a real API are slow, cost money, need secrets, and give different answers each run. So the second implementation is a fake:

```rust
// ---------- Mock provider for tests ----------

/// Returns a canned reply and records every conversation it was sent.
pub struct MockProvider {
    reply: String,
    pub seen: Mutex<Vec<Vec<Message>>>,
}

impl MockProvider {
    pub fn new(reply: impl Into<String>) -> Self {
        Self { reply: reply.into(), seen: Mutex::new(Vec::new()) }
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        self.seen.lock().unwrap().push(messages.to_vec());
        Ok(Message::assistant(self.reply.clone()))
    }
}
```

It always gives the same reply, and it records every conversation it receives, so a test can check *what the harness sent*, not just what came back. `seen` is behind a `Mutex` because `complete` takes `&self` (shared access), and changing shared data in Rust requires a lock. The compiler enforces that, which is something I'll appreciate later.

[personal note: a moment where the compiler stopped me, and how I worked out what it wanted]

## The CLI: `harness ask`

The binary's dependencies:

```toml
[package]
name = "harness-cli"
version = "0.1.0"
edition = "2021"
rust-version = "1.85"

[[bin]]
name = "harness"
path = "src/main.rs"

[dependencies]
harness-core = { path = "../harness-core" }
clap = { version = "4", features = ["derive", "env"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

And the whole program:

```rust
use clap::{Parser, Subcommand};
use harness_core::{Message, MockProvider, OpenAiCompatible, Provider, ProviderError};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "harness", about = "A tiny agent harness")]
struct Cli {
    /// Base URL of an OpenAI-compatible API
    #[arg(long, env = "OPENAI_BASE_URL", default_value = "https://api.openai.com/v1")]
    base_url: String,

    /// API key (local servers like Ollama accept any value)
    #[arg(long, env = "OPENAI_API_KEY", default_value = "")]
    api_key: String,

    /// Model name
    #[arg(long, env = "HARNESS_MODEL", default_value = "gpt-4o-mini")]
    model: String,

    /// Use the offline mock provider instead of a real API
    #[arg(long)]
    mock: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Send one prompt and print the reply
    Ask { prompt: String },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<(), ProviderError> {
    let provider: Box<dyn Provider> = if cli.mock {
        Box::new(MockProvider::new("(mock) I heard you."))
    } else {
        Box::new(OpenAiCompatible::new(&cli.base_url, &cli.api_key, &cli.model))
    };

    match cli.command {
        Command::Ask { prompt } => {
            let messages = vec![
                Message::system("You are a helpful assistant."),
                Message::user(prompt),
            ];
            let reply = provider.complete(&messages).await?;
            println!("{}", reply.content);
        }
    }
    Ok(())
}
```

**clap** turns the `Cli` struct into a complete argument parser, with `--help` generated from the doc comments. Each option can be given as a flag or as an environment variable, and has a default:

| Setting | Flag | Env var | Default |
|---|---|---|---|
| API base URL | `--base-url` | `OPENAI_BASE_URL` | `https://api.openai.com/v1` |
| API key | `--api-key` | `OPENAI_API_KEY` | empty |
| Model | `--model` | `HARNESS_MODEL` | `gpt-4o-mini` |
| Offline mode | `--mock` | — | off |

Inside `main`:
- `Box<dyn Provider>` is where the trait pays off. The rest of the code doesn't know or care which provider it got.
- `main` prints errors with their `Display` message (`error: API returned 401: ...`) and returns a failure exit code, instead of dumping Rust's debug output.

Configuring it through environment variables is the same approach Hermes takes: secrets live in `~/.hermes/.env`, and the settings that pick a provider and model live in its config file. Mine are just environment variables for now:

```bash
# OpenAI
export OPENAI_API_KEY=sk-...
cargo run -p harness-cli -- ask "Explain a turn loop in one sentence."

# OpenRouter
export OPENAI_BASE_URL=https://openrouter.ai/api/v1
export OPENAI_API_KEY=sk-or-...
export HARNESS_MODEL=nousresearch/hermes-3-llama-3.1-70b
cargo run -p harness-cli -- ask "Hello!"

# Ollama, running locally (no real key needed)
export OPENAI_BASE_URL=http://localhost:11434/v1
export HARNESS_MODEL=llama3.2
cargo run -p harness-cli -- ask "Hello!"

# No network at all
cargo run -p harness-cli -- --mock ask "Hello!"
# (mock) I heard you.
```

The model names are examples. Use whatever your provider lists.

## Tests

The tests live in `crates/harness-core/tests/provider.rs`:

```rust
use harness_core::provider::{ChatRequest, ChatResponse};
use harness_core::{Message, MockProvider, Provider, Role};

#[tokio::test]
async fn mock_provider_answers_and_records() {
    let mock = MockProvider::new("Hello, Daniel's harness!");
    let convo = vec![Message::system("Be brief."), Message::user("Say hi")];

    let reply = mock.complete(&convo).await.unwrap();

    assert_eq!(reply.role, Role::Assistant);
    assert_eq!(reply.content, "Hello, Daniel's harness!");
    let seen = mock.seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0][1].content, "Say hi");
}

#[test]
fn request_serializes_to_openai_shape() {
    let msgs = [Message::user("hi")];
    let req = ChatRequest { model: "gpt-4o-mini", messages: &msgs };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [{ "role": "user", "content": "hi" }]
        })
    );
}

#[test]
fn response_deserializes_from_openai_shape() {
    let body = r#"{
        "id": "chatcmpl-123",
        "choices": [{ "index": 0,
                      "message": { "role": "assistant", "content": "Hi there" },
                      "finish_reason": "stop" }],
        "usage": { "prompt_tokens": 5, "completion_tokens": 2 }
    }"#;
    let parsed: ChatResponse = serde_json::from_str(body).unwrap();
    assert_eq!(parsed.choices[0].message, Message::assistant("Hi there"));
}
```

There are three tests, each guarding one boundary:
1. **The mock works and records what it was sent.** Every future test of the agent loop will use this.
2. **Requests serialize to exactly the JSON shape the API expects.** If I rename a field by accident, this fails before a real API rejects it.
3. **A realistic API response, including fields I don't model, parses correctly.**

`#[tokio::test]` is `#[test]` with a tokio runtime, so the test itself can be `async`.

## Did it work?

From the workspace root, `cargo build` and `cargo test` both succeed:

```
$ cargo test
...
running 3 tests
test request_serializes_to_openai_shape ... ok
test response_deserializes_from_openai_shape ... ok
test mock_provider_answers_and_records ... ok

test result: ok. 3 passed; 0 failed
```

To check the real HTTP path without spending tokens, I pointed the CLI at a tiny local fake server that echoes the request back:

```
$ OPENAI_BASE_URL=http://127.0.0.1:18931/v1 OPENAI_API_KEY=sk-test \
  HARNESS_MODEL=llama3.2 ./target/debug/harness ask "What is Rust?"
echo: What is Rust? (model llama3.2, auth Bearer sk-test)
```

This confirms the URL, the model name, the bearer token and the message all made it over the wire. And when nothing is listening, the error is readable:

```
$ OPENAI_BASE_URL=http://127.0.0.1:9 ./target/debug/harness ask hi
error: network error: error sending request for url (http://127.0.0.1:9/chat/completions)
```

[personal note: the first reply from a real model, and which model/provider I used]

## What we built

- A two-crate workspace: the logic lives in a library; the CLI is a thin shell over it.
- `Message` and `Role` types that serialize to OpenAI's format, which is also Hermes's internal format.
- A `Provider` trait: the single seam between the harness and every model it will ever talk to.
- `OpenAiCompatible`, which works with OpenAI, OpenRouter, Ollama and similar servers through three settings.
- `MockProvider`, so the harness can be tested offline and the same way every time.
- `harness ask "..."`, a working command-line tool.
- Tests that pin the wire format in both directions.

That's roughly 200 lines of Rust. Along the way I used cargo workspaces, structs, enums, derives, serde, traits and trait objects, `Result` and `?`, and enough async to make an HTTP call.

[personal note: what felt harder or easier than expected, compared with languages I know]

## What's next

One question and one answer isn't a conversation. In **Chapter 2: Conversations and a Chat REPL**, the harness keeps history: a `Conversation` type that owns its messages, a `harness chat` command that keeps going until you quit, and slash commands like `/new` and `/exit`. That's where Rust's ownership rules start to matter: who owns the history, who can borrow it, and who can change it. It's also where Hermes's rule about never changing earlier messages in a conversation starts to make sense.
